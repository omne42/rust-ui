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
