mod attrs;
pub mod category_defaults;
mod codegen;
mod doc_raw;
mod link_normalize;
mod model;
mod names;
mod props;
mod sections;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Token;

/// Whether the consumer crate definitely does **not** have `feature = "name"` enabled.
///
/// Cargo sets `CARGO_CFG_FEATURE` for the consumer crate during proc-macro expansion.
/// When the variable is absent, return false so expansion still runs (preview builds stay correct).
fn consumer_lacks_feature(name: &str) -> bool {
    match std::env::var("CARGO_CFG_FEATURE") {
        Ok(features) => !features
            .split(&[',', ' '][..])
            .filter(|part| !part.is_empty())
            .any(|part| part == name),
        Err(_) => false,
    }
}

pub fn expand_component_doc(attr: TokenStream, input: TokenStream) -> TokenStream {
    if consumer_lacks_feature("preview") {
        return input;
    }

    let metas = parse_macro_input!(attr with Punctuated::<syn::Meta, Token![,]>::parse_terminated);
    let attrs = match attrs::ComponentDocAttrs::parse(&metas) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };
    let input_fn = parse_macro_input!(input as syn::ItemFn);
    codegen::expand(&attrs, &input_fn).into()
}
