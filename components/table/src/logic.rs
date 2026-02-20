use crate::table::{TableCellAlign, TableColumn, TableRow, TableState, TableStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Data table";
pub const DEFAULT_EMPTY_TEXT: &str = "No data";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableVariant {
    #[default]
    Default,
    Quiet,
    Outline,
}

impl TableVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            TableVariant::Default => "ui-table--variant-default",
            TableVariant::Quiet => "ui-table--variant-quiet",
            TableVariant::Outline => "ui-table--variant-outline",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TableVariant::Default => "default",
            TableVariant::Quiet => "quiet",
            TableVariant::Outline => "outline",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_empty_text(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_EMPTY_TEXT.into())
}

pub fn normalize_columns(columns: Vec<TableColumn>) -> Vec<TableColumn> {
    if columns.is_empty() {
        return vec![
            TableColumn::new("name", "Name"),
            TableColumn::new("value", "Value").with_align(TableCellAlign::End),
        ];
    }

    columns
        .into_iter()
        .enumerate()
        .map(|(index, column)| {
            let key = normalize_optional_text(Some(column.key))
                .unwrap_or_else(|| format!("col-{}", index + 1));
            let label = normalize_optional_text(Some(column.label))
                .unwrap_or_else(|| format!("Column {}", index + 1));

            TableColumn {
                key,
                label,
                align: column.align,
            }
        })
        .collect()
}

pub fn normalize_rows(rows: Vec<TableRow>, column_count: usize) -> Vec<TableRow> {
    let effective_column_count = column_count.max(1);

    rows.into_iter()
        .enumerate()
        .map(|(index, row)| {
            let id = normalize_optional_text(Some(row.id))
                .unwrap_or_else(|| format!("row-{}", index + 1));

            let mut cells: Vec<String> = row
                .cells
                .into_iter()
                .map(|cell| {
                    normalize_optional_text(Some(cell)).unwrap_or_else(|| DEFAULT_EMPTY_TEXT.into())
                })
                .collect();

            if cells.len() > effective_column_count {
                cells.truncate(effective_column_count);
            } else if cells.len() < effective_column_count {
                cells.resize(effective_column_count, DEFAULT_EMPTY_TEXT.into());
            }

            TableRow { id, cells }
        })
        .collect()
}

pub fn resolve_state(input: TableStateInput) -> TableState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let is_empty = input.row_count == 0;

    TableState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        density: input.density,
        density_class: input.density.class_name(),
        density_attr: input.density.as_attr(),
        layout: input.layout,
        layout_class: input.layout.class_name(),
        layout_attr: input.layout.as_attr(),
        is_striped: input.striped,
        has_sticky_header: input.sticky_header,
        has_caption: input.has_caption,
        row_count: input.row_count,
        is_empty,
        has_rows: !is_empty,
        data_state_attr: if is_empty { "empty" } else { "data" },
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TableState) -> String {
    let mut classes = vec![
        "ui-table".to_string(),
        state.variant_class.into(),
        state.density_class.into(),
        state.layout_class.into(),
    ];

    if state.is_striped {
        classes.push("ui-table--striped".to_string());
    }
    if state.has_sticky_header {
        classes.push("ui-table--sticky-header".to_string());
    }
    if state.has_caption {
        classes.push("ui-table--with-caption".to_string());
    }
    if state.is_empty {
        classes.push("ui-table--empty".to_string());
    } else {
        classes.push("ui-table--has-rows".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-table--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
