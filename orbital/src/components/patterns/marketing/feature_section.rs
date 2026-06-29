use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use crate::components::{
    Body1, Card, CardHeader, CardHeaderDescription, CardPreview, Subtitle1, Title3,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatureVariant {
    Normal,
    Alt,
}

fn feature_section_card(title: String, description: String) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Subtitle1>{title}</Subtitle1>
                <CardHeaderDescription slot>{description}</CardHeaderDescription>
            </CardHeader>
            <CardPreview>
                <div
                    aria-hidden="true"
                    style="min-height: 180px; background: var(--orb-color-surface-subtle);"
                />
            </CardPreview>
        </Card>
    }
}

/// Full-viewport feature row — copy plus a [`Card`] in a two-column layout.
///
/// Uses `100svh` minimum height so each feature gets a full-screen scroll stop on landing pages. `FeatureVariant::Normal` places copy left; `Alt` mirrors the card to the left. Pair with [`HeroSection`] above for marketing pages. Compose custom card content with [`Card`], [`CardHeader`], and [`CardPreview`] directly when you need more control.
///
/// # Examples
///
/// ## Default
/// Normal layout with copy on the left and feature card on the right.
/// <!-- preview -->
/// ```rust
/// view! {
///     <FeatureSection
///         id="feature-1".to_string()
///         title="Automations that feel like magic".to_string()
///         body="Create workflows in minutes and let the platform handle the busywork. Triggers, conditions, and actions — all observable and auditable.".to_string()
///         card_title="Visual builder".to_string()
///         card_description="Drag, connect, deploy".to_string()
///         variant=FeatureVariant::Normal
///     />
/// }
/// ```
///
/// ## Alt Variant
/// Mirrored layout with the feature card on the left.
/// <!-- preview -->
/// ```rust
/// view! {
///     <FeatureSection
///         id="feature-2".to_string()
///         title="Real-time insights".to_string()
///         body="Dashboards update live with server events. Export any view, schedule reports, and share securely with your team.".to_string()
///         card_title="Analytics".to_string()
///         card_description="Live KPIs & exports".to_string()
///         variant=FeatureVariant::Alt
///     />
/// }
/// ```
#[component_doc(
    category = "Patterns",
    preview_slug = "components/patterns/feature-section",
    preview_label = "Feature Section",
    preview_icon = icondata::AiAppstoreOutlined,
)]
#[component]
pub fn FeatureSection(
    /// Section ID (for anchor links)
    id: String,
    /// Section title
    title: String,
    /// Section body text
    body: String,
    /// Feature card title
    card_title: String,
    /// Feature card description
    card_description: String,
    /// Background variant
    #[prop(optional, default = FeatureVariant::Normal)]
    variant: FeatureVariant,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Layer {
            position: relative;
            min-height: 100svh;
            padding: 80px clamp(16px, 4vw, 40px);
            display: grid;
            align-items: center;
            overflow: clip;
            isolation: isolate;
            transform: translateZ(0);
            background:
                radial-gradient(60% 50% at 15% 35%, rgba(59,130,246,.10), transparent 60%),
                radial-gradient(60% 50% at 85% 55%, rgba(16,185,129,.10), transparent 60%),
                linear-gradient(180deg, #ffffff 0%, #fbfcff 100%);
        }

        .LayerAlt {
            background:
                radial-gradient(60% 50% at 20% 25%, rgba(244,114,182,.10), transparent 60%),
                radial-gradient(60% 50% at 80% 65%, rgba(245,158,11,.10), transparent 60%),
                linear-gradient(180deg, #ffffff 0%, #fbfbfd 100%);
        }

        .SectionInner {
            display: grid;
            gap: 24px;
            align-items: center;
            grid-template-columns: 1fr;
            max-width: 1100px;
            margin-inline: auto;
        }

        .SectionInnerAlt {
            display: grid;
            gap: 24px;
            align-items: center;
            grid-template-columns: 1fr;
            max-width: 1100px;
            margin-inline: auto;
        }

        .SectionCopy {
            display: grid;
            gap: 8px;
            min-width: 0;
            overflow-wrap: break-word;
            word-wrap: break-word;
            position: relative;
            z-index: 1;
        }

        .SectionCopy :is(h1, h2, h3, h4, h5, h6, span, p) {
            white-space: normal !important;
        }

        @media (min-width: 900px) {
            .SectionInner {
                grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr);
            }

            .SectionInnerAlt {
                grid-template-columns: minmax(0, 1fr) minmax(0, 1.2fr);
            }
        }
    };

    let layer_class = Signal::derive(move || {
        let mut classes = vec![class_names.layer.to_string()];
        if variant == FeatureVariant::Alt {
            classes.push(class_names.layer_alt.to_string());
        }
        classes.join(" ")
    });

    let title_for_aria = title.clone();
    let is_alt = variant == FeatureVariant::Alt;

    let inner_class = if is_alt {
        class_names.section_inner_alt
    } else {
        class_names.section_inner
    };

    let card = feature_section_card(card_title.clone(), card_description.clone());

    view! {
        <style>{style_sheet}</style>
        <section class=layer_class id=id.clone() aria-label=format!("Feature: {}", title_for_aria)>
            <div class=inner_class>
                {if is_alt {
                    view! {
                        <>
                            {card}
                            <div class=class_names.section_copy>
                                <Title3 tag=crate::components::TextTag::H2>{title.clone()}</Title3>
                                <Body1>{body.clone()}</Body1>
                            </div>
                        </>
                    }.into_any()
                } else {
                    view! {
                        <>
                            <div class=class_names.section_copy>
                                <Title3 tag=crate::components::TextTag::H2>{title.clone()}</Title3>
                                <Body1>{body.clone()}</Body1>
                            </div>
                            {card}
                        </>
                    }.into_any()
                }}
            </div>
        </section>
    }
}
