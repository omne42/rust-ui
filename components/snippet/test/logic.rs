use super::*;

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-snippet".to_string()),
        resolve_state(SnippetStateInput {
            is_multiline: false,
            has_text: false,
            has_label: true,
            is_copyable: true,
            has_custom_copied_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-snippet",
        "ui-snippet--state-single-line",
        "ui-snippet--copy-disabled",
        "ui-snippet--default-copied-label",
        "ui-snippet--with-label",
        "ui-snippet--empty",
        "ui-snippet--custom-class",
        "docs-snippet",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn snippet_logic_supports_controlled_copied_axis() {
    let (is_copied, set_is_copied) = signal(false);

    let logic = use_snippet_logic_with_options(SnippetLogicOptions {
        text: "cargo test".to_string(),
        is_copyable: true,
        is_copied: Some(is_copied.into()),
        default_copied: Some(true),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    });

    assert!(!logic.copied.get_untracked());
    set_is_copied.set(true);
    assert!(logic.copied.get_untracked());
}

#[test]
fn default_snippet_logic_is_uncontrolled_and_not_busy() {
    let logic = use_snippet_logic_with_options(SnippetLogicOptions {
        text: "cargo fmt --all".to_string(),
        is_copyable: true,
        is_copied: None,
        default_copied: Some(false),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    });
    assert!(!logic.copied.get_untracked());
    assert!(!logic.is_loading.get_untracked());
    assert!(!logic.has_error.get_untracked());
    assert_eq!(logic.aria_busy.get_untracked(), None);
}

#[test]
fn resolve_text_contract_centralizes_defaults() {
    let contract = resolve_text_contract(None, None, None, None, SnippetTextFallbacks::default());
    assert_eq!(contract.copy_label, DEFAULT_COPY_LABEL);
    assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
    assert_eq!(contract.copy_aria_label, DEFAULT_COPY_ARIA_LABEL);
    assert_eq!(contract.copy_error_label, DEFAULT_COPY_ERROR_LABEL);
}

#[test]
fn resolve_text_contract_prefers_props_then_i18n_then_defaults() {
    let contract = resolve_text_contract(
        None,
        Some("Copied now".to_string()),
        None,
        None,
        SnippetTextFallbacks {
            copy_label: Some("复制".to_string()),
            copied_label: Some("已复制".to_string()),
            copy_aria_label: Some("复制到剪贴板".to_string()),
            copy_error_label: Some("复制失败，请重试".to_string()),
        },
    );

    assert_eq!(contract.copy_label, "复制");
    assert_eq!(contract.copied_label, "Copied now");
    assert_eq!(contract.copy_aria_label, "复制到剪贴板");
    assert_eq!(contract.copy_error_label, "复制失败，请重试");
}

#[test]
fn resolve_copyable_contract_tracks_source() {
    let from_default = resolve_copyable_contract(None, None);
    assert_eq!(from_default.source, SnippetCopyableSource::Default);
    assert!(from_default.is_copyable);

    let from_new_prop = resolve_copyable_contract(Some(false), Some(true));
    assert_eq!(from_new_prop.source, SnippetCopyableSource::IsCopyableProp);
    assert!(!from_new_prop.is_copyable);

    let from_legacy_prop = resolve_copyable_contract(None, Some(false));
    assert_eq!(
        from_legacy_prop.source,
        SnippetCopyableSource::LegacyCopyableProp
    );
    assert!(!from_legacy_prop.is_copyable);
}
