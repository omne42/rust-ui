use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toast_does_not_expose_logic_module() {
    let source = load_source("src/toast/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Toast's `logic` module should stay private to avoid leaking store internals into the public API."
    );
}

#[test]
fn toast_module_exposes_slot_and_part_state_contracts() {
    let source = load_source("src/toast/mod.rs");

    for needle in [
        "pub enum ToastSlot",
        "pub struct ToastPartStateInput",
        "pub struct ToastPartState",
        "pub enum ToastViewportSlot",
        "pub enum ToastStoreSource",
        "pub struct ToastViewportStateInput",
        "pub struct ToastViewportState",
        "DEFAULT_VIEWPORT_PORTAL",
        "DEFAULT_VIEWPORT_MAX_TOASTS",
    ] {
        assert!(
            source.contains(needle),
            "toast::mod should include `{needle}` for stable toast contracts."
        );
    }
}

#[test]
fn toast_is_publicly_exported_from_toast_module_and_crate_root() {
    let toast_mod = load_source("src/toast/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toast_mod.contains("pub use view::{Toast, ToastViewport};"),
        "toast::mod should re-export both Toast and ToastViewport."
    );
    assert!(
        crate_root.contains("Toast, ToastMotion"),
        "crate root should expose Toast together with toast types."
    );
}

#[test]
fn toast_logic_models_state_and_source_contracts() {
    let source = load_source("src/toast/logic.rs");

    for needle in [
        "pub const DEFAULT_TITLE: &str = \"Notification\";",
        "pub const DEFAULT_VIEWPORT_PORTAL: bool = true;",
        "pub const DEFAULT_VIEWPORT_MAX_TOASTS: usize = 3;",
        "pub fn toast_state_attr(is_open: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn close_mode_attr(has_on_close: bool)",
        "pub fn viewport_state_attr(portal: bool)",
        "pub fn viewport_queue_attr(max_toasts: usize)",
        "pub fn normalize_viewport_max_toasts(max_toasts: usize) -> usize",
        "pub fn resolve_state(input: ToastPartStateInput) -> ToastPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ToastPartState)",
        "pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState",
        "pub fn compose_viewport_class_name(",
    ] {
        assert!(
            source.contains(needle),
            "Toast logic should include `{needle}` for centralized source-state derivation."
        );
    }
}

