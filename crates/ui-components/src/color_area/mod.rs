mod logic;
pub mod styles;
mod view;

pub use view::ColorArea;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaStateInput {
    pub disabled: bool,
    pub step: f32,
    pub value: (f32, f32),
    pub grid_size: usize,
    pub has_preview_color: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaState {
    pub is_disabled: bool,
    pub step: f32,
    pub value_x: f32,
    pub value_y: f32,
    pub value_x_percent: u8,
    pub value_y_percent: u8,
    pub grid_size: usize,
    pub selected_col: usize,
    pub selected_row: usize,
    pub data_state_attr: &'static str,
    pub has_preview_color: bool,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
