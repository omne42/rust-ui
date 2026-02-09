use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn collapsible_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/collapsible/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Collapsible;"),
        "collapsible module should export `Collapsible`."
    );
    assert!(
        module_source.contains("CollapsibleMotion"),
        "collapsible module should expose a motion contract alias."
    );
    assert!(
        crate_source.contains("pub use collapsible::{Collapsible, CollapsibleMotion};"),
        "crate root should re-export `Collapsible` and `CollapsibleMotion`."
    );
}

#[test]
fn collapsible_view_has_disclosure_style_semantics() {
    let source = load_source("src/collapsible/view.rs");

    for needle in [
        "DisclosureIds::new",
        "motion::attach_panel_motion",
        "data-slot=\"collapsible\"",
        "data-slot=\"collapsible-trigger\"",
        "data-slot=\"collapsible-panel\"",
        "ui-collapsible",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible view should include `{needle}` for disclosure-style behavior and contracts."
        );
    }
}

#[test]
fn collapsible_css_contains_state_markers() {
    let css = load_source("src/collapsible/styles.rs");

    for needle in [
        ".ui-collapsible {",
        ".ui-collapsible[data-open=\"true\"]",
        ".ui-collapsible[data-closed=\"true\"]",
        ".ui-collapsible .ui-disclosure__panel",
    ] {
        assert!(
            css.contains(needle),
            "Collapsible CSS should include `{needle}` selector."
        );
    }
}
