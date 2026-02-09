use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn picker_button_does_not_expose_view_module() {
    let source = load_source("src/picker_button/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "PickerButton internals should stay private; found `pub mod view`."
    );
}

#[test]
fn picker_button_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/picker_button/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::PickerButton;"),
        "picker_button module should export `PickerButton`."
    );
    assert!(
        crate_source.contains("pub use picker_button::PickerButton;"),
        "crate root should re-export `PickerButton`."
    );
}

#[test]
fn picker_button_wraps_field_button_contract() {
    let source = load_source("src/picker_button/view.rs");

    for needle in [
        "pub fn PickerButton(",
        "<FieldButton",
        "on_press: Option<OnPress>",
    ] {
        assert!(
            source.contains(needle),
            "PickerButton wrapper should preserve FieldButton contract marker `{needle}`."
        );
    }
}

#[test]
fn picker_button_docs_page_exists() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/actions_extra_picker_button.rs",
    );

    for needle in [
        "pub(super) fn picker_button() -> AnyView",
        "title=\"PickerButton\"",
        "slug=\"picker-button\"",
        "<PickerButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra_picker_button docs page should contain `{needle}`."
        );
    }
}
