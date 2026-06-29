pub const YEAR_PANEL_SPAN: i32 = 12;

pub fn year_panel_start(year: i32) -> i32 {
    year - (year.rem_euclid(YEAR_PANEL_SPAN))
}

pub fn build_years(start_year: i32) -> Vec<i32> {
    (0..YEAR_PANEL_SPAN)
        .map(|offset| start_year + offset)
        .collect()
}
