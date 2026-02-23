use super::*;

#[test]
fn aria_expanded_is_false_when_closed() {
    let (open, set_open) = signal(false);
    let expanded = aria_expanded(open.into());

    assert_eq!(expanded.get_untracked(), "false");

    set_open.set(true);
    assert_eq!(expanded.get_untracked(), "true");
}

#[test]
fn locale_attrs_trims_lang_and_maps_dir() {
    let attrs = locale_attrs(Some("  zh-CN  ".to_string()), Some(A11yDirection::Rtl));
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn locale_attrs_drops_blank_lang() {
    let attrs = locale_attrs(Some("   ".to_string()), None);
    assert_eq!(attrs.lang, None);
    assert_eq!(attrs.dir, None);
}

#[test]
fn live_region_attrs_maps_priority_to_role_and_aria_live() {
    let polite = live_region_attrs(LiveRegionPriority::Polite);
    assert_eq!(polite.role, "status");
    assert_eq!(polite.aria_live, "polite");

    let assertive = live_region_attrs(LiveRegionPriority::Assertive);
    assert_eq!(assertive.role, "alert");
    assert_eq!(assertive.aria_live, "assertive");
}

#[test]
fn error_view_attrs_maps_live_region_visibility_and_locale() {
    let (is_visible, set_is_visible) = signal(false);
    let attrs = error_view_attrs(
        is_visible.into(),
        " Email error ".to_string(),
        Some("  zh-CN ".to_string()),
        Some(A11yDirection::Rtl),
    );

    assert_eq!(attrs.role, "alert");
    assert_eq!(attrs.aria_live.get_untracked(), "off");
    assert_eq!(attrs.aria_hidden.get_untracked(), Some("true"));
    assert_eq!(attrs.aria_label, " Email error ");
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));

    set_is_visible.set(true);
    assert_eq!(attrs.aria_live.get_untracked(), "assertive");
    assert_eq!(attrs.aria_hidden.get_untracked(), None);
}

#[test]
fn region_attrs_maps_role_label_and_locale() {
    let attrs = region_attrs(
        "Notifications".to_string(),
        Some("  en-US ".to_string()),
        Some(A11yDirection::Rtl),
    );

    assert_eq!(attrs.role, "region");
    assert_eq!(attrs.aria_label, "Notifications");
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn navigation_attrs_maps_label_and_locale_without_overriding_role() {
    let attrs = navigation_attrs(
        "Breadcrumb".to_string(),
        Some("  en-US ".to_string()),
        Some(A11yDirection::Ltr),
    );

    assert_eq!(attrs.aria_label, "Breadcrumb");
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("ltr"));
}

#[test]
fn fieldset_attrs_maps_label_state_and_locale() {
    let attrs = fieldset_attrs(
        "Notification group".to_string(),
        true,
        false,
        Some("  zh-CN ".to_string()),
        Some(A11yDirection::Rtl),
    );

    assert_eq!(attrs.aria_label, "Notification group");
    assert_eq!(attrs.aria_disabled, Some("true"));
    assert_eq!(attrs.aria_invalid, None);
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn disclosure_trigger_attrs_exposes_typed_aria_and_locale_fields() {
    let (open, set_open) = signal(false);
    let attrs = disclosure_trigger_attrs(
        open.into(),
        "demo-controls".to_string(),
        Some("en-US".to_string()),
        Some(A11yDirection::Ltr),
    );

    assert_eq!(attrs.aria_expanded.get_untracked(), "false");
    assert_eq!(attrs.aria_controls, "demo-controls");
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("ltr"));

    set_open.set(true);
    assert_eq!(attrs.aria_expanded.get_untracked(), "true");
}

#[test]
fn aria_controls_when_open_is_none_when_closed() {
    let (open, set_open) = signal(false);
    let controls = aria_controls_when_open(open.into(), "demo-controls".to_string());

    assert_eq!(controls.get_untracked(), None);

    set_open.set(true);
    assert_eq!(controls.get_untracked(), Some("demo-controls".to_string()));
}

