use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn picker_does_not_expose_view_module() {
    let source = load_source("src/picker/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Picker internals should stay private; found `pub mod view`."
    );
}

#[test]
fn picker_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/picker/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Picker;"),
        "picker module should export `Picker`."
    );
    assert!(
        crate_source.contains("pub use picker::Picker;"),
        "crate root should re-export `Picker`."
    );
}

#[test]
fn picker_wraps_select_contract() {
    let source = load_source("src/picker/view.rs");

    for needle in ["pub fn Picker(", "<Select", "placement: PopoverPlacement"] {
        assert!(
            source.contains(needle),
            "Picker wrapper should preserve Select contract marker `{needle}`."
        );
    }
}

#[test]
fn picker_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_picker.rs");

    for needle in [
        "pub(super) fn picker() -> AnyView",
        "title=\"Picker\"",
        "slug=\"picker\"",
        "<Picker",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_picker docs page should contain `{needle}`."
        );
    }
}
