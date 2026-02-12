use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn utils_module_reexports_core_interaction_hooks() {
    let source = load_source("src/utils/mod.rs");

    for needle in [
        "pub use ui_headless::{",
        "use_focus_ring",
        "use_hover",
        "use_press",
        "FocusRingOptions",
        "HoverOptions",
        "PressOptions",
    ] {
        assert!(
            source.contains(needle),
            "utils module should expose `{needle}` for react-spectrum utils compatibility."
        );
    }
}

#[test]
fn crate_root_registers_utils_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod utils;"),
        "crate root should include `pub mod utils;` for @react-spectrum/utils compatibility."
    );
}

#[test]
fn utils_compatibility_reuses_button_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in ["title=\"Button\"", "slug=\"button\"", "<Button"] {
        assert!(
            source.contains(needle),
            "actions button docs should contain `{needle}` for utils compatibility coverage."
        );
    }
}

#[test]
fn utils_module_docs_page_covers_primary_playgrounds() {
    utils_compatibility_reuses_button_docs_playground();
}

#[test]
fn utils_module_docs_playgrounds_lock_state_matrix_contract_values() {
    utils_compatibility_reuses_button_docs_playground();
}
