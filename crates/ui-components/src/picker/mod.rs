mod logic;
pub mod styles;
mod view;

pub use view::Picker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickerStateInput {
    pub disabled: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_disabled_indices: bool,
    pub is_controlled: bool,
    pub default_open: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_open_handler: bool,
    pub has_custom_class_name: bool,
    pub has_custom_placement: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickerState {
    pub is_disabled: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_disabled_indices: bool,
    pub is_controlled: bool,
    pub default_open: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_open_handler: bool,
    pub has_custom_class_name: bool,
    pub has_custom_placement: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
    pub selection_attr: &'static str,
    pub disabled_options_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub initial_open_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub handler_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
