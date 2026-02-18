use leptos::prelude::*;

pub use ui_state_primitives::text_area::{
    TextAreaState, TextAreaStateInput, normalize_optional_text, resolve_label_with_fallback,
    resolve_state,
};

pub struct ValueAxisInput {
    pub value: Option<Signal<String>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<String>>,
    pub set_value: Option<WriteSignal<String>>,
}

pub struct ValueAxisState {
    pub value: Option<Signal<String>>,
    pub default_value: String,
    pub on_value_change: Option<Callback<String>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<String>>,
    set_value: Option<WriteSignal<String>>,
) -> Option<Callback<String>> {
    on_value_change.or_else(|| {
        set_value.map(|set_value| Callback::new(move |next: String| set_value.set(next)))
    })
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let is_controlled = input.value.is_some();
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let has_legacy_set_value = input.set_value.is_some();
    let default_value = normalize_default_value(input.default_value);
    let on_value_change = normalize_on_value_change_handler(input.on_value_change, input.set_value);

    let control_mode_attr = if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    };
    let default_value_source_attr = if has_default_value {
        "custom"
    } else {
        "default"
    };
    let value_change_source_attr = if has_on_value_change {
        "on_value_change"
    } else if has_legacy_set_value {
        "set_value"
    } else {
        "none"
    };
    let has_value_change_handler = has_on_value_change || has_legacy_set_value;

    ValueAxisState {
        value: input.value,
        default_value,
        on_value_change,
        is_controlled,
        control_mode_attr,
        default_value_source_attr,
        value_change_source_attr,
        has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_read_only: Option<bool>,
    pub read_only: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let is_required = input
        .is_required
        .or(input.required)
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_invalid = input
        .is_invalid
        .or(input.invalid)
        .unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        is_read_only: input.is_read_only.unwrap_or(input.read_only),
        is_required,
        is_invalid,
    }
}

pub struct ResolvedTextAreaPropsInput {
    pub label: String,
    pub fallback_label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub class_name: Option<String>,
}

pub struct ResolvedTextAreaProps {
    pub label: String,
    pub has_custom_label: bool,
    pub description: Option<String>,
    pub has_custom_description: bool,
    pub error: Option<String>,
    pub has_custom_error: bool,
    pub placeholder: Option<String>,
    pub has_custom_placeholder: bool,
    pub rows: Option<u32>,
    pub has_custom_rows: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
}

