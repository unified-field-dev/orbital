/// Compiled button-group stylesheet and stable `orbital-button-group*` class names.
///
pub fn button_group_styles() -> &'static str {
    r#"
.orbital-button-group {
    display: inline-flex;
}

.orbital-button-group--vertical {
    display: inline-flex;
    flex-direction: column;
}

/* Only direct child buttons are segmented; nested triggers (e.g. ActionMenuButton menu) keep their own radii. */
.orbital-button-group--vertical > .orbital-button:first-child {
    border-bottom-left-radius: 0 !important;
    border-bottom-right-radius: 0 !important;
}

.orbital-button-group--vertical > .orbital-button:last-child {
    border-top-left-radius: 0 !important;
    border-top-right-radius: 0 !important;
}

.orbital-button-group--vertical > .orbital-button:not(:first-child):not(:last-child),
.orbital-button-group:not(.orbital-button-group--vertical)
    > .orbital-button:not(:first-child):not(:last-child) {
    border-radius: 0 !important;
}

.orbital-button-group:not(.orbital-button-group--vertical) > .orbital-button:first-child {
    border-top-right-radius: 0 !important;
    border-bottom-right-radius: 0 !important;
}

.orbital-button-group:not(.orbital-button-group--vertical) > .orbital-button:last-child {
    border-top-left-radius: 0 !important;
    border-bottom-left-radius: 0 !important;
}
"#
}
