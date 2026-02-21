use super::*;

#[test]
fn link_logic_consumes_state_primitives() {
    let (_, source) = normalize_is_disabled(Some(true));
    assert_eq!(source.as_attr(), "is-prop");

    let rel = resolve_rel(LinkTargetKind::Blank, Some("sponsored".to_string()));
    assert_eq!(rel, Some("noopener noreferrer sponsored".to_string()));

    let state = resolve_state(LinkStateInput {
        is_disabled: false,
        has_href: true,
        target_kind: LinkTargetKind::Blank,
        has_explicit_rel: false,
        has_aria_label: false,
        has_custom_class_name: false,
    });
    assert_eq!(state.state, LinkVisualState::Enabled);
}

#[test]
fn compose_class_name_includes_state_tokens() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(LinkStateInput {
            is_disabled: false,
            has_href: true,
            target_kind: LinkTargetKind::Blank,
            has_explicit_rel: false,
            has_aria_label: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-link",
        "ui-link--enabled",
        "ui-link--rel-auto",
        "ui-link--external",
        "ui-link--with-aria-label",
        "ui-link--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
