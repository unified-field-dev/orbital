/// Compiled slider stylesheet and stable `orbital-slider*` class names (excluded from turf hashing).
///
pub fn slider_styles() -> &'static str {
    r#"
.orbital-slider {
    min-height: 32px;
    justify-items: center;
    touch-action: none;
    display: inline-grid;
    position: relative;
    align-items: center;
    --orbital-slider__rail--size: 4px;
    --orbital-slider__thumb--size: 20px;
    --orbital-slider__thumb--color: var(--orb-color-brand-compound-bg);
    --orbital-slider__progress--color: var(--orb-color-brand-compound-bg);
    --orbital-slider__rail--color: var(--orb-color-border-accessible);
}
.orbital-slider--horizontal {
    min-width: 120px;
    grid-template-rows: 1fr var(--orbital-slider__thumb--size) 1fr;
    grid-template-columns: 1fr calc(100% - var(--orbital-slider__thumb--size)) 1fr;
}
.orbital-slider--vertical {
    min-height: 120px;
    grid-template-rows: 1fr calc(100% - var(--orbital-slider__thumb--size)) 1fr;
    grid-template-columns: 1fr var(--orbital-slider__thumb--size) 1fr;
}
.orbital-slider:hover {
    --orbital-slider__progress--color: var(--orb-color-brand-compound-bg-hover);
    --orbital-slider__thumb--color: var(--orb-color-brand-compound-bg-hover);
}
.orbital-slider:active {
    --orbital-slider__progress--color: var(--orb-color-brand-compound-bg-pressed);
    --orbital-slider__thumb--color: var(--orb-color-brand-compound-bg-pressed);
}
.orbital-slider__input {
    grid-column-end: -1;
    grid-column-start: 1;
    grid-row-end: -1;
    grid-row-start: 1;
    margin: 0;
    padding: 0;
    opacity: 0;
    cursor: pointer;
}
.orbital-slider--horizontal .orbital-slider__input {
    width: 100%;
    height: var(--orbital-slider__thumb--size);
}
.orbital-slider--vertical .orbital-slider__input {
    width: var(--orbital-slider__thumb--size);
    height: 100%;
    -webkit-appearance: slider-vertical;
}
.orbital-slider__rail {
    position: relative;
    forced-color-adjust: none;
    grid-column-end: 2;
    grid-column-start: 2;
    grid-row-end: 2;
    grid-row-start: 2;
    background-image: linear-gradient(
        var(--orbital-slider--direction),
        var(--orbital-slider__progress--color) 0%,
        var(--orbital-slider__progress--color) var(--orbital-slider--progress),
        var(--orbital-slider__rail--color) var(--orbital-slider--progress)
    );
    border-radius: var(--orb-radius-xl);
    outline: 1px solid var(--orb-color-border-transparent);
    pointer-events: none;
}
.orbital-slider--horizontal .orbital-slider__rail {
    width: 100%;
    height: var(--orbital-slider__rail--size);
}
.orbital-slider--vertical .orbital-slider__rail {
    width: var(--orbital-slider__rail--size);
    height: 100%;
}
.orbital-slider__rail::before {
    content: "";
    position: absolute;
    background-image: repeating-linear-gradient(
        var(--orbital-slider--direction),
        #0000 0%,
        #0000 calc(var(--orbital-slider--steps-percent) - 1px),
        var(--orb-color-surface-canvas) calc(var(--orbital-slider--steps-percent) - 1px),
        var(--orb-color-surface-canvas) var(--orbital-slider--steps-percent)
    );
}
.orbital-slider--horizontal .orbital-slider__rail::before {
    height: var(--orbital-slider__rail--size);
    right: -1px;
    left: -1px;
}
.orbital-slider--vertical .orbital-slider__rail::before {
    width: var(--orbital-slider__rail--size);
    top: -1px;
    bottom: -1px;
}
.orbital-slider__thumb {
    position: absolute;
    forced-color-adjust: none;
    grid-column-end: 2;
    grid-column-start: 2;
    grid-row-end: 2;
    grid-row-start: 2;
    height: var(--orbital-slider__thumb--size);
    width: var(--orbital-slider__thumb--size);
    background-color: var(--orbital-slider__thumb--color);
    outline-style: none;
    pointer-events: none;
    border-radius: var(--orb-radius-circular);
    box-shadow: 0 0 0 calc(var(--orbital-slider__thumb--size) * 0.2) var(--orb-color-surface-canvas) inset;
}
.orbital-slider--horizontal .orbital-slider__thumb {
    transform: translateX(-50%);
    left: var(--orbital-slider--progress);
}
.orbital-slider--vertical .orbital-slider__thumb {
    transform: translateY(50%);
    bottom: var(--orbital-slider--progress);
}
.orbital-slider__thumb::before {
    content: "";
    position: absolute;
    bottom: 0;
    right: 0;
    left: 0;
    top: 0;
    box-sizing: border-box;
    border-radius: var(--orb-radius-circular);
    border: calc(var(--orbital-slider__thumb--size) * 0.05) solid var(--orb-color-border-default);
}
.orbital-slider__datalist {
    display: block;
    position: absolute;
}
.orbital-slider--horizontal .orbital-slider__datalist {
    width: 100%;
    top: calc(var(--orbital-slider__thumb--size) + 4px);
}
.orbital-slider--vertical .orbital-slider__datalist {
    height: 100%;
    left: calc(var(--orbital-slider__thumb--size) + 4px);
}
.orbital-slider-label {
    position: absolute;
    display: inline-block;
}
.orbital-slider-label--horizontal {
    transform: translateX(-50%);
}
.orbital-slider-label--vertical {
    transform: translateY(50%);
}
"#
}
