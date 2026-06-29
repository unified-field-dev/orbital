pub fn counter_badge_styles() -> &'static str {
    r#"
.orbital-counter-badge {
    position: relative;
    display: inline-flex;
    vertical-align: middle;
}

.orbital-counter-badge__pill {
    position: absolute;
    top: 0;
    right: 0;
    transform: translate(25%, -25%);
    z-index: 1;
}
"#
}
