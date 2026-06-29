pub const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const MONTH_SHORT_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn month_name(month: u32) -> &'static str {
    MONTH_NAMES
        .get(month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("")
}

pub fn month_short_name(month: u32) -> &'static str {
    MONTH_SHORT_NAMES
        .get(month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("")
}
