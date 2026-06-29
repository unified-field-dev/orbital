/// Header chrome inset provided by a Fixed or Sticky app bar.
///
/// Consumed by layout regions to reserve space matching app bar density.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppBarInset {
    pub height_px: u16,
}
