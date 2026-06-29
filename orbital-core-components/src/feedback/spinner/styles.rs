/// Spinner stylesheet.
pub fn spinner_styles() -> &'static str {
    r#".orbital-spinner {
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 0;
    gap: 8px;
    overflow: hidden;
}

.orbital-spinner__spinner {
    width: 32px;
    height: 32px;
    position: relative;
    flex-shrink: 0;
    mask-image: radial-gradient(
        closest-side,
        transparent calc(100% - var(--orbital-spinner--stroke-width) - 1px),
        white calc(100% - var(--orbital-spinner--stroke-width)) calc(100% - 1px),
        transparent 100%
    );
    background-color: var(--orb-color-brand-stroke-contrast);
    color: var(--orb-color-brand-stroke);
    animation-duration: 1.5s;
    animation-iteration-count: infinite;
    animation-timing-function: linear;
    animation-name: orbital-spinner;

    --orbital-spinner--stroke-width: var(--orb-stroke-thicker);
}

.orbital-spinner--extra-tiny > .orbital-spinner__spinner {
    --orbital-spinner--stroke-width: var(--orb-stroke-thick);
    width: 16px;
    height: 16px;
}

.orbital-spinner--tiny > .orbital-spinner__spinner {
    --orbital-spinner--stroke-width: var(--orb-stroke-thick);
    width: 20px;
    height: 20px;
}

.orbital-spinner--extra-small > .orbital-spinner__spinner {
    --orbital-spinner--stroke-width: var(--orb-stroke-thick);
    width: 24px;
    height: 24px;
}

.orbital-spinner--small > .orbital-spinner__spinner {
    --orbital-spinner--stroke-width: var(--orb-stroke-thick);
    width: 28px;
    height: 28px;
}

.orbital-spinner--medium > .orbital-spinner__spinner {
    width: 32px;
    height: 32px;
}

.orbital-spinner--large > .orbital-spinner__spinner {
    width: 36px;
    height: 36px;
}

.orbital-spinner--extra-large > .orbital-spinner__spinner {
    width: 40px;
    height: 40px;
}

.orbital-spinner--huge > .orbital-spinner__spinner {
    --orbital-spinner--stroke-width: var(--orb-stroke-thickest);
    width: 44px;
    height: 44px;
}

@keyframes orbital-spinner {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

.orbital-spinner__spinner-tail {
    position: absolute;
    display: block;
    width: 100%;
    height: 100%;
    mask-image: conic-gradient(transparent 105deg, white 105deg);
    animation-duration: 1.5s;
    animation-iteration-count: infinite;
    animation-timing-function: var(--orb-motion-ease-standard);
    animation-name: orbital-spinner-tail;
}

@keyframes orbital-spinner-tail {
    0% {
        transform: rotate(-135deg);
    }
    50% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(225deg);
    }
}

.orbital-spinner__spinner-tail::before,
.orbital-spinner__spinner-tail::after {
    content: "";
    position: absolute;
    display: block;
    width: 100%;
    height: 100%;
    animation: inherit;
    background-image: conic-gradient(currentcolor 135deg, transparent 135deg);
}

.orbital-spinner__spinner-tail::before {
    animation-name: orbital-spinner-tail-before;
}

@keyframes orbital-spinner-tail-before {
    0% {
        transform: rotate(0deg);
    }
    50% {
        transform: rotate(105deg);
    }
    100% {
        transform: rotate(0deg);
    }
}

.orbital-spinner__spinner-tail::after {
    animation-name: orbital-spinner-tail-after;
}

@keyframes orbital-spinner-tail-after {
    0% {
        transform: rotate(0deg);
    }
    50% {
        transform: rotate(225deg);
    }
    100% {
        transform: rotate(0deg);
    }
}

.orbital-spinner__label {
    font-family: var(--orb-type-family-sans);
    font-weight: var(--orb-type-weight-semibold);
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-lg);
    color: var(--orb-color-text-primary);
}

.orbital-spinner--extra-tiny > .orbital-spinner__label,
.orbital-spinner--tiny > .orbital-spinner__label,
.orbital-spinner--extra-small > .orbital-spinner__label,
.orbital-spinner--small > .orbital-spinner__label {
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
}

.orbital-spinner--huge > .orbital-spinner__label {
    font-size: var(--orb-type-size-lg);
    line-height: var(--orb-type-line-xl);
}
"#
}
