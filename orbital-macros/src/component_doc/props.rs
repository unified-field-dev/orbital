use quote::quote;
use syn::{FnArg, PatType};

use super::doc_raw;

pub struct ExtractedProp {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

/// Collapse whitespace from `quote!` type output (`MaybeProp < String >` → `MaybeProp<String>`).
fn normalize_type_name(raw: &str) -> String {
    raw.split_whitespace().collect::<String>()
}

pub fn extract_props(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> Vec<ExtractedProp> {
    let mut props = Vec::new();

    for input in inputs {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = input {
            let param_name = if let syn::Pat::Ident(pat_ident) = &**pat {
                pat_ident.ident.to_string()
            } else {
                continue;
            };

            let param_type = normalize_type_name(&quote::quote! { #ty }.to_string());
            let param_doc = doc_raw::extract_doc_comments(attrs).trim().to_string();

            props.push(ExtractedProp {
                name: param_name,
                type_name: param_type,
                description: super::link_normalize::normalize_rustdoc_links(&param_doc),
            });
        }
    }

    props
}

pub fn emit_props_const(props: &[ExtractedProp], props_ty: &syn::Path) -> proc_macro2::TokenStream {
    if props.is_empty() {
        return quote! { &[] };
    }

    let entries = props.iter().map(|p| {
        let name = &p.name;
        let type_name = &p.type_name;
        let description = &p.description;
        quote! {
            #props_ty {
                name: #name,
                type_name: #type_name,
                description: #description,
            }
        }
    });

    quote! { &[#(#entries),*] }
}

#[cfg(test)]
mod tests {
    use super::{extract_props, normalize_type_name};
    use syn::parse_quote;

    #[test]
    fn normalize_type_collapses_quote_whitespace() {
        assert_eq!(
            normalize_type_name("MaybeProp < String >"),
            "MaybeProp<String>"
        );
    }

    #[test]
    fn extract_props_splits_name_type_description() {
        let inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = parse_quote! {
            /// Leading icon from the icondata catalog.
            #[prop(optional, into)]
            icon: MaybeProp<icondata::IconData>,
            /// Button label text.
            children: leptos::Children
        };
        let props = extract_props(&inputs);
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].name, "icon");
        assert!(props[0].type_name.contains("MaybeProp"));
        assert_eq!(
            props[0].description,
            "Leading icon from the icondata catalog."
        );
        assert_eq!(props[1].name, "children");
        assert_eq!(props[1].description, "Button label text.");
    }
}
