use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

#[proc_macro_derive(WireSize, attributes(options))]
pub fn derive_wire_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let wire_size_impl = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let field_sizes = fields_named.named.into_iter().map(|field| {
                    let options = field.attrs.into_iter().find_map(|attr| {
                        if attr.path().is_ident("options") {
                            Some(attr.parse_args::<Options>())
                        } else {
                            None
                        }
                    });

                    let field_name = &field.ident;
                    match options {
                        Some(Ok(options)) => {
                            let options = options.to_tokens(&field.ty);
                            quote! {
                                size += self.#field_name.wire_size(#options);
                            }
                        }
                        Some(Err(err)) => err.to_compile_error(),
                        None => {
                            quote! {
                                size += self.#field_name.wire_size(::core::default::Default::default());
                            }
                        }
                    }
                });

                quote! {
                    let mut size = 0;
                    #(#field_sizes)*
                    size
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_sizes =
                    fields_unnamed
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let options = field.attrs.into_iter().find_map(|attr| {
                                if attr.path().is_ident("options") {
                                    Some(attr.parse_args::<Options>())
                                } else {
                                    None
                                }
                            });

                            let index = syn::Index::from(index);
                            match options {
                                Some(Ok(options)) => {
                                    let options = options.to_tokens(&field.ty);
                                    quote! {
                                        size += self.#index.wire_size(#options);
                                    }
                                }
                                Some(Err(err)) => err.to_compile_error(),
                                None => {
                                    quote! {
                                        size += self.#index.wire_size(::core::default::Default::default());
                                    }
                                }
                            }
                        });

                quote! {
                    let mut size = 0;
                    #(#field_sizes)*
                    size
                }
            }
            Fields::Unit => {
                quote! {
                    0
                }
            }
        },
        _ => panic!("WireSize can only be derived for structs"),
    };

    let expanded = quote! {
        #[automatically_derived]
        impl minecrevy_encdec::WireSize for #name {
            type Options = ();

            fn wire_size(&self, (): Self::Options) -> usize {
                #wire_size_impl
            }
        }
    };

    TokenStream::from(expanded)
}

struct Options(Punctuated<Option, syn::Token![,]>);

impl Parse for Options {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse_terminated(Option::parse, syn::Token![,])?))
    }
}

impl Options {
    pub fn to_tokens(&self, ty: &Type) -> proc_macro2::TokenStream {
        let options = self.0.iter();
        quote! {
            {
                let mut opts = <#ty as minecrevy_encdec::WireSize>::Options::default();
                #(opts #options ;)*
                opts
            }
        }
    }
}

struct Option {
    pub dot: syn::Token![.],
    pub field: syn::Ident,
    pub value: syn::Expr,
}

impl Parse for Option {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Option {
            dot: input.parse()?,
            field: input.parse()?,
            value: {
                input.parse::<syn::Token![=]>()?;
                input.parse()?
            },
        })
    }
}

impl quote::ToTokens for Option {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Option { dot, field, value } = self;
        quote! {
            #dot #field = #value
        }
        .to_tokens(tokens);
    }
}
