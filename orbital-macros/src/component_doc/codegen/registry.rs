use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::component_doc::attrs::ComponentDocAttrs;
use crate::component_doc::category_defaults::{
    default_category_collapsed, default_category_priority, default_group, default_group_priority,
    default_section_from_path, default_section_priority,
};
use crate::component_doc::names::preview_registration_static_name;

pub fn emit_registration(
    attrs: &ComponentDocAttrs,
    fn_name: &Ident,
    preview_name: &Ident,
    source_path: &str,
) -> TokenStream {
    let slug = attrs
        .preview_slug
        .as_ref()
        .expect("checked in is_preview_enabled");
    let label = attrs
        .preview_label
        .clone()
        .unwrap_or_else(|| fn_name.to_string());
    let category = if attrs.nav_item {
        attrs.category.clone().unwrap_or_default()
    } else {
        attrs
            .category
            .clone()
            .unwrap_or_else(|| "Components".to_string())
    };
    let section = default_section_from_path(
        source_path,
        &category,
        attrs.section.as_deref(),
        attrs.nav_item,
    );
    let section_priority =
        default_section_priority(&section, attrs.nav_item, attrs.section_priority);
    let category_priority = attrs
        .category_priority
        .unwrap_or_else(|| default_category_priority(&category));
    let category_default_collapsed = attrs
        .category_default_collapsed
        .unwrap_or_else(|| default_category_collapsed(&category));
    let group = default_group(&category, slug, attrs.group.as_deref());
    let group_priority = default_group_priority(&category, &group, attrs.group_priority);
    let nav_item = attrs.nav_item;
    let icon = attrs
        .preview_icon
        .as_ref()
        .map(|path| quote! { #path })
        .unwrap_or_else(|| quote! { icondata::AiFileOutlined });
    let reg_static = preview_registration_static_name(fn_name);

    quote! {
        #[cfg(feature = "preview")]
        pub static #reg_static: crate::preview::PreviewRegistration = crate::preview::PreviewRegistration {
            slug: #slug,
            label: #label,
            section: #section,
            section_priority: #section_priority,
            category: #category,
            category_priority: #category_priority,
            category_default_collapsed: #category_default_collapsed,
            group: #group,
            group_priority: #group_priority,
            nav_item: #nav_item,
            icon: #icon,
            render: || #preview_name().into_any(),
        };

        #[cfg(all(feature = "preview", not(target_arch = "wasm32")))]
        ::inventory::submit! {
            crate::preview::PreviewRegistration {
                slug: #slug,
                label: #label,
                section: #section,
                section_priority: #section_priority,
                category: #category,
                category_priority: #category_priority,
                category_default_collapsed: #category_default_collapsed,
                group: #group,
                group_priority: #group_priority,
                nav_item: #nav_item,
                icon: #icon,
                render: || #preview_name().into_any(),
            }
        }
    }
}
