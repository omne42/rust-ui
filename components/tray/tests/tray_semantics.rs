use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mapped = match rel_path {
        "src/css.rs" => "../../crates/ui/src/css.rs".to_string(),
        _ if rel_path.starts_with("src/tray/") => {
            format!("src/{}", &rel_path["src/tray/".len()..])
        }
        _ => rel_path.to_string(),
    };
    let path = manifest_dir.join(mapped);
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
fn tray_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/tray/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use motion::TrayMotion;",
        "pub use view::Tray;",
        "pub use ui_state_primitives::tray::{TrayPartState, TrayPartStateInput, TraySlot};",
    ] {
        assert!(
            module_source.contains(needle),
            "tray module should include `{needle}` state contracts."
        );
    }

    assert!(
        crate_source.contains("pub use tray::{Tray, TrayMotion};")
            || (crate_source.contains("pub use tray::Tray;")
                && crate_source.contains("pub use tray::TrayMotion;")),
        "crate root should re-export `Tray` and `TrayMotion` contracts."
    );
}

#[test]
fn tray_logic_exposes_state_helpers() {
    let source = load_source("src/tray/logic.rs");

    for needle in [
        "pub use ui_state_primitives::tray::{",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_SHOW_CLOSE_BUTTON",
        "DEFAULT_FIXED_HEIGHT",
        "DEFAULT_DISMISSABLE",
        "DEFAULT_KEYBOARD_DISMISS_DISABLED",
        "normalize_optional_text",
        "normalize_required_text",
        "normalize_id_base",
        "resolve_state",
        "pub fn compose_class_name(base_class_name: Option<String>, state: TrayPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Tray logic should include `{needle}` while consuming state primitives from ui-state-primitives."
        );
    }
}

#[test]
fn tray_composes_sheet_with_bottom_placement_and_motion_contract() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "<Sheet",
        "placement=SheetPlacement::Bottom",
        "aria_labelledby=panel_aria_labelledby.get_value()",
        "aria_describedby=panel_aria_describedby.get_value()",
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
fn tray_view_uses_logic_state_contracts() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "let state_inputs = logic::TrayStateInputs {",
        "let resolved_states = logic::resolve_states(state_inputs);",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-size=root_state.size_attr",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-size-source=root_state.size_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-footer=(root_state.footer_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-close=(root_state.close_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-size=(root_state.size_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-dismiss=(root_state.dismiss_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-keyboard-dismiss=(root_state.keyboard_dismiss_source_attr == \"custom\").then_some(\"true\")",
        "data-slot=header_state.slot_attr",
        "data-slot=title_state.slot_attr",
        "data-slot=body_state.slot_attr",
        "data-slot=footer_state.slot_attr",
        "data-slot=close_state.slot_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tray view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn tray_uses_headless_overlay_a11y_contract() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, overlay_dialog_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let panel_a11y = overlay_dialog_attrs(",
        "root_state.show_description",
        ".then_some(description_id.clone())",
        "let panel_aria_labelledby = StoredValue::new(panel_a11y.aria_labelledby);",
        "let panel_aria_describedby = StoredValue::new(panel_a11y.aria_describedby);",
        "let panel_lang = StoredValue::new(panel_a11y.lang);",
        "let panel_dir = panel_a11y.dir;",
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=panel_aria_describedby.get_value()",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tray should consume typed overlay A11y attrs from ui-headless (`{needle}`)."
        );
    }
}

#[test]
fn tray_styles_include_state_and_source_markers() {
    let source = load_source("src/tray/styles.rs");

    for selector in [
        ".ui-tray[data-motion-source=\"custom\"]",
        ".ui-tray[data-custom-motion=\"true\"]",
        ".ui-tray--custom-description",
        ".ui-tray[data-custom-description=\"true\"]",
        ".ui-tray[data-description-source=\"custom\"]",
        ".ui-tray--custom-footer",
        ".ui-tray[data-custom-footer=\"true\"]",
        ".ui-tray[data-footer-source=\"custom\"]",
        ".ui-tray--custom-close",
        ".ui-tray[data-custom-close=\"true\"]",
        ".ui-tray[data-close-source=\"custom\"]",
        ".ui-tray--custom-size",
        ".ui-tray[data-custom-size=\"true\"]",
        ".ui-tray[data-size-source=\"custom\"]",
        ".ui-tray[data-dismiss-source=\"custom\"]",
        ".ui-tray[data-custom-dismiss=\"true\"]",
        ".ui-tray[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-tray[data-custom-keyboard-dismiss=\"true\"]",
        ".ui-tray--custom-id",
        ".ui-tray[data-id-source=\"custom\"]",
        ".ui-tray[data-custom-id=\"true\"]",
        ".ui-tray--custom-title",
        ".ui-tray[data-title-source=\"custom\"]",
        ".ui-tray[data-custom-title=\"true\"]",
        ".ui-tray[data-class-source=\"custom\"]",
        ".ui-tray[data-exit-source=\"custom\"]",
        ".ui-tray[data-custom-exit=\"true\"]",
        ".ui-tray--fixed-height",
        ".ui-tray[data-size=\"auto\"]",
        ".ui-tray--with-description",
        ".ui-tray[data-state=\"title-only\"]",
        ".ui-tray--close-shown .ui-tray__header",
        ".ui-tray[data-close-button=\"shown\"] .ui-tray__header",
        ".ui-tray[data-footer=\"present\"] .ui-tray__footer",
        ".ui-tray__header[data-slot=\"tray-header\"]",
        ".ui-tray__title[data-slot=\"tray-title\"]",
        ".ui-tray__body[data-slot=\"tray-body\"]",
        ".ui-tray__footer[data-slot=\"tray-footer\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tray styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tray_motion_contract_exposes_default_and_custom_sheet_checks() {
    let source = load_source("src/tray/motion.rs");

    for needle in [
        "pub struct TrayMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Tray motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn tray_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::tray::styles::CSS);"),
        "ui css aggregator should include tray styles."
    );
}

#[test]
fn tray_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn tray() -> AnyView",
        "title=\"Tray\"",
        "slug=\"tray\"",
        "State + Source Markers",
        "data-size-source",
        "<Tray",
    ] {
        assert!(
            source.contains(needle),
            "tray docs page should contain `{needle}`."
        );
    }
}

#[test]
fn tray_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "let custom_motion = TrayMotion {",
        "sheet: ui::SheetMotion {",
        "initial_offset_px: 46.0",
        "id_base=\"docs-tray-fixed\".to_string()",
        "motion=custom_motion",
        "is_fixed_height=true",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "show_close_button=false",
        "class_name=\"docs-tray-custom\".to_string()",
        "Inspect data-size-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "tray docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tray_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/tray/motion.rs");
    let view_source = load_source("src/tray/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TrayMotion) -> TrayMotion",
        "sheet: crate::sheet::SheetMotion",
        "crate::sheet::motion::sanitize_motion(motion.sheet)",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_offset_range()",
        "fn sanitize_motion_delegates_to_sheet_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tray motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::tray::motion::sanitize_motion(motion);"),
        "Tray view should sanitize motion before forwarding it to Sheet.",
    );
}

#[test]
fn tray_docs_page_covers_primary_playgrounds() {
    tray_docs_page_contains_state_source_playground();
}

#[test]
fn tray_docs_playgrounds_lock_state_matrix_contract_values() {
    tray_docs_custom_motion_playground_locks_contract_values();
}
