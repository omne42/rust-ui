use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(
        normalize_optional_text(Some("  Toggle theme  ".to_string())),
        Some("Toggle theme".to_string())
    );
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(normalize_optional_text(None), None);
}

#[test]
fn normalize_modes_deduplicates_and_falls_back_to_defaults() {
    assert_eq!(
        normalize_modes(vec![ThemeMode::Dark, ThemeMode::Dark, ThemeMode::Light]),
        vec![ThemeMode::Dark, ThemeMode::Light]
    );
    assert_eq!(
        normalize_modes(Vec::new()),
        vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
    );
}

#[test]
fn next_cycles_through_modes() {
    let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled];
    assert_eq!(resolve_next(ThemeMode::Light, &modes), ThemeMode::Dark);
    assert_eq!(resolve_next(ThemeMode::Dark, &modes), ThemeMode::Oled);
    assert_eq!(resolve_next(ThemeMode::Oled, &modes), ThemeMode::Light);
}

#[test]
fn next_falls_back_to_first_when_current_not_found() {
    let modes = [ThemeMode::Dark, ThemeMode::Light];
    assert_eq!(resolve_next(ThemeMode::Oled, &modes), ThemeMode::Dark);
}

#[test]
fn empty_modes_defaults_to_light() {
    assert_eq!(resolve_next(ThemeMode::Dark, &[]), ThemeMode::Light);
}

#[test]
fn resolve_state_tracks_mode_enablement_and_metadata() {
    let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled];
    let enabled = resolve_state(ThemeMode::Dark, &modes, false, true, true, true);
    assert!(enabled.is_enabled);
    assert!(!enabled.is_disabled);
    assert_eq!(enabled.mode_count, 3);
    assert_eq!(enabled.current_mode_attr, "dark");
    assert_eq!(enabled.next_mode_attr, "oled");
    assert!(enabled.has_custom_modes);
    assert!(enabled.has_custom_aria_label);
    assert!(enabled.has_custom_class_name);

    let disabled = resolve_state(ThemeMode::Light, &modes, true, false, false, false);
    assert!(disabled.is_disabled);
    assert!(!disabled.is_enabled);
    assert_eq!(disabled.next_mode, ThemeMode::Dark);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(
            ThemeMode::Light,
            &[ThemeMode::Light, ThemeMode::Dark],
            false,
            true,
            true,
            true,
        ),
    );

    for token in [
        "ui-theme-toggle-button",
        "ui-theme-toggle-button--enabled",
        "ui-theme-toggle-button--custom-modes",
        "ui-theme-toggle-button--custom-aria-label",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
