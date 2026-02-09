use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn textfield_does_not_expose_view_module() {
    let source = load_source("src/textfield/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Textfield internals should stay private; found `pub mod view`."
    );
}

#[test]
fn textfield_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/textfield/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Textfield;"),
        "textfield module should export `Textfield`."
    );
    assert!(
        crate_source.contains("pub use textfield::Textfield;"),
        "crate root should re-export `Textfield`."
    );
}

#[test]
fn textfield_wraps_text_field_contract() {
    let source = load_source("src/textfield/view.rs");

    for needle in ["pub fn Textfield(", "<TextField", "required: Signal<bool>"] {
        assert!(
            source.contains(needle),
            "Textfield wrapper should preserve TextField contract marker `{needle}`."
        );
    }
}

#[test]
fn textfield_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_textfield.rs");

    for needle in [
        "pub(super) fn textfield() -> AnyView",
        "title=\"Textfield\"",
        "slug=\"textfield\"",
        "<Textfield",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_textfield docs page should contain `{needle}`."
        );
    }
}
