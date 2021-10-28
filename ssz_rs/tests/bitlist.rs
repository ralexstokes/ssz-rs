mod test_utils;

use ssz_rs::prelude::*;
use test_utils::{
    deserialize, hash_tree_root, read_ssz_snappy_from_test_data, root_from_hex, serialize,
};

#[test]
fn test_bitlist_bitlist_8_random_4() {
    let value = Bitlist::<8>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_random_2() {
    let value = Bitlist::<16>::from_iter([true, true, false, true, true, true, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("8bd00e1a82454504a094276182544df713103259ba3f96133871a55281b44d18");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_lengthy_0() {
    let value = Bitlist::<31>::from_iter([
        false, false, true, false, false, false, false, false, true, true, false, true, false,
        true, true, false, true, false, true, false, true, true, true, true, false, false, true,
        false, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("2b4e175a3cabe516e47026098d7a07a105d94c6e1d7859c5f8e99d81d5fb73e5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_random_3() {
    let value = Bitlist::<8>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_random_2() {
    let value = Bitlist::<3>::from_iter([true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ff55c97976a840b4ced964ed49e3794594ba3f675238b5fd25d282b60f70a194");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_random_2() {
    let value = Bitlist::<2>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_max_4() {
    let value = Bitlist::<31>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4b5bcf109d8b0381e1ca551794c9fb864838f5b07057e05da75830f7999d96de");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_nil_1() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_max_3() {
    let value = Bitlist::<31>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("b8570b9c932d5fd3d2bd727a64d527f790d8261acd9f6ce2786cc1fa34dd2fa8");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_max_0() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_zero_4() {
    let value = Bitlist::<4>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d647eb2598d33d7216256356596d29cecd31c1ba7a7ff25ccb5be4a453410b9d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_zero_3() {
    let value = Bitlist::<4>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_lengthy_1() {
    let value = Bitlist::<513>::from_iter([
        true, false, false, true, true, true, false, false, true, true, false, true, true, false,
        true, false, false, false, true, false, false, true, false, false, true, true, false, true,
        true, true, false, true, false, false, false, false, false, false, true, false, false,
        false, true, true, true, false, false, true, false, false, true, false, true, true, true,
        true, false, false, false, false, false, true, false, false, true, true, false, true, true,
        true, false, true, false, false, true, false, true, false, true, false, true, false, true,
        false, false, false, false, true, true, true, true, true, false, true, false, false, false,
        false, true, false, true, false, false, true, false, false, false, true, true, true, false,
        true, false, true, false, true, true, true, false, false, true, false, false, false, true,
        true, true, true, true, true, false, false, false, false, false, false, false, true, false,
        false, true, false, true, false, false, true, true, true, true, true, false, false, false,
        true, false, true, false, false, true, true, true, true, true, false, false, true, false,
        false, true, true, false, true, false, true, false, true, true, false, true, true, true,
        true, false, true, true, true, true, false, false, false, false, false, false, false, true,
        false, false, true, true, false, false, true, false, true, false, false, false, false,
        false, true, false, false, false, true, false, false, false, false, false, true, true,
        true, false, false, false, true, false, true, true, true, false, false, false, true, false,
        false, true, false, true, true, true, true, false, true, true, true, false, true, false,
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        false, true, true, true, false, false, true, false, false, true, false, true, true, false,
        false, false, true, false, true, true, false, false, false, false, true, true, true, false,
        true, false, false, true, false, true, false, true, false, true, false, true, true, true,
        true, true, true, true, true, true, false, false, true, true, false, false, true, true,
        true, true, false, true, true, true, false, false, false, true, false, true, true, true,
        true, false, true, true, false, false, false, true, true, false, false, true, true, false,
        true, false, false, true, true, true, false, true, false, false, true, false, false, false,
        false, false, true, true, true, true, false, false, false, false, false, true, true, false,
        false, true, false, true, false, false, true, true, true, true, false, true, true, false,
        true, false, false, true, false, true, true, false, false, false, false, false, false,
        false, false, false, false, true, false, false, true, true, false, true, true, true, true,
        false, false, true, false, false, true, false, true, false, false, true, false, false,
        false, true, false, true, true, false, true, false, true, false, false, false, false,
        false, false, false, false, true, true, true, false, true, false, true, true, true, false,
        false, false, false, true, false, false, true, true, true, true, true, false, false, false,
        true, true, true, false, true, false, false, false, false, false, true, true, false, false,
        true, false, true, false, false, false, false, false, true, false, true, true, true, false,
        true, false, false, true, true, false, false, true, false, false, true, true, true, false,
        true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("10041d4cf07da1077e84c9b5c01fa6d5f29ba8feb934ebdf7ca184a2857cdf55");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_zero_1() {
    let value = Bitlist::<5>::from_iter([false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("16aaf795af421b6156d4c3319879d422a0c3ffd26db07207a54d6cafcbef0b10");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_zero_3() {
    let value = Bitlist::<8>::from_iter([false, false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7d360196d14b15261c9e5f576df8dc8b48d18d79b4198f167741052747704352");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_nil_0() {
    let value = Bitlist::<513>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28ba1834a3a7b657460ce79fa3a1d909ab8828fd557659d4d0554a9bdbc0ec30");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_zero_4() {
    let value = Bitlist::<8>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d647eb2598d33d7216256356596d29cecd31c1ba7a7ff25ccb5be4a453410b9d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_lengthy_0() {
    let value = Bitlist::<2>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_nil_0() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_max_2() {
    let value = Bitlist::<31>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("e78c29807c3f3ced69109d22d734a1c69d361e0671c21b8681a1761333e95537");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_random_2() {
    let value = Bitlist::<8>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_random_3() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_random_3() {
    let value = Bitlist::<2>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_random_4() {
    let value = Bitlist::<16>::from_iter([true, false, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("53de69c30b9c07be9cba006e32db34dc1e4ebfe649bc94aa7c8aae0ef419aeed");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_random_3() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, true, false, true, true, false, true, true, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("160937bf5c6f4256c285385214969c965a8c841be474c62d7ed3c184ec3cdb69");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_lengthy_1() {
    let value = Bitlist::<31>::from_iter([
        false, false, false, false, true, false, false, false, true, true, false, true, false,
        true, false, false, true, true, false, false, true, false, true, true, true, false, true,
        true, true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5054e572357a7c57c9f05e8f79208348c9dfe9f28461d7935700459b1ae2307");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_random_4() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_random_4() {
    let value = Bitlist::<3>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_nil_1() {
    let value = Bitlist::<513>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28ba1834a3a7b657460ce79fa3a1d909ab8828fd557659d4d0554a9bdbc0ec30");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_lengthy_1() {
    let value = Bitlist::<2>::from_iter([true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ff55c97976a840b4ced964ed49e3794594ba3f675238b5fd25d282b60f70a194");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_lengthy_0() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, true, false, true, true, false, false, false, true, true, false, true,
        true, false, true, true, false, false, true, false, false, true, false, true, false, false,
        true, false, true, false, true, true, true, false, true, true, true, true, false, true,
        false, true, false, false, false, false, true, false, false, true, false, true, false,
        false, false, false, true, true, false, true, true, true, true, false, false, false, true,
        false, true, false, true, true, true, true, true, true, true, true, true, true, true,
        false, true, true, true, false, true, false, false, false, true, false, false, true, false,
        false, true, true, false, false, false, false, true, true, true, true, true, true, false,
        false, false, false, true, true, true, true, true, false, true, true, false, false, false,
        false, false, true, true, false, true, false, true, false, false, false, true, true, false,
        false, true, false, true, true, false, false, true, false, false, false, true, true, true,
        false, true, true, true, true, true, false, true, false, false, false, false, true, false,
        true, false, true, false, false, true, true, true, true, false, true, true, true, true,
        false, true, true, true, false, false, false, false, false, true, true, false, false, true,
        true, true, false, false, true, true, false, true, false, false, false, false, true, false,
        false, true, false, false, false, true, false, false, true, true, true, false, true, false,
        true, true, false, false, false, true, false, false, false, false, true, true, true, false,
        true, true, true, false, false, false, true, false, false, false, false, true, false, true,
        false, false, true, false, false, false, false, false, true, true, false, false, true,
        false, false, false, true, true, false, false, true, true, false, false, true, false,
        false, true, false, true, false, false, true, true, false, false, false, false, false,
        true, false, true, false, false, false, false, true, false, true, false, false, false,
        true, true, false, false, false, false, false, true, true, false, true, true, true, false,
        false, false, false, false, true, false, false, false, false, true, false, true, true,
        true, true, true, true, true, true, false, true, true, false, false, true, true, true,
        true, true, false, true, true, false, true, true, true, false, false, true, true, true,
        true, true, true, false, false, true, false, false, true, true, true, false, true, true,
        true, false, true, true, false, true, true, false, false, true, true, true, false, true,
        false, true, true, true, false, false, true, false, false, true, false, false, false,
        false, false, false, false, true, true, false, true, false, false, false, false, false,
        true, false, false, true, false, true, false, true, true, true, true, false, false, true,
        false, false, true, true, true, true, false, false, false, true, false, false, true, true,
        false, false, true, false, false, true, true, false, false, false, false, true, true, true,
        true, false, true, false, true, true, true, true, true, true, true, true, false, false,
        false, false, true, true, false, true, true, false, true, true, false, true, false, true,
        true, true, false, false, true, false, false, false, false, false, true, true, true, true,
        false, true, true, true, true, true, false, false, true, false, false, false, false, true,
        false, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("77184930e328732d5413240f6114e269a9df6573d8b177f03d328eda7d3ffae2");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_zero_0() {
    let value = Bitlist::<5>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_zero_2() {
    let value = Bitlist::<8>::from_iter([false, false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7d360196d14b15261c9e5f576df8dc8b48d18d79b4198f167741052747704352");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_zero_2() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_max_1() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_nil_1() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_lengthy_0() {
    let value = Bitlist::<4>::from_iter([true, true, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("9d2816f451512382c000156fad1578555537321084d091d3c7b228aa705c36aa");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_max_1() {
    let value = Bitlist::<513>::from_iter([true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("b00f282b126680bcbd302d657b117dc32294c4cb586f76c244932141012e6a82");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_lengthy_0() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, true, false, true, false, false, true, true, true,
        false, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("6232812aa34ca3e9ce77374f8915f059832b1671edbbe38e8816196b2be450d5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_max_0() {
    let value = Bitlist::<4>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_random_0() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_nil_2() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_max_0() {
    let value = Bitlist::<513>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("3116c9a3fab7c6ebf0978f8ef07aa2c27ea9c79887d773980a39b95e5c035593");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_lengthy_1() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, false, true, false, true, true, false, false, true,
        true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("8c2b7bd1b88a7d1be36dad5c3734873af45f38d2d4618f83211b394aa65a665e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_lengthy_1() {
    let value = Bitlist::<4>::from_iter([true, false, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("e90722eb4d2a891700f1f3aa2e95661e707b19e60e147a96f8cf089e8cbc4bec");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_nil_0() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_max_1() {
    let value = Bitlist::<4>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_random_1() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_nil_3() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_nil_4() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_max_3() {
    let value = Bitlist::<16>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("16472e350c0d8e0cf112307b5cfa66561668ffef5f9f3281c9ad0af85122ba2c");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_zero_0() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_random_1() {
    let value = Bitlist::<512>::from_iter([
        false, true, false, true, false, false, false, true, false, true, true, false, true, false,
        true, true, false, false, true, false, false, true, false, false, true, true, true, true,
        true, true, true, false, false, false, true, false, false, false, false, false, false,
        false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4979bcefe3ded00d52ea1342595d1390e372a93c4acf10ed2c3c1fc604d1a92e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_random_1() {
    let value = Bitlist::<513>::from_iter([
        true, false, true, false, true, true, true, true, true, false, false, false, false, true,
        false, true, true, true, true, false, false, true, true, false, true, true, true, false,
        false, false, false, true, false, false, false, true, true, true, false, true, true, false,
        false, true, true, false, false, true, false, false, false, true, true, true, false, true,
        true, true, false, true, false, true, true, false, false, false, true, false, true, false,
        true, true, true, true, false, false, true, true, true, false, false, false, false, false,
        true, true, true, true, false, false, true, false, true, true, true, false, true, true,
        false, false, false, false, true, false, true, true, false, true, false, false, false,
        true, true, true, true, false, true, true, false, true, false, false, true, false, true,
        true, false, true, false, false, false, false, false, false, true, true, false, false,
        false, true, true, true, false, false, false, true, false, true, false, true, false, false,
        true, true, false, true, true, true, false, false, true, false, true, true, true, false,
        true, false, false, false, true, true, false, false, false, true, true, false, true, true,
        true, true, true, true, false, true, true, false, false, true, false, false, true, true,
        false, true, false, true, true, true, true, false, false, true, false, true, false, true,
        false, true, true, true, true, true, false, true, true, true, false, true, false, true,
        true, false, false, false, false, true, false, false, true, false, true, true, false, true,
        false, true, true, true, true, true, false, false, true, true, false, true, false, true,
        false, false, false, false, true, false, true, true, true, true, false, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("339f84a3e78443af74c3ea49f06c6d1933f3b4e3440dc631820662651085a306");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_zero_0() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("3f398072fb9acafba24683799d8250de322a96a12e3016134220db24526b372d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_max_4() {
    let value = Bitlist::<16>::from_iter([true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4b07c3799db025f3aa92ced1e8545367a2b6e44960f479d3f9d62b61812892d5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_max_0() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_zero_2() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0f4ea9e6bc6fce537e76838bafa08072aec839c4acc1d3a8c62bb4a253a0a451");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_zero_2() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_nil_0() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_lengthy_1() {
    let value = Bitlist::<512>::from_iter([
        false, true, true, true, true, false, true, false, false, false, false, true, true, true,
        false, true, true, false, false, false, false, false, false, false, true, true, false,
        false, false, true, true, false, true, false, false, false, true, true, false, false, true,
        true, false, true, false, true, true, true, false, true, false, false, true, false, true,
        false, true, false, true, true, false, false, true, true, true, false, false, false, false,
        false, true, true, false, true, true, false, true, true, false, true, false, false, false,
        false, true, true, false, false, true, false, false, true, true, false, true, true, true,
        false, true, false, true, true, true, true, false, true, true, false, false, true, true,
        true, false, true, false, true, true, false, true, true, false, true, true, true, true,
        true, false, true, false, false, false, true, false, false, true, true, false, false,
        false, false, true, false, false, true, false, true, false, false, false, false, false,
        true, true, false, true, false, true, true, false, false, false, false, false, false, true,
        false, true, false, true, true, true, false, false, true, false, true, false, true, true,
        true, false, true, true, false, true, false, false, false, true, false, true, true, true,
        false, true, false, true, false, true, false, true, false, false, true, true, true, true,
        true, true, false, true, false, true, false, true, true, false, true, false, true, false,
        false, false, true, false, false, true, false, false, true, false, true, true, true, true,
        false, true, false, false, true, true, true, true, true, false, true, false, true, true,
        false, true, true, true, true, true, false, false, false, false, true, false, false, true,
        true, true, false, false, true, true, true, true, true, false, false, true, false, false,
        false, true, true, true, true, true, true, false, false, true, true, false, true, true,
        false, true, false, true, true, false, true, true, true, false, true, false, true, true,
        true, true, false, false, false, true, true, false, false, false, true, false, false, true,
        false, true, false, true, false, false, false, false, true, true, false, false, false,
        false, false, true, false, false, true, false, true, false, false, true, true, true, false,
        true, true, false, true, false, true, true, false, false, true, true, true, true, false,
        true, false, false, true, false, false, false, false, false, false, true, false, false,
        true, true, false, true, false, true, true, true, false, false, false, false, true, true,
        false, false, false, false, false, true, true, false, false, true, false, true, true, true,
        true, true, true, true, false, false, false, false, true, true, true, true, false, true,
        true, true, false, false, false, true, true, false, true, true, true, true, false, false,
        true, false, false, true, true, false, false, true, true, false, false, false, true, false,
        true, true, true, false, false, true, false, true, false, false, true, false, false, true,
        false, true, true, false, true, false, true, false, true, false, true, true, false, true,
        false, false, true, true, true, true, false, true, true, false, false, false, true, true,
        false, false, false, false, false, false, true, false, true, true, false, true, true,
        false, false, false, false, false, false, false, false, false, true, false, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("2b28c2217c3f1f99e0c5ad46c77be392323ae7c6e68612e6b1701e762a0285e7");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_lengthy_3() {
    let value = Bitlist::<1>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_max_1() {
    let value = Bitlist::<3>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_lengthy_4() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_lengthy_0() {
    let value = Bitlist::<3>::from_iter([false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d86ae2ca925345bf2412bde450ac175742d979c1ea7b961bd1efe10beb9500cf");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_nil_1() {
    let value = Bitlist::<512>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7a0501f5957bdf9cb3a8ff4966f02265f968658b7a9c62642cba1165e86642f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_max_2() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_zero_0() {
    let value = Bitlist::<31>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("6463f4376faab07e62e5a4737d2d95ad690892f8fae0b9559c0ed3ae96bb2790");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_nil_1() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_zero_4() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c3f35acbdbda16dc35969a4b0c817b2a7c9f8b037ace72cae4efb76797d8d4c4");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_zero_3() {
    let value = Bitlist::<2>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_zero_4() {
    let value = Bitlist::<2>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_zero_3() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ec7fd7a922a87b641e3c8e0f2b092b1f470050c14409fcd95985c07024a429f4");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_random_0() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, false, false, true, true, false, false, false, false, false,
        true, true, true, false, false, false, false, false, true, false, true, true, false, false,
        true, false, true, false, true, false, false, true, true, false, false, true, true, false,
        false, true, false, false, true, true, false, true, false, true, false, false, false, true,
        false, false, true, true, false, true, false, true, true, false, true, false, true, false,
        false, true, true, true, true, false, false, false, false, true, true, false, true, false,
        true, false, true, true, true, true, false, true, true, true, true, false, false, false,
        false, true, false, true, false, false, false, false, false, false, true, false, false,
        true, true, false, false, false, true, false, false, true, false, false, false, false,
        true, true, true, true, true, false, true, false, false, false, false, false, true, false,
        true, false, false, true, true, false, false, true, true, true, true, false, false, false,
        true, true, true, false, false, true, true, true, true, true, true, false, false, true,
        true, false, false, false, true, true, false, false, false, true, false, false, true,
        false, true, true, true, false, true, false, false, true, true, false, true, false, true,
        false, true, false, true, true, true, false, false, false, true, false, true, true, true,
        true, false, false, false, false, false, false, false, true, true, false, false, true,
        true, false, true, true, false, false, false, false, false, false, true, false, false,
        false, true, true, true, true, true, true, false, false, false, true, false, false, false,
        true, true, true, false, false, false, false, true, true, true, true, false, false, true,
        true, true, false, false, true, false, false, false, true, true, true, true, false, false,
        false, false, true, false, true, false, true, false, true, false, true, true, true, false,
        false, true, true, true, false, false, true, true, false, true, true, true, false, true,
        true, true, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d01782fa00046d31ecef1828d806bc82a0635ba68a829abaea5bc5e83cfc3b39");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_random_0() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, true, true, false, true, true, true, true, false, true, false,
        true, true, false, false, true, false, false, true, true, true, false, false, true, false,
        true, true, false, true, true, true, true, false, true, true, false, false, false, true,
        true, false, false, true, true, false, false, true, true, false, true, false, false, false,
        false, false, false, true, true, true, false, false, true, true, false, true, false, true,
        false, true, false, false, false, false, false, false, false, false, false, true, false,
        false, true, false, true, true, false, true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("62110ea980c0e8b321149e2681d66a3c9ca6d2af615ed3f7b2ea1f950519cee3");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_zero_1() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("25f3b33649409489b22232a7706a5ae5c4f5b62cadee098a758d3fa16d1087d2");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_max_1() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_zero_1() {
    let value = Bitlist::<3>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_max_2() {
    let value =
        Bitlist::<16>::from_iter([true, true, true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("5879404f965b9356ffe1e124c2ef7aef85a31eda844aa967aa74d3422a7e2b2e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_max_3() {
    let value = Bitlist::<8>::from_iter([true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb9e73cb5c2e4ef66fa63540f8220301d31eea7edfccedb2b47b9bdf849ccee7");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_nil_0() {
    let value = Bitlist::<512>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7a0501f5957bdf9cb3a8ff4966f02265f968658b7a9c62642cba1165e86642f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_zero_1() {
    let value = Bitlist::<31>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7d934ef6667cff3afea0633d57baa9a82a7009f89b0f8c12f47150047098b396");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_max_4() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_lengthy_1() {
    let value = Bitlist::<3>::from_iter([false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d86ae2ca925345bf2412bde450ac175742d979c1ea7b961bd1efe10beb9500cf");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_lengthy_0() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, true, false, false, true, false,
        true, false, false, false, false, false, false, false, false, false, true, false, false,
        true, true, true, false, false, true, true, false, false, false, false, false, true, false,
        false, false, true, true, false, true, false, false, false, false, false, true, true,
        false, false, true, true, false, true, false, true, false, true, true, true, false, false,
        false, true, false, false, true, true, false, false, true, true, true, true, true, true,
        true, true, false, false, true, true, true, true, true, false, false, false, false, false,
        false, false, true, true, false, true, true, false, false, false, false, false, false,
        true, true, false, true, true, false, false, false, false, true, false, true, false, false,
        true, false, true, false, false, false, true, false, true, false, true, true, true, true,
        false, false, false, true, true, true, true, true, true, false, false, true, false, false,
        true, true, false, false, false, false, false, true, false, false, false, true, true, true,
        false, true, false, true, false, true, false, false, true, true, false, false, true, true,
        false, true, true, true, true, false, true, true, false, false, true, true, true, true,
        false, true, true, true, true, false, true, true, false, false, true, false, false, true,
        true, false, false, false, true, false, false, false, false, false, false, true, false,
        false, false, false, false, true, true, true, true, false, false, false, false, false,
        false, true, true, false, false, false, true, true, true, true, true, true, true, false,
        false, false, false, true, false, true, true, false, false, false, false, true, false,
        true, false, true, true, true, false, true, false, false, true, false, false, true, true,
        true, false, false, true, false, true, false, false, false, false, false, true, false,
        true, true, false, true, false, false, false, false, true, true, false, true, false, true,
        true, true, true, false, false, true, false, true, true, true, true, true, true, true,
        false, false, true, true, true, false, false, false, false, true, true, true, false, false,
        true, false, true, true, false, false, false, false, true, true, true, true, false, false,
        false, false, true, false, true, false, false, false, false, true, true, false, false,
        true, true, true, false, false, false, false, false, true, false, false, true, true, false,
        false, true, true, true, true, true, false, false, true, false, true, true, true, true,
        false, true, false, false, true, false, false, true, false, false, true, false, true,
        false, true, false, true, true, true, true, true, false, false, true, true, true, true,
        true, true, true, false, false, false, false, true, false, false, false, false, true, true,
        true, true, true, true, true, true, false, false, false, false, false, true, false, false,
        false, true, true, false, false, true, false, false, false, true, true, true, false, false,
        false, false, true, false, false, false, true, false, false, false, false, true, false,
        true, false, false, false, true, true, true, false, false, false, true, false, false,
        false, true, false, true, true, false, true, false, true, true, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, true, false, false,
        false, false, false, false, false, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("bc152fc83f6fefea40b3b3fdf626dc1af7eaea74e6bce7aba12a6602679004e1");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_max_0() {
    let value = Bitlist::<3>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_lengthy_2() {
    let value = Bitlist::<1>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_nil_0() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_random_1() {
    let value = Bitlist::<31>::from_iter([false, true, false, true, false, true, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("62681102fbb14f3973d9db3e302be35e5bbd79984aed6a85c532c63189afb38a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_zero_3() {
    let value = Bitlist::<16>::from_iter([false, false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7d360196d14b15261c9e5f576df8dc8b48d18d79b4198f167741052747704352");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_random_2() {
    let value = Bitlist::<4>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_random_2() {
    let value = Bitlist::<5>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_zero_1() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_lengthy_0() {
    let value = Bitlist::<5>::from_iter([false, false, true, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("bd50456d5ad175ae99a1612a53ca229124b65d3eaabd9ff9c7ab979a385cf6b3");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_zero_4() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("a44a029e04493b8d2fe7893391c2b3ceefec1603c585aad6203f2d14e07bfead");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_nil_4() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_max_0() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("bbf3224946b87b12d7c3c24d4887a1a1bdb6afd356e3fb40bfa7a42cd0a7d478");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_nil_3() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_lengthy_1() {
    let value = Bitlist::<8>::from_iter([false, false, true, false, true, true, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("5b6af4c3df02247b90fc3736e0a2ff746b5a7f7dc54e7edc66bbb0f68f1b7206");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_nil_2() {
    let value = Bitlist::<16>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_nil_1() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_max_1() {
    let value = Bitlist::<5>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_lengthy_0() {
    let value = Bitlist::<8>::from_iter([false, true, true, true, false, false, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("095847dd477b5ac2b2a5930d0633975f09e835630c2d4a832b6469e8c0d106d1");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_nil_2() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_max_1() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ca6250f3556974d64650a327c0551859f706d9778399caff8a6be920d88fb39f");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_random_4() {
    let value = Bitlist::<5>::from_iter([false, true, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("894e8a2ce460c6c6ba12d467634e6c34ce2a1b58d0c6dfe3d98b532898c58611");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_random_4() {
    let value = Bitlist::<4>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_random_0() {
    let value = Bitlist::<31>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_zero_2() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("b04cc2cb8ea6754f94c2e7403cf58e20c9023a98350c84282966e0bd6729d3ca");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_nil_1() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_random_3() {
    let value = Bitlist::<4>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_lengthy_1() {
    let value = Bitlist::<5>::from_iter([true, false, false, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7000b9bd26fb753d24a4ed870faee659894843b795377a89ade25b649246e773");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_random_3() {
    let value = Bitlist::<5>::from_iter([false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0e01f8d9a6720610a44a70c2c91bbe750ec6cd67892d92b1016394abfc382cf9");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_zero_0() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_max_0() {
    let value = Bitlist::<5>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_nil_0() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_nil_4() {
    let value = Bitlist::<16>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_nil_3() {
    let value = Bitlist::<16>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_max_4() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_zero_0() {
    let value = Bitlist::<4>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d647eb2598d33d7216256356596d29cecd31c1ba7a7ff25ccb5be4a453410b9d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_max_3() {
    let value = Bitlist::<2>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_nil_3() {
    let value = Bitlist::<513>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28ba1834a3a7b657460ce79fa3a1d909ab8828fd557659d4d0554a9bdbc0ec30");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_lengthy_3() {
    let value = Bitlist::<2>::from_iter([true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ff55c97976a840b4ced964ed49e3794594ba3f675238b5fd25d282b60f70a194");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_nil_4() {
    let value = Bitlist::<513>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28ba1834a3a7b657460ce79fa3a1d909ab8828fd557659d4d0554a9bdbc0ec30");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_lengthy_2() {
    let value = Bitlist::<513>::from_iter([
        true, false, false, false, false, true, false, false, false, true, false, true, true,
        false, true, true, false, true, true, false, false, true, false, true, true, true, true,
        true, false, false, false, false, true, false, false, true, true, true, true, true, true,
        false, false, false, false, true, true, true, false, true, false, true, true, false, false,
        true, true, false, true, true, false, false, true, true, false, true, true, false, false,
        false, false, false, true, true, true, false, true, false, true, true, false, false, true,
        false, false, true, true, false, true, false, true, true, false, false, true, false, false,
        true, true, true, false, false, false, false, false, false, false, false, true, false,
        false, false, true, true, true, true, false, false, true, true, false, false, true, false,
        false, false, true, true, true, true, true, true, true, true, false, true, false, false,
        false, true, true, false, true, true, false, true, true, false, true, true, true, false,
        false, true, true, false, false, true, true, false, true, true, true, true, true, true,
        false, false, true, true, true, true, true, false, true, true, true, true, false, false,
        true, false, true, true, true, true, true, true, false, true, false, false, true, false,
        false, false, true, false, true, false, true, false, false, true, false, true, true, false,
        false, false, true, false, true, false, false, true, true, true, true, false, true, false,
        false, true, false, true, true, false, false, true, true, true, true, false, true, false,
        false, true, true, false, true, true, true, false, false, false, true, true, false, false,
        false, false, false, false, true, false, true, true, true, true, true, true, false, false,
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        false, true, false, false, false, false, true, true, false, true, false, true, false, true,
        false, false, true, false, false, true, true, true, true, true, true, true, true, true,
        true, false, false, false, true, true, false, true, false, false, false, true, false, true,
        true, false, false, false, true, false, true, true, true, true, false, true, false, false,
        true, false, true, true, true, false, true, true, false, false, false, false, false, false,
        true, true, false, true, true, false, true, true, true, true, false, false, true, true,
        false, true, true, true, false, false, true, false, false, true, false, true, false, true,
        false, false, true, true, false, true, true, true, false, false, true, true, true, true,
        true, true, false, false, true, false, true, true, true, true, true, true, false, true,
        false, false, true, false, false, true, true, false, true, true, false, true, false, true,
        false, false, true, false, true, false, true, false, true, false, false, true, true, true,
        false, false, true, true, true, true, true, true, false, true, false, true, false, true,
        true, true, false, false, true, true, false, false, true, true, false, false, true, true,
        false, true, false, false, true, false, false, true, true, false, true, true, true, false,
        false, true, false, false, false, false, true, false, true, true, false, true, false,
        false, true, false, false, true, true, false, false, true, true, true, true, false, false,
        true, true, false, true, false, true, false, false, false, true, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("fcc1fb245d5eae1370c4cfaf51a23a68d24fc931eb75d8e3b337eadf1c94b4be");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_zero_2() {
    let value = Bitlist::<5>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d647eb2598d33d7216256356596d29cecd31c1ba7a7ff25ccb5be4a453410b9d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_lengthy_4() {
    let value = Bitlist::<2>::from_iter([true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ff55c97976a840b4ced964ed49e3794594ba3f675238b5fd25d282b60f70a194");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_zero_0() {
    let value = Bitlist::<8>::from_iter([false, false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7d360196d14b15261c9e5f576df8dc8b48d18d79b4198f167741052747704352");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_random_0() {
    let value = Bitlist::<8>::from_iter([true, true, true, true, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("385e8de0fb7865579bcaf9d0a9c86e4cca08a6911d1ce85530f96ce202a38d21");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_lengthy_4() {
    let value = Bitlist::<31>::from_iter([
        false, true, true, true, false, true, false, false, true, false, false, false, false,
        false, false, true, false, true, false, false, true, false, true, true, false, true, false,
        false, false, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("152b52ebbfc701c7a39758748e1f14b4361ae37dd480b6914aa725824cde97f2");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_random_1() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_random_1() {
    let value = Bitlist::<2>::from_iter([false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0e01f8d9a6720610a44a70c2c91bbe750ec6cd67892d92b1016394abfc382cf9");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_random_1() {
    let value = Bitlist::<16>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_lengthy_3() {
    let value = Bitlist::<31>::from_iter([
        false, false, true, true, true, true, false, true, true, true, true, false, false, false,
        true, false, false, true, true, true, false, false, false, false, false, true, true, true,
        true, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0faa1049c965bf5a37db3b457dcc3a2ee179ef704c42a29722641b2ec3bb3658");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_nil_2() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_max_0() {
    let value = Bitlist::<31>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ebe018d5287ea5be7d789946da9587c27f5dd82d8c120a594ae0e8ddd2e21802");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_lengthy_3() {
    let value = Bitlist::<513>::from_iter([
        false, false, true, false, true, true, true, true, false, false, true, false, true, true,
        true, false, true, false, true, true, true, true, true, false, false, true, true, false,
        true, true, true, true, true, false, false, true, true, true, false, true, false, true,
        false, true, true, true, true, false, true, false, true, false, false, true, false, false,
        true, false, false, true, true, true, true, false, false, false, false, true, false, true,
        false, false, true, true, true, false, false, false, false, true, true, false, true, true,
        true, true, false, false, false, false, false, false, false, true, true, false, false,
        true, false, false, true, true, true, false, false, false, false, true, true, false, true,
        false, true, true, false, false, false, true, false, true, false, true, true, false, false,
        true, false, true, false, true, true, true, true, true, true, true, false, false, true,
        false, false, false, true, false, true, false, false, false, true, true, false, true,
        false, false, false, false, true, false, false, false, false, true, false, true, false,
        false, false, false, false, true, false, true, true, true, true, false, false, false,
        false, true, true, false, false, false, true, true, true, true, true, false, true, false,
        true, true, false, false, false, true, false, false, false, false, true, true, false,
        false, true, true, false, true, false, true, false, true, false, true, true, false, false,
        false, false, true, true, true, true, true, true, true, false, false, true, true, false,
        false, true, true, false, true, true, false, false, true, true, true, true, false, false,
        true, true, true, false, false, true, true, false, true, false, false, false, true, true,
        true, true, true, false, true, false, true, true, true, false, true, true, false, false,
        false, true, false, true, false, true, false, true, true, true, true, true, true, false,
        false, false, true, true, true, true, false, true, true, false, false, false, true, false,
        true, false, false, true, false, true, false, false, true, true, false, true, true, true,
        false, true, true, true, false, false, false, true, true, false, false, true, false, true,
        true, true, true, true, true, false, true, true, false, false, true, true, false, false,
        false, true, false, false, false, false, false, true, false, false, false, true, true,
        true, true, false, true, true, false, false, false, true, false, true, true, false, true,
        false, true, false, true, false, false, true, false, false, true, true, true, true, false,
        true, false, true, false, false, true, false, false, true, false, true, true, true, true,
        true, false, true, true, false, true, true, true, false, true, true, false, true, false,
        true, true, true, true, false, false, false, false, true, true, true, true, false, true,
        false, false, false, false, false, true, false, true, true, true, true, false, true, true,
        false, false, true, true, false, false, false, true, true, true, true, false, false, true,
        true, true, true, true, true, true, false, true, false, true, true, true, false, false,
        true, true, true, false, true, false, false, false, false, false, true, true, true, true,
        false, false, true, true, false, false, true, true, false, true, false, false, true, true,
        false, true, false, true, false, false, false, false, false, false, false, true, false,
        true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("eb5acc36387e3d3e44187bd6c086e4409fab204daa33ad40a99226dd2c487d8e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_zero_3() {
    let value = Bitlist::<5>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_zero_1() {
    let value = Bitlist::<8>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_nil_2() {
    let value = Bitlist::<513>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28ba1834a3a7b657460ce79fa3a1d909ab8828fd557659d4d0554a9bdbc0ec30");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_zero_4() {
    let value = Bitlist::<5>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d647eb2598d33d7216256356596d29cecd31c1ba7a7ff25ccb5be4a453410b9d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_lengthy_4() {
    let value = Bitlist::<513>::from_iter([
        true, false, true, true, true, true, false, false, true, false, true, true, true, false,
        true, false, true, false, true, true, false, true, true, false, true, true, true, true,
        false, true, true, true, false, false, false, true, true, true, true, true, true, false,
        true, true, true, true, false, false, false, false, true, false, false, true, true, true,
        false, true, false, false, false, false, false, true, false, true, true, true, true, true,
        false, true, false, false, true, true, true, true, true, false, true, true, false, false,
        true, true, true, true, true, false, true, true, true, false, true, false, true, true,
        true, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, true, false, false, true, true, true, true, false, true, false, true, true, false,
        true, false, false, false, true, false, true, true, true, false, false, false, true, true,
        false, false, true, true, true, true, true, false, false, true, true, false, true, false,
        true, false, false, false, true, false, true, true, false, true, true, true, true, false,
        true, true, true, false, true, false, false, true, false, true, true, false, false, false,
        false, false, false, true, true, true, true, false, true, true, false, true, false, true,
        true, false, true, true, true, false, true, true, false, false, true, false, false, false,
        false, false, false, true, false, false, false, true, false, false, true, false, true,
        false, true, false, false, false, true, false, false, false, false, true, false, true,
        false, false, false, false, true, true, false, true, true, false, false, false, true,
        false, true, false, false, false, false, true, true, true, false, true, true, true, false,
        true, false, true, false, true, false, true, false, false, true, true, true, false, true,
        false, true, false, true, false, true, true, false, false, true, false, true, false, false,
        false, false, false, true, true, false, true, false, false, false, false, true, true, true,
        true, true, false, true, false, true, true, false, false, false, true, true, false, true,
        true, false, false, false, false, false, false, false, true, false, false, false, true,
        false, false, false, true, true, false, true, false, false, true, true, true, true, false,
        false, true, false, true, false, true, true, true, false, true, false, true, false, false,
        false, true, true, false, true, true, false, true, true, true, false, true, false, false,
        true, false, true, false, false, false, true, false, true, false, false, true, false,
        false, false, true, true, false, false, true, true, true, true, true, false, true, true,
        false, true, true, false, true, false, false, true, false, false, false, true, true, true,
        false, true, false, true, false, true, false, false, true, true, false, false, false,
        false, true, true, false, false, false, true, true, true, true, true, true, true, false,
        true, true, false, true, false, false, false, false, true, true, false, false, false, true,
        false, true, true, false, true, false, false, true, true, false, true, false, false, false,
        false, true, false, false, true, true, false, false, false, false, true, true, true, true,
        true, true, true, true, true, false, true, true, true, true, true, false, false, false,
        false, false, false, true, true, false, false, true, true, true, true, false, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4edc0e0f8cb3511f8e89e5a9d73fdd50270e49aa8bfa62ffe8c8e99c161e76ba");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_lengthy_2() {
    let value = Bitlist::<2>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_max_2() {
    let value = Bitlist::<2>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_zero_1() {
    let value = Bitlist::<4>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_nil_4() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_nil_3() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_max_1() {
    let value = Bitlist::<31>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_random_0() {
    let value =
        Bitlist::<16>::from_iter([false, false, true, false, true, true, false, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("eec57ef94d128f67c545a95b84f97501237ed672f583769110409b2df50bce84");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_lengthy_2() {
    let value = Bitlist::<31>::from_iter([
        true, false, true, false, true, false, false, true, false, true, false, false, true, false,
        false, true, true, true, false, false, true, false, true, false, false, false, true, true,
        false, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("57bdf36005bb9113c2b89db95c10946d97609b3173d4397a1a74755d0c6490f8");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_random_1() {
    let value = Bitlist::<8>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_random_0() {
    let value = Bitlist::<3>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_random_0() {
    let value = Bitlist::<2>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_max_3() {
    let value = Bitlist::<4>::from_iter([true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4b07c3799db025f3aa92ced1e8545367a2b6e44960f479d3f9d62b61812892d5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_random_3() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_nil_1() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_max_4() {
    let value = Bitlist::<4>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_random_4() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_nil_2() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_max_2() {
    let value = Bitlist::<513>::from_iter([true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("848557322ff06141bbb7ac657b15c24e6300986a5ff8ce878ef4b198c0bd51b0");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_lengthy_3() {
    let value = Bitlist::<16>::from_iter([
        true, false, false, true, false, false, false, true, true, true, false, false, false,
        false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("50fea858f788bbc2f17f809e05682bf855493a7b8c594f4c2342b469ac7bdb53");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_lengthy_4() {
    let value = Bitlist::<4>::from_iter([false, true, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("894e8a2ce460c6c6ba12d467634e6c34ce2a1b58d0c6dfe3d98b532898c58611");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_lengthy_3() {
    let value = Bitlist::<4>::from_iter([true, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f9c5ada16029ed1580188989686f19e749c006b2eac37d3ef087b824b31ba997");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_lengthy_4() {
    let value = Bitlist::<16>::from_iter([
        false, true, true, true, true, false, false, false, true, false, false, false, true, false,
        true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("983039dcf7ee961e2a2c1b1d0b57ad04491b8674c0f9f6dc326244e48dacd851");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_max_2() {
    let value = Bitlist::<4>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_random_2() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_nil_0() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_max_4() {
    let value = Bitlist::<513>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("e05d10ac23b945573dca5263c13a7eaf50854397bf48f920175a10509bf65ecf");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_lengthy_2() {
    let value = Bitlist::<4>::from_iter([true, true, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("374bd7c88680671ad4be6e1b576db80646d992d893a5eeb1d1d0f403c3331b32");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_max_3() {
    let value = Bitlist::<513>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("a575735c9960d438c8bdd59d05fedefce22f8e5b77b09efb5b4e9942b847468e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_lengthy_2() {
    let value = Bitlist::<16>::from_iter([
        true, false, false, false, false, true, false, true, true, true, true, false, true, false,
        false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("fc0027195d4d241e8d3111d41d749a46f62e2d0e78aa503b856774abe6b7e6c3");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_nil_3() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_nil_4() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_lengthy_3() {
    let value = Bitlist::<3>::from_iter([false, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d3156136ef0ebd0cb8945f7c18cfe8ad539d08d8703744bc11371e49e6a4d9ad");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_lengthy_2() {
    let value = Bitlist::<512>::from_iter([
        true, false, false, false, false, true, false, true, false, true, true, false, true, true,
        false, false, false, false, false, false, false, true, true, true, true, false, true, true,
        true, false, false, false, false, true, false, true, true, true, true, true, true, true,
        false, false, true, false, false, false, false, true, false, true, true, false, false,
        true, true, false, true, true, true, true, false, false, false, true, false, true, false,
        true, true, true, false, true, true, true, false, true, true, false, false, false, true,
        false, true, true, false, true, false, false, false, false, true, true, true, true, true,
        true, true, true, false, true, true, true, false, false, true, true, true, false, false,
        true, true, false, true, false, true, false, false, false, false, false, false, false,
        false, false, true, false, true, true, false, true, false, true, true, false, false, true,
        false, true, false, true, true, false, true, false, false, false, true, false, false,
        false, true, false, false, true, false, true, true, true, false, false, false, false, true,
        false, true, false, true, false, true, false, false, true, false, false, true, false, true,
        false, true, true, false, true, true, true, false, false, false, false, true, false, true,
        true, true, false, false, true, false, false, false, false, true, true, false, true, false,
        false, true, false, false, true, false, true, true, false, false, false, false, true,
        false, true, true, true, false, false, true, false, false, false, true, true, true, false,
        true, false, false, false, false, true, true, false, false, true, false, false, false,
        false, false, true, true, true, true, false, false, true, true, false, true, true, true,
        true, false, true, false, false, false, false, true, true, true, true, true, false, true,
        false, true, true, false, false, false, true, true, true, true, true, false, false, true,
        true, true, false, true, false, true, false, false, false, false, false, false, true,
        false, false, false, true, false, false, false, true, false, true, false, true, true,
        false, false, true, false, true, true, true, true, true, true, false, true, false, false,
        true, true, true, false, false, false, true, false, true, false, true, true, true, false,
        true, true, true, true, true, false, true, true, true, false, false, false, false, false,
        true, false, false, false, false, false, false, true, false, false, false, true, false,
        false, false, false, true, true, true, false, true, true, true, true, true, false, true,
        true, true, false, true, false, true, true, false, false, true, false, true, false, true,
        false, true, false, false, false, true, false, false, true, false, false, false, false,
        true, true, false, false, false, true, false, false, false, true, false, true, false,
        false, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, false, true, false, false, true, true, true, true, false, false, true, false, true,
        true, false, false, true, true, false, true, false, false, false, true, false, true, false,
        false, true, false, false, false, false, false, true, false, true, true, true, false, true,
        false, false, true, true, false, true, false, true, true, true, false, false, true, false,
        false, true, true, true, true, false, false, false, false, true, false, false, true, true,
        true, false, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("04d0ff41239e5365cafa09c58dedb823eb13cb4912afea9fc26a658b955a4594");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_max_2() {
    let value = Bitlist::<3>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_lengthy_0() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_lengthy_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_lengthy_4() {
    let value = Bitlist::<3>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("251d8bd955c85219bb8f6de682810b4aafe3e0c3d3c624020fb39f81dbb85910");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_max_1() {
    let value = Bitlist::<8>::from_iter([true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("017d2fa0f6934ed2354e4cdb7a2230ccf8f31fe758c7a47442e37fdea1d68bfe");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_nil_2() {
    let value = Bitlist::<512>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7a0501f5957bdf9cb3a8ff4966f02265f968658b7a9c62642cba1165e86642f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_zero_3() {
    let value = Bitlist::<31>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("04997ec49450b710d4d92e2e6e92c47b193b0ec6f841d7d692bf0f410cbc7269");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_zero_4() {
    let value = Bitlist::<31>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_random_2() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, false, true, true, false, false, false, false, true, false, true, false,
        true, true, true, false, false, false, true, true, true, true, false, false, true, false,
        false, true, true, false, false, true, false, true, false, true, true, true, false, false,
        true, true, true, true, false, true, false, false, false, false, false, false, false,
        false, false, true, false, false, false, true, false, false, true, true, false, true, true,
        false, false, false, false, false, false, false, false, false, true, true, false, true,
        true, true, true, true, false, true, true, true, true, true, true, true, true, false, true,
        true, false, true, true, false, true, false, true, false, false, false, true, false, true,
        true, true, false, true, false, false, true, true, true, false, true, true, true, false,
        false, false, true, true, true, true, false, false, true, false, true, false, false, true,
        true, true, false, false, false, true, false, true, true, false, false, true, true, false,
        false, true, false, true, false, true, false, false, true, true, true, false, true, true,
        true, false, true, false, false, true, false, true, true, false, true, false, false, true,
        true, true, true, false, true, false, true, false, false, false, true, false, false, true,
        false, false, true, false, true, true, false, true, true, true, true, true, false, false,
        false, true, true, true, false, false, true, true, true, true, false, false, true, false,
        true, true, true, true, true, false, false, false, true, true, false, true, true, true,
        true, true, false, false, true, true, false, false, true, false, true, true, true, true,
        true, false, true, false, false, true, true, false, false, true, true, true, true, true,
        false, true, false, true, true, true, false, false, true, true, true, true, true, true,
        false, true, true, false, false, false, false, false, true, false, true, false, false,
        true, true, true, true, false, true, false, false, true, false, true, false, true, true,
        true, false, true, true, true, true, true, true, true, false, true, true, true, true, true,
        false, true, true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0885e8d339f7016801875ef256eb180be417810e6151703137877f68926952f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_random_2() {
    let value = Bitlist::<513>::from_iter([
        false, true, false, true, true, true, false, true, true, true, true, true, false, false,
        true, false, true, false, true, false, false, true, true, true, true, true, false, false,
        false, true, true, false, true, false, false, false, false, true, true, true, false, true,
        false, false, true, true, true, false, true, false, true, true, true, true, true, true,
        true, true, false, true, false, true, true, true, false, true, false, true, true, false,
        false, true, false, false, true, false, true, true, true, false, true, true, false, false,
        true, true, false, true, false, false, true, true, true, true, false, false, false, false,
        false, true, true, false, false, true, true, false, true, true, false, true, true, false,
        true, true, true, false, true, true, true, false, false, true, false, true, false, true,
        false, true, true, false, true, true, true, false, true, false, true, true, true, true,
        true, true, false, false, false, true, true, true, false, true, false, true, false, true,
        false, false, false, true, false, true, true, false, false, false, true, true, false, true,
        false, true, true, false, true, true, false, true, true, false, false, true, true, false,
        true, false, false, false, true, true, false, false, false, false, true, false, false,
        false, true, false, false, true, false, false, true, true, false, false, false, true, true,
        true, true, true, true, false, true, false, false, false, true, true, false, true, false,
        false, false, true, true, true, false, false, false, true, true, true, true, true, false,
        true, true, true, true, false, false, false, true, true, false, false, false, true, false,
        true, false, false, true, false, true, false, false, true, false, false, true, false, true,
        false, true, false, true, true, false, true, false, true, false, false, true, false, false,
        true, false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, true, false, false, true, true, false, false, true, false, true, true, true, false,
        true, true, true, true, true, true, false, false, true, true, true, true, false, true,
        true, true, false, true, true, true, false, false, false, true, false, false, true, true,
        false, true, true, false, false, false, false, false, false, false, true, false, true,
        false, true, true, false, false, true, false, false, false, false, false, false, false,
        false, false, true, true, false, true, false, true, false, false, true, true, true, false,
        false, true, false, true, false, false, false, true, true, true, false, false, false,
        false, false, false, false, false, false, true, false, true, true, true, true, false, true,
        false, false, false, true, true, false, false, true, true, false, false, false, false,
        true, false, true, true, true, true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("bb76eb1bab23fc2865c84717251e4305221771924259082d793d3bbaa6444ba1");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_zero_3() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("54bfe2c647e52bf3897cff9675165d53f277e1f7dbd7c620f630a2deb02ce0c8");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_zero_4() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_max_3() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_max_4() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_zero_3() {
    let value = Bitlist::<3>::from_iter([false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d86ae2ca925345bf2412bde450ac175742d979c1ea7b961bd1efe10beb9500cf");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_zero_4() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("9b7d4ffa3720c8ea2c66e59f1890a83c86ef2b4442a5ebe6d757fb4cbe0b3231");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_max_0() {
    let value = Bitlist::<16>::from_iter([true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("017d2fa0f6934ed2354e4cdb7a2230ccf8f31fe758c7a47442e37fdea1d68bfe");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_zero_1() {
    let value = Bitlist::<2>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_nil_3() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_nil_4() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_zero_1() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4d8dddd9769fcea91305afd9f96b9b187ad7dbd994a67cea4eeb7e2c0348c292");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_nil_4() {
    let value = Bitlist::<512>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7a0501f5957bdf9cb3a8ff4966f02265f968658b7a9c62642cba1165e86642f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_nil_3() {
    let value = Bitlist::<512>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7a0501f5957bdf9cb3a8ff4966f02265f968658b7a9c62642cba1165e86642f5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_max_0() {
    let value = Bitlist::<8>::from_iter([true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("017d2fa0f6934ed2354e4cdb7a2230ccf8f31fe758c7a47442e37fdea1d68bfe");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_zero_2() {
    let value = Bitlist::<31>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("967293ee9d7ba679c3ef076bef139e2ceb96d45d19a624cc59bb5a3c1649ce38");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_lengthy_3() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, false, false, false, false, false, true, false, true, true, false, false,
        true, false, true, false, false, false, false, false, true, true, true, false, false,
        false, false, true, true, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, true, true, true, false, true, false, true, false,
        true, true, false, true, false, false, false, false, false, false, true, true, false,
        false, false, false, true, false, false, true, true, true, false, false, true, true, true,
        true, true, false, false, false, false, true, true, true, true, true, true, false, false,
        false, false, true, false, true, true, true, true, true, false, false, true, true, true,
        true, false, true, true, false, true, false, false, false, true, true, true, false, true,
        false, true, true, true, false, true, true, false, true, false, true, false, false, false,
        true, true, true, true, false, false, true, true, true, true, true, false, true, true,
        true, true, true, true, false, true, true, true, false, false, true, true, false, true,
        false, true, true, true, true, false, true, false, true, true, true, false, true, false,
        false, true, true, false, true, true, false, true, true, true, false, false, true, false,
        false, true, true, false, true, false, false, false, true, false, true, false, false,
        false, true, false, true, true, true, false, true, true, false, true, true, false, false,
        false, true, false, false, true, false, false, false, true, false, true, false, false,
        false, false, true, true, true, true, true, true, false, false, true, false, false, true,
        true, false, false, true, false, true, false, true, false, false, true, true, true, true,
        true, false, true, false, false, false, true, true, false, true, false, false, false,
        false, true, true, true, false, true, false, true, false, true, true, true, true, false,
        true, true, false, true, false, false, true, true, true, true, false, true, true, false,
        false, false, true, false, true, true, false, false, true, false, true, false, false, true,
        false, true, false, true, false, true, true, true, false, false, false, false, true, true,
        false, true, false, true, true, true, true, false, true, false, false, true, true, true,
        false, false, false, false, true, true, true, false, true, false, false, true, false, true,
        true, false, true, false, false, true, true, false, true, true, true, false, true, false,
        false, false, false, false, true, true, true, true, false, false, true, true, true, true,
        true, false, true, false, false, true, true, true, true, true, false, true, true, true,
        true, false, true, false, false, true, true, false, false, false, false, false, true, true,
        true, false, true, false, false, false, true, true, false, false, false, false, false,
        false, true, false, true, false, false, false, true, true, true, true, true, false, true,
        true, false, false, true, false, false, false, false, false, false, false, false, true,
        false, false, true, true, false, false, true, true, false, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true, false, true, false, true,
        false, true, true, true, false, true, true, true, true, true, true, true, false, true,
        true, true, false, false, false, false, true, true, true, false, false, true, true, false,
        true, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("57d984dd8dc742665160586d43e684d59f48ea2fbf7ff6fc6742cdcf050bea09");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_lengthy_1() {
    let value = Bitlist::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_lengthy_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_max_3() {
    let value = Bitlist::<3>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("56d8a66fbae0300efba7ec2c531973aaae22e7a2ed6ded081b5b32d07a32780a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_max_4() {
    let value = Bitlist::<3>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_lengthy_4() {
    let value = Bitlist::<512>::from_iter([
        false, true, false, false, true, false, false, false, false, true, false, false, true,
        true, false, false, true, false, true, false, true, true, false, true, true, true, true,
        false, false, true, true, true, false, false, true, false, true, true, true, false, false,
        false, true, false, true, true, false, false, false, false, false, true, false, true, true,
        true, false, true, true, false, false, true, true, false, false, false, true, false, false,
        false, false, false, true, true, true, true, true, true, false, true, false, false, false,
        false, false, true, false, false, true, false, false, false, false, false, false, false,
        true, false, false, true, true, true, true, true, false, true, true, false, false, true,
        false, true, false, true, true, false, false, true, true, false, false, true, false, false,
        false, false, true, true, true, true, true, false, true, true, false, true, true, true,
        true, true, false, false, true, false, true, true, true, false, true, true, true, false,
        false, false, false, true, false, false, false, false, false, false, false, false, false,
        false, true, true, false, true, true, true, false, false, true, true, false, false, true,
        false, false, true, false, false, true, false, true, true, false, true, false, false,
        false, true, true, false, true, false, false, true, true, true, true, false, false, false,
        true, false, true, true, false, false, false, true, false, false, true, false, true, true,
        false, false, false, true, true, false, false, true, true, true, false, true, false, false,
        false, true, true, false, false, true, true, true, false, true, true, true, true, true,
        true, true, true, false, false, true, false, true, false, true, false, false, false, false,
        true, true, true, false, false, true, true, false, true, false, false, true, false, true,
        true, false, true, false, false, true, true, false, true, false, false, false, true, true,
        false, true, false, false, false, false, true, false, false, true, false, false, true,
        true, false, false, false, false, false, true, true, false, false, true, true, true, true,
        true, true, false, false, true, false, false, false, false, false, true, false, true, true,
        true, true, true, false, false, false, true, false, false, true, true, true, false, true,
        false, true, true, false, true, true, true, false, true, false, false, true, false, false,
        true, true, true, true, false, false, true, false, true, false, true, true, true, true,
        true, true, true, false, true, false, false, false, false, false, false, false, false,
        true, true, true, true, false, false, false, true, true, false, false, false, false, true,
        true, true, true, false, true, false, true, false, false, false, false, true, true, false,
        true, true, false, false, false, false, false, false, false, false, true, false, true,
        false, false, true, false, false, false, true, true, false, true, true, false, true, true,
        true, false, true, true, true, true, true, true, true, false, true, false, true, true,
        false, false, true, false, false, true, false, true, true, true, false, false, true, false,
        true, false, false, true, false, true, true, false, false, false, false, true, false, true,
        false, true, false, false, false, false, false, true, false, false, false, true, true,
        false, true, true, false, true, true, true, false, true, true, true, false, true, true,
        true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("28933deb812002abaf34c610f6b2f77cb8acbc617d5a8f8a320ca4813c29fea2");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_lengthy_2() {
    let value = Bitlist::<3>::from_iter([true, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("caea92341df83aa8d4225099f16e86cbf457ec7ea97ccddb4ba5560062eee695");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_zero_0() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("b9622f5ac7a4f2982e31494019e6fc83a8510ba1313084df18fe74cfd63fff28");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_nil_2() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_2_zero_0() {
    let value = Bitlist::<2>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_2_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1205f4789155711e2542dba1a64d226626fe3eb43baa854752d0b59077e010fc");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_random_4() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, true, false, true, true, false, false, false, true, true, false,
        false, true, false, true, false, false, true, true, true, false, true, false, true, false,
        false, true, false, true, false, false, false, false, true, false, false, false, true,
        true, true, false, true, false, true, true, true, false, false, true, false, false, true,
        true, true, false, true, true, true, true, false, false, false, true, true, false, true,
        true, true, false, true, true, false, false, false, false, false, true, true, false, false,
        false, true, false, true, true, true, false, true, false, false, false, true, false, false,
        false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ba02d7073304a825d35943f503cb081434b0b49713afdff5b5a6ab1f46d14171");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_random_4() {
    let value = Bitlist::<512>::from_iter([
        false, false, false, false, false, true, true, false, false, true, true, true, false, true,
        false, false, true, false, false, false, false, false, false, false, true, true, true,
        false, false, false, true, false, false, true, false, true, false, true, true, true, true,
        true, false, true, false, false, true, true, false, true, true, true, false, false, false,
        false, false, true, false, true, true, true, true, true, false, false, false, true, true,
        true, true, false, false, false, false, false, false, false, true, true, false, true,
        false, false, false, true, false, false, true, false, false, true, false, false, true,
        false, true, false, false, true, false, true, false, true, false, false, false, false,
        false, false, true, true, false, true, true, false, true, true, false, true, true, false,
        true, false, false, true, false, true, true, false, false, false, false, true, false, true,
        true, false, true, false, true, false, true, true, false, true, false, false, true, true,
        true, false, false, false, false, false, false, false, false, true, false, true, false,
        true, true, true, true, true, false, false, false, true, false, false, true, false, true,
        true, true, false, true, true, false, true, false, true, true, false, true, true, false,
        true, false, false, true, false, false, true, false, true, false, true, false, false,
        false, false, false, false, false, true, false, true, false, false, false, true, true,
        true, false, false, true, false, true, true, false, true, true, false, true, true, false,
        false, true, false, false, true, false, true, true, true, true, false, false, true, false,
        false, false, true, true, false, true, true, false, true, true, true, true, false, true,
        false, true, false, false, false, true, false, true, false, true, true, false, false,
        false, true, false, false, true, true, true, true, true, false, true, false, false, true,
        true, true, true, false, false, true, true, false, true, true, false, false, true, false,
        false, true, false, false, true, true, false, true, true, true, true, false, false, false,
        true, false, false, true, true, true, false, false, true, true, false, true, false, true,
        true, true, true, true, true, true, false, false, true, true, false, true, false, false,
        false, true, true, false, true, false, false, false, true, false, true, false, true, false,
        true, true, true, true, false, true, false, false, false, false, false, true, false, false,
        false, true, false, false, false, true, true, false, true, false, true, true, true, true,
        false, true, true, false, false, false, true, false, true, false, true, true, false, false,
        true, false, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("813ccb937403bbd02d4ce9cd7e101c3bf3214ed4a1d8c11199288fbcdca45860");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_max_1() {
    let value = Bitlist::<16>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("dc8212e2404720c98554dfddc81733f88cbbe307a1d4ca5eae4b88e55e382392");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_zero_2() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_random_3() {
    let value = Bitlist::<512>::from_iter([
        false, true, false, false, true, true, true, false, false, false, false, true, true, true,
        false, true, false, true, true, false, false, true, true, true, true, true, true, true,
        false, true, false, true, true, false, false, false, true, false, false, false, false,
        false, true, true, true, false, false, false, false, true, true, true, true, true, false,
        true, true, true, false, false, false, true, true, true, false, true, true, false, false,
        true, false, true, true, true, false, false, false, true, true, false, false, false, true,
        true, true, false, true, false, true, true, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, false, true, false, false, true, true,
        false, false, true, false, true, false, true, false, true, false, true, true, true, false,
        true, true, true, true, false, false, true, false, true, true, true, false, true, true,
        false, false, false, true, true, true, false, false, true, true, true, true, false, false,
        false, true, false, false, false, true, false, true, false, false, true, false, false,
        false, true, true, false, true, true, false, true, false, true, false, true, true, true,
        false, false, false, false, true, true, false, false, true, true, false, true, true, false,
        false, false, false, true, true, false, true, true, false, true, false, true, false, true,
        false, true, true, true, true, true, false, true, false, false, false, true, false, true,
        true, false, false, false, false, true, true, true, false, true, true, true, false, false,
        false, false, false, false, false, true, false, true, false, true, false, false, false,
        true, true, true, false, true, true, false, true, false, false, true, true, false, false,
        true, true, false, false, true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0c24b4aa44483bc91415618c8d23fa1ec87cbbf57dd1747ac001513f3ddeea8c");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_random_3() {
    let value = Bitlist::<513>::from_iter([
        false, true, false, true, true, false, true, true, false, true, true, false, true, false,
        true, false, false, true, false, false, true, false, true, true, false, true, false, true,
        true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("32370b95731ef776a513ca5ef154a83ba935260f2f4bdbba23c21b33e12f7b62");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_513_zero_2() {
    let value = Bitlist::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_513_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("38ab4aeb5726a3fb78af0101063f2586905c3e8466206bfc8777f44ed9e6ef20");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_max_2() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_nil_2() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_nil_1() {
    let value = Bitlist::<16>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_max_2() {
    let value = Bitlist::<5>::from_iter([true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4b07c3799db025f3aa92ced1e8545367a2b6e44960f479d3f9d62b61812892d5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_nil_4() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_lengthy_4() {
    let value = Bitlist::<5>::from_iter([false, false, false, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("88b744d02033bbb6a4ebc2dc3f31c4910681596c7bcb9349d9483a33e45899c7");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_nil_3() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_random_2() {
    let value = Bitlist::<31>::from_iter([
        false, true, false, true, false, true, true, false, true, false, true, true, false, false,
        false, true, true, false, true, false, true, true, true, false, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1037ee25750a944efe9b3dc796628f6468a9f242bd791013c439ca785c134482");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_zero_0() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("69713c9ac33bde909bd8763512e69a7f523d544adcfb8c892e24bc8f6341ea16");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_random_1() {
    let value = Bitlist::<4>::from_iter([true, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f9c5ada16029ed1580188989686f19e749c006b2eac37d3ef087b824b31ba997");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_lengthy_3() {
    let value = Bitlist::<5>::from_iter([false, true, false, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("5d40a4acd8c5f8b674c29a7b7814a546fade497a96d0e7bb51c3a4951fb1fa7e");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_zero_2() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_random_1() {
    let value = Bitlist::<5>::from_iter([false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d86ae2ca925345bf2412bde450ac175742d979c1ea7b961bd1efe10beb9500cf");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_nil_0() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_max_3() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("8cbf50b584a296a316a71c486b4d4e1fd94edae9bf75f1aff71b8f609dc8352c");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_lengthy_2() {
    let value = Bitlist::<8>::from_iter([true, true, false, false, true, false, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("eeb7a380c63f2182c38a556ee4170cb9fd06b86b5014181e7a01ce0097627cf0");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_max_4() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1bb7ab569c8b46d1e40884241195c1369ea760bf957583d3a78a4315c0e2f495");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_max_4() {
    let value = Bitlist::<5>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("c397e31994d6b872c69af43765ab16a1cef673be726a820dacd2637bea2f5fbb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_max_3() {
    let value = Bitlist::<5>::from_iter([true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("4b07c3799db025f3aa92ced1e8545367a2b6e44960f479d3f9d62b61812892d5");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_nil_0() {
    let value = Bitlist::<16>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_nil_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_nil_4() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_nil_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_nil_3() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_nil_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_lengthy_4() {
    let value = Bitlist::<8>::from_iter([true, true, false, false, false, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_lengthy_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("6d1fd4c1b192e8aeb35074214855c593805c2ed1ff79f7aa7c6128814fa41bf3");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_lengthy_3() {
    let value = Bitlist::<8>::from_iter([true, true, false, false, false, true, true, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_lengthy_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("b8148b13b48faa79622d9a6975e7abdf85dd4639a25e53412eb0aa5c34386019");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_512_max_2() {
    let value = Bitlist::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_512_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("08e61443f630601ca65f47622a47ef029baad7a757f3f1d10de0098c9add4589");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_8_nil_1() {
    let value = Bitlist::<8>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_8_nil_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_random_3() {
    let value = Bitlist::<31>::from_iter([true, true, false, true, true, false, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("5940967aaa293730d0e7876047dfceb9cf5512fafb5d4be3d05c776902163786");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_16_zero_1() {
    let value = Bitlist::<16>::from_iter([
        false, false, false, false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_16_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("7b460f51b362b95b384743dda74f56fbcd35f4d8e7ebda7206632e60c91e663d");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_3_nil_2() {
    let value = Bitlist::<3>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_3_nil_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_4_random_0() {
    let value = Bitlist::<4>::from_iter([true, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_4_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cf8ca64c265b9b6234fb7573a200745204fd04fecf680f1157f27367ee8f4aa2");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_zero_3() {
    let value = Bitlist::<1>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_random_0() {
    let value = Bitlist::<5>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_5_lengthy_2() {
    let value = Bitlist::<5>::from_iter([false, true, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_5_lengthy_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("d13061c7b549c86b29ad2389cbe4fb2fd05bbdf3170da634e67f77ab981b82cb");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_31_random_4() {
    let value = Bitlist::<31>::from_iter([]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_31_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitlist_bitlist_1_zero_4() {
    let value = Bitlist::<1>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/valid/bitlist_1_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitlist<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("cb592844121d926f1ca3ad4e1d6fb9d8e260ed6e3216361f7732e975a0e8bbf6");
    assert_eq!(root, expected_root);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_1_but_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_1_but_2/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_8_but_9() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_8_but_9/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<8>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_3_but_4() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_3_but_4/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<3>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_512_but_513() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_512_but_513/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<512>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_4_but_5() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_4_but_5/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<4>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_32_but_64() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_32_but_64/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<32>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_no_delimiter_empty() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_no_delimiter_empty/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<256>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_2_but_3() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_2_but_3/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<2>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_no_delimiter_zero_byte() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_no_delimiter_zero_byte/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<256>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_5_but_6() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_5_but_6/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<5>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_1_but_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_1_but_8/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_1_but_9() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_1_but_9/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_32_but_33() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_32_but_33/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<32>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitlist_bitlist_no_delimiter_zeroes() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/bitlist/invalid/bitlist_no_delimiter_zeroes/serialized.ssz_snappy",
    );

    deserialize::<Bitlist<256>>(&encoding);
}
