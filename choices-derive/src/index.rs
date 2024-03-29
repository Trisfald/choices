use crate::util::compute_type_string;
use crate::{attributes::Attributes, DEFAULT_ROOT_MESSAGE};
use derive_new::new;
use proc_macro2::TokenStream;
#[cfg(not(feature = "json"))]
use proc_macro_error::abort_call_site;
use syn::{punctuated::Punctuated, token::Comma, *};

#[derive(new)]
pub(crate) struct IndexData {
    pub(crate) body: String,
    pub(crate) content_type: &'static str,
}

/// Returns the body of the configuration index page.
pub(crate) fn compute_index(
    fields: &Punctuated<Field, Comma>,
    json: bool,
    root_message: &Option<TokenStream>,
) -> IndexData {
    if json {
        compute_index_json(fields)
    } else {
        compute_index_text(fields, root_message)
    }
}

fn compute_index_text(
    fields: &Punctuated<Field, Comma>,
    root_message: &Option<TokenStream>,
) -> IndexData {
    let mut index = if let Some(ts) = root_message {
        let ts_str = ts.to_string();
        ts_str[1..ts_str.len() - 1].to_string()
    } else {
        DEFAULT_ROOT_MESSAGE.to_string()
    };
    if !index.is_empty() {
        index += "\n";
    }
    fields.iter().for_each(|field| {
        let field_attr = Attributes::from_field(field);
        if !field_attr.skip {
            let field_ident = field
                .ident
                .as_ref()
                .expect("unnamed fields are not supported!");
            let type_name = compute_type_string(&field.ty);
            index += &format!("  - {}: {}\n", &field_ident.to_string(), type_name);
        }
    });
    IndexData::new(index, crate::constants::CONTENT_TYPE_TEXT)
}

fn compute_index_json(_fields: &Punctuated<Field, Comma>) -> IndexData {
    #[cfg(not(feature = "json"))]
    abort_call_site!("you must enable the choices feature `json` in order to use it in a macro");

    #[cfg(feature = "json")]
    {
        use serde::Serialize;

        #[derive(Serialize, new)]
        struct Entry {
            name: String,
            r#type: String,
        }

        let v: Vec<_> = _fields
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
                    Some(Entry::new(
                        field_ident.to_string(),
                        compute_type_string(&field.ty),
                    ))
                }
            })
            .collect();
        IndexData::new(
            serde_json::to_string(&v).expect("failed to serialize index json"),
            crate::constants::CONTENT_TYPE_JSON,
        )
    }
}
