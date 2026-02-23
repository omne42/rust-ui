use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::text_input::text_area::TextAreaMotion;
use ui::{
    Checkbox, CheckboxGroup, CheckboxSize, CheckboxVariant, Form, FormLabelAlign,
    FormLabelPosition, Input, InputGroup, InputOtp, InputSize, InputVariant, NumberField, Radio,
    RadioGroup, RadioGroupOrientation, SearchField, SearchFieldMotion, SegmentedControl,
    SegmentedControlItem, SegmentedControlItemSpec, SegmentedControlMotion,
    SegmentedControlOrientation, SegmentedControlSize, Switch, TextArea,
};
use ui_headless::A11yDirection;

#[path = "forms/checkbox.rs"]
mod checkbox;
#[path = "forms/checkbox_group.rs"]
mod checkbox_group;
#[path = "forms/form.rs"]
mod form;
#[path = "forms/input.rs"]
mod input;
#[path = "forms/input_group.rs"]
mod input_group;
#[path = "forms/input_otp.rs"]
mod input_otp;
#[path = "forms/number_field.rs"]
mod number_field;
#[path = "forms/radio.rs"]
mod radio;
#[path = "forms/radio_group.rs"]
mod radio_group;
#[path = "forms/search_field.rs"]
mod search_field;
#[path = "forms/segmented_control.rs"]
mod segmented_control;
#[path = "forms/switch.rs"]
mod switch;
#[path = "forms/text_area.rs"]
mod text_area;

pub(super) use checkbox::checkbox;
pub(super) use checkbox_group::checkbox_group;
pub(super) use form::form;
pub(super) use input::input;
pub(super) use input_group::input_group;
pub(super) use input_otp::input_otp;
pub(super) use number_field::number_field;
pub(super) use radio::radio;
pub(super) use radio_group::radio_group;
pub(super) use search_field::search_field;
pub(super) use segmented_control::segmented_control;
pub(super) use switch::switch;
pub(super) use text_area::text_area;
