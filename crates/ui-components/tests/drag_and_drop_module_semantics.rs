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
