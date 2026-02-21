use super::*;

#[test]
fn element_default_contract_is_stable() {
    assert_eq!(DescriptionElement::default(), DescriptionElement::Paragraph);
}

#[test]
fn locale_attrs_are_headless_backed_and_normalized() {
    let attrs = resolve_locale_attrs(Some("  zh-CN  ".to_string()), Some(A11yDirection::Rtl));
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));

    let attrs = resolve_locale_attrs(Some(" \n ".to_string()), None);
    assert_eq!(attrs.lang, None);
    assert_eq!(attrs.dir, None);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-description".to_string()),
        resolve_state(DescriptionStateInput {
            tone: DescriptionTone::Negative,
            disabled: true,
            truncate: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-description",
        "ui-description--tone-negative",
        "ui-description--disabled",
        "ui-description--truncate",
        "ui-description--custom-class",
        "docs-description",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should contain `{token}`"
        );
    }
}

#[test]
fn resolve_view_model_centralizes_default_sources() {
    let model = resolve_view_model(DescriptionViewModelInput {
        text: " \n ".to_string(),
        tone: DescriptionTone::Default,
        is_disabled: false,
        is_truncated: false,
        aria_label: Some("  ".to_string()),
        class_name: Some("   ".to_string()),
        lang: Some("  zh-CN  ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(model.text.as_str(), DEFAULT_TEXT);
    assert_eq!(model.aria_label.as_str(), DEFAULT_ARIA_LABEL);
    assert_eq!(model.class_name, None);
    assert_eq!(model.state.aria_source_attr, "default");
    assert_eq!(model.state.class_source_attr, "default");
    assert_eq!(model.lang.as_deref(), Some("zh-CN"));
    assert_eq!(model.dir, Some("rtl"));
}

#[test]
fn resolve_view_model_keeps_explicit_overrides() {
    let model = resolve_view_model(DescriptionViewModelInput {
        text: "Size shown in cm".to_string(),
        tone: DescriptionTone::Muted,
        is_disabled: true,
        is_truncated: true,
        aria_label: Some("Product description".to_string()),
        class_name: Some("docs-description".to_string()),
        lang: Some("en-US".to_string()),
        dir: Some(A11yDirection::Ltr),
    });

    assert_eq!(model.text.as_str(), "Size shown in cm");
    assert_eq!(model.aria_label.as_str(), "Product description");
    assert_eq!(model.class_name.as_deref(), Some("docs-description"));
    assert_eq!(model.state.tone, DescriptionTone::Muted);
    assert!(model.state.is_disabled);
    assert!(model.state.is_truncated);
    assert_eq!(model.state.aria_source_attr, "custom");
    assert_eq!(model.state.class_source_attr, "custom");
    assert_eq!(model.lang.as_deref(), Some("en-US"));
    assert_eq!(model.dir, Some("ltr"));
}

#[test]
fn semantic_marker_values_are_closed_and_enumerable() {
    let allowed_tone = ["default", "muted", "negative"];
    let allowed_state = ["default", "disabled", "truncate"];
    let allowed_source = ["default", "custom"];

    for tone in [
        DescriptionTone::Default,
        DescriptionTone::Muted,
        DescriptionTone::Negative,
    ] {
        for disabled in [false, true] {
            for truncated in [false, true] {
                let state = resolve_state(DescriptionStateInput {
                    tone,
                    disabled,
                    truncate: truncated,
                    has_custom_aria_label: disabled || truncated,
                    has_custom_class_name: truncated,
                });

                assert!(
                    allowed_tone.contains(&state.tone_attr),
                    "unexpected tone marker: {}",
                    state.tone_attr
                );
                assert!(
                    allowed_state.contains(&state.data_state_attr),
                    "unexpected data-state marker: {}",
                    state.data_state_attr
                );
                assert!(
                    allowed_source.contains(&state.aria_source_attr),
                    "unexpected aria-source marker: {}",
                    state.aria_source_attr
                );
                assert!(
                    allowed_source.contains(&state.class_source_attr),
                    "unexpected class-source marker: {}",
                    state.class_source_attr
                );
            }
        }
    }
}
