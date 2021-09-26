//! `SimpleSerialize` provides a macro to derive SSZ containers and union types from
//! native Rust structs and enums.
//! Refer to the `examples` in the `ssz-rs` crate for a better idea on how to use this derive macro.
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

// NOTE: copied here from `ssz_rs` crate as it is unlikely to change
// and can keep it out of the crate's public interface.
const BYTES_PER_LENGTH_OFFSET: usize = 4;
const BYTES_PER_CHUNK: usize = 32;

fn derive_container_set_by_index_impl(name: &Ident, data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let set_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        #i => { self.#field_name = <#field_type>::deserialize(encoding)?; },
                    }
                });

                quote! {
                    impl #name {
                        fn __ssz_rs_set_by_index(&mut self, index: usize, encoding: &[u8]) -> Result<(), ssz_rs::DeserializeError> {
                            match index {
                                #(#set_by_field)*
                                _ => unreachable!(),
                            }
                            Ok(())
                        }
                    }
                }
            }
            _ => unreachable!(),
        },
        Data::Enum(..) => quote! {},
        Data::Union(..) => unreachable!(),
    }
}

fn derive_union_default_impl(name: &Ident, data: &Data) -> TokenStream {
    match data {
        Data::Struct(..) => quote! {},
        Data::Enum(ref data) => {
            let variant = &data.variants[0];
            let variant_name = &variant.ident;
            let default_impl = match &variant.fields {
                Fields::Unnamed(inner) => {
                    let variant_type = &inner.unnamed[0];
                    quote_spanned! { variant.span() =>
                        Self::#variant_name(<#variant_type>::default())
                    }
                }
                Fields::Unit => {
                    quote_spanned! { variant.span() =>
                        Self::None
                    }
                }
                _ => unreachable!(),
            };

            quote! {
                impl std::default::Default for #name {
                    fn default() -> Self {
                        #default_impl
                    }
                }
            }
        }
        Data::Union(..) => unreachable!(),
    }
}

fn derive_serialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let serialization_by_field = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        let mut element_buffer = Vec::with_capacity(<#field_type>::size_hint());
                        self.#field_name.serialize(&mut element_buffer)?;

                        if <#field_type>::is_variable_size() {
                            let buffer_len = element_buffer.len();
                            fixed.push(None);
                            fixed_lengths_sum += #BYTES_PER_LENGTH_OFFSET;
                            variable.push(element_buffer);
                            variable_lengths.push(buffer_len);
                        } else {
                            let buffer_len = element_buffer.len();
                            fixed.push(Some(element_buffer));
                            fixed_lengths_sum += buffer_len;
                            variable_lengths.push(0)
                        }
                    }
                });

                quote! {
                    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, ssz_rs::SerializeError> {
                        let mut fixed = vec![];
                        let mut variable = vec![];
                        let mut variable_lengths = vec![];
                        let mut fixed_lengths_sum = 0;

                        #(#serialization_by_field)*

                        ssz_rs::internal::serialize_composite_from_components(fixed, variable, variable_lengths, fixed_lengths_sum, buffer)
                    }
                }
            }
            _ => unreachable!(),
        },
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
        Data::Union(..) => unreachable!(),
    }
}

fn derive_deserialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let deserialization_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        let bytes_read = if <#field_type>::is_variable_size() {
                            let end = start + #BYTES_PER_LENGTH_OFFSET;
                            let next_offset = u32::deserialize(&encoding[start..end])?;
                            offsets.push((#i, next_offset as usize));

                            #BYTES_PER_LENGTH_OFFSET
                        } else {
                            let encoded_length = <#field_type>::size_hint();
                            let end = start + encoded_length;
                            let result = <#field_type>::deserialize(&encoding[start..end])?;
                            container.#field_name = result;
                            encoded_length
                        };
                        start += bytes_read;
                    }
                });

                quote! {
                    fn deserialize(encoding: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
                        let mut start = 0;
                        let mut offsets = vec![];
                        let mut container = Self::default();

                        #(#deserialization_by_field)*

                        if let Some((_, offset)) = offsets.first() {
                            // NOTE: this invariant should always hold
                            // and also quiets a warning about the last write
                            // to `start` not being used otherwise...
                            assert_eq!(start, *offset);
                        }

                        // NOTE: this value is not read
                        let dummy_index = 0;
                        offsets.push((dummy_index, encoding.len()));

                        for span in offsets.windows(2) {
                            let (index, start) = span[0];
                            let (_, end) = span[1];

                            container.__ssz_rs_set_by_index(index, &encoding[start..end])?;
                        }

                        Ok(container)
                    }
                }
            }
            _ => unreachable!(),
        },
        Data::Enum(ref data) => {
            let deserialization_by_variant =
                data.variants.iter().enumerate().map(|(i, variant)| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unnamed(inner) => {
                            let variant_type = &inner.unnamed[0];
                            quote_spanned! { variant.span() =>
                                #i => {
                                    let value = <#variant_type>::deserialize(&encoding[1..])?;
                                    Ok(Self::#variant_name(value))
                                }
                            }
                        }
                        Fields::Unit => {
                            quote_spanned! { variant.span() =>
                                0 => Ok(Self::None),
                            }
                        }
                        _ => unreachable!(),
                    }
                });

            quote! {
                fn deserialize(encoding: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
                    if encoding.is_empty() {
                        return Err(ssz_rs::DeserializeError::InputTooShort);
                    }

                    match &encoding[0].into() {
                        #(#deserialization_by_variant)*
                        _ => Err(ssz_rs::DeserializeError::InvalidInput),
                    }
                }
            }
        }
        Data::Union(..) => unreachable!(),
    }
}

