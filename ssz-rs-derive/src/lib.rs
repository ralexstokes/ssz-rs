//! `SimpleSerialize` provides a macro to derive SSZ containers and union types from
//! native Rust structs and enums.
//! Refer to the `examples` in the `ssz_rs` crate for a better idea on how to use this derive macro.
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Field, Fields, Generics,
    Ident, Meta, NestedMeta, PathArguments,
};

// NOTE: copied here from `ssz_rs` crate as it is unlikely to change
// and can keep it out of the crate's public interface.
const BYTES_PER_CHUNK: usize = 32;

const SSZ_HELPER_ATTRIBUTE: &str = "ssz";

fn derive_serialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                // "regular" struct with 1+ fields
                Fields::Named(ref fields) => &fields.named,
                // "tuple" struct
                // only support the case with one unnamed field, to support "newtype" pattern
                Fields::Unnamed(..) => {
                    return quote! {
                        fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, ssz_rs::SerializeError> {
                                self.0.serialize(buffer)
                        }
                    }
                }
                _ => unimplemented!(
                    "this type of struct is currently not supported by this derive macro"
                ),
            };
            let serialization_by_field = fields.iter().map(|f| match &f.ident {
                Some(field_name) => quote_spanned! { f.span() =>
                    serializer.with_element(&self.#field_name)?;
                },
                None => panic!("should have already returned an impl"),
            });

            quote! {
                fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, ssz_rs::SerializeError> {
                    let mut serializer = ssz_rs::__internal::Serializer::default();

                    #(#serialization_by_field)*

                    serializer.serialize(buffer)
                }
            }
        }
        Data::Enum(ref data) => {
            let serialization_by_variant = data.variants.iter().enumerate().map(|(i, variant)| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unnamed(..) => {
                        quote_spanned! { variant.span() =>
                            Self::#variant_name(value) => {
                                let selector = #i as u8;
                                let selector_bytes = selector.serialize(buffer)?;
                                let value_bytes  = value.serialize(buffer)?;
                                Ok(selector_bytes + value_bytes)
                            }
                        }
                    }
                    Fields::Unit => {
                        quote_spanned! { variant.span() =>
                            Self::None => {
                                0u8.serialize(buffer)
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            });

            quote! {
                fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, ssz_rs::SerializeError> {
                    match self {
                        #(#serialization_by_variant)*
                    }
                }
            }
        }
        Data::Union(..) => unreachable!("data was already validated to exclude union types"),
    }
}

fn derive_deserialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                // "regular" struct with 1+ fields
                Fields::Named(ref fields) => &fields.named,
                // "tuple" struct
                // only support the case with one unnamed field, to support "newtype" pattern
                Fields::Unnamed(ref fields) => {
                    // SAFETY: index is safe because Punctuated always has a first element; qed
                    let f = &fields.unnamed[0];
                    let field_type = &f.ty;
                    return quote! {
                        fn deserialize(encoding: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
                            let result = <#field_type>::deserialize(&encoding)?;
                            Ok(Self(result))
                        }
                    }
                }
                _ => unimplemented!(
                    "this type of struct is currently not supported by this derive macro"
                ),
            };
            let deserialization_by_field = fields.iter().map(|f| {
                let field_type = &f.ty;
                match &f.ident {
                    Some(_) => quote_spanned! { f.span() =>
                        deserializer.parse::<#field_type>(encoding)?;
                    },
                    None => panic!("should have already returned an impl"),
                }
            });

            let initialization_by_field = fields.iter().enumerate().map(|(i, f)| {
                let field_type = &f.ty;
                match &f.ident {
                    Some(field_name) => quote_spanned! { f.span() =>
                        #field_name: <#field_type>::deserialize(&encoding[spans[2*#i]..spans[2*#i+1]])?,
                    },
                    None => panic!("should have already returned an impl"),
                }
            });

            quote! {
                fn deserialize(encoding: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
                    let mut deserializer = ssz_rs::__internal::ContainerDeserializer::default();

                    #(#deserialization_by_field)*

                    let spans = deserializer.finalize(encoding)?;

                    Ok(Self {
                        #(#initialization_by_field)*
                    })
                }
            }
        }
        Data::Enum(ref data) => {
            let deserialization_by_variant =
                data.variants.iter().enumerate().map(|(i, variant)| {
                    // NOTE: this is "safe" as the number of legal variants fits into `u8`
                    let i = i as u8;
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unnamed(inner) => {
                            // SAFETY: index is safe because Punctuated always has a first element;
                            // qed
                            let variant_type = &inner.unnamed[0];
                            quote_spanned! { variant.span() =>
                                #i => {
                                    // SAFETY: index is safe because encoding isn't empty; qed
                                    let value = <#variant_type>::deserialize(&encoding[1..])?;
                                    Ok(Self::#variant_name(value))
                                }
                            }
                        }
                        Fields::Unit => {
                            quote_spanned! { variant.span() =>
                                0 => {
                                    if encoding.len() != 1 {
                                        return Err(DeserializeError::AdditionalInput {
                                            provided: encoding.len(),
                                            expected: 1,
                                        })
                                    }
                                    Ok(Self::None)
                                },
                            }
                        }
                        _ => unreachable!(),
                    }
                });

            quote! {
                fn deserialize(encoding: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
                    if encoding.is_empty() {
                        return Err(ssz_rs::DeserializeError::ExpectedFurtherInput {
                            provided: 0,
                            expected: 1,
                        });
                    }

                    // SAFETY: index is safe because encoding isn't empty; qed
                    match encoding[0] {
                        #(#deserialization_by_variant)*
                        b => Err(ssz_rs::DeserializeError::InvalidByte(b)),
                    }
                }
            }
        }
        Data::Union(..) => unreachable!("data was already validated to exclude union types"),
    }
}

