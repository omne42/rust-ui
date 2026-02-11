use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_search_input_module_reexports_component_and_motion() {
    let source = load_source("src/button_search_input/mod.rs");

    for needle in [
        "pub use motion::SearchInputButtonMotion;",
        "pub use view::SearchInputButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_search_input module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_search_input_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_search_input;",
        "pub use button_search_input::{SearchInputButton, SearchInputButtonMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_search_input compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_search_input_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"SearchInputButton\"",
        "slug=\"search-input-button\"",
        "<SearchInputButton",
        "placeholder=\"Find components\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for search-input-button coverage.",
        );
    }
}
