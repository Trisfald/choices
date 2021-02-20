//! Implementation of the configuration HTTP server built upon `warp`.

use crate::attributes::Attributes;
use crate::index::{compute_index, IndexData};
use crate::{GenChoicesOutput, DEFAULT_ROOT_PATH};
use proc_macro2::{Ident, TokenStream};
#[cfg(not(feature = "json"))]
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

pub(crate) fn gen_choices(
    fields: &Punctuated<Field, Comma>,
    struct_attrs: &[Attribute],
) -> GenChoicesOutput {
    let attrs = Attributes::from_struct(struct_attrs);
    let root_path = attrs.root_path.unwrap_or(quote! { #DEFAULT_ROOT_PATH });

    let index_data = compute_index(fields, attrs.json);
    let fields_resources = gen_fields_resources(fields, &root_path, attrs.json);
    let fields_resources_mutable = gen_fields_resources_mutable(fields, &root_path, attrs.json);

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
    json: bool,
) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let field_attr = Attributes::from_field(field);
            if field_attr.skip {
                None
            } else {
                let field_ident = field
                    .ident
                    .as_ref()
                    .expect("unnamed fields are not supported!");
                let field_name = field_ident.to_string();
                let get_reply = if json {
                    get_reply_for_field_json(field_ident, quote! { $self })
                } else {
                    get_reply_for_field_text(field_ident, quote! { $self })
                };
                Some(quote! {
                    choices::warp::path!(#root_path / #field_name).and(choices::warp::path::end()).#get_reply
                })
            }
        })
        .collect()
}

/// Generates the warp::reply map() for GET field, to return text.
///
/// `field_ident` is the ident of the field, `access_pattern` represents the way the field
/// can be accessed.
fn get_reply_for_field_text(field_ident: &Ident, access_pattern: TokenStream) -> TokenStream {
    let content_type_header = crate::constants::CONTENT_TYPE_HEADER;
    let content_type_value = crate::constants::CONTENT_TYPE_TEXT;
    quote! {
        map(choices::warp::reply)
        .map(move |reply| {
            choices::warp::reply::with_header(
                #access_pattern.#field_ident.body_string(),
                #content_type_header,
                #content_type_value
            )
        })
    }
}

/// Generates the warp::reply map() for GET field, to return json.
///
/// `field_ident` is the ident of the field, `_access_pattern` represents the way the field
/// can be accessed.
fn get_reply_for_field_json(_field_ident: &Ident, _access_pattern: TokenStream) -> TokenStream {
    #[cfg(not(feature = "json"))]
    abort_call_site!("you must enable the choices feature `json` in order to use it in a macro");

    #[cfg(feature = "json")]
    {
        let content_type_header = crate::constants::CONTENT_TYPE_HEADER;
        let content_type_value = crate::constants::CONTENT_TYPE_JSON;
        quote! {
            map(choices::warp::reply)
            .map(move |reply| {
                let body = choices::serde_json::to_string(&#_access_pattern.#_field_ident);
                let status = if body.is_ok() {
                    choices::warp::http::StatusCode::OK
                } else {
                    choices::warp::http::StatusCode::INTERNAL_SERVER_ERROR
                };
                choices::warp::reply::with_status(
                    choices::warp::reply::with_header(
                        match body {
                            Ok(v) => v,
                            Err(err) => format!("\"{}\"", err.to_string())
                        },
                        #content_type_header,
                        #content_type_value
                    ),
                    status
                )
            })
        }
    }
}

/// Generates the mutable fields' HTTP resources, i.e. the GET methods to retrieve the value of
/// fields from an Arc<Mutex<T>> and the PUT methods to modify such fields.
fn gen_fields_resources_mutable(
    fields: &Punctuated<Field, Comma>,
    root_path: &TokenStream,
    json: bool,
) -> Vec<TokenStream> {
    fields.iter().filter_map(|field| {
        let field_attr = Attributes::from_field(field);
        if field_attr.skip {
            None
        } else {
            let field_ident = field
                .ident
                .as_ref()
                .expect("unnamed fields are not supported!");
            let field_name = field_ident.to_string();
            let setter_ident = quote::format_ident!("set_{}", field_ident);
            let arg_type = &field.ty;
            let (get_reply, put_reply) = if json {
                (get_reply_for_field_json(field_ident, quote! { choices.lock().unwrap() }),
                put_reply_for_field_json(arg_type, &setter_ident))
            } else {
                (get_reply_for_field_text(field_ident, quote! { choices.lock().unwrap() }),
                put_reply_for_field_text(arg_type, &setter_ident))
            };
            Some(quote! {{
                let choices = $choices.clone();
                let get = choices::warp::get().#get_reply;
                let choices = $choices.clone();
                let put = choices::warp::put()
                    .and(choices::warp::body::content_length_limit(1024 * 16))
                    .#put_reply;
                    choices::warp::path!(#root_path / #field_name).and(choices::warp::path::end()).and(get.or(put))
            }})
        }
    }).collect()
}

/// Generates the warp::reply map() for PUT field, accepting text.
fn put_reply_for_field_text(arg_type: &Type, setter_ident: &Ident) -> TokenStream {
    quote! {
        and(choices::warp::body::bytes())
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
        })
    }
}

