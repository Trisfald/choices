use crate::util::compute_type_string;
use syn::{punctuated::Punctuated, token::Comma, *};

/// Returns the body of the configuration index page.
pub(crate) fn compute_index_string(fields: &Punctuated<Field, Comma>) -> String {
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
