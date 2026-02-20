use crate::table::{
    TableCellAlign, TableColumn, TableDensity, TableLayout, TableRow, TableStateInput,
    logic::{self, TableVariant},
};
use leptos::prelude::*;

#[component]
pub fn Table(
    #[prop(optional)] columns: Vec<TableColumn>,
    #[prop(optional)] rows: Vec<TableRow>,
    #[prop(optional)] variant: TableVariant,
    #[prop(optional)] density: TableDensity,
    #[prop(optional)] layout: TableLayout,
    #[prop(optional)] striped: bool,
    #[prop(optional)] sticky_header: bool,
    #[prop(optional, into)] caption: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let columns = logic::normalize_columns(columns);
    let rows = logic::normalize_rows(rows, columns.len());
    let column_keys: Vec<String> = columns.iter().map(|column| column.key.clone()).collect();
    let column_aligns: Vec<TableCellAlign> = columns.iter().map(|column| column.align).collect();

    let caption = logic::normalize_optional_text(caption);
    let has_caption = caption.is_some();
    let empty_label = logic::normalize_empty_text(empty_label);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(TableStateInput {
        variant,
        density,
        layout,
        striped,
        sticky_header,
        has_caption,
        row_count: rows.len(),
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    let columns = StoredValue::new(columns);
    let rows = StoredValue::new(rows);
    let column_keys = StoredValue::new(column_keys);
    let column_aligns = StoredValue::new(column_aligns);
    let caption = StoredValue::new(caption);
    let empty_label = StoredValue::new(empty_label);

    view! {
        <div
            class=class
            data-slot="table"
            data-variant=state.variant_attr
            data-density=state.density_attr
            data-layout=state.layout_attr
            data-state=state.data_state_attr
            data-striped=state.is_striped.then_some("true")
            data-sticky-header=state.has_sticky_header.then_some("true")
            data-has-caption=state.has_caption.then_some("true")
            data-row-count=state.row_count.to_string()
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role="region"
            aria-label=aria_label
        >
            <table class="ui-table__table" data-slot="table-element">
                {caption.get_value().map(|caption| {
                    view! {
                        <caption class="ui-table__caption" data-slot="table-caption">
                            {caption}
                        </caption>
                    }
                })}
                <thead class="ui-table__head" data-slot="table-head">
                    <tr class="ui-table__head-row" data-slot="table-head-row">
                        {columns
                            .get_value()
                            .into_iter()
                            .map(|column| {
                                let class = format!("ui-table__head-cell {}", column.align.class_name());
                                let key_attr = column.key.clone();
                                view! {
                                    <th
                                        class=class
                                        scope="col"
                                        data-slot="table-head-cell"
                                        data-column=key_attr
                                        data-align=column.align.as_attr()
                                    >
                                        {column.label}
                                    </th>
                                }
                            })
                            .collect_view()}
                    </tr>
                </thead>
                <tbody class="ui-table__body" data-slot="table-body">
                    {if state.is_empty {
                        view! {
                            <tr class="ui-table__row ui-table__row--empty" data-slot="table-row-empty" data-row-id="empty">
                                {columns
                                    .get_value()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, column)| {
                                        let align = column.align;
                                        let class = format!("ui-table__cell {}", align.class_name());
                                        let key_attr = column.key;
                                        let content = if index == 0 {
                                            empty_label.get_value()
                                        } else {
                                            String::new()
                                        };
                                        view! {
                                            <td
                                                class=class
                                                data-slot="table-cell"
                                                data-column=key_attr
                                                data-align=align.as_attr()
                                                aria-hidden=(index > 0).then_some("true")
                                            >
                                                {content}
                                            </td>
                                        }
                                    })
                                    .collect_view()}
                            </tr>
                        }
                        .into_any()
                    } else {
                        rows
                            .get_value()
                            .into_iter()
                            .map(|row| {
                                let row_id_attr = row.id.clone();
                                view! {
                                    <tr class="ui-table__row" data-slot="table-row" data-row-id=row_id_attr>
                                        {row
                                            .cells
                                            .into_iter()
                                            .enumerate()
                                            .map(|(index, cell)| {
                                                let column_keys = column_keys.get_value();
                                                let column_aligns = column_aligns.get_value();
                                                let column_key = column_keys
                                                    .get(index)
                                                    .cloned()
                                                    .unwrap_or_else(|| format!("col-{}", index + 1));
                                                let align = column_aligns
                                                    .get(index)
                                                    .copied()
                                                    .unwrap_or(TableCellAlign::Start);
                                                let class = format!("ui-table__cell {}", align.class_name());

                                                view! {
                                                    <td
                                                        class=class
                                                        data-slot="table-cell"
                                                        data-column=column_key
                                                        data-align=align.as_attr()
                                                    >
                                                        {cell}
                                                    </td>
                                                }
                                            })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }}
                </tbody>
            </table>
        </div>
    }
}
