use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn header_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/header/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Header internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn header_uses_logic_state_model() {
    let logic_source = load_source("src/header/logic.rs");
    let render_source = load_source("src/header/view.rs");

    for needle in [
        "pub enum HeaderTone",
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
            "Header logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(HeaderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Header render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn header_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/header/view.rs");

    for attr in [
        "data-slot=\"header\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Header should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn header_styles_include_tone_border_and_custom_markers() {
    let source = load_source("src/header/styles.rs");

    for selector in [
        ".ui-header--tone-default",
        ".ui-header[data-tone=\"default\"]",
        ".ui-header--tone-strong",
        ".ui-header[data-tone=\"strong\"]",
        ".ui-header--bordered",
        ".ui-header[data-bordered=\"true\"]",
        ".ui-header--custom-class",
        ".ui-header[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Header styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn header_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn header() -> AnyView",
        "title=\"Header\"",
        "slug=\"header\"",
        "Playground title=\"Semantic Header + Tone\"",
        "Playground title=\"Bordered + Custom Aria/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Header.",
        );
    }
}

#[test]
fn header_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic Header + Tone\"",
        "<Header>",
        "<Header tone=HeaderTone::Strong>",
        "title=\"Bordered + Custom Aria/Class\"",
        "tone=HeaderTone::Strong",
        "bordered=true",
        "aria_label=\"Settings header\".to_string()",
        "class_name=\"docs-header-custom\".to_string()",
        "Header above content, matching Spectrum container semantics.",
    ] {
        assert!(
            source.contains(needle),
            "header docs playgrounds should contain `{needle}`.",
        );
    }
}
