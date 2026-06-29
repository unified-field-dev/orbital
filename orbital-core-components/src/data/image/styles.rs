pub fn image_styles() -> &'static str {
    r#"
.orbital-image {
    display: inline-block;
    box-sizing: border-box;
    border-color: var(--orb-color-border-default);
    border-radius: var(--orb-radius-none);
}

.orbital-image--rounded {
    border-radius: var(--orb-radius-md);
}

.orbital-image--circular {
    border-radius: var(--orb-radius-circular);
}

.orbital-image--block {
    width: 100%;
}

.orbital-image--fit-none {
    object-fit: none;
}
.orbital-image--fit-contain {
    object-fit: contain;
}
.orbital-image--fit-cover {
    object-fit: cover;
}
.orbital-image--fit-fill {
    object-fit: fill;
}
.orbital-image--fit-scale-down {
    object-fit: scale-down;
}

.orbital-image--shadow {
    box-shadow: var(--orb-elev-raised-sm);
}
"#
}
