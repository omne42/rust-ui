use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  ok  ".to_string())),
        Some("ok".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_default_when_missing() {
    let (aria_label, custom) = normalize_aria_label(None);
    assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
    assert!(!custom);

    let (aria_label, custom) = normalize_aria_label(Some(" Inbox updates ".to_string()));
    assert_eq!(aria_label, "Inbox updates");
    assert!(custom);
}

#[test]
fn normalize_max_height_ignores_zero() {
    assert_eq!(normalize_max_height(Some(0)), None);
    assert_eq!(normalize_max_height(Some(240)), Some(240));
}

#[test]
fn resolve_state_tracks_markers() {
    let state = resolve_state(ScrollAreaStateInput {
        orientation: ScrollAreaOrientation::Horizontal,
        disabled: true,
        max_height_px: Some(180),
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "horizontal");
    assert_eq!(state.max_height_attr, ScrollAreaMaxHeightAttr::Custom);
    assert_eq!(state.aria_source_attr, ScrollAreaSourceAttr::Custom);
    assert_eq!(state.class_source_attr, ScrollAreaSourceAttr::Custom);
    assert!(state.disabled);
}

#[test]
fn resolve_state_uses_closed_enumerated_marker_values() {
    for has_custom_max_height in [false, true] {
        for has_custom_aria_label in [false, true] {
            for has_custom_class_name in [false, true] {
                let state = resolve_state(ScrollAreaStateInput {
                    orientation: ScrollAreaOrientation::Vertical,
                    disabled: false,
                    max_height_px: has_custom_max_height.then_some(160),
                    has_custom_aria_label,
                    has_custom_class_name,
                });

                assert!(matches!(
                    state.max_height_attr,
                    ScrollAreaMaxHeightAttr::Default | ScrollAreaMaxHeightAttr::Custom
                ));
                assert!(matches!(
                    state.aria_source_attr,
                    ScrollAreaSourceAttr::Default | ScrollAreaSourceAttr::Custom
                ));
                assert!(matches!(
                    state.class_source_attr,
                    ScrollAreaSourceAttr::Default | ScrollAreaSourceAttr::Custom
                ));
            }
        }
    }
}
