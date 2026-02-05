use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn illustrated_message_does_not_expose_logic_module() {
    let source = load_source("src/illustrated_message/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "IllustratedMessage's `logic` module should stay private to avoid leaking internal view-state helpers into the public API."
    );
}

#[test]
fn illustrated_message_emits_expected_data_slots() {
    let source = load_source("src/illustrated_message/view.rs");

    for attr in [
        "data-slot=\"illustrated-message\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "IllustratedMessage should set `{attr}` for Spectrum-style styling and inspection."
        );
    }
}

#[test]
fn illustrated_message_uses_spring_driven_opacity_and_y_css_vars() {
    let styles = load_source("src/illustrated_message/styles.rs");
    let motion = load_source("src/illustrated_message/motion.rs");

    for needle in [
        "--ui-im-opacity",
        "--ui-im-y",
        "opacity: var(--ui-im-opacity)",
        "transform: translateY(var(--ui-im-y))",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should reference `{needle}` for spring-driven enter motion."
        );
    }

    for needle in ["--ui-im-opacity", "--ui-im-y"] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should write `{needle}` to drive enter animation."
        );
    }
}

#[test]
fn illustrated_message_attaches_motion_driver() {
    let source = load_source("src/illustrated_message/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "IllustratedMessage should attach its motion driver from `motion.rs`."
    );
}
