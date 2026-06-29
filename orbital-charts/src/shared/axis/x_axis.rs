//! [`XAxis`] — horizontal axis tick and label rendering.

use leptos::prelude::*;
use orbital_theme::{use_theme_options, Direction};

use crate::axis_categories;
use crate::context::{use_chart_context, use_x_scale, use_x_ticks};
use crate::shared::axis::ticks::{
    band_ticks, linear_ticks, x_axis_title_y, x_label_position, x_tick_line,
};
use crate::{AxisPosition, ScaleType};

/// Renders the x-axis ticks and labels for one axis id.
#[component]
pub fn XAxis(
    /// Target axis id (defaults to the first x-axis).
    #[prop(optional, into)]
    axis_id: Option<String>,
    /// Position override (defaults to axis definition or bottom).
    #[prop(optional)]
    position: Option<AxisPosition>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let id = axis_id.unwrap_or_else(|| {
        ctx.x_axes
            .first()
            .map(|a| a.id.clone())
            .unwrap_or_else(|| "x".into())
    });
    let axis = ctx
        .x_axes
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .unwrap_or_default();
    let pos = position.unwrap_or(axis.position);
    let scale_type = axis.scale_type;
    let tick_format = axis.tick_format;
    let axis_label = axis.label.clone();
    let scale = use_x_scale(id.clone());
    let tick_values = use_x_ticks(id.clone());
    let area = ctx.drawing_area;
    let categories = axis_categories(&axis, ctx.projected.as_ref());

    let tick_placement = axis.tick_placement;
    let ticks = Memo::new(move |_| match scale_type {
        ScaleType::Band | ScaleType::Point => band_ticks(&scale, &categories, tick_placement),
        ScaleType::Linear | ScaleType::Log | ScaleType::Sqrt => {
            linear_ticks(&scale, &tick_values, tick_format.as_ref())
        }
        _ => Vec::new(),
    });

    let y_base = match pos {
        AxisPosition::Top => area.top,
        _ => area.top + area.plot_height,
    };

    let axis_class = match pos {
        AxisPosition::Top => "orb-axis orb-x-axis orb-x-axis--top",
        _ => "orb-axis orb-x-axis orb-x-axis--bottom",
    };

    let x1 = area.left;
    let x2 = area.left + area.plot_width;
    let theme_options = use_theme_options();
    let is_rtl = move || theme_options.get().direction == Direction::Rtl;

    view! {
        <g class=axis_class dir=move || theme_options.get().direction.as_str()>
            <line
                class="orb-axis-line"
                x1=x1
                y1=y_base
                x2=x2
                y2=y_base
            />
            {move || ticks.get().into_iter().map(|tick| {
                let (lx1, ly1) = (area.left + tick.position, y_base);
                let (_, (lx2, ly2)) = x_tick_line(area.left + tick.position, y_base);
                let (tx, ty) = x_label_position(area.left + tick.position, y_base);
                let anchor = if is_rtl() { "middle" } else { "middle" };
                view! {
                    <g class="orb-axis-tick-group">
                        <line class="orb-axis-tick" x1=lx1 y1=ly1 x2=lx2 y2=ly2 />
                        <text
                            class="orb-axis-tick-label"
                            x=tx
                            y=ty
                            text-anchor=anchor
                            dominant-baseline="hanging"
                        >
                            {tick.label}
                        </text>
                    </g>
                }
            }).collect_view()}
            {axis_label.map(|label| {
                let tx = area.left + area.plot_width / 2.0;
                let ty = x_axis_title_y(y_base, pos);
                view! {
                    <text
                        class="orb-axis-label"
                        x=tx
                        y=ty
                        text-anchor="middle"
                        dominant-baseline="hanging"
                    >
                        {label}
                    </text>
                }
            })}
        </g>
    }
}
