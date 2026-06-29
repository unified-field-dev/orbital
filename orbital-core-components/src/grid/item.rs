use leptos::prelude::*;
use orbital_base_components::BaseGridItem;

use super::types::GridItemConfig;

/// Grid cell wrapper for column span and offset within a [`Grid`].
///
/// Every direct child of [`Grid`] should be a `GridItem` so span and offset math resolves against the parent grid.
#[component]
pub fn GridItem(
    /// Span and offset configuration for this cell.
    #[prop(optional, into)]
    config: GridItemConfig,
    /// Optional CSS class merged onto the cell wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Cell content.
    children: Children,
) -> impl IntoView {
    view! {
        <BaseGridItem
            class=class
            column=Signal::from(config.span)
            offset=Signal::from(config.offset)
        >
            {children()}
        </BaseGridItem>
    }
}
