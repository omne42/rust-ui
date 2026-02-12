use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn hover_card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/hover_card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "HoverCard internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn hover_card_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/hover_card/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::HoverCard;"),
        "hover_card module should export `HoverCard`."
    );
    assert!(
        module_source.contains("pub struct HoverCardPartStateInput"),
        "hover_card module should expose `HoverCardPartStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use hover_card::{HoverCard, HoverCardMotion};"),
        "crate root should re-export `HoverCard` contracts."
    );
}

#[test]
fn hover_card_logic_exposes_state_helpers() {
    let source = load_source("src/hover_card/logic.rs");

    for needle in [
        "pub const DEFAULT_OPEN_DELAY_MS: u64 = 140;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64 = 180;",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn has_custom_delays(open_delay_ms: u64, close_delay_ms: u64)",
        "pub fn resolve_part_state(input: HoverCardPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: HoverCardPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
        "pub fn should_handle_escape(key: &str, is_open: bool, is_composing: bool)",
    ] {
        assert!(
            source.contains(needle),
            "HoverCard logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn hover_card_view_uses_logic_state_contracts() {
    let source = load_source("src/hover_card/view.rs");

    for needle in [
        "pub fn HoverCard(",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_id(id, format!(\"ui-hover-card-{}\", next_id()))",
        "logic::has_custom_delays(open_delay_ms, close_delay_ms)",
        "logic::resolve_part_state(HoverCardPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(",
        "logic::should_handle_escape(&ev.key(), open_signal.get_untracked(), is_composing)",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-delay-source=root_state.delay_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-custom-delay=root_state.has_custom_delays.then_some(\"true\")",
        "data-custom-id=root_state.has_custom_id.then_some(\"true\")",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
    ] {
        assert!(
            source.contains(needle),
            "HoverCard view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn hover_card_trigger_stays_non_interactive_wrapper() {
    let source = load_source("src/hover_card/view.rs");

    assert!(
        !source.contains("aria-describedby="),
        "HoverCard should not bind `aria-describedby` directly in markup; attach it to the focused element dynamically."
    );
    assert!(
        source.contains("class=trigger_class"),
        "HoverCard trigger wrapper should use the trigger-class contract."
    );
    assert!(
        source.contains("data-slot=trigger_state.slot_attr"),
        "HoverCard trigger wrapper should keep the trigger slot marker."
    );
    assert!(
        source.contains("<span\n                class=trigger_class"),
        "HoverCard trigger wrapper must remain a non-interactive <span>."
    );
}

#[test]
fn hover_card_styles_include_state_and_source_markers() {
    let source = load_source("src/hover_card/styles.rs");

    for selector in [
        ".ui-hover-card {",
        ".ui-hover-card[data-state=\"open\"]",
        ".ui-hover-card[data-open=\"true\"]",
        ".ui-hover-card[data-state=\"closed\"]",
        ".ui-hover-card[data-closed=\"true\"]",
        ".ui-hover-card[data-class-source=\"custom\"]",
        ".ui-hover-card[data-motion-source=\"custom\"]",
        ".ui-hover-card[data-delay-source=\"custom\"]",
        ".ui-hover-card[data-id-source=\"custom\"]",
        ".ui-hover-card__trigger[data-state=\"trigger\"]",
        ".ui-hover-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(selector),
            "HoverCard styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn hover_card_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::hover_card::styles::CSS);"),
        "ui-components css aggregator should include hover_card styles."
    );
}

#[test]
fn hover_card_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "pub(super) fn hover_card() -> AnyView",
        "title=\"HoverCard\"",
        "slug=\"hover-card\"",
        "State + Source Markers",
        "data-delay-source",
        "<HoverCard",
    ] {
        assert!(
            source.contains(needle),
            "hover card docs page should contain `{needle}`."
        );
    }
}

#[test]
fn hover_card_motion_contract_exposes_default_and_customization_tests() {
    let mod_source = load_source("src/hover_card/mod.rs");
    let motion_source = load_source("src/hover_card/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::HoverCardMotion;",
        "pub struct HoverCardMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "HoverCard motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn hover_card_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/hover_card/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let _ = sanitize_motion(motion);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset_ranges()",
    ] {
        assert!(
            motion_source.contains(needle),
            "HoverCard motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn hover_card_docs_page_includes_custom_motion_contract_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion = HoverCardMotion {",
        "initial_scale: 0.93",
        "offset_y_px: 18.0",
        "motion=custom_motion",
        "motion=HoverCardMotion::default()",
    ] {
        assert!(
            source.contains(needle),
            "hover card docs page should include `{needle}` for custom motion demos."
        );
    }
}

#[test]
fn hover_card_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "open_delay_ms=220",
        "close_delay_ms=260",
        "class_name=\"docs-hover-card-state\".to_string()",
        "id=\"docs-hover-card\".to_string()",
        "motion=HoverCardMotion {",
        "initial_scale: 0.96",
        "offset_y_px: 14.0",
        "Inspect data-delay-source and data-id-source on root/trigger/panel.",
    ] {
        assert!(
            source.contains(needle),
            "hover card docs state/source playground should contain `{needle}`."
        );
    }
}

#[test]
fn hover_card_docs_custom_motion_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion = HoverCardMotion {",
        "initial_scale: 0.93",
        "offset_y_px: 18.0",
        "motion=custom_motion",
        "motion=HoverCardMotion::default()",
        "content=move || view! { \"Custom spring + offset motion\" }",
        "content=move || view! { \"Default motion\" }",
    ] {
        assert!(
            source.contains(needle),
            "hover card docs custom-motion playground should contain `{needle}`."
        );
    }
}
