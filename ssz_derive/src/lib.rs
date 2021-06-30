use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn derive_set_by_index_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if fields.named.is_empty() {
                    panic!("ssz containers with no fields are illegal")
                }
                let set_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        #i => { self.#field_name = <#field_type>::deserialize(encoding)?; },
                    }
                });

                quote! {
                    fn __ssz_set_by_index(&mut self, index: usize, encoding: &[u8]) -> Result<(), ssz::DeserializeError> {
                        match index {
                            #(#set_by_field)*
                            _ => unreachable!(),
                        }
                        Ok(())
                    }
                }
            }
            _ => panic!("not supported"),
        },
        Data::Enum(_data) => {
            unimplemented!()
        }
        Data::Union(..) => panic!("not supported"),
    }
}

fn derive_serialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if fields.named.is_empty() {
                    panic!("ssz containers with no fields are illegal")
                }
                let serialization_by_field = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        let mut element_buffer = Vec::with_capacity(<#field_type>::size_hint());
                        self.#field_name.serialize(&mut element_buffer)?;

                        if <#field_type>::is_variable_size() {
                            let buffer_len = element_buffer.len();
                            fixed.push(None);
                            fixed_lengths_sum += ssz::ser::BYTES_PER_LENGTH_OFFSET;
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
                    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, ssz::SerializeError> {
                        let mut fixed = vec![];
                        let mut variable = vec![];
                        let mut variable_lengths = vec![];
                        let mut fixed_lengths_sum = 0;

                        #(#serialization_by_field)*

                        ssz::ser::serialize_composite_from_components(fixed, variable, variable_lengths, fixed_lengths_sum, buffer)
                    }
                }
            }
            _ => panic!("not supported"),
        },
        Data::Enum(_data) => {
            unimplemented!()
        }
        Data::Union(..) => panic!("not supported"),
    }
}

fn derive_deserialize_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                if fields.named.is_empty() {
                    panic!("ssz containers with no fields are illegal")
                }
                let deserialization_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        let bytes_read = if <#field_type>::is_variable_size() {
                            let end = start + ssz::ser::BYTES_PER_LENGTH_OFFSET;
                            let next_offset = u32::deserialize(&encoding[start..end])?;
                            offsets.push((#i, next_offset as usize));

                            ssz::ser::BYTES_PER_LENGTH_OFFSET
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
                    fn deserialize(encoding: &[u8]) -> Result<Self, ssz::DeserializeError> {
                        let mut start = 0;
                        let mut offsets = vec![];
                        let mut container = Self::default();

                        #(#deserialization_by_field){}*

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

                            container.__ssz_set_by_index(index, &encoding[start..end])?;
                        }

                        Ok(container)
                    }
                }
            }
            _ => panic!("not supported"),
        },
        Data::Enum(_data) => {
            unimplemented!()
        }
        Data::Union(..) => panic!("not supported"),
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
            _ => panic!("not supported"),
        },
        Data::Enum(..) => {
            quote! { true }
        }
        Data::Union(..) => panic!("not supported"),
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
            _ => panic!("not supported"),
        },
        Data::Enum(..) => {
            unimplemented!()
        }
        Data::Union(..) => panic!("not supported"),
    }
}

#[proc_macro_derive(Serialize)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = &input.data;

    let set_by_index_impl = derive_set_by_index_impl(data);
    let serialize_impl = derive_serialize_impl(data);
    let deserialize_impl = derive_deserialize_impl(data);
    let is_variable_size_impl = derive_variable_size_impl(data);
    let size_hint_impl = derive_size_hint_impl(data);

    let expansion = quote! {

        impl #name {
            #set_by_index_impl
        }

        impl ssz::Serialize for #name {
            #serialize_impl
        }

        impl ssz::Deserialize for #name {
            #deserialize_impl
        }

        impl ssz::SSZ for #name {
            fn is_variable_size() -> bool {
                #is_variable_size_impl
            }

            fn size_hint() -> usize {
                #size_hint_impl
            }
        }
    };

    proc_macro::TokenStream::from(expansion)
}
