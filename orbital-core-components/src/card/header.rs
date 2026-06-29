use leptos::prelude::*;
use orbital_macros::component_doc;

use super::styles::card_header_styles;

/// Card header with title row and optional description and action slots.
///
/// # Examples
///
/// ## Title only
/// Header row with a single title line.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardHeader, Subtitle1};
/// view! {
///     <div data-testid="card-header-preview">
///         <div data-testid="card-header-title" style="max-width: 360px;">
///             <Card>
///                 <CardHeader>
///                     <Subtitle1>"Overview"</Subtitle1>
///                 </CardHeader>
///             </Card>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Title and description
/// Header with a secondary description row below the title.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardHeader, CardHeaderDescription, Subtitle1};
/// view! {
///     <div data-testid="card-header-description" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Team workspace"</Subtitle1>
///                 <CardHeaderDescription slot>
///                     <span>"Shared files and tasks"</span>
///                 </CardHeaderDescription>
///             </CardHeader>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Title, description, and action
/// Header with description and a trailing action control.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, Card, CardHeader, CardHeaderAction, CardHeaderDescription, Subtitle1};
/// view! {
///     <div data-testid="card-header-action" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Team workspace"</Subtitle1>
///                 <CardHeaderDescription slot>
///                     <span>"Shared files and tasks"</span>
///                 </CardHeaderDescription>
///                 <CardHeaderAction slot>
///                     <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
///                 </CardHeaderAction>
///             </CardHeader>
///         </Card>
///     </div>
/// }
/// ```
///
/// ## Header with media
/// Title and description above a hero image — images belong in [`CardMedia`](crate::CardMedia), not inside the header.
/// <!-- preview -->
/// ```rust
/// use crate::{Card, CardContent, CardHeader, CardHeaderDescription, CardMedia, Subtitle1};
/// view! {
///     <div data-testid="card-header-media" style="max-width: 360px;">
///         <Card>
///             <CardHeader>
///                 <Subtitle1>"Mountain trail"</Subtitle1>
///                 <CardHeaderDescription slot>
///                     <span>"Photo by Picsum"</span>
///                 </CardHeaderDescription>
///             </CardHeader>
///             <CardMedia
///                 src="https://picsum.photos/seed/orbital-card/360/140"
///                 alt="Sample card illustration"
///                 height=140
///             />
///             <CardContent>"Card body copy below the hero image."</CardContent>
///         </Card>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Surfaces",
    preview_slug = "card-header",
    preview_label = "Card Header",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn CardHeader(
    /// Extra CSS class names merged onto the header row container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional subtitle slot rendered below the title row via [`CardHeaderDescription`].
    #[prop(optional)]
    card_header_description: Option<CardHeaderDescription>,
    /// Optional trailing action slot (icon button, menu) via [`CardHeaderAction`].
    #[prop(optional)]
    card_header_action: Option<CardHeaderAction>,
    /// Title row content — typically [`Subtitle1`] or a heading preset.
    children: Children,
) -> impl IntoView {
    let style_sheet = card_header_styles();
    let has_description = card_header_description.is_some();
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        let base = if has_description {
            "orbital-card-header orbital-card-header-with-description"
        } else {
            "orbital-card-header"
        };
        if extra.is_empty() {
            base.to_string()
        } else {
            format!("{base} {extra}")
        }
    });

    view! {
        <style>{style_sheet}</style>
        <div class=root_class>
            <div class="orbital-card-header__header">{children()}</div>
            {card_header_description.map(|desc| {
                view! {
                    <div class="orbital-card-header__description">{(desc.children)()}</div>
                }
            })}
            {card_header_action.map(|action| {
                view! {
                    <div class="orbital-card-header__action">{(action.children)()}</div>
                }
            })}
        </div>
    }
}

#[slot]
pub struct CardHeaderDescription {
    children: Children,
}

#[slot]
pub struct CardHeaderAction {
    children: Children,
}
