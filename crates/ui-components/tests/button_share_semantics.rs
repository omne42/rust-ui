use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_share_module_reexports_component_motion_and_types() {
    let source = load_source("src/button_share/mod.rs");

    for needle in [
        "pub use logic::{ShareButtonIconPlacement, ShareButtonItem, SharePlatform};",
        "pub use motion::ShareButtonMotion;",
        "pub use view::ShareButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_share module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_share_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_share;",
        "pub use button_share::{",
        "ShareButton, ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_share compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_share_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "title=\"ShareButton\"",
        "slug=\"share-button\"",
        "<ShareButton",
        "ShareButtonIconPlacement::None",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for share-button coverage.",
        );
    }
}
