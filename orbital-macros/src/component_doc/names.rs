use syn::Ident;

pub fn doc_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}_DOC", fn_name.to_string().to_uppercase()),
        fn_name.span(),
    )
}

pub fn props_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}_PROPS", fn_name.to_string().to_uppercase()),
        fn_name.span(),
    )
}

pub fn description_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}_DESCRIPTION", fn_name.to_string().to_uppercase()),
        fn_name.span(),
    )
}

pub fn best_practices_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("{}_BEST_PRACTICES", fn_name.to_string().to_uppercase()),
        fn_name.span(),
    )
}

pub fn variant_code_const_name(fn_name: &Ident, slug: &str) -> Ident {
    let upper = fn_name.to_string().to_uppercase();
    let slug_upper = slug.replace('-', "_").to_uppercase();
    Ident::new(
        &format!("{upper}_PREVIEW_EXAMPLE_{slug_upper}_CODE"),
        fn_name.span(),
    )
}

pub fn variant_description_const_name(fn_name: &Ident, slug: &str) -> Ident {
    let upper = fn_name.to_string().to_uppercase();
    let slug_upper = slug.replace('-', "_").to_uppercase();
    Ident::new(
        &format!("{upper}_PREVIEW_EXAMPLE_{slug_upper}_DESCRIPTION"),
        fn_name.span(),
    )
}

pub fn default_example_title_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!(
            "{}_PREVIEW_DEFAULT_TITLE",
            fn_name.to_string().to_uppercase()
        ),
        fn_name.span(),
    )
}

pub fn default_example_description_const_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!(
            "{}_PREVIEW_DEFAULT_DESCRIPTION",
            fn_name.to_string().to_uppercase()
        ),
        fn_name.span(),
    )
}

pub fn preview_component_name(fn_name: &Ident) -> Ident {
    Ident::new(&format!("{fn_name}Preview"), fn_name.span())
}

pub fn preview_registration_static_name(fn_name: &Ident) -> Ident {
    Ident::new(
        &format!(
            "{}_PREVIEW_REGISTRATION",
            fn_name.to_string().to_uppercase()
        ),
        fn_name.span(),
    )
}

pub fn variant_component_name(fn_name: &Ident, slug: &str) -> Ident {
    let slug_pascal = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>();
    Ident::new(&format!("{fn_name}Preview{slug_pascal}"), fn_name.span())
}

pub fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
