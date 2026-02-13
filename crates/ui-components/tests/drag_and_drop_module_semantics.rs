use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn drag_and_drop_module_reexports_drop_zone_and_file_trigger_contracts() {
    let source = load_source("src/drag_and_drop/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{DragAndDropState, DragAndDropStateInput, compose_class_name, resolve_state};",
        "pub use motion::sanitize_motion;",
        "pub use crate::drop_zone::{DropZone as DragAndDrop, DropZoneMotion as DragAndDropMotion};",
        "pub use crate::drop_zone::{DropZone, DropZoneMotion, DroppedFile};",
        "pub use crate::file_trigger::{FileTrigger, FileTriggerFile, FileTriggerMotion};",
    ] {
        assert!(
            source.contains(needle),
            "drag_and_drop module should expose `{needle}` for react-aria-components DragAndDrop compatibility."
        );
    }
}

#[test]
fn drag_and_drop_motion_contract_delegates_to_drop_zone_sanitizer() {
    let source = load_source("src/drag_and_drop/motion.rs");

    for needle in [
        "pub use crate::drop_zone::DropZoneMotion as DragAndDropMotion;",
        "pub fn sanitize_motion(motion: DragAndDropMotion) -> DragAndDropMotion",
        "crate::drop_zone::motion::sanitize_motion(motion)",
        "pub fn source_attr(motion: DragAndDropMotion) -> &'static str",
    ] {
        assert!(
            source.contains(needle),
            "drag_and_drop motion module should include `{needle}` to provide a stable compatibility motion contract."
        );
    }
}

#[test]
fn drag_and_drop_logic_contract_exposes_state_derivation_helpers() {
    let source = load_source("src/drag_and_drop/logic.rs");

    for needle in [
        "pub struct DragAndDropStateInput",
        "pub struct DragAndDropState",
        "pub fn resolve_state(input: DragAndDropStateInput) -> DragAndDropState",
        "pub fn compose_class_name(class_name: Option<String>) -> String",
    ] {
        assert!(
            source.contains(needle),
            "drag_and_drop logic module should include `{needle}` for stable compatibility-state derivation."
        );
    }
}

#[test]
fn crate_root_registers_drag_and_drop_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod drag_and_drop;",
        "pub use drag_and_drop::{DragAndDrop, DragAndDropMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for drag-and-drop compatibility."
        );
    }
}

#[test]
fn drag_and_drop_compatibility_reuses_drop_zone_and_file_trigger_docs_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "title=\"FileTrigger\"",
        "slug=\"file-trigger\"",
        "<FileTrigger",
        "title=\"DropZone\"",
        "slug=\"drop-zone\"",
        "<DropZone",
    ] {
        assert!(
            source.contains(needle),
            "files docs should contain `{needle}` for drag-and-drop compatibility coverage."
        );
    }
}

#[test]
fn drag_and_drop_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "pub(super) fn file_trigger() -> AnyView",
        "title=\"FileTrigger\"",
        "slug=\"file-trigger\"",
        "<Playground title=\"Pick files\" code_signal=code>",
        "<Playground title=\"Pick files with custom motion\" code_signal=motion_code>",
        "pub(super) fn drop_zone() -> AnyView",
        "title=\"DropZone\"",
        "slug=\"drop-zone\"",
        "<Playground title=\"Drop / paste\" code_signal=code>",
        "<Playground title=\"Drop / paste with custom motion\" code_signal=motion_code>",
    ] {
        assert!(
            source.contains(needle),
            "files docs page should include `{needle}` for drag_and_drop_module primary playground coverage.",
        );
    }
}

#[test]
fn drag_and_drop_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "multiple=true",
        "on_files=on_files",
        "motion=FileTriggerMotion {",
        "hover_scale: 1.04",
        "tap_scale: 0.94",
        "on_files=on_custom_files",
        "\"Pick files (custom motion)\"",
        "label=\"Upload\".to_string()",
        "on_drop_files=on_drop_files",
        "motion=DropZoneMotion {",
        "hover_scale: 1.015",
        "drop_scale: 1.03",
        "hover_highlight: 0.42",
        "label=\"Upload (custom motion)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "drag_and_drop_module docs playgrounds should contain `{needle}`.",
        );
    }
}
