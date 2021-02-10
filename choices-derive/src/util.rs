use proc_macro_error::abort_call_site;
use syn::*;

/// Returns a string representation of a type.
pub(crate) fn compute_type_string(ty: &Type) -> String {
    match ty {
        Type::Path(ref typepath) if typepath.qself.is_none() => typepath
            .path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                if let PathArguments::AngleBracketed(ref arguments) = &v.arguments {
                    if arguments.args.len() >= 1 {
                        acc.push_str("<");
                        for (i, arg) in arguments.args.iter().enumerate() {
                            if let GenericArgument::Type(inner_type) = arg {
                                if i != 0 {
                                    acc.push_str(", ");
                                }
                                acc.push_str(&compute_type_string(&inner_type));
                            } else {
                                abort_call_site!(
                                    "choices supports only generic arguments of kind Type"
                                )
                            }
                        }
                        acc.push_str(">");
                    }
                }
                acc
            }),
        _ => abort_call_site!("choices supports only simple types (syn::Type::Path) for fields"),
    }
}
