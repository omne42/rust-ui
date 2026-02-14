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
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in ["title=\"UiRoot\"", "slug=\"ui-root\"", "<UiRoot"] {
        assert!(
            source.contains(needle),
            "ui_root docs should contain `{needle}` for story-utils compatibility coverage."
        );
    }
}

#[test]
fn story_utils_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "description=\"Provider that injects theme tokens + layered component CSS and exposes stable root state attrs.\"",
        "<Playground title=\"Usage\" code_signal=usage_code>",
        "<Playground title=\"State Contract\" code_signal=contract_code>",
        "<UiRoot",
    ] {
        assert!(
            source.contains(needle),
            "ui_root docs should include `{needle}` for story_utils_module primary playground coverage.",
        );
    }
}

#[test]
fn story_utils_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/ui_root.rs");

    for needle in [
        "title=\"Usage\"",
        "safe_area=true",
        "This docs app already mounts a global UiRoot at startup.",
        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place.",
        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells.",
        "title=\"State Contract\"",
        "`data-slot=ui-root` for stable root targeting.",
        "`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`).",
        "`data-state` + `data-safe-area` describe safe-area mode.",
        "Use these attrs to write app-level overrides without coupling to internal implementation details.",
    ] {
        assert!(
            source.contains(needle),
            "ui_root playgrounds should contain `{needle}` for story_utils_module contracts.",
        );
    }
}
