use leptos::{ev, html, leptos_dom::helpers::window_event_listener, prelude::*};
use orbital_base_components::{BaseOverflow, OverflowDirection};
use orbital_core_components::{Button, ButtonAppearance, Menu, MenuItem, MenuTrigger};
use orbital_style::inject_style;

const OVERFLOW_STYLES: &str = r#"
.orbital-overflow {
    display: flex;
    align-items: center;
    width: 100%;
    min-width: 0;
}

.orbital-overflow--horizontal {
    flex-direction: row;
    flex-wrap: nowrap;
    overflow: hidden;
}

.orbital-overflow__items {
    display: inherit;
    flex-direction: inherit;
    flex-wrap: inherit;
    align-items: inherit;
    gap: var(--orb-space-inline-xs);
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
}

.orbital-overflow__menu {
    flex: 0 0 auto;
}

.orbital-overflow:not(.orbital-overflow--clipped) .orbital-overflow__menu {
    display: none;
}
"#;

/// Toolbar overflow with wired menu selection (opens clipped toolbar popovers/menus).
#[component]
pub fn DataTableToolbarOverflow(
    on_select: Callback<String, ()>,
    show_filter: bool,
    show_columns: bool,
    show_pivot: bool,
    show_export: bool,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-data-table-toolbar-overflow", OVERFLOW_STYLES);

    let has_overflow = RwSignal::new(false);
    let items_ref = NodeRef::<html::Div>::new();

    let measure_overflow = move || {
        if let Some(el) = items_ref.get() {
            let scroll = el.scroll_width();
            let client = el.client_width();
            has_overflow.set(scroll > client);
        }
    };

    Effect::new(move |_| {
        let _ = items_ref.get();
        measure_overflow();
        let handle = window_event_listener(ev::resize, move |_| measure_overflow());
        on_cleanup(move || handle.remove());
    });

    view! {
        <BaseOverflow
            overflow_direction=OverflowDirection::Horizontal
            has_overflow=has_overflow.read_only()
        >
            <div class="orbital-overflow__items" node_ref=items_ref>
                {children()}
            </div>
            <div class="orbital-overflow__menu">
                <Menu on_select=move |value: String| on_select.run(value)>
                    <MenuTrigger slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            attr:data-testid="data-table-toolbar-overflow"
                            attr:aria-label="More table actions"
                        >
                            "..."
                        </Button>
                    </MenuTrigger>
                    {show_filter.then(|| view! {
                        <MenuItem value="filter".to_string()>"Filters"</MenuItem>
                    })}
                    {show_columns.then(|| view! {
                        <MenuItem value="columns".to_string()>"Columns"</MenuItem>
                    })}
                    {show_pivot.then(|| view! {
                        <MenuItem value="pivot".to_string()>"Pivot"</MenuItem>
                    })}
                    {show_export.then(|| view! {
                        <MenuItem value="export".to_string()>"Export"</MenuItem>
                    })}
                </Menu>
            </div>
        </BaseOverflow>
    }
}