fn derive_variable_size_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let impl_by_field = fields.named.iter().map(|f| {
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        <#field_type>::is_variable_size()
                    }
                });

                quote! {
                    #(#impl_by_field)&& *
                }
            }
            _ => unreachable!(),
        },
        Data::Enum(..) => {
            quote! { true }
        }
        Data::Union(..) => unreachable!(),
    }
}

fn derive_size_hint_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let impl_by_field = fields.named.iter().map(|f| {
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
            _ => unreachable!(),
        },
        Data::Enum(..) => {
            quote! { 0 }
        }
        Data::Union(..) => unreachable!(),
    }
}

fn is_valid_none_identifier(ident: &Ident) -> bool {
    *ident == format_ident!("None")
}

// Validates the incoming data follows the rules
// for mapping the Rust term to something that can
// implement the `SimpleSerialize` trait.
fn validate_derive_data(data: ValidationState) -> ValidationState {
    let data = match data {
        ValidationState::Unvalidated(data) => data,
        data @ ValidationState::Validated(..) => return data,
    };

    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if fields.named.is_empty() {
                    panic!("ssz_rs containers with no fields are illegal")
                }
            }
            _ => panic!("Structs with unit or unnnamed fields are not supported"),
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
                match &variant.fields {
                    Fields::Unnamed(inner) => {
                        if i == 0 {
                            none_forbidden = true;
                        }
                        if inner.unnamed.len() != 1 {
                            panic!("Enums can only have 1 type per variant");
                        }
                    }
                    Fields::Unit => {
                        if none_forbidden {
                            panic!("only the first variant can be `None`");
                        }
                        if already_has_none {
                            panic!("cannot duplicate a unit variant (as only `None` is allowed)");
                        }
                        if !is_valid_none_identifier(&variant.ident) {
                            panic!("Variant identifier is invalid: must be `None`");
                        }
                        assert!(i == 0);
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

    ValidationState::Validated(data)
}

fn derive_merkleization_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_count = fields.named.iter().len();
                let impl_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    quote_spanned! { f.span() =>
                        let chunk = self.#field_name.hash_tree_root()?;
                        let range = #i*#BYTES_PER_CHUNK..(#i+1)*#BYTES_PER_CHUNK;
                        chunks[range].copy_from_slice(&chunk);
                    }
                });
                quote! {
                    fn hash_tree_root(&self) -> Result<ssz_rs::Root, ssz_rs::MerkleizationError> {
                        let mut chunks = vec![0u8; #field_count * #BYTES_PER_CHUNK];
                        #(#impl_by_field)*
                        Ok(ssz_rs::internal::merkleize(&chunks, None)?)
                    }
                }
            }
            _ => unreachable!(),
        },
        Data::Enum(ref data) => {
            let hash_tree_root_by_variant = data.variants.iter().enumerate().map(|(i, variant)| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unnamed(..) => {
                        quote_spanned! { variant.span() =>
                            Self::#variant_name(value) => {
                                let selector = #i as u8 as usize;
                                let data_root  = value.hash_tree_root()?;
                                Ok(ssz_rs::internal::mix_in_selector(&data_root, selector))
                            }
                        }
                    }
                    Fields::Unit => {
                        quote_spanned! { variant.span() =>
                            Self::None => Ok(ssz_rs::internal::mix_in_selector(
                                &ssz_rs::Root::default(),
                                0,
                            )),
                        }
                    }
                    _ => unreachable!(),
                }
            });
            quote! {
                fn hash_tree_root(&self) -> Result<ssz_rs::Root, ssz_rs::MerkleizationError> {
                    match self {
                            #(#hash_tree_root_by_variant)*
                    }
                }
            }
        }
        Data::Union(..) => unreachable!(),
    }
}

// Refers to the validation state of proc macro's input
enum ValidationState<'a> {
    Unvalidated(&'a Data),
    Validated(&'a Data),
}

#[proc_macro_derive(SimpleSerialize)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = ValidationState::Unvalidated(&input.data);

    let data = &validate_derive_data(data);

    let data = match *data {
        ValidationState::Validated(data) => data,
        ValidationState::Unvalidated(..) => panic!("do not process unvalidated input"),
    };

    let set_by_index_impl = derive_container_set_by_index_impl(name, data);
    let union_default_impl = derive_union_default_impl(name, data);
    let serialize_impl = derive_serialize_impl(data);
    let deserialize_impl = derive_deserialize_impl(data);
    let is_variable_size_impl = derive_variable_size_impl(data);
    let size_hint_impl = derive_size_hint_impl(data);
    let merkleization_impl = derive_merkleization_impl(data);

    let expansion = quote! {
        #union_default_impl

        #set_by_index_impl

        impl ssz_rs::Serialize for #name {
            #serialize_impl
        }

        impl ssz_rs::Deserialize for #name {
            #deserialize_impl
        }

        impl ssz_rs::Sized for #name {
            fn is_variable_size() -> bool {
                #is_variable_size_impl
            }

            fn size_hint() -> usize {
                #size_hint_impl
            }
        }

        impl ssz_rs::Merkleized for #name {
            #merkleization_impl
        }

        impl ssz_rs::SimpleSerialize for #name {}
    };

    proc_macro::TokenStream::from(expansion)
}
