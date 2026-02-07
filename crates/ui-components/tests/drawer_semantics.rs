use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn drawer_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/drawer/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Drawer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn drawer_uses_logic_state_model() {
    let view_source = load_source("src/drawer/view.rs");
    let logic_source = load_source("src/drawer/logic.rs");

    for needle in [
        "pub struct DrawerStateInput",
        "pub struct DrawerState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_required_text(",
        "pub fn normalize_id_base(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Drawer logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, \"Drawer\")",
        "logic::normalize_optional_text(description)",
        "logic::resolve_state(DrawerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Drawer view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn drawer_composes_sheet_with_stable_aria_ids() {
    let source = load_source("src/drawer/view.rs");

    for needle in [
        "<Sheet",
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
        "id=move || title_id_attr.get()",
    ] {
        assert!(
            source.contains(needle),
            "Drawer should expose stable a11y id wiring (`{needle}`)."
        );
    }
}

#[test]
fn drawer_only_sets_describedby_when_description_exists() {
    let source = load_source("src/drawer/view.rs");

    assert!(
        source.contains("if state.show_description"),
        "Drawer should branch on description presence so `aria-describedby` is only set when needed."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "id=move || description_id_attr.get()",
    ] {
        assert!(
            source.contains(needle),
            "Drawer should wire description ids only on described path (`{needle}`)."
        );
    }
}

#[test]
fn drawer_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/drawer/view.rs");

    for attr in [
        "data-slot=\"drawer\"",
        "data-state=state.state_attr",
        "data-placement=state.placement_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-close-button=state.close_button_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"drawer-header\"",
        "data-slot=\"drawer-body\"",
        "data-slot=\"drawer-footer\"",
    ] {
        assert!(
            source.contains(attr),
            "Drawer should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn drawer_styles_include_state_marker_contracts() {
    let source = load_source("src/drawer/styles.rs");

    for selector in [
        ".ui-drawer--placement-left",
        ".ui-drawer[data-placement=\"right\"]",
        ".ui-drawer--with-description",
        ".ui-drawer[data-state=\"title-only\"]",
        ".ui-drawer--close-hidden",
        ".ui-drawer[data-close-button=\"shown\"]",
        ".ui-drawer[data-footer=\"present\"]",
        ".ui-drawer--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Drawer styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn drawer_close_button_and_motion_contracts_are_preserved() {
    let source = load_source("src/drawer/view.rs");

    for needle in [
        "data-slot=\"drawer-close\"",
        "<IconButton",
        "aria_label=close_label",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Drawer should preserve close/motion contracts (`{needle}`)."
        );
    }
}
