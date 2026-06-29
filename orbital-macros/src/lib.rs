//! Proc macros for Orbital design-system documentation and routing helpers.

use proc_macro::TokenStream;

mod component_doc;
mod routes;
mod routes_extract;
mod write_css_vars;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};

/// Extract documentation and props from a component for preview tooling.
#[proc_macro_attribute]
pub fn component_doc(attr: TokenStream, input: TokenStream) -> TokenStream {
    component_doc::expand_component_doc(attr, input)
}

struct RouteComponents {
    components: Vec<syn::Ident>,
}

impl Parse for RouteComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut components = Vec::new();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            components.push(ident);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(RouteComponents { components })
    }
}

/// Include multiple route components in a Routes view.
#[proc_macro]
pub fn orbital_routes(input: TokenStream) -> TokenStream {
    let RouteComponents { components } = parse_macro_input!(input);
    let route_components = components.iter().map(|ident| {
        quote! { <#ident /> }
    });
    quote! { #(#route_components)* }.into()
}

/// Extract typed path constants from a route component's `view!` body.
#[proc_macro_attribute]
pub fn orbital_routes_extract(attr: TokenStream, input: TokenStream) -> TokenStream {
    routes_extract::expand_routes(attr, input)
}

/// Derives `write_css_vars` for theme token structs (snake_case field → `--camelCase` CSS var).
#[proc_macro_derive(WriteCSSVars)]
pub fn write_css_vars(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    write_css_vars::expand_write_css_vars(input).into()
}
