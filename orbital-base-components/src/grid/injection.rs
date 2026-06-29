use leptos::prelude::*;

#[derive(Clone)]
pub struct GridInjection {
    pub x_gap: Signal<u16>,
}

impl GridInjection {
    pub fn new(x_gap: Signal<u16>) -> Self {
        Self { x_gap }
    }
}

pub fn use_grid() -> GridInjection {
    expect_context()
}
