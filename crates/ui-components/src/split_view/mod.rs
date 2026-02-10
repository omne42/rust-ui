mod logic;
pub mod styles;
mod view;

pub use view::SplitView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitViewStateInput {
    pub orientation: crate::resizable::ResizableOrientation,
    pub disabled: bool,
    pub with_handle: bool,
    pub is_controlled: bool,
    pub has_custom_default_split: bool,
    pub has_custom_bounds: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_change_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitViewState {
    pub orientation: crate::resizable::ResizableOrientation,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub with_handle: bool,
    pub is_controlled: bool,
    pub has_custom_default_split: bool,
    pub has_custom_bounds: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_change_handler: bool,
    pub orientation_attr: &'static str,
    pub state_attr: &'static str,
    pub split_mode_attr: &'static str,
    pub handle_attr: &'static str,
    pub default_split_source_attr: &'static str,
    pub bounds_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}
