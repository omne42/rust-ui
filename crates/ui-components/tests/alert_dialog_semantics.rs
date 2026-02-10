use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn alert_dialog_composes_overlay_with_role_and_label_ids() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "<Overlay",
        "role=\"alertdialog\"",
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should compose Overlay and wire alertdialog semantics (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_supports_secondary_and_cancel_actions() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "secondary_label",
        "on_secondary",
        "on_cancel",
        "view_state.show_secondary",
        "view_state.show_cancel",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should expose secondary/cancel action props and view-state gates (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_confirm_and_secondary_close_before_running_callbacks() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "let on_confirm_press",
        "on_close_for_confirm.run(())",
        "on_confirm.run(())",
        "let on_secondary_press",
        "on_close_for_secondary.run(())",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog actions should close (on_close) before running user callbacks (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_supports_autofocus_button_contract() {
    let source = load_source("src/alert_dialog/view.rs");
    let logic = load_source("src/alert_dialog/logic.rs");

    for needle in [
        "AlertDialogAutoFocusButton",
        "auto_focus_button",
        "focus_button_soon",
        "NodeRef<html::Button>",
    ] {
        assert!(
            source.contains(needle) || logic.contains(needle),
            "AlertDialog should support an auto-focus button contract (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_type_icon_is_present_for_warning_and_error_variants() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "alert-dialog-type-icon",
        "AlertDialogVariant::Warning",
        "AlertDialogVariant::Error",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should render a stable type icon slot for warning/error variants (`{needle}`)."
        );
    }
}

#[test]
fn alert_dialog_emits_state_variant_and_motion_markers() {
    let source = load_source("src/alert_dialog/view.rs");

    for needle in [
        "data-slot=\"alert-dialog\"",
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-variant=variant.data_attr()",
        "data-with-description=view_state.show_description.then_some(\"true\")",
        "data-show-cancel=view_state.show_cancel.then_some(\"true\")",
        "data-show-secondary=view_state.show_secondary.then_some(\"true\")",
        "data-confirm-disabled=confirm_disabled.then_some(\"true\")",
        "data-secondary-disabled=secondary_disabled.then_some(\"true\")",
        "data-motion-source=if motion == AlertDialogMotion::default()",
        "data-custom-motion=(motion != AlertDialogMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog should expose `{needle}` for stable state/motion marker contracts."
        );
    }
}

#[test]
fn alert_dialog_styles_include_motion_marker_selectors() {
    let source = load_source("src/alert_dialog/styles.rs");

    for selector in [
        ".ui-alert-dialog[data-motion-source=\"custom\"]",
        ".ui-alert-dialog[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "AlertDialog styles should include `{selector}` as stable motion-marker contracts."
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
fn alert_dialog_variant_logic_exposes_data_attr_contract() {
    let source = load_source("src/alert_dialog/logic.rs");

    for needle in [
        "pub fn data_attr(self) -> &'static str",
        "\"default\"",
        "\"confirmation\"",
        "\"destructive\"",
        "\"warning\"",
        "\"error\"",
    ] {
        assert!(
            source.contains(needle),
            "AlertDialog variant logic should include `{needle}` for deterministic variant markers."
        );
    }
}
