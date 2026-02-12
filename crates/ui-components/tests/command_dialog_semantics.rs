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
fn command_dialog_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/command_dialog/mod.rs");

    for needle in [
        "pub enum CommandDialogSlot",
        "pub struct CommandDialogPartStateInput",
        "pub struct CommandDialogPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_CLOSE_ON_ACTION",
        "DEFAULT_DISABLED",
        "DEFAULT_DEFAULT_OPEN",
    ] {
        assert!(
            source.contains(needle),
            "command_dialog::mod should include `{needle}` contracts."
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
fn command_dialog_logic_exposes_state_helpers() {
    let source = load_source("src/command_dialog/logic.rs");

    for needle in [
        "pub fn state_attr(is_open: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn close_on_action_attr(close_on_action: bool)",
        "pub fn disabled_attr(disabled: bool)",
        "pub fn open_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(value: Option<String>)",
        "pub fn normalize_title(value: Option<String>)",
        "pub fn resolve_state(input: CommandDialogPartStateInput) -> CommandDialogPartState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn command_dialog_view_uses_logic_state_contracts() {
    let source = load_source("src/command_dialog/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_title(title)",
        "logic::normalize_optional_text(description)",
        "logic::resolve_state(CommandDialogPartStateInput {",
        "slot: CommandDialogSlot::Root",
        "logic::compose_class_name(class_name.get_value(), state)",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-description=move || root_state.get().description_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-title-source=move || root_state.get().title_source_attr",
        "data-description-source=move || root_state.get().description_source_attr",
        "data-placeholder-source=move || root_state.get().placeholder_source_attr",
        "data-empty-label-source=move || root_state.get().empty_label_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-default-open-source=move || root_state.get().default_open_source_attr",
        "data-close-on-action-source=move || root_state.get().close_on_action_source_attr",
        "data-disabled-source=move || root_state.get().disabled_source_attr",
        "data-command-motion-source=move || root_state.get().command_motion_source_attr",
        "data-overlay-motion-source=move || root_state.get().overlay_motion_source_attr",
        "data-custom-class=move || root_state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-command-motion=move ||",
        "data-custom-overlay-motion=move ||",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog view should include `{needle}` for stable state/source marker contracts."
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
        "let is_controlled = open.is_some()",
        "let has_custom_default_open = default_open.is_some()",
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
        "class_name=modal_class.get_value()",
        "<Command",
        "class_name=command_class.get_value()",
        "on_action=on_action_wrapped",
    ] {
        assert!(
            source.contains(needle),
            "CommandDialog should compose Modal+Command with presence via `{needle}`."
        );
    }
}

#[test]
fn command_dialog_styles_include_state_and_source_markers() {
    let source = load_source("src/command_dialog/styles.rs");

    for selector in [
        ".ui-command-dialog {",
        ".ui-command-dialog__modal.ui-modal {",
        ".ui-command-dialog__command.ui-command {",
        ".ui-command-dialog--open",
        ".ui-command-dialog[data-state=\"open\"]",
        ".ui-command-dialog--with-description",
        ".ui-command-dialog[data-description=\"present\"]",
        ".ui-command-dialog--persistent",
        ".ui-command-dialog[data-close-on-action=\"false\"]",
        ".ui-command-dialog--controlled",
        ".ui-command-dialog[data-open-mode=\"controlled\"]",
        ".ui-command-dialog[data-id-source=\"custom\"]",
        ".ui-command-dialog[data-title-source=\"custom\"]",
        ".ui-command-dialog[data-description-source=\"custom\"]",
        ".ui-command-dialog[data-placeholder-source=\"custom\"]",
        ".ui-command-dialog[data-empty-label-source=\"custom\"]",
        ".ui-command-dialog[data-aria-label-source=\"custom\"]",
        ".ui-command-dialog[data-action-source=\"custom\"]",
        ".ui-command-dialog[data-open-change-source=\"custom\"]",
        ".ui-command-dialog[data-command-motion-source=\"custom\"]",
        ".ui-command-dialog[data-overlay-motion-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "CommandDialog styles should include `{selector}` marker contracts."
        );
    }
}

#[test]
fn command_dialog_docs_page_contains_state_source_playground() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "State + Source Markers",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-placeholder-source",
        "data-action-source",
        "<CommandDialog",
    ] {
        assert!(
            docs.contains(needle),
            "CommandDialog docs page should contain `{needle}`."
        );
    }
}

