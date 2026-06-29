use leptos::prelude::*;
use orbital_base_components::BaseAccordion;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use std::collections::HashSet;

#[cfg(feature = "preview")]
use super::item::{AccordionHeader, AccordionItem};
use super::styles::accordion_styles;

/// `Accordion` groups expandable panels for FAQs, settings sections, or nested detail.
///
/// Bind `open_items` to a `RwSignal<HashSet<String>>` of open panel ids. Set `multiple` to
/// allow several panels open at once and `collapsible` so the last open panel can close.
/// Each [`AccordionItem`] needs a distinct `value` and an [`AccordionHeader`] slot.
/// Orbital tracks open state in one parent set — not separate per-panel booleans.
///
/// # When to use
///
/// - Group related content that users expand on demand — policy sections, advanced settings, nested detail
/// - Single open panel by default; enable `multiple` when users compare sections side by side
/// - For a single expand/collapse region without a shared header list, compose a one-off panel instead
///
/// # Usage
///
/// 1. Create `open_items` as `RwSignal::new(HashSet::new())` (or seed with initial ids).
/// 2. Pass the signal to `Accordion` and give each [`AccordionItem`] a matching `value`.
/// 3. Put the clickable label in [`AccordionHeader`]; panel body is the item's remaining children.
/// 4. Optional: `multiple=true` for several open panels; `collapsible=true` to close the last panel.
/// 5. Optional: pass `motion: MotionSlot` on [`AccordionItem`] to customize expand/collapse transitions (defaults to theme motion).
///
/// # Examples
///
/// ## Single panel
/// Parent-owned `open_items` set with one panel expanded. Bind the signal at your page root so URL or save logic can sync open sections.
/// <!-- preview -->
/// ```rust
/// let open = RwSignal::new(HashSet::from(["one".to_string()]));
/// view! {
///     <div data-testid="accordion-preview">
///         <Accordion open_items=open>
///             <AccordionItem value="one">
///                 <AccordionHeader slot>"Section one"</AccordionHeader>
///                 <p data-testid="accordion-panel-one">"Panel content"</p>
///             </AccordionItem>
///             <AccordionItem value="two">
///                 <AccordionHeader slot>"Section two"</AccordionHeader>
///                 <p data-testid="accordion-panel-two">"Hidden panel"</p>
///             </AccordionItem>
///         </Accordion>
///     </div>
/// }
/// ```
///
/// ## Multiple open
/// Accordion that allows several panels to stay expanded with `multiple=true`. Choose this when users compare content across sections or need several groups visible at once.
/// <!-- preview -->
/// ```rust
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="accordion-multiple">
///         <Accordion open_items=open multiple=true>
///             <AccordionItem value="a">
///                 <AccordionHeader slot>"A"</AccordionHeader>
///                 <p data-testid="accordion-panel-a">"A body"</p>
///             </AccordionItem>
///             <AccordionItem value="b">
///                 <AccordionHeader slot>"B"</AccordionHeader>
///                 <p data-testid="accordion-panel-b">"B body"</p>
///             </AccordionItem>
///         </Accordion>
///     </div>
/// }
/// ```
///
/// ## Collapsible
/// Single-section accordion where the open panel can be collapsed entirely via `collapsible=true`. Use when every section should be closable, including the last remaining panel.
/// <!-- preview -->
/// ```rust
/// let open = RwSignal::new(HashSet::from(["only".to_string()]));
/// view! {
///     <div data-testid="accordion-collapsible">
///         <Accordion open_items=open collapsible=true>
///             <AccordionItem value="only">
///                 <AccordionHeader slot>"Only section"</AccordionHeader>
///                 <p data-testid="accordion-panel-only">"Can close last panel"</p>
///             </AccordionItem>
///         </Accordion>
///     </div>
/// }
/// ```
///
/// ## Keyboard toggle
/// Accordion headers that respond to keyboard activation (Enter/Space) for expand and collapse. Pair with `collapsible=true` when focus-driven workflows must toggle panels without a mouse.
/// <!-- preview -->
/// ```rust
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="accordion-keyboard">
///         <Accordion open_items=open collapsible=true>
///             <AccordionItem value="kb">
///                 <AccordionHeader slot>"Keyboard section"</AccordionHeader>
///                 <p data-testid="accordion-panel-kb">"Toggle with Enter"</p>
///             </AccordionItem>
///         </Accordion>
///     </div>
/// }
/// ```
///
/// ## Theme tokens
/// Accordion styled with Orbital theme tokens on headers and panels. Use as a baseline when matching accordion chrome to surrounding surfaces and typography.
/// <!-- preview -->
/// ```rust
/// let open = RwSignal::new(HashSet::from(["theme".to_string()]));
/// view! {
///     <div data-testid="accordion-theme">
///         <Accordion open_items=open>
///             <AccordionItem value="theme">
///                 <AccordionHeader slot>"Themed header"</AccordionHeader>
///                 <p>"Panel body"</p>
///             </AccordionItem>
///         </Accordion>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "accordion",
    preview_label = "Accordion",
    preview_icon = icondata::AiDownOutlined,
)]
#[component]
pub fn Accordion(
    /// Extra CSS class names merged onto the accordion root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Parent-owned set of open panel `value` strings; updates as panels expand or collapse.
    #[prop(default = RwSignal::new(HashSet::new()), into)]
    open_items: RwSignal<HashSet<String>>,
    /// When true, more than one panel may be open at a time.
    #[prop(optional)]
    multiple: bool,
    /// When true, clicking an open panel header collapses it (single-open mode still closes others).
    #[prop(optional)]
    collapsible: bool,
    /// [`AccordionItem`] children defining each expandable section.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-accordion", accordion_styles());

    view! {
        <BaseAccordion
            class=class
            open_items=open_items
            multiple=multiple
            collapsible=collapsible
        >
            {children()}
        </BaseAccordion>
    }
}
