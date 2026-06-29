use orbital_data::DataValue;

use crate::types::ColumnType;

/// Filter operator for typed [`DataValue`] comparisons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FilterOperator {
    IsEmpty,
    IsNotEmpty,
    Contains,
    NotContains,
    Equals,
    NotEquals,
    StartsWith,
    EndsWith,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Is,
    IsNot,
}

impl FilterOperator {
    /// Operators valid for a given [`ColumnType`].
    pub fn allowed_for(col_type: ColumnType) -> &'static [FilterOperator] {
        match col_type {
            ColumnType::Text => &[
                FilterOperator::Contains,
                FilterOperator::NotContains,
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::StartsWith,
                FilterOperator::EndsWith,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            ColumnType::Number => &[
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::GreaterThan,
                FilterOperator::GreaterThanOrEqual,
                FilterOperator::LessThan,
                FilterOperator::LessThanOrEqual,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            ColumnType::Date => &[
                FilterOperator::Is,
                FilterOperator::IsNot,
                FilterOperator::GreaterThan,
                FilterOperator::GreaterThanOrEqual,
                FilterOperator::LessThan,
                FilterOperator::LessThanOrEqual,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            ColumnType::Boolean => &[FilterOperator::Is, FilterOperator::IsNot],
            ColumnType::SingleSelect => &[
                FilterOperator::Is,
                FilterOperator::IsNot,
                FilterOperator::Contains,
                FilterOperator::Equals,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            ColumnType::Actions => &[],
        }
    }
}

/// How multiple filter rules combine in the filter panel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FilterLogic {
    #[default]
    And,
    Or,
}

/// Single column filter rule.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FilterRule {
    /// Column field key to filter.
    pub field: String,
    /// Comparison operator (must be valid for the column [`ColumnType`]).
    pub operator: FilterOperator,
    /// Operand value for the operator.
    pub value: DataValue,
}

/// Structured filter model for the filter panel and controlled `filter` prop.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataTableFilter {
    /// Active filter rules.
    pub items: Vec<FilterRule>,
    /// How multiple rules combine (`And` or `Or`).
    pub logic: FilterLogic,
}
