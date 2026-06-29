pub fn divider_styles() -> &'static str {
    r#"
.orbital-divider {
    position: relative;
    display: flex;
    flex-direction: row;
    align-items: center;
    flex-grow: 1;
    width: 100%;
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-regular);
    color: var(--orb-color-text-secondary);
    line-height: var(--orb-type-line-sm);
    box-sizing: border-box;
    text-align: center;
}
.orbital-divider--vertical {
    flex-direction: column;
    width: auto;
    height: 100%;
    min-height: 20px;
}
.orbital-divider::before {
    content: "";
    display: flex;
    flex-grow: 1;
    min-width: 8px;
    border-top-width: var(--orb-stroke-thin);
    border-top-style: solid;
    border-color: var(--orb-color-border-subtle);
    box-sizing: border-box;
}
.orbital-divider--vertical::before {
    min-width: auto;
    min-height: 8px;
    border-top-width: medium;
    border-top-style: none;
    border-right-width: var(--orb-stroke-thin);
    border-right-style: solid;
}
.orbital-divider::after {
    content: "";
    display: flex;
    flex-grow: 1;
    min-width: 8px;
    border-top-width: var(--orb-stroke-thin);
    border-top-style: solid;
    border-color: var(--orb-color-border-subtle);
    box-sizing: border-box;
}
.orbital-divider--vertical::after {
    min-width: auto;
    min-height: 8px;
    border-top-width: medium;
    border-top-style: none;
    border-right-width: var(--orb-stroke-thin);
    border-right-style: solid;
}
.orbital-divider--unlabeled::after {
    display: none;
}
.orbital-divider--unlabeled::before {
    flex-grow: 1;
    width: 100%;
}
.orbital-divider--unlabeled.orbital-divider--vertical::before {
    flex-grow: 1;
    height: 100%;
}
.orbital-divider__wrapper {
    margin: 0 var(--orb-space-inline-sm);
}
.orbital-divider--vertical > .orbital-divider__wrapper {
    margin: var(--orb-space-block-sm) 0;
}
"#
}
