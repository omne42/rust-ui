use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn bottom_sheet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/bottom_sheet/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "BottomSheet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_uses_logic_state_model() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");

    for needle in [
        "pub struct BottomSheetStateInput",
        "pub struct BottomSheetState",
    ] {
        assert!(
            mod_source.contains(needle),
            "BottomSheet module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_required_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_bottom_inset_px(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, \"Bottom sheet\")",
        "logic::normalize_optional_text(description)",
        "logic::normalize_bottom_inset_px(bottom_inset_px)",
        "logic::resolve_state(BottomSheetStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_composes_sheet_with_bottom_placement_and_motion_contract() {
    let source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "<Sheet",
        "placement=SheetPlacement::Bottom",
        "is_dismissable=is_dismissable",
        "is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet should compose Sheet with stable overlay + motion contracts (`{needle}`)."
        );
    }
}

#[test]
fn bottom_sheet_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/bottom_sheet/view.rs");

    for attr in [
        "data-slot=\"bottom-sheet\"",
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=if motion == BottomSheetMotion::default()",
        "data-custom-motion=(motion != BottomSheetMotion::default()).then_some(\"true\")",
        "data-slot=\"bottom-sheet-handle\"",
        "data-slot=\"bottom-sheet-title\"",
        "data-slot=\"bottom-sheet-description\"",
        "data-slot=\"bottom-sheet-body\"",
        "data-slot=\"bottom-sheet-footer\"",
        "data-bottom-inset=state.inset_attr",
    ] {
        assert!(
            source.contains(attr),
            "BottomSheet should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn bottom_sheet_styles_include_state_marker_contracts() {
    let source = load_source("src/bottom_sheet/styles.rs");

    for selector in [
        ".ui-bottom-sheet[data-motion-source=\"custom\"]",
        ".ui-bottom-sheet[data-custom-motion=\"true\"]",
        ".ui-bottom-sheet--detached",
        ".ui-bottom-sheet[data-detached=\"false\"]",
        ".ui-bottom-sheet--inset-md",
        ".ui-bottom-sheet__handle-bar",
        ".ui-bottom-sheet--close-shown .ui-bottom-sheet__header",
        ".ui-bottom-sheet[data-close-button=\"shown\"] .ui-bottom-sheet__header",
        ".ui-bottom-sheet[data-footer=\"present\"] .ui-bottom-sheet__footer",
        ".ui-bottom-sheet--title-only .ui-bottom-sheet__description",
        ".ui-bottom-sheet--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "BottomSheet styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn bottom_sheet_close_button_contracts_are_preserved() {
    let source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "data-slot=\"bottom-sheet-close\"",
        "<IconButton",
        "aria_label=close_label",
        "on_press=on_close",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet should preserve close button contracts (`{needle}`)."
        );
    }
}

#[test]
fn bottom_sheet_motion_contract_exposes_default_and_custom_sheet_tests() {
    let source = load_source("src/bottom_sheet/motion.rs");

    for needle in [
        "pub struct BottomSheetMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet motion module should include `{needle}` for HeroUI-level contract coverage."
        );
    }
}
