pub type FormFieldTone = ui_form_field::FormFieldTone;
pub type FormFieldIndicatorVariant = ui_form_field::FormFieldIndicatorVariant;
pub type FormFieldIndicatorPlacement = ui_form_field::FormFieldIndicatorPlacement;
pub type A11yDirection = ui_headless::A11yDirection;

pub const FORM_FIELD_AGENT_SCHEMA: &str;
pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str;

pub enum FormFieldAgentIntent {
    SelectionControl,
}

pub enum FormFieldAgentAction {
    RenderSnapshot,
}

pub enum FormFieldAgentStateAxis {
    Unselected,
    Selected,
    Disabled,
    Invalid,
    SelectedDisabled,
    SelectedInvalid,
    InvalidDisabled,
}

pub enum FormFieldAgentSourceAxis {
    Controlled,
    Uncontrolled,
}

pub enum FormFieldAgentStreamSupport {
    Optional,
}

pub enum FormFieldAgentStreamFallback {
    Snapshot,
}

pub enum FormFieldAgentOutputStatus {
    Verified,
}

pub struct FormFieldAgentContractAttrs {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn FormField(
    is_selected: Option<leptos::prelude::Signal<bool>>,
    default_selected: Option<bool>,
    on_selected_change: Option<leptos::prelude::Callback<bool>>,
    is_disabled: bool,
    is_invalid: bool,
    tone: FormFieldTone,
    indicator_variant: FormFieldIndicatorVariant,
    indicator_placement: FormFieldIndicatorPlacement,
    id_base: Option<String>,
    label: Option<String>,
    description: Option<String>,
    error_message: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
