use leptos::prelude::*;
use orbital_base_components::{BaseUpload, Handler, UploadFileList};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::{upload_dragger_styles, upload_styles};
use super::types::UploadConfig;

/// File selection trigger wired to a hidden native file input.
///
/// Upload wires a hidden file input to whatever trigger you provide — a button, icon, or [`UploadDragger`](crate::UploadDragger) drop zone. Set `accept` and `multiple` on [`UploadConfig`], then handle selected files in `on_change`. Orbital does not upload to a server, show progress, or manage a file list. You own validation, preview, and network calls.
///
/// # When to use
///
/// - File selection with a custom trigger (button, drop zone, icon)
/// - Drag-and-drop selection affordances via [`UploadDragger`](crate::UploadDragger)
/// - Form-associated file inputs with `config.name` / `config.id`
///
/// # Scope
///
/// Upload is selection-only. For progress bars, thumbnails, or managed file lists, compose [`Upload`] with your own UI and network layer.
///
/// # Usage
///
/// 1. Provide trigger UI as children (button or [`UploadDragger`](crate::UploadDragger)).
/// 2. Handle selected files in `on_change`.
/// 3. Set `config.accept` and `config.multiple` to constrain selection.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use [`UploadDragger`](crate::UploadDragger) for drop-zone affordances
/// * Validate file type and size in `on_change` before uploading
///
/// ## Don'ts
///
/// * Do not rely on the hidden input alone for UX — always provide a visible trigger
/// * Do not expect built-in upload progress or file-list UI — implement those in app code
///
/// # Examples
///
/// ## Basic button trigger
/// Hidden file input with a button trigger; `on_change` receives the selected files.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, Button, ButtonAppearance};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-preview">
///         <Upload config=UploadConfig::accept("image/*") on_change=Handler::on(|_files: UploadFileList| {})>
///             <Button appearance=ButtonAppearance::Secondary>"Choose file"</Button>
///         </Upload>
///     </div>
/// }
/// ```
///
/// ## Multiple files
/// Input exposes the `multiple` attribute for multi-select dialogs.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, Button, ButtonAppearance};
/// use leptos::prelude::*;
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-multiple">
///         <Upload
///             config=UploadConfig { multiple: true.into(), ..UploadConfig::accept("image/*") }
///             on_change=Handler::on(|_files: UploadFileList| {})
///         >
///             <Button appearance=ButtonAppearance::Secondary>"Choose files"</Button>
///         </Upload>
///     </div>
/// }
/// ```
///
/// ## Accept filter
/// Restricts the picker to image MIME types via `accept`.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, Button, ButtonAppearance};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-accept">
///         <Upload config=UploadConfig::accept("image/*") on_change=Handler::on(|_files: UploadFileList| {})>
///             <Button appearance=ButtonAppearance::Secondary>"Images only"</Button>
///         </Upload>
///     </div>
/// }
/// ```
///
/// ## Drop zone
/// [`UploadDragger`] provides a dashed drop target that highlights on drag-over.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, UploadDragger};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-dragger" style="width: 100%; max-width: 420px;">
///         <Upload config=UploadConfig::default() on_change=Handler::on(|_files: UploadFileList| {})>
///             <UploadDragger>"Drop files here or click to browse"</UploadDragger>
///         </Upload>
///     </div>
/// }
/// ```
///
/// ## Custom trigger
/// [`UploadDragger`] can wrap any trigger content, including native buttons.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, UploadDragger, Button, ButtonAppearance};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-custom" style="width: 100%; max-width: 420px;">
///         <Upload config=UploadConfig::default() on_change=Handler::on(|_files: UploadFileList| {})>
///             <UploadDragger>
///                 <Button appearance=ButtonAppearance::Primary>"Browse files"</Button>
///             </UploadDragger>
///         </Upload>
///     </div>
/// }
/// ```
///
/// ## Callback with selected files
/// `on_change` receives the browser [`FileList`](crate::FileList) when files are selected.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, Button, ButtonAppearance};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-callback">
///         <Upload
///             config=UploadConfig { multiple: true.into(), ..Default::default() }
///             on_change=Handler::on(|files: UploadFileList| {
///                 let _count = files.length();
///             })
///         >
///             <Button appearance=ButtonAppearance::Secondary>"Select files"</Button>
///         </Upload>
///         <p data-testid="upload-file-list">"Choose files to populate the list"</p>
///     </div>
/// }
/// ```
///
/// ## Theme stroke
/// Drop-zone border uses neutral stroke tokens from the theme.
/// <!-- preview -->
/// ```rust
/// use crate::{Upload, UploadConfig, UploadDragger};
/// use orbital_base_components::{Handler, UploadFileList};
/// view! {
///     <div data-testid="upload-theme" style="width: 100%; max-width: 420px;">
///         <Upload config=UploadConfig::default() on_change=Handler::on(|_files: UploadFileList| {})>
///             <div data-testid="upload-theme-dragger">
///                 <UploadDragger>"Themed drop zone"</UploadDragger>
///             </div>
///         </Upload>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "upload",
    preview_label = "Upload",
    preview_icon = icondata::AiCloudUploadOutlined,
)]
#[component]
pub fn Upload(
    /// Accept filter, multiple flag, and form association fields.
    #[prop(default = UploadConfig::default())]
    config: UploadConfig,
    /// Handler invoked when files are selected or dropped.
    #[prop(into)]
    on_change: Handler<UploadFileList>,
    /// Extra CSS class names merged onto the hidden file input wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Trigger UI (button, [`UploadDragger`](crate::UploadDragger), etc.).
    children: Children,
) -> impl IntoView {
    inject_style("orbital-upload", upload_styles());
    inject_style("orbital-upload-dragger", upload_dragger_styles());

    view! {
        <BaseUpload
            class=class
            id=config.id
            name=config.name
            accept=config.accept
            multiple=config.multiple
            on_change=on_change
        >
            {children()}
        </BaseUpload>
    }
}
