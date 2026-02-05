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
        "data-slot=\"accordion-item\"",
        "data-slot=\"accordion-trigger\"",
        "data-slot=\"accordion-label\"",
        "data-slot=\"accordion-indicator\"",
        "data-slot=\"accordion-panel\"",
        "data-slot=\"accordion-panel-surface\"",
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
}

#[test]
fn accordion_motion_is_spring_driven() {
    let source = load_source("src/accordion/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Accordion motion should use SpringAnimator to match the motion spec."
    );
}
