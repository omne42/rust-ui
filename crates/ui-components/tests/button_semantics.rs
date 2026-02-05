use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_does_not_expose_logic_module() {
    let source = load_source("src/button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Button's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Button should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "data-slot=\"button\"",
        "data-hovered",
        "data-pressed",
        "data-loading",
        "data-loading-placement",
    ] {
        assert!(
            source.contains(attr),
            "Button should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn button_forwards_headless_button_semantics() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Button should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn button_loading_forces_disabled_and_sets_aria_busy() {
    let source = load_source("src/button/view.rs");

    assert!(
        source.contains("resolve_state"),
        "Button should normalize `disabled`/`is_loading` via `resolve_state` to keep the contract testable and consistent."
    );

    for needle in [
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Button should wire loading/disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button/styles.rs");
    let motion = load_source("src/button/motion.rs");

    for needle in [
        "--ui-button-scale",
        "transform: scale(var(--ui-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "Button styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("--ui-button-scale"),
        "Button motion should write `--ui-button-scale` to drive interaction feedback without triggering rerenders."
    );
}

#[test]
fn button_spinner_respects_reduced_motion() {
    let styles = load_source("src/button/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "Button spinner should disable its CSS animation under reduced-motion via `{needle}`."
        );
    }
}
