bitflags::bitflags! {
    /// Opt-in date picker capabilities (single-tier; no license checks).
    pub struct DatePickerFeatures: u32 {
        /// Date/time/datetime range pickers and range fields (DP-15..21).
        const RANGE_PICKERS = 0b0001;
        /// TimeClock and DigitalClock surfaces (DP-11, DP-12).
        const CLOCK_VIEWS = 0b0010;
    }
}
