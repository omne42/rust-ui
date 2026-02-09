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
