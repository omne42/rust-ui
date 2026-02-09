use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toaster_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toaster/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Toaster internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toaster_is_publicly_exported_from_module_and_crate_root() {
    let toaster_mod = load_source("src/toaster/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toaster_mod.contains("pub use view::Toaster;"),
        "toaster::mod should re-export Toaster."
    );
    assert!(
        crate_root.contains("pub use toaster::{Toaster, ToasterPosition};"),
        "crate root should expose Toaster and ToasterPosition."
    );
}

#[test]
fn toaster_exposes_spectrum_style_state_and_accessibility_contracts() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "data-slot=\"toaster\"",
        "data-position=state.position_attr",
        "data-portal=state.portal_attr",
        "data-max-toasts=state.max_toasts.to_string()",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "role=\"region\"",
        "aria-label=aria_label",
    ] {
        assert!(
            source.contains(needle),
            "Toaster should include `{needle}` for state/a11y contracts."
        );
    }
}

#[test]
fn toaster_composes_sonner_as_host_layer() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "<Sonner",
        "store=store",
        "position=position",
        "class_name=sonner_class_name",
        "max_toasts=state.max_toasts",
    ] {
        assert!(
            source.contains(needle),
            "Toaster should compose Sonner via `{needle}`."
        );
    }
}

#[test]
fn toaster_logic_models_positions_and_normalization() {
    let source = load_source("src/toaster/logic.rs");

    for needle in [
        "pub enum ToasterPosition",
        "TopLeft",
        "TopCenter",
        "TopRight",
        "BottomLeft",
        "BottomCenter",
        "BottomRight",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn compose_toaster_class_name(base_class_name: Option<String>, state: &ToasterState)",
    ] {
        assert!(
            source.contains(needle),
            "Toaster logic should include `{needle}` for stable contracts."
        );
    }
}

#[test]
fn toaster_docs_page_exists_in_overlays_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn toaster() -> AnyView",
        "title=\"Toaster\"",
        "slug=\"toaster\"",
        "<Toaster",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs page should contain `{needle}`."
        );
    }
}
