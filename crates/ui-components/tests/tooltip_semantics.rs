use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tooltip_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tooltip/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tooltip internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tooltip_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/tooltip/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Tooltip;"),
        "tooltip module should export `Tooltip`."
    );
    assert!(
        module_source.contains("pub struct TooltipPartStateInput"),
        "tooltip module should expose `TooltipPartStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use tooltip::Tooltip;")
            && crate_source.contains("pub use tooltip::TooltipMotion;"),
        "crate root should re-export `Tooltip` and `TooltipMotion` contracts."
    );
}

#[test]
fn tooltip_logic_exposes_state_helpers() {
    let source = load_source("src/tooltip/logic.rs");

    for needle in [
        "pub const DEFAULT_DELAY_MS: u64 = 1500;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64 = 500;",
        "pub const DEFAULT_SHOULD_CLOSE_ON_PRESS: bool = true;",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn trigger_attr(trigger: TooltipTriggerMode)",
        "pub fn press_behavior_attr(should_close_on_press: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn has_custom_delays(delay_ms: u64, close_delay_ms: u64)",
        "pub fn resolve_state(input: TooltipPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: TooltipPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64)",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn tooltip_view_uses_logic_state_contracts() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "pub fn Tooltip(",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_id(id, format!(\"ui-tooltip-{}\", next_id()))",
        "logic::has_custom_delays(delay_ms, close_delay_ms)",
        "logic::resolve_state(TooltipPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(position.top_px.get(), position.left_px.get())",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-trigger=root_state.trigger_attr",
        "data-press-behavior=root_state.press_behavior_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-delay-source=root_state.delay_source_attr",
        "data-trigger-source=root_state.trigger_source_attr",
        "data-press-source=root_state.press_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-custom-delay=root_state.has_custom_delays.then_some(\"true\")",
        "data-custom-trigger=root_state.has_custom_trigger_mode.then_some(\"true\")",
        "data-custom-press=root_state.has_custom_press_behavior.then_some(\"true\")",
        "data-custom-id=root_state.has_custom_id.then_some(\"true\")",
        "data-slot=panel_state.slot_attr",
        "data-state=panel_state.state_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn tooltip_uses_headless_trigger_and_position_hooks() {
    let source = load_source("src/tooltip/view.rs");

    for needle in ["use_tooltip_trigger", "use_tooltip_position"] {
        assert!(
            source.contains(needle),
            "Tooltip should use headless `{needle}` hooks for Spectrum-style behavior and positioning."
        );
    }
}

#[test]
fn tooltip_uses_presence_for_exit_motion_unmounting() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "use_presence(open)",
        "presence.finish_exit",
        "motion::attach_motion",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn tooltip_manages_aria_describedby_on_the_focused_element() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "aria-describedby",
        "set_attribute(\"aria-describedby\"",
        "remove_attribute(\"aria-describedby\"",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should manage `aria-describedby` on the focused element via `{needle}` (Spectrum parity)."
        );
    }
}

#[test]
fn tooltip_styles_include_state_and_source_marker_contracts() {
    let source = load_source("src/tooltip/styles.rs");

    for needle in [
        ".ui-tooltip[data-motion-source=\"custom\"]",
        ".ui-tooltip[data-custom-motion=\"true\"]",
        ".ui-tooltip[data-delay-source=\"custom\"]",
        ".ui-tooltip[data-trigger-source=\"custom\"]",
        ".ui-tooltip[data-press-source=\"custom\"]",
        ".ui-tooltip[data-id-source=\"custom\"]",
        ".ui-tooltip[data-state=\"open\"]",
        ".ui-tooltip[data-state=\"closed\"]",
        ".ui-tooltip__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip styles should include `{needle}` for motion/state/source selector contracts."
        );
    }
}

#[test]
fn tooltip_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::tooltip::styles::CSS);"),
        "ui-components css aggregator should include tooltip styles."
    );
}

#[test]
fn tooltip_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "pub(super) fn tooltip() -> AnyView",
        "title=\"Tooltip\"",
        "slug=\"tooltip\"",
        "State + Source Markers",
        "data-delay-source",
        "<Tooltip",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs page should contain `{needle}`."
        );
    }
}

#[test]
fn tooltip_motion_contract_exposes_default_and_custom_test_coverage() {
    let source = load_source("src/tooltip/motion.rs");

    for needle in [
        "pub struct TooltipMotion",
        "fn default_motion_uses_soft_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip motion contract should include `{needle}` for HeroUI-style regression coverage."
        );
    }
}

#[test]
fn tooltip_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/tooltip/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TooltipMotion) -> TooltipMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let _ = sanitize_motion(motion);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset_ranges()",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn tooltip_docs_page_includes_custom_motion_contract_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion = TooltipMotion {",
        "initial_scale: 0.92",
        "offset_y_px: 14.0",
        "motion=custom_motion",
        "motion=TooltipMotion::default()",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs page should include `{needle}` for custom motion demos."
        );
    }
}
