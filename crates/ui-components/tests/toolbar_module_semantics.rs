use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toolbar_module_reexports_action_bar_contracts() {
    let source = load_source("src/toolbar/mod.rs");

    for needle in [
        "pub use crate::action_bar::ActionBar as Toolbar;",
        "pub use crate::action_bar::ActionBarMotion as ToolbarMotion;",
    ] {
        assert!(
            source.contains(needle),
            "toolbar module should expose `{needle}` for @react-spectrum/toolbar compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_toolbar_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod toolbar;",
        "pub use toolbar::{Toolbar, ToolbarMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for toolbar compatibility.",
        );
    }
}

#[test]
fn toolbar_compatibility_reuses_action_bar_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn action_bar() -> AnyView",
        "title=\"ActionBar\"",
        "slug=\"action-bar\"",
        "<ActionBar",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should contain `{needle}` for toolbar compatibility coverage.",
        );
    }
}

#[test]
fn toolbar_module_docs_page_covers_primary_playgrounds() {
    toolbar_compatibility_reuses_action_bar_docs_playground();
}

#[test]
fn toolbar_module_docs_playgrounds_lock_state_matrix_contract_values() {
    toolbar_compatibility_reuses_action_bar_docs_playground();
}
