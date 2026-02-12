use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn spinbutton_module_reexports_number_field_contract() {
    let source = load_source("src/spinbutton/mod.rs");

    let needle = "pub use crate::number_field::NumberField as SpinButton;";
    assert!(
        source.contains(needle),
        "spinbutton module should expose `{needle}` for @react-aria/spinbutton compatibility.",
    );
}

#[test]
fn crate_root_registers_spinbutton_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in ["pub mod spinbutton;", "pub use spinbutton::SpinButton;"] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for spinbutton compatibility.",
        );
    }
}

#[test]
fn spinbutton_compatibility_reuses_number_field_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn number_field() -> AnyView",
        "title=\"NumberField\"",
        "slug=\"number-field\"",
        "<NumberField",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should contain `{needle}` for spinbutton compatibility coverage.",
        );
    }
}

#[test]
fn spinbutton_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn number_field() -> AnyView",
        "title=\"NumberField\"",
        "slug=\"number-field\"",
        "description=\"Numeric input with steppers and keyboard control.\"",
        "<Playground title=\"Stepper\" code=code>",
        "<NumberField",
    ] {
        assert!(
            source.contains(needle),
            "forms number_field docs should include `{needle}` for spinbutton_module primary playground coverage.",
        );
    }
}

#[test]
fn spinbutton_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Stepper\"",
        "id=\"docs-number-field\".to_string()",
        "label=\"Quantity\".to_string()",
        "value=value",
        "set_value=set_value",
        "min=0",
        "max=100",
        "\"value: \"",
        "{move || value.get().to_string()}",
    ] {
        assert!(
            source.contains(needle),
            "forms number_field playground should contain `{needle}` for spinbutton_module contracts.",
        );
    }
}
