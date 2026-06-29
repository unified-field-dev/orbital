pub fn upload_styles() -> &'static str {
    r#".orbital-upload__input {
    width: 0;
    height: 0;
    opacity: 0;
}

.orbital-upload__trigger {
    display: inline-block;
}

.orbital-upload--drag-over .orbital-upload-dragger {
    border: var(--orb-stroke-thin) dashed var(--orb-color-brand-fg);
}
"#
}

pub fn upload_dragger_styles() -> &'static str {
    r#".orbital-upload-dragger {
    width: 100%;
    padding: 20px;
    background-color: var(--orb-color-surface-canvas-hover);
    border: var(--orb-stroke-thin) dashed var(--orb-color-border-default);
    border-radius: 3px;
    text-align: center;
    cursor: pointer;
    transition: border 0.3s;
    box-sizing: border-box;
}

.orbital-upload-dragger:hover {
    border: var(--orb-stroke-thin) dashed var(--orb-color-brand-fg);
}
"#
}
