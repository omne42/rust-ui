use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn hidden_date_input_module_reexports_date_input_group_contracts() {
    let source = load_source("src/hidden_date_input/mod.rs");

    for needle in [
        "pub use crate::date_input_group::DateInputGroup as HiddenDateInput;",
        "pub use crate::date_input_group::DateInputGroupVariant as HiddenDateInputVariant;",
    ] {
        assert!(
            source.contains(needle),
            "hidden_date_input module should expose `{needle}` for react-aria-components HiddenDateInput compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_hidden_date_input_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod hidden_date_input;",
        "pub use hidden_date_input::{HiddenDateInput, HiddenDateInputVariant};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for hidden-date-input compatibility.",
        );
    }
}

#[test]
fn hidden_date_input_compatibility_reuses_date_input_group_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "<DateInputGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms-groups docs should contain `{needle}` for hidden-date-input compatibility coverage.",
        );
    }
}
