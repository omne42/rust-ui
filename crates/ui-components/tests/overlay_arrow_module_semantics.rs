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

#[test]
fn overlay_arrow_module_docs_page_covers_primary_playgrounds() {
    let icon_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let popover_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn icon() -> AnyView",
        "title=\"Icon\"",
        "slug=\"icon\"",
        "description=\"Spectrum-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers.\"",
        "<Playground title=\"Size + Tone Matrix\" code=matrix_code>",
        "<Playground title=\"Accessible + Disabled + Custom Class\" code=states_code>",
        "<Icon",
    ] {
        assert!(
            icon_source.contains(needle),
            "display_extra docs should include `{needle}` for overlay-arrow icon primary playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "<Playground title=\"Popover\" code=code>",
        "title=\"State + Source Markers\"",
        "code=motion_code",
        "<Popover",
    ] {
        assert!(
            popover_source.contains(needle),
            "overlays docs should include `{needle}` for overlay-arrow popover primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"overlay-arrow\" => &[\"icon\", \"popover\"]"),
        "components mod mapping should keep `overlay-arrow` mapped to `icon` and `popover` slugs.",
    );
}

#[test]
fn overlay_arrow_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let icon_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let popover_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Size + Tone Matrix\"",
        "size=IconSize::Sm",
        "tone=IconTone::Default",
        "size=IconSize::Lg",
        "tone=IconTone::Danger",
        "title=\"Accessible + Disabled + Custom Class\"",
        "decorative=false",
        "aria_label=\"Sync successful\".to_string()",
        "disabled=true",
        "class_name=\"docs-icon-custom\".to_string()",
    ] {
        assert!(
            icon_source.contains(needle),
            "overlay-arrow icon docs playgrounds should contain `{needle}`.",
        );
    }

    for needle in [
        "title=\"Popover\"",
        "anchor_ref=anchor_ref",
        "on_exit_complete=on_exit_complete",
        "title=\"State + Source Markers\"",
        "let custom_motion = PopoverMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
    ] {
        assert!(
            popover_source.contains(needle),
            "overlay-arrow popover docs playgrounds should contain `{needle}`.",
        );
    }
}
