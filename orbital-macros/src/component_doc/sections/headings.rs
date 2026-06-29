use super::super::model::ComponentDocModel;
use super::examples;

pub struct RawSection {
    pub heading: Option<String>,
    pub body: String,
}

pub fn split_top_level_sections(raw: &str) -> Vec<RawSection> {
    let mut sections = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();

    for line in raw.lines() {
        if let Some(title) = top_level_heading(line) {
            if current_heading.is_some() || !current_lines.is_empty() {
                sections.push(RawSection {
                    heading: current_heading.take(),
                    body: current_lines.join("\n").trim().to_string(),
                });
                current_lines.clear();
            }
            current_heading = Some(title);
        } else {
            current_lines.push(line.to_string());
        }
    }

    sections.push(RawSection {
        heading: current_heading,
        body: current_lines.join("\n").trim().to_string(),
    });

    sections
}

fn top_level_heading(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("##") {
        return None;
    }
    trimmed
        .strip_prefix('#')
        .and_then(|rest| rest.strip_prefix(' '))
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .map(str::to_string)
}

pub fn build_model(sections: Vec<RawSection>) -> ComponentDocModel {
    let mut model = ComponentDocModel::default();

    for section in sections {
        match section.heading.as_deref() {
            None => model.summary = section.body,
            Some(title) if is_when_to_use(title) => model.when_to_use = section.body,
            Some(title) if is_usage(title) => model.usage = section.body,
            Some(title) if is_best_practices(title) => model.best_practices = section.body,
            Some(title) if is_examples(title) => {
                model.examples = examples::parse_example_variants(&section.body);
            }
            Some(_) => {}
        }
    }

    model.description = compose_description(&model);
    normalize_model_links(&mut model);
    model
}

fn normalize_model_links(model: &mut ComponentDocModel) {
    use super::super::link_normalize::normalize_rustdoc_links;

    model.summary = normalize_rustdoc_links(&model.summary);
    model.when_to_use = normalize_rustdoc_links(&model.when_to_use);
    model.usage = normalize_rustdoc_links(&model.usage);
    model.description = normalize_rustdoc_links(&model.description);
    model.best_practices = normalize_rustdoc_links(&model.best_practices);
    for example in &mut model.examples {
        example.description = normalize_rustdoc_links(&example.description);
    }
}

fn is_when_to_use(title: &str) -> bool {
    title.eq_ignore_ascii_case("when to use")
}

fn is_usage(title: &str) -> bool {
    title.eq_ignore_ascii_case("usage")
}

fn is_best_practices(title: &str) -> bool {
    title.eq_ignore_ascii_case("best practices") || section_body_is_bold_best_practices(title)
}

fn section_body_is_bold_best_practices(title: &str) -> bool {
    title.starts_with("**Best Practices:**") || title.starts_with("**Best practices:**")
}

fn is_examples(title: &str) -> bool {
    title.eq_ignore_ascii_case("examples") || title.eq_ignore_ascii_case("example")
}

fn strip_fenced_code_blocks(text: &str) -> String {
    let mut out = Vec::new();
    let mut in_fence = false;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push(line.to_string());
        }
    }

    out.join("\n").trim().to_string()
}

fn compose_description(model: &ComponentDocModel) -> String {
    let mut parts = Vec::new();
    if !model.summary.is_empty() {
        parts.push(strip_fenced_code_blocks(&model.summary));
    }
    if !model.when_to_use.is_empty() {
        parts.push(strip_fenced_code_blocks(&model.when_to_use));
    }
    if !model.usage.is_empty() {
        parts.push(strip_fenced_code_blocks(&model.usage));
    }
    parts
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}
