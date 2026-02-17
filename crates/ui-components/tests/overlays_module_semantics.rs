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
            "overlays module should expose `{needle}` for ui-baseline overlays compatibility."
        );
    }
}

#[test]
fn crate_root_registers_overlays_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod overlays;"),
        "crate root should include `pub mod overlays;` for @ui-baseline/overlays compatibility."
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

#[test]
fn overlays_module_docs_page_covers_primary_playgrounds() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn overlay() -> AnyView",
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "<Playground title=\"Overlay presence\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "<Playground title=\"Popover\" code_signal=code>",
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "<Playground title=\"Label + Description\" code_signal=semantic_code>",
        "<Overlay",
        "<Popover",
        "<Modal",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs should include `{needle}` for overlays module primary playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn tray() -> AnyView",
        "title=\"Tray\"",
        "slug=\"tray\"",
        "<Playground title=\"Tray + Footer Actions\" code_signal=semantic_code>",
        "title=\"State + Source Markers\"",
        "<Tray",
    ] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs should include `{needle}` for overlays module tray primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"overlays\" => &[\"overlay\", \"popover\", \"modal\", \"tray\"]"),
        "components mod mapping should keep `overlays` mapped to overlay/popover/modal/tray slugs.",
    );
}

#[test]
fn overlays_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Overlay presence\"",
        "<Button on_press=open_overlay>\"Open overlay\"</Button>",
        "<Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>",
        "title=\"State + Source Markers\"",
        "role=\"alertdialog\"",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=marker_motion",
        "class_name=\"docs-overlay-state\".to_string()",
        "aria_labelledby=\"overlay-marker-title\".to_string()",
        "aria_describedby=\"overlay-marker-desc\".to_string()",
        "on_exit_complete=on_marker_exit_complete",
        "title=\"Popover\"",
        "anchor_ref=anchor_ref",
        "on_exit_complete=on_exit_complete",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
        "title=\"Label + Description\"",
        "id_base=\"docs-modal-semantic\".to_string()",
        "description=\"Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.\".to_string()",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "motion=custom_motion",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for overlays module contracts.",
        );
    }

    for needle in [
        "title=\"Tray + Footer Actions\"",
        "id_base=\"docs-tray-semantic\".to_string()",
        "description=\"Tray composes Sheet with title/description wiring and footer action slots.\".to_string()",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-tray-fixed\".to_string()",
        "motion=custom_motion",
        "is_fixed_height=true",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "show_close_button=false",
        "class_name=\"docs-tray-custom\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
    ] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs playgrounds should contain `{needle}` for overlays module tray contracts.",
        );
    }
}
