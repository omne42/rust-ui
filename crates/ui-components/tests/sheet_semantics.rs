use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sheet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sheet/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sheet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sheet_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/sheet/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use logic::SheetPlacement;",
        "pub use motion::SheetMotion;",
        "pub use view::Sheet;",
        "pub enum SheetSlot",
        "pub struct SheetPartStateInput",
        "pub struct SheetPartState",
    ] {
        assert!(
            module_source.contains(needle),
            "sheet module should include `{needle}` state contracts."
        );
    }

    assert!(
        crate_source.contains("pub use sheet::{Sheet, SheetMotion, SheetPlacement};")
            || (crate_source.contains("pub use sheet::Sheet;")
                && crate_source.contains("pub use sheet::SheetMotion;")
                && crate_source.contains("pub use sheet::SheetPlacement;")),
        "crate root should re-export `Sheet`, `SheetPlacement`, and `SheetMotion` contracts."
    );
}

#[test]
fn sheet_logic_exposes_state_helpers() {
    let source = load_source("src/sheet/logic.rs");

    for needle in [
        "pub const DEFAULT_DISMISSABLE: bool = true;",
        "pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn dismiss_attr(is_dismissable: bool)",
        "pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: SheetPartStateInput)",
        "pub fn compose_class_name(state: SheetPartState)",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            source.contains(needle),
            "Sheet logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn sheet_escape_respects_default_prevented_composition_and_keyboard_flag() {
    let source = load_source("src/sheet/view.rs");

    for needle in [
        "default_prevented",
        "is_composing",
        "logic::should_close_on_escape(",
        "is_keyboard_dismiss_disabled",
        "stop_propagation()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet should include `{needle}` for stable Escape-dismiss behavior."
        );
    }
}

#[test]
fn sheet_view_uses_logic_state_contracts() {
    let source = load_source("src/sheet/view.rs");

    for needle in [
        "logic::normalize_optional_text(aria_labelledby)",
        "logic::normalize_optional_text(aria_describedby)",
        "logic::resolve_state(SheetPartStateInput {",
        "logic::compose_class_name(root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-placement=root_state.placement_attr",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=backdrop_state.slot_attr",
        "data-slot=panel_state.slot_attr",
        "data-state=panel_state.state_attr",
    ] {
        assert!(
            source.contains(needle),
            "Sheet view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn sheet_styles_include_state_and_source_marker_selectors() {
    let source = load_source("src/sheet/styles.rs");

    for needle in [
        ".ui-sheet[data-motion-source=\"custom\"]",
        ".ui-sheet[data-custom-motion=\"true\"]",
        ".ui-sheet[data-placement-source=\"custom\"]",
        ".ui-sheet[data-dismiss-source=\"custom\"]",
        ".ui-sheet[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-sheet[data-aria-labelledby-source=\"custom\"]",
        ".ui-sheet[data-aria-describedby-source=\"custom\"]",
        ".ui-sheet[data-exit-source=\"custom\"]",
        ".ui-sheet[data-custom-exit=\"true\"]",
        ".ui-sheet[data-dismissable=\"true\"] .ui-sheet__backdrop",
        ".ui-sheet[data-keyboard-dismiss-disabled=\"true\"] .ui-sheet__panel",
        ".ui-sheet__backdrop[data-state=\"backdrop\"]",
        ".ui-sheet__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(needle),
            "Sheet styles should include `{needle}` for deterministic marker behavior."
        );
    }
}

#[test]
fn sheet_motion_contract_exposes_default_custom_and_direction_tests() {
    let source = load_source("src/sheet/motion.rs");

    for needle in [
        "pub struct SheetMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_maps_to_sheet_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet motion module should include `{needle}` for HeroUI-style regression coverage."
        );
    }
}

#[test]
fn sheet_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::sheet::styles::CSS);"),
        "ui-components css aggregator should include sheet styles."
    );
}

#[test]
fn sheet_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn sheet() -> AnyView",
        "title=\"Sheet\"",
        "slug=\"sheet\"",
        "State + Source Markers",
        "data-placement-source",
        "<Sheet",
    ] {
        assert!(
            source.contains(needle),
            "sheet docs page should contain `{needle}`."
        );
    }
}

#[test]
fn sheet_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/sheet/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SheetMotion) -> SheetMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_offset_px",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let _ = sanitize_motion(motion);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_offset_range()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn sheet_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = SheetMotion {",
        "initial_offset_px: 56.0",
        "title=\"State + Source Markers\"",
        "placement=SheetPlacement::Right",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=custom_motion",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_marker_exit_complete",
        "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "sheet docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn sheet_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn sheet() -> AnyView",
        "title=\"Sheet\"",
        "slug=\"sheet\"",
        "description=\"Sheet overlay (mobile-friendly) with placement, spring enter/exit, and dismiss control flags.\"",
        "<Playground title=\"Bottom sheet\" code=code>",
        "title=\"State + Source Markers\"",
        "code=marker_code",
        "<Sheet",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for sheet primary playground coverage.",
        );
    }
}

#[test]
fn sheet_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Bottom sheet\"",
        "<Button on_press=open_sheet>\"Open sheet\"</Button>",
        "open=open",
        "placement=SheetPlacement::Bottom",
        "on_close=on_close",
        "on_exit_complete=on_exit_complete",
        "\"Esc/backdrop closes. Focus trap enabled.\"",
        "title=\"State + Source Markers\"",
        "description=\"Inspect `data-state`, `data-placement-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts.\"",
        "<Button on_press=open_marker>\"Open marker sheet\"</Button>",
        "open=marker_open",
        "placement=SheetPlacement::Right",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=custom_motion",
        "on_exit_complete=on_marker_exit_complete",
        "initial_offset_px: 56.0",
        "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for sheet contracts.",
        );
    }
}
