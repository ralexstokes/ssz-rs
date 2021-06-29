use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                let element_count = fields.named.len();
                let deserialization_by_field = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! { f.span() =>
                        let bytes_read = if <#field_type>::is_variable_size() {
                            let end = start + ssz::ser::BYTES_PER_LENGTH_OFFSET;
                            let end_of_element = u32::deserialize(&encoding[start..end])? as usize;

                            if let Some(start_of_element) = last_offset {
                                container.#field_name = <#field_type>::deserialize(&encoding[start_of_element..end_of_element])?;
                            }
                            last_offset.insert(end_of_element);

                            if #i == #element_count - 1 {
                                container.#field_name = <#field_type>::deserialize(&encoding[end_of_element..])?;
                            }

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
                        let mut last_offset = None;

                        let mut container = Self::default();

                        #(#deserialization_by_field)*

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

    let serialize_impl = derive_serialize_impl(data);
    let deserialize_impl = derive_deserialize_impl(data);
    let is_variable_size_impl = derive_variable_size_impl(data);
    let size_hint_impl = derive_size_hint_impl(data);

    let expansion = quote! {
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
