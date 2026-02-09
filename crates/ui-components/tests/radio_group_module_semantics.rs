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
