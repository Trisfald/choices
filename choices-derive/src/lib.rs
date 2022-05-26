//! Proc macros for the `choices` crate.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate proc_macro;

mod attributes;
mod constants;
mod index;
mod util;
mod warp;

use derive_new::new;
use proc_macro2::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error, set_dummy};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

pub(crate) const DEFAULT_ROOT_PATH: &str = "config";
pub(crate) const DEFAULT_ROOT_MESSAGE: &str = "Available configuration options:";

/// Output of choice's generator.
#[derive(new)]
pub(crate) struct GenChoicesOutput {
    macro_block: TokenStream,
    impl_block: TokenStream,
    trait_block: TokenStream,
}

/// Generates the `Choices` impl.
#[proc_macro_derive(Choices, attributes(choices))]
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
        #[choices::async_trait]
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
    attrs: &[Attribute],
) -> TokenStream {
    let choices = warp::gen_choices(fields, attrs);

    let macro_block = choices.macro_block;
    let impl_block = choices.impl_block;
    let trait_block = choices.trait_block;

    quote! {
        #macro_block

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
            #impl_block
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
        #[choices::async_trait]
        impl ::choices::Choices for #name {
            #trait_block
        }
    }
}
