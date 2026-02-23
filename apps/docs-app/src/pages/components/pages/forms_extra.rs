use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys as browser_sys;
use ui::time_field::TimeFieldMotion;
use ui::{
    Calendar, CalendarFirstWeekday, CalendarTone, DateField, DateFieldTone, DatePicker,
    DatePickerMotion, DatePickerTone, DateRangePicker, DateRangePickerTone, Description,
    DescriptionElement, DescriptionTone, ErrorMessage, ErrorMessageElement, ErrorMessageTone,
    Field, FieldError, FieldErrorTone, FieldOrientation, FieldTone, Fieldset, FieldsetOrientation,
    FieldsetTone, HelpText, HelpTextTone, Label, LabelEmphasis, PopoverMotion, SegmentedControl,
    SegmentedControlSize, Slider, SliderMotion, Snippet, Switch, Textarea, TimeField,
    TimeFieldTone, field_form::field::FieldMotion,
};
use ui_headless::{A11yDirection, PopoverPlacement};

#[cfg(target_arch = "wasm32")]
const FIELD_WORKBENCH_STORAGE_KEY: &str = "docs:field:workbench:v1";
#[cfg(target_arch = "wasm32")]
const FIELD_WORKBENCH_STORAGE_VERSION: u8 = 1;

#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
struct FieldWorkbenchState {
    orientation_key: String,
    tone_key: String,
    required: bool,
    invalid: bool,
    disabled: bool,
    custom_class: bool,
    custom_error: bool,
    motion_ms: u16,
}

impl Default for FieldWorkbenchState {
    fn default() -> Self {
        Self {
            orientation_key: "vertical".to_string(),
            tone_key: "default".to_string(),
            required: true,
            invalid: false,
            disabled: false,
            custom_class: false,
            custom_error: false,
            motion_ms: 160,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize, serde::Deserialize)]
struct FieldWorkbenchStorage {
    version: u8,
    state: FieldWorkbenchState,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum FieldWorkbenchStorageError {
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    UnsupportedVersion(u8),
}

#[cfg(target_arch = "wasm32")]
impl FieldWorkbenchStorageError {
    fn as_code(&self) -> &'static str {
        match self {
            Self::Serialize(_) => "serialize_error",
            Self::Deserialize(_) => "deserialize_error",
            Self::UnsupportedVersion(_) => "unsupported_version",
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FieldWorkbenchState {
    fn sanitize(self) -> Self {
        let Self {
            orientation_key,
            tone_key,
            required,
            invalid,
            disabled,
            custom_class,
            custom_error,
            motion_ms,
        } = self;

        let orientation_key = if orientation_key == "horizontal" {
            "horizontal".to_string()
        } else {
            "vertical".to_string()
        };

        let tone_key = if tone_key == "muted" {
            "muted".to_string()
        } else {
            "default".to_string()
        };

        Self {
            orientation_key,
            tone_key,
            required,
            invalid,
            disabled,
            custom_class,
            custom_error,
            motion_ms: motion_ms.clamp(1, 800),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FieldWorkbenchState {
    fn encode(self) -> Result<String, FieldWorkbenchStorageError> {
        serde_json::to_string(&FieldWorkbenchStorage {
            version: FIELD_WORKBENCH_STORAGE_VERSION,
            state: self.sanitize(),
        })
        .map_err(FieldWorkbenchStorageError::Serialize)
    }

    fn decode(raw: &str) -> Result<Self, FieldWorkbenchStorageError> {
        let storage: FieldWorkbenchStorage =
            serde_json::from_str(raw).map_err(FieldWorkbenchStorageError::Deserialize)?;
        if storage.version != FIELD_WORKBENCH_STORAGE_VERSION {
            return Err(FieldWorkbenchStorageError::UnsupportedVersion(
                storage.version,
            ));
        }

        Ok(storage.state.sanitize())
    }
}

#[cfg(target_arch = "wasm32")]
fn load_field_workbench_state() -> Option<FieldWorkbenchState> {
    let storage = browser_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(FIELD_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    match FieldWorkbenchState::decode(&raw) {
        Ok(state) => Some(state),
        Err(error) => {
            leptos::logging::warn!(
                "field workbench decode failed: code={} error={error:?}",
                error.as_code()
            );
            None
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_field_workbench_state() -> Option<FieldWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_field_workbench_state(state: FieldWorkbenchState) {
    if let Some(storage) =
        browser_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        match state.encode() {
            Ok(encoded) => {
                drop(storage.set_item(FIELD_WORKBENCH_STORAGE_KEY, &encoded));
            }
            Err(error) => {
                leptos::logging::warn!(
                    "field workbench encode failed: code={} error={error:?}",
                    error.as_code()
                );
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_field_workbench_state(_state: FieldWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_field_workbench_state() {
    if let Some(storage) =
        browser_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(FIELD_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_field_workbench_state() {}

#[path = "forms_extra/calendar.rs"]
mod calendar;
#[path = "forms_extra/date_field.rs"]
mod date_field;
#[path = "forms_extra/date_picker.rs"]
mod date_picker;
#[path = "forms_extra/date_range_picker.rs"]
mod date_range_picker;
#[path = "forms_extra/description.rs"]
mod description;
#[path = "forms_extra/error_message.rs"]
mod error_message;
#[path = "forms_extra/field.rs"]
mod field;
#[path = "forms_extra/field_error.rs"]
mod field_error;
#[path = "forms_extra/fieldset.rs"]
mod fieldset;
#[path = "forms_extra/help_text.rs"]
mod help_text;
#[path = "forms_extra/label.rs"]
mod label;
#[path = "forms_extra/slider.rs"]
mod slider;
#[path = "forms_extra/textarea.rs"]
mod textarea;
#[path = "forms_extra/time_field.rs"]
mod time_field;

pub(super) use calendar::calendar;
pub(super) use date_field::date_field;
pub(super) use date_picker::date_picker;
pub(super) use date_range_picker::date_range_picker;
pub(super) use description::description;
pub(super) use error_message::error_message;
pub(super) use field::field;
pub(super) use field_error::field_error;
pub(super) use fieldset::fieldset;
pub(super) use help_text::help_text;
pub(super) use label::label;
pub(super) use slider::slider;
pub(super) use textarea::textarea;
pub(super) use time_field::time_field;
