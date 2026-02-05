use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn switch_uses_headless_hooks() {
    let source = load_source("src/switch/view.rs");

    for needle in ["use_switch", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Switch should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn switch_attaches_thumb_motion_driver() {
    let source = load_source("src/switch/view.rs");

    assert!(
        source.contains("attach_thumb_motion"),
        "Switch should attach a motion driver for thumb micro-interactions."
    );
}

#[test]
fn switch_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/switch/view.rs");

    for attr in [
        "data-slot=\"switch\"",
        "data-state",
        "data-pressed",
        "data-hovered",
        "data-focused",
        "data-focus-visible",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "Switch should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn switch_motion_uses_spring_animator() {
    let source = load_source("src/switch/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Switch motion should be spring-driven to match the repo's motion spec."
    );
}
