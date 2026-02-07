use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_input_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_search_input/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SearchInputButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn search_input_button_uses_logic_state_model() {
    let view_source = load_source("src/button_search_input/view.rs");
    let logic_source = load_source("src/button_search_input/logic.rs");

    for needle in [
        "pub struct SearchInputButtonState",
        "pub is_enabled: bool",
        "pub has_shortcut: bool",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let placeholder = logic::normalize_optional_text(placeholder);",
        "let compact_placeholder = logic::normalize_optional_text(compact_placeholder);",
        "let state = logic::resolve_state(",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn search_input_button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button_search_input/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SearchInputButton should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn search_input_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_search_input/view.rs");

    for attr in [
        "data-slot=\"search-input-button\"",
        "data-state=if state.is_disabled { \"disabled\" } else { \"enabled\" }",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-shortcut=if state.has_shortcut { \"visible\" } else { \"hidden\" }",
        "data-placeholder=if state.has_custom_placeholder {",
        "data-compact-placeholder=if state.has_custom_compact_placeholder {",
        "data-hovered",
        "data-pressed",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should set `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn search_input_button_forwards_headless_button_semantics() {
    let source = load_source("src/button_search_input/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn search_input_button_uses_fallback_aria_label_from_placeholder() {
    let source = load_source("src/button_search_input/view.rs");

    for needle in [
        "let aria_label = aria_label.unwrap_or_else(|| view_state.placeholder.clone());",
        "let aria_label = StoredValue::new(aria_label);",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton should normalize aria labeling using `{needle}`."
        );
    }
}

#[test]
fn search_input_button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button_search_input/styles.rs");
    let motion = load_source("src/button_search_input/motion.rs");

    for needle in [
        "--ui-search-input-button-scale",
        "transform: scale(var(--ui-search-input-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "SearchInputButton styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("set_property(\"--ui-search-input-button-scale\""),
        "SearchInputButton motion should write `--ui-search-input-button-scale` to drive interaction feedback without triggering rerenders."
    );

    assert!(
        motion.contains("if is_disabled {\n        return;\n    }"),
        "SearchInputButton motion should short-circuit when disabled to avoid unnecessary work and keep disabled visuals stable."
    );
}
