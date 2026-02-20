use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-alert ".to_string())),
        Some("docs-alert".to_string())
    );
}

#[test]
fn resolve_state_core_tracks_content_flags() {
    let detailed = resolve_state_core(AlertStateCoreInput {
        has_title: true,
        has_description: true,
        has_actions: false,
    });
    assert!(detailed.has_title);
    assert_eq!(detailed.title_attr, "present");
    assert!(detailed.has_description);
    assert_eq!(detailed.description_attr, "present");
    assert!(!detailed.has_actions);
    assert_eq!(detailed.actions_attr, "absent");
    assert_eq!(detailed.state_attr, "detailed");

    let compact = resolve_state_core(AlertStateCoreInput {
        has_title: false,
        has_description: true,
        has_actions: true,
    });
    assert!(!compact.has_title);
    assert_eq!(compact.title_attr, "absent");
    assert!(compact.has_description);
    assert_eq!(compact.description_attr, "present");
    assert!(compact.has_actions);
    assert_eq!(compact.actions_attr, "present");
    assert_eq!(compact.state_attr, "compact");
}
