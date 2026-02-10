use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/toggle/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Toggle;"),
        "toggle module should export `Toggle`."
    );
    assert!(
        module_source.contains("ToggleVariant"),
        "toggle module should expose variant alias."
    );
    assert!(
        crate_source.contains("pub use toggle::{Toggle, ToggleMotion, ToggleSize, ToggleVariant};"),
        "crate root should re-export toggle types."
    );
}

#[test]
fn toggle_view_contains_press_and_state_contracts() {
    let source = load_source("src/toggle/view.rs");

    for needle in [
        "data-slot=\"toggle\"",
        "data-slot=\"toggle-label\"",
        "aria-pressed=",
        "data-state=move || if pressed.get()",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "motion::attach_motion",
    ] {
        assert!(
            source.contains(needle),
            "Toggle view should include `{needle}` for stable behavior contracts."
        );
    }
}

#[test]
fn toggle_css_contains_expected_state_selectors() {
    let css = load_source("src/toggle/styles.rs");

    for needle in [
        ".ui-toggle {",
        ".ui-toggle[data-selected=\"true\"]",
        ".ui-toggle[data-motion-source=\"custom\"]",
        ".ui-toggle[data-custom-motion=\"true\"]",
        ".ui-toggle[data-unselected=\"true\"]",
    ] {
        assert!(
            css.contains(needle),
            "Toggle CSS should include `{needle}` selector."
        );
    }
}
