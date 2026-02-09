use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dnd_module_reexports_drop_zone_and_file_trigger_contracts() {
    let source = load_source("src/dnd/mod.rs");

    for needle in [
        "pub use crate::drop_zone::{DropZone, DropZoneMotion, DroppedFile};",
        "pub use crate::file_trigger::{FileTrigger, FileTriggerFile, FileTriggerMotion};",
    ] {
        assert!(
            source.contains(needle),
            "dnd module should expose `{needle}` for react-spectrum dnd compatibility."
        );
    }
}

#[test]
fn crate_root_registers_dnd_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod dnd;"),
        "crate root should include `pub mod dnd;` for @react-spectrum/dnd compatibility."
    );
}

#[test]
fn dnd_compatibility_reuses_drop_zone_and_file_trigger_docs_playgrounds() {
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
            "files docs should contain `{needle}` for dnd compatibility coverage."
        );
    }
}
