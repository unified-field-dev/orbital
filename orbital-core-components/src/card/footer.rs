use leptos::prelude::*;
use orbital_macros::component_doc;

use super::styles::card_footer_styles;

/// Footer row inside a card for supplemental actions or metadata.
///
/// Place action buttons below body content. Footer controls stay outside [`CardButtonArea`] so they do not share the primary card click handler.
///
/// # Examples
///
/// ## Single action
/// Footer with one primary action button.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Button, Card, CardContent, CardFooter};
/// view! {
///     <div data-testid="card-footer-preview">
///         <div data-testid="card-footer-single" style="max-width: 360px;">
///             <Card>
///                 <CardContent>"Card body copy."</CardContent>
///                 <CardFooter>
///                     <Button>"Save"</Button>
///                 </CardFooter>
///             </Card>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Multiple actions
/// Footer row with primary and secondary actions.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Card, CardContent, CardFooter};
/// view! {
///     <div data-testid="card-footer-actions" style="max-width: 360px;">
///         <Card>
///             <CardContent>"Card body copy."</CardContent>
///             <CardFooter>
///                 <Button>"Reply"</Button>
///                 <Button appearance=ButtonAppearance::Secondary>"Share"</Button>
///             </CardFooter>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-footer",
    preview_label = "Card Footer",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardFooter(
    /// Extra CSS class names merged onto the footer action row.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Footer actions — typically a row of [`Button`] components aligned to the trailing edge.
    children: Children,
) -> impl IntoView {
    let style_sheet = card_footer_styles();
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-card-footer".to_string()
        } else {
            format!("orbital-card-footer {extra}")
        }
    });

    view! {
        <style>{style_sheet}</style>
        <div class=root_class>
            {children()}
        </div>
    }
}
