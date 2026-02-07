use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn radio_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/radio/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Radio internals should stay private; found `{needle}`."
        );
    }
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
fn radio_group_supports_accessible_name_resolution() {
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_labelledby: Option<String>",
        "resolve_accessible_name",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "RadioGroup should wire `{needle}` for Spectrum-style accessible naming."
        );
    }

    assert!(
        logic_source.contains("aria_label: Some(\"Radio group\".to_string())"),
        "RadioGroup logic should provide a fallback accessible label when no labels are supplied."
    );
}

#[test]
fn radio_group_exposes_state_and_orientation_data_attributes() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "data-slot=\"radio-group\"",
        "data-disabled=disabled.then_some(\"true\")",
        "data-empty=is_empty.then_some(\"true\")",
        "data-orientation=orientation.data_orientation()",
        "data-has-label=has_label.then_some(\"true\")",
        "data-slot=\"radio\"",
        "data-active=move || (aria.active_index.get() == index).then_some(\"true\")",
        "data-checked",
        "data-focus-visible",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should expose `{needle}` for Spectrum-style state styling and inspection."
        );
    }
}

#[test]
fn radio_group_sets_aria_orientation_and_option_label_fallback() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "aria-orientation=orientation.aria_orientation()",
        "format!(\"Option {}\", index + 1)",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should keep `{needle}` for robust ARIA semantics and predictable labels."
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
fn radio_motion_uses_spring_animator() {
    let source = load_source("src/radio/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Radio motion should be spring-driven to match the repo's motion spec."
    );
}
