use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn command_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/command/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Command internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn command_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/command/mod.rs");

    for needle in [
        "pub struct CommandItem",
        "pub struct CommandGroup",
        "pub struct FilteredCommandItem",
        "pub struct FilteredCommandGroup",
        "pub struct CommandFilterState",
        "pub enum CommandSlot",
        "pub struct CommandPartStateInput",
        "pub struct CommandPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_PLACEHOLDER",
        "DEFAULT_EMPTY_LABEL",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_DISABLED",
        "pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion as CommandMotion;",
    ] {
        assert!(
            source.contains(needle),
            "command::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn command_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/command/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Command;"),
        "command module should export `Command`."
    );
    assert!(
        crate_source
            .contains("pub use command::{Command, CommandGroup, CommandItem, CommandMotion};"),
        "crate root should re-export command contracts."
    );
}

#[test]
fn command_logic_exposes_state_helpers() {
    let source = load_source("src/command/logic.rs");

    for needle in [
        "pub fn state_attr(item_count: usize, is_disabled: bool, has_query: bool)",
        "pub fn item_attr(item_count: usize)",
        "pub fn group_attr(group_count: usize)",
        "pub fn query_attr(has_query: bool)",
        "pub fn disabled_attr(is_disabled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_placeholder(value: Option<String>)",
        "pub fn resolve_empty_label(value: Option<String>)",
        "pub fn resolve_aria_label(value: Option<String>)",
        "pub fn filter_groups(groups: &[CommandGroup], query: &str)",
        "pub fn resolve_state(input: CommandPartStateInput) -> CommandPartState",
        "pub fn compose_class_name(class_name: Option<String>, state: CommandPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Command logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn command_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/command/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_placeholder(placeholder)",
        "logic::resolve_empty_label(empty_label)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_state(CommandPartStateInput {",
        "slot: CommandSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-groups=move || root_state.get().group_attr",
        "data-query=move || root_state.get().query_attr",
        "data-disabled=move || root_state.get().disabled_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-placeholder-source=move || root_state.get().placeholder_source_attr",
        "data-empty-label-source=move || root_state.get().empty_label_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-disabled-source=move || root_state.get().disabled_source_attr",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-placeholder=move || root_state.get().has_custom_placeholder.then_some(\"true\")",
        "data-custom-empty-label=move || root_state.get().has_custom_empty_label.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=move || root_state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-disabled=move || root_state.get().has_custom_disabled.then_some(\"true\")",
        "data-custom-action=move || root_state.get().has_custom_on_action.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Command view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn command_view_has_keyboard_filter_and_slot_contracts() {
    let source = load_source("src/command/view.rs");

    for needle in [
        "use_listbox(ListBoxOptions",
        "attach_active_highlight_motion",
        "data-slot=input_slot.as_attr()",
        "data-slot=list_slot.as_attr()",
        "data-slot=item_slot.as_attr()",
        "role=\"combobox\"",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "on:keydown=on_input_key_down",
    ] {
        assert!(
            source.contains(needle),
            "Command view should include `{needle}` for stable behavior contracts."
        );
    }
}

#[test]
fn command_uses_active_highlight_motion_contract() {
    let source = load_source("src/command/view.rs");

    for needle in [
        "use ui_visual_primitive::active_highlight::{",
        "attach_active_highlight_motion",
        "ActiveHighlightMotion",
        "let options_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "attach_active_highlight_motion(",
        "data-slot=highlight_slot.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "Command should compose active-highlight motion via `{needle}` for baseline-level feedback continuity."
        );
    }
}

#[test]
fn command_styles_include_state_and_source_markers() {
    let css = load_source("src/command/styles.rs");

    for needle in [
        ".ui-command {",
        ".ui-command--disabled",
        ".ui-command[data-disabled=\"disabled\"]",
        ".ui-command--querying",
        ".ui-command[data-query=\"present\"]",
        ".ui-command[data-id-source=\"custom\"]",
        ".ui-command[data-custom-id=\"true\"]",
        ".ui-command--custom-id",
        ".ui-command[data-placeholder-source=\"custom\"]",
        ".ui-command[data-custom-placeholder=\"true\"]",
        ".ui-command--custom-placeholder",
        ".ui-command[data-empty-label-source=\"custom\"]",
        ".ui-command[data-custom-empty-label=\"true\"]",
        ".ui-command--custom-empty-label",
        ".ui-command[data-aria-label-source=\"custom\"]",
        ".ui-command[data-custom-aria-label=\"true\"]",
        ".ui-command--custom-aria-label",
        ".ui-command[data-class-source=\"custom\"]",
        ".ui-command[data-custom-class=\"true\"]",
        ".ui-command--custom-class",
        ".ui-command[data-disabled-source=\"custom\"]",
        ".ui-command[data-custom-disabled=\"true\"]",
        ".ui-command--custom-disabled",
        ".ui-command[data-action-source=\"custom\"]",
        ".ui-command[data-custom-action=\"true\"]",
        ".ui-command--custom-action",
        ".ui-command[data-motion-source=\"custom\"]",
        ".ui-command--custom-motion",
        ".ui-command[data-custom-motion=\"true\"]",
        ".ui-command__option[data-state=\"selected\"] .ui-command__item-label",
    ] {
        assert!(
            css.contains(needle),
            "Command CSS should include `{needle}` as stable state/source contracts."
        );
    }
}

#[test]
fn command_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Command\"",
        "slug=\"command\"",
        "State + Source Markers",
        "data-id-source",
        "data-placeholder-source",
        "data-empty-label-source",
        "data-aria-label-source",
        "data-action-source",
        "data-motion-source",
        "<Command",
    ] {
        assert!(
            source.contains(needle),
            "Command docs page should contain `{needle}`."
        );
    }
}

