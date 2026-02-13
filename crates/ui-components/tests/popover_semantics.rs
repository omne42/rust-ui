use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn popover_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/popover/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Popover internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn popover_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/popover/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Popover;"),
        "popover module should export `Popover`."
    );
    assert!(
        module_source.contains("pub struct PopoverPartStateInput"),
        "popover module should expose `PopoverPartStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use popover::Popover;")
            && crate_source.contains("pub use popover::PopoverMotion;"),
        "crate root should re-export `Popover` and `PopoverMotion` contracts."
    );
}

#[test]
fn popover_logic_exposes_state_helpers() {
    let source = load_source("src/popover/logic.rs");

    for needle in [
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn modal_attr(is_modal: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: PopoverPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: PopoverPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            source.contains(needle),
            "Popover logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn popover_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/popover/view.rs");

    for needle in [
        "default_prevented",
        "is_composing",
        "logic::should_close_on_escape(",
        "stop_propagation()",
    ] {
        assert!(
            source.contains(needle),
            "Popover should include `{needle}` for stable Escape-dismiss behavior."
        );
    }
}

#[test]
fn popover_view_uses_logic_state_contracts() {
    let source = load_source("src/popover/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(PopoverPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-modal=root_state.modal_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-placement=root_state.has_custom_placement.then_some(\"true\")",
        "data-non-modal=(!root_state.is_modal).then_some(\"true\")",
        "data-custom-modal=(!root_state.is_modal).then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=panel_state.slot_attr",
        "data-state=panel_state.state_attr",
        "data-modal=panel_state.modal_attr",
    ] {
        assert!(
            source.contains(needle),
            "Popover view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn popover_styles_include_state_and_source_markers() {
    let source = load_source("src/popover/styles.rs");

    for selector in [
        ".ui-popover[data-motion-source=\"custom\"]",
        ".ui-popover[data-custom-motion=\"true\"]",
        ".ui-popover[data-placement-source=\"custom\"]",
        ".ui-popover--custom-modal",
        ".ui-popover[data-modal-source=\"custom\"]",
        ".ui-popover[data-custom-modal=\"true\"]",
        ".ui-popover[data-modal=\"non-modal\"]",
        ".ui-popover[data-class-source=\"custom\"]",
        ".ui-popover[data-exit-source=\"custom\"]",
        ".ui-popover[data-state=\"open\"]",
        ".ui-popover[data-state=\"closed\"]",
        ".ui-popover__panel[data-state=\"panel\"]",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
        ".ui-popover__panel[data-placement=\"top-end\"]",
    ] {
        assert!(
            source.contains(selector),
            "Popover styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn popover_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::popover::styles::CSS);"),
        "ui-components css aggregator should include popover styles."
    );
}

#[test]
fn popover_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "State + Source Markers",
        "data-modal-source",
        "<Popover",
    ] {
        assert!(
            source.contains(needle),
            "popover docs page should contain `{needle}`."
        );
    }
}

#[test]
fn popover_motion_contract_exposes_default_and_placement_offset_helpers() {
    let mod_source = load_source("src/popover/mod.rs");
    let motion_source = load_source("src/popover/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::PopoverMotion;",
        "pub struct PopoverMotion",
        "fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64",
        "fn default_motion_matches_heroui_style_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Popover motion contract should include `{needle}` for HeroUI-level spring configuration and directional offsets."
        );
    }
}

#[test]
fn popover_motion_contract_sanitizes_custom_values() {
    let source = load_source("src/popover/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: PopoverMotion) -> PopoverMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset()",
    ] {
        assert!(
            source.contains(needle),
            "Popover motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn popover_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = PopoverMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "title=\"State + Source Markers\"",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect `data-modal-source`/`data-placement-source` while tuning PopoverMotion.",
    ] {
        assert!(
            source.contains(needle),
            "popover docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn popover_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "description=\"Positioned portal panel anchored to a trigger with Spectrum-style state markers and HeroUI-grade spring motion contract. Requires presence to unmount after exit.\"",
        "<Playground title=\"Popover\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "code_signal=motion_code",
        "<Popover",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for popover primary playground coverage.",
        );
    }
}

#[test]
fn popover_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Popover\"",
        "anchor_ref=anchor_ref",
        "on_close=on_close",
        "on_exit_complete=on_exit_complete",
        "Positioned via anchor rect + CSS vars.",
        "title=\"State + Source Markers\"",
        "node_ref=custom_anchor_ref",
        "on_press=toggle_custom",
        "open=custom_open",
        "anchor_ref=custom_anchor_ref",
        "on_close=close_custom",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for popover contracts.",
        );
    }
}
