use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn file_trigger_does_not_expose_logic_module() {
    let source = load_source("src/file_trigger/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "FileTrigger's `logic` module should stay private to avoid leaking DOM/web-sys details into the public API."
    );
}

#[test]
fn file_trigger_clears_input_value_before_click() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("input.set_value(\"\")"),
        "FileTrigger should clear the input value before invoking `click()` so selecting the same file twice still triggers `change`."
    );
}

#[test]
fn file_trigger_forwards_motion_to_button() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("motion=motion.trigger"),
        "FileTrigger should forward its motion contract to the internal Button trigger."
    );
}

#[test]
fn file_trigger_emits_motion_source_markers() {
    let source = load_source("src/file_trigger/view.rs");

    for needle in [
        "data-slot=\"file-trigger\"",
        "data-motion-source=if motion == FileTriggerMotion::default()",
        "data-custom-motion=(motion != FileTriggerMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FileTrigger should expose `{needle}` for Spectrum/HeroUI motion inspection."
        );
    }
}

#[test]
fn file_trigger_input_is_hidden_from_tab_order() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "FileTrigger should set `tabindex=\"-1\"` on the hidden input to avoid it receiving focus."
    );

    assert!(
        source.contains("aria-hidden=\"true\""),
        "FileTrigger should set `aria-hidden=\"true\"` on the hidden input to keep the accessibility tree focused on the trigger."
    );
}

#[test]
fn file_trigger_supports_directory_and_capture_attrs() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("set_attribute(\"webkitdirectory\""),
        "FileTrigger should support directory selection via the `webkitdirectory` attribute."
    );

    assert!(
        source.contains("set_attribute(\"capture\""),
        "FileTrigger should support media capture via the `capture` attribute."
    );
}

#[test]
fn file_trigger_styles_include_motion_marker_contracts() {
    let source = load_source("src/file_trigger/styles.rs");

    for selector in [
        ".ui-file-trigger[data-motion-source=\"custom\"]",
        ".ui-file-trigger[data-custom-motion=\"true\"]",
        ".ui-file-trigger__input",
    ] {
        assert!(
            source.contains(selector),
            "FileTrigger styles should include `{selector}` as stable contracts."
        );
    }
}

#[test]
fn file_trigger_motion_contract_exposes_default_and_custom_trigger_tests() {
    let source = load_source("src/file_trigger/motion.rs");

    for needle in [
        "pub struct FileTriggerMotion",
        "pub trigger: ButtonMotion",
        "fn default_motion_uses_default_button_motion_contract()",
        "fn supports_custom_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "FileTrigger motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}

#[test]
fn file_trigger_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/file_trigger/motion.rs");
    let view_source = load_source("src/file_trigger/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FileTriggerMotion) -> FileTriggerMotion",
        "trigger: crate::button::motion::sanitize_motion(motion.trigger)",
        "fn sanitize_motion_delegates_to_button_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "FileTrigger motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::file_trigger::motion::sanitize_motion(motion);"),
        "FileTrigger view should sanitize motion before forwarding to Button.",
    );
}
