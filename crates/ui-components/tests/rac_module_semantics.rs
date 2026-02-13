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

#[test]
fn rac_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

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
            "layout docs should include `{needle}` for rac module primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"rac\" => &[\"ui-root\"]"),
        "components mod mapping should keep `rac` mapped to `ui-root` slug.",
    );
}

#[test]
fn rac_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Usage\"",
        "let theme = Signal::derive(|| Theme::dark());",
        "<UiRoot theme=theme safe_area=true>",
        "This docs app already mounts a global UiRoot at startup.",
        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place.",
        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells.",
        "title=\"State Contract\"",
        "data-slot=\"ui-root\"",
        "data-theme-scheme=\"light|dark\"",
        "data-state=\"default|safe-area\"",
        "data-safe-area=\"true\"",
        "`data-slot=ui-root` for stable root targeting.",
        "`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`).",
        "`data-state` + `data-safe-area` describe safe-area mode.",
    ] {
        assert!(
            source.contains(needle),
            "layout docs playgrounds should contain `{needle}` for rac module contracts.",
        );
    }
}
