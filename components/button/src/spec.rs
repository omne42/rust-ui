use crate::button::{
    Button, ButtonColor, ButtonLoadingPlacement, ButtonMotion, ButtonRadius, ButtonSize,
    ButtonVariant,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use ui_headless::OnPress;

pub const BUTTON_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonIntent {
    #[default]
    Primary,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonIntent::Primary => "primary",
            ButtonIntent::Accent => "accent",
            ButtonIntent::Destructive => "destructive",
            ButtonIntent::Outline => "outline",
            ButtonIntent::Secondary => "secondary",
            ButtonIntent::Ghost => "ghost",
            ButtonIntent::Link => "link",
        }
    }

    pub fn into_variant(self) -> ButtonVariant {
        match self {
            ButtonIntent::Primary => ButtonVariant::Default,
            ButtonIntent::Accent => ButtonVariant::Accent,
            ButtonIntent::Destructive => ButtonVariant::Destructive,
            ButtonIntent::Outline => ButtonVariant::Outline,
            ButtonIntent::Secondary => ButtonVariant::Secondary,
            ButtonIntent::Ghost => ButtonVariant::Ghost,
            ButtonIntent::Link => ButtonVariant::Link,
        }
    }
}

#[derive(Clone)]
pub enum ButtonText {
    Static(String),
    Dynamic(Signal<String>),
}

impl Default for ButtonText {
    fn default() -> Self {
        ButtonText::Static("Button".to_string())
    }
}

impl ButtonText {
    pub fn static_text(value: impl Into<String>) -> Self {
        ButtonText::Static(value.into())
    }

    pub fn dynamic(value: Signal<String>) -> Self {
        ButtonText::Dynamic(value)
    }

    fn resolve(&self) -> String {
        match self {
            ButtonText::Static(value) => value.clone(),
            ButtonText::Dynamic(value) => value.get(),
        }
    }
}

#[derive(Clone, Default)]
pub struct ButtonA11y {
    pub label: Option<String>,
}

impl ButtonA11y {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
}

#[derive(Default)]
pub enum ButtonAction {
    #[default]
    None,
    Callback(OnPress),
}

impl ButtonAction {
    pub fn callback(handler: OnPress) -> Self {
        ButtonAction::Callback(handler)
    }

    pub fn increment_u32(set_count: WriteSignal<u32>) -> Self {
        ButtonAction::Callback(Callback::new(move |_| {
            set_count.update(|value| *value += 1);
        }))
    }

    pub fn increment_i32(set_count: WriteSignal<i32>) -> Self {
        ButtonAction::Callback(Callback::new(move |_| {
            set_count.update(|value| *value += 1);
        }))
    }

    pub fn toggle_bool(set_value: WriteSignal<bool>) -> Self {
        ButtonAction::Callback(Callback::new(move |_| {
            set_value.update(|value| *value = !*value);
        }))
    }

    fn into_on_press(self) -> OnPress {
        match self {
            ButtonAction::None => Callback::new(move |_| {}),
            ButtonAction::Callback(handler) => handler,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ButtonSchema {
    pub schema_version: u16,
    pub element_id: String,
    pub intent: ButtonIntent,
    pub action_signature: String,
    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize)]
struct ButtonSchemaPayload {
    schema_version: u16,
    element_id: String,
    intent: ButtonIntent,
    action_signature: String,
    #[serde(default)]
    requires_confirmation: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSchemaErrorKind {
    Serialize,
    Deserialize,
    UnsupportedVersion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonSchemaError {
    pub kind: ButtonSchemaErrorKind,
    pub code: &'static str,
    pub message: String,
    pub schema_version: Option<u16>,
    pub supported_schema_version: u16,
}

impl ButtonSchemaError {
    fn serialize(message: String, schema_version: Option<u16>) -> Self {
        Self {
            kind: ButtonSchemaErrorKind::Serialize,
            code: "button_schema_serialize_failed",
            message,
            schema_version,
            supported_schema_version: BUTTON_SCHEMA_VERSION,
        }
    }

    fn deserialize(message: String) -> Self {
        Self {
            kind: ButtonSchemaErrorKind::Deserialize,
            code: "button_schema_deserialize_failed",
            message,
            schema_version: None,
            supported_schema_version: BUTTON_SCHEMA_VERSION,
        }
    }

    fn unsupported_version(found: u16) -> Self {
        Self {
            kind: ButtonSchemaErrorKind::UnsupportedVersion,
            code: "button_schema_unsupported_version",
            message: format!(
                "Unsupported button schema_version={found}, max_supported={BUTTON_SCHEMA_VERSION}"
            ),
            schema_version: Some(found),
            supported_schema_version: BUTTON_SCHEMA_VERSION,
        }
    }
}

impl std::fmt::Display for ButtonSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} (schema_version={:?}, supported={})",
            self.code, self.message, self.schema_version, self.supported_schema_version
        )
    }
}

