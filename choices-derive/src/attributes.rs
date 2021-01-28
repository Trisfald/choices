//! Parsing of macro attributes.

use proc_macro2::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{
    self,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Token,
};

/// All types of attribute available in choices.
pub(crate) enum ChoicesAttribute {
    // ident = "string literal"
    RootPath(Ident, LitStr),
}

impl Parse for ChoicesAttribute {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        use ChoicesAttribute::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            let assign_token = input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                let lit_str = lit.value();

                let check_empty_lit = |s| {
                    if lit_str.is_empty() {
                        abort!(lit, "`#[choices({} = \"\")]` is not allowed", s);
                    }
                };

                match &*name_str {
                    "path" => {
                        check_empty_lit("path");
                        Ok(RootPath(name, lit))
                    }
                    _ => abort!(name, "unexpected attribute: {}", name_str),
                }
            } else {
                abort! {
                    assign_token,
                    "expected `string literal` or after `=`"
                }
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            abort!(name, "unexpected attribute: {}", name_str);
        } else {
            // Attributes represented with a sole identifier.
            abort!(name, "unexpected attribute: {}", name_str);
        }
    }
}

fn parse_choices_attributes(attrs: &[Attribute]) -> Vec<ChoicesAttribute> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("choices"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<ChoicesAttribute, Token![,]>::parse_terminated)
                .unwrap_or_abort()
        })
        .collect()
}

pub(crate) struct Attributes {
    pub(crate) root_path: Option<TokenStream>,
}

impl Attributes {
    fn new() -> Self {
        Self { root_path: None }
    }

    fn push_attrs(&mut self, attrs: &[Attribute]) {
        use ChoicesAttribute::*;

        for attr in parse_choices_attributes(attrs) {
            match attr {
                RootPath(_, path) => {
                    self.root_path = Some(quote!(#path));
                }
            }
        }
    }

    pub(crate) fn from_struct(attrs: &[Attribute]) -> Self {
        let mut res = Self::new();
        res.push_attrs(attrs);
        res
    }
}
