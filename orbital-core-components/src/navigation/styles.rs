use turf::inline_style_sheet_values;

#[allow(dead_code)]
pub struct NavigationClassNames {
    pub surface: &'static str,
    pub root_column: &'static str,
    pub header: &'static str,
    pub body: &'static str,
    pub footer: &'static str,
    pub item: &'static str,
    pub item_selected: &'static str,
    pub item_disabled: &'static str,
    pub item_sub: &'static str,
    pub item_app: &'static str,
    pub category_header: &'static str,
    pub category_header_selected: &'static str,
    pub category_chevron: &'static str,
    pub category_chevron_open: &'static str,
    pub category_label: &'static str,
    pub category_icon: &'static str,
    pub category_root: &'static str,
    pub collapsed_trigger: &'static str,
    pub category_subitems: &'static str,
    pub sub_item_group: &'static str,
    pub sub_item_group_hidden: &'static str,
    pub section_header: &'static str,
    pub section_header_band: &'static str,
    pub category_header_section_folder: &'static str,
    pub divider: &'static str,
    pub item_row: &'static str,
    pub item_badge: &'static str,
    pub collapse_footer: &'static str,
}

pub fn navigation_styles() -> (&'static str, NavigationClassNames) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Surface {
            width: var(--orbital-navigation-width, 260px);
            max-width: 100%;
            height: 100%;
            overflow: hidden;
            transition: width 0.3s ease;
        }

        .RootColumn {
            height: 100%;
            min-height: 0;
        }

        .Header {
            flex-shrink: 0;
            padding: var(--orb-space-block-sm) var(--orb-space-inline-mnudge);
        }

        .Body {
            flex: 1;
            min-height: 0;
            padding: 0 var(--orb-space-block-mnudge);
        }

        .Footer {
            flex-shrink: 0;
            padding: 0 var(--orb-space-block-mnudge);
        }

        .ItemRow {
            display: flex;
            align-items: center;
            gap: var(--orb-space-inline-sm);
            width: 100%;
        }

        .orbital-navigation-item {
            width: 100%;
            box-sizing: border-box;
        }

        .orbital-navigation-item--focused > .Item {
            outline: 2px solid var(--orb-color-brand-stroke);
            outline-offset: -2px;
        }

        .Item, .CategoryHeader, .AppItem {
            display: flex;
            text-transform: none;
            position: relative;
            justify-content: flex-start;
            align-items: center;
            gap: var(--orb-space-block-lg);
            width: 100%;
            padding: var(--orb-space-block-mnudge);
            background-color: inherit;
            border-radius: var(--orb-radius-md);
            color: var(--orb-color-text-secondary);
            text-decoration-line: none;
            border: none;
            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-sm);
            font-weight: var(--orb-type-weight-regular);
            line-height: var(--orb-type-line-md);
            cursor: pointer;
            box-sizing: border-box;
        }

        .Item:hover, .CategoryHeader:hover, .AppItem:hover {
            background-color: var(--orb-color-surface-overlay-hover);
        }

        .Item:active, .CategoryHeader:active, .AppItem:active {
            background-color: var(--orb-color-surface-overlay-pressed);
        }

        .ItemSelected::after,
        .CategoryHeaderSelected[aria-expanded="false"]::after {
            content: "";
            position: absolute;
            width: 4px;
            height: 20px;
            background-color: var(--orb-color-text-secondary-brand-selected);
            border-radius: var(--orb-radius-circular);
            margin-inline-start: -18px;
        }

        .ItemSub {
            padding-inline-start: 46px;
        }

        .ItemSub.ItemSelected::after {
            margin-inline-start: -54px;
        }

        .ItemDisabled {
            opacity: 0.5;
            cursor: not-allowed;
            pointer-events: none;
        }

        .AppItem {
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
        }

        .CategoryChevron {
            margin-inline-start: auto;
            height: 20px;
            display: flex;
            align-items: center;
            transition: transform var(--orb-motion-duration-md) var(--orb-motion-ease-emphasis);
        }

        .CategoryChevronOpen {
            transform: rotate(90deg);
        }

        .CategoryLabel {
            flex: 1;
            text-align: start;
        }

        .CategoryIcon {
            display: flex;
            align-items: center;
        }

        .CategoryRoot {
            position: relative;
        }

        .CollapsedTrigger {
            box-sizing: border-box;
        }

        .CategorySubitems {
            box-sizing: border-box;
        }

        .orbital-navigation__category-collapsed-trigger {
            display: none;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-header-expanded {
            display: none;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-collapsed-trigger {
            display: flex;
            justify-content: center;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-subitems {
            display: none;
            position: absolute;
            left: calc(100% + 4px);
            top: 0;
            min-width: 200px;
            padding: var(--orb-space-block-xs);
            background: var(--orb-color-surface-canvas);
            border: 1px solid var(--orb-color-border-subtle);
            border-radius: var(--orb-radius-md);
            box-shadow: var(--orb-elev-raised-md);
            z-index: 1000;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-root:hover .orbital-navigation__category-subitems,
        .orbital-navigation--collapsed .orbital-navigation__category-root:focus-within .orbital-navigation__category-subitems {
            display: block;
        }

        .SubItemGroup {
            overflow: hidden;
        }

        .SubItemGroupHidden {
            display: none;
        }

        .SectionHeader {
            padding: var(--orb-space-block-sm) var(--orb-space-block-mnudge);
            font-size: var(--orb-type-size-md);
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-tertiary);
            text-transform: none;
        }

        .SectionHeaderBand {
            color: var(--orb-color-text-quaternary);
            font-weight: var(--orb-type-weight-semibold);
            text-transform: uppercase;
            font-size: var(--orb-type-size-sm);
        }

        .CategoryHeaderSectionFolder {
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
        }

        .CategoryHeaderSectionFolder .CategoryLabel {
            flex: 1;
        }

        .Divider {
            height: 1px;
            background: var(--orb-color-border-subtle);
            margin: var(--orb-space-block-sm) 0;
            border: none;
        }

        .ItemBadge {
            margin-inline-start: auto;
            font-size: var(--orb-type-size-xs);
            background: var(--orb-color-brand-bg);
            color: var(--orb-color-text-on-brand);
            border-radius: var(--orb-radius-circular);
            padding: 0 6px;
            min-width: 18px;
            text-align: center;
        }

        .CollapseFooter {
            padding: var(--orb-space-block-sm) 0;
        }

        .orbital-navigation--density-compact .Item,
        .orbital-navigation--density-compact .CategoryHeader,
        .orbital-navigation--density-compact .AppItem {
            padding: var(--orb-space-block-snudge) var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .Surface {
            width: var(--orbital-navigation-collapsed-width, 48px);
        }

        .orbital-navigation--collapsed .Body {
            padding: 0;
        }

        .orbital-navigation--collapsed .CategoryLabel,
        .orbital-navigation--collapsed .CategoryChevron,
        .orbital-navigation--collapsed .SectionHeader,
        .orbital-navigation--collapsed .ItemBadge,
        .orbital-navigation--collapsed .CollapseFooter span {
            display: none;
        }

        .orbital-navigation--collapsed .CategoryHeader,
        .orbital-navigation--collapsed .Item,
        .orbital-navigation--collapsed .AppItem {
            justify-content: center;
            padding-inline: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .ItemSub {
            padding-inline-start: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .CollapsedFlyout .ItemSub {
            padding-inline-start: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--closed {
            display: none;
        }
    };

    (
        style_sheet,
        NavigationClassNames {
            surface: class_names.surface,
            root_column: class_names.root_column,
            header: class_names.header,
            body: class_names.body,
            footer: class_names.footer,
            item: class_names.item,
            item_selected: class_names.item_selected,
            item_disabled: class_names.item_disabled,
            item_sub: class_names.item_sub,
            item_app: class_names.app_item,
            category_header: class_names.category_header,
            category_header_selected: class_names.category_header_selected,
            category_chevron: class_names.category_chevron,
            category_chevron_open: class_names.category_chevron_open,
            category_label: class_names.category_label,
            category_icon: class_names.category_icon,
            category_root: class_names.category_root,
            collapsed_trigger: class_names.collapsed_trigger,
            category_subitems: class_names.category_subitems,
            sub_item_group: class_names.sub_item_group,
            sub_item_group_hidden: class_names.sub_item_group_hidden,
            section_header: class_names.section_header,
            section_header_band: class_names.section_header_band,
            category_header_section_folder: class_names.category_header_section_folder,
            divider: class_names.divider,
            item_row: class_names.item_row,
            item_badge: class_names.item_badge,
            collapse_footer: class_names.collapse_footer,
        },
    )
}
