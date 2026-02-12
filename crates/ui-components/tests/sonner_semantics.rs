use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sonner_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sonner/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sonner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sonner_is_publicly_exported_from_module_and_crate_root() {
    let sonner_mod = load_source("src/sonner/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        sonner_mod.contains("pub use view::Sonner;"),
        "sonner::mod should re-export Sonner."
    );
    assert!(
        sonner_mod.contains("DEFAULT_PORTAL") && sonner_mod.contains("DEFAULT_MAX_TOASTS"),
        "sonner::mod should expose default portal/max-toasts contracts."
    );
    assert!(
        crate_root.contains("pub use sonner::{Sonner, SonnerPosition};"),
        "crate root should expose Sonner and SonnerPosition."
    );
}

#[test]
fn sonner_module_exposes_slot_and_part_state_contracts() {
    let source = load_source("src/sonner/mod.rs");

    for needle in [
        "pub enum SonnerSlot",
        "pub enum SonnerStoreSource",
        "pub struct SonnerPartStateInput",
        "pub struct SonnerPartState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn base_class(self) -> &'static str",
    ] {
        assert!(
            source.contains(needle),
            "Sonner module should include `{needle}` for stable slot/part-state contracts."
        );
    }
}

#[test]
fn sonner_view_uses_logic_state_contracts() {
    let source = load_source("src/sonner/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_max_toasts(max_toasts)",
        "logic::resolve_state(SonnerPartStateInput {",
        "slot: SonnerSlot::Root",
        "slot: SonnerSlot::Viewport",
        "logic::compose_class_name(class_name.get_value(), root_state)",
        "logic::compose_class_name(None, viewport_state)",
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
        "data-viewport-slot=viewport_state.slot_attr",
        "data-viewport-state=viewport_state.state_attr",
        "data-viewport-position=viewport_state.position_attr",
        "data-viewport-portal=viewport_state.portal_attr",
        "data-viewport-queue=viewport_state.queue_attr",
        "aria-label=aria_label.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "Sonner view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn sonner_view_tracks_store_source_resolution() {
    let source = load_source("src/sonner/view.rs");

    for needle in [
        "if let Some(provided_store) = store",
        "SonnerStoreSource::Provided",
        "crate::toast::use_toast_store()",
        "SonnerStoreSource::Context",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "SonnerStoreSource::Local",
        "max_toasts: normalized_max_toasts",
    ] {
        assert!(
            source.contains(needle),
            "Sonner view should include `{needle}` for stable store-source derivation."
        );
    }
}

#[test]
fn sonner_composes_toast_viewport_and_forwards_stateful_props() {
    let source = load_source("src/sonner/view.rs");

    for needle in [
        "<ToastViewport",
        "store=store",
        "class_name=viewport_class_name",
        "max_toasts=viewport_state.max_toasts",
        "portal=viewport_state.portal",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should compose ToastViewport using `{needle}`."
        );
    }
}

#[test]
fn sonner_logic_models_positions_queue_and_part_state() {
    let source = load_source("src/sonner/logic.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Notifications\";",
        "pub const DEFAULT_PORTAL: bool = true;",
        "pub const DEFAULT_MAX_TOASTS: usize = 3;",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn state_attr(portal: bool) -> &'static str",
        "pub fn queue_attr(max_toasts: usize) -> &'static str",
        "pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: SonnerPartState)",
        "pub fn as_attr(self) -> &'static str",
        "pub fn class_suffix(self) -> &'static str",
    ] {
        assert!(
            source.contains(needle),
            "Sonner logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn sonner_styles_include_state_and_source_marker_contracts() {
    let source = load_source("src/sonner/styles.rs");

    for selector in [
        ".ui-sonner[data-motion-source=\"custom\"]",
        ".ui-sonner[data-custom-motion=\"true\"]",
        ".ui-sonner[data-position-source=\"custom\"]",
        ".ui-sonner[data-custom-position=\"true\"]",
        ".ui-sonner[data-portal-source=\"custom\"]",
        ".ui-sonner[data-custom-portal=\"true\"]",
        ".ui-sonner[data-max-toasts-source=\"custom\"]",
        ".ui-sonner[data-custom-max-toasts=\"true\"]",
        ".ui-sonner[data-store-source=\"provided\"]",
        ".ui-sonner[data-store-source=\"context\"]",
        ".ui-sonner[data-store-source=\"local\"]",
        ".ui-sonner[data-state=\"inline\"]",
        ".ui-sonner[data-queue=\"single\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner[data-queue=\"bounded\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner__viewport--inline.ui-toast-viewport",
        ".ui-sonner__viewport--top-center.ui-toast-viewport",
        ".ui-sonner__viewport--bottom-right.ui-toast-viewport",
    ] {
        assert!(
            source.contains(selector),
            "Sonner styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn sonner_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn sonner() -> AnyView",
        "title=\"Sonner\"",
        "slug=\"sonner\"",
        "State + Source Markers",
        "data-position-source",
        "data-store-source",
        "<Sonner",
    ] {
        assert!(
            source.contains(needle),
            "sonner docs page should contain `{needle}`."
        );
    }
}

#[test]
fn sonner_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::sonner::styles::CSS);"),
        "ui-components css aggregator should include sonner styles."
    );
}

#[test]
fn sonner_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "portal=false",
        "position=SonnerPosition::TopLeft",
        "max_toasts=4",
        "aria_label=\"Status updates\".to_string()",
        "class_name=\"docs-sonner-source\".to_string()",
        "motion=ToastMotion {",
        "let custom_motion = ToastMotion {",
        "initial_y_px: 22.0",
        "initial_scale: 0.94",
        "..ToastMotion::default()",
        "motion=custom_motion",
        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "sonner docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn sonner_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn sonner() -> AnyView",
        "title=\"Sonner\"",
        "slug=\"sonner\"",
        "description=\"Shadcn/HeroUI-style toast host that composes ToastViewport with position presets, queue limits, and stable Sonner slot/source-state data contracts.\"",
        "<Playground title=\"Portal Queue + Variants\" code=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code=state_code>",
        "title=\"State + Source Markers\"",
        "<Sonner",
    ] {
        assert!(
            source.contains(needle),
            "overlays_extra sonner docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn sonner_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Portal Queue + Variants\"",
        "variant=ButtonVariant::Secondary",
        "on_press=push_saved",
        "\"Push success\"",
        "variant=ButtonVariant::Destructive",
        "on_press=push_danger",
        "\"Push danger\"",
        "<Sonner store=portal_store.get_value() />",
        "title=\"Inline Top-Center + Max Queue\"",
        "store=inline_store.get_value()",
        "portal=false",
        "position=SonnerPosition::TopCenter",
        "max_toasts=2",
        "class_name=\"docs-sonner-inline\".to_string()",
        "title=\"State + Source Markers\"",
        "store=source_store.get_value()",
        "position=SonnerPosition::TopLeft",
        "max_toasts=4",
        "aria_label=\"Status updates\".to_string()",
        "class_name=\"docs-sonner-source\".to_string()",
        "motion=custom_motion",
        "let custom_motion = ToastMotion {",
        "initial_y_px: 22.0",
        "initial_scale: 0.94",
        "..ToastMotion::default()",
        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "overlays_extra sonner playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
