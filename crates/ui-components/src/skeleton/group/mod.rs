mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant,
};
pub use view::SkeletonGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonGroupStateInput {
    pub is_loading: bool,
    pub is_skeleton_only: bool,
    pub variant: SkeletonGroupVariant,
    pub layout: SkeletonGroupLayout,
    pub density: SkeletonGroupDensity,
    pub has_custom_is_loading: bool,
    pub has_custom_is_skeleton_only: bool,
    pub has_custom_variant: bool,
    pub has_custom_layout: bool,
    pub has_custom_density: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonGroupState {
    pub is_loading: bool,
    pub is_loaded: bool,
    pub is_skeleton_only: bool,
    pub should_hide_root: bool,
    pub variant: SkeletonGroupVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub layout: SkeletonGroupLayout,
    pub layout_class: &'static str,
    pub layout_attr: &'static str,
    pub density: SkeletonGroupDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub state_attr: &'static str,
    pub visibility_attr: &'static str,
    pub loading_mode_attr: &'static str,
    pub loading_source_attr: &'static str,
    pub skeleton_only_source_attr: &'static str,
    pub variant_source_attr: &'static str,
    pub layout_source_attr: &'static str,
    pub density_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