pub fn resolve_props(input: ResolvedTextAreaPropsInput) -> ResolvedTextAreaProps {
    let (label, has_custom_label) = resolve_label_with_fallback(input.label, &input.fallback_label);
    let description = normalize_optional_text(input.description);
    let has_custom_description = description.is_some();
    let error = normalize_optional_text(input.error);
    let has_custom_error = error.is_some();
    let placeholder = normalize_optional_text(input.placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let rows = input.rows.filter(|rows| *rows > 0);
    let has_custom_rows = rows.is_some();
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    ResolvedTextAreaProps {
        label,
        has_custom_label,
        description,
        has_custom_description,
        error,
        has_custom_error,
        placeholder,
        has_custom_placeholder,
        rows,
        has_custom_rows,
        class_name,
        has_custom_class_name,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: TextAreaState) -> String {
    let mut classes = vec![
        "ui-text-area".to_string(),
        format!("ui-text-area--state-{}", state.state_attr),
        format!("ui-text-area--value-{}", state.value_attr),
        format!("ui-text-area--requirement-{}", state.requirement_attr),
    ];

    if state.has_custom_class_name {
        classes.push("ui-text-area--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
        let (preferred_required, _set_preferred_required) = signal(true);
        let (legacy_required, _set_legacy_required) = signal(false);
        let (preferred_invalid, _set_preferred_invalid) = signal(true);
        let (legacy_invalid, _set_legacy_invalid) = signal(false);

        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(true),
            disabled: false,
            is_read_only: Some(true),
            read_only: false,
            is_required: Some(preferred_required.into()),
            required: Some(legacy_required.into()),
            is_invalid: Some(preferred_invalid.into()),
            invalid: Some(legacy_invalid.into()),
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
        assert!(state.is_required.get_untracked());
        assert!(state.is_invalid.get_untracked());
    }

    #[test]
    fn normalize_accessibility_state_falls_back_to_legacy_aliases() {
        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: None,
            disabled: true,
            is_read_only: None,
            read_only: true,
            is_required: None,
            required: None,
            is_invalid: None,
            invalid: None,
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
        assert!(!state.is_required.get_untracked());
        assert!(!state.is_invalid.get_untracked());
    }

    #[test]
    fn normalize_default_value_uses_empty_string_when_absent() {
        assert_eq!(normalize_default_value(None), String::new());
        assert_eq!(
            normalize_default_value(Some("prefilled".to_string())),
            "prefilled".to_string()
        );
    }

    #[test]
    fn normalize_on_value_change_handler_prefers_on_value_change() {
        let (from_on_value_change, set_from_on_value_change) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change = Callback::new(move |next: String| set_from_on_value_change.set(next));

        let handler =
            normalize_on_value_change_handler(Some(on_value_change), Some(set_legacy_value))
                .expect("handler should exist");

        handler.run("new-value".to_string());

        assert_eq!(
            from_on_value_change.get_untracked(),
            "new-value",
            "on_value_change should have priority over legacy set_value"
        );
    }

    #[test]
    fn normalize_on_value_change_handler_falls_back_to_set_value_alias() {
        let (legacy_value, set_legacy_value) = signal(String::new());

        let handler = normalize_on_value_change_handler(None, Some(set_legacy_value))
            .expect("legacy set_value should be normalized to on_value_change callback");

        handler.run("legacy-updated".to_string());

        assert_eq!(legacy_value.get_untracked(), "legacy-updated");
    }

    #[test]
    fn normalize_value_axis_centralizes_default_priority_and_sources() {
        let (value, _set_value) = signal("controlled".to_string());
        let state = normalize_value_axis(ValueAxisInput {
            value: Some(value.into()),
            default_value: Some("default".to_string()),
            on_value_change: None,
            set_value: None,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.default_value, "default");
        assert_eq!(state.default_value_source_attr, "custom");
        assert_eq!(state.value_change_source_attr, "none");
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_prefers_on_value_change_over_set_value_alias() {
        let (on_value_change_value, set_on_value_change_value) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change =
            Callback::new(move |next: String| set_on_value_change_value.set(next));
        let state = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: None,
            on_value_change: Some(on_value_change),
            set_value: Some(set_legacy_value),
        });

        let handler = state
            .on_value_change
            .expect("value axis should keep normalized callback");
        handler.run("prioritized".to_string());

        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.default_value_source_attr, "default");
        assert_eq!(state.value_change_source_attr, "on_value_change");
        assert!(state.has_value_change_handler);
        assert_eq!(on_value_change_value.get_untracked(), "prioritized");
    }

    #[test]
    fn resolve_props_uses_fallback_and_normalizes_optional_inputs() {
        let resolved = resolve_props(ResolvedTextAreaPropsInput {
            label: "  ".to_string(),
            fallback_label: "Localized Text area".to_string(),
            description: Some("  desc  ".to_string()),
            error: Some("  ".to_string()),
            placeholder: Some("  hint  ".to_string()),
            rows: Some(0),
            class_name: Some("  docs-text-area  ".to_string()),
        });

        assert_eq!(resolved.label, "Localized Text area");
        assert!(!resolved.has_custom_label);
        assert_eq!(resolved.description.as_deref(), Some("desc"));
        assert!(resolved.has_custom_description);
        assert_eq!(resolved.error, None);
        assert!(!resolved.has_custom_error);
        assert_eq!(resolved.placeholder.as_deref(), Some("hint"));
        assert!(resolved.has_custom_placeholder);
        assert_eq!(resolved.rows, None);
        assert!(!resolved.has_custom_rows);
        assert_eq!(resolved.class_name.as_deref(), Some("docs-text-area"));
        assert!(resolved.has_custom_class_name);
    }

    #[test]
    fn resolve_label_with_fallback_uses_default_for_blank_values() {
        assert_eq!(
            resolve_label_with_fallback(
                "  ".to_string(),
                ui_state_primitives::text_area::DEFAULT_LABEL
            ),
            (
                ui_state_primitives::text_area::DEFAULT_LABEL.to_string(),
                false
            )
        );
        assert_eq!(
            resolve_label_with_fallback(
                "  Team notes  ".to_string(),
                ui_state_primitives::text_area::DEFAULT_LABEL,
            ),
            ("Team notes".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_rows_markers() {
        let state = resolve_state(TextAreaStateInput {
            disabled: false,
            read_only: true,
            required: true,
            invalid: false,
            has_value: true,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_placeholder: true,
            has_custom_rows: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.state_attr, "readonly");
        assert_eq!(state.value_attr, "filled");
        assert_eq!(state.requirement_attr, "required");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.placeholder_source_attr, "custom");
        assert_eq!(state.rows_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(TextAreaStateInput {
            disabled: true,
            read_only: false,
            required: false,
            invalid: false,
            has_value: false,
            has_custom_label: false,
            has_custom_description: false,
            has_custom_error: false,
            has_custom_placeholder: false,
            has_custom_rows: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-text-area".to_string()), state);

        for token in [
            "ui-text-area",
            "ui-text-area--state-disabled",
            "ui-text-area--value-empty",
            "ui-text-area--requirement-optional",
            "ui-text-area--custom-class",
            "docs-text-area",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn default_label_is_sourced_from_state_primitives() {
        assert_eq!(ui_state_primitives::text_area::DEFAULT_LABEL, "Text area");
    }
}
