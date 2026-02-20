use super::*;

#[test]
fn flex_contract_tokens_are_stable() {
    assert_eq!(FlexDirection::Row.class_name(), "ui-flex--direction-row");
    assert_eq!(FlexWrap::Wrap.class_name(), "ui-flex--wrap-wrap");
    assert_eq!(
        FlexJustify::SpaceEvenly.class_name(),
        "ui-flex--justify-space-evenly"
    );
    assert_eq!(FlexAlign::Baseline.class_name(), "ui-flex--align-baseline");
    assert_eq!(FlexGap::Md.class_name(), "ui-flex--gap-md");

    assert_eq!(FlexDirection::ColumnReverse.as_attr(), "column-reverse");
    assert_eq!(FlexWrap::NoWrap.as_attr(), "nowrap");
    assert_eq!(FlexJustify::SpaceBetween.as_attr(), "space-between");
    assert_eq!(FlexAlign::Stretch.as_attr(), "stretch");
    assert_eq!(FlexGap::None.as_attr(), "none");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-flex  ".to_string())),
        Some("docs-flex".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Toolbar layout  ".to_string()));
    assert_eq!(label, "Toolbar layout");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_layout_and_sources() {
    let state = resolve_state(FlexStateInput {
        direction: FlexDirection::Column,
        wrap: FlexWrap::Wrap,
        justify: FlexJustify::SpaceBetween,
        align: FlexAlign::Center,
        gap: FlexGap::Lg,
        inline: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.direction_attr, "column");
    assert_eq!(state.wrap_attr, "wrap");
    assert_eq!(state.justify_attr, "space-between");
    assert_eq!(state.align_attr, "center");
    assert_eq!(state.gap_attr, "lg");
    assert!(state.is_inline);
    assert_eq!(state.data_state_attr, "inline-wrap-gapped");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(FlexStateInput {
        direction: FlexDirection::Row,
        wrap: FlexWrap::Wrap,
        justify: FlexJustify::Start,
        align: FlexAlign::Stretch,
        gap: FlexGap::Sm,
        inline: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-flex-custom".to_string()), state);

    for token in [
        "ui-flex",
        "ui-flex--direction-row",
        "ui-flex--wrap-wrap",
        "ui-flex--justify-start",
        "ui-flex--align-stretch",
        "ui-flex--gap-sm",
        "ui-flex--custom-class",
        "docs-flex-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
