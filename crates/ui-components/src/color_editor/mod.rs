mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_LABEL};
pub use view::ColorEditor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorEditorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsb,
}

impl ColorEditorFormat {
    pub fn as_attr(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "hex",
            ColorEditorFormat::Rgb => "rgb",
            ColorEditorFormat::Hsl => "hsl",
            ColorEditorFormat::Hsb => "hsb",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "ui-color-editor--format-hex",
            ColorEditorFormat::Rgb => "ui-color-editor--format-rgb",
            ColorEditorFormat::Hsl => "ui-color-editor--format-hsl",
            ColorEditorFormat::Hsb => "ui-color-editor--format-hsb",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ColorEditorFormat::Hex => "HEX",
            ColorEditorFormat::Rgb => "RGB",
            ColorEditorFormat::Hsl => "HSL",
            ColorEditorFormat::Hsb => "HSB",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorStateInput {
    pub disabled: bool,
    pub hide_alpha_channel: bool,
    pub format: ColorEditorFormat,
    pub has_selection: bool,
    pub has_custom_motion: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorState {
    pub is_disabled: bool,
    pub hide_alpha_channel: bool,
    pub format: ColorEditorFormat,
    pub format_class: &'static str,
    pub data_state_attr: &'static str,
    pub format_attr: &'static str,
    pub alpha_visibility_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
