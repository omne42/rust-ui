use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn snippet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/snippet/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Snippet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn snippet_uses_logic_state_model() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");

    for needle in [
        "pub struct SnippetStateInput",
        "pub struct SnippetViewState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: SnippetStateInput)",
        "pub fn compose_class_name(",
        "pub struct SnippetLogic",
        "pub fn use_snippet_logic(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Snippet logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(label)",
        "logic::normalize_optional_text(copied_label)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SnippetStateInput {",
        "logic::compose_class_name(class_name, state)",
        "use_snippet_logic(text.clone())",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn snippet_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/snippet/view.rs");

    for attr in [
        "data-slot=\"snippet\"",
        "data-state=state.state_attr",
        "data-copy=state.copy_state_attr",
        "data-multiline=state.is_multiline.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-label=state.has_label.then_some(\"true\")",
        "data-copyable=state.copyable.then_some(\"true\")",
        "data-copy-actionable=state.copy_is_actionable.then_some(\"true\")",
        "data-copied-label=state.copied_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"snippet-copy-button\"",
        "data-slot=\"snippet-copied-status\"",
    ] {
        assert!(
            source.contains(attr),
            "Snippet should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn snippet_styles_include_state_marker_contracts() {
    let source = load_source("src/snippet/styles.rs");

    for selector in [
        ".ui-snippet--state-multiline",
        ".ui-snippet[data-state=\"single-line\"]",
        ".ui-snippet--copyable",
        ".ui-snippet[data-copy=\"disabled\"]",
        ".ui-snippet--copy-static",
        ".ui-snippet--custom-copied-label",
        ".ui-snippet[data-copied-label=\"custom\"]",
        ".ui-snippet--custom-class",
        ".ui-snippet[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Snippet styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn snippet_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn snippet() -> AnyView",
        "title=\"Snippet\"",
        "slug=\"snippet\"",
        "Playground title=\"Copyable + Copied Label\"",
        "Playground title=\"Static + Multiline Custom\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Snippet.",
        );
    }
}

#[test]
fn snippet_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Copyable + Copied Label\"",
        "<Snippet text=\"cargo fmt --all\".to_string() label=\"Command\".to_string() copyable=true />",
        "<Snippet text=\"RUST_LOG=debug\".to_string() copyable=true copied_label=\"Done\".to_string() />",
        "title=\"Static + Multiline Custom\"",
        "text=\"cargo test -p ui-components --test snippet_semantics\".to_string()",
        "text=\"cargo fmt --all\\ncargo clippy -p ui-components -p docs-app --all-targets -- -D warnings\".to_string()",
        "label=\"CI\".to_string()",
        "copyable=false",
        "class_name=\"docs-snippet-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "snippet docs playgrounds should contain `{needle}`.",
        );
    }
}