#[test]
fn popup_trigger_attrs_maps_controls_expanded_and_locale() {
    let (open, set_open) = signal(false);
    let attrs = popup_trigger_attrs(
        Some("dialog"),
        Some("fallback-controls".to_string()),
        None,
        Some(open.into()),
        Some("  en-US ".to_string()),
        Some(A11yDirection::Ltr),
    );

    assert_eq!(attrs.aria_haspopup, Some("dialog"));
    assert_eq!(
        attrs.aria_controls.get_untracked(),
        Some("fallback-controls".to_string())
    );
    assert_eq!(attrs.aria_expanded.get_untracked(), Some("false"));
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("ltr"));

    set_open.set(true);
    assert_eq!(attrs.aria_expanded.get_untracked(), Some("true"));
}

#[test]
fn popup_trigger_attrs_prefers_signal_controls_when_present() {
    let (controls, set_controls) = signal(Some("signal-controls".to_string()));
    let attrs = popup_trigger_attrs(
        Some("menu"),
        Some("fallback-controls".to_string()),
        Some(controls.into()),
        None,
        None,
        None,
    );

    assert_eq!(
        attrs.aria_controls.get_untracked(),
        Some("signal-controls".to_string())
    );
    assert_eq!(attrs.aria_expanded.get_untracked(), None);

    set_controls.set(Some("next-controls".to_string()));
    assert_eq!(
        attrs.aria_controls.get_untracked(),
        Some("next-controls".to_string())
    );
}

#[test]
fn image_fallback_attrs_only_applies_when_image_is_hidden() {
    let hidden = image_fallback_attrs(false, "Avatar".to_string());
    assert_eq!(hidden.role, Some("img"));
    assert_eq!(hidden.aria_label.as_deref(), Some("Avatar"));

    let shown = image_fallback_attrs(true, "Avatar".to_string());
    assert_eq!(shown.role, None);
    assert_eq!(shown.aria_label, None);
}

#[test]
fn labeled_group_attrs_exposes_typed_group_role_label_and_locale() {
    let attrs = labeled_group_attrs(
        "Collaborators".to_string(),
        Some(" en-US ".to_string()),
        Some(A11yDirection::Rtl),
    );
    assert_eq!(attrs.role, "group");
    assert_eq!(attrs.aria_label, "Collaborators");
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn labeled_toolbar_attrs_exposes_toolbar_role_state_and_locale() {
    let attrs = labeled_toolbar_attrs(
        "Formatting".to_string(),
        "vertical",
        true,
        Some(" zh-CN ".to_string()),
        Some(A11yDirection::Ltr),
    );

    assert_eq!(attrs.role, "toolbar");
    assert_eq!(attrs.aria_label, "Formatting");
    assert_eq!(attrs.aria_orientation, "vertical");
    assert_eq!(attrs.aria_disabled, Some("true"));
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("ltr"));
}

