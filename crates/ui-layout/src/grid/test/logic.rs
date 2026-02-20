use super::*;

#[test]
fn grid_tokens_contract_is_stable() {
    assert_eq!(GridColumns::One.class_name(), "ui-grid--columns-1");
    assert_eq!(GridColumns::AutoFit.as_attr(), "auto-fit");

    assert_eq!(GridRows::Equal.class_name(), "ui-grid--rows-equal");
    assert_eq!(GridRows::Compact.as_attr(), "compact");

    assert_eq!(GridGap::Md.class_name(), "ui-grid--gap-md");
    assert_eq!(GridGap::None.as_attr(), "none");

    assert_eq!(GridJustify::Center.class_name(), "ui-grid--justify-center");
    assert_eq!(GridJustify::Stretch.as_attr(), "stretch");

    assert_eq!(GridAlign::End.class_name(), "ui-grid--align-end");
    assert_eq!(GridAlign::Start.as_attr(), "start");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-grid  ".to_string())),
        Some("docs-grid".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_fallback_when_missing() {
    let (label, custom) = normalize_aria_label(Some("  Metrics Grid  ".to_string()));
    assert_eq!(label, "Metrics Grid");
    assert!(custom);

    let (label, custom) = normalize_aria_label(Some("  ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);
}

#[test]
fn resolve_state_tracks_layout_and_sources() {
    let state = resolve_state(GridStateInput {
        columns: GridColumns::AutoFit,
        rows: GridRows::Equal,
        gap: GridGap::Lg,
        justify: GridJustify::Center,
        align: GridAlign::Stretch,
        dense: true,
        inline: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.columns_attr, "auto-fit");
    assert_eq!(state.rows_attr, "equal");
    assert_eq!(state.gap_attr, "lg");
    assert_eq!(state.justify_attr, "center");
    assert_eq!(state.align_attr, "stretch");
    assert!(state.is_dense);
    assert!(!state.is_inline);
    assert_eq!(state.data_state_attr, "dense");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(GridStateInput {
        columns: GridColumns::Three,
        rows: GridRows::Auto,
        gap: GridGap::Sm,
        justify: GridJustify::Start,
        align: GridAlign::Start,
        dense: false,
        inline: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-grid-custom".to_string()), state);

    for token in [
        "ui-grid",
        "ui-grid--columns-3",
        "ui-grid--rows-auto",
        "ui-grid--gap-sm",
        "ui-grid--justify-start",
        "ui-grid--align-start",
        "ui-grid--inline",
        "ui-grid--custom-class",
        "docs-grid-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
