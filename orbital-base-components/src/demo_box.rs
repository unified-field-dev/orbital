//! Themed dashed-border box for layout demos and unimplemented primitive stubs.

use leptos::{prelude::*, tachys::view::any_view::IntoAny};
use turf::inline_style_sheet_values;

use crate::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};

/// A themed dashed-border box for preview demos and primitive stubs awaiting implementation.
///
/// Use inside layout previews to show container bounds, or as the body of a gap stub with `label=placeholder_label("ComponentName")`.
#[component]
pub fn DemoBox(
    /// Optional label text when no children are provided.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// CSS width value (for example `280px` or `100%`). Ignored when `fill` is true.
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// CSS height value (for example `160px` or `100%`). Ignored when `fill` is true.
    #[prop(optional, into)]
    height: MaybeProp<String>,
    /// Theme-aware padding; defaults to medium inset on all sides.
    #[prop(optional, into)]
    padding: MaybeProp<SpacingInset>,
    /// When true, sets `width: 100%; height: 100%` to fill the parent container.
    #[prop(optional, default = false)]
    fill: bool,
    /// `data-testid` attribute for e2e tests.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Root {
            box-sizing: border-box;
        }
    };

    let merged_style = move || {
        let mut parts = Vec::new();
        if fill {
            parts.push("width: 100%".to_string());
            parts.push("height: 100%".to_string());
        } else {
            if let Some(w) = width.get() {
                if !w.is_empty() {
                    parts.push(format!("width: {w}"));
                }
            }
            if let Some(h) = height.get() {
                if !h.is_empty() {
                    parts.push(format!("height: {h}"));
                }
            }
        }
        let pad = padding.get().unwrap_or(SpacingInset::all_m());
        parts.push(pad.padding_css().trim_end_matches(';').to_string());
        parts.push(format!(
            "border: {} dashed {}",
            StrokeWidth::Thin.css_var(),
            ThemeColor::NeutralStroke1.css_var()
        ));
        parts.push(format!("border-radius: {}", BorderRadius::Medium.css_var()));
        parts.push(format!(
            "color: {}",
            ThemeColor::NeutralForeground1.css_var()
        ));
        parts.join("; ")
    };

    view! {
        <style>{style_sheet}</style>
        <div
            class=move || {
                match class.get() {
                    Some(extra) if !extra.trim().is_empty() => {
                        format!("{} {}", class_names.root, extra)
                    }
                    _ => class_names.root.to_string(),
                }
            }
            style=merged_style
            data-testid=move || data_testid.get()
        >
            {if let Some(children) = children {
                children().into_any()
            } else if let Some(label_text) = label.get() {
                view! { {label_text} }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