#[test]
fn command_dialog_docs_controlled_open_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Controlled Open + Action Close\"",
        "id_base=\"docs-command-dialog-controlled\".to_string()",
        "title=\"Quick Actions\".to_string()",
        "description=\"Press ⌘K-style filtering and Enter to run actions.\".to_string()",
        "open=open",
        "on_open_change=on_open_change",
        "on_action=on_action",
    ] {
        assert!(
            docs.contains(needle),
            "CommandDialog docs controlled-open playground should contain `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_docs_state_source_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-command-dialog-marker\".to_string()",
        "title=\"Workspace Commands\".to_string()",
        "default_open=true",
        "close_on_action=false",
        "placeholder=\"Search pages, actions, and settings...\".to_string()",
        "empty_label=\"No command matches your search.\".to_string()",
        "aria_label=\"Workspace command dialog\".to_string()",
        "class_name=\"docs-command-dialog-custom\".to_string()",
        "let marker_overlay_motion = ui_components::OverlayMotion {",
        "initial_scale: 0.95",
        "initial_y_px: 10.0",
        "overlay_motion=marker_overlay_motion",
        "Inspect data-id-source / data-title-source / data-description-source / data-placeholder-source / data-action-source / data-overlay-motion-source in DevTools.",
        "close_on_action: false (dialog stays open)",
    ] {
        assert!(
            docs.contains(needle),
            "CommandDialog docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_docs_page_covers_primary_playgrounds() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "description=\"Shadcn-compatible command dialog that composes Modal + Command, supports controlled/uncontrolled open state, emits Spectrum data contracts, and reuses HeroUI-level overlay/active-highlight spring motion.\"",
        "<Playground title=\"Controlled Open + Action Close\" code=code>",
        "<Playground title=\"State + Source Markers\" code=marker_code>",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-placeholder-source",
        "data-action-source",
        "data-overlay-motion-source",
    ] {
        assert!(
            docs.contains(needle),
            "collections_command docs page should include `{needle}` for command_dialog primary coverage.",
        );
    }
}

#[test]
fn command_dialog_docs_playgrounds_lock_state_matrix_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "id_base=\"docs-command-dialog-controlled\".to_string()",
        "title=\"Quick Actions\".to_string()",
        "description=\"Press ⌘K-style filtering and Enter to run actions.\".to_string()",
        "groups=groups.clone()",
        "open=open",
        "on_open_change=on_open_change",
        "on_action=on_action",
        "\"open: \"",
        "\"last action: \"",
        "id_base=\"docs-command-dialog-marker\".to_string()",
        "title=\"Workspace Commands\".to_string()",
        "description=\"close_on_action=false keeps the dialog open after choosing an action.\".to_string()",
        "groups=marker_groups",
        "default_open=true",
        "close_on_action=false",
        "placeholder=\"Search pages, actions, and settings...\".to_string()",
        "empty_label=\"No command matches your search.\".to_string()",
        "aria_label=\"Workspace command dialog\".to_string()",
        "class_name=\"docs-command-dialog-custom\".to_string()",
        "let marker_overlay_motion = ui_components::OverlayMotion {",
        "initial_scale: 0.95",
        "initial_y_px: 10.0",
        "overlay_motion=marker_overlay_motion",
        "\"close_on_action: false (dialog stays open)\"",
    ] {
        assert!(
            docs.contains(needle),
            "command_dialog docs playgrounds should contain `{needle}`.",
        );
    }
}
