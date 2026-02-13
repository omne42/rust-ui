use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn group_module_reexports_field_group_contracts() {
    let source = load_source("src/group/mod.rs");

    for needle in [
        "pub use crate::field_group::FieldGroup as Group;",
        "pub use crate::field_group::FieldGroupDensity as GroupDensity;",
        "pub use crate::field_group::FieldGroupOrientation as GroupOrientation;",
    ] {
        assert!(
            source.contains(needle),
            "group module should expose `{needle}` for react-aria-components Group compatibility."
        );
    }
}

#[test]
fn crate_root_registers_group_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod group;",
        "pub use group::{Group, GroupDensity, GroupOrientation};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for group compatibility.",
        );
    }
}

#[test]
fn group_compatibility_reuses_field_group_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn field_group() -> AnyView",
        "title=\"FieldGroup\"",
        "slug=\"field-group\"",
        "<FieldGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms-groups docs should contain `{needle}` for Group compatibility coverage.",
        );
    }
}

#[test]
fn group_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn field_group() -> AnyView",
        "title=\"FieldGroup\"",
        "slug=\"field-group\"",
        "description=\"Spectrum/HeroUI-compatible field clustering primitive with centralized orientation/density/aria/class-state contracts and stable slot + data markers.\"",
        "<Playground title=\"Vertical + Label + Description\" code_signal=base_code>",
        "<Playground title=\"Horizontal + Compact + Invalid + Disabled\" code_signal=states_code>",
        "<FieldGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups field_group docs should include `{needle}` for group_module primary playground coverage.",
        );
    }
}

#[test]
fn group_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "title=\"Vertical + Label + Description\"",
        "id_base=\"docs-field-group-account\".to_string()",
        "label=\"Account details\".to_string()",
        "description=\"Group related fields to keep form scanning predictable.\".to_string()",
        "placeholder=\"Ada Lovelace\"",
        "placeholder=\"ada@example.com\"",
        "title=\"Horizontal + Compact + Invalid + Disabled\"",
        "id_base=\"docs-field-group-billing\".to_string()",
        "orientation=FieldGroupOrientation::Horizontal",
        "density=FieldGroupDensity::Compact",
        "invalid=true",
        "disabled=true",
        "class_name=\"docs-field-group-custom\".to_string()",
        "aria_label=\"Billing field cluster\".to_string()",
        "error_message=\"VAT ID is required\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "group module docs playgrounds should contain `{needle}`.",
        );
    }

    assert!(
        mod_source.contains("\"group\" => &[\"field-group\"]"),
        "docs component module mapping should keep `\"group\" => &[\"field-group\"]` for group compatibility contracts.",
    );
}