fn derive_variable_size_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                Fields::Named(ref fields) => &fields.named,
                Fields::Unnamed(ref fields) => &fields.unnamed,
                _ => unimplemented!(
                    "this type of struct is currently not supported by this derive macro"
                ),
            };
            let impl_by_field = fields.iter().map(|f| {
                let field_type = &f.ty;
                quote_spanned! { f.span() =>
                    <#field_type>::is_variable_size()
                }
            });

            quote! {
                #(#impl_by_field)|| *
            }
        }
        Data::Enum(..) => {
            quote! { true }
        }
        Data::Union(..) => unreachable!("data was already validated to exclude union types"),
    }
}

fn derive_size_hint_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                Fields::Named(ref fields) => &fields.named,
                Fields::Unnamed(ref fields) => &fields.unnamed,
                _ => unimplemented!(
                    "this type of struct is currently not supported by this derive macro"
                ),
            };
            let impl_by_field = fields.iter().map(|f| {
                let field_type = &f.ty;
                quote_spanned! { f.span() =>
                    <#field_type>::size_hint()
                }
            });

            quote! {
                if Self::is_variable_size() {
                    0
                } else {
                    #(#impl_by_field)+ *
                }
            }
        }
        Data::Enum(..) => {
            quote! { 0 }
        }
        Data::Union(..) => unreachable!("data was already validated to exclude union types"),
    }
}

fn derive_merkleization_impl(
    data: &Data,
    name: &Ident,
    generics: &Generics,
    helper_attr: Option<&HelperAttr>,
) -> TokenStream {
    let method = match data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                Fields::Named(ref fields) => &fields.named,
                Fields::Unnamed(ref fields) => &fields.unnamed,
                _ => unimplemented!(
                    "this type of struct is currently not supported by this derive macro"
                ),
            };
            let field_count = fields.iter().len();
            let impl_by_field = fields.iter().enumerate().map(|(i, f)| match &f.ident {
                Some(field_name) => quote_spanned! { f.span() =>
                    let chunk = self.#field_name.hash_tree_root()?;
                    let range = #i*#BYTES_PER_CHUNK..(#i+1)*#BYTES_PER_CHUNK;
                    chunks[range].copy_from_slice(chunk.as_ref());
                },
                None => quote_spanned! { f.span() =>
                    let chunk = self.0.hash_tree_root()?;
                    let range = #i*#BYTES_PER_CHUNK..(#i+1)*#BYTES_PER_CHUNK;
                    chunks[range].copy_from_slice(chunk.as_ref());
                },
            });
            quote! {
                fn hash_tree_root(&mut self) -> Result<ssz_rs::Node, ssz_rs::MerkleizationError> {
                    let mut chunks = vec![0u8; #field_count * #BYTES_PER_CHUNK];
                    #(#impl_by_field)*
                    ssz_rs::__internal::merkleize(&chunks, None)
                }
            }
        }
        Data::Enum(ref data) => {
            let hash_tree_root_by_variant = data.variants.iter().enumerate().map(|(i, variant)| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unnamed(..) => {
                        // NOTE: validated to only be `transparent` operation at this point...
                        if helper_attr.is_some() {
                            quote_spanned! { variant.span() =>
                               Self::#variant_name(value) => value.hash_tree_root(),
                            }
                        } else {
                            quote_spanned! { variant.span() =>
                               Self::#variant_name(value) => {
                                   let selector = #i;
                                   let data_root  = value.hash_tree_root()?;
                                   Ok(ssz_rs::__internal::mix_in_selector(&data_root, selector))
                               }
                            }
                        }
                    }
                    Fields::Unit => {
                        quote_spanned! { variant.span() =>
                            Self::None => Ok(ssz_rs::__internal::mix_in_selector(
                                &ssz_rs::Node::default(),
                                0,
                            )),
                        }
                    }
                    _ => unreachable!(),
                }
            });
            quote! {
                fn hash_tree_root(&mut self) -> Result<ssz_rs::Node, ssz_rs::MerkleizationError> {
                    match self {
                            #(#hash_tree_root_by_variant)*
                    }
                }
            }
        }
        Data::Union(..) => unreachable!("data was already validated to exclude union types"),
    };
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    quote! {
        impl #impl_generics ssz_rs::Merkleized for #name #ty_generics {
            #method
        }
    }
}

