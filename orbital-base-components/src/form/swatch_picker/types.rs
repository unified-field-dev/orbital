/// Layout mode for [`super::BaseSwatchPicker`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwatchPickerLayout {
    #[default]
    Row,
    Grid,
}

/// Swatch shape for [`super::BaseSwatchPickerItem`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwatchPickerShape {
    #[default]
    Rounded,
    Square,
}

/// Swatch size for [`super::BaseSwatchPickerItem`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwatchPickerSize {
    #[default]
    Small,
    Medium,
}
