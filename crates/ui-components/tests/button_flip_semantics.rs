use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_flip_module_reexports_flip_button_contracts() {
    let source = load_source("src/button_flip/mod.rs");

    for needle in [
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_flip module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_flip_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_flip;",
        "pub use button_flip::{FlipButton, FlipButtonMotion, FlipDirection};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_flip compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_flip_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "<FlipButton",
        "FlipDirection::Top",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip-button coverage.",
        );
    }
}
