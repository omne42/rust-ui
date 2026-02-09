use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toast_does_not_expose_logic_module() {
    let source = load_source("src/toast/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Toast's `logic` module should stay private to avoid leaking store internals into the public API."
    );
}

#[test]
fn toast_is_publicly_exported_from_toast_module_and_crate_root() {
    let toast_mod = load_source("src/toast/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toast_mod.contains("pub use view::{Toast, ToastViewport};"),
        "toast::mod should re-export both Toast and ToastViewport."
    );
    assert!(
        crate_root.contains("Toast, ToastMotion"),
        "crate root should expose Toast together with toast types."
    );
}

#[test]
fn toast_viewport_marks_portaled_content_as_overlay_portal() {
    let source = load_source("src/toast/view.rs");

    assert!(
        source.contains("<Portal>"),
        "ToastViewport should render in a Portal by default."
    );
    assert!(
        source.contains("data-ui-overlay-portal"),
        "ToastViewport portal root should be marked as an overlay portal so modal aria-hidden logic doesn't hide it."
    );
}

#[test]
fn toast_has_spectrum_style_accessibility_semantics() {
    let source = load_source("src/toast/view.rs");

    for needle in [
        "role=\"status\"",
        "aria-live=variant.aria_live()",
        "aria-atomic=\"true\"",
        "aria-label=\"Dismiss toast\"",
    ] {
        assert!(
            source.contains(needle),
            "Toast should include `{needle}` for Spectrum-style accessibility semantics."
        );
    }
}

#[test]
fn toast_has_state_and_description_data_contracts() {
    let source = load_source("src/toast/view.rs");

    for needle in [
        "data-state=",
        "data-description=if has_description { \"present\" } else { \"absent\" }",
        "data-custom-class=has_custom_class_name.then_some(\"true\")",
        "data-open=move || open.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Toast should expose `{needle}` for styling/state contracts."
        );
    }
}

#[test]
fn toast_supports_escape_to_dismiss() {
    let source = load_source("src/toast/view.rs");

    assert!(
        source.contains("if ev.key() == \"Escape\""),
        "Toast should dismiss when Escape is pressed."
    );
}
