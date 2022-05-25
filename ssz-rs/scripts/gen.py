import os
import sys
from pathlib import Path
import yaml
import shutil

input_root_path = "consensus-spec-tests/tests/general/phase0/ssz_generic/"
# relative to workspace root
output_root_path = "ssz-rs/tests/data/"

test_src_fmt = """mod test_utils;

use ssz_rs::prelude::*;
use test_utils::{
    deserialize, hash_tree_root, read_ssz_snappy_from_test_data, root_from_hex, serialize,
};
"""

containers_defn_fmt = """
#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct SingleFieldTestStruct {
    a: u8,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct SmallTestStruct {
    a: u16,
    b: u16,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, SimpleSerialize)]
struct FixedTestStruct {
    a: u8,
    b: u64,
    c: u32,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, SimpleSerialize)]
struct VarTestStruct {
    a: u16,
    b: List<u16, 1024>,
    c: u8,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct ComplexTestStruct {
    a: u16,
    b: List<u16, 128>,
    c: u8,
    d: List<u8, 256>,
    e: VarTestStruct,
    f: Vector<FixedTestStruct, 4>,
    g: Vector<VarTestStruct, 2>,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct BitsStruct {
    a: Bitlist<5>,
    b: Bitvector<2>,
    c: Bitvector<1>,
    d: Bitlist<6>,
    e: Bitvector<8>,
}
"""

valid_test_fmt = """
#[test]
fn test_{type}_{handler}() {{
    let mut value = {value};
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data("{data_path}");
    assert_eq!(encoding, expected_encoding);

    let recovered_value: {rust_type} = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root = root_from_hex("{root}");
    assert_eq!(root, expected_root);
}}
"""

invalid_test_fmt = """
#[test]
#[should_panic]
fn test_{type}_{handler}() {{
    let encoding = read_ssz_snappy_from_test_data("{data_path}");

    deserialize::<{rust_type}>(&encoding);
}}
"""


def _read_yaml(path):
    with open(path) as f:
        return yaml.load(f, Loader=yaml.FullLoader)


def _map_to_rust_u256(value):
    data = int(value).to_bytes(32, "little")
    as_bytes = []
    for byte in data:
        as_bytes.append(byte)
    return f"U256({as_bytes})"


def _map_to_rust_vector(value, rust_type):
    if "U256" in rust_type:
        elements = []
        for element in value:
            elements.append(_map_to_rust_u256(element))
        inline = ", ".join(elements)
        return f"{rust_type}::from_iter([{inline}])"
    elif "u128" in rust_type:
        inline = ", ".join(value)
        return f"{rust_type}::from_iter([{inline}])"
    else:
        return f"{rust_type}::from_iter({str(value).lower()})"


def _decode_bitvector(value, bound):
    value = bytes.fromhex(value[2:])
    value = [b for b in value]
    bits = []
    bit_count = 0
    for byte in value:
        valid_bits = format(byte, "#010b")[2:]
        valid_bits = valid_bits[::-1]
        for bit in valid_bits:
            bits.append("true" if bit == "1" else "false")
            bit_count += 1
    inline = ", ".join(bits[:bound])
    return f"Bitvector::<{bound}>::from_iter([{inline}])"


def _decode_bitlist(value, bound):
    value = bytes.fromhex(value[2:])
    value = [b for b in value]
    bits = []
    for byte in value:
        valid_bits = format(byte, "#010b")[2:]
        valid_bits = valid_bits[::-1]
        for bit in valid_bits:
            bits.append("true" if bit == "1" else "false")
    while bits[-1] == "false":
        bits.pop()
    assert bits[-1] == "true"
    bits.pop()
    assert len(bits) <= bound
    inline = ", ".join(bits)
    return f"Bitlist::<{bound}>::from_iter([{inline}])"


