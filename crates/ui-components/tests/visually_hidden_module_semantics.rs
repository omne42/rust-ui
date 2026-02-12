use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn visually_hidden_module_exposes_component_and_css_contract() {
    let source = load_source("src/visually_hidden/mod.rs");

    for needle in [
        "pub fn VisuallyHidden(",
        "data-slot=\"visually-hidden\"",
        "pub const CSS: &str = r#\"",
        ".ui-visually-hidden--focusable:focus-within",
    ] {
        assert!(
            source.contains(needle),
            "visually_hidden module should include `{needle}` for @react-aria/visually-hidden compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_visually_hidden_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod visually_hidden;",
        "pub use visually_hidden::VisuallyHidden;",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for visually-hidden compatibility.",
        );
    }
}

#[test]
fn visually_hidden_css_is_injected_by_ui_root_aggregation() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::visually_hidden::CSS);"),
        "ui-components css aggregation should include visually_hidden CSS for runtime compatibility.",
    );
}

#[test]
fn visually_hidden_has_dedicated_docs_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    for needle in [
        "title=\"VisuallyHidden\"",
        "slug=\"visually-hidden\"",
        "<VisuallyHidden",
    ] {
        assert!(
            source.contains(needle),
            "forms visually-hidden docs page should contain `{needle}` for compatibility coverage.",
        );
    }
}
