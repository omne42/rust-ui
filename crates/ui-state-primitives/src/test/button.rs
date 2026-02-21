use super::*;

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  button  ".to_string())),
        Some("button".to_string())
    );
}

#[test]
fn resolve_aria_label_only_uses_explicit_label() {
    assert_eq!(
        resolve_aria_label(Some("  Save  ".to_string())),
        (Some("Save".to_string()), ButtonLabelSource::Explicit)
    );
    assert_eq!(resolve_aria_label(None), (None, ButtonLabelSource::None));
}

#[test]
fn resolve_state_core_derives_disabled_and_state_attr() {
    let ready = resolve_state_core(ButtonStateCoreInput {
        is_disabled: false,
        is_loading: false,
        is_full_width: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });
    assert!(!ready.is_disabled);
    assert_eq!(ready.state_attr, "ready");

    let loading = resolve_state_core(ButtonStateCoreInput {
        is_disabled: false,
        is_loading: true,
        is_full_width: true,
        has_custom_class_name: true,
        has_custom_motion: true,
    });
    assert!(loading.is_disabled);
    assert_eq!(loading.state_attr, "loading");
}
