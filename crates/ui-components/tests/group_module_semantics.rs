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
