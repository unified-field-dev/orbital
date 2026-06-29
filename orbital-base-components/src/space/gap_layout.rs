use crate::flex::FlexGap;

/// Shared flex `gap` CSS used by BaseFlex and BaseSpace.
pub fn flex_gap_css(gap: FlexGap) -> String {
    gap.css_value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flex::FlexGap;

    #[test]
    fn gap_presets_match_flex() {
        assert_eq!(flex_gap_css(FlexGap::Small), "4px 8px");
        assert_eq!(flex_gap_css(FlexGap::Medium), "8px 12px");
        assert_eq!(flex_gap_css(FlexGap::Large), "12px 16px");
    }
}
