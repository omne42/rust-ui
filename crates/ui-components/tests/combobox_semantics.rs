use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn combobox_does_not_expose_view_module() {
    let source = load_source("src/combobox/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Combobox internals should stay private; found `pub mod view`."
    );
}

#[test]
fn combobox_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/combobox/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Combobox;"),
        "combobox module should export `Combobox`."
    );
    assert!(
        crate_source.contains("pub use combobox::Combobox;"),
        "crate root should re-export `Combobox`."
    );
}

#[test]
fn combobox_wraps_combo_box_contract() {
    let source = load_source("src/combobox/view.rs");

    for needle in ["pub fn Combobox(", "<ComboBox", "motion: ComboBoxMotion"] {
        assert!(
            source.contains(needle),
            "Combobox wrapper should preserve ComboBox contract marker `{needle}`."
        );
    }
}

#[test]
fn combobox_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "pub(super) fn combobox() -> AnyView",
        "title=\"Combobox\"",
        "slug=\"combobox\"",
        "<Combobox",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_combobox docs page should contain `{needle}`."
        );
    }
}
