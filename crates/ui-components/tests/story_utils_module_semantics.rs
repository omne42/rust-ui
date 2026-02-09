use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn story_utils_module_exports_story_theme_and_root_contracts() {
    let source = load_source("src/story_utils/mod.rs");

    for needle in [
        "pub use crate::{Theme, UiRoot};",
        "pub fn story_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "story_utils module should include `{needle}` for @react-spectrum/story-utils compatibility."
        );
    }
}

#[test]
fn crate_root_registers_story_utils_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod story_utils;"),
        "crate root should include `pub mod story_utils;` for @react-spectrum/story-utils compatibility."
    );
}

#[test]
fn story_utils_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in ["title=\"UiRoot\"", "slug=\"ui-root\"", "<UiRoot"] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for story-utils compatibility coverage."
        );
    }
}
