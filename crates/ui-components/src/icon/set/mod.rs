pub use crate::icon::{IconSize as IconsetSize, IconTone as IconsetTone};

mod logic;
pub mod styles;
mod view;

pub use view::Iconset;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconsetGlyph {
    pub name: String,
    pub glyph: String,
    pub aria_label: Option<String>,
}

impl IconsetGlyph {
    pub fn new(name: impl Into<String>, glyph: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            glyph: glyph.into(),
            aria_label: None,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsetStateInput {
    pub disabled: bool,
    pub decorative: bool,
    pub has_registry_match: bool,
    pub has_registry_label: bool,
    pub has_custom_iconset_prop: bool,
    pub has_iconset_in_icon_reference: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_size: bool,
    pub has_custom_tone: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsetState {
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub has_registry_match: bool,
    pub has_registry_label: bool,
    pub has_custom_iconset_prop: bool,
    pub has_iconset_in_icon_reference: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_size: bool,
    pub has_custom_tone: bool,
    pub state_attr: &'static str,
    pub icon_source_attr: &'static str,
    pub iconset_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub tone_source_attr: &'static str,
}
