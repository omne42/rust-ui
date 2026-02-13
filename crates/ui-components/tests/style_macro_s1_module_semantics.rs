use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn style_macro_s1_module_exports_css_builder_contract() {
    let source = load_source("src/style_macro_s1/mod.rs");

    for needle in [
        "pub use crate::css::push_components_css;",
        "pub fn build_s1_layer_css() -> String {",
        "push_components_css(&mut css);",
    ] {
        assert!(
            source.contains(needle),
            "style_macro_s1 module should include `{needle}` for @react-spectrum/style-macro-s1 compatibility."
        );
    }
}

#[test]
fn crate_root_registers_style_macro_s1_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod style_macro_s1;"),
        "crate root should include `pub mod style_macro_s1;` for @react-spectrum/style-macro-s1 compatibility."
    );
}

#[test]
fn style_macro_s1_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "UiRoot injects BASE_CSS",
    ] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for style-macro-s1 compatibility coverage."
        );
    }
}

#[test]
fn style_macro_s1_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

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
            "layout ui_root docs should include `{needle}` for style_macro_s1_module primary playground coverage.",
        );
    }
}

#[test]
fn style_macro_s1_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

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
            "layout ui_root playgrounds should contain `{needle}` for style_macro_s1_module contracts.",
        );
    }
}
