mod logic;
pub mod styles;
mod view;

pub use view::Tags;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagsStateInput {
    pub disabled: bool,
    pub has_tags: bool,
    pub has_disabled_tags: bool,
    pub has_removable_tags: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_remove_handler: bool,
    pub has_custom_id_base: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_variant: bool,
    pub has_custom_size: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagsState {
    pub is_disabled: bool,
    pub has_tags: bool,
    pub has_disabled_tags: bool,
    pub has_removable_tags: bool,
    pub is_invalid: bool,
    pub is_required: bool,
    pub has_remove_handler: bool,
    pub has_custom_id_base: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_aria_describedby: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_variant: bool,
    pub has_custom_size: bool,
    pub state_attr: &'static str,
    pub content_attr: &'static str,
    pub removal_attr: &'static str,
    pub constraint_attr: &'static str,
    pub id_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub describedby_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub variant_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub handler_source_attr: &'static str,
}
