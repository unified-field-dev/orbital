use quote::quote;
use syn::{Ident, Path};

use crate::component_doc::attrs::ComponentDocAttrs;
use crate::component_doc::model::ComponentDocModel;
use crate::component_doc::names::{
    best_practices_const_name, default_example_description_const_name,
    default_example_title_const_name, description_const_name, doc_const_name, props_const_name,
    variant_code_const_name, variant_description_const_name,
};

fn default_props_ty() -> Path {
    syn::parse_quote!(::orbital_base_components::ComponentPropDoc)
}

pub fn emit_base(
    attrs: &ComponentDocAttrs,
    fn_name: &Ident,
    doc_comments: &str,
    props: &[crate::component_doc::props::ExtractedProp],
    model: &ComponentDocModel,
) -> proc_macro2::TokenStream {
    let doc_const = doc_const_name(fn_name);
    let props_const = props_const_name(fn_name);
    let props_ty = attrs.props_import.clone().unwrap_or_else(default_props_ty);
    let props_slice = crate::component_doc::props::emit_props_const(props, &props_ty);
    let description_const = description_const_name(fn_name);
    let best_practices_const = best_practices_const_name(fn_name);

    let variant_code_consts = model.examples.iter().map(|variant| {
        let const_name = variant_code_const_name(fn_name, &variant.slug);
        let code = &variant.code;
        quote! {
            #[doc(hidden)]
            #[cfg(feature = "preview")]
            pub const #const_name: &str = #code;
        }
    });

    let variant_description_consts = model
        .examples
        .iter()
        .filter(|v| !v.description.is_empty())
        .map(|variant| {
            let const_name = variant_description_const_name(fn_name, &variant.slug);
            let description = &variant.description;
            quote! {
                #[doc(hidden)]
                #[cfg(feature = "preview")]
                pub const #const_name: &str = #description;
            }
        });

    let default_example_consts = model.default_variant().map(|default| {
        let title_const = default_example_title_const_name(fn_name);
        let desc_const = default_example_description_const_name(fn_name);
        let title = &default.title;
        let description = &default.description;
        quote! {
            #[doc(hidden)]
            #[cfg(feature = "preview")]
            pub const #title_const: &str = #title;
            #[doc(hidden)]
            #[cfg(feature = "preview")]
            pub const #desc_const: &str = #description;
        }
    });

    let description = &model.description;
    let best_practices = &model.best_practices;

    quote! {
        #[doc(hidden)]
        #[cfg(feature = "preview")]
        pub const #doc_const: &str = #doc_comments;

        #[doc(hidden)]
        #[cfg(feature = "preview")]
        pub const #props_const: &[#props_ty] = #props_slice;

        #[doc(hidden)]
        #[cfg(feature = "preview")]
        pub const #description_const: &str = #description;

        #[doc(hidden)]
        #[cfg(feature = "preview")]
        pub const #best_practices_const: &str = #best_practices;

        #(#variant_code_consts)*

        #(#variant_description_consts)*

        #default_example_consts
    }
}
