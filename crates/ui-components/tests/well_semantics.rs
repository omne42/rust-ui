use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn well_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/well/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Well internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn well_uses_logic_state_model() {
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");

    for needle in [
        "pub enum WellTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Well logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(WellStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Well view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn well_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/well/view.rs");

    for attr in [
        "data-slot=\"well\"",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || if state.get().is_inset { \"inset\" } else { \"default\" }",
        "data-inset=move || state.get().is_inset.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "role=\"region\"",
    ] {
        assert!(
            source.contains(attr),
            "Well should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn well_styles_include_tone_density_and_source_markers() {
    let source = load_source("src/well/styles.rs");

    for selector in [
        ".ui-well--density-comfortable",
        ".ui-well[data-density=\"comfortable\"]",
        ".ui-well--density-compact",
        ".ui-well[data-density=\"compact\"]",
        ".ui-well--tone-default",
        ".ui-well[data-tone=\"default\"]",
        ".ui-well--tone-quiet",
        ".ui-well--tone-strong",
        ".ui-well--inset",
        ".ui-well[data-inset=\"true\"]",
        ".ui-well--label-custom",
        ".ui-well[data-label-source=\"custom\"]",
        ".ui-well--custom-class",
        ".ui-well[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Well styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn well_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn well() -> AnyView",
        "title=\"Well\"",
        "slug=\"well\"",
        "Playground title=\"Tone + Density + Inset\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Well.",
        );
    }
}

#[test]
fn well_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Tone + Density + Inset\"",
        "<Well tone=WellTone::Default>",
        "<Well tone=WellTone::Quiet density=WellDensity::Compact>",
        "<Well tone=WellTone::Strong inset=true>",
        "title=\"Custom Label + Class\"",
        "aria_label=\"Selection summary\".to_string()",
        "class_name=\"docs-well-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "well docs playgrounds should contain `{needle}`.",
        );
    }
}
