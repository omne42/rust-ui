use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_root_defines_centralized_state_model() {
    let source = load_source("src/root.rs");

    for needle in [
        "pub struct UiRootStateInput",
        "pub struct UiRootState",
        "pub fn resolve_theme_scheme(",
        "pub fn resolve_state(",
    ] {
        assert!(
            source.contains(needle),
            "UiRoot should include `{needle}` for centralized state derivation."
        );
    }
}

#[test]
fn ui_root_derives_state_and_emits_spectrum_style_data_attributes() {
    let source = load_source("src/root.rs");

    for needle in [
        "resolve_state(UiRootStateInput {",
        "data-slot=\"ui-root\"",
        "data-state=move || {",
        "data-theme-scheme=move || state.get().theme_scheme_attr",
        "data-safe-area=move || state.get().has_safe_area.then_some(\"true\")",
        "class:ui-root--safe-area=move || state.get().has_safe_area",
    ] {
        assert!(
            source.contains(needle),
            "UiRoot should expose `{needle}` for stable Spectrum-style state contracts."
        );
    }
}

#[test]
fn ui_root_css_includes_safe_area_state_selectors() {
    let source = load_source("src/root.rs");

    for selector in [
        ".ui-root {",
        ".ui-root--safe-area",
        ".ui-root[data-safe-area=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "UiRoot CSS contract should include `{selector}` for stateful overrides."
        );
    }
}

#[test]
fn ui_root_injects_theme_and_component_css_layers() {
    let source = load_source("src/root.rs");

    for needle in [
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "out.push_str(css::SAFE_AREA_CSS);",
    ] {
        assert!(
            source.contains(needle),
            "UiRoot should keep `{needle}` to inject theme/component/safe-area CSS contracts."
        );
    }
}
