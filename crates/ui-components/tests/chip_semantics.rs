use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn chip_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/chip/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Chip internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn chip_uses_logic_state_model() {
    let view_source = load_source("src/chip/view.rs");
    let logic_source = load_source("src/chip/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/chip.rs");

    for needle in [
        "pub use ui_state_primitives::chip::{",
        "ChipStateInput",
        "ChipState",
        "normalize_optional_text",
        "resolve_dismiss_aria_label",
        "resolve_state",
        "pub fn compose_class_name(",
        "ui-chip--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "Chip logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub struct ChipStateInput {",
        "pub struct ChipState {",
        "pub enum ChipVariant {",
        "pub enum ChipSize {",
        "pub fn resolve_dismiss_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Chip logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum ChipVariant",
        "pub enum ChipSize",
        "pub struct ChipStateInput",
        "pub struct ChipState",
        "pub fn resolve_dismiss_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Chip state primitive layer should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_dismiss_aria_label(dismiss_aria_label)",
        "logic::resolve_state(ChipStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Chip view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn chip_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/chip/view.rs");

    for attr in [
        "data-slot=\"chip\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-removable=state.has_dismiss_action.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-dismiss-label-source=state.dismiss_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-slot=\"chip-content\"",
        "data-slot=\"chip-dismiss\"",
        "data-label-source=state.dismiss_label_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Chip should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn chip_styles_include_variant_size_and_state_source_markers() {
    let source = load_source("src/chip/styles.rs");

    for selector in [
        ".ui-chip--size-sm",
        ".ui-chip[data-size=\"md\"]",
        ".ui-chip--variant-danger",
        ".ui-chip[data-variant=\"outline\"]",
        ".ui-chip--enabled",
        ".ui-chip[data-state=\"disabled\"]",
        ".ui-chip[data-state=\"static\"]",
        ".ui-chip[data-state=\"removable\"]",
        ".ui-chip--dismiss-label-custom",
        ".ui-chip[data-dismiss-label-source=\"custom\"]",
        ".ui-chip--custom-class",
        ".ui-chip[data-custom-class=\"true\"]",
        ".ui-chip[data-class-source=\"custom\"]",
        ".ui-chip__dismiss[data-disabled=\"true\"]",
        ".ui-chip__dismiss[data-label-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Chip styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn chip_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn chip() -> AnyView",
        "title=\"Chip\"",
        "slug=\"chip\"",
        "Playground title=\"Removable\"",
        "Playground title=\"Variants + Sizes\"",
        "Playground title=\"Custom Label + Class\"",
        "Playground title=\"Disabled + Static\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Chip.",
        );
    }
}

#[test]
fn chip_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Removable\"",
        "dismiss_aria_label=\"Remove reviewer\".to_string()",
        "title=\"Variants + Sizes\"",
        "<Chip variant=ChipVariant::Default size=ChipSize::Sm>\"Default\"</Chip>",
        "<Chip variant=ChipVariant::Accent size=ChipSize::Md>\"Accent\"</Chip>",
        "<Chip variant=ChipVariant::Danger size=ChipSize::Lg>\"Danger\"</Chip>",
        "title=\"Custom Label + Class\"",
        "dismiss_aria_label=\"  Remove reviewer  \".to_string()",
        "class_name=\"docs-chip-custom\".to_string()",
        "title=\"Disabled + Static\"",
        "<Chip disabled=true variant=ChipVariant::Outline>",
        "<Chip variant=ChipVariant::Default size=ChipSize::Sm>",
    ] {
        assert!(
            source.contains(needle),
            "chip docs playgrounds should contain `{needle}`.",
        );
    }
}
