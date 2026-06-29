/// Column and gap configuration for [`crate::Grid`].
#[derive(Clone, Copy, Default)]
pub struct GridConfig {
    /// Number of grid columns.
    pub cols: u16,
    /// Horizontal gap between columns, in pixels.
    pub x_gap: u16,
    /// Vertical gap between rows, in pixels.
    pub y_gap: u16,
}

impl GridConfig {
    pub fn new(cols: u16) -> Self {
        Self {
            cols,
            x_gap: 0,
            y_gap: 0,
        }
    }

    pub fn with_gaps(cols: u16, x_gap: u16, y_gap: u16) -> Self {
        Self { cols, x_gap, y_gap }
    }
}

impl From<u16> for GridConfig {
    fn from(cols: u16) -> Self {
        Self::new(cols)
    }
}

/// Span and offset configuration for [`crate::GridItem`].
#[derive(Clone, Copy, Default)]
pub struct GridItemConfig {
    /// Number of columns this item spans.
    pub span: u16,
    /// Number of empty columns to skip before this item.
    pub offset: u16,
}

impl GridItemConfig {
    pub fn span(span: u16) -> Self {
        Self { span, offset: 0 }
    }

    pub fn with_offset(span: u16, offset: u16) -> Self {
        Self { span, offset }
    }
}

impl From<u16> for GridItemConfig {
    fn from(span: u16) -> Self {
        Self::span(span)
    }
}
