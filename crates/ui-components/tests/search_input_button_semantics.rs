use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_input_button_does_not_expose_logic_module() {
    let source = load_source("src/button_search_input/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "SearchInputButton's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
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
        "data-disabled",
        "data-hovered",
        "data-pressed",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should set `{attr}` to support Spectrum-style styling and state inspection."
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
