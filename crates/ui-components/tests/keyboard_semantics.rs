use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    if let Some(suffix) = rel_path.strip_prefix("src/keyboard/") {
        workspace_dir.join("components/keyboard/src").join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui-components/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui-components/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui-components/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    }
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn keyboard_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/keyboard/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Keyboard internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn keyboard_uses_logic_state_model() {
    let logic_source = load_source("src/keyboard/logic.rs");
    let render_source = load_source("src/keyboard/view.rs");

    for needle in [
        "pub enum KeyboardTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Keyboard logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(KeyboardStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Keyboard render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn keyboard_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/keyboard/view.rs");

    for attr in [
        "data-slot=\"keyboard\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-compact=move || state.get().is_compact.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Keyboard should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn keyboard_styles_include_tone_compact_and_custom_markers() {
    let source = load_source("src/keyboard/styles.rs");

    for selector in [
        ".ui-keyboard--tone-default",
        ".ui-keyboard[data-tone=\"muted\"]",
        ".ui-keyboard--compact",
        ".ui-keyboard[data-compact=\"true\"]",
        ".ui-keyboard--custom-class",
        ".ui-keyboard[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Keyboard styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn keyboard_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn keyboard() -> AnyView",
        "title=\"Keyboard\"",
        "slug=\"keyboard\"",
        "description=\"Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts.\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Tone / Compact / Source Markers)\"",
        "test_css_source=keyboard_test_css_source",
        "test_source_path=\"components/keyboard/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "<Keyboard",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs should include `{needle}` for keyboard primary playground coverage.",
        );
    }
}

#[test]
fn keyboard_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "id_base=\"docs-keyboard-tone\".to_string()",
        "id_base=\"docs-keyboard-key\".to_string()",
        "\"is_compact\"",
        "\"Custom aria_label\"",
        "\"Custom class_name\"",
        "title=\"Comparison Matrix (Tone / Compact / Source Markers)\"",
        "is_compact=true",
        "aria_label=\"Open command palette\".to_string()",
        "class_name=\"docs-keyboard-custom\".to_string()",
        "\"Ctrl+Shift+P\"",
    ] {
        assert!(
            source.contains(needle),
            "keyboard docs playgrounds should contain `{needle}`.",
        );
    }
}
