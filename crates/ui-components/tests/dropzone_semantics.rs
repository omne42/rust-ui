use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropzone_does_not_expose_view_module() {
    let source = load_source("src/dropzone/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Dropzone internals should stay private; found `pub mod view`."
    );
}

#[test]
fn dropzone_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/dropzone/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Dropzone;"),
        "dropzone module should export `Dropzone`."
    );
    assert!(
        crate_source.contains("pub use dropzone::Dropzone;"),
        "crate root should re-export `Dropzone`."
    );
}

#[test]
fn dropzone_wraps_drop_zone_contract() {
    let source = load_source("src/dropzone/view.rs");

    for needle in ["pub fn Dropzone(", "<DropZone", "motion: DropZoneMotion"] {
        assert!(
            source.contains(needle),
            "Dropzone wrapper should preserve DropZone contract marker `{needle}`."
        );
    }
}

#[test]
fn dropzone_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/files_extra_dropzone.rs");

    for needle in [
        "pub(super) fn dropzone() -> AnyView",
        "title=\"Dropzone\"",
        "slug=\"dropzone\"",
        "<Dropzone",
    ] {
        assert!(
            source.contains(needle),
            "files_extra_dropzone docs page should contain `{needle}`."
        );
    }
}
