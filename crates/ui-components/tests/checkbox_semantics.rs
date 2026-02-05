use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn checkbox_uses_headless_hooks() {
    let source = load_source("src/checkbox/view.rs");

    for needle in ["use_checkbox", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Checkbox should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn checkbox_attaches_motion_drivers() {
    let source = load_source("src/checkbox/view.rs");

    for needle in ["attach_root_motion", "attach_indicator_motion"] {
        assert!(
            source.contains(needle),
            "Checkbox should attach motion driver `{needle}`."
        );
    }
}

#[test]
fn checkbox_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/checkbox/view.rs");

    for attr in [
        "data-slot=\"checkbox\"",
        "data-state",
        "data-hovered",
        "data-pressed",
        "data-focused",
        "data-focus-visible",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "Checkbox should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn checkbox_motion_uses_spring_animator() {
    let source = load_source("src/checkbox/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Checkbox motion should be spring-driven to match the repo's motion spec."
    );
}
