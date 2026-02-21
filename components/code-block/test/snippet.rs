use super::*;

#[test]
fn snippet_logic_options_support_controlled_copied_axis() {
    let (is_copied, set_is_copied) = signal(false);
    let logic = use_snippet_logic_with_options(SnippetLogicOptions {
        text: "cargo test".to_string(),
        copied: Some(is_copied.into()),
        default_copied: true,
        on_copied_change: None,
        copied_source: CodeBlockCopiedSource::Controlled,
        lang: None,
        dir: None,
    });

    assert_eq!(logic.copied_source, CodeBlockCopiedSource::Controlled);
    assert!(!logic.copied.get_untracked());
    assert!(!logic.is_loading.get_untracked());
    assert!(!logic.has_error.get_untracked());
    assert_eq!(logic.aria_busy.get_untracked(), None);
    set_is_copied.set(true);
    assert!(logic.copied.get_untracked());
}

#[test]
fn snippet_logic_default_path_is_uncontrolled() {
    let logic = use_snippet_logic("cargo fmt --all".to_string());
    assert_eq!(logic.copied_source, CodeBlockCopiedSource::Uncontrolled);
    assert!(!logic.is_loading.get_untracked());
    assert!(!logic.has_error.get_untracked());
    assert_eq!(logic.aria_busy.get_untracked(), None);
}

#[test]
fn snippet_logic_passes_lang_and_dir_to_headless_contract() {
    let logic = use_snippet_logic_with_options(SnippetLogicOptions {
        text: "cargo check".to_string(),
        copied: None,
        default_copied: false,
        on_copied_change: None,
        copied_source: CodeBlockCopiedSource::Uncontrolled,
        lang: Some("  en-US ".to_string()),
        dir: Some(ui_headless::a11y::A11yDirection::Rtl),
    });

    assert_eq!(logic.lang.as_deref(), Some("en-US"));
    assert_eq!(logic.dir, Some("rtl"));
}
