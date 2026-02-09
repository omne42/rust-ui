use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn command_dialog_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/command_dialog/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CommandDialog internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn command_dialog_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/command_dialog/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::CommandDialog;"),
        "command_dialog module should export `CommandDialog`."
    );
    assert!(
        crate_source.contains("pub use command_dialog::CommandDialog;"),
        "crate root should re-export CommandDialog."
    );
}

#[test]
fn command_dialog_uses_logic_state_model() {
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");

    for needle in [
        "pub struct CommandDialogStateInput",
        "pub struct CommandDialogState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_title(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CommandDialog logic should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_title(title)",
        "logic::normalize_optional_text(description)",
        "logic::resolve_state(CommandDialogStateInput {",
        "logic::compose_class_name(class_name.get_value(), state)",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn command_dialog_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/command_dialog/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "overlay_open::use_controllable_open_state(open, default_open, on_open_change)",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog should support `{needle}` for controllable open state."
        );
    }
}

#[test]
fn command_dialog_composes_modal_command_and_presence() {
    let source = load_source("src/command_dialog/view.rs");

    for needle in [
        "use_presence(open)",
        "<Modal",
        "on_close=on_close",
        "on_exit_complete=presence.finish_exit",
        "<Command",
        "on_action=on_action_wrapped",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog should compose Modal+Command with presence via `{needle}`."
        );
    }
}

#[test]
fn command_dialog_emits_spectrum_state_data_attributes() {
    let source = load_source("src/command_dialog/view.rs");

    for needle in [
        "data-slot=\"command-dialog\"",
        "data-state=move || state.get().state_attr",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-description=move || state.get().description_attr",
        "data-close-on-action=move || state.get().close_on_action_attr",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog should expose `{needle}` for stable styling and regression tests."
        );
    }
}

#[test]
fn command_dialog_styles_include_state_marker_contracts() {
    let source = load_source("src/command_dialog/styles.rs");

    for selector in [
        ".ui-command-dialog {",
        ".ui-command-dialog__modal.ui-modal {",
        ".ui-command-dialog__command.ui-command {",
        ".ui-command-dialog--open",
        ".ui-command-dialog[data-state=\"open\"]",
        ".ui-command-dialog--persistent",
        ".ui-command-dialog[data-close-on-action=\"false\"]",
        ".ui-command-dialog--disabled",
        ".ui-command-dialog[data-disabled=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CommandDialog styles should include `{selector}` marker contracts."
        );
    }
}

#[test]
fn command_dialog_docs_page_exists_in_collections_command() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "<CommandDialog",
    ] {
        assert!(
            docs.contains(needle),
            "CommandDialog docs page should contain `{needle}`."
        );
    }
}