/// Generates the warp::reply map() for PUT field, accepting json.
fn put_reply_for_field_json(_arg_type: &Type, _setter_ident: &Ident) -> TokenStream {
    #[cfg(not(feature = "json"))]
    abort_call_site!("you must enable the choices feature `json` in order to use it in a macro");

    #[cfg(feature = "json")]
    {
        quote! {
            and(choices::warp::body::json())
            .map(move |value: #_arg_type| {
                choices.lock().unwrap().#_setter_ident(value);
                choices::warp::reply::with_status("".to_string(), choices::warp::http::StatusCode::OK)
            })
        }
    }
}

/// Generates the macros used to build the warp filters.
fn gen_macros(
    root_path: &TokenStream,
    index_data: IndexData,
    fields_resources: &[TokenStream],
    fields_resources_mutable: &[TokenStream],
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
        pub fn filter(
            &'static self,
        ) -> choices::warp::filters::BoxedFilter<(impl choices::warp::Reply,)> {
            use choices::warp::Filter;
            create_filter!(self).boxed()
        }

        /// If you want more control over the http server instance you can use this
        /// function to retrieve the configuration's `warp::Filter`.
        pub fn filter_mutable(
            choices: std::sync::Arc<std::sync::Mutex<Self>>,
        ) -> choices::warp::filters::BoxedFilter<(impl choices::warp::Reply,)> {
            use choices::warp::Filter;
            create_filter_mutable!(choices).boxed()
        }
    }
}

/// Generates the fields' setters.
fn gen_setters(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let setters = fields.iter().map(|field| {
        let field_attr = Attributes::from_field(field);
        if field_attr.skip {
            None
        } else {
            let field_ident = field
                .ident
                .as_ref()
                .expect("unnamed fields are not supported!");
            let setter_ident = quote::format_ident!("set_{}", field_ident);
            let arg_type = &field.ty;
            // Generate the callback tokenstream.
            let callback = if let Some(callback) = field_attr.on_set {
                quote! { #callback(&value); }
            } else {
                quote! {}
            };
            // Output the setter tokenstream.
            Some(quote! {
                pub fn #setter_ident(&mut self, value: impl Into<#arg_type>) {
                    let value = value.into();
                    #callback
                    self.#field_ident = value;
                }
            })
        }
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

        async fn run_mutable<T: Into<std::net::SocketAddr> + Send>(
            choices: std::sync::Arc<std::sync::Mutex<Self>>,
            addr: T,
        ) {
            let filter = create_filter_mutable!(choices);
            choices::warp::serve(filter).run(addr).await
        }
    }
}
