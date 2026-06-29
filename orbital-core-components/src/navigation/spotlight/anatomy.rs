use leptos::prelude::*;

/// Pre-rendered anatomy regions for a spotlight surface.
pub struct SpotlightAnatomyViews {
    pub header: Option<AnyView>,
    pub body: Option<AnyView>,
    pub media: Option<AnyView>,
    pub actions: Option<AnyView>,
    pub footer: Option<AnyView>,
}

/// Shared header/body/media/actions/footer layout for all spotlight components.
pub fn spotlight_anatomy(views: SpotlightAnatomyViews) -> impl IntoView {
    let SpotlightAnatomyViews {
        header,
        body,
        media,
        actions,
        footer,
    } = views;

    view! {
        {header.map(|content| view! {
            <div class="orbital-spotlight__header" data-testid="spotlight-header">
                {content}
            </div>
        })}
        {media.map(|content| view! {
            <div class="orbital-spotlight__media" data-testid="spotlight-media">
                {content}
            </div>
        })}
        {body.map(|content| view! {
            <div class="orbital-spotlight__body" data-testid="spotlight-body">
                {content}
            </div>
        })}
        {actions.map(|content| view! {
            <div class="orbital-spotlight__actions" data-testid="spotlight-actions">
                {content}
            </div>
        })}
        {footer.map(|content| view! {
            <div class="orbital-spotlight__footer" data-testid="spotlight-footer">
                {content}
            </div>
        })}
    }
}
