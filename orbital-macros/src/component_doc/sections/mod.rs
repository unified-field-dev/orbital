mod examples;
mod headings;

use super::model::ComponentDocModel;

pub fn parse_doc_string(raw: &str) -> ComponentDocModel {
    let sections = headings::split_top_level_sections(raw);
    headings::build_model(sections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn fixture(name: &str) -> String {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/doc_strings")
            .join(name);
        fs::read_to_string(path).expect("fixture")
    }

    #[test]
    fn p01_summary_only() {
        let model = parse_doc_string(&fixture("summary_only.txt"));
        assert!(model.summary.contains("Offset/limit"));
        assert!(model.when_to_use.is_empty());
        assert!(model.examples.is_empty());
    }

    #[test]
    fn p02_all_sections() {
        let model = parse_doc_string(&fixture("all_sections.txt"));
        assert!(!model.when_to_use.is_empty());
        assert!(!model.usage.is_empty());
        assert!(!model.best_practices.is_empty());
    }

    #[test]
    fn p03_best_practices_bold_alias() {
        let model = parse_doc_string(&fixture("best_practices_bold.txt"));
        assert!(model.best_practices.contains("Reset"));
    }

    #[test]
    fn p04_example_heading_singular() {
        let model = parse_doc_string(&fixture("example_heading_singular.txt"));
        assert_eq!(model.examples.len(), 1);
    }

    #[test]
    fn p05_examples_plural() {
        let model = parse_doc_string(&fixture("examples_plural.txt"));
        assert!(model.examples.len() >= 2);
    }

    #[test]
    fn p05b_example_aliases_equivalent() {
        let singular = parse_doc_string(&fixture("example_heading_singular.txt"));
        let plural = parse_doc_string(&fixture("examples_plural.txt"));
        assert_eq!(singular.examples[0].title, plural.examples[0].title);
    }

    #[test]
    fn p06_description_excludes_fenced_code() {
        let model = parse_doc_string(&fixture("usage_with_fence.txt"));
        assert!(model.description.contains("Choose direction"));
        assert!(!model.description.contains("view!"));
        assert!(!model.description.contains("ButtonAppearance"));
        assert!(!model.description.contains("```"));
    }

    #[test]
    fn p07_description_includes_usage_prose() {
        let model = parse_doc_string(&fixture("usage_with_fence.txt"));
        assert!(model.description.contains("Flexbox layout"));
        assert!(model.description.contains("One-dimensional layouts"));
        assert!(model.description.contains("1. Choose direction"));
    }

    #[test]
    fn p08_best_practices_not_in_description() {
        let model = parse_doc_string(&fixture("usage_with_fence.txt"));
        assert!(model.best_practices.contains("margin hacks"));
        assert!(!model.description.contains("margin hacks"));
    }

    #[test]
    fn p09_example_markers() {
        let model = parse_doc_string(&fixture("examples_markers.txt"));
        assert_eq!(model.examples.len(), 3);

        let default = model
            .examples
            .iter()
            .find(|v| v.title == "Default variant")
            .unwrap();
        assert!(default.is_default);
        assert!(default.render);
        assert!(!default.code_only);

        let live = model
            .examples
            .iter()
            .find(|v| v.title == "Live preview only")
            .unwrap();
        assert!(!live.is_default);
        assert!(live.render);

        let code_only = model
            .examples
            .iter()
            .find(|v| v.title == "Code only")
            .unwrap();
        assert!(code_only.code_only);
        assert!(!code_only.render);
    }

    #[test]
    fn p10_default_variant_selection() {
        let model = parse_doc_string(&fixture("examples_markers.txt"));
        let default = model.default_variant().expect("default variant");
        assert_eq!(default.title, "Default variant");
    }
}
