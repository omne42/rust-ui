use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn overlay_arrow_module_reexports_icon_and_placement_contracts() {
    let source = load_source("src/overlay_arrow/mod.rs");

    for needle in [
        "pub use crate::icon::Icon as OverlayArrow;",
        "pub use crate::icon::IconSize as OverlayArrowSize;",
        "pub use crate::icon::IconTone as OverlayArrowTone;",
        "pub use ui_headless::PopoverPlacement as OverlayArrowPlacement;",
    ] {
        assert!(
            source.contains(needle),
            "overlay_arrow module should expose `{needle}` for react-aria-components OverlayArrow compatibility."
        );
    }
}

#[test]
fn crate_root_registers_overlay_arrow_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod overlay_arrow;",
        "pub use overlay_arrow::{OverlayArrow, OverlayArrowPlacement, OverlayArrowSize, OverlayArrowTone};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for overlay-arrow compatibility.",
        );
    }
}

#[test]
fn overlay_arrow_compatibility_reuses_icon_and_popover_docs_playgrounds() {
    let icon_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let popover_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in ["title=\"Icon\"", "slug=\"icon\"", "<Icon"] {
        assert!(
            icon_source.contains(needle),
            "display-extra docs should contain `{needle}` for overlay-arrow compatibility coverage.",
        );
    }

    for needle in ["title=\"Popover\"", "slug=\"popover\"", "<Popover"] {
        assert!(
            popover_source.contains(needle),
            "overlays docs should contain `{needle}` for overlay-arrow compatibility coverage.",
        );
    }
}
