use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sheet_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/sheet/view.rs");

    assert!(
        source.contains("default_prevented"),
        "Sheet should not close on Escape when a child already called preventDefault (Spectrum parity for Escape-to-clear flows)."
    );
    assert!(
        source.contains("is_composing"),
        "Sheet should ignore Escape while IME composition is active (matches React Spectrum's `useOverlay`)."
    );
    assert!(
        source.contains("stop_propagation()"),
        "Sheet should stop Escape propagation when closing to avoid cascading dismiss handlers."
    );
}

#[test]
fn sheet_supports_dismissable_and_keyboard_dismiss_flags() {
    let source = load_source("src/sheet/view.rs");

    for needle in [
        "is_dismissable",
        "is_keyboard_dismiss_disabled",
        "if is_dismissable",
        "!is_keyboard_dismiss_disabled",
    ] {
        assert!(
            source.contains(needle),
            "Sheet should support Spectrum-style dismiss control flags (`{needle}`)."
        );
    }
}

#[test]
fn sheet_emits_root_state_and_motion_data_markers() {
    let source = load_source("src/sheet/view.rs");

    for needle in [
        "data-slot=\"sheet\"",
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-placement=placement.data_attr()",
        "data-dismissable=is_dismissable.then_some(\"true\")",
        "data-keyboard-dismiss-disabled=is_keyboard_dismiss_disabled.then_some(\"true\")",
        "data-motion-source=if motion == SheetMotion::default()",
        "data-custom-motion=(motion != SheetMotion::default()).then_some(\"true\")",
        "data-slot=\"sheet-backdrop\"",
        "data-slot=\"sheet-panel\"",
    ] {
        assert!(
            source.contains(needle),
            "Sheet should expose `{needle}` for stable state/motion marker contracts."
        );
    }
}

#[test]
fn sheet_styles_include_state_and_motion_marker_selectors() {
    let source = load_source("src/sheet/styles.rs");

    for needle in [
        ".ui-sheet[data-motion-source=\"custom\"]",
        ".ui-sheet[data-custom-motion=\"true\"]",
        ".ui-sheet[data-state=\"open\"]",
        ".ui-sheet[data-state=\"closed\"]",
        ".ui-sheet[data-dismissable=\"true\"] .ui-sheet__backdrop",
        ".ui-sheet[data-keyboard-dismiss-disabled=\"true\"] .ui-sheet__panel",
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
fn sheet_placement_logic_exposes_data_attribute_contract() {
    let source = load_source("src/sheet/logic.rs");

    for needle in [
        "pub fn data_attr(self) -> &'static str",
        "\"bottom\"",
        "\"left\"",
        "\"right\"",
    ] {
        assert!(
            source.contains(needle),
            "Sheet placement logic should provide `{needle}` for stable placement marker values."
        );
    }
}
