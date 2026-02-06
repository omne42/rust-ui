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
