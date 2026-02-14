use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn provider_alias_is_exposed() {
    let source = load_source("src/provider.rs");
    assert!(
        source.contains("pub use ui_components::UiRoot as Provider;"),
        "provider shim should alias UiRoot as Provider"
    );

    let root = load_source("src/lib.rs");
    for needle in ["pub mod provider;", "pub use provider::Provider;"] {
        assert!(
            root.contains(needle),
            "crate root should include `{needle}`"
        );
    }
}

#[test]
fn story_utils_exports_story_theme_contract() {
    let source = load_source("src/story_utils.rs");

    for needle in [
        "pub use ui_components::{Theme, UiRoot};",
        "pub fn story_theme() -> Theme {",
        "Theme::light()",
    ] {
        assert!(
            source.contains(needle),
            "story_utils should include `{needle}`"
        );
    }
}

#[test]
fn test_utils_exports_theme_css_snapshot_contract() {
    let source = load_source("src/test_utils.rs");

    for needle in [
        "pub use ui_components::Theme;",
        "pub fn snapshot_theme_css(theme: Theme) -> String {",
        "theme.to_css_variables()",
    ] {
        assert!(
            source.contains(needle),
            "test_utils should include `{needle}`"
        );
    }
}

#[test]
fn s2_exports_theme_and_root() {
    let source = load_source("src/s2.rs");
    assert!(
        source.contains("pub use ui_components::{Theme, UiRoot};"),
        "s2 shim should re-export Theme and UiRoot"
    );
}

#[test]
fn utils_reexports_headless_hooks() {
    let source = load_source("src/utils.rs");
    for needle in [
        "pub use ui_headless::",
        "use_focus_ring",
        "use_hover",
        "use_press",
    ] {
        assert!(source.contains(needle), "utils should include `{needle}`");
    }
}

#[test]
fn rac_exposes_minimal_compat_surface() {
    let source = load_source("src/rac.rs");
    for needle in [
        "pub use ui_components::DirectionMode as Direction;",
        "pub use ui_components::DirectionProvider as I18nProvider;",
        "pub type Key = String;",
        "pub fn is_rtl(direction: Direction) -> bool {",
        "pub fn use_locale(direction: Direction) -> &'static str {",
        "pub fn use_filter(value: &str, query: &str) -> bool {",
        "pub fn get_localization_script(direction: Direction) -> String {",
        "pub fn direction_data_attr(direction: Direction) -> &'static str {",
    ] {
        assert!(source.contains(needle), "rac should include `{needle}`");
    }
}

#[test]
fn style_macro_s1_can_build_layered_css() {
    let source = load_source("src/style_macro_s1.rs");
    for needle in [
        "pub const STYLE_MACRO_S1_LAYER: &str = \"@layer ui\";",
        "ui_components::push_components_css(&mut css);",
        "pub fn build_s1_layer_css() -> String {",
    ] {
        assert!(
            source.contains(needle),
            "style_macro_s1 should include `{needle}`"
        );
    }
}
