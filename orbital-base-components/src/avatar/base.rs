use leptos::{either::Either, prelude::*};

use super::color::AvatarColor;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum AvatarShape {
    #[default]
    Circular,
    Square,
}

impl AvatarShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Circular => "circular",
            Self::Square => "square",
        }
    }
}

/// Derive display initials from a full name (first + last word initials).
pub fn initials_from_name(name: &str) -> String {
    let initials: Vec<_> = name
        .split_whitespace()
        .filter_map(|word| word.chars().next().and_then(|c| c.to_uppercase().next()))
        .collect();

    match initials.as_slice() {
        [first, .., last] => format!("{first}{last}"),
        [first] => first.to_string(),
        [] => String::new(),
    }
}

fn avatar_font_size_token(size: u8) -> Option<&'static str> {
    match size {
        0..=24 => Some("var(--orb-type-size-2xs)"),
        25..=28 => Some("var(--orb-type-size-xs)"),
        29..=40 => None,
        41..=56 => Some("var(--orb-type-size-md)"),
        57..=96 => Some("var(--orb-type-size-lg)"),
        97..=128 => Some("var(--orb-type-size-xl)"),
        _ => Some("var(--orb-type-size-xl)"),
    }
}

#[component]
pub fn BaseAvatar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] src: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] initials: MaybeProp<String>,
    #[prop(optional, into)] shape: Signal<AvatarShape>,
    #[prop(optional, into)] size: MaybeProp<u8>,
    #[prop(optional, into)] color: Signal<AvatarColor>,
    #[prop(optional, into)] id_for_color: MaybeProp<String>,
) -> impl IntoView {
    let style = move || {
        let size = size.get()?;
        let mut style = format!("width: {size}px; height: {size}px;");
        if let Some(font_size) = avatar_font_size_token(size) {
            style.push_str(&format!("font-size: {font_size};"));
        }
        Some(style)
    };

    let image_hidden = RwSignal::new(false);
    let is_show_default_icon = Memo::new(move |_| {
        let has_name = name.with(|n| n.is_some());
        let has_visible_image = src.with(|s| s.is_some()) && !image_hidden.get();
        let has_initials = initials.with(|i| i.is_some());
        !has_name && !has_visible_image && !has_initials
    });

    let on_load = move |_| {
        image_hidden.maybe_update(|hidden| {
            if *hidden {
                *hidden = false;
            }
            true
        });
    };

    let on_error = move |_| {
        image_hidden.set(true);
    };

    view! {
        <span
            class=move || {
                let resolved = color.get().resolve(
                    name.get().as_deref(),
                    initials.get().as_deref(),
                    id_for_color.get().as_deref(),
                );
                let mut parts = vec![
                    "orbital-avatar".to_string(),
                    format!("orbital-avatar--{}", shape.get().as_str()),
                    format!("orbital-avatar--color-{}", resolved.as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            style=style
            role="img"
            aria-label=move || name.get()
        >
            {move || {
                let mut display_initials = initials.get();
                if display_initials.is_none() {
                    if let Some(name) = name.get() {
                        display_initials = Some(initials_from_name(&name));
                    }
                }
                display_initials.map(|text| {
                    view! { <span class="orbital-avatar__initials">{text}</span> }
                })
            }}
            {move || {
                src.get().map(|src| {
                    view! {
                        <img
                            src=src
                            class="orbital-avatar__image"
                            role="presentation"
                            aria-hidden="true"
                            hidden=move || image_hidden.get()
                            on:load=on_load
                            on:error=on_error
                        />
                    }
                })
            }}
            {move || {
                if is_show_default_icon.get() {
                    Either::Left(view! {
                        <span aria-hidden="true" class="orbital-avatar__icon">
                            <svg
                                fill="currentColor"
                                aria-hidden="true"
                                width="1em"
                                height="1em"
                                viewBox="0 0 20 20"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M10 2a4 4 0 1 0 0 8 4 4 0 0 0 0-8ZM7 6a3 3 0 1 1 6 0 3 3 0 0 1-6 0Zm-2 5a2 2 0 0 0-2 2c0 1.7.83 2.97 2.13 3.8A9.14 9.14 0 0 0 10 18c1.85 0 3.58-.39 4.87-1.2A4.35 4.35 0 0 0 17 13a2 2 0 0 0-2-2H5Zm-1 2a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1c0 1.3-.62 2.28-1.67 2.95A8.16 8.16 0 0 1 10 17a8.16 8.16 0 0 1-4.33-1.05A3.36 3.36 0 0 1 4 13Z"
                                    fill="currentColor"
                                />
                            </svg>
                        </span>
                    })
                } else {
                    Either::Right(())
                }
            }}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::initials_from_name;

    #[test]
    fn initials_from_name_cases() {
        assert_eq!(initials_from_name("Jane Doe"), "JD");
        assert_eq!(initials_from_name("Ben"), "B");
        assert_eq!(initials_from_name(""), "");
        assert_eq!(initials_from_name("山"), "山");
    }
}
