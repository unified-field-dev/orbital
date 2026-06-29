use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use crate::components::{
    AvatarColor, Caption1, Card, CardContent, Flex, FlexAlign, FlexGap, FlexWrap, Link, Persona,
    PersonaConfig, PersonaPrimaryText, PersonaSecondaryText, PersonaSize, Subtitle1,
};

fn map_avatar_size(px: u8) -> PersonaSize {
    match px {
        0..=22 => PersonaSize::ExtraSmall,
        23..=27 => PersonaSize::Small,
        28..=33 => PersonaSize::Medium,
        34..=37 => PersonaSize::Large,
        38..=47 => PersonaSize::ExtraLarge,
        _ => PersonaSize::Huge,
    }
}

/// Compact identity surface — avatar plus title and subtitle — nested inside a larger card.
///
/// `name` drives the [`Persona`] avatar (initials and color). `title` is the primary line; `subtitle` is secondary text. Nest inside a [`Card`] body for ownership, contact, or team summary blocks.
///
/// # Examples
///
/// ## Default
/// Compact identity card with avatar, title, and subtitle.
/// <!-- preview -->
/// ```rust
/// view! {
///   <div data-testid="identity-card-preview">
///     <div data-testid="identity-card-default" style="max-width: 420px;">
///       <IdentityCard
///         name="Taylor Reid"
///         title="Core Observability"
///         subtitle="Owns dashboards, SLOs, and incident response runbooks."
///         avatar_size=40
///       />
///     </div>
///   </div>
/// }
/// ```
///
/// ## With contact
/// Email and handle rendered below the persona block.
/// <!-- preview -->
/// ```rust
/// view! {
///   <div data-testid="identity-card-contact" style="max-width: 420px;">
///     <IdentityCard
///       name="Taylor Reid"
///       title="Core Observability"
///       subtitle="Platform reliability team"
///       email="taylor.reid@example.com"
///       handle="@taylor-reid"
///       avatar_size=40
///     />
///   </div>
/// }
/// ```
///
/// ## Colorful avatar
/// Name-derived palette color on the avatar initials.
/// <!-- preview -->
/// ```rust
/// view! {
///   <div data-testid="identity-card-colorful" style="max-width: 420px;">
///     <IdentityCard
///       name="Taylor Reid"
///       title="Core Observability"
///       subtitle="Colorful avatar from name hash"
///       color=AvatarColor::Colorful
///       avatar_size=40
///     />
///   </div>
/// }
/// ```
///
/// ## Nested in parent card
/// Intended usage as an inner identity surface inside a larger card body.
/// <!-- preview -->
/// ```rust
/// view! {
///   <div data-testid="identity-card-nested" style="max-width: 420px;">
///     <Card>
///       <CardContent>
///         <Subtitle1>"Team ownership"</Subtitle1>
///         <IdentityCard
///           embedded=true
///           name="Taylor Reid"
///           title="Core Observability"
///           subtitle="Primary on-call rotation"
///           avatar_size=40
///         />
///       </CardContent>
///     </Card>
///   </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "components/patterns/identity-card",
    preview_label = "Identity Card",
    preview_icon = icondata::AiIdcardOutlined,
)]
#[component]
pub fn IdentityCard(
    /// Name shown on the avatar (used to compute initials).
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Primary title (e.g. team or owner name).
    #[prop(into)]
    title: MaybeProp<String>,
    /// Secondary subtitle/description.
    #[prop(optional, into)]
    subtitle: MaybeProp<String>,
    /// Optional contact email (rendered as a mailto link).
    #[prop(optional, into)]
    email: MaybeProp<String>,
    /// Optional contact handle (e.g. Slack/Discord handle).
    #[prop(optional, into)]
    handle: MaybeProp<String>,
    /// Avatar size in pixels.
    #[prop(optional, into)]
    avatar_size: MaybeProp<u8>,
    /// Avatar background color preset.
    #[prop(optional, into)]
    color: MaybeProp<AvatarColor>,
    /// When true, render persona content without the outer identity card shell.
    #[prop(optional, default = false)]
    embedded: bool,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            max-width: 100%;
            margin: 0;
            box-sizing: border-box;
            border: 1px solid var(--orb-color-border-default);
            border-radius: 10px;
            background: var(--orb-color-surface-canvas-hover);
        }

        .Content {
            --orbital-card-content-padding: 12px;
        }

        .Muted {
            color: var(--orb-color-text-tertiary);
        }
    };

    view! {
        <style>{style_sheet}</style>
        {move || {
            let persona = move || {
                let avatar_px = avatar_size.get().unwrap_or(40);
                let persona_config = PersonaConfig {
                    name: name.get(),
                    size: map_avatar_size(avatar_px),
                    color: color.get().unwrap_or(AvatarColor::Colorful),
                    ..Default::default()
                };
                let title_text = title.get().unwrap_or_default();

                if let Some(sub) = subtitle.get() {
                    view! {
                        <Persona config=persona_config>
                            <PersonaPrimaryText slot>
                                <Subtitle1>{title_text.clone()}</Subtitle1>
                            </PersonaPrimaryText>
                            <PersonaSecondaryText slot>{sub}</PersonaSecondaryText>
                        </Persona>
                    }
                    .into_any()
                } else {
                    view! {
                        <Persona config=persona_config>
                            <PersonaPrimaryText slot>
                                <Subtitle1>{title_text}</Subtitle1>
                            </PersonaPrimaryText>
                        </Persona>
                    }
                    .into_any()
                }
            };

            let contact = move || {
                let email_val = email.get();
                let handle_val = handle.get();
                if email_val.is_none() && handle_val.is_none() {
                    return None;
                }
                Some(view! {
                    <Flex gap=FlexGap::Small wrap=FlexWrap::Wrap align=FlexAlign::Center>
                        {email_val.map(|e| {
                            let href = format!("mailto:{e}");
                            view! {
                                <Link href=href inline=true>{e}</Link>
                            }
                        })}
                        {handle_val.map(|h| view! {
                            <Caption1 class=class_names.muted>{h}</Caption1>
                        })}
                    </Flex>
                })
            };

            if embedded {
                view! {
                    <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Stretch>
                        {persona()}
                        {contact()}
                    </Flex>
                }
                .into_any()
            } else {
                view! {
                    <Card class=class_names.card>
                        <CardContent class=class_names.content>
                            <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Stretch>
                                {persona()}
                                {contact()}
                            </Flex>
                        </CardContent>
                    </Card>
                }
                .into_any()
            }
        }}
    }
}
