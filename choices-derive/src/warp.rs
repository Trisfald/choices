//! Implementation of the configuration HTTP server built upon `warp`.

use crate::attributes::Attributes;
use crate::index::{compute_index, IndexData};
use crate::{GenChoicesOutput, DEFAULT_ROOT_PATH};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

pub(crate) fn gen_choices(
    fields: &Punctuated<Field, Comma>,
    struct_attrs: &[Attribute],
) -> GenChoicesOutput {
    let attrs = Attributes::from_struct(struct_attrs);
    let root_path = attrs.root_path.unwrap_or(quote! { #DEFAULT_ROOT_PATH });

    let index_data = compute_index(fields, attrs.json);
    let fields_resources = gen_fields_resources(fields, &root_path);
    let fields_resources_mutable = gen_fields_resources_mutable(fields, &root_path);

    let macros_tk = gen_macros(
        &root_path,
        index_data,
        &fields_resources,
        &fields_resources_mutable,
    );
    let impl_tk = gen_impl(fields);
    let trait_tk = gen_trait();

    GenChoicesOutput::new(macros_tk, impl_tk, trait_tk)
}

/// Generates the fields' HTTP resources, i.e. the GET methods to retrieve the value of
/// fields from an immutable `self`.
fn gen_fields_resources(
    fields: &Punctuated<Field, Comma>,
    root_path: &TokenStream,
) -> Vec<Option<TokenStream>> {
    fields.iter().map(|field| {
        let field_ident = field
            .ident
            .as_ref()
            .expect("unnamed fields are not supported!");
        let field_name = field_ident.to_string();
        Some(quote! {
            choices::warp::path!(#root_path / #field_name).and(choices::warp::path::end()).map(move || format!("{}", $self.#field_ident.body_string()) )
        })
    }).collect()
}

/// Generates the mutable fields' HTTP resources, i.e. the GET methods to retrieve the value of
/// fields from an Arc<Mutex<T>> and the PUT methods to modify such fields.
fn gen_fields_resources_mutable(
    fields: &Punctuated<Field, Comma>,
    root_path: &TokenStream,
) -> Vec<Option<TokenStream>> {
    fields.iter().map(|field| {
        let field_ident = field
            .ident
            .as_ref()
            .expect("unnamed fields are not supported!");
        let field_name = field_ident.to_string();
        let setter_ident = quote::format_ident!("set_{}", field_ident);
        let arg_type = &field.ty;
        Some(quote! {{
            let choices = $choices.clone();
            let get = choices::warp::get()
                .map(move || format!("{}", choices.lock().unwrap().#field_ident.body_string()));
            let choices = $choices.clone();
            let put = choices::warp::put()
                .and(choices::warp::body::content_length_limit(1024 * 16))
                .and(choices::warp::body::bytes())
                .map(move |bytes: choices::bytes::Bytes| {
                    let result: choices::ChoicesResult<#arg_type> = choices::ChoicesInput::from_chars(&bytes);
                    match result {
                        Ok(value) => {
                            choices.lock().unwrap().#setter_ident(value);
                            choices::warp::reply::with_status("".to_string(), choices::warp::http::StatusCode::OK)
                        }
                        Err(err) => {
                            choices::warp::reply::with_status(err.to_string(), choices::warp::http::StatusCode::BAD_REQUEST)
                        }
                    }
                });
                choices::warp::path!(#root_path / #field_name).and(choices::warp::path::end()).and(get.or(put))
        }})
    }).collect()
}

/// Generates the macros used to build the warp filters.
fn gen_macros(
    root_path: &TokenStream,
    index_data: IndexData,
    fields_resources: &[Option<TokenStream>],
    fields_resources_mutable: &[Option<TokenStream>],
) -> TokenStream {
    let content_type_header = crate::constants::CONTENT_TYPE_HEADER;
    let index_body = index_data.body;
    let index_content_type = index_data.content_type;

    quote! {
        macro_rules! create_filter {
            ($self:ident) => {{
                use choices::warp::Filter;
                #[allow(unused_imports)]
                use choices::ChoicesOutput;

                choices::warp::path(#root_path)
                    .and(choices::warp::path::end())
                    .map(choices::warp::reply)
                    .map(|reply| {
                        choices::warp::reply::with_header(#index_body, #content_type_header, #index_content_type)
                    })
                #( .or(#fields_resources) )*
            }};
        }

        macro_rules! create_filter_mutable {
            ($choices:ident) => {{
                use choices::warp::Filter;
                #[allow(unused_imports)]
                use choices::{ChoicesInput, ChoicesOutput};

                choices::warp::path(#root_path)
                    .and(choices::warp::path::end())
                    .map(choices::warp::reply)
                    .map(|reply| {
                        choices::warp::reply::with_header(#index_body, #content_type_header, #index_content_type)
                    })
                #( .or(#fields_resources_mutable) )*
            }};
        }
    }
}

/// Generates the struct impl block.
fn gen_impl(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let setters = gen_setters(fields);

    quote! {
        #setters

        /// If you want more control over the http server instance you can use this
        /// function to retrieve the configuration's `warp::Filter`.
        fn filter(&'static self) -> choices::warp::filters::BoxedFilter<(impl choices::warp::Reply,)> {
            use choices::warp::Filter;
            create_filter!(self).boxed()
        }

        /// If you want more control over the http server instance you can use this
        /// function to retrieve the configuration's `warp::Filter`.
        fn filter_mutable(choices: std::sync::Arc<std::sync::Mutex<Self>>) -> choices::warp::filters::BoxedFilter<(impl choices::warp::Reply,)> {
            use choices::warp::Filter;
            create_filter_mutable!(choices).boxed()
        }
    }
}

/// Generates the fields' setters.
fn gen_setters(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let setters = fields.iter().map(|field| {
        let field_ident = field
            .ident
            .as_ref()
            .expect("unnamed fields are not supported!");
        let setter_ident = quote::format_ident!("set_{}", field_ident);
        let arg_type = &field.ty;
        Some(quote! {
            fn #setter_ident(&mut self, value: impl Into<#arg_type>) {
                self.#field_ident = value.into();
            }
        })
    });
    quote! {
        #( #setters )*
    }
}

/// Generates the Choices trait impl block.
fn gen_trait() -> TokenStream {
    quote! {
        async fn run<T: Into<std::net::SocketAddr> + Send>(&'static self, addr: T) {
            let filter = create_filter!(self);
            choices::warp::serve(filter).run(addr).await
        }

        async fn run_mutable<T: Into<std::net::SocketAddr> + Send>(choices: std::sync::Arc<std::sync::Mutex<Self>>, addr: T) {
            let filter = create_filter_mutable!(choices);
            choices::warp::serve(filter).run(addr).await
        }
    }
}
