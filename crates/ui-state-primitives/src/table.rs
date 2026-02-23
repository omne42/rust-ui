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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableDensity {
    #[default]
    Comfortable,
    Compact,
}

impl TableDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            TableDensity::Comfortable => "ui-table--density-comfortable",
            TableDensity::Compact => "ui-table--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TableDensity::Comfortable => "comfortable",
            TableDensity::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableLayout {
    #[default]
    Auto,
    Fixed,
}

impl TableLayout {
    pub fn class_name(self) -> &'static str {
        match self {
            TableLayout::Auto => "ui-table--layout-auto",
            TableLayout::Fixed => "ui-table--layout-fixed",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TableLayout::Auto => "auto",
            TableLayout::Fixed => "fixed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TableCellAlign {
    #[default]
    Start,
    Center,
    End,
}

impl TableCellAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            TableCellAlign::Start => "ui-table__cell--align-start",
            TableCellAlign::Center => "ui-table__cell--align-center",
            TableCellAlign::End => "ui-table__cell--align-end",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TableCellAlign::Start => "start",
            TableCellAlign::Center => "center",
            TableCellAlign::End => "end",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub align: TableCellAlign,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            align: TableCellAlign::Start,
        }
    }

    pub fn with_align(mut self, align: TableCellAlign) -> Self {
        self.align = align;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
}

impl TableRow {
    pub fn new(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            id: id.into(),
            cells,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableStateInput {
    pub variant: TableVariant,
    pub density: TableDensity,
    pub layout: TableLayout,
    pub striped: bool,
    pub sticky_header: bool,
    pub has_caption: bool,
    pub row_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableState {
    pub variant: TableVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub density: TableDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub layout: TableLayout,
    pub layout_class: &'static str,
    pub layout_attr: &'static str,
    pub is_striped: bool,
    pub has_sticky_header: bool,
    pub has_caption: bool,
    pub row_count: usize,
    pub is_empty: bool,
    pub has_rows: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

#[cfg(test)]
#[path = "test/table.rs"]
mod tests;
