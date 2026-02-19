mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, GridAlign, GridColumns, GridGap, GridJustify, GridRows};
pub use motion::GridMotion;
pub use view::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridStateInput {
    pub columns: GridColumns,
    pub rows: GridRows,
    pub gap: GridGap,
    pub justify: GridJustify,
    pub align: GridAlign,
    pub dense: bool,
    pub inline: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridState {
    pub columns: GridColumns,
    pub columns_class: &'static str,
    pub columns_attr: &'static str,
    pub rows: GridRows,
    pub rows_class: &'static str,
    pub rows_attr: &'static str,
    pub gap: GridGap,
    pub gap_class: &'static str,
    pub gap_attr: &'static str,
    pub justify: GridJustify,
    pub justify_class: &'static str,
    pub justify_attr: &'static str,
    pub align: GridAlign,
    pub align_class: &'static str,
    pub align_attr: &'static str,
    pub is_dense: bool,
    pub is_inline: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
