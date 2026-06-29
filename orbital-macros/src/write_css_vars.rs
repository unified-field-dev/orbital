use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub(crate) fn expand_write_css_vars(input: DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Expected a struct!"),
    };

    let mut css_var_names = vec![];
    let mut field_names = vec![];
    match data.fields {
        Fields::Named(fields) => {
            for field in fields.named {
                let field_name = field.ident.unwrap();
                css_var_names.push(format!(
                    "--{}: {{}};",
                    to_camel_case(field_name.to_string())
                ));
                field_names.push(field_name);
            }
        }
        _ => panic!("Expected named fields!"),
    };

    let field_names = field_names.iter();
    quote! {
        impl #struct_name {
            pub fn write_css_vars(&self, css_vars: &mut String) {
                #(css_vars.push_str(&format!(#css_var_names, self.#field_names));)*
            }
        }
    }
}

fn to_camel_case(s: String) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = false;
    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                camel_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                camel_case.push(c.to_ascii_lowercase());
            }
        } else {
            capitalize_next = true;
        }
    }
    camel_case
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    #[test]
    fn camel_case_converts_snake_case() {
        assert_eq!(to_camel_case("font_family_base".into()), "fontFamilyBase");
    }
}
