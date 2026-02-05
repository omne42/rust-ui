use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toggle_button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "ToggleButton's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
    assert!(
        !source.contains("pub mod view"),
        "ToggleButton's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn toggle_button_uses_headless_hooks() {
    let source = load_source("src/toggle_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "ToggleButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn toggle_button_attaches_motion_driver() {
    let source = load_source("src/toggle_button/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "ToggleButton should attach a motion driver to match the repo's motion spec."
    );
}

#[test]
fn toggle_button_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/toggle_button/view.rs");

    for attr in [
        "data-slot=\"toggle-button\"",
        "data-selected",
        "data-hovered",
        "data-pressed",
        "data-focused",
        "data-focus-visible",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "ToggleButton should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn toggle_button_styles_define_scale_css_var() {
    let source = load_source("src/toggle_button/styles.rs");

    assert!(
        source.contains("--ui-toggle-button-scale"),
        "ToggleButton styles should define `--ui-toggle-button-scale` so motion can update scale without re-rendering."
    );
}

#[test]
fn toggle_button_motion_uses_spring_animator() {
    let source = load_source("src/toggle_button/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "ToggleButton motion should be spring-driven to match the repo's motion spec."
    );
}