fn is_valid_none_identifier(ident: &Ident) -> bool {
    *ident == format_ident!("None")
}

fn filter_ssz_attrs<'a>(
    attrs: impl Iterator<Item = &'a Attribute>,
) -> impl Iterator<Item = &'a Attribute> {
    attrs.filter(|&f| {
        if let Some(path) = f.path.segments.first() {
            path.ident == format_ident!("{SSZ_HELPER_ATTRIBUTE}")
        } else {
            false
        }
    })
}

fn validate_no_attrs<'a>(fields: impl Iterator<Item = &'a Field>) {
    let mut ssz_attrs = fields.flat_map(|field| filter_ssz_attrs(field.attrs.iter()));
    if ssz_attrs.next().is_some() {
        panic!("macro attribute `{SSZ_HELPER_ATTRIBUTE}` is only allowed at struct or enum level")
    }
}

// Validates the incoming data follows the rules
// for mapping the Rust term to something that can
// implement the `SimpleSerialize` trait.
//
// Panics if validation fails which aborts the macro derivation.
fn validate_derive_input(data: &Data, helper_attrs: &[HelperAttr]) {
    if helper_attrs.len() > 1 {
        panic!("only one argument to the helper attribute is allowed")
    }
    let mut is_transparent = false;
    if let Some(attr) = helper_attrs.first() {
        match attr {
            HelperAttr::Transparent => is_transparent = true,
        }
    }
    if is_transparent && !matches!(data, Data::Enum(..)) {
        panic!("`transparent` option is only compatible with enums")
    }
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if fields.named.is_empty() {
                    panic!("ssz_rs containers with no fields are illegal")
                }
                validate_no_attrs(fields.named.iter())
            }
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                validate_no_attrs(fields.unnamed.iter())
            }
            _ => panic!("structs with unit or multiple unnnamed fields are not supported"),
        },
        Data::Enum(ref data) => {
            if data.variants.is_empty() {
                panic!("SSZ unions must have at least 1 variant; this enum has none");
            }

            if data.variants.len() > 127 {
                panic!("SSZ unions cannot have more than 127 variants; this enum has more");
            }

            let mut none_forbidden = false;
            let mut already_has_none = false;
            for (i, variant) in data.variants.iter().enumerate() {
                let mut variant_ssz_attrs = filter_ssz_attrs(variant.attrs.iter());
                if variant_ssz_attrs.next().is_some() {
                    panic!("macro attribute `{SSZ_HELPER_ATTRIBUTE}` is only allowed at struct or enum level")
                }
                validate_no_attrs(variant.fields.iter());
                match &variant.fields {
                    Fields::Unnamed(inner) => {
                        if i == 0 {
                            none_forbidden = true;
                        }
                        if inner.unnamed.len() != 1 {
                            panic!("enums can only have 1 type per variant");
                        }
                        validate_no_attrs(inner.unnamed.iter());
                    }
                    Fields::Unit => {
                        if is_transparent {
                            panic!("`transparent` option is only compatible with unnamed variants")
                        }
                        if none_forbidden {
                            panic!(
                                "found unit variant that conflicts with previous unnamed variants"
                            );
                        }
                        if already_has_none {
                            panic!("cannot duplicate a unit variant (as only `None` is allowed)");
                        }
                        if !is_valid_none_identifier(&variant.ident) {
                            panic!("variant identifier is invalid: must be `None`");
                        }
                        if i != 0 {
                            panic!("only the first variant can be unit type (and must be `None`)")
                        }
                        if data.variants.len() < 2 {
                            panic!(
                                "SSZ unions must have more than 1 selector if the first is `None`"
                            );
                        }
                        already_has_none = true;
                    }
                    Fields::Named(..) => {
                        panic!("Enums with named fields in variants are not supported");
                    }
                };
            }
        }
        Data::Union(..) => panic!("Rust unions cannot produce valid SSZ types"),
    }
}