#[test]
fn toast_view_uses_logic_state_contracts() {
    let source = load_source("src/toast/view.rs");

    for needle in [
        "logic::resolve_state(ToastPartStateInput {",
        "slot: ToastSlot::Root",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "data-slot=move || state.get().slot_attr",
        "data-state=move || state.get().state_attr",
        "data-variant=move || state.get().variant_attr",
        "data-description=move || state.get().description_attr",
        "data-open=move || state.get().open_attr",
        "data-close-mode=move || state.get().close_mode_attr",
        "data-id-source=move || state.get().id_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-close-source=move || state.get().close_source_attr",
        "data-exit-source=move || state.get().exit_source_attr",
        "data-custom-id=move || state.get().has_custom_id.then_some(\"true\")",
        "data-custom-description=move || state.get().has_custom_description.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-close=move || state.get().has_custom_on_close.then_some(\"true\")",
        "data-custom-exit=move || state.get().has_custom_on_exit_complete.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Toast view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toast_viewport_uses_logic_state_contracts() {
    let source = load_source("src/toast/view.rs");

    for needle in [
        "logic::resolve_viewport_state(ToastViewportStateInput {",
        "slot: ToastViewportSlot::Root",
        "logic::compose_viewport_class_name(class_name, viewport_state)",
        "if let Some(provided_store) = store",
        "ToastStoreSource::Provided",
        "ToastStoreSource::Context",
        "ToastStoreSource::Local",
        "<Portal>",
        "data-ui-overlay-portal",
        "data-slot=move || viewport_state.get_value().slot_attr",
        "data-state=move || viewport_state.get_value().state_attr",
        "data-queue=move || viewport_state.get_value().queue_attr",
        "data-portal=move || viewport_state.get_value().portal_attr",
        "data-max-toasts=move || viewport_state.get_value().max_toasts.to_string()",
        "data-portal-source=move || viewport_state.get_value().portal_source_attr",
        "data-max-toasts-source=move || viewport_state.get_value().max_toasts_source_attr",
        "data-class-source=move || viewport_state.get_value().class_source_attr",
        "data-motion-source=move || viewport_state.get_value().motion_source_attr",
        "data-store-source=move || viewport_state.get_value().store_source_attr",
        "data-custom-portal=move || viewport_state.get_value().has_custom_portal.then_some(\"true\")",
        "data-custom-max-toasts=move || viewport_state.get_value().has_custom_max_toasts.then_some(\"true\")",
        "data-custom-class=move || viewport_state.get_value().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || viewport_state.get_value().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ToastViewport should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toast_has_spectrum_style_accessibility_semantics() {
    let source = load_source("src/toast/view.rs");

    for needle in [
        "role=\"status\"",
        "aria-live=variant.aria_live()",
        "aria-atomic=\"true\"",
        "aria-label=\"Dismiss toast\"",
        "if ev.key() == \"Escape\"",
    ] {
        assert!(
            source.contains(needle),
            "Toast should include `{needle}` for Spectrum-style accessibility semantics."
        );
    }
}

#[test]
fn toast_styles_include_state_and_source_marker_contracts() {
    let source = load_source("src/toast/styles.rs");

    for selector in [
        ".ui-toast[data-motion-source=\"custom\"]",
        ".ui-toast[data-custom-motion=\"true\"]",
        ".ui-toast[data-id-source=\"custom\"]",
        ".ui-toast[data-custom-id=\"true\"]",
        ".ui-toast[data-description-source=\"custom\"]",
        ".ui-toast[data-custom-description=\"true\"]",
        ".ui-toast[data-close-source=\"custom\"]",
        ".ui-toast[data-custom-close=\"true\"]",
        ".ui-toast[data-exit-source=\"custom\"]",
        ".ui-toast[data-custom-exit=\"true\"]",
        ".ui-toast[data-close-mode=\"noop\"] .ui-toast__close",
        ".ui-toast[data-variant=\"accent\"]",
        ".ui-toast[data-variant=\"danger\"]",
        ".ui-toast-viewport[data-motion-source=\"custom\"]",
        ".ui-toast-viewport[data-custom-motion=\"true\"]",
        ".ui-toast-viewport[data-store-source=\"provided\"]",
        ".ui-toast-viewport[data-store-source=\"context\"]",
        ".ui-toast-viewport[data-store-source=\"local\"]",
        ".ui-toast-viewport[data-state=\"inline\"]",
        ".ui-toast-viewport[data-queue=\"single\"]",
    ] {
        assert!(
            source.contains(selector),
            "Toast styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn toast_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn toast() -> AnyView",
        "title=\"Toast\"",
        "slug=\"toast\"",
        "State + Source Markers",
        "data-id-source",
        "data-description-source",
        "data-close-source",
        "data-exit-source",
        "data-motion-source",
        "<Toast",
    ] {
        assert!(
            source.contains(needle),
            "toast docs page should contain `{needle}`."
        );
    }
}

#[test]
fn toast_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/toast/motion.rs");
    let view_source = load_source("src/toast/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ToastMotion) -> ToastMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_number(value: f64, fallback: f64) -> f64",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Toast motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    for needle in [
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let motion = crate::toast::motion::sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle) || view_source.contains(needle),
            "Toast should include `{needle}` to sanitize motion at component and runtime boundaries.",
        );
    }
}

#[test]
fn toast_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let danger_motion = ToastMotion {",
        "initial_y_px: 18.0",
        "initial_scale: 0.96",
        "title=\"State + Source Markers\"",
        "id=\"docs-toast-danger\".to_string()",
        "class_name=\"docs-toast-custom\".to_string()",
        "motion=danger_motion",
        "variant=ToastVariant::Danger",
        "Inspect data-id-source / data-description-source / data-close-source / data-exit-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "toast docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn toast_docs_page_covers_primary_playgrounds() {
    toast_docs_page_contains_state_source_playground();
}

#[test]
fn toast_docs_playgrounds_lock_state_matrix_contract_values() {
    toast_docs_custom_motion_playground_locks_contract_values();
}
