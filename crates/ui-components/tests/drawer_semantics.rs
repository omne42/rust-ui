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
fn drawer_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/drawer/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use logic::DrawerPlacement;",
        "pub use motion::DrawerMotion;",
        "pub use view::Drawer;",
        "pub enum DrawerSlot",
        "pub struct DrawerPartStateInput",
        "pub struct DrawerPartState",
    ] {
        assert!(
            module_source.contains(needle),
            "drawer module should include `{needle}` state contracts."
        );
    }

    assert!(
        crate_source.contains("pub use drawer::{Drawer, DrawerMotion, DrawerPlacement};")
            || (crate_source.contains("pub use drawer::Drawer;")
                && crate_source.contains("pub use drawer::DrawerMotion;")
                && crate_source.contains("pub use drawer::DrawerPlacement;")),
        "crate root should re-export `Drawer`, `DrawerPlacement`, and `DrawerMotion` contracts."
    );
}

#[test]
fn drawer_logic_exposes_state_helpers() {
    let source = load_source("src/drawer/logic.rs");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-drawer\";",
        "pub const DEFAULT_TITLE: &str = \"Drawer\";",
        "pub const DEFAULT_PLACEMENT: DrawerPlacement = DrawerPlacement::Right;",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn footer_attr(has_footer: bool)",
        "pub fn close_button_attr(show_close_button: bool)",
        "pub fn placement_class(placement: DrawerPlacement)",
        "pub fn placement_attr(placement: DrawerPlacement)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: DrawerPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: DrawerPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Drawer logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn drawer_view_uses_logic_state_contracts() {
    let source = load_source("src/drawer/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(DrawerPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-placement=root_state.placement_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
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
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=header_state.slot_attr",
        "data-slot=title_state.slot_attr",
        "data-slot=body_state.slot_attr",
        "data-slot=footer_state.slot_attr",
        "data-slot=close_state.slot_attr",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Drawer view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn drawer_only_sets_describedby_when_description_exists() {
    let source = load_source("src/drawer/view.rs");

    assert!(
        source.contains("if root_state.show_description"),
        "Drawer should branch on description presence so `aria-describedby` is only set when needed."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Drawer should wire description ids only on described path (`{needle}`)."
        );
    }
}

#[test]
fn drawer_styles_include_state_and_source_markers() {
    let source = load_source("src/drawer/styles.rs");

    for selector in [
        ".ui-drawer[data-motion-source=\"custom\"]",
        ".ui-drawer[data-custom-motion=\"true\"]",
        ".ui-drawer[data-placement-source=\"custom\"]",
        ".ui-drawer--custom-description",
        ".ui-drawer[data-custom-description=\"true\"]",
        ".ui-drawer[data-description-source=\"custom\"]",
        ".ui-drawer--custom-footer",
        ".ui-drawer[data-custom-footer=\"true\"]",
        ".ui-drawer[data-footer-source=\"custom\"]",
        ".ui-drawer--custom-close",
        ".ui-drawer[data-custom-close=\"true\"]",
        ".ui-drawer[data-close-source=\"custom\"]",
        ".ui-drawer--custom-id",
        ".ui-drawer[data-custom-id=\"true\"]",
        ".ui-drawer[data-id-source=\"custom\"]",
        ".ui-drawer--custom-title",
        ".ui-drawer[data-custom-title=\"true\"]",
        ".ui-drawer[data-title-source=\"custom\"]",
        ".ui-drawer[data-class-source=\"custom\"]",
        ".ui-drawer[data-exit-source=\"custom\"]",
        ".ui-drawer[data-custom-exit=\"true\"]",
        ".ui-drawer--placement-left",
        ".ui-drawer[data-placement=\"right\"]",
        ".ui-drawer--with-description",
        ".ui-drawer[data-state=\"title-only\"]",
        ".ui-drawer--close-hidden",
        ".ui-drawer[data-close-button=\"shown\"]",
        ".ui-drawer[data-footer=\"present\"]",
        ".ui-drawer__header[data-slot=\"drawer-header\"]",
        ".ui-drawer__title[data-slot=\"drawer-title\"]",
        ".ui-drawer__body[data-slot=\"drawer-body\"]",
        ".ui-drawer__footer[data-slot=\"drawer-footer\"]",
    ] {
        assert!(
            source.contains(selector),
            "Drawer styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn drawer_motion_contract_exposes_default_and_custom_sheet_tests() {
    let source = load_source("src/drawer/motion.rs");

    for needle in [
        "pub struct DrawerMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Drawer motion module should include `{needle}` for HeroUI-level contract coverage."
        );
    }
}

#[test]
fn drawer_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::drawer::styles::CSS);"),
        "ui-components css aggregator should include drawer styles."
    );
}

#[test]
fn drawer_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
        "State + Source Markers",
        "data-placement-source",
        "<Drawer",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs page should contain `{needle}`."
        );
    }
}

#[test]
fn drawer_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/drawer/motion.rs");
    let view_source = load_source("src/drawer/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DrawerMotion) -> DrawerMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
        "fn sanitize_motion_delegates_to_sheet_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Drawer motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::drawer::motion::sanitize_motion(motion);"),
        "Drawer view should sanitize motion before forwarding to Sheet.",
    );
}

#[test]
fn drawer_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = DrawerMotion {",
        "sheet: SheetMotion {",
        "initial_offset_px: 52.0",
        "title=\"State + Source Markers\"",
        "motion=custom_motion",
        "placement=DrawerPlacement::Left",
        "show_close_button=false",
        "class_name=\"docs-drawer-custom\".to_string()",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-placement-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn drawer_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
        "description=\"Sheet composition with centralized placement/description/footer/close state attrs and stable drawer slots.\"",
        "<Playground title=\"Right Drawer + Slots\" code_signal=semantic_code>",
        "title=\"State + Source Markers\"",
        "<Drawer",
        "placement=DrawerPlacement::Right",
        "placement=DrawerPlacement::Left",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs page should include `{needle}` for drawer primary playground coverage.",
        );
    }
}

#[test]
fn drawer_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "id_base=\"docs-drawer-right\".to_string()",
        "title=\"Drawer title\".to_string()",
        "description=\"Drawer composes Sheet and keeps labeled/description semantics aligned.\".to_string()",
        "placement=DrawerPlacement::Right",
        "on_exit_complete=on_semantic_exit_complete",
        "id_base=\"docs-drawer-left\".to_string()",
        "title=\"Left drawer\".to_string()",
        "placement=DrawerPlacement::Left",
        "show_close_button=false",
        "class_name=\"docs-drawer-custom\".to_string()",
        "motion=custom_motion",
        "let custom_motion = DrawerMotion {",
        "initial_offset_px: 52.0",
        "on_exit_complete=on_custom_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs playgrounds should contain `{needle}`.",
        );
    }
}
