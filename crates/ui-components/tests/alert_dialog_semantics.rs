use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn alert_dialog_does_not_expose_logic_module() {
    let source = load_source("src/alert_dialog/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "AlertDialog's `logic` module should stay private to avoid leaking internal details."
    );
}

#[test]
fn alert_dialog_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/alert_dialog/mod.rs");

    for needle in [
        "pub enum AlertDialogSlot",
        "pub struct AlertDialogPartStateInput",
        "pub struct AlertDialogPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_CONFIRM_LABEL",
        "DEFAULT_CANCEL_LABEL",
        "DEFAULT_AUTO_FOCUS_BUTTON",
    ] {
        assert!(
            source.contains(needle),
            "alert_dialog::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn alert_dialog_logic_exposes_state_helpers() {
    let source = load_source("src/alert_dialog/logic.rs");

    for needle in [
        "pub fn state_attr(is_open: bool)",
        "pub fn description_attr(show_description: bool)",
        "pub fn action_visibility_attr(show: bool)",
        "pub fn disabled_attr(disabled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn normalize_cancel_label(value: Option<String>)",
        "pub fn normalize_secondary_label(value: Option<String>)",
        "pub fn resolve_state(input: AlertDialogPartStateInput) -> AlertDialogPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: AlertDialogPartState)",
        "pub fn data_attr(self) -> &'static str",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn alert_dialog_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_cancel_label(cancel_label)",
        "logic::normalize_secondary_label(secondary_label)",
        "logic::resolve_state(AlertDialogPartStateInput {",
        "slot: AlertDialogSlot::Root",
        "logic::compose_class_name(None, root_state.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-variant=move || root_state.get().variant_attr",
        "data-description=move || root_state.get().description_attr",
        "data-cancel=move || root_state.get().cancel_attr",
        "data-secondary=move || root_state.get().secondary_attr",
        "data-auto-focus=move || root_state.get().auto_focus_attr",
        "data-variant-source=move || root_state.get().variant_source_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-title-source=move || root_state.get().title_source_attr",
        "data-description-source=move || root_state.get().description_source_attr",
        "data-cancel-source=move || root_state.get().cancel_source_attr",
        "data-secondary-source=move || root_state.get().secondary_source_attr",
        "data-confirm-source=move || root_state.get().confirm_source_attr",
        "data-auto-focus-source=move || root_state.get().auto_focus_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-exit-source=move || root_state.get().exit_source_attr",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-exit=move || root_state.get().has_on_exit_complete.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn alert_dialog_composes_overlay_with_alert_role_and_optional_describedby() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "<Overlay",
        "role=\"alertdialog\"",
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
        "let description_id = format!(\"{id_base}-description\")",
        "if show_description",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should include `{needle}` for stable overlay/a11y semantics."
        );
    }
}

#[test]
fn alert_dialog_confirm_and_secondary_close_before_running_callbacks() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "let on_confirm_press",
        "on_close.get_value().run(())",
        "on_confirm.get_value().run(())",
        "let on_secondary_press",
        "on_secondary.get_value()",
        "let on_cancel_press",
        "on_cancel.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog actions should close first, then run optional callbacks (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_supports_autofocus_button_contract() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "AlertDialogAutoFocusButton",
        "auto_focus_button",
        "focus_button_soon",
        "AlertDialogAutoFocusButton::Cancel",
        "AlertDialogAutoFocusButton::Secondary",
        "AlertDialogAutoFocusButton::Confirm",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should support autofocus button contracts (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_styles_include_state_and_source_markers() {
    let source = load_source("src/alert_dialog/styles.rs");

    for selector in [
        ".ui-alert-dialog[data-motion-source=\"custom\"]",
        ".ui-alert-dialog[data-custom-motion=\"true\"]",
        ".ui-alert-dialog[data-variant-source=\"custom\"]",
        ".ui-alert-dialog[data-custom-variant=\"true\"]",
        ".ui-alert-dialog[data-id-source=\"custom\"]",
        ".ui-alert-dialog[data-title-source=\"custom\"]",
        ".ui-alert-dialog[data-description-source=\"custom\"]",
        ".ui-alert-dialog[data-cancel-source=\"custom\"]",
        ".ui-alert-dialog[data-secondary-source=\"custom\"]",
        ".ui-alert-dialog[data-confirm-source=\"custom\"]",
        ".ui-alert-dialog[data-auto-focus-source=\"custom\"]",
        ".ui-alert-dialog[data-exit-source=\"custom\"]",
        ".ui-alert-dialog--with-description",
        ".ui-alert-dialog[data-description=\"present\"]",
        ".ui-alert-dialog__title[data-slot=\"alert-dialog-title\"]",
        ".ui-alert-dialog__description[data-slot=\"alert-dialog-description\"]",
        ".ui-alert-dialog__footer[data-slot=\"alert-dialog-footer\"]",
    ] {
        assert!(
            source.contains(selector),
            "AlertDialog styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn alert_dialog_motion_contract_exposes_default_and_custom_overlay_tests() {
    let source = load_source("src/alert_dialog/motion.rs");

    for needle in [
        "pub struct AlertDialogMotion",
        "pub overlay: crate::overlay::OverlayMotion",
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog motion module should include `{needle}` for HeroUI-level contract coverage."
        );
    }
}

#[test]
fn alert_dialog_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs");

    for needle in [
        "pub(super) fn alert_dialog() -> AnyView",
        "title=\"AlertDialog\"",
        "slug=\"alert-dialog\"",
        "State + Source Markers",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-cancel-source",
        "data-secondary-source",
        "data-motion-source",
        "<AlertDialog",
    ] {
        assert!(
            source.contains(needle),
            "alert dialog docs page should contain `{needle}`."
        );
    }
}

#[test]
fn alert_dialog_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/alert_dialog/motion.rs");
    let view_source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AlertDialogMotion) -> AlertDialogMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "fn sanitize_motion_delegates_to_overlay_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "AlertDialog motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::alert_dialog::motion::sanitize_motion(motion);"),
        "AlertDialog view should sanitize motion before forwarding to Overlay.",
    );
}
