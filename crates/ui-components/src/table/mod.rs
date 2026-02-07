mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_EMPTY_TEXT, TableVariant};
pub use view::Table;

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
