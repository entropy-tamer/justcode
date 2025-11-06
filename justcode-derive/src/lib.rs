//! Derive macros for justcode Encode and Decode traits.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro for the `Encode` trait.
#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
                    Fields::Named(fields) => {
                        let field_encodes = fields.named.iter().map(|field| {
                            let field_name = &field.ident;
                            quote! {
                                self.#field_name.encode(writer)?;
                            }
                        });
                        quote! {
                            #(#field_encodes)*
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_encodes = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let index = syn::Index::from(i);
                            quote! {
                                self.#index.encode(writer)?;
                            }
                        });
                        quote! {
                            #(#field_encodes)*
                        }
                    }
                    Fields::Unit => {
                        quote! {}
                    }
        },
        Data::Enum(data_enum) => {
            // Encode variant index first, then variant data
            let variant_encodes = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_idx = idx as u32;
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                        let field_encodes = field_names.iter().map(|field_name| {
                            quote! {
                                #field_name.encode(writer)?;
                            }
                        });
                        quote! {
                            #name::#variant_name { #(#field_names,)* } => {
                                use justcode_core::varint::encode_length;
                                encode_length(writer, #variant_idx as usize, writer.config())?;
                                #(#field_encodes)*
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_count = fields.unnamed.len();
                        let field_indices: Vec<_> = (0..field_count)
                            .map(|i| syn::Index::from(i))
                            .collect();
                        let field_encodes = field_indices.iter().map(|index| {
                            quote! {
                                #index.encode(writer)?;
                            }
                        });
                        let field_patterns = field_indices.clone();
                        quote! {
                            #name::#variant_name(#(#field_patterns,)*) => {
                                use justcode_core::varint::encode_length;
                                encode_length(writer, #variant_idx as usize, writer.config())?;
                                #(#field_encodes)*
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => {
                                use justcode_core::varint::encode_length;
                                encode_length(writer, #variant_idx as usize, writer.config())?;
                            }
                        }
                    }
                }
            });
            quote! {
                match self {
                    #(#variant_encodes)*
                }
            }
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                name,
                "justcode does not support encoding unions",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics justcode_core::Encode for #name #ty_generics #where_clause {
            fn encode(&self, writer: &mut justcode_core::writer::Writer) -> justcode_core::Result<()> {
                #encode_body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for the `Decode` trait.
#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let decode_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_decodes = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_ty = &field.ty;
                    quote! {
                        #field_name: <#field_ty as justcode_core::Decode>::decode(reader)?
                    }
                });
                quote! {
                    Ok(#name {
                        #(#field_decodes,)*
                    })
                }
            }
            Fields::Unnamed(fields) => {
                let field_decodes = fields.unnamed.iter().map(|field| {
                    let field_ty = &field.ty;
                    quote! {
                        <#field_ty as justcode_core::Decode>::decode(reader)?
                    }
                });
                quote! {
                    Ok(#name(#(#field_decodes,)*))
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(#name)
                }
            }
        },
        Data::Enum(data_enum) => {
            // Decode variant index first, then variant data
            let variant_decodes = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_idx = idx as u32;
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_decodes = fields.named.iter().map(|field| {
                            let field_name = &field.ident;
                            let field_ty = &field.ty;
                            quote! {
                                #field_name: <#field_ty as justcode_core::Decode>::decode(reader)?
                            }
                        });
                        quote! {
                            #variant_idx => Ok(#name::#variant_name {
                                #(#field_decodes,)*
                            })
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_decodes = fields.unnamed.iter().map(|field| {
                            let field_ty = &field.ty;
                            quote! {
                                <#field_ty as justcode_core::Decode>::decode(reader)?
                            }
                        });
                        quote! {
                            #variant_idx => Ok(#name::#variant_name(
                                #(#field_decodes,)*
                            ))
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #variant_idx => Ok(#name::#variant_name)
                        }
                    }
                }
            });
            quote! {
                use justcode_core::varint::decode_length;
                let variant_idx = decode_length(reader, reader.config())? as u32;
                match variant_idx {
                    #(#variant_decodes,)*
                    _ => Err(justcode_core::error::JustcodeError::custom(format!("invalid variant index: {}", variant_idx))),
                }
            }
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                name,
                "justcode does not support decoding unions",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics justcode_core::Decode for #name #ty_generics #where_clause {
            fn decode(reader: &mut justcode_core::reader::Reader) -> justcode_core::Result<Self> {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}

