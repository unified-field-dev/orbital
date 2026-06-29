use leptos::prelude::*;
use orbital_base_components::{Handler, OverlayDismiss};
use orbital_core_components::{
    Button, ButtonAppearance, Popover, PopoverLifecycle, PopoverPosition, PopoverSize,
    PopoverTrigger, PopoverTriggerType, Select, SelectSize,
};

use crate::types::{AggregationFn, DataTableFeatures, DataTablePivotModel, DataTableTableState};

fn field_options_view(options: &[(String, String)]) -> impl IntoView + use<> {
    options
        .iter()
        .map(|(v, label)| {
            let val = v.clone();
            let lbl = label.clone();
            view! { <option value=val>{lbl}</option> }
        })
        .collect_view()
}

/// Pivot configuration panel (Popover) for row/column/value field assignment.
#[component]
pub fn DataTablePivotPanel(state: DataTableTableState) -> impl IntoView {
    if !state.features.contains(DataTableFeatures::PIVOTING) {
        return view! { <span></span> }.into_any();
    }

    let field_options: StoredValue<Vec<(String, String)>> = StoredValue::new(
        state
            .columns
            .get_value()
            .iter()
            .map(|c| (c.field.clone(), c.header_name.clone()))
            .collect(),
    );

    let row_field = RwSignal::new(String::new());
    let col_field = RwSignal::new(String::new());
    let value_field = RwSignal::new(String::new());

    let load_draft = move || {
        let current = state.pivot.get();
        row_field.set(current.row_fields.first().cloned().unwrap_or_default());
        col_field.set(current.column_fields.first().cloned().unwrap_or_default());
        value_field.set(current.value_fields.first().cloned().unwrap_or_default());
    };

    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::BottomEnd
            size=PopoverSize::Large
            lifecycle=PopoverLifecycle {
                on_open: Some(Handler::new(load_draft)),
                ..Default::default()
            }
        >
            <PopoverTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    attr:data-testid="data-table-pivot-trigger"
                >
                    "Pivot"
                </Button>
            </PopoverTrigger>
            <DataTablePivotPanelContent
                state=state
                field_options=field_options
                row_field=row_field
                col_field=col_field
                value_field=value_field
            />
        </Popover>
    }
    .into_any()
}

#[component]
fn DataTablePivotPanelContent(
    state: DataTableTableState,
    field_options: StoredValue<Vec<(String, String)>>,
    row_field: RwSignal<String>,
    col_field: RwSignal<String>,
    value_field: RwSignal<String>,
) -> impl IntoView {
    let dismiss = expect_context::<OverlayDismiss>();

    let apply_pivot = Callback::new(move |_| {
        let mut model = DataTablePivotModel::default();
        let rf = row_field.get();
        let cf = col_field.get();
        let vf = value_field.get();
        if !rf.is_empty() {
            model.row_fields.push(rf);
        }
        if !cf.is_empty() {
            model.column_fields.push(cf);
        }
        if !vf.is_empty() {
            model.value_fields.push(vf);
        }
        model.value_fn = AggregationFn::Sum;
        state.set_pivot(model);
        dismiss.close.run(());
    });

    view! {
        <div
            class="orbital-data-table__pivot-panel"
            data-testid="data-table-pivot-panel"
        >
            <div class="orbital-data-table__pivot-panel-title">"Pivot configuration"</div>
            <div class="orbital-data-table__pivot-form">
                <label>"Row field"</label>
                <Select
                    bind=row_field
                    attr:data-testid="data-table-pivot-row-field"
                    appearance=SelectSize::Medium
                >
                    <option value="">"(none)"</option>
                    {field_options_view(&field_options.get_value())}
                </Select>
                <label>"Column field"</label>
                <Select
                    bind=col_field
                    attr:data-testid="data-table-pivot-col-field"
                    appearance=SelectSize::Medium
                >
                    <option value="">"(none)"</option>
                    {field_options_view(&field_options.get_value())}
                </Select>
                <label>"Value field"</label>
                <Select
                    bind=value_field
                    attr:data-testid="data-table-pivot-value-field"
                    appearance=SelectSize::Medium
                >
                    <option value="">"(none)"</option>
                    {field_options_view(&field_options.get_value())}
                </Select>
            </div>
            <div class="orbital-data-table__pivot-panel-actions">
                <span data-testid="data-table-pivot-apply">
                    <Button appearance=ButtonAppearance::Primary on_click=apply_pivot>
                        "Apply pivot"
                    </Button>
                </span>
            </div>
        </div>
    }
}