def _decode_field_value(name, value, rust_type):
    if rust_type == "VarTestStruct":
        if name == "B":
            inline = ", ".join(map(str, value))
            return f"List::<u16, 1024>::from_iter([{inline}])"
        else:
            return value
    elif rust_type == "ComplexTestStruct":
        if name == "B":
            inline = ", ".join(map(str, value))
            return f"List::<u16, 128>::from_iter([{inline}])"
        elif name == "D":
            value = bytes.fromhex(value[2:])
            inline = ", ".join(map(str, value))
            return f"List::<u8, 256>::from_iter([{inline}])"
        elif name == "E":
            return _map_to_rust_struct(value, "VarTestStruct")
        elif name == "F":
            inner = [_map_to_rust_struct(v, "FixedTestStruct") for v in value]
            inline = ", ".join(inner)
            return f"Vector::<FixedTestStruct, 4>::from_iter([{inline}])"
        elif name == "G":
            inner = [_map_to_rust_struct(v, "VarTestStruct") for v in value]
            inline = ", ".join(inner)
            return f"Vector::<VarTestStruct, 2>::from_iter([{inline}])"
        else:
            return value
    elif rust_type == "BitsStruct":
        if name == "A":
            return _decode_bitlist(value, 5)
        elif name == "B":
            return _decode_bitvector(value, 2)
        elif name == "C":
            return _decode_bitvector(value, 1)
        elif name == "D":
            return _decode_bitlist(value, 6)
        elif name == "E":
            return _decode_bitvector(value, 8)
        else:
            raise AssertionError("unsupported field for BitsStruct")
    else:
        return value


def _map_to_rust_struct(value, rust_type):
    inline = []
    for k, v in value.items():
        value = _decode_field_value(k, v, rust_type)
        field = f"{k.lower()}: {value}"
        inline.append(field)
    return f"{rust_type}{{{', '.join(inline)}}}"


container_types_to_snake_case = {
    "SingleFieldTestStruct": "single_field_test_struct",
    "SmallTestStruct": "small_test_struct",
    "FixedTestStruct": "fixed_test_struct",
    "VarTestStruct": "var_test_struct",
    "ComplexTestStruct": "complex_test_struct",
    "BitsStruct": "bits_struct",
}


def _map_to_snake_case(handler):
    for k, v in container_types_to_snake_case.items():
        if k in handler:
            return handler.replace(k, v)
    return handler


def _map_to_rust_str(test_data):
    value = test_data["value"]
    if "uint_256" in test_data["handler"]:
        return _map_to_rust_u256(value)
    elif "bitvec" in test_data["handler"]:
        return _decode_bitvector(value, int(test_data["bound"]))
    elif "bitlist" in test_data["handler"]:
        return _decode_bitlist(value, int(test_data["bound"]))
    elif "vec_" in test_data["handler"]:
        return _map_to_rust_vector(value, test_data["rust_type"])
    elif any(
        map(
            lambda typ: typ in test_data["handler"],
            container_types_to_snake_case.keys(),
        )
    ):
        return _map_to_rust_struct(value, test_data["rust_type"])
    else:
        return str(value).lower()


def _map_to_data_path(path):
    target = path.removeprefix(input_root_path)
    return output_root_path + target


def _do_copy(out_path, in_path):
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    shutil.copyfile(in_path, out_path)


def _map_element_type(typ):
    if typ == "bool":
        return typ
    elif typ == "uint256":
        return "U256"
    else:
        return f"u{typ[4:]}"


def _resolve_rust_type(test_data):
    if test_data["type"] == "boolean":
        return "bool"
    elif "uint" in test_data["type"]:
        parts = test_data["handler"].split("_")
        width = parts[1]
        if "256" in width:
            return "U256"
        else:
            return f"u{width}"
    elif "basic_vector" in test_data["type"]:
        parts = test_data["handler"].split("_")
        element_type = _map_element_type(parts[1])
        length = parts[2]
        return f"Vector::<{element_type}, {length}>"
    elif "containers" in test_data["type"]:
        parts = test_data["handler"].split("_")
        return parts[0]
    elif "bitvector" in test_data["type"]:
        parts = test_data["handler"].split("_")
        bound = parts[1]
        test_data["bound"] = bound
        return f"Bitvector::<{bound}>"
    elif "bitlist" in test_data["type"]:
        parts = test_data["handler"].split("_")
        bound = parts[1]
        if bound == "no":
            bound = 256
        test_data["bound"] = bound
        return f"Bitlist::<{bound}>"
    else:
        raise AssertionError("unsupported type")


