use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn rac_module_exposes_heroui_rac_compat_contract() {
    let source = load_source("src/rac/mod.rs");

    for needle in [
        "pub use crate::DirectionMode as Direction;",
        "pub use crate::DirectionProvider as I18nProvider;",
        "pub use crate::Item as Collection;",
        "pub use crate::ListBoxItem as ListBoxLoadMoreItem;",
        "pub use crate::UiRoot as RouterProvider;",
        "pub fn is_rtl(direction: Direction) -> bool {",
        "pub fn use_locale(direction: Direction) -> &'static str {",
        "pub fn use_filter(value: &str, query: &str) -> bool {",
        "pub fn get_localization_script(direction: Direction) -> String {",
    ] {
        assert!(
            source.contains(needle),
            "rac module should include `{needle}` for HeroUI rac compatibility."
        );
    }
}

#[test]
fn crate_root_registers_rac_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod rac;"),
        "crate root should include `pub mod rac;` for HeroUI rac compatibility."
    );
}

#[test]
fn rac_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in ["title=\"UiRoot\"", "slug=\"ui-root\"", "Theme::dark()"] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for rac compatibility coverage."
        );
    }
}
