use super::*;

#[test]
fn contract_maps_locale_and_actionability() {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: "cargo fmt --all".to_string(),
        is_copyable: true,
        is_copied: None,
        default_copied: None,
        on_copied_change: None,
        on_copy_error: None,
        lang: Some("  en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert!(contract.state.is_copyable);
    assert!(contract.state.is_actionable);
    assert!(!contract.state.copied.get_untracked());
    assert!(!contract.state.is_loading.get_untracked());
    assert!(!contract.state.has_error.get_untracked());
    assert_eq!(contract.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.aria_busy.get_untracked(), None);
}

#[test]
fn contract_respects_controlled_copied_state() {
    let (is_copied, set_is_copied) = signal(false);
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: "cargo test".to_string(),
        is_copyable: true,
        is_copied: Some(is_copied.into()),
        default_copied: Some(true),
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    });

    assert!(!contract.state.copied.get_untracked());
    set_is_copied.set(true);
    assert!(contract.state.copied.get_untracked());
}

#[test]
fn contract_disables_copy_when_text_is_blank() {
    let contract = use_snippet_copy(SnippetCopyOptions {
        text: "\n\t ".to_string(),
        is_copyable: true,
        is_copied: None,
        default_copied: None,
        on_copied_change: None,
        on_copy_error: None,
        lang: None,
        dir: None,
    });

    assert!(contract.state.is_copyable);
    assert!(!contract.state.is_actionable);

    contract.handlers.on_copy.run(());
    assert!(!contract.state.is_loading.get_untracked());
    assert!(!contract.state.has_error.get_untracked());
    assert!(!contract.state.copied.get_untracked());
}