def _render_valid_boolean_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)


def _render_valid_uint_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)


def _render_valid_basic_vector_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)


def _render_valid_container_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)
    test_data["handler"] = _map_to_snake_case(test_data["handler"])


def _render_valid_bitvector_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)
    test_data["handler"] = _map_to_snake_case(test_data["handler"])


def _render_valid_bitlist_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["root"] = test_data["root"][2:]
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["value"] = _map_to_rust_str(test_data)
    test_data["handler"] = _map_to_snake_case(test_data["handler"])


def _render_valid_test_data(test_data, typ):
    extra = ""
    if "boolean" in typ:
        _render_valid_boolean_test(test_data)
    elif "uint" in typ:
        _render_valid_uint_test(test_data)
    elif "basic_vector" in typ:
        _render_valid_basic_vector_test(test_data)
    elif "containers" in typ:
        _render_valid_container_test(test_data)
        extra = containers_defn_fmt
    elif "bitvector" in typ:
        _render_valid_bitvector_test(test_data)
    elif "bitlist" in typ:
        _render_valid_bitlist_test(test_data)
    else:
        raise AssertionError("unsupported type")
    print(f"{valid_test_fmt.format(**test_data)}")


def _handle_valid(typ, path, handler):
    root_dir = path + "/" + handler
    test_data = {
        "type": typ,
        "handler": handler,
    }

    for part in os.listdir(root_dir):
        target_path = root_dir + "/" + part
        if "meta" in part:
            meta = _read_yaml(target_path)
            test_data["root"] = meta["root"]
        elif "value" in part:
            value = _read_yaml(target_path)
            test_data["value"] = value
        else:
            assert "ssz_snappy" in part
            test_data["input_data_path"] = target_path
    _render_valid_test_data(test_data, typ)


def _render_invalid_boolean_test(test_data):
    test_data["data_path"] = _map_to_data_path(test_data["input_data_path"])
    _do_copy(test_data["data_path"], test_data["input_data_path"])
    test_data["rust_type"] = _resolve_rust_type(test_data)
    test_data["handler"] = _map_to_snake_case(test_data["handler"])


def _render_invalid_test_data(test_data, typ):
    if "boolean" in typ:
        _render_invalid_boolean_test(test_data)
    elif "uint" in typ:
        _render_invalid_boolean_test(test_data)
    elif "basic_vector" in typ:
        _render_invalid_boolean_test(test_data)
    elif "containers" in typ:
        _render_invalid_boolean_test(test_data)
    elif "bitvector" in typ:
        _render_invalid_boolean_test(test_data)
    elif "bitlist" in typ:
        _render_invalid_boolean_test(test_data)
    else:
        raise AssertionError("unsupported type")
    print(invalid_test_fmt.format(**test_data))


def _handle_invalid(typ, path, handler):
    root_dir = path + "/" + handler
    test_data = {
        "type": typ,
        "handler": handler,
    }
    for part in os.listdir(root_dir):
        target_path = root_dir + "/" + part
        assert "ssz_snappy" in part
        test_data["input_data_path"] = target_path
    _render_invalid_test_data(test_data, typ)


def _main(typ):
    for fmt in ("valid", "invalid"):
        if fmt == "valid":
            print(test_src_fmt)
            if typ == "containers":
                print(containers_defn_fmt)
        path = input_root_path + typ + "/" + fmt
        for handler in os.listdir(path):
            if fmt == "valid":
                _handle_valid(typ, path, handler)
            else:
                assert fmt == "invalid"
                _handle_invalid(typ, path, handler)


if __name__ == "__main__":
    typ = sys.argv[1]
    _main(typ)
