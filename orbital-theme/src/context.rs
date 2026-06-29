use leptos::prelude::*;

use crate::Theme;

#[derive(Clone, Copy)]
pub struct ThemeInjection {
    pub theme: RwSignal<Theme>,
    pub dir: Option<RwSignal<crate::Direction>>,
    id: StoredValue<String>,
}

impl ThemeInjection {
    pub fn id(&self) -> String {
        self.id.get_value()
    }

    pub fn use_theme(default: impl Fn() -> Theme) -> ReadSignal<Theme> {
        use_context::<Self>()
            .map_or_else(|| RwSignal::new(default()).split().0, |c| c.theme.split().0)
    }

    pub fn use_rw_theme() -> RwSignal<Theme> {
        expect_context::<Self>().theme
    }

    pub(crate) fn new(
        theme: RwSignal<Theme>,
        dir: Option<RwSignal<crate::Direction>>,
        id: String,
    ) -> Self {
        Self {
            theme,
            dir,
            id: StoredValue::new(id),
        }
    }
}

pub fn scoped_selector(id: &str) -> String {
    format!(".orbital-theme-provider[data-orbital-theme-id=\"{id}\"]")
}

pub fn scoped_css(id: &str, css_vars: &str) -> String {
    format!("{} {{{css_vars}}}", scoped_selector(id))
}
