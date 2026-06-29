use std::sync::LazyLock;

const SKELETON_ITEM_SIZES: &[u16] = &[
    8, 12, 14, 16, 20, 22, 24, 28, 32, 36, 40, 48, 52, 56, 64, 72, 92, 96, 120, 128,
];

fn skeleton_item_size_rules() -> String {
    SKELETON_ITEM_SIZES
        .iter()
        .map(|size| {
            format!(
                ".orbital-skeleton-item--rectangle.orbital-skeleton-item--size-{size} {{ height: {size}px; }}\n\
                 .orbital-skeleton-item--square.orbital-skeleton-item--size-{size}, \
                 .orbital-skeleton-item--circle.orbital-skeleton-item--size-{size} {{ \
                 width: {size}px; height: {size}px; }}\n"
            )
        })
        .collect()
}

fn build_skeleton_item_styles() -> String {
    format!(
        r#".orbital-skeleton-item {{
    background-image: linear-gradient(
        to right,
        var(--orb-color-stencil-primary) 0%,
        var(--orb-color-stencil-secondary) 50%,
        var(--orb-color-stencil-primary) 100%
    );
    animation-name: orbital-skeleton-item;
    animation-timing-function: linear;
    animation-duration: 3s;
    animation-iteration-count: infinite;
    background-attachment: fixed;
    background-position-y: 50%;
    background-position-x: 50%;
    background-size: 300% 100%;
    position: relative;
    overflow: hidden;
}}

.orbital-skeleton-item--rectangle {{
    display: block;
    width: 100%;
    border-radius: 4px;
}}

.orbital-skeleton-item--square {{
    display: block;
    border-radius: 4px;
}}

.orbital-skeleton-item--circle {{
    display: block;
    border-radius: 50%;
}}

{size_rules}

@keyframes orbital-skeleton-item {{
    0% {{
        background-position-x: 300%;
    }}

    100% {{
        background-position-x: 0%;
    }}
}}
"#,
        size_rules = skeleton_item_size_rules()
    )
}

static SKELETON_ITEM_STYLES: LazyLock<&'static str> =
    LazyLock::new(|| Box::leak(build_skeleton_item_styles().into_boxed_str()));

/// Skeleton item stylesheet.
pub fn skeleton_item_styles() -> &'static str {
    *SKELETON_ITEM_STYLES
}
