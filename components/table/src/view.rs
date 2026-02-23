use crate::table::{
    TableColumn, TableDensity, TableLayout, TableRow, TableStateInput,
    logic::{self, TableVariant},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, TableA11yOptions, use_table_a11y};

#[component]
pub fn Table(
    #[prop(optional)] columns: Vec<TableColumn>,
    #[prop(optional)] rows: Vec<TableRow>,
    #[prop(optional)] variant: TableVariant,
    #[prop(optional)] density: TableDensity,
    #[prop(optional)] layout: TableLayout,
    #[prop(optional)] is_striped: bool,
    #[prop(optional)] is_sticky_header: bool,
    #[prop(optional, into)] caption: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let columns = logic::normalize_columns(columns);
    let rows = logic::normalize_rows(rows, columns.len());
    let column_cells = columns
        .iter()
        .map(|column| (column.key.clone(), column.align))
        .collect::<Vec<_>>();

    let caption = logic::normalize_optional_text(caption);
    let has_caption = caption.is_some();
    let empty_label = logic::normalize_empty_text(empty_label);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(TableStateInput {
        variant,
        density,
        layout,
        striped: is_striped,
        sticky_header: is_sticky_header,
        has_caption,
        row_count: rows.len(),
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });
    let table_a11y = use_table_a11y(TableA11yOptions {
        state,
        aria_label,
        lang,
        dir,
    });
    let table_a11y_attrs = table_a11y.attrs;

    let class = logic::compose_class_name(class_name, state);

    let columns = StoredValue::new(columns);
    let rows = StoredValue::new(rows);
    let column_cells = StoredValue::new(column_cells);
    let caption = StoredValue::new(caption);
    let empty_label = StoredValue::new(empty_label);

    view! {
        <div
            class=class
            data-slot="table"
            data-variant=table_a11y_attrs.data_variant
            data-density=table_a11y_attrs.data_density
            data-layout=table_a11y_attrs.data_layout
            data-state=table_a11y_attrs.data_state
            data-striped=table_a11y_attrs.data_striped
            data-sticky-header=table_a11y_attrs.data_sticky_header
            data-has-caption=table_a11y_attrs.data_has_caption
            data-row-count=table_a11y_attrs.data_row_count
            data-aria-source=table_a11y_attrs.data_aria_source
            data-custom-class=table_a11y_attrs.data_custom_class
            data-class-source=table_a11y_attrs.data_class_source
            role=table_a11y_attrs.role
            aria-label=table_a11y_attrs.aria_label
            lang=table_a11y_attrs.lang
            dir=table_a11y_attrs.dir
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
                                let column_cells = column_cells.get_value();
                                view! {
                                    <tr class="ui-table__row" data-slot="table-row" data-row-id=row_id_attr>
                                        {row
                                            .cells
                                            .into_iter()
                                            .zip(column_cells.into_iter())
                                            .map(|(cell, (column_key, align))| {
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
