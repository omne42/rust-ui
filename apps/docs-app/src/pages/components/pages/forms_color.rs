use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::color::handle::ColorHandleMotion;
use ui::color::loupe::ColorLoupeOutputState;
use ui::color_thumb::ColorThumbMotion;
use ui::{
    ColorArea, ColorEditor, ColorEditorFormat, ColorField, ColorHandle, ColorLoupe, ColorPicker,
    ColorSlider, ColorSliderChannel, ColorSliderMotion, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorThumb, ColorWheel, ColorWheelMotion, SegmentedControl, SegmentedControlSize, Switch,
};
use ui_headless::A11yDirection;

#[path = "forms_color/color_area.rs"]
mod color_area;
#[path = "forms_color/color_editor.rs"]
mod color_editor;
#[path = "forms_color/color_field.rs"]
mod color_field;
#[path = "forms_color/color_handle.rs"]
mod color_handle;
#[path = "forms_color/color_loupe.rs"]
mod color_loupe;
#[path = "forms_color/color_picker.rs"]
mod color_picker;
#[path = "forms_color/color_slider.rs"]
mod color_slider;
#[path = "forms_color/color_thumb.rs"]
mod color_thumb;
#[path = "forms_color/color_wheel.rs"]
mod color_wheel;

pub(super) use color_area::color_area;
pub(super) use color_editor::color_editor;
pub(super) use color_field::color_field;
pub(super) use color_handle::color_handle;
pub(super) use color_loupe::color_loupe;
pub(super) use color_picker::color_picker;
pub(super) use color_slider::color_slider;
pub(super) use color_thumb::color_thumb;
pub(super) use color_wheel::color_wheel;
