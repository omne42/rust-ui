use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn kbd_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/kbd/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Kbd internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn kbd_uses_logic_state_model() {
    let view_source = load_source("src/kbd/view.rs");
    let logic_source = load_source("src/kbd/logic.rs");

    for needle in [
        "pub struct KbdStateInput",
        "pub struct KbdState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: KbdStateInput)",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Kbd logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(keys)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(KbdStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Kbd view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn kbd_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/kbd/view.rs");

    for attr in [
        "data-slot=\"kbd\"",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-keys=state.has_keys.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"kbd-keys\"",
        "data-slot=\"kbd-label\"",
    ] {
        assert!(
            source.contains(attr),
            "Kbd should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn kbd_styles_include_size_and_state_markers() {
    let source = load_source("src/kbd/styles.rs");

    for selector in [
        ".ui-kbd--size-sm",
        ".ui-kbd[data-size=\"md\"]",
        ".ui-kbd--state-with-keys",
        ".ui-kbd[data-state=\"label-only\"]",
        ".ui-kbd--custom-class",
        ".ui-kbd[data-custom-class=\"true\"]",
        ".ui-kbd__label",
    ] {
        assert!(
            source.contains(selector),
            "Kbd styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn kbd_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn kbd() -> AnyView",
        "title=\"Kbd\"",
        "slug=\"kbd\"",
        "Playground title=\"Size + Keys Matrix\"",
        "Playground title=\"Custom Class + Label Only\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Kbd.",
        );
    }
}

#[test]
fn kbd_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Size + Keys Matrix\"",
        "<Kbd size=KbdSize::Md keys=\"Ctrl\".to_string()>\"K\"</Kbd>",
        "<Kbd size=KbdSize::Sm keys=\"⌘\".to_string()>\"P\"</Kbd>",
        "<Kbd size=KbdSize::Md keys=\"Alt\".to_string()>\"Enter\"</Kbd>",
        "title=\"Custom Class + Label Only\"",
        "<Kbd size=KbdSize::Md class_name=\"docs-kbd-custom\".to_string()>\"Esc\"</Kbd>",
        "keys=\"Shift\".to_string()",
        "class_name=\"docs-kbd-custom\".to_string()",
        "\"Tab\"",
    ] {
        assert!(
            source.contains(needle),
            "kbd docs playgrounds should contain `{needle}`.",
        );
    }
}
