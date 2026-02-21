use super::*;
use leptos::prelude::{GetUntracked, Set};

#[test]
fn variant_class_names_are_stable() {
    assert_eq!(
        CheckboxVariant::Default.class_name(),
        "ui-checkbox--variant-default"
    );
    assert_eq!(
        CheckboxVariant::Accent.class_name(),
        "ui-checkbox--variant-accent"
    );
}

#[test]
fn size_class_names_are_stable() {
    assert_eq!(
        CheckboxSize::Default.class_name(),
        "ui-checkbox--size-default"
    );
    assert_eq!(CheckboxSize::Sm.class_name(), "ui-checkbox--size-sm");
    assert_eq!(CheckboxSize::Lg.class_name(), "ui-checkbox--size-lg");
}

#[test]
fn resolve_state_is_consumed_from_state_primitives() {
    let state = resolve_state(CheckboxStateInput {
        is_checked: true,
        is_disabled: false,
        is_pressed: false,
        is_hovered: false,
        is_focused: false,
        is_focus_visible: false,
    });

    let _: ui_state_primitives::checkbox::CheckboxState = state;
}

#[test]
fn derive_render_state_centralizes_state_derivation_and_source_marker() {
    let render_state = derive_render_state(CheckboxRenderStateInput {
        checked_state: CheckboxCheckedState::Checked,
        is_disabled: false,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
        control_mode: CheckedControlMode::Controlled,
    });

    assert!(render_state.state.is_checked);
    assert_eq!(render_state.state_source_attr, "controlled");

    let disabled = derive_render_state(CheckboxRenderStateInput {
        checked_state: CheckboxCheckedState::Unchecked,
        is_disabled: true,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
        control_mode: CheckedControlMode::Uncontrolled,
    });
    assert!(disabled.state.is_disabled);
    assert!(!disabled.state.is_pressed);
    assert!(!disabled.state.is_hovered);
    assert!(!disabled.state.is_focused);
    assert!(!disabled.state.is_focus_visible);
    assert_eq!(disabled.state_source_attr, "uncontrolled");
}

#[test]
fn checked_state_enum_closes_mutually_exclusive_status_space() {
    assert_eq!(
        CheckboxCheckedState::from_bool(true),
        CheckboxCheckedState::Checked
    );
    assert_eq!(
        CheckboxCheckedState::from_bool(false),
        CheckboxCheckedState::Unchecked
    );
    assert!(CheckboxCheckedState::Checked.is_checked());
    assert!(!CheckboxCheckedState::Unchecked.is_checked());
}

#[test]
fn normalize_checked_signal_prefers_primary_name_over_alias() {
    let (primary, _) = leptos::prelude::signal(true);
    let (alias, _) = leptos::prelude::signal(false);

    let resolved = normalize_checked_signal(Some(primary), Some(alias)).expect("signal");
    assert!(resolved.get_untracked());
}

#[test]
fn normalize_checked_change_handler_prefers_primary_name_over_alias() {
    let (primary_value, primary_set) = leptos::prelude::signal(false);
    let (alias_value, alias_set) = leptos::prelude::signal(false);

    let resolved =
        normalize_checked_change_handler(Some(primary_set), Some(alias_set)).expect("handler");
    resolved.set(true);
    assert!(primary_value.get_untracked());
    assert!(!alias_value.get_untracked());
}

#[test]
fn normalize_is_disabled_prefers_is_prefix_value() {
    assert!(normalize_is_disabled(Some(true), false));
    assert!(!normalize_is_disabled(Some(false), true));
    assert!(normalize_is_disabled(None, true));
}

#[test]
fn normalize_optional_text_drops_empty_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("".to_string())), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  custom-class  ".to_string())),
        Some("custom-class".to_string())
    );
}

#[test]
fn compose_class_name_applies_default_in_logic_and_appends_custom_class() {
    assert_eq!(
        compose_class_name(None, CheckboxVariant::Default, CheckboxSize::Default),
        "ui-checkbox ui-checkbox--variant-default ui-checkbox--size-default"
    );
    assert_eq!(
        compose_class_name(
            Some(" docs-checkbox ".to_string()),
            CheckboxVariant::Accent,
            CheckboxSize::Lg,
        ),
        "ui-checkbox ui-checkbox--variant-accent ui-checkbox--size-lg docs-checkbox"
    );
}

#[test]
fn resolve_checked_control_uncontrolled_uses_default_and_internal_writer() {
    let resolved = resolve_checked_control(None, None, None, None, Some(true));
    assert_eq!(resolved.mode, CheckedControlMode::Uncontrolled);
    assert!(resolved.checked.get_untracked());
    assert_eq!(resolved.checked_source_attr, "default-checked");
    assert_eq!(resolved.handler_source_attr, "missing");

    let writer = resolved.on_checked_change.expect("uncontrolled writer");
    writer.set(false);
    assert!(!resolved.checked.get_untracked());
}

#[test]
fn resolve_checked_control_controlled_without_writer_stays_read_only() {
    let (external_checked, _) = leptos::prelude::signal(true);

    let resolved = resolve_checked_control(Some(external_checked), None, None, None, Some(false));
    assert_eq!(resolved.mode, CheckedControlMode::Controlled);
    assert!(resolved.checked.get_untracked());
    assert!(resolved.on_checked_change.is_none());
    assert_eq!(resolved.checked_source_attr, "is-checked");
    assert_eq!(resolved.handler_source_attr, "missing");
}

#[test]
fn resolve_checked_control_controlled_prefers_primary_writer_over_alias() {
    let (external_checked, _) = leptos::prelude::signal(false);
    let (primary_value, primary_set) = leptos::prelude::signal(false);
    let (alias_value, alias_set) = leptos::prelude::signal(false);

    let resolved = resolve_checked_control(
        Some(external_checked),
        None,
        Some(primary_set),
        Some(alias_set),
        None,
    );
    assert_eq!(resolved.mode, CheckedControlMode::Controlled);
    assert_eq!(resolved.checked_source_attr, "is-checked");
    assert_eq!(resolved.handler_source_attr, "on-checked-change");

    let writer = resolved.on_checked_change.expect("controlled writer");
    writer.set(true);
    assert!(primary_value.get_untracked());
    assert!(!alias_value.get_untracked());
}
