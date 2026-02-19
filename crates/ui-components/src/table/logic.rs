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
mod tests {
    use super::*;
    use crate::table::{TableDensity, TableLayout, TableStateInput};

    #[test]
    fn variant_class_names_and_attrs_are_stable() {
        assert_eq!(
            TableVariant::Default.class_name(),
            "ui-table--variant-default"
        );
        assert_eq!(TableVariant::Quiet.class_name(), "ui-table--variant-quiet");
        assert_eq!(
            TableVariant::Outline.class_name(),
            "ui-table--variant-outline"
        );

        assert_eq!(TableVariant::Default.as_attr(), "default");
        assert_eq!(TableVariant::Quiet.as_attr(), "quiet");
        assert_eq!(TableVariant::Outline.as_attr(), "outline");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" ready ".to_string())),
            Some("ready".to_string())
        );
    }

    #[test]
    fn normalize_columns_and_rows_shape_data() {
        let columns = normalize_columns(vec![
            TableColumn::new("  ", " Service "),
            TableColumn::new("uptime", " ").with_align(TableCellAlign::End),
        ]);
        assert_eq!(columns[0].key, "col-1");
        assert_eq!(columns[0].label, "Service");
        assert_eq!(columns[1].label, "Column 2");

        let rows = normalize_rows(
            vec![TableRow::new(" ", vec![" API ".to_string()])],
            columns.len(),
        );

        assert_eq!(rows[0].id, "row-1");
        assert_eq!(
            rows[0].cells,
            vec!["API".to_string(), DEFAULT_EMPTY_TEXT.into()]
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_data_state() {
        let state = resolve_state(TableStateInput {
            variant: TableVariant::Outline,
            density: TableDensity::Compact,
            layout: TableLayout::Fixed,
            striped: true,
            sticky_header: true,
            has_caption: false,
            row_count: 0,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.variant_attr, "outline");
        assert_eq!(state.density_attr, "compact");
        assert_eq!(state.layout_attr, "fixed");
        assert_eq!(state.data_state_attr, "empty");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert!(state.is_empty);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-table".to_string()),
            resolve_state(TableStateInput {
                variant: TableVariant::Quiet,
                density: TableDensity::Comfortable,
                layout: TableLayout::Auto,
                striped: true,
                sticky_header: true,
                has_caption: true,
                row_count: 3,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-table",
            "ui-table--variant-quiet",
            "ui-table--density-comfortable",
            "ui-table--layout-auto",
            "ui-table--striped",
            "ui-table--sticky-header",
            "ui-table--with-caption",
            "ui-table--has-rows",
            "ui-table--custom-class",
            "docs-table",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
