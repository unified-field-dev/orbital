use leptos::context::Provider;
use leptos::prelude::*;
use orbital_base_components::{BaseNavigation, CollectionStateInjection, NavigationInjection};
use orbital_macros::component_doc;

use super::collapse::NavigationCollapseToggle;
use super::slots::{
    default_navigation_material, NavigationBody, NavigationFooter, NavigationHeader,
    NavigationMaterial,
};
use super::styles::navigation_styles;
use super::types::NavigationConfig;
use crate::material::material_modifier_classes;
use crate::material::{material_flat_outline_modifier, MaterialOutlineEdge};
use crate::{Flex, Material, ScrollArea};

/// `Navigation` is a side-rail compound for app shells — selection, expandable categories,
/// and optional collapse into an icon rail.
///
/// Bind [`NavigationConfig::selected_value`] and [`NavigationConfig::open_categories`] with
/// Leptos signals; compose items inside [`NavigationBody`]. Pair with
/// [`NavigationLink`](orbital::NavigationLink) when using Leptos Router for client-side active styling.
/// Theme width uses `--orbital-navigation-width` and `--orbital-navigation-width-collapsed`.
///
/// # When to use
///
/// - Primary app shell wayfinding — routes, grouped tools, admin sections
/// - Expandable category groups with nested sub-items
/// - Collapsible rail that shrinks to icon-only width in dense layouts
///
/// # Usage
///
/// 1. Create `selected_value` and `open_categories` signals at the app shell root.
/// 2. Build [`NavigationConfig`] with `.with_selected_value(…)` and `.with_open_categories(…)`.
/// 3. Compose [`NavigationBody`] with [`NavigationItem`], [`NavigationCategory`], and section headers.
/// 4. Optional: [`NavigationHeader`] / [`NavigationFooter`] slots; [`NavigationMaterial`] for surface treatment.
///
/// # Examples
///
/// ## Default
/// Side rail with icon-and-label items and a parent-owned selected value. Bind signals at the shell so route changes can sync selection.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationBody, NavigationConfig, NavigationItem};
/// let selected = RwSignal::new(Some("home".to_string()));
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-preview" style="height: 240px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationItem config="home" icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
///                 <NavigationItem config="settings" icon=icondata::AiSettingOutlined>"Settings"</NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## Categories
/// Expandable category groups with nested sub-items bound to `open_categories`. Use [`NavigationCategory`] when a section has many related links that should stay grouped under one header.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Navigation, NavigationBody, NavigationCategory, NavigationCategoryHeader,
///     NavigationConfig, NavigationSubItem, NavigationSubItemGroup,
/// };
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec!["tools".to_string()]);
/// view! {
///     <div data-testid="navigation-categories" style="height: 280px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationCategory value="tools">
///                     <NavigationCategoryHeader slot icon=icondata::AiToolOutlined>"Tools"</NavigationCategoryHeader>
///                     <NavigationSubItemGroup>
///                         <NavigationSubItem config="a">"Tool A"</NavigationSubItem>
///                         <NavigationSubItem config="b">"Tool B"</NavigationSubItem>
///                     </NavigationSubItemGroup>
///                 </NavigationCategory>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## Sections
/// Static section labels that divide items into labeled blocks without accordion behavior. Add [`NavigationSectionHeader`] rows to organize long nav lists into scannable groups.
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationBody, NavigationItem, NavigationSectionHeader};
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-sections" style="height: 240px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationSectionHeader>"Workspace"</NavigationSectionHeader>
///                 <NavigationItem config="a" icon=icondata::AiHomeOutlined>"Overview"</NavigationItem>
///                 <NavigationSectionHeader>"Admin"</NavigationSectionHeader>
///                 <NavigationItem config="b" icon=icondata::AiSettingOutlined>"Settings"</NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## App item
/// Branding row at the top of the rail linking back to the app root. [`NavigationAppItem`] pairs an icon with an `href` for product identity above everyday route links.
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationAppItem, NavigationBody, NavigationItem};
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-app-item" style="height: 220px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationAppItem icon=icondata::AiUserOutlined href=Signal::derive(|| "/".to_string())>
///                     "Contoso HR"
///                 </NavigationAppItem>
///                 <NavigationItem config="home" icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## Links
/// Navigation rows that navigate via `href` instead of selection-only state. Pass `href` on [`NavigationItem`] when the row should behave like a router link.
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationBody, NavigationConfig, NavigationItem, NavigationItemConfig};
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-links" style="height: 200px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationItem config=NavigationItemConfig::new("docs").with_href(Signal::derive(|| "/docs".to_string())) icon=icondata::AiFileOutlined>
///                     "Documentation"
///                 </NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## External link
/// Outbound link that opens in a new tab with `target="_blank"`. Use for documentation, support, or third-party destinations that leave the current app.
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationBody, NavigationConfig, NavigationItem, NavigationItemConfig};
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-target-blank" style="height: 200px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open)>
///             <NavigationBody slot>
///                 <NavigationItem config=NavigationItemConfig::new("ext").with_target("_blank").with_href(Signal::derive(|| "https://example.com".to_string())) icon=icondata::AiExportOutlined>
///                     "External"
///                 </NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## Density compact
/// Tighter row spacing via `density=NavigationDensity::Compact`. Choose compact density for secondary sidebars, nested panels, or layouts that need more links in less vertical space.
/// <!-- preview -->
/// ```rust
/// use crate::{Navigation, NavigationBody, NavigationConfig, NavigationDensity, NavigationItem};
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec![] as Vec<String>);
/// view! {
///     <div data-testid="navigation-density-compact" style="height: 200px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open).with_density(NavigationDensity::Compact)>
///             <NavigationBody slot>
///                 <NavigationItem config="a" icon=icondata::AiHomeOutlined>"Compact row"</NavigationItem>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
///
/// ## Collapsible rail
/// Collapsible side rail that shrinks to icon-only width with `collapsible=true` and a parent-owned `collapsed` signal. Keeps categories and badges usable while reclaiming horizontal space in dense layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Navigation, NavigationBody, NavigationCategory, NavigationCategoryHeader,
///     NavigationConfig, NavigationItem, NavigationItemConfig, NavigationSubItem,
///     NavigationSubItemGroup,
/// };
/// let selected = RwSignal::new(None::<String>);
/// let open = RwSignal::new(vec!["tools".to_string()]);
/// let collapsed = RwSignal::new(true);
/// view! {
///     <div data-testid="navigation-collapsible" style="height: 260px">
///         <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open).with_collapsible(true).with_collapsed(collapsed)>
///             <NavigationBody slot>
///                 <NavigationItem config=NavigationItemConfig::new("home").with_badge("3") icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
///                 <NavigationCategory value="tools">
///                     <NavigationCategoryHeader slot icon=icondata::AiToolOutlined>"Tools"</NavigationCategoryHeader>
///                     <NavigationSubItemGroup>
///                         <NavigationSubItem config="a">"Tool A"</NavigationSubItem>
///                     </NavigationSubItemGroup>
///                 </NavigationCategory>
///             </NavigationBody>
///         </Navigation>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "navigation",
    preview_label = "Navigation",
    preview_icon = icondata::AiMenuOutlined,
)]
#[component]
pub fn Navigation(
    /// Extra CSS class names merged onto the Material surface root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional `data-testid` for E2E hooks on the navigation root.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// Selection, expansion, density, and collapse settings.
    #[prop(optional, into)]
    config: NavigationConfig,
    /// Optional [`Material`] treatment override for the navigation surface.
    #[prop(optional)]
    navigation_material: Option<NavigationMaterial>,
    /// Optional header slot — branding, search, or utility controls above the body.
    #[prop(optional)]
    navigation_header: Option<NavigationHeader>,
    /// Required body slot — items, categories, and section headers via [`NavigationBody`].
    #[prop(optional)]
    navigation_body: Option<NavigationBody>,
    /// Optional footer slot — secondary links or account controls below the body.
    #[prop(optional)]
    navigation_footer: Option<NavigationFooter>,
) -> impl IntoView {
    let NavigationConfig {
        selected_value,
        selected_category_value,
        open_categories,
        multiple,
        density,
        mode,
        open,
        collapsible,
        collapsed,
    } = config;

    let (variant, elevation, corners) = navigation_material
        .map(|m| (m.variant, m.elevation, m.corners))
        .unwrap_or_else(default_navigation_material);
    let collapsed_signal = Signal::derive(move || collapsed.get());

    let density_signal = Signal::derive(move || density);
    let injection = NavigationInjection::new(
        selected_value,
        selected_category_value,
        open_categories,
        multiple,
        density_signal,
        collapsed,
        open,
    );
    let collection_state = injection.collection.clone();

    let (style_sheet, class_names) = navigation_styles();
    let material_modifiers = material_modifier_classes(variant, elevation, corners);
    let outline = material_flat_outline_modifier(variant, elevation, MaterialOutlineEdge::End)
        .map(|class| format!(" {class}"))
        .unwrap_or_default();
    let material_class = format!(
        "orbital-navigation__material {} orbital-material {material_modifiers}{outline}",
        class_names.surface
    );

    let test_id = data_testid
        .get()
        .unwrap_or_else(|| "navigation".to_string());

    view! {
        <style>{style_sheet}</style>
        <Provider value=CollectionStateInjection(collection_state)>
            <Provider value=injection>
            <BaseNavigation
                data_testid=test_id
                density=density
                mode=mode
                open=open
                collapsed=collapsed_signal
                class=class
            >
                <Material class=material_class variant=variant elevation=elevation corners=corners>
                    <Flex vertical=true fill=true full_width=true class=class_names.root_column>
                        {navigation_header.map(|slot| view! {
                            <div class=class_names.header>{(slot.children)()}</div>
                        })}
                        {navigation_body.map(|slot| view! {
                            <ScrollArea class=class_names.body>
                                {(slot.children)()}
                            </ScrollArea>
                        })}
                        {navigation_footer.map(|slot| view! {
                            <div class=class_names.footer>{(slot.children)()}</div>
                        })}
                        {collapsible.then(|| view! { <NavigationCollapseToggle /> })}
                    </Flex>
                </Material>
            </BaseNavigation>
            </Provider>
        </Provider>
    }
}
