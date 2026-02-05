use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn segmented_control_uses_headless_hooks() {
    let source = load_source("src/segmented_control/view.rs");

    for needle in ["use_radio_group", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SegmentedControl should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn segmented_control_attaches_indicator_motion_driver() {
    let source = load_source("src/segmented_control/view.rs");

    assert!(
        source.contains("attach_indicator_motion"),
        "SegmentedControl should attach a motion driver for the selection indicator (HeroUI-style feel)."
    );
}

#[test]
fn segmented_control_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/segmented_control/view.rs");

    for attr in [
        "data-slot=\"segmented-control\"",
        "data-disabled",
        "data-slot=\"segmented-control-option\"",
        "data-selected",
        "data-hovered",
        "data-disabled=",
        "data-focused",
        "data-focus-visible",
    ] {
        assert!(
            source.contains(attr),
            "SegmentedControl should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn segmented_control_styles_define_indicator_css_vars() {
    let source = load_source("src/segmented_control/styles.rs");

    for var in [
        "--ui-segmented-control-indicator-x",
        "--ui-segmented-control-indicator-y",
        "--ui-segmented-control-indicator-w",
        "--ui-segmented-control-indicator-h",
        "--ui-segmented-control-indicator-o",
    ] {
        assert!(
            source.contains(var),
            "SegmentedControl styles should define `{var}` so motion can update the indicator without re-rendering."
        );
    }
}

#[test]
fn segmented_control_motion_uses_spring_animator() {
    let source = load_source("src/segmented_control/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SegmentedControl motion should be spring-driven to match the repo's motion spec."
    );
}
