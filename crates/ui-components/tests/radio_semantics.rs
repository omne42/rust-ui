use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn radio_group_uses_headless_roving_and_interaction_hooks() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "use_radio_group",
        "use_focus_ring",
        "use_hover",
        "use_press",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn radio_attaches_motion_driver() {
    let source = load_source("src/radio/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Radio should attach motion via `radio::motion::attach_motion`."
    );
}

#[test]
fn radio_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/radio/view.rs");

    for attr in [
        "data-slot=\"radio-group\"",
        "data-slot=\"radio\"",
        "data-checked",
        "data-disabled",
        "data-hovered",
        "data-pressed",
        "data-focused",
        "data-focus-visible",
    ] {
        assert!(
            source.contains(attr),
            "Radio should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn radio_motion_uses_spring_animator() {
    let source = load_source("src/radio/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Radio motion should be spring-driven to match the repo's motion spec."
    );
}
