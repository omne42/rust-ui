use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn view_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/view/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "View internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn view_uses_logic_state_model() {
    let logic_source = load_source("src/view/logic.rs");
    let view_source = load_source("src/view/render.rs");

    for needle in [
        "pub enum ViewBackground",
        "pub enum ViewBorder",
        "pub enum ViewPadding",
        "pub enum ViewRadius",
        "pub enum ViewShadow",
        "pub enum ViewElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "View logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ViewStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "View rendering should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn view_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/view/render.rs");

    for attr in [
        "data-slot=\"view\"",
        "data-element=move || state.get().element_attr",
        "data-background=move || state.get().background_attr",
        "data-border=move || state.get().border_attr",
        "data-padding=move || state.get().padding_attr",
        "data-radius=move || state.get().radius_attr",
        "data-shadow=move || state.get().shadow_attr",
        "data-state=move || state.get().data_state_attr",
        "data-fluid=move || state.get().is_fluid.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "View should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn view_styles_include_surface_and_spacing_markers() {
    let source = load_source("src/view/styles.rs");

    for selector in [
        ".ui-view--element-div",
        ".ui-view[data-element=\"section\"]",
        ".ui-view--background-default",
        ".ui-view[data-background=\"accent\"]",
        ".ui-view--border-none",
        ".ui-view[data-border=\"strong\"]",
        ".ui-view--padding-md",
        ".ui-view[data-padding=\"lg\"]",
        ".ui-view--radius-sm",
        ".ui-view[data-radius=\"lg\"]",
        ".ui-view--shadow-sm",
        ".ui-view[data-shadow=\"md\"]",
        ".ui-view--fluid",
        ".ui-view[data-fluid=\"true\"]",
        ".ui-view--custom-class",
        ".ui-view[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "View styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
