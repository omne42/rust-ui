mod logic;
mod render;
pub mod styles;

pub use logic::{DEFAULT_ARIA_LABEL, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap};
pub use render::Flex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlexStateInput {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify: FlexJustify,
    pub align: FlexAlign,
    pub gap: FlexGap,
    pub inline: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlexState {
    pub direction: FlexDirection,
    pub direction_class: &'static str,
    pub direction_attr: &'static str,
    pub wrap: FlexWrap,
    pub wrap_class: &'static str,
    pub wrap_attr: &'static str,
    pub justify: FlexJustify,
    pub justify_class: &'static str,
    pub justify_attr: &'static str,
    pub align: FlexAlign,
    pub align_class: &'static str,
    pub align_attr: &'static str,
    pub gap: FlexGap,
    pub gap_class: &'static str,
    pub gap_attr: &'static str,
    pub is_inline: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
