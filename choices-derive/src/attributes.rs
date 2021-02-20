//! Parsing of macro attributes.

use proc_macro2::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{
    self,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, Ident, LitStr, Token,
};

/// All types of attribute available in choices.
#[allow(clippy::large_enum_variant)]
pub(crate) enum ChoicesAttribute {
    // single-identifier attributes
    Json(Ident),
    Skip(Ident),
    // ident = "string literal"
    RootPath(Ident, LitStr),
    // ident = arbitrary_expr
    OnSet(Ident, Expr),
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
                match input.parse::<Expr>() {
                    Ok(expr) => {
                        if name_str == "on_set" {
                            Ok(OnSet(name, expr))
                        } else {
                            abort!(name, "unexpected attribute: {}", name_str);
                        }
                    }
                    Err(_) => abort! {
                        assign_token,
                        "expected `string literal` or `expression` after `=`"
                    },
                }
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            abort!(name, "unexpected attribute: {}", name_str);
        } else {
            // Attributes represented with a sole identifier.
            match name_str.as_ref() {
                "json" => Ok(Json(name)),
                "skip" => Ok(Skip(name)),
                _ => abort!(name, "unexpected attribute: {}", name_str),
            }
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
    pub(crate) json: bool,
    pub(crate) on_set: Option<Expr>,
    pub(crate) skip: bool,
}

impl Attributes {
    fn new() -> Self {
        Self {
            root_path: None,
            json: false,
            on_set: None,
            skip: false,
        }
    }

    fn push_attrs(&mut self, attrs: &[Attribute], from_struct: bool) {
        use ChoicesAttribute::*;

        for attr in parse_choices_attributes(attrs) {
            match attr {
                Json(ident) => {
                    if !from_struct {
                        abort!(ident, "#[choices(json)] can be used only on struct level");
                    }
                    self.json = true;
                }
                RootPath(ident, path) => {
                    if !from_struct {
                        abort!(ident, "#[choices(path)] can be used only on struct level");
                    }
                    self.root_path = Some(quote!(#path));
                }
                OnSet(ident, expr) => {
                    if from_struct {
                        abort!(ident, "#[choices(on_set)] can be used only on field level");
                    }
                    self.on_set = Some(expr);
                }
                Skip(ident) => {
                    if from_struct {
                        abort!(ident, "#[choices(skip)] can be used only on field level");
                    }
                    self.skip = true;
                }
            }
        }
    }

    pub(crate) fn from_struct(attrs: &[Attribute]) -> Self {
        let mut res = Self::new();
        res.push_attrs(attrs, true);
        res
    }

    pub(crate) fn from_field(field: &syn::Field) -> Self {
        let mut res = Self::new();
        res.push_attrs(&field.attrs, false);
        res
    }
}
