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

#[test]
fn hidden_date_input_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn date_input_group() -> AnyView",
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "description=\"Spectrum/HeroUI-style date-input grouping primitive with centralized variant/width/prefix-suffix state contracts and segmented slot markers.\"",
        "<Playground title=\"DateField + Prefix/Suffix\" code_signal=code>",
        "<Playground title=\"Secondary + Full Width + Invalid\" code_signal=states_code>",
        "<DateInputGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups date_input_group docs should include `{needle}` for hidden_date_input_module primary playground coverage.",
        );
    }
}

#[test]
fn hidden_date_input_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "title=\"DateField + Prefix/Suffix\"",
        "aria_label=\"Invoice date controls\".to_string()",
        "segmented=true",
        "id_base=\"docs-date-input-group-invoice\".to_string()",
        "label=\"Invoice date\".to_string()",
        "tone=DateFieldTone::Quiet",
        "title=\"Secondary + Full Width + Invalid\"",
        "full_width=true",
        "variant=DateInputGroupVariant::Secondary",
        "invalid=true",
        "aria_label=\"Ship window controls\".to_string()",
        "class_name=\"docs-date-input-group-custom\".to_string()",
        "id_base=\"docs-date-input-group-time\".to_string()",
        "label=\"Ship window\".to_string()",
        "tone=TimeFieldTone::Strong",
        "minute_step=5",
    ] {
        assert!(
            source.contains(needle),
            "hidden_date_input module docs playgrounds should contain `{needle}`.",
        );
    }

    assert!(
        mod_source.contains("\"hidden-date-input\" => &[\"date-input-group\"]"),
        "docs component module mapping should keep `\"hidden-date-input\" => &[\"date-input-group\"]` for hidden-date-input compatibility contracts.",
    );
}