impl std::error::Error for ButtonSchemaError {}

#[cfg(feature = "button-wasm-debug")]
const BUTTON_SPEC_TRACE_TARGET: &str = "ui::button::spec";

#[cfg(feature = "button-wasm-debug")]
fn trace_button_spec_event(
    event: &'static str,
    schema_version: Option<u16>,
    status: &'static str,
    error_code: Option<&'static str>,
) {
    tracing::event!(
        target: BUTTON_SPEC_TRACE_TARGET,
        tracing::Level::DEBUG,
        event,
        schema_version = schema_version.unwrap_or(BUTTON_SCHEMA_VERSION),
        status,
        error_code = error_code.unwrap_or("none"),
        "button spec transition"
    );
}

impl ButtonSchema {
    pub fn new(
        element_id: impl Into<String>,
        intent: ButtonIntent,
        action_signature: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: BUTTON_SCHEMA_VERSION,
            element_id: element_id.into(),
            intent,
            action_signature: action_signature.into(),
            requires_confirmation: false,
        }
    }

    pub fn schema_version(mut self, value: u16) -> Self {
        self.schema_version = value;
        self
    }

    pub fn requires_confirmation(mut self, value: bool) -> Self {
        self.requires_confirmation = value;
        self
    }

    pub fn to_json(&self) -> String {
        match self.to_json_result() {
            Ok(json) => json,
            Err(error) => {
                #[cfg(feature = "button-wasm-debug")]
                trace_button_spec_event(
                    "button.schema.serialize",
                    error.schema_version,
                    "error",
                    Some(error.code),
                );
                format!(
                    "{{\"code\":\"{}\",\"message\":\"{}\",\"schema_version\":{},\"supported_schema_version\":{}}}",
                    error.code,
                    escape_json(&error.message),
                    error
                        .schema_version
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "null".to_string()),
                    error.supported_schema_version
                )
            }
        }
    }

    pub fn to_json_result(&self) -> Result<String, ButtonSchemaError> {
        let schema_version = Some(self.schema_version);
        match serde_json::to_string(self) {
            Ok(json) => {
                #[cfg(feature = "button-wasm-debug")]
                trace_button_spec_event("button.schema.serialize", schema_version, "ok", None);
                Ok(json)
            }
            Err(error) => {
                let structured = ButtonSchemaError::serialize(error.to_string(), schema_version);
                #[cfg(feature = "button-wasm-debug")]
                trace_button_spec_event(
                    "button.schema.serialize",
                    schema_version,
                    "error",
                    Some(structured.code),
                );
                Err(structured)
            }
        }
    }

    pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError> {
        let payload: ButtonSchemaPayload = serde_json::from_str(raw)
            .map_err(|error| ButtonSchemaError::deserialize(error.to_string()))?;
        let schema_version = payload.schema_version;
        if schema_version != BUTTON_SCHEMA_VERSION {
            let error = ButtonSchemaError::unsupported_version(schema_version);
            #[cfg(feature = "button-wasm-debug")]
            trace_button_spec_event(
                "button.schema.deserialize",
                Some(schema_version),
                "error",
                Some(error.code),
            );
            return Err(error);
        }

        #[cfg(feature = "button-wasm-debug")]
        trace_button_spec_event(
            "button.schema.deserialize",
            Some(schema_version),
            "ok",
            None,
        );
        Ok(Self {
            schema_version,
            element_id: payload.element_id,
            intent: payload.intent,
            action_signature: payload.action_signature,
            requires_confirmation: payload.requires_confirmation,
        })
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub struct ButtonSpec {
    intent: ButtonIntent,
    color: ButtonColor,
    radius: ButtonRadius,
    size: ButtonSize,
    is_disabled: bool,
    is_loading: bool,
    loading_placement: ButtonLoadingPlacement,
    is_full_width: bool,
    motion: ButtonMotion,
    text: ButtonText,
    a11y: ButtonA11y,
    action: ButtonAction,
    schema: Option<ButtonSchema>,
}

impl Default for ButtonSpec {
    fn default() -> Self {
        Self {
            intent: ButtonIntent::Primary,
            color: ButtonColor::Primary,
            radius: ButtonRadius::Md,
            size: ButtonSize::M,
            is_disabled: false,
            is_loading: false,
            loading_placement: ButtonLoadingPlacement::Start,
            is_full_width: false,
            motion: ButtonMotion::default(),
            text: ButtonText::default(),
            a11y: ButtonA11y::default(),
            action: ButtonAction::None,
            schema: None,
        }
    }
}

impl ButtonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intent(mut self, value: ButtonIntent) -> Self {
        self.intent = value;
        self
    }

    pub fn variant(mut self, value: ButtonVariant) -> Self {
        self.intent = match value {
            ButtonVariant::Default => ButtonIntent::Primary,
            ButtonVariant::Solid => ButtonIntent::Primary,
            ButtonVariant::Faded => ButtonIntent::Accent,
            ButtonVariant::Bordered => ButtonIntent::Outline,
            ButtonVariant::Light => ButtonIntent::Ghost,
            ButtonVariant::Flat => ButtonIntent::Secondary,
            ButtonVariant::Shadow => ButtonIntent::Primary,
            ButtonVariant::Accent => ButtonIntent::Accent,
            ButtonVariant::Destructive => ButtonIntent::Destructive,
            ButtonVariant::Outline => ButtonIntent::Outline,
            ButtonVariant::Secondary => ButtonIntent::Secondary,
            ButtonVariant::Ghost => ButtonIntent::Ghost,
            ButtonVariant::Link => ButtonIntent::Link,
        };
        self
    }

    pub fn size(mut self, value: ButtonSize) -> Self {
        self.size = value;
        self
    }

    pub fn color(mut self, value: ButtonColor) -> Self {
        self.color = value;
        self
    }

    pub fn radius(mut self, value: ButtonRadius) -> Self {
        self.radius = value;
        self
    }

    pub fn is_disabled(mut self, value: bool) -> Self {
        self.is_disabled = value;
        self
    }

    pub fn is_loading(mut self, value: bool) -> Self {
        self.is_loading = value;
        self
    }

    pub fn loading_placement(mut self, value: ButtonLoadingPlacement) -> Self {
        self.loading_placement = value;
        self
    }

    pub fn is_full_width(mut self, value: bool) -> Self {
        self.is_full_width = value;
        self
    }

    pub fn motion(mut self, value: ButtonMotion) -> Self {
        self.motion = value;
        self
    }

    pub fn label(mut self, value: ButtonText) -> Self {
        self.text = value;
        self
    }

    pub fn accessibility(mut self, value: ButtonA11y) -> Self {
        self.a11y = value;
        self
    }

    pub fn on_action(mut self, value: ButtonAction) -> Self {
        self.action = value;
        self
    }

    pub fn schema(mut self, value: ButtonSchema) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn render(self) -> impl IntoView {
        let text = StoredValue::new(self.text);
        let schema_json = self.schema.map(|schema| schema.to_json());
        let on_press = self.action.into_on_press();
        let variant = self.intent.into_variant();

        match (self.a11y.label, schema_json) {
            (Some(aria_label), Some(schema_json)) => view! {
                <Button
                    is_disabled=self.is_disabled
                    is_loading=self.is_loading
                    variant=variant
                    color=self.color
                    radius=self.radius
                    size=self.size
                    is_full_width=self.is_full_width
                    motion=self.motion
                    loading_placement=self.loading_placement
                    schema_json=schema_json
                    aria_label=aria_label
                    on_press=on_press
                >
                    {move || text.get_value().resolve()}
                </Button>
            }
            .into_any(),
            (Some(aria_label), None) => view! {
                <Button
                    is_disabled=self.is_disabled
                    is_loading=self.is_loading
                    variant=variant
                    color=self.color
                    radius=self.radius
                    size=self.size
                    is_full_width=self.is_full_width
                    motion=self.motion
                    loading_placement=self.loading_placement
                    aria_label=aria_label
                    on_press=on_press
                >
                    {move || text.get_value().resolve()}
                </Button>
            }
            .into_any(),
            (None, Some(schema_json)) => view! {
                <Button
                    is_disabled=self.is_disabled
                    is_loading=self.is_loading
                    variant=variant
                    color=self.color
                    radius=self.radius
                    size=self.size
                    is_full_width=self.is_full_width
                    motion=self.motion
                    loading_placement=self.loading_placement
                    schema_json=schema_json
                    on_press=on_press
                >
                    {move || text.get_value().resolve()}
                </Button>
            }
            .into_any(),
            (None, None) => view! {
                <Button
                    is_disabled=self.is_disabled
                    is_loading=self.is_loading
                    variant=variant
                    color=self.color
                    radius=self.radius
                    size=self.size
                    is_full_width=self.is_full_width
                    motion=self.motion
                    loading_placement=self.loading_placement
                    on_press=on_press
                >
                    {move || text.get_value().resolve()}
                </Button>
            }
            .into_any(),
        }
    }
}

#[cfg(test)]
#[path = "../test/spec.rs"]
mod tests;
