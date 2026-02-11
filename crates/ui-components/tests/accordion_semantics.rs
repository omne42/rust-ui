use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn accordion_does_not_expose_logic_motion_or_view_modules() {
    let source = load_source("src/accordion/mod.rs");

    for needle in ["pub mod logic", "pub mod motion", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Accordion's internal modules should stay private; found `{needle}`."
        );
    }
}

#[test]
fn accordion_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "open_indices: Option<Signal<BTreeSet<usize>>>",
        "default_open_indices: Option<BTreeSet<usize>>",
        "on_open_change: Option<Callback<BTreeSet<usize>>>",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should accept `{needle}` to support controlled/uncontrolled open state."
        );
    }
}

#[test]
fn accordion_uses_headless_hooks() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "use_roving_tabindex",
        "use_press",
        "use_focus_ring",
        "use_hover",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn accordion_attaches_motion_drivers() {
    let source = load_source("src/accordion/view.rs");

    for needle in ["attach_indicator_motion", "attach_panel_motion"] {
        assert!(
            source.contains(needle),
            "Accordion should attach `{needle}` for HeroUI-style spring motion."
        );
    }
}

#[test]
fn accordion_emits_spectrum_style_data_attributes() {
    let source = load_source("src/accordion/view.rs");

    for attr in [
        "data-slot=\"accordion\"",
        "data-disabled=disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-open-count=move || state.get().open_count.to_string()",
        "data-all-closed=move || (!state.get().has_open_items).then_some(\"true\")",
        "data-multiple-open=move || state.get().has_multiple_open.then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-selection-mode=match selection_mode",
        "data-motion-source=if motion == AccordionMotion::default()",
        "data-custom-motion=(motion != AccordionMotion::default()).then_some(\"true\")",
        "data-slot=\"accordion-item\"",
        "data-slot=\"accordion-trigger\"",
        "data-slot=\"accordion-label\"",
        "data-slot=\"accordion-indicator\"",
        "data-slot=\"accordion-panel\"",
        "data-slot=\"accordion-panel-surface\"",
        "data-index=index",
        "data-open",
        "data-hovered",
        "data-pressed",
        "data-focused",
        "data-focus-visible",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "Accordion should set `{attr}` to support Spectrum-style styling and regression testing."
        );
    }
}

#[test]
fn accordion_uses_logic_state_model() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "pub struct AccordionState",
        "pub fn resolve_state(",
        "pub open_count: usize",
        "pub has_disabled_items: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should include `{needle}` for centralized root-state derivation."
        );
    }
    assert!(
        view_source.contains("logic::resolve_state("),
        "Accordion view should derive root state through resolve_state."
    );
    assert!(
        view_source.contains("has_disabled_items"),
        "Accordion state derivation should include disabled-item state."
    );
}

#[test]
fn accordion_panels_are_labeled_regions() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "role=\"region\"",
        "aria-expanded",
        "aria-controls",
        "aria-labelledby=trigger_id",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should wire `{needle}` for accessible disclosure semantics."
        );
    }
}

#[test]
fn accordion_styles_define_motion_css_vars() {
    let source = load_source("src/accordion/styles.rs");

    for var in [
        "--ui-accordion-indicator-rotation",
        "--ui-accordion-panel-height",
        "--ui-accordion-panel-opacity",
        "--ui-accordion-panel-y",
    ] {
        assert!(
            source.contains(var),
            "Accordion styles should define `{var}` so motion can update without re-rendering."
        );
    }

    for selector in [
        ".ui-accordion[data-motion-source=\"custom\"]",
        ".ui-accordion[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Accordion styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn accordion_motion_is_spring_driven() {
    let source = load_source("src/accordion/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Accordion motion should use SpringAnimator to match the motion spec."
    );
}

#[test]
fn accordion_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/accordion/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AccordionMotion) -> AccordionMotion",
        "fn sanitize_spring(value: SpringConfig) -> SpringConfig",
        "indicator_closed_rotation_deg:",
        "indicator_open_rotation_deg:",
        "panel_offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_rotation_and_offset_ranges()",
    ] {
        assert!(
            source.contains(needle),
            "Accordion motion should include `{needle}` so invalid custom values cannot leak into runtime animation state.",
        );
    }
}
