use syn::Attribute;

/// Rewrite ` ```rust` fences to ` ```rust,ignore` so Leptos-generated Props / helper
/// doc copies do not become failing doctests. Live preview codegen still renders
/// examples marked with `<!-- preview -->`.
pub fn sanitize_doc_attrs_for_doctest(attrs: &mut [Attribute]) {
    for attr in attrs.iter_mut() {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
            let sanitized = sanitize_rust_fences(&lit.value());
            *attr = syn::parse_quote!(#[doc = #sanitized]);
        }
    }
}

fn sanitize_rust_fences(doc: &str) -> String {
    doc.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("```") {
                return line.to_string();
            }
            let info = trimmed.trim_start_matches('`');
            if info.starts_with("rust") && !info.contains("ignore") {
                let prefix_len = line.len().saturating_sub(trimmed.len());
                let prefix = &line[..prefix_len];
                return format!("{prefix}```rust,ignore");
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn extract_doc_comments(attrs: &[Attribute]) -> String {
    let mut docs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
                docs.push(meta.value());
            } else if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        docs.push(lit_str.value());
                    }
                }
            }
        }
    }

    docs.join("\n")
}
