use super::*;
#[test]
fn roving_orientation_matches_headless_contract() {
    assert_eq!(
        roving_orientation(RadioGroupOrientation::Vertical),
        RovingOrientation::Vertical
    );
    assert_eq!(
        roving_orientation(RadioGroupOrientation::Horizontal),
        RovingOrientation::Horizontal
    );
}

#[test]
fn normalize_disabled_prop_prefers_is_disabled_alias() {
    let from_is = normalize_disabled_prop(DisabledPropInput {
        is_disabled: Some(true),
        disabled: false,
    });
    assert!(from_is.is_disabled);
    assert_eq!(from_is.disabled_source_attr, "is_disabled");

    let from_legacy = normalize_disabled_prop(DisabledPropInput {
        is_disabled: None,
        disabled: true,
    });
    assert!(from_legacy.is_disabled);
    assert_eq!(from_legacy.disabled_source_attr, "disabled");
}

#[test]
fn normalize_checked_axis_prefers_is_checked_alias() {
    let from_new = normalize_checked_axis(CheckedAxisInput {
        is_checked: Some(Signal::derive(|| false)),
        checked: Some(Signal::derive(|| true)),
        default_checked: Some(true),
        on_checked_change: Some(Callback::new(|_: bool| {})),
        on_change: Some(Callback::new(|_: bool| {})),
    });
    assert!(from_new.is_controlled);
    assert_eq!(from_new.control_mode_attr, "controlled");
    assert_eq!(from_new.checked_source_attr, "is_checked");
    assert_eq!(from_new.default_checked_source_attr, "provided");
    assert_eq!(from_new.checked_change_source_attr, "on_checked_change");

    let uncontrolled = normalize_checked_axis(CheckedAxisInput {
        is_checked: None,
        checked: None,
        default_checked: None,
        on_checked_change: None,
        on_change: Some(Callback::new(|_: bool| {})),
    });
    assert!(!uncontrolled.is_controlled);
    assert_eq!(uncontrolled.control_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.checked_source_attr, "default");
    assert_eq!(uncontrolled.default_checked_source_attr, "default");
    assert_eq!(uncontrolled.checked_change_source_attr, "on_change");
}
