use leptos::{either::Either, prelude::*};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum RatingSize {
    Small,
    Medium,
    Large,
    #[default]
    ExtraLarge,
}

impl RatingSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum RatingColor {
    #[default]
    Brand,
    Neutral,
    Marigold,
}

impl RatingColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Neutral => "neutral",
            Self::Marigold => "marigold",
        }
    }
}

/// Fraction of a star filled (0.0, 0.5, or 1.0) for a given item index.
pub fn filled_fraction(displayed_value: f32, item_value: u8, step: f32) -> f32 {
    let value = f32::from(item_value);
    let displayed = if step == 0.5 {
        (displayed_value * 2.0).round() / 2.0
    } else {
        displayed_value.round()
    };

    if displayed >= value {
        1.0
    } else if step == 0.5 && displayed >= value - 0.5 {
        0.5
    } else {
        0.0
    }
}

#[component]
pub fn BaseRatingItem(
    value: u8,
    #[prop(optional, into)] displayed_value: Signal<f32>,
    #[prop(optional, into)] step: Signal<f32>,
    #[prop(optional, into)] size: Signal<RatingSize>,
    #[prop(optional, into)] color: Signal<RatingColor>,
    #[prop(optional, into)] interactive: Signal<bool>,
    #[prop(optional, into)] name: Signal<String>,
) -> impl IntoView {
    let icon_fill_width =
        Memo::new(move |_| filled_fraction(displayed_value.get(), value, step.get()));
    let half = Signal::derive(move || step.get() == 0.5);

    view! {
        <span
            class=move || {
                let mut parts = vec!["orbital-rating-item".to_string()];
                if !interactive.get() {
                    parts.push("orbital-rating-item--filled".to_string());
                }
                parts.push(format!("orbital-rating-item--{}", color.get().as_str()));
                parts.push(format!("orbital-rating-item--{}", size.get().as_str()));
                parts.join(" ")
            }
        >
            {move || {
                if interactive.get() {
                    Either::Left(view! {
                        {half.get().then(|| view! {
                            <input
                                type="radio"
                                name=move || name.get()
                                aria-label=f32::from(value) - 0.5
                                class="orbital-rating-item__half-value-input"
                                value=f32::from(value) - 0.5
                            />
                        })}
                        <input
                            type="radio"
                            name=move || name.get()
                            aria-label=value
                            class="orbital-rating-item__full-value-input"
                            value=value
                        />
                    })
                } else {
                    Either::Right(())
                }
            }}
            <span
                class="orbital-rating-item__icon"
                style=move || format!("--orbital-rating-fill: {};", icon_fill_width.get())
            />
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::filled_fraction;

    #[test]
    fn filled_fraction_whole_steps() {
        assert_eq!(filled_fraction(3.0, 1, 1.0), 1.0);
        assert_eq!(filled_fraction(3.0, 3, 1.0), 1.0);
        assert_eq!(filled_fraction(3.0, 4, 1.0), 0.0);
    }

    #[test]
    fn filled_fraction_half_steps() {
        assert_eq!(filled_fraction(2.5, 3, 0.5), 0.5);
        assert_eq!(filled_fraction(3.0, 3, 0.5), 1.0);
    }
}
