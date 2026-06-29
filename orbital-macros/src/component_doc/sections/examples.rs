use crate::component_doc::model::ExampleVariant;
use crate::component_doc::names::slugify;

pub fn parse_example_variants(body: &str) -> Vec<ExampleVariant> {
    let chunks = split_variants(body);
    let mut variants = Vec::new();
    let mut saw_default_marker = false;

    for (title, chunk) in chunks {
        let variant = parse_variant_chunk(&title, &chunk);
        if variant.is_default {
            saw_default_marker = true;
        }
        variants.push(variant);
    }

    if !saw_default_marker {
        if let Some(first) = variants.iter_mut().find(|v| v.render && !v.code_only) {
            first.is_default = true;
        }
    }

    variants
}

fn split_variants(body: &str) -> Vec<(String, String)> {
    let mut chunks = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();

    for line in body.lines() {
        if let Some(title) = variant_heading(line) {
            if let Some(prev) = current_title.take() {
                chunks.push((prev, current_lines.join("\n")));
                current_lines.clear();
            }
            current_title = Some(title);
        } else {
            current_lines.push(line.to_string());
        }
    }

    if let Some(title) = current_title {
        chunks.push((title, current_lines.join("\n")));
    }

    chunks
}

fn variant_heading(line: &str) -> Option<String> {
    let trimmed = line.trim();
    trimmed
        .strip_prefix("##")
        .and_then(|rest| rest.strip_prefix(' '))
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map(strip_markers_from_heading)
}

fn strip_markers_from_heading(title: &str) -> String {
    title
        .replace("<!-- preview -->", "")
        .replace("<!-- default -->", "")
        .replace("<!-- code-only -->", "")
        .trim()
        .to_string()
}

fn parse_variant_chunk(title: &str, chunk: &str) -> ExampleVariant {
    let heading_line = format!("## {title}");
    let has_preview_marker =
        chunk.contains("<!-- preview -->") || heading_line.contains("<!-- preview -->");
    let is_default = chunk.contains("<!-- default -->");
    let code_only = chunk.contains("<!-- code-only -->");

    let (description, code, _fence_info) = extract_fence_and_description(chunk);
    // `ignore` in the fence skips rustdoc doctests only; live preview still renders.
    let render = !code_only && has_preview_marker;

    ExampleVariant {
        title: title.to_string(),
        slug: slugify(title),
        description,
        code,
        render,
        is_default,
        code_only,
    }
}

fn extract_fence_and_description(chunk: &str) -> (String, String, String) {
    let mut description_lines = Vec::new();
    let mut in_fence = false;
    let mut fence_info = String::new();
    let mut code_lines = Vec::new();

    for line in chunk.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_fence {
                break;
            }
            in_fence = true;
            fence_info = trimmed.trim_start_matches('`').to_string();
            continue;
        }
        if in_fence {
            code_lines.push(line.to_string());
        } else if !trimmed.is_empty() && trimmed != "<!-- preview -->" {
            description_lines.push(line.to_string());
        }
    }

    (
        description_lines.join("\n").trim().to_string(),
        code_lines.join("\n").trim().to_string(),
        fence_info,
    )
}