#[test]
fn overlay_dialog_attrs_trims_ids_and_maps_locale() {
    let attrs = overlay_dialog_attrs(
        Some(" dialog-title ".to_string()),
        Some(" dialog-description ".to_string()),
        Some(" zh-CN ".to_string()),
        Some(A11yDirection::Rtl),
    );

    assert_eq!(attrs.aria_labelledby.as_deref(), Some("dialog-title"));
    assert_eq!(
        attrs.aria_describedby.as_deref(),
        Some("dialog-description")
    );
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn overlay_dialog_attrs_drops_blank_optional_ids() {
    let attrs = overlay_dialog_attrs(
        Some("   ".to_string()),
        Some("\n\t".to_string()),
        None,
        None,
    );

    assert_eq!(attrs.aria_labelledby, None);
    assert_eq!(attrs.aria_describedby, None);
    assert_eq!(attrs.lang, None);
    assert_eq!(attrs.dir, None);
}

#[test]
fn tooltip_panel_attrs_exposes_typed_role_id_locale_and_open_state() {
    let contract = tooltip_panel_attrs(TooltipPanelA11yOptions {
        tooltip_id: " docs-tooltip ".to_string(),
        is_open: true,
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "tooltip");
    assert_eq!(contract.attrs.id, " docs-tooltip ");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert!(contract.state.is_open);
}

#[test]
fn tooltip_panel_attrs_drops_blank_lang_and_tracks_closed_state() {
    let contract = tooltip_panel_attrs(TooltipPanelA11yOptions {
        tooltip_id: "docs-tooltip".to_string(),
        is_open: false,
        lang: Some("   ".to_string()),
        dir: None,
    });

    assert_eq!(contract.attrs.role, "tooltip");
    assert_eq!(contract.attrs.id, "docs-tooltip");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert!(!contract.state.is_open);
}

#[test]
fn progressbar_attrs_maps_determinate_values_and_locale() {
    let contract = progressbar_attrs(ProgressbarA11yOptions {
        aria_label: "Upload progress".to_string(),
        aria_valuemin: 0.0,
        aria_valuemax: 100.0,
        aria_valuenow: Some(42.0),
        aria_valuetext: Some("42%".to_string()),
        is_indeterminate: false,
        lang: Some(" en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "progressbar");
    assert_eq!(contract.attrs.aria_label, "Upload progress");
    assert_eq!(contract.attrs.aria_valuemin, "0");
    assert_eq!(contract.attrs.aria_valuemax, "100");
    assert_eq!(contract.attrs.aria_valuenow.as_deref(), Some("42"));
    assert_eq!(contract.attrs.aria_valuetext.as_deref(), Some("42%"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_state, "determinate");
    assert_eq!(contract.attrs.data_determinate, Some("true"));
    assert_eq!(contract.attrs.data_indeterminate, None);
    assert_eq!(contract.state.phase, ProgressbarA11yPhase::Determinate);
    assert!(contract.state.is_determinate);
    assert!(!contract.state.is_indeterminate);
}

#[test]
fn progressbar_attrs_hides_valuenow_when_indeterminate() {
    let contract = progressbar_attrs(ProgressbarA11yOptions {
        aria_label: "Syncing".to_string(),
        aria_valuemin: f64::NAN,
        aria_valuemax: f64::INFINITY,
        aria_valuenow: Some(64.0),
        aria_valuetext: None,
        is_indeterminate: true,
        lang: None,
        dir: Some(A11yDirection::Ltr),
    });

    assert_eq!(contract.attrs.aria_valuemin, "0");
    assert_eq!(contract.attrs.aria_valuemax, "100");
    assert_eq!(contract.attrs.aria_valuenow, None);
    assert_eq!(contract.attrs.data_state, "indeterminate");
    assert_eq!(contract.attrs.data_indeterminate, Some("true"));
    assert_eq!(contract.attrs.data_determinate, None);
    assert_eq!(contract.state.phase, ProgressbarA11yPhase::Indeterminate);
    assert!(contract.state.is_indeterminate);
    assert!(!contract.state.is_determinate);
}

#[test]
fn focusable_element_kind_covers_tag_and_tabindex_rules() {
    assert!(is_focusable_element_kind("button", false, false, None));
    assert!(is_focusable_element_kind("a", true, false, None));
    assert!(!is_focusable_element_kind("a", false, false, None));
    assert!(is_focusable_element_kind("div", false, true, None));
    assert!(is_focusable_element_kind("div", false, false, Some("0")));
    assert!(is_focusable_element_kind("div", false, false, Some(" 2 ")));
    assert!(!is_focusable_element_kind("div", false, false, Some("-1")));
    assert!(!is_focusable_element_kind("div", false, false, Some("")));
    assert!(!is_focusable_element_kind("div", false, false, Some("abc")));
}
