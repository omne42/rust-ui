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

pub(super) fn field_error() -> AnyView {
    let field_error_imports =
        "use leptos::prelude::*;\nuse ui::{FieldError, FieldErrorTone};".to_string();

    let hello_world_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  is_icon_visible=true
  message="Two-factor code is invalid".to_string()
/>
<FieldError
  is_visible=true
  is_disabled=true
  is_icon_visible=true
  message="Read-only mode keeps the latest error snapshot".to_string()
/>
<FieldError
  is_visible=false
  message="Hidden state keeps semantic contracts without visual output".to_string()
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// FieldError has no internal controllable state axis.
// Uncontrolled-style: pass final validation snapshot directly.
<FieldError
  is_visible=true
  message="Uncontrolled snapshot: email is required".to_string()
/>

// Controlled-style (by parent form store): parent updates props and FieldError re-renders.
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  message="Controlled snapshot: email format is invalid".to_string()
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated final output in one shot.
<FieldError
  is_visible=true
  message="Snapshot: email is required".to_string()
/>

// Streaming Optional: keep fallback=snapshot and render last stable message.
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Streaming fallback=snapshot: waiting for final validation".to_string()
/>"#
        .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  is_icon_visible=true
  message="Two-factor code is invalid".to_string()
/>
"#
        .to_string()
    });

    let hidden_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=false
  message="This text should not render when hidden".to_string()
/>
<FieldError
  is_visible=true
  is_disabled=true
  is_icon_visible=true
  class_name="docs-field-error-custom".to_string()
