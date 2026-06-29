#[cfg(test)]
mod tests {
    use syn::parse_quote;

    #[test]
    fn paginator_doc_expansion_parses() {
        let attrs = crate::component_doc::attrs::ComponentDocAttrs {
            section: None,
            section_priority: None,
            category: Some("Components".into()),
            category_priority: None,
            category_default_collapsed: None,
            group: None,
            group_priority: None,
            nav_item: false,
            preview_slug: Some("paginator".into()),
            preview_label: Some("Paginator".into()),
            preview_icon: Some(syn::parse_quote! { icondata::AiStepForwardOutlined }),
            preview_import: None,
            props_import: None,
            preview_manual: false,
        };
        let item: syn::ItemFn = parse_quote! {
            /// Summary
            ///
            /// # Examples
            ///
            /// ## Demo
            /// <!-- preview -->
            /// ```rust
            /// view! { <div></div> }
            /// ```
            pub fn Paginator() -> impl leptos::IntoView {
                leptos::view! { <div></div> }
            }
        };
        let tokens = crate::component_doc::codegen::expand(&attrs, &item);
        let output = tokens.to_string();
        let parsed = syn::parse_file(&output);
        assert!(parsed.is_ok(), "{:?}", parsed.err());
        assert!(output.contains("example_anchors"));
        assert!(output.contains("default_example_id"));
        assert!(output.contains("example_id"));
    }

    #[test]
    fn props_const_emits_structured_entries() {
        let attrs = crate::component_doc::attrs::ComponentDocAttrs {
            section: None,
            section_priority: None,
            category: Some("Inputs".into()),
            category_priority: None,
            category_default_collapsed: None,
            group: None,
            group_priority: None,
            nav_item: false,
            preview_slug: Some("demo".into()),
            preview_label: Some("Demo".into()),
            preview_icon: None,
            preview_import: None,
            props_import: None,
            preview_manual: false,
        };
        let item: syn::ItemFn = parse_quote! {
            /// Summary
            pub fn DemoButton(
                /// Optional CSS class on the root element.
                #[prop(optional, into)]
                class: MaybeProp<String>,
            ) -> impl leptos::IntoView {
                leptos::view! { <button></button> }
            }
        };
        let tokens = crate::component_doc::codegen::expand(&attrs, &item);
        let output = tokens.to_string();
        assert!(output.contains("ComponentPropDoc"));
        assert!(output.contains("name"));
        assert!(output.contains("\"class\""));
        assert!(output.contains("MaybeProp<String>"));
        assert!(output.contains("Optional CSS class on the root element"));
        assert!(!output.contains("class:"));
    }
}
