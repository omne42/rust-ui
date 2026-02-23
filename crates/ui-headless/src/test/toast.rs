use super::*;

#[test]
fn toast_priority_attr_maps_live_region_priority() {
    assert_eq!(toast_priority_attr(LiveRegionPriority::Polite), "polite");
    assert_eq!(
        toast_priority_attr(LiveRegionPriority::Assertive),
        "assertive"
    );
}

#[test]
fn should_dismiss_toast_on_escape_requires_open_non_composing_non_prevented_escape() {
    assert!(should_dismiss_toast_on_escape("Escape", true, false, false));
    assert!(!should_dismiss_toast_on_escape("Enter", true, false, false));
    assert!(!should_dismiss_toast_on_escape(
        "Escape", false, false, false
    ));
    assert!(!should_dismiss_toast_on_escape("Escape", true, true, false));
    assert!(!should_dismiss_toast_on_escape("Escape", true, false, true));
}

#[test]
fn use_toast_a11y_maps_live_region_locale_and_shortcut_attrs() {
    let contract = use_toast_a11y(ToastA11yOptions {
        is_open: Signal::derive(|| true),
        priority: LiveRegionPriority::Assertive,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
        on_dismiss_request: Callback::new(|_| {}),
    });

    assert_eq!(contract.attrs.role, "alert");
    assert_eq!(contract.attrs.aria_live, "assertive");
    assert_eq!(contract.attrs.aria_atomic, "true");
    assert_eq!(contract.attrs.aria_keyshortcuts, "Escape");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.state.priority, LiveRegionPriority::Assertive);
    assert_eq!(contract.state.priority_attr, "assertive");
}
