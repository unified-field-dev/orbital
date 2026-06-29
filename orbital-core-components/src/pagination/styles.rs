/// Pagination layout styles (`orbital-pagination*`).
pub fn pagination_styles() -> &'static str {
    r#"
.orbital-pagination {
    display: flex;
    column-gap: 5px;
}

.orbital-pagination-item,
.orbital-button.orbital-pagination-item {
    max-width: 32px;
    min-width: 32px;
}

div.orbital-pagination-item {
    display: flex;
    justify-content: center;
    align-items: center;
}
"#
}
