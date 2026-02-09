use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn overlays_module_reexports_overlay_family_contracts() {
    let source = load_source("src/overlays/mod.rs");

    for needle in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            source.contains(needle),
            "overlays module should expose `{needle}` for react-spectrum overlays compatibility."
        );
    }
}

#[test]
fn crate_root_registers_overlays_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod overlays;"),
        "crate root should include `pub mod overlays;` for @react-spectrum/overlays compatibility."
    );
}

#[test]
fn overlays_compatibility_reuses_overlay_popover_modal_tray_docs_playgrounds() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "title=\"Modal\"",
        "slug=\"modal\"",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs should contain `{needle}` for overlays compatibility coverage."
        );
    }

    for needle in ["title=\"Tray\"", "slug=\"tray\"", "<Tray"] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs should contain `{needle}` for Tray compatibility coverage."
        );
    }
}
