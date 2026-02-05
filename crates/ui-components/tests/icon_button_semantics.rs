use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icon_button_does_not_expose_view_module() {
    let source = load_source("src/icon_button/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "IconButton's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn icon_button_requires_accessible_name() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        source.contains("aria_label: String"),
        "IconButton should require an `aria_label` for accessible icon-only buttons."
    );

    assert!(
        source.contains("aria_label=aria_label"),
        "IconButton should forward its `aria_label` to the underlying Button."
    );
}

#[test]
fn icon_button_defaults_to_icon_size() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        source.contains("default = ButtonSize::Icon"),
        "IconButton should default to ButtonSize::Icon to match Spectrum-style icon button sizing."
    );
}

#[test]
fn icon_button_is_thin_wrapper_around_button() {
    let source = load_source("src/icon_button/view.rs");

    for needle in [
        "use crate::button::{Button, ButtonMotion, ButtonSize, ButtonVariant};",
        "<Button",
        "disabled=disabled",
        "variant=variant",
        "size=size",
        "motion=motion",
        "class_name=class_name",
        "button_type=button_type",
        "node_ref=node_ref",
        "on_press=on_press",
    ] {
        assert!(
            source.contains(needle),
            "IconButton should forward `{needle}` to Button to keep semantics consistent and centralized."
        );
    }
}

#[test]
fn icon_button_avoids_branching_on_optional_press_handler() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        !source.contains("match on_press"),
        "IconButton should avoid duplicating Button markup based on whether `on_press` is present."
    );
}

#[test]
fn icon_button_does_not_use_web_sys_directly() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        !source.contains("web_sys"),
        "IconButton should not touch `web_sys` directly; it should delegate behavior to `ui-headless` via Button."
    );
}
