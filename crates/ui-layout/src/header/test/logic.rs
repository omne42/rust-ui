use super::*;

#[test]
fn state_primitives_are_reexported_from_ui_state_primitives() {
    let (label, custom) = normalize_aria_label(Some("  Header area  ".to_string()));
    assert_eq!(label, "Header area");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some(" ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(HeaderStateInput {
        tone: HeaderTone::Default,
        bordered: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-header-custom".to_string()), state);
    for token in [
        "ui-header",
        "ui-header--tone-default",
        "ui-header--bordered",
        "ui-header--custom-class",
        "docs-header-custom",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}

#[test]
fn resolve_agent_contract_uses_tone_and_bordered_axes() {
    let state = resolve_state(HeaderStateInput {
        tone: HeaderTone::Strong,
        bordered: true,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    let contract = resolve_agent_contract(state);

    assert_eq!(contract.schema_attr, "ui.header");
    assert_eq!(contract.intent_attr, "section-heading");
    assert_eq!(contract.action.as_attr(), "static-header");
    assert_eq!(contract.state.as_attr(), "strong-bordered");
    assert_eq!(contract.source.as_attr(), "props-strong");
    assert_eq!(contract.stream_support.as_attr(), "unsupported");
    assert_eq!(contract.stream_fallback.as_attr(), "snapshot");
    assert_eq!(contract.output_status.as_attr(), "verified");
}