fn derive_serializable_impl(
    data: &Data,
    name: &Ident,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    let serialize_impl = derive_serialize_impl(data);
    let deserialize_impl = derive_deserialize_impl(data);
    let is_variable_size_impl = derive_variable_size_impl(data);
    let size_hint_impl = derive_size_hint_impl(data);

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    quote! {
        impl #impl_generics ssz_rs::Serialize for #name #ty_generics {
            #serialize_impl
        }

        impl #impl_generics ssz_rs::Deserialize for #name #ty_generics {
            #deserialize_impl
        }

        impl #impl_generics ssz_rs::Serializable for #name #ty_generics {
            fn is_variable_size() -> bool {
                #is_variable_size_impl
            }

            fn size_hint() -> usize {
                #size_hint_impl
            }
        }
    }
}

fn derive_simple_serialize_impl(name: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    quote! {
        impl #impl_generics ssz_rs::SimpleSerialize for #name #ty_generics {}
    }
}

#[derive(Debug)]
enum HelperAttr {
    Transparent,
}

fn parse_helper_attr(ident: &Ident) -> HelperAttr {
    match ident.to_string().as_str() {
        "transparent" => HelperAttr::Transparent,
        ident => panic!("unsupported helper attribute:{ident}"),
    }
}

fn extract_helper_attrs(input: &DeriveInput) -> Vec<HelperAttr> {
    filter_ssz_attrs(input.attrs.iter())
        .flat_map(|attr| {
            let meta = attr.parse_meta().unwrap();
            match meta {
                Meta::List(args) => args
                    .nested
                    .iter()
                    .map(|arg| match arg {
                        NestedMeta::Meta(meta) => {
                            if let Meta::Path(path) = meta {
                                assert!(path.leading_colon.is_none());
                                assert_eq!(path.segments.len(), 1);
                                let path = &path.segments[0];
                                match path.arguments {
                                    PathArguments::None => parse_helper_attr(&path.ident),
                                    _ => panic!("no arguments are supported to attribute symbols"),
                                }
                            } else {
                                panic!("unsupported argument to helper attribute");
                            }
                        }
                        _ => panic!("literals unsupported in attributes"),
                    })
                    .collect::<Vec<_>>(),
                _ => panic!("only list-like attributes are supported"),
            }
        })
        .collect()
}

#[proc_macro_derive(Serializable, attributes(ssz))]
pub fn derive_serializable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = &input.data;
    let helper_attrs = extract_helper_attrs(&input);
    validate_derive_input(data, &helper_attrs);

    let name = &input.ident;
    let generics = &input.generics;

    let expansion = derive_serializable_impl(data, name, generics);
    proc_macro::TokenStream::from(expansion)
}

#[proc_macro_derive(Merkleized, attributes(ssz))]
pub fn derive_merkleized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = &input.data;
    let helper_attrs = extract_helper_attrs(&input);
    validate_derive_input(data, &helper_attrs);
    let helper_attr = helper_attrs.first();

    let name = &input.ident;
    let generics = &input.generics;

    let expansion = derive_merkleization_impl(data, name, generics, helper_attr);
    proc_macro::TokenStream::from(expansion)
}

#[proc_macro_derive(SimpleSerialize, attributes(ssz))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = &input.data;
    let helper_attrs = extract_helper_attrs(&input);
    validate_derive_input(data, &helper_attrs);
    let helper_attr = helper_attrs.first();

    let name = &input.ident;
    let generics = &input.generics;
    let merkleization_impl = derive_merkleization_impl(data, name, generics, helper_attr);

    let serializable_impl = derive_serializable_impl(data, name, generics);

    let simple_serialize_impl = derive_simple_serialize_impl(name, generics);

    let expansion = quote! {
        #serializable_impl

        #merkleization_impl

        #simple_serialize_impl
    };

    proc_macro::TokenStream::from(expansion)
}
