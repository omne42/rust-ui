use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sonner_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sonner/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sonner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sonner_is_publicly_exported_from_module_and_crate_root() {
    let sonner_mod = load_source("src/sonner/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        sonner_mod.contains("pub use view::Sonner;"),
        "sonner::mod should re-export Sonner."
    );
    assert!(
        crate_root.contains("pub use sonner::{Sonner, SonnerPosition};"),
        "crate root should expose Sonner and SonnerPosition."
    );
}

#[test]
fn sonner_exposes_spectrum_style_state_and_accessibility_contracts() {
    let source = load_source("src/sonner/view.rs");

    for needle in [
        "data-slot=\"sonner\"",
        "data-position=state.position_attr",
        "data-portal=state.portal_attr",
        "data-max-toasts=state.max_toasts.to_string()",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "role=\"region\"",
        "aria-label=aria_label",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should include `{needle}` for state/a11y contracts."
        );
    }
}

#[test]
fn sonner_styles_include_motion_marker_contracts() {
    let source = load_source("src/sonner/styles.rs");

    for selector in [
        ".ui-sonner[data-motion-source=\"custom\"]",
        ".ui-sonner[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Sonner styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn sonner_composes_toast_viewport_and_forwards_store_position_classes() {
    let source = load_source("src/sonner/view.rs");

    for needle in [
        "<ToastViewport",
        "store=store",
        "class_name=viewport_class_name",
        "max_toasts=state.max_toasts",
        "portal=state.portal",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should compose ToastViewport using `{needle}`."
        );
    }
}

#[test]
fn sonner_logic_models_positions_and_normalization() {
    let source = load_source("src/sonner/logic.rs");

    for needle in [
        "pub enum SonnerPosition",
        "TopLeft",
        "TopCenter",
        "TopRight",
        "BottomLeft",
        "BottomCenter",
        "BottomRight",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn compose_viewport_class_name(position: SonnerPosition) -> String",
    ] {
        assert!(
            source.contains(needle),
            "Sonner logic should include `{needle}` for stable contracts."
        );
    }
}
