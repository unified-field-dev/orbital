mod constants;
mod preview;
mod registry;

#[cfg(test)]
mod expand_smoke;

use quote::quote;
use syn::ItemFn;

use super::attrs::ComponentDocAttrs;
use super::doc_raw;
use super::props;
use super::sections;

use syn::spanned::Spanned;

pub fn expand(attrs: &ComponentDocAttrs, input_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut attrs = attrs.clone();
    let mut input_fn = input_fn.clone();
    doc_raw::sanitize_doc_attrs_for_doctest(&mut input_fn.attrs);

    let fn_name = &input_fn.sig.ident;
    let source_path = super::category_defaults::caller_source_path(input_fn.span());
    if attrs.props_import.is_none() && source_path.contains("orbital-motion/") {
        attrs.props_import = Some(syn::parse_quote!(crate::preview::ComponentPropDoc));
    }
    let doc_comments = doc_raw::extract_doc_comments(&input_fn.attrs);
    let props = props::extract_props(&input_fn.sig.inputs);
    let model = sections::parse_doc_string(&doc_comments);

    let base = constants::emit_base(&attrs, fn_name, &doc_comments, &props, &model);
    let preview = if attrs.is_preview_enabled() {
        preview::emit_preview(&attrs, fn_name, &model, &doc_comments, &source_path)
    } else {
        quote! {}
    };

    let expanded = quote! {
        #input_fn
        #base
        #preview
    };

    expanded
}
