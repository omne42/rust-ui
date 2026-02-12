use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toaster_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toaster/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Toaster internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toaster_is_publicly_exported_from_module_and_crate_root() {
    let toaster_mod = load_source("src/toaster/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toaster_mod.contains("pub use view::Toaster;"),
        "toaster::mod should re-export Toaster."
    );
    assert!(
        toaster_mod.contains("DEFAULT_PORTAL") && toaster_mod.contains("DEFAULT_MAX_TOASTS"),
        "toaster::mod should expose default portal/max-toasts contracts."
    );
    assert!(
        crate_root.contains("pub use toaster::{Toaster, ToasterPosition};"),
        "crate root should expose Toaster and ToasterPosition."
    );
}

#[test]
fn toaster_module_exposes_slot_and_part_state_contracts() {
    let source = load_source("src/toaster/mod.rs");

    for needle in [
        "pub enum ToasterSlot",
        "pub enum ToasterStoreSource",
        "pub struct ToasterPartStateInput",
        "pub struct ToasterPartState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn base_class(self) -> &'static str",
    ] {
        assert!(
            source.contains(needle),
            "Toaster module should include `{needle}` for stable slot/part-state contracts."
        );
    }
}

#[test]
fn toaster_view_uses_logic_state_contracts() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_max_toasts(max_toasts)",
        "logic::resolve_state(ToasterPartStateInput {",
        "slot: ToasterSlot::Root",
        "slot: ToasterSlot::Sonner",
        "logic::compose_class_name(class_name.get_value(), root_state)",
        "logic::compose_class_name(None, sonner_state)",
        "logic::map_to_sonner_position(root_state.position)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-portal=root_state.portal_attr",
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-max-toasts-source=root_state.max_toasts_source_attr",
        "data-aria-source=root_state.aria_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
        "data-custom-position=root_state.has_custom_position.then_some(\"true\")",
        "data-custom-portal=root_state.has_custom_portal.then_some(\"true\")",
        "data-custom-max-toasts=root_state.has_custom_max_toasts.then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=root_state.has_custom_class_name.then_some(\"true\")",
        "data-sonner-slot=sonner_state.slot_attr",
        "data-sonner-state=sonner_state.state_attr",
        "data-sonner-position=sonner_state.position_attr",
        "data-sonner-portal=sonner_state.portal_attr",
        "data-sonner-queue=sonner_state.queue_attr",
        "aria-label=aria_label.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "Toaster view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toaster_view_tracks_store_source_resolution() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "if let Some(provided_store) = store",
        "ToasterStoreSource::Provided",
        "crate::toast::use_toast_store()",
        "ToasterStoreSource::Context",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "ToasterStoreSource::Local",
        "max_toasts: normalized_max_toasts",
    ] {
        assert!(
            source.contains(needle),
            "Toaster view should include `{needle}` for stable store-source derivation."
        );
    }
}

#[test]
fn toaster_composes_sonner_as_host_layer() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "<Sonner",
        "store=store",
        "position=sonner_position",
        "class_name=sonner_class_name",
        "max_toasts=sonner_state.max_toasts",
        "portal=sonner_state.portal",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "Toaster should compose Sonner via `{needle}`."
        );
    }
}

#[test]
fn toaster_logic_models_positions_queue_and_part_state() {
    let source = load_source("src/toaster/logic.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Toaster notifications\";",
        "pub const DEFAULT_PORTAL: bool = true;",
        "pub const DEFAULT_MAX_TOASTS: usize = 3;",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn state_attr(portal: bool) -> &'static str",
        "pub fn queue_attr(max_toasts: usize) -> &'static str",
        "pub fn resolve_state(input: ToasterPartStateInput) -> ToasterPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ToasterPartState)",
        "pub fn map_to_sonner_position(position: ToasterPosition)",
    ] {
        assert!(
            source.contains(needle),
            "Toaster logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn toaster_styles_include_state_and_source_marker_contracts() {
    let source = load_source("src/toaster/styles.rs");

    for selector in [
        ".ui-toaster[data-motion-source=\"custom\"]",
        ".ui-toaster[data-custom-motion=\"true\"]",
        ".ui-toaster[data-position-source=\"custom\"]",
        ".ui-toaster[data-custom-position=\"true\"]",
        ".ui-toaster[data-portal-source=\"custom\"]",
        ".ui-toaster[data-custom-portal=\"true\"]",
        ".ui-toaster[data-max-toasts-source=\"custom\"]",
        ".ui-toaster[data-custom-max-toasts=\"true\"]",
        ".ui-toaster[data-store-source=\"provided\"]",
        ".ui-toaster[data-store-source=\"context\"]",
        ".ui-toaster[data-store-source=\"local\"]",
        ".ui-toaster[data-state=\"inline\"]",
        ".ui-toaster[data-queue=\"single\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-queue=\"bounded\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster__sonner[data-slot=\"toaster-sonner\"].ui-sonner",
    ] {
        assert!(
            source.contains(selector),
            "Toaster styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn toaster_docs_page_contains_state_source_playground() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn toaster() -> AnyView",
        "title=\"Toaster\"",
        "slug=\"toaster\"",
        "State + Source Markers",
        "data-position-source",
        "data-store-source",
        "<Toaster",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs page should contain `{needle}`."
        );
    }
}

#[test]
fn toaster_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::toaster::styles::CSS);"),
        "ui-components css aggregator should include toaster styles."
    );
}

#[test]
fn toaster_docs_custom_motion_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "portal=false",
        "position=ToasterPosition::TopLeft",
        "max_toasts=4",
        "aria_label=\"Alert stream\".to_string()",
        "class_name=\"docs-toaster-source\".to_string()",
        "motion=ToastMotion {",
        "let custom_motion = ToastMotion {",
        "initial_y_px: 20.0",
        "initial_scale: 0.95",
        "..ToastMotion::default()",
        "motion=custom_motion",
        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools.",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn toaster_docs_page_covers_primary_playgrounds() {
    toaster_docs_page_contains_state_source_playground();
}

#[test]
fn toaster_docs_playgrounds_lock_state_matrix_contract_values() {
    toaster_docs_custom_motion_playground_locks_contract_values();
}
