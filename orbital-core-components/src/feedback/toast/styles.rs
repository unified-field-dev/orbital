/// Toaster stylesheet.
pub fn toaster_styles() -> &'static str {
    r#"div.orbital-toaster-wrapper {
    z-index: 2050;
    position: absolute;
    top: 0px;
    left: 0px;
    right: 0px;
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    text-align: left;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
}

.orbital-toaster {
    position: fixed;
    width: 292px;
    pointer-events: none;
}

.orbital-toaster--top {
    top: var(--orbital-toast-offset-y, 16px);
    left: calc(50% + var(--orbital-toast-offset-x, 20px));
    transform: translateX(-50%);
}

.orbital-toaster--top-start {
    top: var(--orbital-toast-offset-y, 16px);
    left: var(--orbital-toast-offset-x, 20px);
}

.orbital-toaster--top-end {
    top: var(--orbital-toast-offset-y, 16px);
    right: var(--orbital-toast-offset-x, 20px);
}

.orbital-toaster--bottom {
    bottom: var(--orbital-toast-offset-y, 16px);
    left: calc(50% + var(--orbital-toast-offset-x, 20px));
    transform: translateX(-50%);
}

.orbital-toaster--bottom-start {
    bottom: var(--orbital-toast-offset-y, 16px);
    left: var(--orbital-toast-offset-x, 20px);
}

.orbital-toaster--bottom-end {
    bottom: var(--orbital-toast-offset-y, 16px);
    right: var(--orbital-toast-offset-x, 20px);
}

.orbital-toast-stack {
    position: fixed;
    width: 292px;
    pointer-events: none;
    z-index: 2050;
}

.orbital-toast-stack--top {
    top: var(--orbital-toast-offset-y, 16px);
    left: calc(50% + var(--orbital-toast-offset-x, 20px));
    transform: translateX(-50%);
}

.orbital-toast-stack--top-start {
    top: var(--orbital-toast-offset-y, 16px);
    left: var(--orbital-toast-offset-x, 20px);
}

.orbital-toast-stack--top-end {
    top: var(--orbital-toast-offset-y, 16px);
    right: var(--orbital-toast-offset-x, 20px);
}

.orbital-toast-stack--bottom {
    bottom: var(--orbital-toast-offset-y, 16px);
    left: calc(50% + var(--orbital-toast-offset-x, 20px));
    transform: translateX(-50%);
}

.orbital-toast-stack--bottom-start {
    bottom: var(--orbital-toast-offset-y, 16px);
    left: var(--orbital-toast-offset-x, 20px);
}

.orbital-toast-stack--bottom-end {
    bottom: var(--orbital-toast-offset-y, 16px);
    right: var(--orbital-toast-offset-x, 20px);
}

.orbital-toast-stack--inline {
    position: absolute;
}

.orbital-toast-trigger {
    display: inline-flex;
}

.orbital-toast-footer__action {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    font: inherit;
    color: var(--orb-color-brand-fg);
}

.orbital-toast-stack .orbital-toast {
    pointer-events: all;
    margin-top: 8px;
}

.orbital-toaster-container {
    box-sizing: border-box;
    margin-top: 16px;
    pointer-events: all;
    border-radius: var(--orb-radius-md);
}

.orbital-toaster-container.fade-in-height-expand-transition-leave-from,
.orbital-toaster-container.fade-in-height-expand-transition-enter-to {
    transform: scale(1);
    opacity: 1;
}

.orbital-toaster-container.fade-in-height-expand-transition-leave-to,
.orbital-toaster-container.fade-in-height-expand-transition-enter-from {
    transform: scale(0.85);
    opacity: 0;
    margin-bottom: 0 !important;
    max-height: 0 !important;
}

.orbital-toaster-container.fade-in-height-expand-transition-leave-active {
    overflow: visible;
    transition: max-height 0.3s cubic-bezier(0.4, 0, 0.2, 1) 0s,
        opacity 0.3s cubic-bezier(0, 0, 0.2, 1) 0s,
        margin-bottom 0.3s cubic-bezier(0.4, 0, 0.2, 1) 0s,
        transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.orbital-toaster-container.fade-in-height-expand-transition-enter-active {
    overflow: visible;
    transition: max-height 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        opacity 0.3s cubic-bezier(0.4, 0, 1, 1),
        margin-bottom 0.3s cubic-bezier(0.4, 0, 0.2, 1),
        transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.orbital-toast {
    display: grid;
    grid-template-columns: auto 1fr auto;
    padding: 12px;
    border-radius: var(--orb-radius-md);
    border: 1px solid var(--orb-color-border-transparent);
    box-shadow: var(--orb-elev-raised-md);
    font-size: var(--orb-type-size-sm);
    line-height: 20px;
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
    background-color: var(--orb-color-surface-canvas);
}

.orbital-toast--success {
    border-color: var(--orb-color-status-success-border);
    background-color: var(--orb-color-status-success-bg);
}

.orbital-toast--warning {
    border-color: var(--orb-color-status-warning-border);
    background-color: var(--orb-color-status-warning-bg);
}

.orbital-toast--error {
    border-color: var(--orb-color-status-danger-border);
    background-color: var(--orb-color-status-danger-bg);
}

.orbital-toast-title__media {
    display: flex;
    padding-top: 2px;
    grid-column-end: 2;
    padding-right: 8px;
    font-size: 16px;
    color: var(--orb-color-text-secondary);
}

.orbital-toast-title__info {
    color: var(--orb-color-text-secondary);
}

.orbital-toast-title__success {
    color: var(--orb-color-status-success-fg);
}

.orbital-toast-title__warning {
    color: var(--orb-color-status-warning-fg);
}

.orbital-toast-title__error {
    color: var(--orb-color-status-danger-fg);
}

.orbital-toast-title__media > svg {
    display: inline;
    line-height: 0;
}

.orbital-toast-title {
    display: flex;
    grid-column-end: 3;
    color: var(--orb-color-text-primary);
    word-break: break-word;
}

.orbital-toast-title__action {
    display: flex;
    align-items: start;
    padding-left: 12px;
    grid-column-end: -1;
    color: var(--orb-color-brand-fg);
}

.orbital-toast-body {
    grid-column-start: 2;
    grid-column-end: 3;
    padding-top: 6px;
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-regular);
    color: var(--orb-color-text-primary);
    word-break: break-word;
}

.orbital-toast-body__subtitle {
    padding-top: 4px;
    grid-column-start: 2;
    grid-column-end: 3;
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-regular);
    color: var(--orb-color-text-secondary);
}

.orbital-toast-footer {
    padding-top: 16px;
    grid-column-start: 2;
    grid-column-end: 3;
    display: flex;
    align-items: center;
    gap: 14px;
}
"#
}
