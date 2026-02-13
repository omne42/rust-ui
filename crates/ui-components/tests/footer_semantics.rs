use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn footer_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/footer/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Footer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn footer_uses_logic_state_model() {
    let logic_source = load_source("src/footer/logic.rs");
    let render_source = load_source("src/footer/view.rs");

    for needle in [
        "pub enum FooterTone",
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
            "Footer logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FooterStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Footer render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn footer_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/footer/view.rs");

    for attr in [
        "data-slot=\"footer\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Footer should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn footer_styles_include_tone_border_and_custom_markers() {
    let source = load_source("src/footer/styles.rs");

    for selector in [
        ".ui-footer--tone-default",
        ".ui-footer[data-tone=\"default\"]",
        ".ui-footer--tone-muted",
        ".ui-footer[data-tone=\"muted\"]",
        ".ui-footer--bordered",
        ".ui-footer[data-bordered=\"true\"]",
        ".ui-footer--custom-class",
        ".ui-footer[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Footer styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn footer_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn footer() -> AnyView",
        "title=\"Footer\"",
        "slug=\"footer\"",
        "Playground title=\"Semantic Footer + Tone\"",
        "Playground title=\"Bordered + Custom Aria/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Footer.",
        );
    }
}

#[test]
fn footer_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic Footer + Tone\"",
        "<Footer>",
        "<Footer tone=FooterTone::Muted>",
        "title=\"Bordered + Custom Aria/Class\"",
        "<Header bordered=true>",
        "<Content padded=true>",
        "tone=FooterTone::Muted",
        "bordered=true",
        "aria_label=\"Settings footer\".to_string()",
        "class_name=\"docs-footer-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "footer docs playgrounds should contain `{needle}`.",
        );
    }
}
