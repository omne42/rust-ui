use leptos::prelude::*;

pub use ui_state_primitives::autocomplete::{
    AutocompleteState, AutocompleteStateInput, filter_indices, map_filtered_to_original,
    map_selected_to_filtered, normalize_disabled_indices, normalize_id_base, normalize_label,
    normalize_optional_text, resolve_empty_message, resolve_placeholder, resolve_state,
};

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub required: Signal<bool>,
    pub invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let required = input
        .is_required
        .or(input.required)
        .unwrap_or_else(|| Signal::derive(|| false));
    let invalid = input
        .is_invalid
        .or(input.invalid)
        .unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        required,
        invalid,
    }
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct OpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = input.is_open.or(input.open);
    OpenState {
        is_controlled: open.is_some(),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub struct RootStateInput {
    pub id_base: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub empty_message: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: Option<String>,
    pub item_count: usize,
    pub disabled_indices: Vec<usize>,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_motion: bool,
}

pub struct RootState {
    pub id_base: String,
    pub label: String,
    pub placeholder: String,
    pub empty_message: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: String,
    pub disabled_indices: Vec<usize>,
    pub state: AutocompleteState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RootDataState {
    Open,
    Disabled,
    Closed,
}

impl RootDataState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Disabled => "disabled",
            Self::Closed => "closed",
        }
    }
}

pub fn resolve_root_data_state(is_open: bool, is_disabled: bool) -> RootDataState {
    if is_open {
        RootDataState::Open
    } else if is_disabled {
        RootDataState::Disabled
    } else {
        RootDataState::Closed
    }
}

pub fn normalize_root_state(input: RootStateInput) -> RootState {
    let has_custom_id_base = normalize_optional_text(Some(input.id_base.clone())).is_some();
    let id_base = normalize_id_base(input.id_base);

    let has_custom_label = !input.label.trim().is_empty();
    let label = normalize_label(input.label);

    let has_custom_placeholder = normalize_optional_text(input.placeholder.clone()).is_some();
    let placeholder = resolve_placeholder(input.placeholder);
    let empty_message = resolve_empty_message(input.empty_message);

    let description = normalize_optional_text(input.description);
    let error = normalize_optional_text(input.error);
    let has_custom_description = description.is_some();
    let has_custom_error = error.is_some();

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);
    let disabled_option_count = disabled_indices.len();

    let state = resolve_state(AutocompleteStateInput {
        item_count: input.item_count,
        disabled_option_count,
        is_disabled: input.is_disabled,
        has_custom_label,
        has_custom_description,
        has_custom_error,
        has_custom_placeholder,
        has_custom_id_base,
        has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        is_controlled: input.is_controlled,
    });

    let class_name = compose_class_name(class_name, state);

    RootState {
        id_base,
        label,
        placeholder,
        empty_message,
        description,
        error,
        class_name,
        disabled_indices,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AutocompleteState) -> String {
    let mut classes = vec!["ui-autocomplete".to_string()];

    if state.is_disabled {
        classes.push("ui-autocomplete--disabled".to_string());
    }
    if state.is_empty {
        classes.push("ui-autocomplete--empty".to_string());
    }
    if state.has_description {
        classes.push("ui-autocomplete--has-description".to_string());
    }
    if state.has_error {
        classes.push("ui-autocomplete--has-error".to_string());
    }
    if state.has_disabled_options {
        classes.push("ui-autocomplete--has-disabled-options".to_string());
    }
    if state.is_controlled {
        classes.push("ui-autocomplete--controlled".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-autocomplete--custom-label".to_string());
    }
    if state.has_custom_description {
        classes.push("ui-autocomplete--custom-description".to_string());
    }
    if state.has_custom_error {
        classes.push("ui-autocomplete--custom-error".to_string());
    }
    if state.has_custom_placeholder {
        classes.push("ui-autocomplete--custom-placeholder".to_string());
    }
    if state.has_custom_id_base {
        classes.push("ui-autocomplete--custom-id".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-autocomplete--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-autocomplete--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
