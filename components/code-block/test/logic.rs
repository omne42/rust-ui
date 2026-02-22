use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  rust  ".to_string())),
        Some("rust".to_string())
    );
}

#[test]
fn resolve_copyable_contract_prefers_is_copyable_then_legacy_then_default() {
    let canonical = resolve_copyable_contract(Some(false), Some(true));
    assert!(!canonical.is_copyable);
    assert_eq!(canonical.source.as_attr(), "is_copyable");

    let legacy = resolve_copyable_contract(None, Some(false));
    assert!(!legacy.is_copyable);
    assert_eq!(legacy.source.as_attr(), "copyable_legacy");

    let defaulted = resolve_copyable_contract(None, None);
    assert!(defaulted.is_copyable);
    assert_eq!(defaulted.source.as_attr(), "default");
}

#[test]
fn resolve_copied_contract_centralizes_default_priority() {
    let (is_copied, _set_is_copied) = leptos::prelude::signal(false);
    let (legacy_copied, _set_legacy_copied) = leptos::prelude::signal(true);
    let on_copied_change = leptos::prelude::Callback::new(|_| {});

    let controlled = resolve_copied_contract(
        Some(is_copied.into()),
        Some(legacy_copied.into()),
        Some(true),
        Some(on_copied_change),
    );
    assert_eq!(controlled.source.as_attr(), "controlled");
    assert!(controlled.copied.is_some());
    assert!(controlled.default_copied);
    assert!(controlled.on_copied_change.is_some());

    let uncontrolled = resolve_copied_contract(None, None, None, None);
    assert_eq!(uncontrolled.source.as_attr(), "uncontrolled");
    assert_eq!(uncontrolled.copied, None);
    assert!(!uncontrolled.default_copied);
    assert!(uncontrolled.on_copied_change.is_none());
}

#[test]
fn resolve_state_is_consumed_from_primitives() {
    let state = resolve_state(CodeBlockStateInput {
        is_multiline: true,
        is_empty: false,
        has_label: true,
        has_language: true,
        copyable: true,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(state.show_header);
    assert_eq!(state.state_class, "ui-code-block--state-multiline");
    assert_eq!(state.header_class, "ui-code-block--header-visible");
    assert_eq!(state.motion_source_class, "ui-code-block--motion-custom");
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-code-block".to_string()),
        resolve_state(CodeBlockStateInput {
            is_multiline: true,
            is_empty: true,
            has_label: true,
            has_language: false,
            copyable: true,
            has_custom_class_name: true,
            has_custom_motion: false,
        }),
    );

    for token in [
        "ui-code-block",
        "ui-code-block--state-multiline",
        "ui-code-block--header-visible",
        "ui-code-block--motion-default",
        "ui-code-block--copyable",
        "ui-code-block--with-label",
        "ui-code-block--empty",
        "ui-code-block--custom-class",
        "docs-code-block",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_render_model_normalizes_inputs_and_derives_state() {
    let model = resolve_render_model(CodeBlockLogicInput {
        code: "let x = 1;\nlet y = 2;".to_string(),
        label: Some("  Demo  ".to_string()),
        language: Some(" rs ".to_string()),
        is_copyable: true,
        class_name: Some(" docs-block ".to_string()),
        has_custom_motion: true,
    });

    assert_eq!(model.label.as_deref(), Some("Demo"));
    assert_eq!(model.language.as_deref(), Some("rs"));
    assert_eq!(model.state.state_attr, "multiline");
    assert_eq!(model.state.motion_source_attr, "custom");
    assert!(model.state.show_header);
    assert!(model.class_name.contains("ui-code-block"));
    assert!(model.class_name.contains("docs-block"));
}

#[test]
fn typed_sources_and_normalization_keep_machine_readable_state_contract() {
    assert_eq!(CodeBlockCopyableSource::Default.as_attr(), "default");
    assert_eq!(
        CodeBlockCopyableSource::IsCopyableProp.as_attr(),
        "is_copyable"
    );
    assert_eq!(
        CodeBlockCopyableSource::LegacyCopyableProp.as_attr(),
        "copyable_legacy"
    );
    assert_eq!(CodeBlockCopiedSource::Controlled.as_attr(), "controlled");
    assert_eq!(
        CodeBlockCopiedSource::Uncontrolled.as_attr(),
        "uncontrolled"
    );

    let model = resolve_render_model(CodeBlockLogicInput {
        code: "let x = 1;".to_string(),
        label: Some("  ".to_string()),
        language: Some("\n\t".to_string()),
        is_copyable: false,
        class_name: Some("  ".to_string()),
        has_custom_motion: false,
    });

    assert_eq!(model.label, None);
    assert_eq!(model.language, None);
    assert_eq!(model.state.state_attr, "single-line");
    assert_eq!(model.state.header_attr, "hidden");
    assert_eq!(model.state.motion_source_attr, "default");
    assert!(!model.state.show_header);
    assert!(!model.state.has_custom_class_name);
}
