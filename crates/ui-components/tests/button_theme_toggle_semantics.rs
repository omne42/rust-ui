use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_theme_toggle_module_reexports_component_motion_and_logic_contracts() {
    let source = load_source("src/button_theme_toggle/mod.rs");

    for needle in [
        "pub use logic::{ThemeMode, ThemeToggleViewState, resolve_view_state};",
        "pub use motion::ThemeToggleMotion;",
        "pub use view::ThemeToggleButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_theme_toggle module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_theme_toggle_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_theme_toggle;",
        "pub use button_theme_toggle::{ThemeMode, ThemeToggleButton, ThemeToggleMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_theme_toggle compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_theme_toggle_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn theme_toggle_button() -> AnyView",
        "title=\"ThemeToggleButton\"",
        "slug=\"theme-toggle-button\"",
        "<ThemeToggleButton",
        "ThemeMode::Light",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for theme-toggle-button coverage.",
        );
    }
}