#[test]
fn command_docs_custom_placeholder_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Custom Placeholder + Empty Label + Disabled Items\"",
        "id_base=\"docs-command-custom\".to_string()",
        "placeholder=\"Search pages, actions, and settings...\".to_string()",
        "empty_label=\"No command matches your search.\".to_string()",
        "class_name=\"docs-command-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Command docs custom-placeholder playground should contain `{needle}`.",
        );
    }
}

#[test]
fn command_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "let mut marker_motion = ui_components::CommandMotion::default();",
        "marker_motion.spring.stiffness = 240.0",
        "marker_motion.spring.damping = 20.0",
        "id_base=\"docs-command-markers\".to_string()",
        "placeholder=\"Search workspace actions...\".to_string()",
        "empty_label=\"No workspace action found.\".to_string()",
        "aria_label=\"Workspace command center\".to_string()",
        "class_name=\"docs-command-custom\".to_string()",
        "motion=marker_motion",
        "Inspect data-id-source / data-placeholder-source / data-empty-label-source / data-aria-label-source / data-action-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "Command docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn command_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Command\"",
        "slug=\"command\"",
        "description=\"baseline-compatible command palette with grouped filtering, listbox keyboard semantics, baseline data contracts, and baseline-level spring active-highlight motion.\"",
        "<Playground title=\"Grouped Search + Keyboard Action\" code_signal=code>",
        "<Playground title=\"Custom Placeholder + Empty Label + Disabled Items\" code_signal=states_code>",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "data-id-source",
        "data-placeholder-source",
        "data-empty-label-source",
        "data-aria-label-source",
        "data-action-source",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "collections_command docs page should include `{needle}` for primary coverage.",
        );
    }
}

#[test]
fn command_docs_interactive_playground_exposes_config_code_css_test_sections() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view! {",
        "data-slot=\"command-workbench-controls\"",
        "id_base=\"docs-command-workbench-scenario\".to_string()",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/command/styles.rs\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "command interactive docs playground should include `{needle}`.",
        );
    }
}

#[test]
fn command_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "id_base=\"docs-command-default\".to_string()",
        "groups=groups.clone()",
        "on_action=on_action",
        "id_base=\"docs-command-custom\".to_string()",
        "groups=custom_groups",
        "on_action=on_custom_action",
        "placeholder=\"Search pages, actions, and settings...\".to_string()",
        "empty_label=\"No command matches your search.\".to_string()",
        "class_name=\"docs-command-custom\".to_string()",
        "id_base=\"docs-command-markers\".to_string()",
        "groups=marker_groups",
        "on_action=on_marker_action",
        "placeholder=\"Search workspace actions...\".to_string()",
        "empty_label=\"No workspace action found.\".to_string()",
        "aria_label=\"Workspace command center\".to_string()",
        "marker_motion.spring.stiffness = 240.0",
        "marker_motion.spring.damping = 20.0",
        "motion=marker_motion",
        "\"last action: \"",
    ] {
        assert!(
            source.contains(needle),
            "command docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn command_check2_marks_core_sections_complete() {
    let source = load_source("src/command/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "### 10. Command 本轮验收证据",
        "component-command -> component-active_highlight",
        "component-command_dialog -> component-command + component-modal",
        "crates/ui-components/src/command_dialog/view.rs",
        "crates/ui-components/src/modal/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "Command check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn command_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/command/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "command check2 should not keep unchecked checklist items"
    );
}
