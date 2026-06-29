use std::collections::HashMap;

use leptos::prelude::*;
use orbital_data::DataValue;

/// Built-in aggregate functions for numeric and count summaries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AggregationFn {
    #[default]
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

/// One column aggregation rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AggregationRule {
    pub field: String,
    pub func: AggregationFn,
}

/// Aggregation model mapping field keys to functions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AggregationModel {
    pub rules: Vec<AggregationRule>,
}

impl AggregationModel {
    pub fn new(rules: Vec<AggregationRule>) -> Self {
        Self { rules }
    }

    pub fn is_active(&self) -> bool {
        !self.rules.is_empty()
    }

    pub fn rule_for_field(&self, field: &str) -> Option<AggregationFn> {
        self.rules.iter().find(|r| r.field == field).map(|r| r.func)
    }
}

/// Where aggregate values are rendered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AggregationPosition {
    #[default]
    Footer,
    GroupInline,
}

/// Signal-backed aggregation model.
pub type AggregationSignal = Signal<AggregationModel>;

/// Inline aggregate values keyed by field (for group header rows).
pub type GroupAggregates = HashMap<String, DataValue>;
