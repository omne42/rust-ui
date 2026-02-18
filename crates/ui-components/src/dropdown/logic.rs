use leptos::prelude::*;

pub use ui_state_primitives::dropdown::{
    DropdownOpenFocusStrategy, DropdownState, DropdownStateInput, focus_strategy_for_open_key,
    normalize_aria_label, normalize_disabled_indices, normalize_id_base, normalize_optional_text,
    resolve_state, resolve_trigger_disabled,
};

pub struct DisabledStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_disabled_state(input: DisabledStateInput) -> bool {
    input.is_disabled.unwrap_or(input.disabled)
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

pub fn compose_class_name(base_class_name: Option<String>, state: DropdownState) -> String {
    let mut classes = vec!["ui-dropdown".to_string()];

    if state.is_disabled {
        classes.push("ui-dropdown--disabled".to_string());
    }
    if state.has_items {
        classes.push("ui-dropdown--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-dropdown--empty".to_string());
    }
    if state.keep_open_on_action {
        classes.push("ui-dropdown--persistent".to_string());
    }
    if state.is_controlled {
        classes.push("ui-dropdown--controlled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-dropdown--custom-class".to_string());
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
        assert_eq!(
            normalize_id_base("   ".to_string()),
            ui_state_primitives::dropdown::DEFAULT_ID_BASE
        );
        assert_eq!(
            normalize_aria_label(None),
            (
                ui_state_primitives::dropdown::DEFAULT_ARIA_LABEL.to_string(),
                false
            )
        );
        assert_eq!(
            focus_strategy_for_open_key("ArrowDown"),
            Some(DropdownOpenFocusStrategy::First)
        );
    }

    #[test]
    fn normalize_disabled_state_prefers_is_prefix() {
        assert!(normalize_disabled_state(DisabledStateInput {
            is_disabled: Some(true),
            disabled: false,
        }));
        assert!(!normalize_disabled_state(DisabledStateInput {
            is_disabled: None,
            disabled: false,
        }));
    }

    #[test]
    fn normalize_open_state_prefers_is_open_and_preserves_triplet() {
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
                .expect("normalized open should exist")
                .get_untracked()
        );
        assert_eq!(open_state.default_open, Some(false));
        assert!(open_state.on_open_change.is_some());
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(DropdownStateInput {
            item_count: 0,
            disabled: true,
            close_on_action: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            is_controlled: false,
            has_disabled_items: false,
            has_item_kinds: false,
        });

        let class_name = compose_class_name(Some("docs-dropdown-custom".to_string()), state);

        assert!(class_name.contains("ui-dropdown"));
        assert!(class_name.contains("ui-dropdown--disabled"));
        assert!(class_name.contains("ui-dropdown--empty"));
        assert!(class_name.contains("ui-dropdown--custom-class"));
        assert!(class_name.contains("docs-dropdown-custom"));
    }
}
