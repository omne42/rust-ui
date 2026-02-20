use super::*;

#[test]
fn variant_class_names_are_stable() {
    assert_eq!(
        ToggleButtonVariant::Default.class_name(),
        "ui-toggle-button--variant-default"
    );
    assert_eq!(
        ToggleButtonVariant::Accent.class_name(),
        "ui-toggle-button--variant-accent"
    );
    assert_eq!(
        ToggleButtonVariant::Destructive.class_name(),
        "ui-toggle-button--variant-destructive"
    );
    assert_eq!(
        ToggleButtonVariant::Outline.class_name(),
        "ui-toggle-button--variant-outline"
    );
    assert_eq!(
        ToggleButtonVariant::Secondary.class_name(),
        "ui-toggle-button--variant-secondary"
    );
    assert_eq!(
        ToggleButtonVariant::Ghost.class_name(),
        "ui-toggle-button--variant-ghost"
    );
}

#[test]
fn size_class_names_are_stable() {
    assert_eq!(
        ToggleButtonSize::Xs.class_name(),
        "ui-toggle-button--size-xs"
    );
    assert_eq!(ToggleButtonSize::S.class_name(), "ui-toggle-button--size-s");
    assert_eq!(ToggleButtonSize::M.class_name(), "ui-toggle-button--size-m");
    assert_eq!(ToggleButtonSize::L.class_name(), "ui-toggle-button--size-l");
    assert_eq!(
        ToggleButtonSize::Xl.class_name(),
        "ui-toggle-button--size-xl"
    );
    assert_eq!(
        ToggleButtonSize::IconXs.class_name(),
        "ui-toggle-button--size-icon-xs"
    );
    assert_eq!(
        ToggleButtonSize::IconS.class_name(),
        "ui-toggle-button--size-icon-s"
    );
    assert_eq!(
        ToggleButtonSize::IconM.class_name(),
        "ui-toggle-button--size-icon-m"
    );
    assert_eq!(
        ToggleButtonSize::IconL.class_name(),
        "ui-toggle-button--size-icon-l"
    );
    assert_eq!(
        ToggleButtonSize::IconXl.class_name(),
        "ui-toggle-button--size-icon-xl"
    );

    assert_eq!(
        ToggleButtonSize::Default.class_name(),
        "ui-toggle-button--size-m"
    );
    assert_eq!(
        ToggleButtonSize::Sm.class_name(),
        "ui-toggle-button--size-s"
    );
    assert_eq!(
        ToggleButtonSize::Lg.class_name(),
        "ui-toggle-button--size-l"
    );
    assert_eq!(
        ToggleButtonSize::Icon.class_name(),
        "ui-toggle-button--size-icon-m"
    );
    assert_eq!(
        ToggleButtonSize::IconSm.class_name(),
        "ui-toggle-button--size-icon-s"
    );
    assert_eq!(
        ToggleButtonSize::IconLg.class_name(),
        "ui-toggle-button--size-icon-l"
    );
}

#[test]
fn resolve_state_tracks_selected_enabled_interactions() {
    let state = resolve_state(true, false, true, true, true, true);

    assert!(state.is_selected);
    assert!(!state.is_unselected);
    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(state.is_pressed);
    assert!(state.is_hovered);
    assert!(state.is_focused);
    assert!(state.is_focus_visible);
    assert_eq!(state.data_state(), "selected");
}

#[test]
fn resolve_state_clears_interaction_flags_when_disabled() {
    let state = resolve_state(false, true, true, true, true, true);

    assert!(!state.is_selected);
    assert!(state.is_unselected);
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(!state.is_pressed);
    assert!(!state.is_hovered);
    assert!(!state.is_focused);
    assert!(!state.is_focus_visible);
    assert_eq!(state.data_state(), "unselected");
}

#[cfg(feature = "component-toggle_button_group")]
#[test]
fn toggle_button_group_orientation_class_and_data_values_are_stable() {
    assert_eq!(
        ToggleButtonGroupOrientation::Horizontal.class_name(),
        "ui-toggle-button-group--horizontal"
    );
    assert_eq!(
        ToggleButtonGroupOrientation::Vertical.class_name(),
        "ui-toggle-button-group--vertical"
    );
    assert_eq!(
        ToggleButtonGroupOrientation::Horizontal.data_orientation(),
        "horizontal"
    );
    assert_eq!(
        ToggleButtonGroupOrientation::Vertical.data_orientation(),
        "vertical"
    );
}

#[cfg(feature = "component-toggle_button_group")]
#[test]
fn toggle_button_group_aria_label_uses_trimmed_label_or_fallback() {
    let (label, explicit) =
        normalize_toggle_button_group_aria_label(Some("  View mode  ".to_string()));
    assert_eq!(label, "View mode");
    assert!(explicit);

    let (label, explicit) = normalize_toggle_button_group_aria_label(Some("   ".to_string()));
    assert_eq!(label, "Toggle group");
    assert!(!explicit);

    let (label, explicit) = normalize_toggle_button_group_aria_label(None);
    assert_eq!(label, "Toggle group");
    assert!(!explicit);
}

#[cfg(feature = "component-toggle_button_group")]
#[test]
fn resolve_toggle_button_group_state_tracks_orientation_attachment_and_label_source() {
    let state =
        resolve_toggle_button_group_state(ToggleButtonGroupOrientation::Vertical, true, true);

    assert!(!state.is_horizontal);
    assert!(state.is_vertical);
    assert!(state.is_attached);
    assert!(!state.is_detached);
    assert!(state.has_explicit_label);
    assert!(!state.has_fallback_label);

    let state =
        resolve_toggle_button_group_state(ToggleButtonGroupOrientation::Horizontal, false, false);

    assert!(state.is_horizontal);
    assert!(!state.is_vertical);
    assert!(!state.is_attached);
    assert!(state.is_detached);
    assert!(!state.has_explicit_label);
    assert!(state.has_fallback_label);
}
