#[path = "icons/mod.rs"]
pub mod icons;
#[path = "ui/mod.rs"]
pub mod icons_ui;
#[path = "workflow/mod.rs"]
pub mod icons_workflow;
#[path = "set/mod.rs"]
pub mod iconset;
mod logic;
mod protocol;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, IconSize, IconTone};
pub use view::Icon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSlotKind {
    None,
    Label,
    Description,
    Icon,
    Custom,
}

impl IconSlotKind {
    pub fn as_attr(self) -> &'static str {
        match self {
            IconSlotKind::None => "none",
            IconSlotKind::Label => "label",
            IconSlotKind::Description => "description",
            IconSlotKind::Icon => "icon",
            IconSlotKind::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconStateInput {
    pub size: IconSize,
    pub tone: IconTone,
    pub disabled: bool,
    pub decorative: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub slot_kind: IconSlotKind,
    pub has_named_slot: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconState {
    pub size: IconSize,
    pub tone: IconTone,
    pub size_class: &'static str,
    pub tone_class: &'static str,
    pub size_attr: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub has_accessible_name: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub slot_kind: IconSlotKind,
    pub has_named_slot: bool,
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
