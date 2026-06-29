use orbital_base_components::OffsetTarget;

/// Configuration for [`Anchor`](crate::Anchor).
#[derive(Default)]
pub struct AnchorConfig {
    /// Scroll container used to calculate link offsets.
    pub offset_target: Option<OffsetTarget>,
}
