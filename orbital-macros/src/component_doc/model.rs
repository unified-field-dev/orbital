#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExampleVariant {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub code: String,
    pub render: bool,
    pub is_default: bool,
    pub code_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentDocModel {
    pub summary: String,
    pub when_to_use: String,
    pub usage: String,
    pub best_practices: String,
    pub description: String,
    pub examples: Vec<ExampleVariant>,
}

impl ComponentDocModel {
    pub fn default_variant(&self) -> Option<&ExampleVariant> {
        self.examples
            .iter()
            .find(|v| v.is_default)
            .or_else(|| self.examples.iter().find(|v| v.render && !v.code_only))
    }

    pub fn card_variants(&self) -> Vec<&ExampleVariant> {
        let default_slug = self.default_variant().map(|v| v.slug.as_str());
        self.examples
            .iter()
            .filter(|v| {
                v.render && !v.code_only && default_slug.map(|s| v.slug != s).unwrap_or(true)
            })
            .collect()
    }
}
