pub use crate::iconset::{IconsetGlyph, IconsetSize as IconsUiSize, IconsetTone as IconsUiTone};

mod logic;
pub mod styles;
mod view;

pub use view::IconsUi;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsUiStateInput {
    pub disabled: bool,
    pub decorative: bool,
    pub has_explicit_icon_reference: bool,
    pub used_default_icon_reference: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_glyphs: bool,
    pub has_custom_size: bool,
    pub has_custom_tone: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconsUiState {
    pub is_disabled: bool,
    pub is_decorative: bool,
    pub has_explicit_icon_reference: bool,
    pub used_default_icon_reference: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_glyphs: bool,
    pub has_custom_size: bool,
    pub has_custom_tone: bool,
    pub state_attr: &'static str,
    pub icon_reference_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub glyph_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub tone_source_attr: &'static str,
}
