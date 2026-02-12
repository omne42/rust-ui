use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn number_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/number/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Number internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn number_module_exports_primitives_and_motion_contracts() {
    let source = load_source("src/number/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use logic::{NumberFormatOptions, format_static_number};",
        "pub use motion::SlidingNumberMotion;",
        "pub use view::{SlidingNumber, StaticNumber};",
    ] {
        assert!(
            source.contains(needle),
            "number module should expose `{needle}`.",
        );
    }

    for needle in [
        "pub mod number;",
        "pub use number::{NumberFormatOptions, SlidingNumber, SlidingNumberMotion, StaticNumber};",
    ] {
        assert!(
            crate_source.contains(needle),
            "crate root should include `{needle}` for number contracts.",
        );
    }
}

#[test]
fn number_logic_exposes_format_state_helpers() {
    let source = load_source("src/number/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_decimal_separator(",
        "pub fn resolve_thousand_separator(",
        "pub fn sanitize_decimal_places(",
        "pub fn sanitize_number(",
        "pub fn resolve_static_number_state(",
        "pub fn compose_static_number_class_name(",
        "pub fn resolve_sliding_number_state(",
        "pub fn compose_sliding_number_class_name(",
        "pub fn format_static_number(value: f64, options: NumberFormatOptions<'_>) -> String",
        "DEFAULT_DECIMAL_SEPARATOR",
    ] {
        assert!(
            source.contains(needle),
            "Number logic should include `{needle}` for centralized formatting/state derivation.",
        );
    }
}

#[test]
fn number_view_wires_motion_sanitization_and_state_markers() {
    let source = load_source("src/number/view.rs");

    for needle in [
        "let motion = crate::number::motion::sanitize_motion(motion);",
        "logic::resolve_static_number_state(logic::StaticNumberStateInput {",
        "logic::resolve_sliding_number_state(logic::SlidingNumberStateInput {",
        "data-slot=\"static-number\"",
        "data-slot=\"sliding-number\"",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-decimal-separator-source=state.decimal_separator_source_attr",
        "data-decimal-places-source=state.decimal_places_source_attr",
        "data-thousand-separator-source=state.thousand_separator_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Number view should include `{needle}` for stable state/motion marker contracts.",
        );
    }
}

#[test]
fn number_motion_contract_defaults_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/number/motion.rs");

    for needle in [
        "pub struct SlidingNumberMotion",
        "ui_motion::presets::spring_slide()",
        "animate: true",
        "motion.animate && !ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn supports_custom_spring_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Number motion contract should include `{needle}` for HeroUI-level spring/reduced-motion stability.",
        );
    }
}

#[test]
fn number_styles_and_css_aggregation_include_stable_selectors() {
    let styles_source = load_source("src/number/styles.rs");
    let css_source = load_source("src/css.rs");

    for selector in [
        ".ui-static-number",
        ".ui-static-number[data-sign=\"negative\"]",
        ".ui-static-number[data-decimal-separator-source=\"custom\"]",
        ".ui-static-number[data-thousand-separator-source=\"custom\"]",
        ".ui-sliding-number",
        ".ui-sliding-number[data-state=\"animated\"]",
        ".ui-sliding-number[data-motion-source=\"custom\"]",
        ".ui-sliding-number__roller",
        ".ui-sliding-number__stack",
    ] {
        assert!(
            styles_source.contains(selector),
            "Number styles should include `{selector}` as a stable marker.",
        );
    }

    assert!(
        css_source.contains("out.push_str(crate::number::styles::CSS);"),
        "ui-components css aggregator should include number styles.",
    );
}

#[test]
fn number_docs_page_contains_static_and_sliding_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "title=\"StaticNumber\"",
        "slug=\"static-number\"",
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"SlidingNumber\"",
        "slug=\"sliding-number\"",
        "<StaticNumber",
        "<SlidingNumber",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for number playground coverage.",
        );
    }
}

#[test]
fn number_docs_static_number_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "<Playground title=\"Formatting Matrix\" code=matrix_code>",
        "number=12345.67",
        "number=-9876.5",
        "number=1000.0",
        "decimal_places=2",
        "decimal_places=1",
        "decimal_places=0",
        "thousand_separator=\",\".to_string()",
        "<Playground title=\"Custom Separators + Class\" code=custom_code>",
        "number=42.123456789",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "number=f64::NAN",
        "class_name=\"docs-static-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number static docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_docs_sliding_number_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn sliding_number() -> AnyView",
        "<Playground title=\"Animated Matrix\" code=matrix_code>",
        "number=number_signal",
        "decimal_places=2",
        "thousand_separator=\",\".to_string()",
        "decimal_places=0",
        "set_value.update(|v| *v += 250.0)",
        "set_value.update(|v| *v -= 100.0)",
        "<Playground title=\"Custom Separators + Motion + Class\" code=custom_code>",
        "number=Signal::derive(|| 42123.456)",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "motion=ui_components::SlidingNumberMotion {",
        "animate: false,",
        "number=Signal::derive(|| f64::NAN)",
        "class_name=\"docs-sliding-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number sliding docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "title=\"StaticNumber\"",
        "slug=\"static-number\"",
        "<Playground title=\"Formatting Matrix\" code=matrix_code>",
        "<Playground title=\"Custom Separators + Class\" code=custom_code>",
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"SlidingNumber\"",
        "slug=\"sliding-number\"",
        "<Playground title=\"Animated Matrix\" code=matrix_code>",
        "<Playground title=\"Custom Separators + Motion + Class\" code=custom_code>",
        "<StaticNumber",
        "<SlidingNumber",
    ] {
        assert!(
            source.contains(needle),
            "display docs should include `{needle}` for number primary playground coverage.",
        );
    }
}

#[test]
fn number_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Formatting Matrix\"",
        "number=12345.67",
        "number=-9876.5",
        "decimal_places=2",
        "thousand_separator=\",\".to_string()",
        "title=\"Custom Separators + Class\"",
        "decimal_separator=\",\".to_string()",
        "thousand_separator=\" \".to_string()",
        "class_name=\"docs-static-number-custom\".to_string()",
        "title=\"Animated Matrix\"",
        "number=number_signal",
        "set_value.update(|v| *v += 250.0)",
        "set_value.update(|v| *v -= 100.0)",
        "title=\"Custom Separators + Motion + Class\"",
        "motion=ui_components::SlidingNumberMotion {",
        "animate: false,",
        "class_name=\"docs-sliding-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number docs playgrounds should contain `{needle}`.",
        );
    }
}
