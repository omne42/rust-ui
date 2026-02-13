use crate::button::{Button, ButtonLoadingPlacement, ButtonMotion, ButtonSize, ButtonVariant};
use leptos::prelude::*;
use ui_headless::OnPress;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonSchema {
    pub element_id: String,
    pub intent: ButtonIntent,
    pub action_signature: String,
    pub requires_confirmation: bool,
}

impl ButtonSchema {
    pub fn new(
        element_id: impl Into<String>,
        intent: ButtonIntent,
        action_signature: impl Into<String>,
    ) -> Self {
        Self {
            element_id: element_id.into(),
            intent,
            action_signature: action_signature.into(),
            requires_confirmation: false,
        }
    }

    pub fn requires_confirmation(mut self, value: bool) -> Self {
        self.requires_confirmation = value;
        self
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"element_id\":\"{}\",\"intent\":\"{}\",\"action_signature\":\"{}\",\"requires_confirmation\":{}}}",
            escape_json(&self.element_id),
            self.intent.as_str(),
            escape_json(&self.action_signature),
            self.requires_confirmation
        )
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
    size: ButtonSize,
    disabled: bool,
    is_loading: bool,
    loading_placement: ButtonLoadingPlacement,
    is_icon_only: bool,
    full_width: bool,
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
            size: ButtonSize::M,
            disabled: false,
            is_loading: false,
            loading_placement: ButtonLoadingPlacement::Start,
            is_icon_only: false,
            full_width: false,
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

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
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

    pub fn icon_only(mut self, value: bool) -> Self {
        self.is_icon_only = value;
        self
    }

    pub fn full_width(mut self, value: bool) -> Self {
        self.full_width = value;
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
                    disabled=self.disabled
                    is_loading=self.is_loading
                    variant=variant
                    size=self.size
                    is_icon_only=self.is_icon_only
                    full_width=self.full_width
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
                    disabled=self.disabled
                    is_loading=self.is_loading
                    variant=variant
                    size=self.size
                    is_icon_only=self.is_icon_only
                    full_width=self.full_width
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
                    disabled=self.disabled
                    is_loading=self.is_loading
                    variant=variant
                    size=self.size
                    is_icon_only=self.is_icon_only
                    full_width=self.full_width
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
                    disabled=self.disabled
                    is_loading=self.is_loading
                    variant=variant
                    size=self.size
                    is_icon_only=self.is_icon_only
                    full_width=self.full_width
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
mod tests {
    use super::*;

    #[test]
    fn intent_variant_mapping_is_stable() {
        assert_eq!(ButtonIntent::Primary.into_variant(), ButtonVariant::Default);
        assert_eq!(
            ButtonIntent::Destructive.into_variant(),
            ButtonVariant::Destructive
        );
        assert_eq!(ButtonIntent::Link.into_variant(), ButtonVariant::Link);
    }

    #[test]
    fn schema_json_is_machine_readable() {
        let json = ButtonSchema::new(
            "btn_del_01",
            ButtonIntent::Destructive,
            "delete_record(id: u32)",
        )
        .requires_confirmation(true)
        .to_json();

        assert!(json.contains("\"element_id\":\"btn_del_01\""));
        assert!(json.contains("\"intent\":\"destructive\""));
        assert!(json.contains("\"action_signature\":\"delete_record(id: u32)\""));
        assert!(json.contains("\"requires_confirmation\":true"));
    }

    #[test]
    fn button_text_resolves_static_and_dynamic_values() {
        let (value, set_value) = signal("Count: 1".to_string());
        let static_text = ButtonText::static_text("Save");
        assert_eq!(static_text.resolve(), "Save");

        let dynamic_text = ButtonText::dynamic(value.into());
        assert_eq!(dynamic_text.resolve(), "Count: 1");

        set_value.set("Count: 2".to_string());
        assert_eq!(dynamic_text.resolve(), "Count: 2");
    }
}
