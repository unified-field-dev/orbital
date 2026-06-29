bitflags::bitflags! {
    /// Opt-in scheduler capabilities (single-tier; no license checks).
    pub struct SchedulerFeatures: u32 {
        /// RFC 5545 recurrence expansion (SC-04).
        const RECURRING_EVENTS = 0b0001;
        /// [`SchedulerDataSource`] lazy load (SC-11, SC-23).
        const LAZY_LOADING = 0b0010;
        /// [`SchedulerTimeline`] product (SC-17..28 rollup).
        const TIMELINE = 0b0100;
    }
}
