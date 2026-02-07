use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn segmented_control_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/segmented_control/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SegmentedControl internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn segmented_control_uses_headless_hooks() {
    let source = load_source("src/segmented_control/view.rs");

    for needle in ["use_radio_group", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SegmentedControl should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn segmented_control_uses_logic_state_model() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");

    for needle in [
        "pub struct SegmentedControlState",
        "pub fn resolve_state(",
        "pub item_count: usize",
        "pub has_disabled_options: bool",
        "pub selected_index: Option<usize>",
        "pub has_selection: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "aria.selected_index.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn segmented_control_attaches_indicator_motion_driver() {
    let source = load_source("src/segmented_control/view.rs");

    assert!(
        source.contains("attach_indicator_motion"),
        "SegmentedControl should attach a motion driver for the selection indicator (HeroUI-style feel)."
    );
}

#[test]
fn segmented_control_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/segmented_control/view.rs");

    for attr in [
        "data-slot=\"segmented-control\"",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selection-empty=move || state.get().selection_empty.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-orientation=orientation.data_orientation()",
        "data-horizontal=move || state.get().is_horizontal.then_some(\"true\")",
        "data-vertical=move || state.get().is_vertical.then_some(\"true\")",
        "data-has-label=move || state.get().has_label.then_some(\"true\")",
        "data-slot=\"segmented-control-option\"",
        "data-index=index",
        "data-selected=move || is_selected().then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "SegmentedControl should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn segmented_control_sets_aria_orientation_and_option_fallback_label() {
    let source = load_source("src/segmented_control/view.rs");

    for needle in [
        "aria-orientation=orientation.aria_orientation()",
        "format!(\"Option {}\", index + 1)",
    ] {
        assert!(
            source.contains(needle),
            "SegmentedControl should keep `{needle}` for robust ARIA semantics and predictable option labels."
        );
    }
}

#[test]
fn segmented_control_styles_define_indicator_css_vars() {
    let source = load_source("src/segmented_control/styles.rs");

    for var in [
        "--ui-segmented-control-indicator-x",
        "--ui-segmented-control-indicator-y",
        "--ui-segmented-control-indicator-w",
        "--ui-segmented-control-indicator-h",
        "--ui-segmented-control-indicator-o",
    ] {
        assert!(
            source.contains(var),
            "SegmentedControl styles should define `{var}` so motion can update the indicator without re-rendering."
        );
    }
}

#[test]
fn segmented_control_motion_uses_spring_animator() {
    let source = load_source("src/segmented_control/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SegmentedControl motion should be spring-driven to match the repo's motion spec."
    );
}
