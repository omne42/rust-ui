use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn radio_group_module_reexports_shadcn_compatible_names() {
    let source = load_source("src/radio_group/mod.rs");

    for needle in [
        "pub use crate::radio::{RadioGroup, RadioGroupOrientation, RadioMotion};",
        "pub use crate::radio::Radio as RadioGroupItem;",
    ] {
        assert!(
            source.contains(needle),
            "radio_group module should expose `{needle}` for compatibility."
        );
    }
}

#[test]
fn crate_root_registers_radio_group_module_and_item_alias() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod radio_group;",
        "pub use radio_group::RadioGroupItem;",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for radio-group compatibility."
        );
    }
}

#[test]
fn radio_group_docs_page_already_covers_group_and_item_usage() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn radio_group() -> AnyView",
        "title=\"RadioGroup\"",
        "<RadioGroup",
        "<Radio",
    ] {
        assert!(
            source.contains(needle),
            "forms radio_group docs should include `{needle}`."
        );
    }
}

#[test]
fn radio_group_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn radio_group() -> AnyView",
        "title=\"RadioGroup\"",
        "slug=\"radio-group\"",
        "description=\"Roving tabindex radiogroup with HeroUI-level spring motion and Spectrum-style root state attrs.\"",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "<Playground title=\"Horizontal + Disabled + Empty\" code_signal=states_code>",
        "<RadioGroup",
        "<Radio",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for radio-group module primary playground coverage.",
        );
    }
}

#[test]
fn radio_group_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Selection + Root State\"",
        "id_base=\"docs-radio-group\".to_string()",
        "label=\"Size\".to_string()",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "title=\"Horizontal + Disabled + Empty\"",
        "id_base=\"docs-radio-group-billing\".to_string()",
        "orientation=RadioGroupOrientation::Horizontal",
        "disabled_indices=billing_disabled_indices",
        "aria_labelledby=external_label_id.clone()",
        "selected_index=billing_selected",
        "set_selected_index=set_billing_selected",
        "disabled options: 1",
        "id_base=\"docs-radio-group-empty\".to_string()",
        "options=empty_options",
        "disabled=true",
        "aria_label=\"No options available\".to_string()",
        "selected_index=empty_selected",
        "set_selected_index=set_empty_selected",
    ] {
        assert!(
            source.contains(needle),
            "forms docs playgrounds should contain `{needle}` for radio-group module contracts.",
        );
    }
}
