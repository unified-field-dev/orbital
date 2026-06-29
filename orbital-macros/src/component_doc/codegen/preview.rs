use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::component_doc::attrs::ComponentDocAttrs;
use crate::component_doc::model::{ComponentDocModel, ExampleVariant};
use crate::component_doc::names::{
    best_practices_const_name, default_example_description_const_name,
    default_example_title_const_name, description_const_name, preview_component_name,
    props_const_name, variant_code_const_name, variant_component_name,
    variant_description_const_name,
};

use super::registry;

pub fn emit_preview(
    attrs: &ComponentDocAttrs,
    fn_name: &Ident,
    model: &ComponentDocModel,
    doc_comments: &str,
    source_path: &str,
) -> TokenStream {
    let preview_name = preview_component_name(fn_name);
    let description_const = description_const_name(fn_name);
    let best_practices_const = best_practices_const_name(fn_name);
    let props_const = props_const_name(fn_name);

    let render_count = model
        .examples
        .iter()
        .filter(|v| v.render && !v.code_only)
        .count();
    let preview_slug = attrs.preview_slug.as_deref().unwrap_or("<unknown>");
    let has_preview_markers = doc_comments.contains("<!-- preview -->");
    let empty_examples_guard = if render_count == 0 && has_preview_markers {
        let message = format!(
            "component_doc: preview_slug=\"{preview_slug}\" requires at least one live example under `# Examples` with `<!-- preview -->`"
        );
        let message = syn::LitStr::new(&message, proc_macro2::Span::call_site());
        quote! {
            compile_error!(#message);
        }
    } else {
        quote! {}
    };

    let label = attrs
        .preview_label
        .clone()
        .unwrap_or_else(|| fn_name.to_string());

    let variant_components = model
        .examples
        .iter()
        .filter(|v| v.render && !v.code_only)
        .map(|v| emit_variant_component(attrs, fn_name, v))
        .collect::<Vec<_>>();

    let default = model.default_variant();
    let default_view = default
        .map(|v| {
            let comp = variant_component_name(fn_name, &v.slug);
            quote! { #comp() }
        })
        .unwrap_or_else(|| quote! { view! { <></> } });
    let default_code = default
        .map(|v| {
            let code_const = variant_code_const_name(fn_name, &v.slug);
            quote! { #code_const }
        })
        .unwrap_or_else(|| quote! { "" });
    let default_title = default
        .map(|_| {
            let title_const = default_example_title_const_name(fn_name);
            quote! { #title_const }
        })
        .unwrap_or_else(|| quote! { "Default" });
    let default_description = default.and_then(|v| {
        if v.description.is_empty() {
            None
        } else {
            Some(default_example_description_const_name(fn_name))
        }
    });
    let default_description_prop = default_description.as_ref().map(|desc_const| {
        quote! { default_description=#desc_const }
    });

    let default_example_id_prop = default.map(|v| {
        let id = format!("example-{}", v.slug);
        let lit = syn::LitStr::new(&id, proc_macro2::Span::call_site());
        quote! { default_example_id=#lit }
    });

    let live_examples = model
        .examples
        .iter()
        .filter(|v| v.render && !v.code_only)
        .collect::<Vec<_>>();
    let example_anchor_entries = live_examples.iter().map(|v| {
        let title = syn::LitStr::new(&v.title, proc_macro2::Span::call_site());
        let slug = syn::LitStr::new(&v.slug, proc_macro2::Span::call_site());
        quote! { (#title, #slug) }
    });
    let example_anchors_prop = if live_examples.is_empty() {
        quote! {}
    } else {
        quote! { example_anchors=&[#(#example_anchor_entries),*] }
    };

    let card_variants = model.card_variants();
    let cards = card_variants.iter().map(|v| {
        let title = &v.title;
        let code_const = variant_code_const_name(fn_name, &v.slug);
        let comp = variant_component_name(fn_name, &v.slug);
        let example_id = syn::LitStr::new(
            &format!("example-{}", v.slug),
            proc_macro2::Span::call_site(),
        );
        let description_prop = if v.description.is_empty() {
            quote! {}
        } else {
            let desc_const = variant_description_const_name(fn_name, &v.slug);
            quote! { description=#desc_const }
        };
        quote! {
            <ComponentPreviewCard title=#title code=#code_const example_id=#example_id #description_prop>
                { #comp() }
            </ComponentPreviewCard>
        }
    });

    let registry_emit = registry::emit_registration(attrs, fn_name, &preview_name, source_path);

    quote! {
        #empty_examples_guard
        #(#variant_components)*

        #[cfg(feature = "preview")]
        #[component]
        pub fn #preview_name() -> impl leptos::IntoView {
            use leptos::prelude::*;
            use crate::components::{
                ComponentPreviewCard, OrbitalComponentView,
            };

            view! {
                <OrbitalComponentView
                    component_name=#label
                    component_description_md=#description_const
                    component_best_practices_md=#best_practices_const
                    component_props=#props_const
                    default_code=#default_code
                    default_example_title=#default_title
                    #default_description_prop
                    #default_example_id_prop
                    #example_anchors_prop
                    default={ #default_view }
                >
                    #(#cards)*
                </OrbitalComponentView>
            }
        }

        #registry_emit
    }
}

fn emit_variant_component(
    attrs: &ComponentDocAttrs,
    fn_name: &Ident,
    variant: &ExampleVariant,
) -> TokenStream {
    let comp_name = variant_component_name(fn_name, &variant.slug);
    let import = attrs
        .preview_import
        .as_ref()
        .map(|path| quote! { use #path; })
        .unwrap_or_else(|| quote! {});

    let code_tokens: TokenStream = match variant.code.parse() {
        Ok(tokens) => tokens,
        Err(err) => {
            let message = format!(
                "component_doc: failed to parse preview code for example \"{}\": {}",
                variant.title, err
            );
            let message = syn::LitStr::new(&message, proc_macro2::Span::call_site());
            return quote! {
                compile_error!(#message);
            };
        }
    };

    quote! {
        #[cfg(feature = "preview")]
        #[component]
        fn #comp_name() -> impl leptos::IntoView {
            use leptos::prelude::*;
            #import
            #code_tokens
        }
    }
}
