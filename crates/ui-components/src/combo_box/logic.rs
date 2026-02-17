use leptos::prelude::*;

pub use ui_state_primitives::combo_box::{
    ComboBoxState, ComboBoxStateInput, filter_indices, map_filtered_to_original,
    map_selected_to_filtered, normalize_disabled_indices, normalize_id_base, normalize_label,
    normalize_optional_text, resolve_empty_message, resolve_placeholder, resolve_state,
    resolve_toggle_aria_label,
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
    pub toggle_button_aria_label: Option<String>,
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
    pub toggle_button_aria_label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub class_name: String,
    pub disabled_indices: Vec<usize>,
    pub state: ComboBoxState,
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
    let toggle_button_aria_label = resolve_toggle_aria_label(input.toggle_button_aria_label);

    let description = normalize_optional_text(input.description);
    let error = normalize_optional_text(input.error);
    let has_custom_description = description.is_some();
    let has_custom_error = error.is_some();

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);
    let disabled_option_count = disabled_indices.len();

    let state = resolve_state(ComboBoxStateInput {
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
        toggle_button_aria_label,
        description,
        error,
        class_name,
        disabled_indices,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ComboBoxState) -> String {
    let mut classes = vec!["ui-combo-box".to_string()];

    if state.is_disabled {
        classes.push("ui-combo-box--disabled".to_string());
    }
    if state.is_empty {
        classes.push("ui-combo-box--empty".to_string());
    }
    if state.has_description {
        classes.push("ui-combo-box--has-description".to_string());
    }
    if state.has_error {
        classes.push("ui-combo-box--has-error".to_string());
    }
    if state.has_disabled_options {
        classes.push("ui-combo-box--has-disabled-options".to_string());
    }
    if state.is_controlled {
        classes.push("ui-combo-box--controlled".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-combo-box--custom-label".to_string());
    }
    if state.has_custom_description {
        classes.push("ui-combo-box--custom-description".to_string());
    }
    if state.has_custom_error {
        classes.push("ui-combo-box--custom-error".to_string());
    }
    if state.has_custom_placeholder {
        classes.push("ui-combo-box--custom-placeholder".to_string());
    }
    if state.has_custom_id_base {
        classes.push("ui-combo-box--custom-id".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-combo-box--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-combo-box--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_primitives_are_reexported_from_ui_state_primitives() {
        assert_eq!(normalize_label("  Language  ".to_string()), "Language");
        assert_eq!(
            normalize_id_base("   ".to_string()),
            ui_state_primitives::combo_box::DEFAULT_ID_BASE
        );
        assert_eq!(
            resolve_placeholder(None),
            ui_state_primitives::combo_box::DEFAULT_PLACEHOLDER
        );
        assert_eq!(
            resolve_empty_message(None),
            ui_state_primitives::combo_box::DEFAULT_EMPTY_MESSAGE
        );
        assert_eq!(
            resolve_toggle_aria_label(None),
            ui_state_primitives::combo_box::DEFAULT_TOGGLE_ARIA_LABEL
        );
    }

    #[test]
    fn normalize_accessibility_state_applies_explicit_priority_and_defaults() {
        let (preferred_required, _set_preferred_required) = signal(true);
        let (legacy_required, _set_legacy_required) = signal(false);
        let (preferred_invalid, _set_preferred_invalid) = signal(true);
        let (legacy_invalid, _set_legacy_invalid) = signal(false);

        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(true),
            disabled: false,
            is_required: Some(preferred_required.into()),
            required: Some(legacy_required.into()),
            is_invalid: Some(preferred_invalid.into()),
            invalid: Some(legacy_invalid.into()),
        });

        assert!(state.is_disabled);
        assert!(state.required.get_untracked());
        assert!(state.invalid.get_untracked());

        let fallback = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: None,
            disabled: false,
            is_required: None,
            required: None,
            is_invalid: None,
            invalid: None,
        });
        assert!(!fallback.required.get_untracked());
        assert!(!fallback.invalid.get_untracked());
    }

    #[test]
    fn normalize_open_state_applies_explicit_priority_and_triplet_passthrough() {
        let (is_open_signal, _set_is_open_signal) = signal(true);
        let (legacy_open_signal, _set_legacy_open_signal) = signal(false);
        let on_open_change = Callback::new(|_: bool| {});

        let open_state = normalize_open_state(OpenStateInput {
            is_open: Some(is_open_signal.into()),
            open: Some(legacy_open_signal.into()),
            default_open: Some(false),
            on_open_change: Some(on_open_change),
        });

        assert!(open_state.is_controlled);
        assert!(
            open_state
                .open
                .expect("normalized open signal should exist")
                .get_untracked()
        );
        assert_eq!(open_state.default_open, Some(false));
        assert!(open_state.on_open_change.is_some());
    }

    #[test]
    fn normalize_root_state_centralizes_normalization_and_state_derivation() {
        let root = normalize_root_state(RootStateInput {
            id_base: "  ".to_string(),
            label: "  ".to_string(),
            placeholder: Some("  ".to_string()),
            empty_message: Some("  nothing  ".to_string()),
            toggle_button_aria_label: Some("  expand  ".to_string()),
            description: Some("  desc  ".to_string()),
            error: Some("  err  ".to_string()),
            class_name: Some("  custom  ".to_string()),
            item_count: 3,
            disabled_indices: vec![2, 2, 9],
            is_disabled: true,
            is_controlled: true,
            has_custom_motion: true,
        });

        assert_eq!(
            root.id_base,
            ui_state_primitives::combo_box::DEFAULT_ID_BASE
        );
        assert_eq!(root.label, ui_state_primitives::combo_box::DEFAULT_LABEL);
        assert_eq!(
            root.placeholder,
            ui_state_primitives::combo_box::DEFAULT_PLACEHOLDER
        );
        assert_eq!(root.empty_message, "nothing");
        assert_eq!(root.toggle_button_aria_label, "expand");
        assert_eq!(root.description.as_deref(), Some("desc"));
        assert_eq!(root.error.as_deref(), Some("err"));
        assert_eq!(root.disabled_indices, vec![2]);
        assert!(root.state.is_disabled);
        assert!(root.state.is_controlled);
        assert!(root.class_name.contains("ui-combo-box"));
    }

    #[test]
    fn resolve_root_data_state_uses_type_safe_exclusive_enum() {
        assert_eq!(resolve_root_data_state(true, true), RootDataState::Open);
        assert_eq!(
            resolve_root_data_state(false, true),
            RootDataState::Disabled
        );
        assert_eq!(resolve_root_data_state(false, false), RootDataState::Closed);
        assert_eq!(RootDataState::Open.as_attr(), "open");
        assert_eq!(RootDataState::Disabled.as_attr(), "disabled");
        assert_eq!(RootDataState::Closed.as_attr(), "closed");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ComboBoxStateInput {
                item_count: 0,
                disabled_option_count: 1,
                is_disabled: true,
                has_custom_label: true,
                has_custom_description: true,
                has_custom_error: true,
                has_custom_placeholder: true,
                has_custom_id_base: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                is_controlled: true,
            }),
        );

        for token in [
            "ui-combo-box",
            "ui-combo-box--disabled",
            "ui-combo-box--empty",
            "ui-combo-box--has-description",
            "ui-combo-box--has-error",
            "ui-combo-box--has-disabled-options",
            "ui-combo-box--controlled",
            "ui-combo-box--custom-label",
            "ui-combo-box--custom-description",
            "ui-combo-box--custom-error",
            "ui-combo-box--custom-placeholder",
            "ui-combo-box--custom-id",
            "ui-combo-box--custom-motion",
            "ui-combo-box--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
