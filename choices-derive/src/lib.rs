//! Proc macros for the `choices` crate.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error, set_dummy};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

const BASE_PATH: &str = "config";

/// Generates the `Choices` impl.
#[proc_macro_derive(Choices)]
#[proc_macro_error]
pub fn choices(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_choices(&input);
    gen.into()
}

fn impl_choices(input: &DeriveInput) -> TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;

    set_dummy(quote! {
        #[async_trait::async_trait]
        impl ::choices::Choices for #struct_name {
            unimplemented!();
        }
    });

    match input.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => impl_choices_for_struct(struct_name, &fields.named, &input.attrs),
        _ => abort_call_site!("choices only supports non-tuple structs"),
    }
}

fn impl_choices_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    _attrs: &[Attribute],
) -> TokenStream {
    let (macros, implementation, choices) = gen_choices_warp(fields);

    quote! {
        #macros

        #[allow(unused_variables)]
        #[allow(unknown_lints)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        #[allow(dead_code, unreachable_code)]
        impl #name {
            #implementation
        }

        #[allow(unused_variables)]
        #[allow(unknown_lints)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        #[allow(dead_code, unreachable_code)]
        #[async_trait::async_trait]
        impl ::choices::Choices for #name {
            #choices
        }
    }
}

fn gen_choices_warp(fields: &Punctuated<Field, Comma>) -> (TokenStream, TokenStream, TokenStream) {
    let fields_populators = fields.iter().map(|field| {
        let field_ident = field
            .ident
            .as_ref()
            .expect("unnamed fields are not supported!");
        let field_name = field_ident.to_string();
        Some(quote! {
            warp::path!(#BASE_PATH / #field_name).map(move || format!("{}", $self.#field_ident.body_string()) )
        })
    });

    let index_string = compute_index_string(fields);

    let macro_tk = quote! {
            macro_rules! create_filter {
            ($self:ident) => {{
                use warp::Filter;
                #[allow(unused_imports)]
                use choices::ChoicesOutput;

                let index = warp::path(#BASE_PATH).map(|| #index_string);
                warp::get().and(
                    index.and(warp::path::end())
                    #( .or(#fields_populators) )*
                )
            }};
        }
    };

    let implementation_tk = quote! {
        /// If you want more control over the http server instance you can use this
        /// function to retrieve the configuration's `warp::Filter`.
        fn filter(&'static self) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
            use warp::Filter;
            create_filter!(self).boxed()
        }
    };

    let trait_tk = quote! {
        async fn run<T: Into<std::net::SocketAddr> + Send>(&'static self, addr: T) {
            let filter = create_filter!(self);
            warp::serve(filter).run(addr).await
        }
    };

    (macro_tk, implementation_tk, trait_tk)
}

fn compute_index_string(fields: &Punctuated<Field, Comma>) -> String {
    let mut index = "Available configuration options:\n".to_string();
    fields.iter().for_each(|field| {
        let field_ident = field
            .ident
            .as_ref()
            .expect("unnamed fields are not supported!");
        let type_name = compute_type_string(&field.ty);
        index += &format!("  - {}: {}\n", &field_ident.to_string(), type_name);
    });
    index
}

fn compute_type_string(ty: &Type) -> String {
    match ty {
        Type::Path(ref typepath) if typepath.qself.is_none() => typepath
            .path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                if let PathArguments::AngleBracketed(ref arguments) = &v.arguments {
                    if arguments.args.len() > 1 {
                        abort_call_site!(
                            "generic types parameterized on more than one type are not supported"
                        )
                    }
                    if let Some(args) = arguments.args.first() {
                        if let GenericArgument::Type(inner_type) = args {
                            acc.push_str(
                                &("<".to_owned() + &compute_type_string(inner_type) + ">"),
                            );
                        }
                    }
                }
                acc
            }),
        _ => abort_call_site!("choices supports only simple types (syn::Type::Path) for fields"),
    }
}