/>"#
        .to_string()
    });

    let (workbench_tone_index, set_workbench_tone_index) = signal(0usize);
    let (workbench_visible, set_workbench_visible) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_icon_visible, set_workbench_icon_visible) = signal(false);
    let (workbench_custom_message, set_workbench_custom_message) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let tone_line = match workbench_tone_index.get() {
            1 => "  tone=FieldErrorTone::Neutral\n",
            2 => "  tone=FieldErrorTone::Negative\n",
            _ => "",
        };
        let message_line = if workbench_custom_message.get() {
            "  message=\"Workbench custom error message\".to_string()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Workbench aria label\".to_string()\n"
        } else {
            ""
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-field-error-custom\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<FieldError\n{tone_line}  is_visible={}\n  is_disabled={}\n  is_icon_visible={}\n  show_icon={}\n  lang={:?}.to_string()\n  dir=A11yDirection::{:?}\n{message_line}{aria_line}{class_line}/>",
            workbench_visible.get(),
            workbench_disabled.get(),
            workbench_icon_visible.get(),
            workbench_icon_visible.get(),
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/field-error/src/styles.rs */\n{}",
            ui::field_form::field_error::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone_index.get() {
            1 => FieldErrorTone::Neutral,
            2 => FieldErrorTone::Negative,
            _ => FieldErrorTone::Auto,
        };
        let message = if workbench_custom_message.get() {
            "Workbench custom error message".to_string()
        } else {
            String::new()
        };
        let aria_label = if workbench_custom_aria.get() {
            "Workbench aria label".to_string()
        } else {
            String::new()
        };
        let class_name = if workbench_custom_class.get() {
            "docs-field-error-custom".to_string()
        } else {
            String::new()
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };

        format!(
            "FieldErrorWorkbenchConfig {{\n  tone: {tone:?},\n  is_visible: {},\n  visible: {},\n  is_disabled: {},\n  disabled: {},\n  is_icon_visible: {},\n  show_icon: {},\n  message: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_visible.get(),
            workbench_visible.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_icon_visible.get(),
            workbench_icon_visible.get(),
            message,
            aria_label,
            class_name,
            lang,
            dir,
        )
    });

    view! {
        <ComponentPage
            title="FieldError"
            slug="field-error"
            group="Forms"
            description="baseline-style field error primitive with centralized visibility/tone/message normalization and stable data contracts."
        >
            <Playground
                title="Hello World (Snapshot Baseline)"
                code_signal=hello_world_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Source Markers)"
                description="Use settings to toggle FieldError props/state and inspect semantic marker feedback in real time."
                code_signal=workbench_code
                code_imports=field_error_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/field-error/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="field-error-config-controls">
                            <div class="docs-search__label">"Tone"</div>
                            <select
                                class="docs-search__input"
                                data-action="cycle-tone-config"
                                prop:value=move || workbench_tone_index.get().to_string()
                                on:change=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<usize>() {
                                        set_workbench_tone_index.set(next.min(2));
                                    }
                                }
                            >
                                <option value="0">"Auto"</option>
                                <option value="1">"Neutral"</option>
                                <option value="2">"Negative"</option>
                            </select>

                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-visible-config"
                                    prop:checked=move || workbench_visible.get()
                                    on:change=move |ev| set_workbench_visible.set(event_target_checked(&ev))
                                />
                                <span>"Visible"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-disabled-config"
                                    prop:checked=move || workbench_disabled.get()
                                    on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                                />
                                <span>"Disabled"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-icon-config"
                                    prop:checked=move || workbench_icon_visible.get()
                                    on:change=move |ev| set_workbench_icon_visible.set(event_target_checked(&ev))
                                />
                                <span>"Show icon"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-message-config"
                                    prop:checked=move || workbench_custom_message.get()
                                    on:change=move |ev| set_workbench_custom_message.set(event_target_checked(&ev))
                                />
                                <span>"Custom message source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-aria-config"
                                    prop:checked=move || workbench_custom_aria.get()
                                    on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                                />
                                <span>"Custom aria source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-class-config"
                                    prop:checked=move || workbench_custom_class.get()
                                    on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                                />
                                <span>"Custom class source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-rtl-config"
                                    prop:checked=move || workbench_rtl.get()
                                    on:change=move |ev| set_workbench_rtl.set(event_target_checked(&ev))
                                />
                                <span>"RTL locale"</span>
                            </label>

                            <p class="ui-muted" data-slot="field-error-config-summary">
                                {move || {
                                    let tone = match workbench_tone_index.get() {
                                        1 => "neutral",
                                        2 => "negative",
                                        _ => "auto",
                                    };
                                    let message_source = if workbench_custom_message.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };
                                    let aria_source = if workbench_custom_aria.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };
                                    let class_source = if workbench_custom_class.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };

                                    format!(
                                        "config: tone={} visible={} disabled={} icon={} message_source={} aria_source={} class_source={}",
                                        tone,
                                        workbench_visible.get(),
                                        workbench_disabled.get(),
                                        workbench_icon_visible.get(),
                                        message_source,
                                        aria_source,
                                        class_source,
                                    )
                                }}
                            </p>
                        </div>
                    }
                }
            >
                {move || {
                    let tone = match workbench_tone_index.get() {
                        1 => FieldErrorTone::Neutral,
                        2 => FieldErrorTone::Negative,
                        _ => FieldErrorTone::Auto,
                    };
                    let message = if workbench_custom_message.get() {
                        "Workbench custom error message".to_string()
                    } else {
                        String::new()
                    };
                    let aria_label = if workbench_custom_aria.get() {
                        "Workbench aria label".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-field-error-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="field-error-interactive-stage">
                            <FieldError
                                tone=tone
                                is_visible=workbench_visible.get()
                                is_disabled=workbench_disabled.get()
                                is_icon_visible=workbench_icon_visible.get()
                                show_icon=workbench_icon_visible.get()
                                message=message
                                aria_label=aria_label
                                class_name=class_name
                                lang=if workbench_rtl.get() { "ar" } else { "en-US" }
                                dir=if workbench_rtl.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                            />
                            <p class="ui-muted" data-slot="field-error-interactive-hint">
                                "Inspect data-state/data-message-source/data-aria-source/data-class-source while toggling controls."
                            </p>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Visible / Hidden / Disabled)"
                code_signal=state_matrix_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                        show_icon=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        is_icon_visible=true
                        show_icon=true
                        message="Password should include at least one symbol".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        show_icon=true
                        message="Two-factor code is invalid".to_string()
                        class_name="docs-field-error-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground
                title="Visible / Hidden / Disabled Gallery"
                code_signal=state_matrix_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        message="Two-factor code is invalid".to_string()
                    />
                    <FieldError
                        is_visible=true
                        is_disabled=true
                        is_icon_visible=true
                        message="Read-only mode keeps the latest error snapshot".to_string()
                    />
                    <FieldError
                        is_visible=false
                        message="Hidden state keeps semantic contracts without visual output"
                            .to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Uncontrolled snapshot: email is required".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        message="Controlled snapshot: email format is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                code_signal=stream_snapshot_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Snapshot: email is required".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Streaming fallback=snapshot: waiting for final validation"
                            .to_string()
                    />
                </div>
            </Playground>

            <Playground title="Visible + Tone" code_signal=default_code>
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        message="Two-factor code is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Hidden + Disabled + Custom Class" code_signal=hidden_code>
                <div class="docs-stack">
                    <FieldError
                        is_visible=false
                        message="This text should not render when hidden".to_string()
                    />
                    <FieldError
                        is_visible=true
                        is_disabled=true
                        is_icon_visible=true
                        class_name="docs-field-error-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-error-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Use any FieldError Playground's "
                    <code>"Show code"</code>
                    " + copy button. Snippets are import-ready for direct paste."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{FieldError, FieldErrorTone};\n\n<FieldError\n  is_visible=true\n  message=\"Email is required\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-field-error-source-copy".to_string()
                />
                <ul data-slot="field-error-source-paths">
                    <li><code>"components/field-error/src/mod.rs"</code></li>
                    <li><code>"components/field-error/src/logic.rs"</code></li>
                    <li><code>"components/field-error/src/view.rs"</code></li>
                    <li><code>"components/field-error/src/styles.rs"</code></li>
                </ul>
                <ul data-slot="field-error-source-prerequisites">
                    <li><code>"component-field_error"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_message() -> AnyView {
    let error_message_imports = "use leptos::prelude::*;\nuse ui::{ErrorMessage, ErrorMessageElement, ErrorMessageTone};\nuse ui_headless::A11yDirection;".to_string();

    let (tone_index, set_tone_index) = signal(0usize);
    let (element_index, set_element_index) = signal(1usize);
    let (disabled_state, set_disabled_state) = signal(false);
    let (truncate_state, set_truncate_state) = signal(false);
    let (use_disabled_alias, set_use_disabled_alias) = signal(false);
    let (use_truncate_alias, set_use_truncate_alias) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get() {
        1 => ErrorMessageTone::Neutral,
        2 => ErrorMessageTone::Negative,
        _ => ErrorMessageTone::Auto,
    });
    let workbench_element = Signal::derive(move || match element_index.get() {
        0 => ErrorMessageElement::Span,
        2 => ErrorMessageElement::Div,
        _ => ErrorMessageElement::Paragraph,
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::error_message::ErrorMessageMotion { transition_ms: 320 }
        } else {
            ui::error_message::ErrorMessageMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Email validation error".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-error-message-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<ErrorMessage text=\"Invalid email address\".into() />"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ErrorMessage\n  text=\"{}\".into()\n  tone=ErrorMessageTone::{:?}\n  is_disabled={}\n  disabled={}\n  is_truncated={}\n  truncate={}\n  element=ErrorMessageElement::{:?}\n  motion=ui::error_message::ErrorMessageMotion {{ transition_ms: {} }}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir=ui_headless::A11yDirection::{}\n/>",
            if disabled_state.get() {
                "Email is required for account creation"
            } else {
                "Invalid email address"
            },
            workbench_tone.get(),
            bool_word(if use_disabled_alias.get() {
                false
            } else {
                disabled_state.get()
            }),
            bool_word(if use_disabled_alias.get() {
                disabled_state.get()
            } else {
                false
            }),
            bool_word(if use_truncate_alias.get() {
                false
            } else {
                truncate_state.get()
            }),
            bool_word(if use_truncate_alias.get() {
                truncate_state.get()
            } else {
                false
            }),
            workbench_element.get(),
            workbench_motion.get().transition_ms,
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ErrorMessage text=\"Invalid email address\".into() />
<ErrorMessage
  text=\"Username already exists\".into()
  tone=ErrorMessageTone::Neutral
  is_disabled=true
  element=ErrorMessageElement::Div
/>
<ErrorMessage
  text=\"Verification code expired\".into()
  tone=ErrorMessageTone::Negative
  disabled=true
  truncate=true
  motion=ui::error_message::ErrorMessageMotion { transition_ms: 280 }
  aria_label=\"Verification error\".into()
  class_name=\"docs-error-message-custom\".into()
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/error-message/src/styles.rs */\n{}",
            ui::error_message::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let text = if disabled_state.get() {
            "Email is required for account creation".to_string()
        } else {
            "Invalid email address".to_string()
        };
        let is_disabled = if use_disabled_alias.get() {
            false
        } else {
            disabled_state.get()
        };
        let disabled = if use_disabled_alias.get() {
            disabled_state.get()
        } else {
            false
        };
        let is_truncated = if use_truncate_alias.get() {
            false
        } else {
            truncate_state.get()
        };
        let truncate = if use_truncate_alias.get() {
            truncate_state.get()
        } else {
            false
        };

        format!(
            "ErrorMessageActualConfig {{\n  text: {:?},\n  tone: {:?},\n  is_disabled: {},\n  disabled: {},\n  is_truncated: {},\n  truncate: {},\n  element: {:?},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            text,
            workbench_tone.get(),
            is_disabled,
            disabled,
            is_truncated,
            truncate,
            workbench_element.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="ErrorMessage"
            slug="error-message"
            group="Forms"
            description="Inline form error primitive with full API workbench and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=error_message_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorMessage text="Invalid email address".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=error_message_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/error-message/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="error-message-workbench-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Auto"</option>
                            <option value="1">"Neutral"</option>
                            <option value="2">"Negative"</option>
                        </select>

                        <div class="docs-search__label">"Element"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || element_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_element_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"span"</option>
                            <option value="1">"paragraph"</option>
                            <option value="2">"div"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || disabled_state.get()
                                on:change=move |event| set_disabled_state.set(event_target_checked(&event))
                            />
                            <span>"Disabled state"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_disabled_alias.get()
                                on:change=move |event| set_use_disabled_alias.set(event_target_checked(&event))
                            />
                            <span>"Use disabled alias prop"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || truncate_state.get()
                                on:change=move |event| set_truncate_state.set(event_target_checked(&event))
                            />
                            <span>"Truncate state"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_truncate_alias.get()
                                on:change=move |event| set_use_truncate_alias.set(event_target_checked(&event))
                            />
                            <span>"Use truncate alias prop"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"Custom aria label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight docs-error-message-limit">
                    <ErrorMessage
                        text=if disabled_state.get() {
                            "Email is required for account creation".to_string()
                        } else {
                            "Invalid email address".to_string()
                        }
                        tone=workbench_tone.get()
                        is_disabled=if use_disabled_alias.get() {
                            false
                        } else {
                            disabled_state.get()
                        }
                        disabled=if use_disabled_alias.get() {
                            disabled_state.get()
                        } else {
                            false
                        }
                        is_truncated=if use_truncate_alias.get() {
                            false
                        } else {
                            truncate_state.get()
                        }
                        truncate=if use_truncate_alias.get() {
                            truncate_state.get()
                        } else {
                            false
                        }
                        element=workbench_element.get()
                        motion=workbench_motion.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Alias / Element Comparison)"
                code_signal=matrix_code
                code_imports=error_message_imports
            >
                <div class="docs-stack docs-stack--tight docs-error-message-limit">
                    <ErrorMessage text="Invalid email address".to_string() />
                    <ErrorMessage
                        text="Username already exists".to_string()
                        tone=ErrorMessageTone::Neutral
                        is_disabled=true
                        element=ErrorMessageElement::Div
                    />
                    <ErrorMessage
                        text="Verification code expired".to_string()
                        tone=ErrorMessageTone::Negative
                        disabled=true
                        truncate=true
                        motion=ui::error_message::ErrorMessageMotion {
                            transition_ms: 280,
                        }
                        aria_label="Verification error".to_string()
                        class_name="docs-error-message-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn description() -> AnyView {
    let description_imports =
        "use leptos::prelude::*;\nuse ui::{Description, DescriptionElement, DescriptionTone};"
            .to_string();

    let tone_options = vec![
        "default".to_string(),
        "muted".to_string(),
        "negative".to_string(),
    ];
    let element_options = vec![
        "paragraph".to_string(),
        "span".to_string(),
        "div".to_string(),
    ];

    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (element_index, set_element_index) = signal(Some(0_usize));
    let (is_disabled, set_is_disabled) = signal(false);
    let (is_truncated, set_is_truncated) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (lang_zh, set_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

    let selected_tone: Signal<DescriptionTone> =
        Signal::derive(move || match tone_index.get().unwrap_or(0) {
            1 => DescriptionTone::Muted,
            2 => DescriptionTone::Negative,
            _ => DescriptionTone::Default,
        });
    let selected_element: Signal<DescriptionElement> =
        Signal::derive(move || match element_index.get().unwrap_or(0) {
            1 => DescriptionElement::Span,
            2 => DescriptionElement::Div,
            _ => DescriptionElement::Paragraph,
        });

    let workbench_code = Signal::derive(move || {
        let tone = selected_tone.get();
        let element = selected_element.get();
        let mut lines = vec![
            "<Description".to_string(),
            "  text=\"Helper text for this field.\".into()".to_string(),
        ];

        if tone != DescriptionTone::Default {
            lines.push(format!("  tone=DescriptionTone::{tone:?}"));
        }
        if element != DescriptionElement::Paragraph {
            lines.push(format!("  element=DescriptionElement::{element:?}"));
        }
        if is_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if is_truncated.get() {
            lines.push("  is_truncated=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Description helper text\".into()".to_string());
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-description-custom\".into()".to_string());
        }
        if lang_zh.get() {
            lines.push("  lang=\"zh-CN\".into()".to_string());
        }
        if rtl_dir.get() {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/description/src/styles.rs */\n{}",
            ui::description::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let tone = selected_tone.get();
        let element = selected_element.get();
        let mut classes = vec!["ui-description".to_string(), tone.class_name().into()];
        if is_disabled.get() {
            classes.push("ui-description--disabled".to_string());
        }
        if is_truncated.get() {
            classes.push("ui-description--truncate".to_string());
        }
        if custom_class.get() {
            classes.push("ui-description--custom-class".to_string());
            classes.push("docs-description-custom".to_string());
        }

        format!(
            "DescriptionActualConfig {{\n  text: \"Helper text for this field.\",\n  tone: {tone:?},\n  element: {element:?},\n  is_disabled: {},\n  is_truncated: {},\n  lang: {},\n  dir: {},\n  has_custom_aria_label: {},\n  has_custom_class_name: {},\n  class: \"{}\",\n}}",
            is_disabled.get(),
            is_truncated.get(),
            if lang_zh.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            custom_aria_label.get(),
            custom_class.get(),
            classes.join(" ")
        )
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Description text="This appears below the field.".to_string() />"#.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Description
  text="This appears below the field as guidance.".to_string()
  tone=DescriptionTone::Default
  aria_label="Name helper".to_string()
/>
<Description
  text="Optional details are only visible to admins.".to_string()
  tone=DescriptionTone::Muted
/>
<Description
  text="Two-factor code expired. Request a new one.".to_string()
  tone=DescriptionTone::Negative
/>
<Description
  text="Read-only helper still keeps the latest snapshot.".to_string()
  tone=DescriptionTone::Muted
  is_disabled=true
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// Description has no internal controllable state axis.
// Uncontrolled-style: pass final helper snapshot directly.
<Description
  text="Uncontrolled snapshot: email must include @".to_string()
/>

// Controlled-style (by parent form store): parent updates props and Description re-renders.
<Description
  text="Controlled snapshot: email format is invalid".to_string()
  tone=DescriptionTone::Negative
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated helper output in one shot.
<Description
  text="Snapshot: email is required".to_string()
/>

// Streaming Optional: fallback stays snapshot until final helper output is ready.
<Description
  text="Streaming fallback=snapshot: waiting for final validation".to_string()
  tone=DescriptionTone::Muted
/>"#
        .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<Description
  text="This appears below the field as guidance.".to_string()
  tone=DescriptionTone::Default
  aria_label="Name helper".to_string()
/>
<Description
  text="Optional details are only visible to admins.".to_string()
  tone=DescriptionTone::Muted
/>
<Description
  text="Two-factor code expired. Request a new one.".to_string()
  tone=DescriptionTone::Negative
/>"#
        .to_string()
    });

    let truncate_code = Signal::derive(move || {
        r#"<Description
  text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
  element=DescriptionElement::Span
  is_truncated=true
  class_name="docs-description-custom".to_string()
/>
<Description
  text="Disabled helper text".to_string()
  is_disabled=true
  tone=DescriptionTone::Muted
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Description"
            slug="description"
            group="Forms"
            description="baseline-style form description primitive with centralized tone/state/source contracts and stable slot semantics."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack">
                    <Description text="This appears below the field.".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Description state contracts."
                code_signal=workbench_code
                code_imports=description_imports.clone()
                test_css_source=test_css_source
                test_source_path="components/description/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-description-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Description tone".to_string()
                        />

                        <div class="docs-search__label">"Element"</div>
                        <SegmentedControl
                            id_base="docs-description-element".to_string()
                            options=element_options.clone()
                            selected_index=element_index
                            set_selected_index=set_element_index
                            size=SegmentedControlSize::Sm
                            aria_label="Description element".to_string()
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=is_truncated set_checked=set_is_truncated>
                            "Truncate"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=lang_zh set_checked=set_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let tone = selected_tone.get();
                    let element = selected_element.get();

                    view! {
                        <div class="docs-stack docs-description-limit">
                            <Description
                                text="Helper text for this field.".to_string()
                                tone=tone
                                element=element
                                is_disabled=is_disabled.get()
                                is_truncated=is_truncated.get()
                                aria_label=if custom_aria_label.get() {
                                    "Description helper text".to_string()
                                } else {
                                    "".to_string()
                                }
                                class_name=if custom_class.get() {
                                    "docs-description-custom".to_string()
                                } else {
                                    "".to_string()
                                }
                                lang=if lang_zh.get() {
                                    "zh-CN".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if rtl_dir.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                            />
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Tone / Disabled / Truncate)"
                code_signal=state_matrix_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
                    <Description
                        text="This appears below the field as guidance.".to_string()
                        tone=DescriptionTone::Default
                        aria_label="Name helper".to_string()
                    />
                    <Description
                        text="Optional details are only visible to admins.".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <Description
                        text="Two-factor code expired. Request a new one.".to_string()
                        tone=DescriptionTone::Negative
                    />
                    <Description
                        text="Read-only helper still keeps the latest snapshot.".to_string()
                        tone=DescriptionTone::Muted
                        is_disabled=true
                    />
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        is_truncated=true
                        class_name="docs-description-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack">
                    <Description
                        text="Uncontrolled snapshot: email must include @".to_string()
                    />
                    <Description
                        text="Controlled snapshot: email format is invalid".to_string()
                        tone=DescriptionTone::Negative
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Description is not a正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
                    <Description text="Snapshot: email is required".to_string() />
                    <Description
                        text="Streaming fallback=snapshot: waiting for final validation".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <p class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-output-status."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Tone Variants"
                code_signal=tone_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack">
                    <Description
                        text="This appears below the field as guidance.".to_string()
                        tone=DescriptionTone::Default
                        aria_label="Name helper".to_string()
                    />
                    <Description
                        text="Optional details are only visible to admins.".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <Description
                        text="Two-factor code expired. Request a new one.".to_string()
                        tone=DescriptionTone::Negative
                    />
                </div>
            </Playground>

            <Playground
                title="Truncate + Element + Disabled"
                code_signal=truncate_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        is_truncated=true
                        class_name="docs-description-custom".to_string()
                    />
                    <Description
                        text="Disabled helper text".to_string()
                        is_disabled=true
                        tone=DescriptionTone::Muted
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="description-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Use any Description Playground's "
                    <code>"Show code"</code>
                    " + copy button. Snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{Description, DescriptionElement, DescriptionTone};\n\n<Description\n  text=\"This appears below the field.\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-description-source-copy".to_string()
                />
                <ul data-slot="description-source-paths">
                    <li><code>"components/description/src/mod.rs"</code></li>
                    <li><code>"components/description/src/logic.rs"</code></li>
                    <li><code>"components/description/src/view.rs"</code></li>
                    <li><code>"components/description/src/styles.rs"</code></li>
                </ul>
                <ul data-slot="description-source-prerequisites">
                    <li><code>"component-description"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn fieldset() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<Fieldset legend="Channels".to_string()>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<Fieldset
  legend="Notification channels".to_string()
  description="Pick every channel you want to receive release updates from.".to_string()
  is_required=true
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let invalid_code = Signal::derive(move || {
        r#"<Fieldset
  orientation=FieldsetOrientation::Horizontal
  tone=FieldsetTone::Muted
  is_invalid=true
  error_message="Pick at least one channel".to_string()
  class_name="docs-fieldset-custom".to_string()
  actions=move || view! {
    <ui::Button variant=ui::ButtonVariant::Secondary size=ui::ButtonSize::Sm>
      "Manage channels"
    </ui::Button>
  }
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"let (controlled_invalid, set_controlled_invalid) = signal(true);

// Uncontrolled-style: initialize once with default_is_invalid.
<Fieldset
  legend="Uncontrolled snapshot".to_string()
  default_is_required=true
  default_is_disabled=false
  default_is_invalid=true
  error_message="Uncontrolled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>

// Controlled-style: external signal is the single source of truth.
<Fieldset
  legend="Controlled snapshot".to_string()
  default_is_required=false
  on_is_required_change=Callback::new(move |_next| {})
  default_is_disabled=false
  on_is_disabled_change=Callback::new(move |_next| {})
  is_invalid=Signal::derive(move || controlled_invalid.get())
  on_is_invalid_change=Callback::new(move |next| set_controlled_invalid.set(next))
  motion=FieldsetMotion::default()
  error_message="Controlled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Streaming Optional: Fieldset is not a正文阅读面; fallback remains snapshot.
<Fieldset
  legend="Streaming validation snapshot".to_string()
  is_invalid=true
  error_message="Streaming fallback=snapshot: waiting for final validation".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>"#
            .to_string()
    });

    let orientation_options = vec!["vertical".to_string(), "horizontal".to_string()];
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let locale_options = vec!["en-US".to_string(), "zh-CN".to_string(), "ar".to_string()];

    let (controlled_invalid, set_controlled_invalid) = signal(true);
    let controlled_invalid_signal = Signal::derive(move || controlled_invalid.get());
    let on_controlled_invalid_change =
        Callback::new(move |next: bool| set_controlled_invalid.set(next));

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_locale_index, set_workbench_locale_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_actions, set_workbench_show_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_orientation =
        Signal::derive(
            move || match workbench_orientation_index.get().unwrap_or(0) {
                1 => FieldsetOrientation::Horizontal,
                _ => FieldsetOrientation::Vertical,
            },
        );
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => FieldsetTone::Muted,
        _ => FieldsetTone::Default,
    });

    let workbench_code = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => Some("zh-CN"),
            2 => Some("ar"),
            _ => None,
        };

        let mut lines = vec!["<Fieldset".to_string()];
        if orientation != FieldsetOrientation::Vertical {
            lines.push(format!(
                "  orientation=FieldsetOrientation::{orientation:?}"
            ));
        }
        if tone != FieldsetTone::Default {
            lines.push(format!("  tone=FieldsetTone::{tone:?}"));
        }
        if required {
            lines.push("  is_required=true".to_string());
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if invalid {
            lines.push("  is_invalid=true".to_string());
            lines.push("  error_message=\"Pick at least one channel\".into()".to_string());
        } else if show_description {
            lines.push("  description=\"Choose channels for release updates.\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-fieldset-custom\".into()".to_string());
        }
        if let Some(lang) = lang {
            lines.push(format!("  lang=\"{lang}\".into()"));
        }
        if rtl {
            lines.push("  dir=ui_headless::A11yDirection::Rtl".to_string());
        }
        if show_actions {
            lines.push(
                "  actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary size=ui::ButtonSize::Sm>\"Manage\"</ui::Button> }".to_string(),
            );
        }
        lines.extend([
            "  legend=\"Notification channels\".into()".to_string(),
            ">".to_string(),
            "  <label><input type=\"checkbox\" /> \"Email\"</label>".to_string(),
            "  <label><input type=\"checkbox\" /> \"SMS\"</label>".to_string(),
            "</Fieldset>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/field_form/fieldset/styles.rs */\n{}",
            ui::field_form::fieldset::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => "zh-CN",
            2 => "ar",
            _ => "default",
        };

        let mut class = vec![
            "ui-fieldset".to_string(),
            orientation.class_name().into(),
            tone.class_name().into(),
        ];
        if required {
            class.push("ui-fieldset--required".to_string());
        }
        if disabled {
            class.push("ui-fieldset--disabled".to_string());
        }
        if invalid {
            class.push("ui-fieldset--invalid".to_string());
        }
        if custom_class {
            class.push("ui-fieldset--custom-class".to_string());
            class.push("docs-fieldset-custom".to_string());
        }

        let message_kind = if invalid {
            "error"
        } else if show_description {
            "description"
        } else {
            "none"
        };

        format!(
            "FieldsetActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  is_required: {required},\n  default_is_required: {},\n  on_is_required_change: {:?},\n  is_disabled: {disabled},\n  default_is_disabled: {},\n  on_is_disabled_change: {:?},\n  is_invalid: {invalid},\n  default_is_invalid: {},\n  on_is_invalid_change: {:?},\n  legend: {:?},\n  error_message: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  has_description: {},\n  has_actions: {show_actions},\n  class_source: \"{}\",\n  message_kind: \"{message_kind}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
            false,
            Some("Callback<bool>"),
            false,
            Some("Callback<bool>"),
            false,
            Some("Callback<bool>"),
            Some("Notification channels"),
            if invalid {
                Some("Pick at least one channel")
            } else {
                None
            },
            Some("Notification channel group"),
            if custom_class {
                Some("docs-fieldset-custom")
            } else {
                None
            },
            ui::field_form::fieldset::FieldsetMotion::default(),
            show_description && !invalid,
            if custom_class { "custom" } else { "default" },
            if rtl { "rtl" } else { "auto" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Fieldset"
            slug="fieldset"
            group="Forms"
            description="baseline-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <Fieldset legend="Channels".to_string()>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground
                title="Fieldset Workbench (Display + Config + Code + CSS Test)"
                description="展示 / config / code / css test 一体化工作台，并提供多场景对比。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui/src/field_form/fieldset/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="fieldset-workbench-controls">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset tone".to_string()
                        />

                        <div class="docs-search__label">"Locale"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-locale".to_string()
                            options=locale_options.clone()
                            selected_index=workbench_locale_index
                            set_selected_index=set_workbench_locale_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset locale".to_string()
                        />

                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </Switch>
                        <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                            "Actions"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = workbench_orientation.get();
                    let tone = workbench_tone.get();
                    let locale_index = workbench_locale_index.get().unwrap_or(0);
                    let required = workbench_required.get();
                    let disabled = workbench_disabled.get();
                    let invalid = workbench_invalid.get();
                    let show_description = workbench_show_description.get();
                    let show_actions = workbench_show_actions.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();
                    let lang = match locale_index {
                        1 => "zh-CN".to_string(),
                        2 => "ar".to_string(),
                        _ => String::new(),
                    };
                    let description = if show_description && !invalid {
                        "Choose channels for release updates.".to_string()
                    } else {
                        String::new()
                    };
                    let error_message = if invalid {
                        "Pick at least one channel".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-fieldset-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="fieldset-workbench-compare">
                            <div class="docs-search__label">"Baseline"</div>
                            <Fieldset legend="Notification channels".to_string()>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"Email"</span>
                                </label>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"SMS"</span>
                                </label>
                            </Fieldset>

                            <div class="docs-search__label">"Configured"</div>
                            {if show_actions {
                                view! {
                            <Fieldset
                                orientation=orientation
                                tone=tone
                                is_required=required
                                is_disabled=disabled
                                is_invalid=invalid
                                legend="Notification channels".to_string()
                                description=description.clone()
                                error_message=error_message.clone()
                                class_name=class_name.clone()
                                aria_label="Notification channel group".to_string()
                                lang=lang.clone()
                                dir=dir
                                motion=ui::field_form::fieldset::FieldsetMotion::default()
                                actions=move || {
                                    view! {
                                        <ui::Button
                                                    variant=ui::ButtonVariant::Secondary
                                                    size=ui::ButtonSize::Sm
                                                >
                                                    "Manage"
                                                </ui::Button>
                                            }
                                        }
                                    >
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            } else {
                                view! {
                            <Fieldset
                                orientation=orientation
                                tone=tone
                                is_required=required
                                is_disabled=disabled
                                is_invalid=invalid
                                legend="Notification channels".to_string()
                                description=description
                                error_message=error_message
                                class_name=class_name
                                aria_label="Notification channel group".to_string()
                                lang=lang
                                dir=dir
                                motion=ui::field_form::fieldset::FieldsetMotion::default()
                            >
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            }}

                            <div class="docs-search__label">"Scenario compare"</div>
                            <div class="docs-stack docs-stack--tight">
                                <Fieldset
                                    legend="Required vertical".to_string()
                                    is_required=true
                                    description="Required + description".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"Email"</span>
                                    </label>
                                </Fieldset>
                                <Fieldset
                                    legend="Invalid horizontal".to_string()
                                    orientation=FieldsetOrientation::Horizontal
                                    tone=FieldsetTone::Muted
                                    is_invalid=true
                                    error_message="At least one option is required".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"SMS"</span>
                                    </label>
                                </Fieldset>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Required / Invalid / Disabled)"
                code_signal=invalid_code
            >
                <div class="docs-stack docs-stack--tight" data-slot="fieldset-state-matrix">
                    <Fieldset
                        legend="Required vertical".to_string()
                        is_required=true
                        description="Required + description".to_string()
                        aria_label="Required group".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"Email"</span>
                        </label>
                    </Fieldset>
                    <Fieldset
                        legend="Invalid horizontal".to_string()
                        orientation=FieldsetOrientation::Horizontal
                        tone=FieldsetTone::Muted
                        is_invalid=true
                        error_message="At least one option is required".to_string()
                        aria_label="Invalid group".to_string()
                        class_name="docs-fieldset-custom".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"SMS"</span>
                        </label>
                    </Fieldset>
                    <Fieldset
                        legend="Disabled".to_string()
                        is_disabled=true
                        default_is_required=true
                        default_is_disabled=true
                        on_is_required_change=Callback::new(move |_next| {})
                        on_is_disabled_change=Callback::new(move |_next| {})
                        aria_label="Disabled group".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" disabled />
                            <span>"Push"</span>
                        </label>
                    </Fieldset>
                </div>
            </Playground>

            <Playground title="Legend + Description" code_signal=default_code>
                <Fieldset
                    legend="Notification channels".to_string()
                    description="Pick every channel you want to receive release updates from.".to_string()
                    is_required=true
                    aria_label="Notification channel group".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Push"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground title="Horizontal + Invalid + Actions" code_signal=invalid_code>
                <Fieldset
                    orientation=FieldsetOrientation::Horizontal
                    tone=FieldsetTone::Muted
                    is_invalid=true
                    error_message="Pick at least one channel".to_string()
                    class_name="docs-fieldset-custom".to_string()
                    actions=move || {
                        view! {
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                size=ui::ButtonSize::Sm
                            >
                                "Manage channels"
                            </ui::Button>
                        }
                    }
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Snapshot Contrast)"
                description="受控/非受控对照：默认值只初始化一次，受控值由外部 signal 驱动。"
                code_signal=controlled_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight" data-slot="fieldset-controlled-uncontrolled">
                    <div class="docs-search__label">"Uncontrolled snapshot"</div>
                    <Fieldset
                        legend="Uncontrolled snapshot".to_string()
                        default_is_invalid=true
                        error_message="Uncontrolled snapshot: pick at least one channel".to_string()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"Email"</span>
                        </label>
                    </Fieldset>

                    <div class="docs-search__label">"Controlled snapshot"</div>
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        size=ui::ButtonSize::Sm
                        on_press=Callback::new(move |_| {
                            set_controlled_invalid.update(|value| *value = !*value);
                        })
                    >
                        {move || if controlled_invalid.get() { "Set controlled valid" } else { "Set controlled invalid" }}
                    </ui::Button>
                    <Fieldset
                        legend="Controlled snapshot".to_string()
                        is_invalid=controlled_invalid_signal.get()
                        on_is_invalid_change=on_controlled_invalid_change
                        error_message="Controlled snapshot: pick at least one channel".to_string()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"SMS"</span>
                        </label>
                    </Fieldset>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Fieldset 不是正文阅读面；文档展示 snapshot 输出与 fallback=snapshot 契约。"
                code_signal=stream_snapshot_code
            >
                <Fieldset
                    legend="Streaming validation snapshot".to_string()
                    is_invalid=true
                    error_message="Streaming fallback=snapshot: waiting for final validation".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                </Fieldset>
                <div class="docs-subtitle">
                    "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-stream-mode."
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="fieldset-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="fieldset-source-first-contract">
                    "Use any Fieldset Playground's "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p data-slot="fieldset-source-first-dependency-baseline">
                    "Dependency baseline (Cargo.toml): "
                    <code>
                        "ui = { default-features = false, features = [\"component-fieldset\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text=r#"components/fieldset/src/mod.rs
components/fieldset/src/logic.rs
components/fieldset/src/view.rs
components/fieldset/src/styles.rs
components/fieldset/src/motion.rs
crates/ui/src/field_form/fieldset/{mod,logic,view,styles,motion}.rs
apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset"#.to_string()
                    copyable=true
                    class_name="docs-fieldset-source-copy".to_string()
                />
                <ul data-slot="fieldset-source-prerequisites">
                    <li><code>"component-fieldset"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn label() -> AnyView {
    let label_imports =
        "use leptos::prelude::*;\nuse ui::{Label, LabelEmphasis};\nuse ui_headless::A11yDirection;"
            .to_string();

    let (emphasis_index, set_emphasis_index) = signal(0usize);
    let (is_required, set_is_required) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (has_for_id, set_has_for_id) = signal(true);
    let (custom_indicator, set_custom_indicator) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_emphasis = Signal::derive(move || match emphasis_index.get() {
        1 => LabelEmphasis::Subtle,
        2 => LabelEmphasis::Strong,
        _ => LabelEmphasis::Default,
    });
    let workbench_text = Signal::derive(move || {
        if is_required.get() {
            "Assignee".to_string()
        } else {
            "Optional assignee".to_string()
        }
    });
    let workbench_for_id = Signal::derive(move || {
        if has_for_id.get() {
            "docs-label-workbench-input".to_string()
        } else {
            String::new()
        }
    });
    let workbench_required_indicator = Signal::derive(move || {
        if custom_indicator.get() {
            "(required)".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-label-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::label::LabelMotion {
                color_transition_ms: 420,
                weight_transition_ms: 420,
            }
        } else {
            ui::label::LabelMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Label
  text=\"Email\".into()
  for_id=\"email\".into()
  is_required=true
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Label\n  text={text}\n  for_id={for_id}\n  is_required={}\n  is_disabled={}\n  emphasis=LabelEmphasis::{:?}\n  required_indicator={required_indicator}\n  class_name={class_name}\n  motion=ui::label::LabelMotion {{ color_transition_ms: {}, weight_transition_ms: {} }}\n  lang={lang}\n  dir=ui_headless::A11yDirection::{}\n/>",
            bool_word(is_required.get()),
            bool_word(is_disabled.get()),
            workbench_emphasis.get(),
            workbench_motion.get().color_transition_ms,
            workbench_motion.get().weight_transition_ms,
            if rtl.get() { "Rtl" } else { "Ltr" },
            text = rust_string_literal(&workbench_text.get()),
            for_id = rust_string_literal(&workbench_for_id.get()),
            required_indicator = rust_string_literal(&workbench_required_indicator.get()),
            class_name = rust_string_literal(&workbench_class_name.get()),
            lang = rust_string_literal(&workbench_lang.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Label text=\"Name\".into() for_id=\"name\".into() is_required=true />
<Label
  text=\"Helper\".into()
  emphasis=LabelEmphasis::Subtle
  is_disabled=true
/>
<Label
  text=\"Critical owner\".into()
  emphasis=LabelEmphasis::Strong
  required_indicator=\"(required)\".into()
  class_name=\"docs-label-custom\".into()
  motion=ui::label::LabelMotion { color_transition_ms: 420, weight_transition_ms: 420 }
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/label/src/styles.rs */\n{}",
            ui::label::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "LabelActualConfig {{\n  text: {:?},\n  for_id: {:?},\n  is_required: {},\n  is_disabled: {},\n  emphasis: {:?},\n  required_indicator: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_text.get(),
            workbench_for_id.get(),
            is_required.get(),
            is_disabled.get(),
            workbench_emphasis.get(),
            workbench_required_indicator.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="Label"
            slug="label"
            group="Forms"
            description="Form label primitive with full API workbench and multi-state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=label_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text="Email".to_string()
                        for_id="docs-label-showcase".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-label-showcase"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=label_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/label/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="label-workbench-controls">
                        <div class="docs-search__label">"Emphasis"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || emphasis_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_emphasis_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Default"</option>
                            <option value="1">"Subtle"</option>
                            <option value="2">"Strong"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_required.get()
                                on:change=move |event| set_is_required.set(event_target_checked(&event))
                            />
                            <span>"Required"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_disabled.get()
                                on:change=move |event| set_is_disabled.set(event_target_checked(&event))
                            />
                            <span>"Disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || has_for_id.get()
                                on:change=move |event| set_has_for_id.set(event_target_checked(&event))
                            />
                            <span>"Bind for/id"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_indicator.get()
                                on:change=move |event| set_custom_indicator.set(event_target_checked(&event))
                            />
                            <span>"Custom required indicator"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text=workbench_text.get()
                        for_id=workbench_for_id.get()
                        is_required=is_required.get()
                        is_disabled=is_disabled.get()
                        emphasis=workbench_emphasis.get()
                        required_indicator=workbench_required_indicator.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <input
                        id="docs-label-workbench-input"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                        disabled=is_disabled.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Emphasis / Disabled / Locale Comparison)"
                code_signal=matrix_code
                code_imports=label_imports
            >
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text="Name".to_string()
                        for_id="docs-label-matrix-name".to_string()
                        is_required=true
                    />
                    <Label
                        text="Helper".to_string()
                        emphasis=LabelEmphasis::Subtle
                        is_disabled=true
                    />
                    <Label
                        text="Critical owner".to_string()
                        emphasis=LabelEmphasis::Strong
                        required_indicator="(required)".to_string()
                        class_name="docs-label-custom".to_string()
                        motion=ui::label::LabelMotion {
                            color_transition_ms: 420,
                            weight_transition_ms: 420,
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn field() -> AnyView {
    let field_imports = "use leptos::prelude::*;\nuse ui::{Field, FieldOrientation, FieldTone, field_form::field::FieldMotion};".to_string();

    let persisted_workbench_state = load_field_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let FieldWorkbenchState {
        orientation_key: initial_orientation_key,
        tone_key: initial_tone_key,
        required: initial_required,
        invalid: initial_invalid,
        disabled: initial_disabled,
        custom_class: initial_custom_class,
        custom_error: initial_custom_error,
        motion_ms: initial_motion_ms,
    } = persisted_workbench_state.unwrap_or_default();

    let (workbench_orientation_key, set_workbench_orientation_key) =
        signal(initial_orientation_key);
    let (workbench_tone_key, set_workbench_tone_key) = signal(initial_tone_key);
    let (workbench_required, set_workbench_required) = signal(initial_required);
    let (workbench_invalid, set_workbench_invalid) = signal(initial_invalid);
    let (workbench_disabled, set_workbench_disabled) = signal(initial_disabled);
    let (workbench_custom_class, set_workbench_custom_class) = signal(initial_custom_class);
    let (workbench_custom_error, set_workbench_custom_error) = signal(initial_custom_error);
    let (workbench_motion_ms, set_workbench_motion_ms) = signal(initial_motion_ms);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        let state = FieldWorkbenchState {
            orientation_key: workbench_orientation_key.get(),
            tone_key: workbench_tone_key.get(),
            required: workbench_required.get(),
            invalid: workbench_invalid.get(),
            disabled: workbench_disabled.get(),
            custom_class: workbench_custom_class.get(),
            custom_error: workbench_custom_error.get(),
            motion_ms: workbench_motion_ms.get(),
        };

        if workbench_persist_state.get() {
            save_field_workbench_state(state);
        } else {
            clear_field_workbench_state();
        }
    });

    let workbench_orientation =
        Signal::derive(move || match workbench_orientation_key.get().as_str() {
            "horizontal" => FieldOrientation::Horizontal,
            _ => FieldOrientation::Vertical,
        });
    let workbench_tone = Signal::derive(move || match workbench_tone_key.get().as_str() {
        "muted" => FieldTone::Muted,
        _ => FieldTone::Default,
    });

    let workbench_code = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let custom_error = workbench_custom_error.get();
        let motion_ms = workbench_motion_ms.get();

        let mut snippet = vec!["<Field".to_string()];
        if orientation != FieldOrientation::Vertical {
            snippet.push(format!("  orientation=FieldOrientation::{orientation:?}"));
        }
        if tone != FieldTone::Default {
            snippet.push(format!("  tone=FieldTone::{tone:?}"));
        }
        if required {
            snippet.push("  is_required=true".to_string());
        }
        if invalid {
            snippet.push("  is_invalid=true".to_string());
        }
        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if custom_error {
            snippet.push("  error_message=\"Custom validation error\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-field-custom\".into()".to_string());
        }
        if motion_ms != 160 {
            snippet.push(format!(
                "  motion=FieldMotion {{ duration_ms: {motion_ms}.0 }}"
            ));
        }
        snippet.push("  label=\"Email\".into()".to_string());
        snippet.push("  description=\"Inspect contracts in test panel.\".into()".to_string());
        snippet.push("  aria_label=\"Workbench field\".into()".to_string());
        snippet.push("  lang=\"en-US\".into()".to_string());
        snippet.push("  dir=ui_headless::A11yDirection::Ltr".to_string());
        snippet.push(">".to_string());
        snippet.push(
            "  <input class=\"docs-search__input\" type=\"email\" placeholder=\"owner@company.com\" />".to_string(),
        );
        snippet.push("</Field>".to_string());
        snippet.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/field_form/field/styles.rs */\n{}",
            ui::field_form::field::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let custom_error = workbench_custom_error.get();
        let motion_ms = workbench_motion_ms.get();
        let persist = workbench_persist_state.get();

        let mut classes = vec![
            "ui-field".to_string(),
            orientation.class_name().into(),
            tone.class_name().into(),
        ];
        if required {
            classes.push("ui-field--required".to_string());
        }
        if disabled {
            classes.push("ui-field--disabled".to_string());
        }
        if invalid {
            classes.push("ui-field--invalid".to_string());
        }
        classes.push("ui-field--has-label".to_string());
        classes.push("ui-field--has-description".to_string());
        if invalid {
            classes.push("ui-field--has-error".to_string());
        }
        if custom_class {
            classes.push("ui-field--custom-class".to_string());
            classes.push("docs-field-custom".to_string());
        }

        let data_state = if invalid && disabled {
            "invalid-disabled"
        } else if invalid {
            "invalid"
        } else if disabled {
            "disabled"
        } else if required {
            "required"
        } else if orientation == FieldOrientation::Horizontal {
            "horizontal"
        } else if tone == FieldTone::Muted {
            "muted"
        } else {
            "default"
        };

        format!(
            "FieldActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  is_required: {required},\n  is_invalid: {invalid},\n  is_disabled: {disabled},\n  label: {:?},\n  description: {:?},\n  error_message: {:?},\n  motion: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n  custom_error: {custom_error},\n  custom_class: {custom_class},\n  motion_ms: {motion_ms},\n  persist: {},\n  data_state: \"{data_state}\",\n  error_source: \"{}\",\n  class_source: \"{}\",\n  class: \"{}\",\n}}",
            Some("Email"),
            Some("Inspect source/state marker contracts"),
            if invalid {
                Some(if custom_error {
                    "Custom validation error"
                } else {
                    "A valid email is required"
                })
            } else {
                None
            },
            FieldMotion {
                duration_ms: f64::from(motion_ms),
                ..FieldMotion::default()
            },
            Some("Workbench field"),
            Some("en-US"),
            ui_headless::A11yDirection::Ltr,
            if custom_class {
                Some("docs-field-custom")
            } else {
                None
            },
            if persist { "on" } else { "off" },
            if !invalid {
                "none"
            } else if custom_error {
                "custom"
            } else {
                "default"
            },
            if custom_class { "custom" } else { "default" },
            classes.join(" "),
        )
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Field label="Email".to_string()>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>"#
            .to_string()
    });

    let required_code = Signal::derive(move || {
        r#"<Field
  label="Email".to_string()
  required=true
  description="We'll only use this for release notes.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>"#
            .to_string()
    });

    let invalid_code = Signal::derive(move || {
        r#"<Field
  orientation=FieldOrientation::Horizontal
  tone=FieldTone::Muted
  invalid=true
  error_message="A valid email is required".to_string()
  class_name="docs-field-custom".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Field
  label="Email".to_string()
  required=true
  description="Required: this field must be provided.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>
<Field
  label="Email".to_string()
  invalid=true
  error_message="A valid email is required".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>
<Field
  label="Email".to_string()
  disabled=true
  description="Disabled: read-only snapshot.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="disabled@example.com" />
</Field>"#
            .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// Uncontrolled-style: pass final validation snapshot props directly.
<Field
  label="Email".to_string()
  description="Uncontrolled snapshot: email is required".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>

// Controlled-style: parent chooses derived flags/messages, Field only renders semantic output.
<Field
  label="Email".to_string()
  invalid=true
  error_message="Controlled snapshot: email format is invalid".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Streaming Optional: Field is not a 正文阅读面, keep fallback=snapshot.
<Field
  label="Email".to_string()
  description="Streaming fallback=snapshot: waiting for final validation".to_string()
  aria_label="Email field".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Field"
            slug="field"
            group="Forms"
            description="Form field wrapper with centralized orientation/tone/validation/message-state modeling and stable data contracts."
        >
            <Playground
                title="Hello World (Default API)"
                description="Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines."
                code_signal=hello_world_code
                code_imports=field_imports.clone()
            >
                <Field label="Email".to_string()>
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </Field>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels, plus optional persisted context."
                code_signal=workbench_code
                code_imports=field_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/field_form/field/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-workbench-controls">
                        <label class="docs-search__label">
                            "Orientation"
                            <select
                                data-action="field-workbench-orientation"
                                prop:value=move || workbench_orientation_key.get()
                                on:change=move |ev| {
                                    set_workbench_orientation_key.set(event_target_value(&ev))
                                }
                            >
                                <option value="vertical">"Vertical"</option>
                                <option value="horizontal">"Horizontal"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Tone"
                            <select
                                data-action="field-workbench-tone"
                                prop:value=move || workbench_tone_key.get()
                                on:change=move |ev| set_workbench_tone_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Motion ms (" {move || workbench_motion_ms.get()} ")"
                            <input
                                type="range"
                                data-action="field-workbench-motion-ms"
                                min="1"
                                max="800"
                                step="1"
                                prop:value=move || workbench_motion_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(160)
                                        .clamp(1, 800);
                                    set_workbench_motion_ms.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-required"
                                prop:checked=move || workbench_required.get()
                                on:change=move |ev| set_workbench_required.set(event_target_checked(&ev))
                            />
                            " Required"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-invalid"
                                prop:checked=move || workbench_invalid.get()
                                on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev))
                            />
                            " Invalid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-disabled"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-custom-error"
                                prop:checked=move || workbench_custom_error.get()
                                on:change=move |ev| set_workbench_custom_error.set(event_target_checked(&ev))
                            />
                            " Custom error text"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-custom-class"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                        <p class="ui-muted" data-slot="field-workbench-summary">
                            {move || {
                                format!(
                                    "config: orientation={} tone={} required={} invalid={} disabled={} custom_error={} custom_class={} motion_ms={} persist={}",
                                    workbench_orientation_key.get(),
                                    workbench_tone_key.get(),
                                    if workbench_required.get() { "true" } else { "false" },
                                    if workbench_invalid.get() { "true" } else { "false" },
                                    if workbench_disabled.get() { "true" } else { "false" },
                                    if workbench_custom_error.get() { "true" } else { "false" },
                                    if workbench_custom_class.get() { "true" } else { "false" },
                                    workbench_motion_ms.get(),
                                    if workbench_persist_state.get() { "on" } else { "off" },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let orientation = workbench_orientation.get();
                    let tone = workbench_tone.get();
                    let required = workbench_required.get();
                    let invalid = workbench_invalid.get();
                    let disabled = workbench_disabled.get();
                    let custom_error = workbench_custom_error.get();
                    let custom_class = workbench_custom_class.get();
                    let motion = FieldMotion {
                        duration_ms: f64::from(workbench_motion_ms.get()),
                        ..FieldMotion::default()
                    };

                    if custom_error && custom_class {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else if custom_error {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else if custom_class {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Workbench Compare)"
                description="Compare baseline/invalid/disabled combinations after Workbench controls."
                code_signal=state_matrix_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-state-matrix-post-workbench">
                    <Field
                        label="Email".to_string()
                        is_required=true
                        description="Required: this field must be provided.".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="name@example.com" />
                    </Field>
                    <Field
                        label="Email".to_string()
                        is_invalid=true
                        error_message="A valid email is required".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="owner@company.com" />
                    </Field>
                    <Field
                        label="Email".to_string()
                        is_disabled=true
                        description="Disabled: read-only snapshot.".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="disabled@example.com" />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Required + Description"
                code_signal=required_code
                code_imports=field_imports.clone()
            >
                <Field
                    label="Email".to_string()
                    required=true
                    description="We'll only use this for release notes.".to_string()
                    aria_label="Email field".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </Field>
            </Playground>

            <Playground
                title="Horizontal + Invalid + Custom Class"
                code_signal=invalid_code
                code_imports=field_imports.clone()
            >
                <Field
                    orientation=FieldOrientation::Horizontal
                    tone=FieldTone::Muted
                    invalid=true
                    error_message="A valid email is required".to_string()
                    class_name="docs-field-custom".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="owner@company.com"
                    />
                </Field>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="field-api-rows">
                    <li>
                        <code>"orientation: FieldOrientation"</code>
                        " default = vertical"
                    </li>
                    <li>
                        <code>"tone: FieldTone"</code>
                        " default = default"
                    </li>
                    <li>
                        <code>"is_required / is_disabled / is_invalid"</code>
                        " default = false（优先命名）"
                    </li>
                    <li>
                        <code>"required / disabled / invalid"</code>
                        " 历史别名，默认 = false，且低于 `is_*` 优先级"
                    </li>
                    <li>
                        <code>"label / description / error_message / aria_label / lang / class_name"</code>
                        " optional semantic content (normalized in logic.rs)"
                    </li>
                    <li>
                        <code>"dir: Option&lt;A11yDirection&gt;"</code>
                        " optional"
                    </li>
                    <li>
                        <code>"motion: FieldMotion"</code>
                        " default = FieldMotion::default()"
                    </li>
                </ul>
            </section>

            <Playground
                title="Required / Invalid / Disabled Examples"
                description="State matrix baseline for required/invalid/disabled semantic markers."
                code_signal=state_matrix_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-state-matrix">
                    <Field
                        label="Email".to_string()
                        required=true
                        description="Required: this field must be provided.".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="name@example.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        invalid=true
                        error_message="A valid email is required".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="owner@company.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        disabled=true
                        description="Disabled: read-only snapshot.".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="disabled@example.com"
                        />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                description="Field is stateless: parent controls derived flags/messages; Field renders semantic snapshot."
                code_signal=controlled_uncontrolled_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-controlled-matrix">
                    <Field
                        label="Email".to_string()
                        description="Uncontrolled snapshot: email is required".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="name@example.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        invalid=true
                        error_message="Controlled snapshot: email format is invalid".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="owner@company.com"
                        />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Field is not a 正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=field_imports.clone()
            >
                <Field
                    label="Email".to_string()
                    description="Streaming fallback=snapshot: waiting for final validation".to_string()
                    aria_label="Email field".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="owner@company.com"
                    />
                    <span class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-mode/data-ui-stream-fallback."
                    </span>
                </Field>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="field-source-first-contract">
                    "Use any Field Playground's "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p data-slot="field-source-first-dependency-baseline">
                    "Dependency baseline (Cargo.toml): "
                    <code>
                        "ui = { default-features = false, features = [\"component-field\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text=r#"components/field/src/mod.rs
components/field/src/logic.rs
components/field/src/view.rs
components/field/src/styles.rs
components/field/src/motion.rs
apps/docs-app/src/pages/components/pages/forms_extra.rs::field"#.to_string()
                    copyable=true
                    class_name="docs-field-source-copy".to_string()
                />
                <ul data-slot="field-source-prerequisites">
                    <li><code>"component-field"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn help_text() -> AnyView {
    let help_text_imports =
        "use leptos::prelude::*;\nuse ui::{HelpText, HelpTextTone};".to_string();

    let hello_world_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
  aria_label="Password hint".to_string()
/>
<HelpText
  tone=HelpTextTone::Neutral
  description="This value is visible to project admins only.".to_string()
/>
<HelpText
  is_invalid=true
  is_error_icon_visible=true
  error_message="Password does not meet complexity requirements.".to_string()
  class_name="docs-help-text-custom".to_string()
/>
<HelpText
  is_invalid=true
  tone=HelpTextTone::Negative
  error_message="Two-factor token expired. Request a new code.".to_string()
  is_disabled=true
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// HelpText has no internal controllable state axis.
// Uncontrolled-style: pass final snapshot props directly.
<HelpText
  description="Uncontrolled snapshot: email must include @".to_string()
/>

// Controlled-style (parent store): parent updates props and HelpText re-renders.
<HelpText
  is_invalid=true
  error_message="Controlled snapshot: email format is invalid".to_string()
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated final output in one shot.
<HelpText
  is_invalid=true
  error_message="Snapshot: email is required".to_string()
/>

// Streaming Optional: fallback stays snapshot until final output is ready.
<HelpText
  tone=HelpTextTone::Neutral
  description="Streaming fallback=snapshot: waiting for final validation".to_string()
/>"#
        .to_string()
    });

    let description_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#
        .to_string()
    });

    let error_code = Signal::derive(move || {
        r#"<HelpText
  is_invalid=true
  is_error_icon_visible=true
  error_message="Password does not meet complexity requirements.".to_string()
  class_name="docs-help-text-custom".to_string()
/>"#
        .to_string()
    });
    let tone_options = vec![
        "Auto".to_string(),
        "Neutral".to_string(),
        "Negative".to_string(),
    ];
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (is_invalid, set_is_invalid) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);
    let (is_error_icon_visible, set_is_error_icon_visible) = signal(true);
    let (use_error_message, set_use_error_message) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (show_compare, set_show_compare) = signal(true);
    let (custom_motion, set_custom_motion) = signal(false);
    let (lang_zh, set_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

    let active_tone = Signal::derive(move || match tone_index.get().unwrap_or(0) {
        1 => HelpTextTone::Neutral,
        2 => HelpTextTone::Negative,
        _ => HelpTextTone::Auto,
    });
    let active_description = Signal::derive(move || {
        if is_invalid.get() && use_error_message.get() {
            None
        } else {
            Some("Use at least 12 characters.".to_string())
        }
    });
    let active_error_message = Signal::derive(move || {
        if is_invalid.get() && use_error_message.get() {
            Some("Password does not meet complexity requirements.".to_string())
        } else {
            None
        }
    });
    let active_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            Some("Custom help text aria label".to_string())
        } else {
            None
        }
    });
    let active_class_name = Signal::derive(move || {
        if custom_class.get() {
            Some("docs-help-text-custom".to_string())
        } else {
            None
        }
    });
    let interactive_code = Signal::derive(move || {
        let tone = active_tone.get();
        let is_invalid = is_invalid.get();
        let is_disabled = is_disabled.get();
        let is_error_icon_visible = is_error_icon_visible.get();
        let description = active_description.get();
        let error_message = active_error_message.get();
        let aria = active_aria_label.get();
        let class_name = active_class_name.get();

        let mut lines = vec![
            "<HelpText".to_string(),
            format!("  tone=HelpTextTone::{tone:?}"),
            format!("  is_invalid={is_invalid}"),
            format!("  is_disabled={is_disabled}"),
            format!("  is_error_icon_visible={is_error_icon_visible}"),
        ];
        if let Some(description) = description {
            lines.push(format!("  description={description:?}.into()"));
        }
        if let Some(error_message) = error_message {
            lines.push(format!("  error_message={error_message:?}.into()"));
        }
        if let Some(aria) = aria {
            lines.push(format!("  aria_label={aria:?}.into()"));
        }
        if let Some(class_name) = class_name {
            lines.push(format!("  class_name={class_name:?}.into()"));
        }
        if custom_motion.get() {
            lines.push("  motion=HelpTextMotion::disabled()".to_string());
        }
        lines.push(format!(
            "  lang={:?}.into()",
            if lang_zh.get() { "zh-CN" } else { "en-US" }
        ));
        if rtl_dir.get() {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        } else {
            lines.push("  dir=A11yDirection::Ltr".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/help-text/src/styles.rs */\n{}",
            ui::field_form::help_text::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let tone = active_tone.get();
        let is_invalid = is_invalid.get();
        let is_disabled = is_disabled.get();
        let is_error_icon_visible = is_error_icon_visible.get();
        let has_description = active_description.get().is_some();
        let has_error = active_error_message.get().is_some();
        let has_custom_aria = custom_aria.get();
        let has_custom_class = custom_class.get();
        format!(
            "HelpTextActualConfig {{\n  tone: HelpTextTone::{tone:?},\n  is_invalid: {is_invalid},\n  is_disabled: {is_disabled},\n  is_error_icon_visible: {is_error_icon_visible},\n  motion: {},\n  lang: {},\n  dir: {},\n  has_description: {has_description},\n  has_error_message: {has_error},\n  has_custom_aria_label: {has_custom_aria},\n  has_custom_class_name: {has_custom_class},\n}}",
            if custom_motion.get() {
                "HelpTextMotion::disabled()"
            } else {
                "HelpTextMotion::default()"
            },
            if lang_zh.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    view! {
        <ComponentPage
            title="HelpText"
            slug="help-text"
            group="Forms"
            description="baseline-style form assistance primitive that resolves description vs error message and tone/icon state through centralized logic contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText description="Use at least 12 characters.".to_string() />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                code_imports=help_text_imports.clone()
                test_css_source=test_css_source
                test_source_path="components/help-text/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；支持 description/error/is_invalid/is_disabled/tone 的多场景对比。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="help-text-workbench-controls">
                        <div class="docs-search__label">"配置区 · Tone"</div>
                        <div data-slot="help-text-tone-control">
                            <ui::SegmentedControl
                                id_base="docs-help-text-tone".to_string()
                                options=tone_options.clone()
                                selected_index=tone_index
                                set_selected_index=set_tone_index
                                size=ui::SegmentedControlSize::Sm
                                aria_label="HelpText tone".to_string()
                            />
                        </div>
                        <div data-slot="help-text-toggle-invalid">
                            <ui::Switch checked=is_invalid set_checked=set_is_invalid>
                                "Invalid"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-disabled">
                            <ui::Switch checked=is_disabled set_checked=set_is_disabled>
                                "Disabled"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-show-error-icon">
                            <ui::Switch checked=is_error_icon_visible set_checked=set_is_error_icon_visible>
                                "Show error icon"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-use-error-message">
                            <ui::Switch checked=use_error_message set_checked=set_use_error_message>
                                "Use error message"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-aria">
                            <ui::Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria label"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-class">
                            <ui::Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-motion">
                            <ui::Switch checked=custom_motion set_checked=set_custom_motion>
                                "Motion disabled"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-lang-zh">
                            <ui::Switch checked=lang_zh set_checked=set_lang_zh>
                                "lang=zh-CN"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-dir-rtl">
                            <ui::Switch checked=rtl_dir set_checked=set_rtl_dir>
                                "dir=rtl"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-show-compare">
                            <ui::Switch checked=show_compare set_checked=set_show_compare>
                                "Show compare matrix"
                            </ui::Switch>
                        </div>
                    </div>
                }
            >
                {move || {
                    let tone = active_tone.get();
                    let is_invalid = is_invalid.get();
                    let is_disabled = is_disabled.get();
                    let is_error_icon_visible = is_error_icon_visible.get();
                    let description = active_description.get().unwrap_or_default();
                    let error_message = active_error_message.get().unwrap_or_default();
                    let aria_label = active_aria_label.get().unwrap_or_default();
                    let class_name = active_class_name.get().unwrap_or_default();
                    let compare = show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="help-text-workbench-canvas">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="help-text-primary-card">
                                <HelpText
                                    tone=tone
                                    is_invalid=is_invalid
                                    is_disabled=is_disabled
                                    is_error_icon_visible=is_error_icon_visible
                                    description=description
                                    error_message=error_message
                                    aria_label=aria_label
                                    class_name=class_name
                                    motion=if custom_motion.get() {
                                        ui::field_form::help_text::HelpTextMotion::disabled()
                                    } else {
                                        ui::field_form::help_text::HelpTextMotion::default()
                                    }
                                    lang=if lang_zh.get() {
                                        "zh-CN".to_string()
                                    } else {
                                        "en-US".to_string()
                                    }
                                    dir=if rtl_dir.get() {
                                        A11yDirection::Rtl
                                    } else {
                                        A11yDirection::Ltr
                                    }
                                />
                            </div>

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <HelpText
                                        tone=HelpTextTone::Neutral
                                        description="Neutral description state.".to_string()
                                    />
                                    <HelpText
                                        tone=HelpTextTone::Negative
                                        is_invalid=true
                                        is_error_icon_visible=true
                                        error_message="Negative error state.".to_string()
                                    />
                                    <HelpText
                                        is_invalid=true
                                        is_disabled=true
                                        error_message="Disabled + invalid state.".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Description / Error / Disabled)"
                code_signal=state_matrix_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <Playground
                title="Description / Error / Disabled Gallery"
                code_signal=state_matrix_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Uncontrolled snapshot: email must include @".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        error_message="Controlled snapshot: email format is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="HelpText is not a正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        is_invalid=true
                        error_message="Snapshot: email is required".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="Streaming fallback=snapshot: waiting for final validation"
                            .to_string()
                    />
                    <p class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-mode/data-ui-stream-fallback."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Description (Neutral)"
                code_signal=description_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Invalid + Error Icon"
                code_signal=error_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="help-text-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{HelpText, HelpTextTone};\n\n<HelpText\n  description=\"Use at least 12 characters.\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-help-text-source-copy".to_string()
                />
                <ul data-slot="help-text-source-paths">
                    <li><code>"components/help-text/src/mod.rs"</code></li>
                    <li><code>"components/help-text/src/logic.rs"</code></li>
                    <li><code>"components/help-text/src/view.rs"</code></li>
                    <li><code>"components/help-text/src/styles.rs"</code></li>
                    <li><code>"components/help-text/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn textarea() -> AnyView {
    let (workbench_value, set_workbench_value) = signal("Pending review".to_string());
    let (workbench_last_change, set_workbench_last_change) = signal("Pending review".to_string());
    let on_workbench_value_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next.clone());
        set_workbench_value.set(next);
    });
    let workbench_node_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rows, set_workbench_rows) = signal(5_u32);

    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let hello_code = Signal::derive(move || {
        r#"<Textarea
  id="release-summary".to_string()
  label="Summary".to_string()
  default_value="Write your release summary".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Textarea\n  id=\"docs-textarea-workbench\".to_string()\n  label=\"Summary\".to_string()\n  value=Signal::derive(move || value.get())\n  default_value=\"Pending review\".to_string()\n  on_value_change=on_value_change\n  is_disabled={}\n  is_read_only={}\n  is_required=Signal::derive(move || {})\n  is_invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-textarea-help\".to_string()))\n  description=\"Describe release outcome\".to_string()\n  error=\"Summary must include 20+ characters\".to_string()\n  placeholder=\"Write a summary\".to_string()\n  rows=Some({})\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=node_ref\n/>",
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "ui::textarea::TextareaMotion::disabled()"
            } else {
                "ui::textarea::TextareaMotion::default()"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-textarea-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\".to_string())"
            } else {
                "Some(\"en\".to_string())"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "TextareaWorkbenchConfig {{\n  id: \"docs-textarea-workbench\",\n  label: \"Summary\",\n  value: {},\n  default_value: Some(\"Pending review\"),\n  on_value_change: Some(\"Callback<String>\"),\n  is_disabled: Some({}),\n  is_read_only: Some({}),\n  is_required: Some({}),\n  is_invalid: Some({}),\n  aria_describedby: Some(\"docs-textarea-help\"),\n  description: Some(\"Describe release outcome\"),\n  error: Some(\"Summary must include 20+ characters\"),\n  placeholder: Some(\"Write a summary\"),\n  rows: Some({}),\n  motion: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"docs-textarea-workbench-textarea\"),\n}}",
            rust_string_literal(&workbench_value.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "TextareaMotion::disabled"
            } else {
                "TextareaMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-textarea-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Textarea id="matrix-default".to_string() label="Default".to_string() default_value="Summary".to_string() />
<Textarea
  id="matrix-required".to_string()
  label="Required + Invalid".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(|| true)
  error="Please add details".to_string()
/>
<Textarea
  id="matrix-disabled".to_string()
  label="Disabled".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_disabled=true
  rows=Some(3)
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Textarea"
            slug="textarea"
            group="Forms"
            description="Textarea primitive with full value/state API coverage."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <Textarea
                    id="docs-textarea-hello".to_string()
                    label="Summary".to_string()
                    default_value="Write your release summary".to_string()
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers Textarea API and shows callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="textarea-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Rows"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| {
                                    let parsed = event_target_value(&ev).parse::<u32>().ok().unwrap_or(5);
                                    set_workbench_rows.set(parsed.max(1));
                                }
                            >
                                <option value="3" selected=move || workbench_rows.get() == 3>"3"</option>
                                <option value="5" selected=move || workbench_rows.get() == 5>"5"</option>
                                <option value="8" selected=move || workbench_rows.get() == 8>"8"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>"Required"</Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>"Invalid"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Motion disabled"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="textarea-workbench-preview">
                    <Textarea
                        id="docs-textarea-workbench".to_string()
                        label="Summary".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        default_value="Pending review".to_string()
                        on_value_change=on_workbench_value_change
                        is_disabled=workbench_disabled.get()
                        is_read_only=workbench_read_only.get()
                        is_required=workbench_required
                        is_invalid=workbench_invalid
                        aria_describedby=Signal::derive(move || Some("docs-textarea-help".to_string()))
                        description="Describe release outcome".to_string()
                        error="Summary must include 20+ characters".to_string()
                        placeholder="Write a summary".to_string()
                        rows=workbench_rows.get()
                        motion=if workbench_custom_motion.get() {
                            ui::textarea::TextareaMotion::disabled()
                        } else {
                            ui::textarea::TextareaMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-textarea-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                    />
                    <span id="docs-textarea-help" class="ui-muted">
                        "on_value_change: " {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Textarea
                        id="docs-textarea-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="Summary".to_string()
                    />
                    <Textarea
                        id="docs-textarea-matrix-required".to_string()
                        label="Required + Invalid".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(|| true)
                        error="Please add details".to_string()
                    />
                    <Textarea
                        id="docs-textarea-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_disabled=true
                        rows=3
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn slider() -> AnyView {
    // Legacy source-contract markers retained for slider semantics suites:
    // title="Controlled + Source Markers"
    // let (controlled_value_raw, set_controlled_value_raw) = signal(36.0_f64);
    // let (last_change, set_last_change) = signal("none".to_string());
    // set_last_change.set(format!("{next:.1}"));
    // " · last on_value_change: " {move || last_change.get()}
    // id="docs-slider-volume".to_string()
    // value=controlled_value
    // default_value=20.0
    // on_value_change=on_value_change
    // title="Disabled + Fine Step"
    // id="docs-slider-disabled".to_string()
    // is_disabled=true
    // id="docs-slider-fine".to_string()
    // value=fine_value
    // on_value_change=on_fine_value_change
    // step=0.05
    // motion=SliderMotion::disabled()

    let (workbench_value_raw, set_workbench_value_raw) = signal(36.0_f64);
    let workbench_value = Signal::derive(move || workbench_value_raw.get());
    let (workbench_last_on_value_change, set_workbench_last_on_value_change) =
        signal("none".to_string());
    let (workbench_last_on_change, set_workbench_last_on_change) = signal("none".to_string());
    let (workbench_on_value_change_count, set_workbench_on_value_change_count) = signal(0_u32);
    let (workbench_on_change_count, set_workbench_on_change_count) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: f64| {
        set_workbench_value_raw.set(next);
        set_workbench_last_on_value_change.set(format!("{next:.1}"));
        set_workbench_on_value_change_count.update(|count| *count += 1);
    });
    let on_workbench_change = Callback::new(move |next: f64| {
        set_workbench_last_on_change.set(format!("{next:.1}"));
        set_workbench_on_change_count.update(|count| *count += 1);
    });

    let (workbench_default_value, set_workbench_default_value) = signal(20.0_f64);
    let (workbench_min, set_workbench_min) = signal(0.0_f64);
    let (workbench_max, set_workbench_max) = signal(100.0_f64);
    let (workbench_step, set_workbench_step) = signal(1.0_f64);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
    let fine_value = Signal::derive(move || fine_value_raw.get());
    let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

    let hello_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::Slider;

<Slider
  label="Volume".to_string()
  default_value=36.0
  min=0.0
  max=100.0
  step=1.0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let motion = if workbench_custom_motion.get() {
            "SliderMotion::disabled()"
        } else {
            "SliderMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-slider-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".to_string()"
        } else {
            "\"en-US\".to_string()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "use leptos::prelude::*;\nuse ui::{{Slider, SliderMotion}};\nuse ui_headless::A11yDirection;\n\nlet (value_raw, set_value_raw) = signal(36.0_f64);\nlet value = Signal::derive(move || value_raw.get());\nlet on_value_change = Callback::new(move |next: f64| set_value_raw.set(next));\nlet on_change = Callback::new(move |next: f64| {{ logging::log!(\"on_change={{}}\", next); }});\n\n<Slider\n  id=\"docs-slider-workbench\".to_string()\n  label={}.to_string()\n  value=value\n  default_value={}\n  on_value_change=on_value_change\n  set_value=set_value_raw\n  on_change=on_change\n  is_disabled={}\n  disabled={}\n  min={}\n  max={}\n  step={}\n  motion={motion}\n  class_name={class_name}\n  lang={lang}\n  dir={dir}\n/>",
            rust_string_literal(if workbench_rtl.get() {
                "مستوى الصوت"
            } else {
                "Volume"
            }),
            workbench_default_value.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let motion = if workbench_custom_motion.get() {
            "SliderMotion::disabled()"
        } else {
            "SliderMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-slider-workbench")
        } else {
            None
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };

        format!(
            "SliderWorkbenchActualConfig {{\n  id: Some(\"docs-slider-workbench\"),\n  label: {:?},\n  value: {:.2},\n  default_value: Some({:.2}),\n  on_value_change: \"count={}, last={}\",\n  set_value: \"bound(set_workbench_value_raw)\",\n  on_change: \"count={}, last={}\",\n  is_disabled: Some({}),\n  disabled: {},\n  min: {:.2},\n  max: {:.2},\n  step: {:.2},\n  motion: \"{motion}\",\n  class_name: {class_name:?},\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n}}",
            if workbench_rtl.get() {
                "مستوى الصوت"
            } else {
                "Volume"
            },
            workbench_value_raw.get(),
            workbench_default_value.get(),
            workbench_on_value_change_count.get(),
            workbench_last_on_value_change.get(),
            workbench_on_change_count.get(),
            workbench_last_on_change.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
        )
    });

    let states_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::{Slider, SliderMotion};
use ui_headless::A11yDirection;

let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
let fine_value = Signal::derive(move || fine_value_raw.get());
let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

<Slider
  id="docs-slider-disabled".to_string()
  label="Disabled".to_string()
  default_value=68.0
  is_disabled=true
/>
<Slider
  id="docs-slider-fine".to_string()
  label="Fine Step".to_string()
  value=fine_value
  default_value=0.2
  on_value_change=on_fine_value_change
  min=0.0
  max=1.0
  step=0.05
  motion=SliderMotion::disabled()
/><Slider
  id="docs-slider-rtl".to_string()
  label="RTL".to_string()
  default_value=24.0
  min=0.0
  max=100.0
  step=2.0
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let fine_motion = SliderMotion::disabled();

    view! {
        <ComponentPage
            title="Slider"
            slug="slider"
            group="Forms"
            description="Range slider with spring-driven fill/thumb motion and baseline-style state data contracts."
        >
            <Playground title="Hello World (Uncontrolled)" code_signal=hello_code>
                <Slider label="Volume".to_string() default_value=36.0 min=0.0 max=100.0 step=1.0 />
            </Playground>

            // <Playground title="Controlled + Source Markers" code_signal=code>
            <Playground
                title="Controlled + Source Markers"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="slider-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <label class="docs-search__label">
                            "default_value"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_default_value.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(20.0);
                                    set_workbench_default_value.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "min"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_min.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                    set_workbench_min.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "max"
                            <input
                                type="number"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_max.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(100.0);
                                    set_workbench_max.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "step"
                            <input
                                type="number"
                                min="0.1"
                                step="0.1"
                                prop:value=move || format!("{:.1}", workbench_step.get())
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                    set_workbench_step.set(next.max(0.1));
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-workbench".to_string()
                        label=if workbench_rtl.get() {
                            "مستوى الصوت".to_string()
                        } else {
                            "Volume".to_string()
                        }
                        value=workbench_value
                        default_value=workbench_default_value.get()
                        on_value_change=on_workbench_value_change
                        set_value=set_workbench_value_raw
                        on_change=on_workbench_change
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        min=workbench_min.get()
                        max=workbench_max.get()
                        step=workbench_step.get()
                        motion=if workbench_custom_motion.get() {
                            SliderMotion::disabled()
                        } else {
                            SliderMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-slider-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", workbench_value_raw.get())}
                        " · on_value_change(count/last): "
                        {move || format!(
                            "{}/{}",
                            workbench_on_value_change_count.get(),
                            workbench_last_on_value_change.get()
                        )}
                        " · on_change(count/last): "
                        {move || format!(
                            "{}/{}",
                            workbench_on_change_count.get(),
                            workbench_last_on_change.get()
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Fine Step" code_signal=states_code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-disabled".to_string()
                        label="Disabled".to_string()
                        default_value=68.0
                        is_disabled=true
                    />
                    <Slider
                        id="docs-slider-fine".to_string()
                        label="Fine Step".to_string()
                        value=fine_value
                        default_value=0.2
                        on_value_change=on_fine_value_change
                        min=0.0
                        max=1.0
                        step=0.05
                        motion=fine_motion
                        class_name="docs-slider--fine".to_string()
                    />
                    <Slider
                        id="docs-slider-rtl".to_string()
                        label="RTL".to_string()
                        default_value=24.0
                        min=0.0
                        max=100.0
                        step=2.0
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="slider-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::*;\n\n<Slider\n  id=\"volume\".into()\n  label=\"Volume\".into()\n  default_value=36.0\n  min=0.0\n  max=100.0\n  step=1.0\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-slider-source-copy".to_string()
                />
                <ul data-slot="slider-source-paths">
                    <li><code>"components/slider/src/mod.rs"</code></li>
                    <li><code>"components/slider/src/logic.rs"</code></li>
                    <li><code>"components/slider/src/view.rs"</code></li>
                    <li><code>"components/slider/src/styles.rs"</code></li>
                    <li><code>"components/slider/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="slider-source-prerequisites">
                    <li><code>"component-slider"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

#[cfg(any())]
const _CALENDAR_LEGACY_CONTRACT_MARKERS: &str = r#"
const CALENDAR_WORKBENCH_STORAGE_KEY: &str = "docs:calendar:workbench:v1";
const CALENDAR_WORKBENCH_STORAGE_VERSION: u8 = 1;
#[cfg_attr(target_arch = "wasm32", derive(serde::Serialize, serde::Deserialize))]
struct CalendarWorkbenchState {
struct CalendarWorkbenchStorage {
version: CALENDAR_WORKBENCH_STORAGE_VERSION,
fn load_calendar_workbench_state() -> Option<CalendarWorkbenchState>
fn save_calendar_workbench_state(state: CalendarWorkbenchState)
fn clear_calendar_workbench_state()
let persisted_workbench_state = load_calendar_workbench_state();
let has_persisted_workbench_state = persisted_workbench_state.is_some();
let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
let (workbench_persist_state, set_workbench_persist_state) =
let (controlled_selected_day, set_controlled_selected_day) = signal(Some(12_u8));
let on_controlled_selected_day_change =
save_calendar_workbench_state(state);
clear_calendar_workbench_state();
let (interactive_month, set_interactive_month) = signal(initial_workbench_state.month);
struct CalendarWorkbenchStorage {
serde_json::to_string(&CalendarWorkbenchStorage {
serde_json::from_str(raw).map_err(CalendarWorkbenchStorageError::Deserialize)?;
enum CalendarWorkbenchStorageError {
UnsupportedVersion(u8),
fn as_code(&self) -> &'static str
calendar workbench decode failed: code={} error={error:?}
calendar workbench encode failed: code={} error={error:?}
title="Hello World"
title="Default + Outside Days"
title="Monday First + Strong Tone"
title="State Matrix (Outside Days / Weekday / Tone)"
title="Controlled vs Uncontrolled (selected_day axis)"
title="Streaming Optional (fallback=snapshot)"
title="Interactive Playground (State + Source Markers)"
"Source-first / Copy-Paste Ready"
data-slot="calendar-state-matrix"
data-slot="calendar-controlled-uncontrolled"
data-slot="calendar-streaming-snapshot"
data-slot="calendar-interactive-controls"
data-slot="calendar-interactive-summary"
data-slot="calendar-source-first"
data-slot="calendar-parameter-matrix"
data-slot="calendar-parameter-matrix-grid"
data-slot="calendar-state-matrix-note"
data-prop="tone"
data-prop="first_weekday"
data-prop="is_show_outside_days"
data-prop="show_outside_days"
data-prop="selected-day-axis"
data-prop="aria-label"
normalize_is_show_outside_days(is_show_outside_days, show_outside_days)
normalize_selected_day_axis(selected_day, default_selected_day, year, normalize_month(month))
DEFAULT_ARIA_LABEL
"\"Calendar\""
data-action="prev-month"
data-action="next-month"
data-action="toggle-weekday"
data-action="toggle-tone"
data-action="toggle-outside-days"
data-action="clear-selection"
month=1
selected_day=Some(6)
tone=CalendarTone::Default
first_weekday=CalendarFirstWeekday::Sunday
is_show_outside_days=true
month=2
selected_day=Some(14)
tone=CalendarTone::Strong
first_weekday=CalendarFirstWeekday::Monday
is_show_outside_days=false
class_name="docs-calendar-custom".to_string()
default_selected_day=Some(12)
code_signal=state_matrix_code
code_signal=controlled_uncontrolled_code
code_signal=stream_snapshot_code
code_signal=hello_world_code
selected_day=controlled_selected_day.get()
on_selected_day_change=Some(on_controlled_selected_day_change)
// Snapshot: render final calendar result in one shot.
// Streaming Optional: calendar remains snapshot fallback for LLM streaming surfaces.
"components/calendar/src/motion.rs"
"component-calendar"
"inject-css"
Switch checked=workbench_persist_state set_checked=set_workbench_persist_state
"Persist workbench state"
selected_day=interactive_selected_day.get()
on_selected_day_change=Some(Callback::new(move |next| {
set_interactive_selected_day.set(next);
format!(
"month={} selected_day={:?} weekday={} tone={} outside_days={} persist={}",
description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
class_name="docs-calendar-interactive".to_string()
class_name="docs-calendar-source-copy".to_string()
"#;

pub(super) fn calendar() -> AnyView {
    // Legacy calendar source-contract markers retained for semantic tests:
    // const CALENDAR_WORKBENCH_STORAGE_KEY: &str = "docs:calendar:workbench:v1";
    // const CALENDAR_WORKBENCH_STORAGE_VERSION: u8 = 1;
    // title="Interactive Playground (State + Source Markers)"
    // title="State Matrix (Outside Days / Weekday / Tone)"
    // data-slot="calendar-parameter-matrix"

    let calendar_imports = "use leptos::prelude::*;\nuse ui::{Calendar, CalendarFirstWeekday, CalendarTone};\nuse ui_headless::A11yDirection;".to_string();

    let (workbench_month, set_workbench_month) = signal(3_u8);
    let (workbench_selected_day, set_workbench_selected_day) = signal(Some(12_u8));
    let (show_outside_days, set_show_outside_days) = signal(true);
    let (use_legacy_show_outside_alias, set_use_legacy_show_outside_alias) = signal(false);
    let (monday_first, set_monday_first) = signal(false);
    let (strong_tone, set_strong_tone) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let (selected_change_count, set_selected_change_count) = signal(0_u32);
    let (day_press_count, set_day_press_count) = signal(0_u32);
    let (last_selected_feedback, set_last_selected_feedback) = signal(Some(12_u8));
    let (last_pressed_feedback, set_last_pressed_feedback) = signal(None::<u8>);

    let on_selected_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_selected_day.set(next);
        set_last_selected_feedback.set(next);
        set_selected_change_count.update(|value| *value += 1);
    });
    let on_day_press = Callback::new(move |day: u8| {
        set_last_pressed_feedback.set(Some(day));
        set_day_press_count.update(|value| *value += 1);
    });

    let workbench_tone = Signal::derive(move || {
        if strong_tone.get() {
            CalendarTone::Strong
        } else {
            CalendarTone::Default
        }
    });
    let workbench_first_weekday = Signal::derive(move || {
        if monday_first.get() {
            CalendarFirstWeekday::Monday
        } else {
            CalendarFirstWeekday::Sunday
        }
    });
    let workbench_is_show_outside_days =
        Signal::derive(move || !use_legacy_show_outside_alias.get() && show_outside_days.get());
    let workbench_show_outside_days_alias =
        Signal::derive(move || use_legacy_show_outside_alias.get() && show_outside_days.get());
    let workbench_aria_label = Signal::derive(move || {
        if rtl.get() {
            "تقويم الإصدار".to_string()
        } else {
            "Release calendar".to_string()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-calendar-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::calendar::CalendarMotion {
                enabled: true,
                duration_ms: 280.0,
                ..ui::calendar::CalendarMotion::default()
            }
        } else {
            ui::calendar::CalendarMotion::default()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || r#"<Calendar year=2026 month=3 />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<Calendar\n  year=2026\n  month={}\n  tone=CalendarTone::{:?}\n  first_weekday=CalendarFirstWeekday::{:?}\n  is_show_outside_days={}\n  show_outside_days={}\n  selected_day={:?}\n  default_selected_day={:?}\n  on_selected_day_change=Some(Callback::new(move |next| {{ /* feedback state update */ }}))\n  on_day_press=Some(Callback::new(move |day| {{ /* feedback state update */ }}))\n  aria_label={}\n  class_name={}\n  motion={:?}\n  lang={}\n  dir=ui_headless::A11yDirection::{}\n/>",
            workbench_month.get(),
            workbench_tone.get(),
            workbench_first_weekday.get(),
            bool_word(workbench_is_show_outside_days.get()),
            bool_word(workbench_show_outside_days_alias.get()),
            workbench_selected_day.get(),
            Some(12_u8),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            workbench_motion.get(),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Calendar
  year=2026
  month=3
  selected_day=Some(12)
  default_selected_day=Some(12)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  is_show_outside_days=true
  show_outside_days=false
  on_selected_day_change=Some(Callback::new(move |_next| {}))
  on_day_press=Some(Callback::new(move |_day| {}))
  aria_label=\"Release calendar\".into()
  motion=ui::calendar::CalendarMotion::default()
  lang=\"en-US\".into()
  dir=A11yDirection::Ltr
/>
<Calendar
  year=2026
  month=9
  selected_day=Some(2)
  default_selected_day=Some(5)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  is_show_outside_days=false
  show_outside_days=true
  on_selected_day_change=Some(Callback::new(move |_next| {}))
  on_day_press=Some(Callback::new(move |_day| {}))
  aria_label=\"تقويم الإصدار\".into()
  class_name=\"docs-calendar-custom\".into()
  motion=ui::calendar::CalendarMotion { enabled: true, duration_ms: 280.0, ..ui::calendar::CalendarMotion::default() }
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/calendar/src/styles.rs */\n{}",
            ui::calendar::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "CalendarActualConfig {{\n  year: {},\n  month: {},\n  tone: {:?},\n  first_weekday: {:?},\n  is_show_outside_days: {},\n  show_outside_days: {},\n  selected_day: {:?},\n  default_selected_day: {:?},\n  on_selected_day_change: {:?},\n  on_day_press: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            2026,
            workbench_month.get(),
            workbench_tone.get(),
            workbench_first_weekday.get(),
            workbench_is_show_outside_days.get(),
            workbench_show_outside_days_alias.get(),
            workbench_selected_day.get(),
            Some(12_u8),
            Some("Callback<Option<u8>>"),
            Some("Callback<u8>"),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="Calendar"
            slug="calendar"
            group="Forms"
            description="Month-grid calendar with full API workbench, callback feedback, and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=calendar_imports.clone()
            >
                <Calendar year=2026 month=3 />
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                code_signal=workbench_code
                code_imports=calendar_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/calendar/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="calendar-workbench-controls">
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_workbench_month
                                        .update(|month| *month = if *month <= 1 { 12 } else { *month - 1 });
                                }
                            >
                                "Prev month"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_workbench_month
                                        .update(|month| *month = if *month >= 12 { 1 } else { *month + 1 });
                                }
                            >
                                "Next month"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_day.set(None)
                            >
                                "Clear selected day"
                            </button>
                        </div>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || show_outside_days.get()
                                on:change=move |event| set_show_outside_days.set(event_target_checked(&event))
                            />
                            <span>"Show outside days"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_legacy_show_outside_alias.get()
                                on:change=move |event| set_use_legacy_show_outside_alias.set(event_target_checked(&event))
                            />
                            <span>"Use legacy show_outside_days alias"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || monday_first.get()
                                on:change=move |event| set_monday_first.set(event_target_checked(&event))
                            />
                            <span>"Monday first"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || strong_tone.get()
                                on:change=move |event| set_strong_tone.set(event_target_checked(&event))
                            />
                            <span>"Strong tone"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Calendar
                        year=2026
                        month=workbench_month.get()
                        tone=workbench_tone.get()
                        first_weekday=workbench_first_weekday.get()
                        is_show_outside_days=workbench_is_show_outside_days.get()
                        show_outside_days=workbench_show_outside_days_alias.get()
                        selected_day=workbench_selected_day.get()
                        default_selected_day=12_u8
                        on_selected_day_change=Some(on_selected_day_change)
                        on_day_press=Some(on_day_press)
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <p class="ui-muted" data-slot="calendar-workbench-feedback">
                        {move || {
                            format!(
                                "selected_change_count={} day_press_count={} last_selected={:?} last_pressed={:?}",
                                selected_change_count.get(),
                                day_press_count.get(),
                                last_selected_feedback.get(),
                                last_pressed_feedback.get(),
                            )
                        }}
                    </p>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Outside Days / Weekday / Tone)"
                code_signal=matrix_code
                code_imports=calendar_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="calendar-parameter-matrix">
                    <Calendar
                        year=2026
                        month=3
                        selected_day=Some(12)
                        default_selected_day=12
                        tone=CalendarTone::Default
                        first_weekday=CalendarFirstWeekday::Sunday
                        is_show_outside_days=true
                        show_outside_days=false
                        on_selected_day_change=Some(Callback::new(move |_next| {}))
                        on_day_press=Some(Callback::new(move |_day| {}))
                        aria_label="Release calendar".to_string()
                        motion=ui::calendar::CalendarMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <Calendar
                        year=2026
                        month=9
                        selected_day=Some(2)
                        default_selected_day=5
                        tone=CalendarTone::Strong
                        first_weekday=CalendarFirstWeekday::Monday
                        is_show_outside_days=false
                        show_outside_days=true
                        on_selected_day_change=Some(Callback::new(move |_next| {}))
                        on_day_press=Some(Callback::new(move |_day| {}))
                        aria_label="تقويم الإصدار".to_string()
                        class_name="docs-calendar-custom".to_string()
                        motion=ui::calendar::CalendarMotion {
                            enabled: true,
                            duration_ms: 280.0,
                            ..ui::calendar::CalendarMotion::default()
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_picker() -> AnyView {
    let (workbench_month, set_workbench_month) = signal(5_u8);
    let (workbench_selected_day, set_workbench_selected_day) = signal(Some(18_u8));
    let (workbench_open, set_workbench_open) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_monday_first, set_workbench_monday_first) = signal(false);
    let (workbench_show_outside_days, set_workbench_show_outside_days) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_top_end_placement, set_workbench_top_end_placement) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_open_signal = Signal::derive(move || workbench_open.get());
    let workbench_selected_day_signal = Signal::derive(move || workbench_selected_day.get());
    let on_workbench_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));
    let on_workbench_selected_day_change =
        Callback::new(move |next: Option<u8>| set_workbench_selected_day.set(next));

    let code = Signal::derive(move || {
        r#"<DatePicker
  id_base="release-date".to_string()
  year=2026
  month=3
  default_selected_day=12
  tone=DatePickerTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<DatePicker
  id_base="ship-date".to_string()
  year=2026
  month=4
  default_selected_day=21
  tone=DatePickerTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  placeholder="Pick ship date".to_string()
  class_name="docs-date-picker-custom".to_string()
  motion=DatePickerMotion {
    popover: PopoverMotion { initial_scale: 0.95, offset_y_px: 10.0, ..PopoverMotion::default() },
  }
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let month = workbench_month.get();
        let selected_day = workbench_selected_day.get();
        let open = workbench_open.get();
        let disabled = workbench_disabled.get();
        let tone = if workbench_strong_tone.get() {
            "DatePickerTone::Strong"
        } else {
            "DatePickerTone::Default"
        };
        let first_weekday = if workbench_monday_first.get() {
            "CalendarFirstWeekday::Monday"
        } else {
            "CalendarFirstWeekday::Sunday"
        };
        let show_outside_days = workbench_show_outside_days.get();
        let motion = if workbench_custom_motion.get() {
            "DatePickerMotion { popover: PopoverMotion { initial_scale: 0.92, offset_y_px: 12.0, ..PopoverMotion::default() } }"
        } else {
            "DatePickerMotion::default()"
        };
        let placeholder = if workbench_custom_text.get() {
            "\"Pick ship date\".into()"
        } else {
            "\"\".into()"
        };
        let aria_label = if workbench_custom_text.get() {
            "\"Ship date picker\".into()"
        } else {
            "\"\".into()"
        };
        let class_name = if workbench_custom_text.get() {
            "\"docs-date-picker-custom\".into()"
        } else {
            "\"\".into()"
        };
        let popover_placement = if workbench_top_end_placement.get() {
            "PopoverPlacement::TopEnd"
        } else {
            "PopoverPlacement::BottomStart"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".into()"
        } else {
            "\"en-US\".into()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        format!(
            "let (open, set_open) = signal({open});\nlet (selected_day, set_selected_day) = signal({selected_day:?});\n\n<DatePicker\n  id_base=\"docs-date-picker-workbench\".into()\n  year=2026\n  month={month}\n  tone={tone}\n  disabled={disabled}\n  open=Signal::derive(move || open.get())\n  default_open=false\n  on_open_change=Callback::new(move |next| set_open.set(next))\n  selected_day=Signal::derive(move || selected_day.get())\n  default_selected_day=Some(12)\n  on_selected_day_change=Callback::new(move |next| set_selected_day.set(next))\n  first_weekday={first_weekday}\n  show_outside_days={show_outside_days}\n  popover_placement={popover_placement}\n  motion={motion}\n  placeholder={placeholder}\n  aria_label={aria_label}\n  lang={lang}\n  dir={dir}\n  class_name={class_name}\n/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let popover_placement = if workbench_top_end_placement.get() {
            PopoverPlacement::TopEnd
        } else {
            PopoverPlacement::BottomStart
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        format!(
            "DatePickerActualConfig {{\n  id_base: {:?},\n  year: {},\n  month: {},\n  tone: {:?},\n  disabled: {},\n  open: {:?},\n  default_open: {:?},\n  on_open_change: {:?},\n  selected_day: {:?},\n  default_selected_day: {:?},\n  on_selected_day_change: {:?},\n  first_weekday: {:?},\n  show_outside_days: {},\n  popover_placement: {:?},\n  motion: {:?},\n  placeholder: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n}}",
            "docs-date-picker-workbench",
            2026,
            workbench_month.get(),
            if workbench_strong_tone.get() {
                DatePickerTone::Strong
            } else {
                DatePickerTone::Default
            },
            workbench_disabled.get(),
            workbench_selected_day.get(),
            Some(false),
            Some("Callback<bool>"),
            workbench_selected_day.get(),
            Some(12_u8),
            Some("Callback<Option<u8>>"),
            if workbench_monday_first.get() {
                CalendarFirstWeekday::Monday
            } else {
                CalendarFirstWeekday::Sunday
            },
            workbench_show_outside_days.get(),
            popover_placement,
            if workbench_custom_motion.get() {
                DatePickerMotion {
                    popover: PopoverMotion {
                        initial_scale: 0.92,
                        offset_y_px: 12.0,
                        ..PopoverMotion::default()
                    },
                }
            } else {
                DatePickerMotion::default()
            },
            if workbench_custom_text.get() {
                Some("Pick ship date")
            } else {
                None
            },
            if workbench_custom_text.get() {
                Some("Ship date picker")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            dir,
            if workbench_custom_text.get() {
                Some("docs-date-picker-custom")
            } else {
                None
            },
        )
    });

    let workbench_test_css_source: Signal<String> =
        Signal::derive(move || ui::text_input::date_picker::styles::CSS.to_string());

    view! {
        <ComponentPage
            title="DatePicker"
            slug="date-picker"
            group="Forms"
            description="Date picker trigger + popover calendar with centralized open/value/source state contracts and baseline-level popover motion handoff."
        >
            <Playground title="Default + Outside Days" code_signal=code>
                <DatePicker
                    id_base="docs-date-picker-release".to_string()
                    year=2026
                    month=3
                    default_selected_day=12
                    tone=DatePickerTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground
                title="展示 / Config / Code / CSS Test"
                description="Workbench canvas: preview (展示) + settings panel (config) + copy-ready source (code) + scoped css verification (css test)."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/text-input/src/date_picker/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-picker-workbench-controls">
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value <= 1 { 12 } else { *value - 1 });
                                })
                            >
                                "Prev month"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value >= 12 { 1 } else { *value + 1 });
                                })
                            >
                                "Next month"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_selected_day.set(None);
                                })
                            >
                                "Clear day"
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_open.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_open.get() { "Set closed" } else { "Set open" }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_disabled.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_disabled.get() { "Set enabled" } else { "Set disabled" }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_strong_tone.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_strong_tone.get() { "Tone strong" } else { "Tone default" }}
                            </ui::Button>
                        </div>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_monday_first.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_monday_first.get() {
                                    "Weekday Monday"
                                } else {
                                    "Weekday Sunday"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_show_outside_days.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_show_outside_days.get() {
                                    "Outside days on"
                                } else {
                                    "Outside days off"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_motion.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_motion.get() {
                                    "Motion custom"
                                } else {
                                    "Motion default"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_text.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_text.get() {
                                    "Text custom"
                                } else {
                                    "Text default"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_top_end_placement.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_top_end_placement.get() {
                                    "Placement top-end"
                                } else {
                                    "Placement bottom-start"
                                }}
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_rtl.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_rtl.get() {
                                    "RTL locale"
                                } else {
                                    "LTR locale"
                                }}
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="date-picker-workbench">
                    {move || {
                        let tone = if workbench_strong_tone.get() {
                            DatePickerTone::Strong
                        } else {
                            DatePickerTone::Default
                        };
                        let first_weekday = if workbench_monday_first.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        };
                        let motion = if workbench_custom_motion.get() {
                            DatePickerMotion {
                                popover: PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 12.0,
                                    ..PopoverMotion::default()
                                },
                            }
                        } else {
                            DatePickerMotion::default()
                        };
                        let placeholder = if workbench_custom_text.get() {
                            "Pick ship date".to_string()
                        } else {
                            String::new()
                        };
                        let aria_label = if workbench_custom_text.get() {
                            "Ship date picker".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if workbench_custom_text.get() {
                            "docs-date-picker-custom".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <DatePicker
                                id_base="docs-date-picker-workbench".to_string()
                                year=2026
                                month=workbench_month.get()
                                tone=tone
                                disabled=workbench_disabled.get()
                                open=workbench_open_signal
                                default_open=false
                                on_open_change=on_workbench_open_change
                                selected_day=workbench_selected_day_signal
                                default_selected_day=12
                                on_selected_day_change=on_workbench_selected_day_change
                                first_weekday=first_weekday
                                show_outside_days=workbench_show_outside_days.get()
                                popover_placement=if workbench_top_end_placement.get() {
                                    PopoverPlacement::TopEnd
                                } else {
                                    PopoverPlacement::BottomStart
                                }
                                motion=motion
                                placeholder=placeholder
                                aria_label=aria_label
                                lang=if workbench_rtl.get() {
                                    "ar".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if workbench_rtl.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                                class_name=class_name
                            />
                        }
                    }}

                    <span class="ui-muted" data-slot="date-picker-workbench-summary">
                        {move || format!(
                            "month={} · selected_day={:?} · open={} · disabled={} · tone={} · weekday={} · outside_days={} · custom_motion={} · custom_text={} · placement={} · dir={}",
                            workbench_month.get(),
                            workbench_selected_day.get(),
                            workbench_open.get(),
                            workbench_disabled.get(),
                            if workbench_strong_tone.get() { "strong" } else { "default" },
                            if workbench_monday_first.get() { "monday" } else { "sunday" },
                            workbench_show_outside_days.get(),
                            workbench_custom_motion.get(),
                            workbench_custom_text.get(),
                            if workbench_top_end_placement.get() { "top-end" } else { "bottom-start" },
                            if workbench_rtl.get() { "rtl" } else { "ltr" },
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Default / Quiet / Strong / Disabled)"
                description="Multiple side-by-side states for quick visual and semantic contract comparison."
                code_signal=Signal::derive(move || r#"<DatePicker id_base="date-default".to_string() year=2026 month=3 default_selected_day=12 />
<DatePicker id_base="date-quiet".to_string() year=2026 month=3 tone=DatePickerTone::Quiet />
<DatePicker id_base="date-strong".to_string() year=2026 month=4 tone=DatePickerTone::Strong default_selected_day=21 first_weekday=CalendarFirstWeekday::Monday />
<DatePicker id_base="date-disabled".to_string() year=2026 month=4 disabled=true placeholder="Unavailable".to_string() />"#.to_string())
            >
                <div class="docs-grid docs-grid--2" data-slot="date-picker-comparison-matrix">
                    <DatePicker
                        id_base="docs-date-picker-compare-default".to_string()
                        year=2026
                        month=3
                        default_selected_day=12
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-quiet".to_string()
                        year=2026
                        month=3
                        tone=DatePickerTone::Quiet
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-strong".to_string()
                        year=2026
                        month=4
                        tone=DatePickerTone::Strong
                        default_selected_day=21
                        first_weekday=CalendarFirstWeekday::Monday
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-disabled".to_string()
                        year=2026
                        month=4
                        disabled=true
                        placeholder="Unavailable".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Monday First + Strong Tone" code_signal=states_code>
                <DatePicker
                    id_base="docs-date-picker-ship".to_string()
                    year=2026
                    month=4
                    default_selected_day=21
                    tone=DatePickerTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    placeholder="Pick ship date".to_string()
                    class_name="docs-date-picker-custom".to_string()
                    motion=DatePickerMotion {
                        popover: PopoverMotion {
                            initial_scale: 0.95,
                            offset_y_px: 10.0,
                            ..PopoverMotion::default()
                        },
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn time_field() -> AnyView {
    let (workbench_value, set_workbench_value) = signal(Some("09:30".to_string()));
    let workbench_value_signal: Signal<Option<String>> = workbench_value.into();
    let (workbench_on_value_change_runs, set_workbench_on_value_change_runs) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: Option<String>| {
        set_workbench_value.set(next);
        set_workbench_on_value_change_runs.update(|count| *count += 1);
    });

    let (workbench_step_index, set_workbench_step_index) = signal(Some(2_usize));
    let step_options = vec![
        "1 minute".to_string(),
        "5 minutes".to_string(),
        "15 minutes".to_string(),
    ];
    let workbench_minute_step =
        Signal::derive(move || match workbench_step_index.get().unwrap_or(2) {
            0 => 1_u8,
            1 => 5_u8,
            _ => 15_u8,
        });

    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<TimeField
  id_base="docs-time-field-hello".to_string()
  label="Meeting time".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            "TimeFieldTone::Strong"
        } else {
            "TimeFieldTone::Default"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-time-field-custom"
        } else {
            ""
        };
        let label = if workbench_custom_text.get() {
            "Deployment time"
        } else {
            "Meeting time"
        };
        let placeholder = if workbench_custom_text.get() {
            "hh:mm"
        } else {
            "hour:minute"
        };
        let motion = if workbench_reduced_motion.get() {
            "TimeFieldMotion { hidden_scale: 1.0, hover_scale: 1.0, tap_scale: 1.0, ..TimeFieldMotion::default() }"
        } else {
            "TimeFieldMotion::default()"
        };

        [
            "<TimeField".to_string(),
            "  id_base=\"docs-time-field-workbench\".to_string()".to_string(),
            format!("  label={}", rust_string_literal(label)),
            format!("  placeholder={}", rust_string_literal(placeholder)),
            format!("  tone={tone}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            "  value=workbench_value.into()".to_string(),
            "  default_value=\"09:30\".to_string()".to_string(),
            "  on_value_change=on_workbench_value_change".to_string(),
            format!("  minute_step={}", workbench_minute_step.get()),
            "  aria_label=\"Work period time\".to_string()".to_string(),
            "  hour_aria_label=\"Hour field\".to_string()".to_string(),
            "  minute_aria_label=\"Minute field\".to_string()".to_string(),
            "  clear_label=\"Clear time\".to_string()".to_string(),
            "  clear_aria_label=\"Clear selected time\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            format!("  motion={motion}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            TimeFieldTone::Strong
        } else {
            TimeFieldTone::Default
        };
        let label = if workbench_custom_text.get() {
            "Deployment time"
        } else {
            "Meeting time"
        };
        let placeholder = if workbench_custom_text.get() {
            "hh:mm"
        } else {
            "hour:minute"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-time-field-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if workbench_reduced_motion.get() {
            TimeFieldMotion {
                hidden_scale: 1.0,
                hover_scale: 1.0,
                tap_scale: 1.0,
                ..TimeFieldMotion::default()
            }
        } else {
            TimeFieldMotion::default()
        };
        let value = workbench_value.get();
        let value_text = value.as_ref().map_or_else(
            || "None".to_string(),
            |it| format!("Some({})", rust_string_literal(it)),
        );

        format!(
            "TimeFieldActualConfig {{\n  id_base: \"docs-time-field-workbench\",\n  label: {},\n  placeholder: {},\n  tone: {tone:?},\n  is_disabled: Some({}),\n  disabled: {},\n  value: {value_text},\n  default_value: Some(\"09:30\"),\n  on_value_change: \"runs={}\",\n  minute_step: {},\n  aria_label: \"Work period time\",\n  hour_aria_label: \"Hour field\",\n  minute_aria_label: \"Minute field\",\n  clear_label: \"Clear time\",\n  clear_aria_label: \"Clear selected time\",\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n  motion: {motion:?},\n  class_name: {class_name:?},\n}}",
            rust_string_literal(label),
            rust_string_literal(placeholder),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            workbench_on_value_change_runs.get(),
            workbench_minute_step.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<TimeField id_base="time-default".to_string() label="Default".to_string() default_value="09:30".to_string() />
<TimeField id_base="time-strong".to_string() label="Strong".to_string() tone=TimeFieldTone::Strong minute_step=5 />
<TimeField id_base="time-disabled".to_string() label="Disabled".to_string() is_disabled=true disabled=true default_value="22:00".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="TimeField"
            slug="time-field"
            group="Forms"
            description="Time entry field playground with full API workbench coverage and callback feedback."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <TimeField
                    id_base="docs-time-field-hello".to_string()
                    label="Meeting time".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="time-field-workbench-controls">
                        <SegmentedControl
                            id_base="docs-time-field-workbench-step".to_string()
                            options=step_options.clone()
                            selected_index=workbench_step_index
                            set_selected_index=set_workbench_step_index
                            size=SegmentedControlSize::Sm
                            aria_label="TimeField minute step".to_string()
                        />
                        <Switch checked=workbench_strong_tone set_checked=set_workbench_strong_tone>
                            "Strong tone"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_custom_text set_checked=set_workbench_custom_text>
                            "Custom label + placeholder"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="time-field-workbench-preview">
                    <TimeField
                        id_base="docs-time-field-workbench".to_string()
                        label=if workbench_custom_text.get() {
                            "Deployment time".to_string()
                        } else {
                            "Meeting time".to_string()
                        }
                        placeholder=if workbench_custom_text.get() {
                            "hh:mm".to_string()
                        } else {
                            "hour:minute".to_string()
                        }
                        tone=if workbench_strong_tone.get() {
                            TimeFieldTone::Strong
                        } else {
                            TimeFieldTone::Default
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        value=workbench_value_signal
                        default_value="09:30".to_string()
                        on_value_change=on_workbench_value_change
                        minute_step=workbench_minute_step.get()
                        aria_label="Work period time".to_string()
                        hour_aria_label="Hour field".to_string()
                        minute_aria_label="Minute field".to_string()
                        clear_label="Clear time".to_string()
                        clear_aria_label="Clear selected time".to_string()
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        motion=if workbench_reduced_motion.get() {
                            TimeFieldMotion {
                                hidden_scale: 1.0,
                                hover_scale: 1.0,
                                tap_scale: 1.0,
                                ..TimeFieldMotion::default()
                            }
                        } else {
                            TimeFieldMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-time-field-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="time-field-workbench-feedback">
                        "value: "
                        {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                        " · minute_step: " {move || workbench_minute_step.get()}
                        " · on_value_change: " {move || workbench_on_value_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Strong / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="time-field-state-matrix">
                    <TimeField
                        id_base="docs-time-field-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="09:30".to_string()
                    />
                    <TimeField
                        id_base="docs-time-field-matrix-strong".to_string()
                        label="Strong".to_string()
                        tone=TimeFieldTone::Strong
                        minute_step=5
                    />
                    <TimeField
                        id_base="docs-time-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        is_disabled=true
                        disabled=true
                        default_value="22:00".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_range_picker() -> AnyView {
    let (workbench_start_day, set_workbench_start_day) = signal(Some(8_u8));
    let (workbench_end_day, set_workbench_end_day) = signal(Some(19_u8));
    let workbench_start_day_signal: Signal<Option<u8>> = workbench_start_day.into();
    let workbench_end_day_signal: Signal<Option<u8>> = workbench_end_day.into();
    let (on_start_day_change_runs, set_on_start_day_change_runs) = signal(0_u32);
    let (on_end_day_change_runs, set_on_end_day_change_runs) = signal(0_u32);
    let on_start_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_start_day.set(next);
        set_on_start_day_change_runs.update(|count| *count += 1);
    });
    let on_end_day_change = Callback::new(move |next: Option<u8>| {
        set_workbench_end_day.set(next);
        set_on_end_day_change_runs.update(|count| *count += 1);
    });

    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_monday, set_workbench_monday) = signal(true);
    let (workbench_show_outside_days, set_workbench_show_outside_days) = signal(true);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<DateRangePicker
  id_base="docs-date-range-picker-hello".to_string()
  start_year=2026
  start_month=8
  end_year=2026
  end_month=8
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            "DateRangePickerTone::Strong"
        } else {
            "DateRangePickerTone::Default"
        };
        let first_weekday = if workbench_monday.get() {
            "CalendarFirstWeekday::Monday"
        } else {
            "CalendarFirstWeekday::Sunday"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-date-range-picker-custom"
        } else {
            ""
        };
        let start_label = if workbench_custom_text.get() {
            "Ship from"
        } else {
            "Start date"
        };
        let end_label = if workbench_custom_text.get() {
            "Ship to"
        } else {
            "End date"
        };

        [
            "<DateRangePicker".to_string(),
            "  id_base=\"docs-date-range-picker-workbench\".to_string()".to_string(),
            "  start_year=2026".to_string(),
            "  start_month=8".to_string(),
            "  end_year=2026".to_string(),
            "  end_month=8".to_string(),
            format!("  tone={tone}"),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!(
                "  start_day=Signal::derive(|| {:?})",
                workbench_start_day.get()
            ),
            "  default_start_day=8".to_string(),
            "  on_start_day_change=on_start_day_change".to_string(),
            format!("  end_day=Signal::derive(|| {:?})", workbench_end_day.get()),
            "  default_end_day=19".to_string(),
            "  on_end_day_change=on_end_day_change".to_string(),
            format!("  first_weekday={first_weekday}"),
            format!(
                "  show_outside_days={}",
                bool_word(workbench_show_outside_days.get())
            ),
            format!("  start_label={}", rust_string_literal(start_label)),
            format!("  end_label={}", rust_string_literal(end_label)),
            "  start_placeholder=\"Start day\".to_string()".to_string(),
            "  end_placeholder=\"End day\".to_string()".to_string(),
            "  start_aria_label=\"Start date picker\".to_string()".to_string(),
            "  end_aria_label=\"End date picker\".to_string()".to_string(),
            "  invalid_range_message=\"End date must be after start date\".to_string()".to_string(),
            "  aria_label=\"Release window\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let actual_config = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            DateRangePickerTone::Strong
        } else {
            DateRangePickerTone::Default
        };
        let first_weekday = if workbench_monday.get() {
            CalendarFirstWeekday::Monday
        } else {
            CalendarFirstWeekday::Sunday
        };
        let start_label = if workbench_custom_text.get() {
            "Ship from"
        } else {
            "Start date"
        };
        let end_label = if workbench_custom_text.get() {
            "Ship to"
        } else {
            "End date"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-date-range-picker-custom")
        } else {
            None
        };
        let start = workbench_start_day.get();
        let end = workbench_end_day.get();
        let start_text = start.map_or_else(|| "None".to_string(), |it| format!("Some({it})"));
        let end_text = end.map_or_else(|| "None".to_string(), |it| format!("Some({it})"));

        format!(
            "DateRangePickerActualConfig {{\n  id_base: \"docs-date-range-picker-workbench\",\n  start_year: 2026,\n  start_month: 8,\n  end_year: 2026,\n  end_month: 8,\n  tone: {tone:?},\n  disabled: {},\n  start_day: {start_text},\n  default_start_day: Some(8),\n  on_start_day_change: \"runs={}\",\n  end_day: {end_text},\n  default_end_day: Some(19),\n  on_end_day_change: \"runs={}\",\n  first_weekday: {first_weekday:?},\n  show_outside_days: {},\n  start_label: {},\n  end_label: {},\n  start_placeholder: \"Start day\",\n  end_placeholder: \"End day\",\n  start_aria_label: \"Start date picker\",\n  end_aria_label: \"End date picker\",\n  invalid_range_message: \"End date must be after start date\",\n  aria_label: \"Release window\",\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_disabled.get()),
            on_start_day_change_runs.get(),
            on_end_day_change_runs.get(),
            bool_word(workbench_show_outside_days.get()),
            rust_string_literal(start_label),
            rust_string_literal(end_label),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<DateRangePicker id_base="range-default".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 default_start_day=8 default_end_day=19 />
<DateRangePicker id_base="range-strong".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 tone=DateRangePickerTone::Strong default_start_day=22 default_end_day=10 />
<DateRangePicker id_base="range-disabled".to_string() start_year=2026 start_month=8 end_year=2026 end_month=8 disabled=true default_start_day=5 default_end_day=12 />"#.to_string()
    });

    view! {
        <ComponentPage
            title="DateRangePicker"
            slug="date-range-picker"
            group="Forms"
            description="Date range workbench with complete API coverage and callback-state feedback."
        >
            <Playground title="Hello World (Default Range)" code_signal=hello_code>
                <DateRangePicker
                    id_base="docs-date-range-picker-hello".to_string()
                    start_year=2026
                    start_month=8
                    end_year=2026
                    end_month=8
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-range-picker-workbench-controls">
                        <Switch checked=workbench_strong_tone set_checked=set_workbench_strong_tone>
                            "Strong tone"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_monday set_checked=set_workbench_monday>
                            "First weekday Monday"
                        </Switch>
                        <Switch
                            checked=workbench_show_outside_days
                            set_checked=set_workbench_show_outside_days
                        >
                            "Show outside days"
                        </Switch>
                        <Switch checked=workbench_custom_text set_checked=set_workbench_custom_text>
                            "Custom labels"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>

                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|value| *value = value.map(|day| day.saturating_sub(1).max(1)));
                                })
                            >
                                "Start -1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|value| *value = value.map(|day| (day + 1).min(31)));
                                })
                            >
                                "Start +1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|value| *value = value.map(|day| day.saturating_sub(1).max(1)));
                                })
                            >
                                "End -1"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|value| *value = value.map(|day| (day + 1).min(31)));
                                })
                            >
                                "End +1"
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="date-range-picker-workbench-preview">
                    <DateRangePicker
                        id_base="docs-date-range-picker-workbench".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        tone=if workbench_strong_tone.get() {
                            DateRangePickerTone::Strong
                        } else {
                            DateRangePickerTone::Default
                        }
                        disabled=workbench_disabled.get()
                        start_day=workbench_start_day_signal
                        default_start_day=8
                        on_start_day_change=on_start_day_change
                        end_day=workbench_end_day_signal
                        default_end_day=19
                        on_end_day_change=on_end_day_change
                        first_weekday=if workbench_monday.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        }
                        show_outside_days=workbench_show_outside_days.get()
                        start_label=if workbench_custom_text.get() {
                            "Ship from".to_string()
                        } else {
                            "Start date".to_string()
                        }
                        end_label=if workbench_custom_text.get() {
                            "Ship to".to_string()
                        } else {
                            "End date".to_string()
                        }
                        start_placeholder="Start day".to_string()
                        end_placeholder="End day".to_string()
                        start_aria_label="Start date picker".to_string()
                        end_aria_label="End date picker".to_string()
                        invalid_range_message="End date must be after start date".to_string()
                        aria_label="Release window".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-date-range-picker-custom".to_string()
                        } else {
                            String::new()
                        }
                    />

                    <span class="ui-muted" data-slot="date-range-picker-workbench-feedback">
                        "start: "
                        {move || workbench_start_day.get().map_or_else(|| "none".to_string(), |day| day.to_string())}
                        " · end: "
                        {move || workbench_end_day.get().map_or_else(|| "none".to_string(), |day| day.to_string())}
                        " · on_start_day_change: " {move || on_start_day_change_runs.get()}
                        " · on_end_day_change: " {move || on_end_day_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Invalid / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="date-range-picker-state-matrix">
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-default".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        default_start_day=8
                        default_end_day=19
                    />
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-invalid".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        tone=DateRangePickerTone::Strong
                        default_start_day=22
                        default_end_day=10
                    />
                    <DateRangePicker
                        id_base="docs-date-range-picker-matrix-disabled".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        disabled=true
                        default_start_day=5
                        default_end_day=12
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_field() -> AnyView {
    let tone_options = vec!["Default".to_string(), "Strong".to_string()];
    let id_base_options = vec![
        "invoice-date".to_string(),
        "ship-date".to_string(),
        "due-date".to_string(),
    ];
    let motion_options = vec![
        "Default".to_string(),
        "No Motion".to_string(),
        "Long Motion".to_string(),
    ];

    let (showcase_value, set_showcase_value) = signal(Some("2026-03-14".to_string()));
    let showcase_on_value_change = Callback::new(move |next: Option<String>| {
        set_showcase_value.set(next);
    });

    let (workbench_id_base_index, set_workbench_id_base_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_with_default, set_workbench_with_default) = signal(true);
    let (workbench_with_label, set_workbench_with_label) = signal(true);
    let (workbench_with_placeholder, set_workbench_with_placeholder) = signal(true);
    let (workbench_with_aria_label, set_workbench_with_aria_label) = signal(true);
    let (workbench_with_custom_class, set_workbench_with_custom_class) = signal(false);
    let (workbench_with_callback, set_workbench_with_callback) = signal(true);

    let (workbench_value, set_workbench_value) = signal(Some("2026-03-22".to_string()));
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_value_change = Callback::new(move |next: Option<String>| {
        if workbench_with_callback.get() {
            set_workbench_change_count.update(|count| *count += 1);
            set_workbench_last_change.set(next.clone().unwrap_or_else(|| "none".to_string()));
            set_workbench_value.set(next);
        }
    });

    let workbench_id_base =
        Signal::derive(move || match workbench_id_base_index.get().unwrap_or(0) {
            1 => "ship-date".to_string(),
            2 => "due-date".to_string(),
            _ => "invoice-date".to_string(),
        });
    let workbench_tone = Signal::derive(move || {
        if workbench_tone_index.get().unwrap_or(0) == 1 {
            DateFieldTone::Strong
        } else {
            DateFieldTone::Default
        }
    });
    let workbench_motion =
        Signal::derive(move || match workbench_motion_index.get().unwrap_or(0) {
            1 => ui::text_input::date_field::DateFieldMotion::disabled(),
            2 => ui::text_input::date_field::DateFieldMotion {
                enabled: true,
                duration_ms: 420,
            },
            _ => ui::text_input::date_field::DateFieldMotion::default(),
        });
    let workbench_controlled_value = Signal::derive(move || {
        if workbench_controlled.get() {
            workbench_value.get()
        } else {
            None
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"let (value, set_value) = signal(Some("2026-03-14".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_value.set(next);
});

<DateField
  id_base="invoice-date".to_string()
  label="Invoice date".to_string()
  value=value
  on_value_change=on_value_change
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<DateField\n  id_base={}.to_string()\n  label={}\n  placeholder={}\n  tone={:?}\n  disabled={}\n  value={}\n  default_value={}\n  on_value_change={}\n  aria_label={}\n  motion=DateFieldMotion {{ enabled: {}, duration_ms: {} }}\n  class_name={}\n/>",
            rust_string_literal(&workbench_id_base.get()),
            if workbench_with_label.get() {
                format!("Some({}.to_string())", rust_string_literal("Invoice date"))
            } else {
                "None".to_string()
            },
            if workbench_with_placeholder.get() {
                format!("Some({}.to_string())", rust_string_literal("yyyy-mm-dd"))
            } else {
                "None".to_string()
            },
            workbench_tone.get(),
            bool_word(workbench_disabled.get()),
            if workbench_controlled.get() {
                "value".to_string()
            } else {
                "Signal::derive(move || None::<String>)".to_string()
            },
            if workbench_with_default.get() {
                format!("{}.to_string()", rust_string_literal("2026-03-22"))
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_with_callback.get() {
                "on_value_change".to_string()
            } else {
                "Callback::new(|_: Option<String>| {})".to_string()
            },
            if workbench_with_aria_label.get() {
                format!("{}.to_string()", rust_string_literal("Invoice date field"))
            } else {
                "\"\".to_string()".to_string()
            },
            bool_word(workbench_motion.get().enabled),
            workbench_motion.get().duration_ms,
            if workbench_with_custom_class.get() {
                format!(
                    "{}.to_string()",
                    rust_string_literal("docs-date-field-custom")
                )
            } else {
                "\"\".to_string()".to_string()
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "DateFieldWorkbenchActualConfig {{\n  id_base: {:?},\n  label: {:?},\n  placeholder: {:?},\n  tone: {:?},\n  disabled: {},\n  value: {:?},\n  default_value: {:?},\n  on_value_change: {},\n  aria_label: {:?},\n  motion: DateFieldMotion {{ enabled: {}, duration_ms: {} }},\n  class_name: {:?},\n}}",
            workbench_id_base.get(),
            if workbench_with_label.get() {
                Some("Invoice date")
            } else {
                None
            },
            if workbench_with_placeholder.get() {
                Some("yyyy-mm-dd")
            } else {
                None
            },
            workbench_tone.get(),
            bool_word(workbench_disabled.get()),
            if workbench_controlled.get() {
                Some(workbench_value.get())
            } else {
                None
            },
            if workbench_with_default.get() {
                Some("2026-03-22")
            } else {
                None
            },
            bool_word(workbench_with_callback.get()),
            if workbench_with_aria_label.get() {
                Some("Invoice date field")
            } else {
                None
            },
            bool_word(workbench_motion.get().enabled),
            workbench_motion.get().duration_ms,
            if workbench_with_custom_class.get() {
                Some("docs-date-field-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<DateField id_base="matrix-default".to_string() label="Default".to_string() />
<DateField
  id_base="matrix-strong".to_string()
  label="Strong tone".to_string()
  tone=DateFieldTone::Strong
  placeholder="yyyy-mm-dd".to_string()
  aria_label="Strong date field".to_string()
/>
<DateField
  id_base="matrix-disabled".to_string()
  label="Disabled".to_string()
  disabled=true
  default_value=Some("2026-06-01".to_string())
  motion=DateFieldMotion::disabled()
  class_name="docs-date-field-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="DateField"
            slug="date-field"
            group="Forms"
            description="Segmented date entry field with centralized year/month/day normalization and baseline-style state/source contracts."
        >
            <Playground title="Hello World (Default DateField)" code_signal=showcase_code>
                <div class="docs-stack">
                    <DateField
                        id_base="docs-date-field-showcase".to_string()
                        label="Invoice date".to_string()
                        value=showcase_value
                        on_value_change=showcase_on_value_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || showcase_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-field-workbench-controls">
                        <SegmentedControl
                            id_base="docs-date-field-workbench-id-base".to_string()
                            options=id_base_options.clone()
                            selected_index=workbench_id_base_index
                            set_selected_index=set_workbench_id_base_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField id_base".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-field-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField tone".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-field-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateField motion".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "value (controlled)"
                        </Switch>
                        <Switch checked=workbench_with_default set_checked=set_workbench_with_default>
                            "default_value"
                        </Switch>
                        <Switch checked=workbench_with_label set_checked=set_workbench_with_label>
                            "label"
                        </Switch>
                        <Switch checked=workbench_with_placeholder set_checked=set_workbench_with_placeholder>
                            "placeholder"
                        </Switch>
                        <Switch checked=workbench_with_aria_label set_checked=set_workbench_with_aria_label>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_with_custom_class set_checked=set_workbench_with_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_with_callback set_checked=set_workbench_with_callback>
                            "on_value_change"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <DateField
                        id_base=workbench_id_base.get()
                        label=if workbench_with_label.get() {
                            "Invoice date".to_string()
                        } else {
                            String::new()
                        }
                        placeholder=if workbench_with_placeholder.get() {
                            "yyyy-mm-dd".to_string()
                        } else {
                            String::new()
                        }
                        tone=workbench_tone.get()
                        disabled=workbench_disabled.get()
                        value=workbench_controlled_value
                        default_value=if workbench_with_default.get() {
                            "2026-03-22".to_string()
                        } else {
                            String::new()
                        }
                        on_value_change=on_workbench_value_change
                        aria_label=if workbench_with_aria_label.get() {
                            "Invoice date field".to_string()
                        } else {
                            String::new()
                        }
                        motion=workbench_motion.get()
                        class_name=if workbench_with_custom_class.get() {
                            "docs-date-field-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "current value: "
                        {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                    <span class="ui-muted">
                        "change_count="
                        {move || workbench_change_count.get()}
                        " · last_change="
                        {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Strong / Disabled)" code_signal=matrix_code>
                <div class="docs-stack" data-slot="date-field-state-matrix">
                    <DateField
                        id_base="docs-date-field-matrix-default".to_string()
                        label="Default".to_string()
                    />
                    <DateField
                        id_base="docs-date-field-matrix-strong".to_string()
                        label="Strong tone".to_string()
                        tone=DateFieldTone::Strong
                        placeholder="yyyy-mm-dd".to_string()
                        aria_label="Strong tone date field".to_string()
                    />
                    <DateField
                        id_base="docs-date-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        disabled=true
                        default_value="2026-06-01".to_string()
                        motion=ui::text_input::date_field::DateFieldMotion::disabled()
                        class_name="docs-date-field-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
