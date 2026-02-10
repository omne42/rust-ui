pub use crate::iconset::{IconsetGlyph as IconsGlyph, IconsetTone as IconsTone};

mod logic;
pub mod styles;
mod view;

pub use view::Icons;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconsSet {
    #[default]
    Ui,
    Workflow,
}

impl IconsSet {
    pub fn as_attr(self) -> &'static str {
        match self {
            IconsSet::Ui => "ui",
            IconsSet::Workflow => "workflow",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconsScale {
    #[default]
    Medium,
    Large,
}

impl IconsScale {
    pub fn as_attr(self) -> &'static str {
        match self {
            IconsScale::Medium => "medium",
            IconsScale::Large => "large",
        }
    }

    pub fn as_ui_size(self) -> crate::icons_ui::IconsUiSize {
        match self {
            IconsScale::Medium => crate::icons_ui::IconsUiSize::Md,
            IconsScale::Large => crate::icons_ui::IconsUiSize::Lg,
        }
    }

    pub fn as_workflow_size(self) -> crate::icons_workflow::IconsWorkflowSize {
        match self {
            IconsScale::Medium => crate::icons_workflow::IconsWorkflowSize::Md,
            IconsScale::Large => crate::icons_workflow::IconsWorkflowSize::Lg,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsStateInput {
    pub set: IconsSet,
    pub scale: IconsScale,
    pub disabled: bool,
    pub decorative: bool,
    pub has_set_prefix_in_name: bool,
    pub has_custom_set_prop: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_glyphs: bool,
    pub has_custom_tone: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsState {
    pub set: IconsSet,
    pub scale: IconsScale,
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub has_set_prefix_in_name: bool,
    pub has_custom_set_prop: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_glyphs: bool,
    pub has_custom_tone: bool,
    pub set_attr: &'static str,
    pub scale_attr: &'static str,
    pub state_attr: &'static str,
    pub set_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub glyph_source_attr: &'static str,
    pub tone_source_attr: &'static str,
}
