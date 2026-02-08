use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tray_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tray/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tray internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tray_uses_logic_state_model() {
    let mod_source = load_source("src/tray/mod.rs");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");

    for needle in ["pub struct TrayStateInput", "pub struct TrayState"] {
        assert!(
            mod_source.contains(needle),
            "Tray module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_required_text(",
        "pub fn normalize_id_base(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tray logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, \"Tray\")",
        "logic::normalize_optional_text(description)",
        "logic::resolve_state(TrayStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn tray_composes_sheet_with_bottom_placement_and_motion_contract() {
    let source = load_source("src/tray/view.rs");

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
            "Tray should compose Sheet with stable overlay + motion contracts (`{needle}`)."
        );
    }
}

#[test]
fn tray_only_sets_describedby_when_description_exists() {
    let source = load_source("src/tray/view.rs");

    assert!(
        source.contains("if state.show_description"),
        "Tray should branch on description presence so `aria-describedby` is only set when needed."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "id=move || description_id_attr.get()",
    ] {
        assert!(
            source.contains(needle),
            "Tray should wire description ids only on described path (`{needle}`)."
        );
    }
}

#[test]
fn tray_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/tray/view.rs");

    for attr in [
        "data-slot=\"tray\"",
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-close-button=state.close_button_attr",
        "data-size=state.size_attr",
        "data-fixed-height=state.is_fixed_height.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"tray-header\"",
        "data-slot=\"tray-body\"",
        "data-slot=\"tray-footer\"",
    ] {
        assert!(
            source.contains(attr),
            "Tray should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn tray_styles_include_state_marker_contracts() {
    let source = load_source("src/tray/styles.rs");

    for selector in [
        ".ui-tray--fixed-height",
        ".ui-tray[data-size=\"auto\"]",
        ".ui-tray--with-description",
        ".ui-tray[data-state=\"title-only\"]",
        ".ui-tray--close-shown .ui-tray__header",
        ".ui-tray[data-close-button=\"shown\"] .ui-tray__header",
        ".ui-tray[data-footer=\"present\"] .ui-tray__footer",
        ".ui-tray--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Tray styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn tray_close_button_contracts_are_preserved() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "data-slot=\"tray-close\"",
        "<IconButton",
        "aria_label=close_label",
        "on_press=on_close",
    ] {
        assert!(
            source.contains(needle),
            "Tray should preserve close button contracts (`{needle}`)."
        );
    }
}
