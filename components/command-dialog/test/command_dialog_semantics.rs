use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    if let Some(suffix) = rel_path.strip_prefix("src/command_dialog/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        return workspace_dir
            .join("components/command-dialog/src")
            .join(suffix);
    }

    manifest_dir.join(rel_path)
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    resolve_path(rel_path).exists()
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
        "pub mod motion;",
        "pub use motion::CommandDialogMotion;",
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
        "pub fn resolve_agent_contract(state: CommandDialogPartState) -> CommandDialogAgentContract",
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
        "motion::attach_motion(command_motion, overlay_motion)",
        "logic::normalize_props(logic::CommandDialogNormalizationInput {",
        "logic::resolve_part_state(&normalized, CommandDialogSlot::Root, open.get())",
        "logic::compose_class_name(class_name.get_value(), state)",
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(root_state.get()));",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-action-source=move || agent_contract.get().action_source",
        "data-ui-open-change-source=move || agent_contract.get().open_change_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
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
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-title=move || root_state.get().has_custom_title.then_some(\"true\")",
        "data-custom-description=move ||",
        "data-custom-placeholder=move ||",
        "data-custom-empty-label=move ||",
        "data-custom-aria-label=move ||",
        "data-custom-action=move || root_state.get().has_custom_on_action.then_some(\"true\")",
        "data-custom-open-change=move ||",
        "data-custom-default-open=move ||",
        "data-custom-close-on-action=move ||",
        "data-custom-disabled=move || root_state.get().has_custom_disabled.then_some(\"true\")",
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
        "is_disabled: Option<bool>",
        "disabled: bool",
        "let disabled = logic::normalize_is_disabled(is_disabled, disabled);",
        "logic::normalize_open_state_options(open_input, default_open)",
        "logic::use_overlay_trigger_state(open_state_options)",
        "logic::apply_open_change(state, open_prop.map(|value| value.get_untracked()), next)",
        "let is_controlled = open_prop.is_some()",
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
        ".ui-command-dialog--custom-id",
        ".ui-command-dialog[data-custom-id=\"true\"]",
        ".ui-command-dialog[data-title-source=\"custom\"]",
        ".ui-command-dialog--custom-title",
        ".ui-command-dialog[data-custom-title=\"true\"]",
        ".ui-command-dialog[data-description-source=\"custom\"]",
        ".ui-command-dialog--custom-description",
        ".ui-command-dialog[data-custom-description=\"true\"]",
        ".ui-command-dialog[data-placeholder-source=\"custom\"]",
        ".ui-command-dialog--custom-placeholder",
        ".ui-command-dialog[data-custom-placeholder=\"true\"]",
        ".ui-command-dialog[data-empty-label-source=\"custom\"]",
        ".ui-command-dialog--custom-empty-label",
        ".ui-command-dialog[data-custom-empty-label=\"true\"]",
        ".ui-command-dialog[data-aria-label-source=\"custom\"]",
        ".ui-command-dialog--custom-aria-label",
        ".ui-command-dialog[data-custom-aria-label=\"true\"]",
        ".ui-command-dialog[data-action-source=\"custom\"]",
        ".ui-command-dialog--custom-action",
        ".ui-command-dialog[data-custom-action=\"true\"]",
        ".ui-command-dialog[data-open-change-source=\"custom\"]",
        ".ui-command-dialog[data-class-source=\"custom\"]",
        ".ui-command-dialog[data-custom-class=\"true\"]",
        ".ui-command-dialog--custom-class",
        ".ui-command-dialog[data-default-open-source=\"custom\"]",
        ".ui-command-dialog[data-custom-default-open=\"true\"]",
        ".ui-command-dialog--custom-default-open",
        ".ui-command-dialog[data-close-on-action-source=\"custom\"]",
        ".ui-command-dialog[data-custom-close-on-action=\"true\"]",
        ".ui-command-dialog--custom-close-on-action",
        ".ui-command-dialog[data-disabled-source=\"custom\"]",
        ".ui-command-dialog[data-custom-disabled=\"true\"]",
        ".ui-command-dialog--custom-disabled",
        ".ui-command-dialog--custom-open-change",
        ".ui-command-dialog[data-custom-open-change=\"true\"]",
        ".ui-command-dialog[data-command-motion-source=\"custom\"]",
        ".ui-command-dialog--custom-command-motion",
        ".ui-command-dialog[data-custom-command-motion=\"true\"]",
        ".ui-command-dialog[data-overlay-motion-source=\"custom\"]",
        ".ui-command-dialog--custom-overlay-motion",
        ".ui-command-dialog[data-custom-overlay-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CommandDialog styles should include `{selector}` marker contracts."
        );
    }
}

#[test]
fn command_dialog_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/command_dialog/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for needle in [
        "var(--ui-checkbox-disabled-opacity,",
        "var(--ui-fallback-checkbox-disabled-opacity)",
        "var(--ui-border-width,",
        "var(--ui-fallback-border-width)",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-overlay-viewport-inset,",
        "var(--ui-fallback-overlay-viewport-inset)",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-command-panel-max-width,",
        "var(--ui-fallback-command-panel-max-width)",
        "var(--ui-space-sm,",
        "var(--ui-fallback-space-sm)",
    ] {
        assert!(
            styles_source.contains(needle),
            "command-dialog styles should keep defensive fallback chain marker `{needle}`.",
        );
    }

    for needle in [
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-command-panel-max-width:",
        "--ui-fallback-space-sm:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`.",
        );
    }

    for forbidden in [
        "var(--ui-overlay-panel-min-width, 280px)",
        "calc(100vw -",
        "280px",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "command-dialog styles should avoid raw terminal token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn command_dialog_check2_marks_defensive_variables_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "command-dialog check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "command_dialog_styles_use_defensive_variable_fallback_chain",
        "command_dialog_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "components/command-dialog/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 defensive-variables section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let styles_source = load_source("src/command_dialog/styles.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-command_dialog\")]",
        "out.push_str(crate::command_dialog::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "command-dialog view should not embed plain inline style assignments."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command-dialog view should not include fragile inline style token `{forbidden}`."
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "command-dialog runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for needle in ["pub const CSS: &str", ".ui-command-dialog", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "command-dialog styles should remain static token css contract `{needle}`."
        );
    }
}

#[test]
fn command_dialog_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn command_dialog_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "command-dialog check2 should mark cascade-layer gate complete.",
    );

    for needle in [
        "command_dialog_cascade_layer_and_runtime_style_contract_is_enforced",
        "command_dialog_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "components/command-dialog/src/view.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 cascade-layer section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let command_dialog_motion_source = load_source("src/command_dialog/motion.rs");
    let command_dialog_motion_test_source =
        load_source("../../components/command-dialog/test/motion.rs");
    let command_dialog_view_source = load_source("src/command_dialog/view.rs");
    let overlay_motion_source = load_source("../../components/overlay/src/motion.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_spring_source = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "pub struct CommandDialogMotion {",
        "pub command: CommandMotion,",
        "pub overlay: OverlayMotion,",
        "fn sanitize_command_spring(",
        "stiffness: if spring.stiffness.is_finite() && spring.stiffness > 0.0 {",
        "damping: if spring.damping.is_finite() && spring.damping > 0.0 {",
        "pub fn sanitize_motion(motion: CommandDialogMotion) -> CommandDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
        "pub fn attach_motion(command: CommandMotion, overlay: OverlayMotion) -> CommandDialogMotion",
        "sanitize_motion(CommandDialogMotion { command, overlay })",
    ] {
        assert!(
            command_dialog_motion_source.contains(needle),
            "command-dialog motion module should keep component-scoped motion contract marker `{needle}`.",
        );
    }

    for needle in [
        "fn default_motion_uses_default_overlay_and_command_motion()",
        "fn sanitize_motion_falls_back_for_invalid_numbers()",
        "fn attach_motion_sanitizes_command_and_overlay()",
        "stiffness: f64::NAN,",
        "damping: -1.0,",
    ] {
        assert!(
            command_dialog_motion_test_source.contains(needle),
            "command-dialog motion regression suite should include `{needle}`.",
        );
    }

    for needle in [
        "let motion = motion::attach_motion(command_motion, overlay_motion);",
        "let command_motion = motion.command;",
        "let overlay_motion = motion.overlay;",
        "motion=overlay_motion",
        "motion=command_motion",
    ] {
        assert!(
            command_dialog_view_source.contains(needle),
            "command-dialog view should attach and forward motion contract via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion should keep platform-safe attach path `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion fast path `{needle}`.",
        );
    }

    for forbidden in ["request_animation_frame", "web_sys::", "SpringAnimator"] {
        assert!(
            !command_dialog_motion_source.contains(forbidden),
            "command-dialog motion should avoid driver/runtime coupling token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_motion_contract_platform_script_covers_guard() {
    let source = load_source("../../scripts/check-ui-platforms.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        source.contains(needle),
        "platform check script should enforce `{needle}`.",
    );
}

#[test]
fn command_dialog_check2_marks_motion_contractualization_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    assert!(
        source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "command-dialog check2 should mark motion contractualization gate complete.",
    );

    for needle in [
        "CommandDialogMotion` + `sanitize_motion` -> `overlay::motion::sanitize_motion`",
        "motion::attach_motion(command_motion, overlay_motion)",
        "pub fn prefers_reduced_motion() -> bool",
        "finish_exit.run(())",
        "command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "command_dialog_motion_contract_platform_script_covers_guard",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 motion contractualization section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-command_dialog\")]",
        "pub mod command_dialog;",
        "pub use command_dialog::CommandDialog;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep marker `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entry should not leak platform/internal marker `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-command_dialog\")]",
        "out.push_str(crate::command_dialog::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css registry should keep feature-gated marker `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`.",
        );
    }

    for forbidden in [
        "CommandDialog",
        "Accordion",
        "Button",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`.",
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui forbidden entrypoint file should not exist: `{forbidden}`.",
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`.",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn command_dialog_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries",
        "command_dialog_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "scripts/check-ui-entrypoints.sh",
        "crates/ui/src/lib.rs",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 fixed-entry-files section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_tree_shaking_feature_registration_and_gated_aggregates() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        cargo_source
            .contains("component-command_dialog = [\"component-command\", \"component-modal\"]"),
        "ui feature tree should register command-dialog with minimal dependency chain.",
    );

    for required in [
        "#[cfg(feature = \"component-command_dialog\")]",
        "#[path = \"../../../components/command-dialog/src/mod.rs\"]",
        "pub mod command_dialog;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib should keep command-dialog gate marker `{required}`.",
        );
    }

    let mut css_gate_is_adjacent = false;
    let css_lines: Vec<&str> = css_source.lines().collect();
    for (idx, line) in css_lines.iter().enumerate() {
        if line.contains("out.push_str(crate::command_dialog::styles::CSS);")
            && idx > 0
            && css_lines[idx - 1].contains("#[cfg(feature = \"component-command_dialog\")]")
        {
            css_gate_is_adjacent = true;
        }
    }
    assert!(
        css_gate_is_adjacent,
        "command-dialog CSS aggregation should be directly gated by component feature in css.rs.",
    );
}

#[test]
fn command_dialog_tree_shaking_script_covers_command_dialog_minimal_feature_chain() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "COMMAND_DIALOG_MIN_FEATURES=\"component-command_dialog,inject-css\"",
        "command-dialog minimal feature tree",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$COMMAND_DIALOG_MIN_FEATURES\"",
        "missing command-line feature: component-command_dialog",
        "command-dialog minimal feature tree should not pull all-components",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_tree_shaking_feature_registration_and_gated_aggregates",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_tree_shaking_feature_gating_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "command_dialog_tree_shaking_feature_registration_and_gated_aggregates",
        "command_dialog_tree_shaking_script_covers_command_dialog_minimal_feature_chain",
        "scripts/check-ui-tree-shaking.sh",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-command_dialog,inject-css",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("src/command_dialog/mod.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let styles_source = load_source("src/command_dialog/styles.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let motion_source = load_source("src/command_dialog/motion.rs");

    for required in [
        "src/command_dialog/mod.rs",
        "src/command_dialog/logic.rs",
        "src/command_dialog/styles.rs",
        "src/command_dialog/view.rs",
        "src/command_dialog/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "command-dialog component directory should include `{required}`.",
        );
    }

    for forbidden_file in ["src/command_dialog/render.rs", "src/command_dialog/spec.rs"] {
        assert!(
            !path_exists(forbidden_file),
            "command-dialog component directory should keep `{forbidden_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::CommandDialog;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in ["pub mod logic", "pub mod view", "mod render;", "mod spec;"] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CommandDialogNormalizationInput",
        "pub struct CommandDialogNormalized",
        "pub fn normalize_props(",
        "pub fn resolve_part_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }
    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "NodeRef",
        "<Modal",
        "view!",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay free of DOM/render token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-command-dialog[data-state=\"open\"]",
        ".ui-command-dialog__modal.ui-modal {",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "use ui_headless",
        "use leptos",
        "Command Menu",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid render/headless/business text token `{forbidden}`.",
        );
    }

    for required in [
        "fn render_dialog_view(",
        "#[component]",
        "pub fn CommandDialog(",
        "let motion = motion::attach_motion(command_motion, overlay_motion);",
        "logic::normalize_props(logic::CommandDialogNormalizationInput {",
        "logic::resolve_part_state(&normalized, CommandDialogSlot::Root, open.get())",
        "data-slot=move || root_state.get().slot_attr",
        "use_presence(open)",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in [
        "@keyframes",
        ".ui-command-dialog {",
        "request_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid styling/runtime engine token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CommandDialogMotion",
        "pub fn sanitize_motion(motion: CommandDialogMotion) -> CommandDialogMotion",
        "pub fn attach_motion(command: CommandMotion, overlay: OverlayMotion) -> CommandDialogMotion",
        "crate::overlay::motion::sanitize_motion(motion.overlay)",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep semantic->motion contract mapping marker `{required}`.",
        );
    }
    for forbidden in ["request_animation_frame", "web_sys::", "set_timeout"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should avoid runtime engine token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_component_files_check_script_covers_standard_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_component_directory_standard_files_follow_contract_and_na_paths",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_file_placement_discipline_is_strict_for_component_scope",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_component_directory_standard_files_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "command_dialog_component_directory_standard_files_follow_contract_and_na_paths",
        "command_dialog_component_files_check_script_covers_standard_directory_contract",
        "scripts/check-ui-component-files.sh",
        "components/command-dialog/src/mod.rs",
        "components/command-dialog/src/logic.rs",
        "components/command-dialog/src/styles.rs",
        "components/command-dialog/src/view.rs",
        "components/command-dialog/src/motion.rs",
        "components/command-dialog/src/spec.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 component-directory-standard section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_file_placement_discipline_is_strict_for_component_scope() {
    command_dialog_component_directory_standard_files_follow_contract_and_na_paths();
}

#[test]
fn command_dialog_check2_marks_file_placement_discipline_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "command_dialog_file_placement_discipline_is_strict_for_component_scope",
        "command_dialog_component_directory_standard_files_follow_contract_and_na_paths",
        "scripts/check-ui-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 file-placement-discipline section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_source("src/command_dialog/mod.rs");
    let readme_source = load_source("../../components/command-dialog/src/README.md");
    let check2_source = load_source("../../components/command-dialog/check2.md");

    assert!(
        !path_exists("src/command_dialog/spec.rs"),
        "command-dialog should not add `spec.rs` unless there is a stable external schema contract.",
    );
    assert!(
        path_exists("src/button/spec.rs"),
        "button should remain the canonical complex component that carries `spec.rs`.",
    );

    for forbidden in ["mod spec", "pub mod spec", "spec::", "CommandDialogSpec"] {
        assert!(
            !mod_source.contains(forbidden),
            "command-dialog module boundary should not expose spec module via `{forbidden}`.",
        );
    }

    for forbidden in ["Spec::new(", ".render()", "schema_version", "spec.rs"] {
        assert!(
            !readme_source.contains(forbidden),
            "command-dialog docs should not force Hyper-Structure builder token `{forbidden}` for simple component scope.",
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`command-dialog` 当前为简单组件装配"),
        "command-dialog check2 should keep explicit no-spec-for-simple-component constraint.",
    );
}

#[test]
fn command_dialog_check2_marks_hyper_structure_builder_item_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design：`command-dialog` 当前为简单组件装配",
        "command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "scripts/check-ui-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep Hyper-Structure builder marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "src/command_dialog/Component.toml",
        "src/command_dialog/command_dialog.rbi",
    ] {
        assert!(
            path_exists(required_file),
            "command-dialog context-compression artifact should exist: `{required_file}`.",
        );
    }

    let manifest_source = load_source("src/command_dialog/Component.toml");
    let rbi_source = load_source("src/command_dialog/command_dialog.rbi");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CommandDialog\"",
        "crate = \"ui-command-dialog\"",
        "name = \"groups\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"on_action\"",
        "name = \"close_on_action\"",
        "name = \"data-ui-schema\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "command-dialog Component.toml should keep context-compression marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum CommandDialogSlot {",
        "pub struct CommandDialogMotion {",
        "pub const DEFAULT_ID_BASE: &str;",
        "pub const DEFAULT_TITLE: &str;",
        "pub fn CommandDialog(",
        "groups: std::sync::Arc<[crate::command::CommandGroup]>",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "on_action: Option<leptos::prelude::Callback<String>>",
        "close_on_action: bool",
    ] {
        assert!(
            rbi_source.contains(needle),
            "command-dialog RBI projection should keep signature marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_component_files_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn command_dialog_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "command-dialog check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/command-dialog/src/Component.toml",
        "components/command-dialog/src/command_dialog.rbi",
        "command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "command_dialog_component_files_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "command_dialog_agent_contract_is_schema_typed_and_machine_readable",
        "command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");

    for needle in [
        "pub const COMMAND_DIALOG_AGENT_SCHEMA: &str = \"ui.command-dialog.agent-contract\";",
        "pub enum CommandDialogAgentSchemaVersion",
        "pub enum CommandDialogAgentIntent",
        "pub enum CommandDialogAgentAction",
        "pub enum CommandDialogAgentState",
        "pub enum CommandDialogAgentSource",
        "pub enum CommandDialogAgentStreamSupport",
        "pub enum CommandDialogAgentConfigPolicy",
        "pub enum CommandDialogAgentOutputStatus",
        "pub struct CommandDialogAgentContract",
        "pub fn resolve_agent_contract(state: CommandDialogPartState) -> CommandDialogAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "command-dialog logic should keep typed agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(root_state.get()));",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "command-dialog view should mount schemaized agent marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()
{
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");

    for typed_source in [
        "schema_version: CommandDialogAgentSchemaVersion::V1",
        "intent: CommandDialogAgentIntent::CommandDiscovery",
        "CommandDialogAgentAction::CloseOnAction",
        "CommandDialogAgentAction::KeepOpen",
        "CommandDialogAgentState::Open",
        "CommandDialogAgentState::Closed",
        "CommandDialogAgentSource::Controlled",
        "CommandDialogAgentSource::Uncontrolled",
        "CommandDialogAgentStreamSupport::Required",
        "CommandDialogAgentStreamSupport::Optional",
        "CommandDialogAgentOutputStatus::Draft",
        "CommandDialogAgentOutputStatus::CommitReady",
        "config_policy: CommandDialogAgentConfigPolicy::Whitelist",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "command-dialog agent fields should stay type-derived via `{typed_source}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "command-dialog agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let styles_source = load_source("src/command_dialog/styles.rs");
    let mod_source = load_source("src/command_dialog/mod.rs");
    let motion_source = load_source("src/command_dialog/motion.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "command-dialog render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "command_dialog_check2_documents_agent_contract_schema_governance_rules",
        "command_dialog_agent_contract_is_schema_typed_and_machine_readable",
        "command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "command_dialog_contract_hygiene_script_covers_agent_contract_schema_guards",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep Agent Contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should pin streaming two-mode definition marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot() {
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let manifest_source = load_source("src/command_dialog/Component.toml");
    let readme_source = load_source("src/command_dialog/README.md");

    for needle in [
        "pub enum CommandDialogAgentStreamMode {",
        "Streaming,",
        "Snapshot,",
        "pub enum CommandDialogAgentStreamSupport {",
        "Required,",
        "Optional,",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_support: CommandDialogAgentStreamSupport::Optional,",
        "stream_mode: CommandDialogAgentStreamMode::Snapshot,",
        "stream_fallback: CommandDialogAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "command-dialog logic should keep stream-mode type marker `{needle}`.",
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "command-dialog view should expose stream-mode marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"data-ui-stream-support\"",
        "ty = \"required | optional\"",
        "name = \"data-stream-mode\"",
        "ty = \"streaming | snapshot\"",
        "name = \"data-stream-fallback\"",
        "name = \"streaming_optional_with_snapshot_fallback\"",
        "name = \"llm_streaming_two_display_modes_only\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "command-dialog manifest should keep stream-definition marker `{needle}`.",
        );
    }

    for needle in [
        "## Streaming 策略",
        "- `Snapshot`：默认路径，消费完整配置并稳定渲染。",
        "- `Streaming Optional`：组件不是 LLM 正文阅读面；若上层处于流式生成，组件走 `fallback=snapshot`。",
    ] {
        assert!(
            readme_source.contains(needle),
            "command-dialog README should keep stream strategy marker `{needle}`.",
        );
    }

    for forbidden in ["token-by-token", "delta-patch-mode", "chunk-stream-mode"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden),
            "command-dialog stream contract should avoid undefined mode token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn command_dialog_check2_marks_streaming_two_mode_definition_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "command_dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot",
        "command_dialog_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep streaming two-mode evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should pin snapshot baseline marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");

    for needle in [
        "let normalized = logic::normalize_props(logic::CommandDialogNormalizationInput {",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "groups=groups.get_value()",
        "placeholder=placeholder_text.get_value()",
        "empty_label=empty_label_text.get_value()",
        "aria_label=aria_label_text.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "command-dialog view should keep stable snapshot render marker `{needle}`.",
        );
    }

    for needle in [
        "stream_mode: CommandDialogAgentStreamMode::Snapshot,",
        "stream_fallback: CommandDialogAgentStreamMode::Snapshot,",
        "output_status: CommandDialogAgentOutputStatus::Verified,",
        "pub fn normalize_props(input: CommandDialogNormalizationInput) -> CommandDialogNormalized",
    ] {
        assert!(
            logic_source.contains(needle),
            "command-dialog logic should keep snapshot baseline contract marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "command_dialog_check2_documents_snapshot_as_default_baseline_capability",
        "command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "command_dialog_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep snapshot baseline evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep streaming required/optional rule `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");

    for needle in [
        "stream_support: CommandDialogAgentStreamSupport::Optional,",
        "output_status: CommandDialogAgentOutputStatus::Verified,",
        "CommandDialogAgentOutputStatus::Draft",
        "CommandDialogAgentOutputStatus::CommitReady",
        "<Modal",
        "<Command",
        "aria_label=aria_label_text.get_value()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "command-dialog optional-streaming scope should keep semantic continuity marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let mod_source = load_source("src/command_dialog/mod.rs");
    let motion_source = load_source("src/command_dialog/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "command-dialog should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_streaming_required_optional_classification_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "command_dialog_check2_documents_streaming_required_optional_classification_rules",
        "command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "command_dialog_streaming_script_covers_required_optional_classification_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep required/optional classification evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep semantics-first testing rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/command_dialog_semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for required in [
        "command_dialog_view_uses_logic_state_contracts",
        "command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "command_dialog_e2e_spec_covers_controlled_and_persistent_paths",
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            semantics_source.contains(required),
            "command-dialog semantic test suite should assert contract marker `{required}`.",
        );
    }

    for required in [
        "page.getByRole(\"button\", { name: \"Open CommandDialog\" }).focus()",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-ui-schema\", \"command-dialog\")",
        "toHaveAttribute(\"data-stream-mode\", \"snapshot\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "command-dialog e2e should keep role/data/aria-path marker `{required}`.",
        );
    }

    let forbidden = [
        ["assert", "_snapshot!("].concat(),
        ["insta::assert", "_snapshot!("].concat(),
        ["to_match", "_snapshot("].concat(),
        ["image", "_snapshot("].concat(),
        "toHaveScreenshot".to_string(),
        "toMatchSnapshot".to_string(),
    ];

    for forbidden in forbidden {
        assert!(
            !semantics_source.contains(&forbidden) && !e2e_source.contains(&forbidden),
            "command-dialog semantics should not rely on snapshot-only assertion `{forbidden}` as primary signal.",
        );
    }
}

#[test]
fn command_dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/command_dialog/view.rs");
    let semantics_source = load_source("tests/command_dialog_semantics.rs");

    for marker in [
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-title-source=move || root_state.get().title_source_attr",
        "data-description-source=move || root_state.get().description_source_attr",
        "data-placeholder-source=move || root_state.get().placeholder_source_attr",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "command-dialog view should expose semantic marker `{marker}`.",
        );
        assert!(
            semantics_source.contains(marker),
            "command-dialog semantic marker `{marker}` changed without matching semantics assertion update.",
        );
    }
}

#[test]
fn command_dialog_contract_hygiene_script_covers_semantics_first_contract_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_docs_page_contains_state_source_playground() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "const COMMAND_DIALOG_DOC_IMPORTS: &str =",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "Hello World (Default API)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
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
        "let marker_overlay_motion = ui::OverlayMotion {",
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
        "const COMMAND_DIALOG_DOC_IMPORTS: &str =",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "description=\"baseline-compatible command dialog that composes Modal + Command, supports controlled/uncontrolled open state, emits baseline data contracts, and reuses baseline-level overlay/active-highlight spring motion.\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled Open + Action Close\"",
        "title=\"State + Source Markers\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()",
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
        "id_base=\"docs-command-dialog-hello\".to_string()",
        "title=\"Quick Start\".to_string()",
        "Hello World path: drop in one group and rely on default snapshot rendering.",
        "title=\"State Matrix\"",
        "id_base=\"docs-command-dialog-state-matrix\".to_string()",
        "Switch scenario to inspect data-open-mode/data-close-on-action/data-disabled markers.",
        "state_matrix_options.clone()",
        "state_matrix_is_controlled.get()",
        "state_matrix_close_on_action.get()",
        "state_matrix_disabled.get()",
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
        "let marker_overlay_motion = ui::OverlayMotion {",
        "initial_scale: 0.95",
        "initial_y_px: 10.0",
        "overlay_motion=marker_overlay_motion",
        "\"close_on_action: false (dialog stays open)\"",
        "id_base=\"docs-command-dialog-compare-controlled\".to_string()",
        "id_base=\"docs-command-dialog-compare-uncontrolled\".to_string()",
        "\"Controlled Dialog\".to_string()",
        "\"Uncontrolled Dialog\".to_string()",
        "title=\"Streaming Optional Contract\".to_string()",
        "id_base=\"docs-command-dialog-stream\".to_string()",
        "data-slot=\"command-dialog-streaming-contract\"",
        "data-requested-stream-mode=move || stream_requested_mode.get()",
        "data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs.contains(needle),
            "command_dialog docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");

    command_dialog_docs_page_covers_primary_playgrounds();
    command_dialog_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "open=if state_matrix_is_controlled.get() {",
        "default_open=if state_matrix_is_controlled.get() {",
        "close_on_action=state_matrix_close_on_action.get()",
        "is_disabled=Some(state_matrix_disabled.get())",
        "open=Some(compare_controlled_open)",
        "on_open_change=Some(on_compare_controlled_open_change.clone())",
        "default_open=true",
        "close_on_action=false",
        "description=\"default_open initializes once; primitive owns later transitions.\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "command-dialog docs examples should keep state-matrix/API sync marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_action: Option<Callback<String>>",
        "#[prop(optional, default = logic::DEFAULT_CLOSE_ON_ACTION)] close_on_action: bool",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, default = logic::DEFAULT_DISABLED)] disabled: bool",
    ] {
        assert!(
            view_source.contains(needle),
            "command-dialog view public API should keep `{needle}` for docs/runtime sync.",
        );
    }

    for needle in [
        "pub const DEFAULT_CLOSE_ON_ACTION: bool = true;",
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub const DEFAULT_DEFAULT_OPEN: bool = false;",
    ] {
        assert!(
            logic_source.contains(needle),
            "command-dialog logic defaults should keep `{needle}` for docs consistency.",
        );
    }

    for forbidden in ["is_open=", "default_is_open", "on_change="] {
        assert!(
            !docs_source.contains(forbidden),
            "command-dialog docs should avoid stale/aliased API token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "const COMMAND_DIALOG_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui::{CommandDialog, CommandGroup, CommandItem};",
        "code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "Streaming is optional; fallback stays snapshot.",
        "requested mode:",
        "requested output status:",
        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs_source.contains(needle),
            "command-dialog docs should keep copy-ready + streaming/snapshot contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`.",
        );
    }

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "description=\"Tune close-on-action/disabled/motion while optionally preserving open+action context in an isolated command-dialog canvas.\"",
        "code_signal=workbench_code",
        "code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/command-dialog/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "ui::command_dialog::styles::CSS",
        "let (workbench_preserve_context, set_workbench_preserve_context) = signal(true);",
        "if !workbench_preserve_context.get() {",
        "set_last_workbench_action.set(\"none\".to_string());",
        "<Switch",
        "\" Preserve open/action context (optional)\"",
        "data-slot=\"command-dialog-workbench-controls\"",
        "data-slot=\"command-dialog-workbench\"",
        "data-slot=\"command-dialog-workbench-actions\"",
        "data-slot=\"command-dialog-workbench-canvas\"",
        "open=workbench_open",
        "on_open_change=on_workbench_open_change",
        "on_action=on_workbench_action",
        "close_on_action=workbench_close_on_action.get()",
        "is_disabled=Some(workbench_disabled.get())",
    ] {
        assert!(
            docs_source.contains(needle),
            "command_dialog workbench should keep DX marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    command_dialog_docs_page_covers_primary_playgrounds();
    command_dialog_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "description=\"Tune close-on-action/disabled/motion while optionally preserving open+action context in an isolated command-dialog canvas.\"",
        "code_signal=workbench_code",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"command-dialog-workbench-controls\"",
        "data-slot=\"command-dialog-workbench-canvas\"",
        "id_base=\"docs-command-dialog-workbench\".to_string()",
        "open=workbench_open",
        "on_open_change=on_workbench_open_change",
        "on_action=on_workbench_action",
        "close_on_action=workbench_close_on_action.get()",
        "is_disabled=Some(workbench_disabled.get())",
        "title=\"Streaming / Snapshot Contract\"",
        "data-requested-stream-mode=move || stream_requested_mode.get()",
        "data-requested-output-status=move || stream_requested_output_status.get()",
        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs_source.contains(needle),
            "command-dialog docs playground should keep interactive marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for needle in [
        "docs-app command-dialog key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/command-dialog\");",
        "[data-slot=\"command-dialog\"][data-ui-schema=\"command-dialog\"]",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"open\")",
        "await page.keyboard.press(\"Escape\");",
        "toHaveCount(0);",
        "await page.reload();",
        "[data-slot=\"command-dialog\"][data-open-mode=\"uncontrolled\"]",
    ] {
        assert!(
            e2e_source.contains(needle),
            "command-dialog interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"command-dialog-source-first\"",
        "<h3>\"Source-first Copy-Paste\"</h3>",
        "<code>\"Show code\"</code>",
        "COMMAND_DIALOG_DOC_IMPORTS",
        "compose_copy_ready_code",
        "code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()",
        "Dependency prerequisites",
        "component-command_dialog",
        "inject-css",
        "data-slot=\"command-dialog-source-paths\"",
        "components/command-dialog/src/mod.rs",
        "components/command-dialog/src/logic.rs",
        "components/command-dialog/src/view.rs",
        "components/command-dialog/src/styles.rs",
        "components/command-dialog/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "command-dialog source-first docs should contain `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`."
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`."
        );
    }
}

#[test]
fn command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let readme_source = load_source("../../components/command-dialog/src/README.md");
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");

    for needle in [
        "### CommandDialog 同步记录（2026-02-20）",
        "`CommandDialog` 参数主轴保持 `open + on_open_change + default_open`、`close_on_action`、`is_disabled/disabled`、`on_action`、`placeholder/empty_label/aria_label`、`command_motion/overlay_motion`、`class_name`",
        "component_doc!(\"CommandDialog\", \"command-dialog\", \"Collections\", collections_command::command_dialog)",
        "`#/components/command-dialog` 可索引访问。",
        "components/command-dialog/src/README.md",
        "研究文档补充判定：本轮为 CommandDialog 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_index_source.contains(needle),
            "command-dialog HeroUI/doc sync record should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "command-dialog docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in ["# CommandDialog", "## Hello World", "## API 约定"] {
        assert!(
            readme_source.contains(needle),
            "command-dialog README should keep docs entry marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_action: Option<Callback<String>>",
        "#[prop(optional, default = logic::DEFAULT_CLOSE_ON_ACTION)] close_on_action: bool",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] command_motion: CommandMotion",
        "#[prop(optional)] overlay_motion: OverlayMotion",
        "pub const DEFAULT_CLOSE_ON_ACTION: bool = true;",
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub const DEFAULT_DEFAULT_OPEN: bool = false;",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "command-dialog parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn command_dialog_dx_check_script_covers_docs_sync_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix guard `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test button_semantics --no-default-features --features component-button,inject-css button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce command-dialog interactive playground guard `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce command-dialog source-first guard `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn command_dialog_check2_marks_docs_sync_and_state_matrix_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "command_dialog_check2_documents_docs_sync_and_state_matrix_rules",
        "command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "command_dialog_dx_check_script_covers_docs_sync_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 docs-sync/state-matrix section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World (Default API)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "COMMAND_DIALOG_DOC_IMPORTS",
        "compose_copy_ready_code",
        "command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "command_dialog_dx_check_script_covers_hot_reload_and_workbench_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_interactive_playground_contract_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "command_dialog_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 interactive-playground section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_source_first_copy_paste_ready_rules() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "command_dialog_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 source-first section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "command_dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog check2 HeroUI/doc sync section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/command_dialog/mod.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let styles_source = load_source("src/command_dialog/styles.rs");
    let motion_source = load_source("src/command_dialog/motion.rs");
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    assert!(
        !resolve_path("src/command_dialog/spec.rs").exists(),
        "CommandDialog should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source
            .contains("component-command_dialog = [\"component-command\", \"component-modal\"]"),
        "CommandDialog feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-command_dialog = [\"dep:serde\"")
            && !cargo_source.contains("component-command_dialog = [\"dep:serde_json\""),
        "CommandDialog should not opt into serde/spec migration dependencies without an explicit schema contract."
    );
    assert!(
        !mod_source.contains("mod protocol;") && !mod_source.contains("pub mod protocol;"),
        "CommandDialog public module boundary should not export dormant protocol/spec surface."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "CommandDialog engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "CommandDialog checklist should keep engineering governance rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/command-dialog/src/Component.toml");
    let rbi_source = load_source("../../components/command-dialog/src/command_dialog.rbi");
    let mod_source = load_source("../../components/command-dialog/src/mod.rs");
    let logic_source = load_source("../../components/command-dialog/src/logic.rs");
    let view_source = load_source("../../components/command-dialog/src/view.rs");
    let styles_source = load_source("../../components/command-dialog/src/styles.rs");
    let motion_source = load_source("../../components/command-dialog/src/motion.rs");
    let protocol_source = load_source("../../components/command-dialog/src/protocol.rs");
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CommandDialog\"",
        "crate = \"ui-command-dialog\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "CommandDialog manifest should keep stable v1 schema marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn CommandDialog(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "on_action: Option<leptos::prelude::Callback<String>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "CommandDialog RBI should keep stable public API marker `{needle}`.",
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "CommandDialog should not introduce major-version migration marker `{forbidden}` in current scope.",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CommandDialog` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "migrate_v1_to_v2",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog/check2.md should keep version-migration governance marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/command_dialog/view.rs");
    let combined = [
        load_source("src/command_dialog/mod.rs"),
        load_source("src/command_dialog/logic.rs"),
        view_source.clone(),
        load_source("src/command_dialog/styles.rs"),
        load_source("src/command_dialog/motion.rs"),
    ]
    .join("\n");

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    for required in [
        "use ui_headless::{Presence, UiTraceEventKind, use_presence, use_ui_trace};",
        "let trace = use_ui_trace();",
        "trace.emit(",
        "UiTraceEventKind::OpenChange { open: next },",
    ] {
        assert!(
            view_source.contains(required),
            "CommandDialog should reuse shared headless tracing semantics via `{required}`.",
        );
    }

    for forbidden_feature in [
        "command-dialog-wasm-debug",
        "command_dialog-wasm-debug",
        "component-command_dialog-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "CommandDialog should not define component-local tracing feature `{forbidden_feature}`.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::command_dialog::",
        "const COMMAND_DIALOG_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "CommandDialog should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn command_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/command_dialog/mod.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let view_source = load_source("src/command_dialog/view.rs");
    let styles_source = load_source("src/command_dialog/styles.rs");
    let motion_source = load_source("src/command_dialog/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "CommandDialog engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "CommandDialog public module boundary should not leak web_sys types."
    );
}

#[test]
fn command_dialog_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn command_dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources() {
    let mut combined = String::new();
    for rel_path in [
        "src/command_dialog/lib.rs",
        "src/command_dialog/mod.rs",
        "src/command_dialog/logic.rs",
        "src/command_dialog/motion.rs",
        "src/command_dialog/protocol.rs",
        "src/command_dialog/styles.rs",
        "src/command_dialog/view.rs",
    ] {
        combined.push_str(&load_source(rel_path));
        combined.push('\n');
    }

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "command-dialog non-test sources should avoid rust-hygiene violation `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str() {
    let logic_source = load_source("src/command_dialog/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];",
        "Cow::Borrowed(\"ui-command-dialog--open\")",
        "Cow::Borrowed(\"ui-command-dialog--custom-overlay-motion\")",
        "classes.push(Cow::Owned(base_class_name));",
    ] {
        assert!(
            logic_source.contains(required),
            "command-dialog string hotspot path should keep Cow marker `{required}`.",
        );
    }
}

#[test]
fn command_dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "find crates apps -type f -name '*.rs' -path '*/src/*' | sort",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "command_dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources",
        "command_dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str",
        "command_dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 rust hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_readme_provides_onboarding_examples() {
    let readme = load_source("src/command_dialog/README.md");

    for needle in [
        "# CommandDialog",
        "## Hello World",
        "## 受控打开状态",
        "## 先用起来，再进阶",
        "<CommandDialog",
        "open + on_open_change + default_open",
        "data-ui-*",
        "Source-first",
    ] {
        assert!(
            readme.contains(needle),
            "command_dialog README should include `{needle}`."
        );
    }
}

#[test]
fn command_dialog_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_source("src/command_dialog/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "# CommandDialog",
        "## Hello World",
        "## 受控打开状态",
        "## 先用起来，再进阶",
        "## API 约定",
        "## Streaming 策略",
        "<CommandDialog groups=groups />",
        "默认路径：`<CommandDialog groups=... />`",
        "进阶控制：按需启用 `open/on_open_change/default_open`",
    ] {
        assert!(
            readme.contains(needle),
            "command-dialog README should include beginner-friendly marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled Open + Action Close\"",
        "title=\"State + Source Markers\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "command-dialog docs entry should include `{needle}`.",
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("CommandDialog README should include Hello World section");
    let readme_controlled = readme
        .find("## 受控打开状态")
        .expect("CommandDialog README should include controlled section");
    let readme_progressive = readme
        .find("## 先用起来，再进阶")
        .expect("CommandDialog README should include beginner-to-advanced section");
    let readme_streaming = readme
        .find("## Streaming 策略")
        .expect("CommandDialog README should include streaming section");
    assert!(
        readme_hello < readme_controlled
            && readme_controlled < readme_progressive
            && readme_progressive < readme_streaming,
        "CommandDialog README should keep default path before advanced guidance.",
    );

    let docs_hello = docs_source
        .find("title=\"Hello World (Default API)\"")
        .expect("CommandDialog docs should include Hello World playground");
    let docs_matrix = docs_source
        .find("title=\"State Matrix\"")
        .expect("CommandDialog docs should include state matrix playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled Open + Action Close\"")
        .expect("CommandDialog docs should include controlled playground");
    let docs_advanced = docs_source
        .find("title=\"Workbench (Display + Config + Code + CSS Test)\"")
        .expect("CommandDialog docs should include workbench playground");

    assert!(
        docs_hello < docs_matrix
            && docs_matrix < docs_controlled
            && docs_controlled < docs_advanced,
        "CommandDialog docs should keep beginner-first order before advanced controls.",
    );
}

#[test]
fn command_dialog_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "command_dialog_check2_documents_documentation_as_product_rules",
        "command_dialog_documentation_entry_exists_with_beginner_first_progression",
        "command_dialog_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep documentation-as-product evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_e2e_spec_covers_controlled_and_persistent_paths() {
    let e2e = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for needle in [
        "docs-app command-dialog controlled playground closes on action",
        "docs-app command-dialog marker playground stays open when close_on_action=false",
        "docs-app command-dialog key flow is repeatable with semantic breakpoints",
        "/#/components/command-dialog",
        "const dialogs = page.locator('[data-slot=\"command-dialog\"][data-ui-schema=\"command-dialog\"]');",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
        "page.getByRole(\"button\", { name: \"Open CommandDialog\" }).focus()",
        "await expect(openButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await page.keyboard.press(\"Escape\");",
        "await page.reload();",
        "[data-slot=\"command-dialog\"][data-open-mode=\"controlled\"]",
        "[data-slot=\"command-dialog\"][data-open-mode=\"uncontrolled\"]",
        "#docs-command-dialog-controlled-command-option-0",
        "await page.locator(\"#docs-command-dialog-controlled-command-option-0\").click();",
        "[data-slot=\"command-dialog-last-action\"][data-open-mode=\"controlled\"]",
        "#docs-command-dialog-marker-command-option-0",
        "[data-slot=\"command-dialog-last-action\"][data-open-mode=\"uncontrolled\"]",
        "data-ui-schema",
        "data-stream-mode",
        "data-stream-fallback",
    ] {
        assert!(
            e2e.contains(needle),
            "command_dialog e2e spec should include `{needle}`."
        );
    }
}

#[test]
fn command_dialog_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep e2e selector/stable-wait rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for needle in [
        "/#/components/command-dialog",
        "[data-slot=\"command-dialog\"][data-ui-schema=\"command-dialog\"]",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
        "page.getByRole(\"button\", { name: \"Open CommandDialog\" }).focus()",
        "[data-slot=\"command-dialog\"][data-open-mode=\"controlled\"]",
        "[data-slot=\"command-dialog\"][data-open-mode=\"uncontrolled\"]",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveCount(0);",
        "toHaveAttribute(\"data-stream-fallback\", \"snapshot\")",
        "[data-slot=\"command-dialog-last-action\"][data-open-mode=\"controlled\"]",
        "[data-slot=\"command-dialog-last-action\"][data-open-mode=\"uncontrolled\"]",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
        "toHaveAttribute(\"data-last-action\", \"new-file\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "command-dialog e2e selector/stable-wait contract should include `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep(", "getByText("] {
        assert!(
            !e2e_source.contains(forbidden),
            "command-dialog e2e contract should avoid flaky/non-semantic wait or selector `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("../../components/command-dialog/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command-dialog checklist should keep repeatable key-flow rule `{required}`.",
        );
    }
}

#[test]
fn command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for needle in [
        "docs-app command-dialog key flow is repeatable with semantic breakpoints",
        "await openButton.focus();",
        "await expect(openButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(controlledDialog).toHaveAttribute(\"data-state\", \"open\");",
        "await expect(controlledDialog).toHaveAttribute(\"data-ui-schema\", \"command-dialog\");",
        "await controlledFirstOption.focus();",
        "await expect(controlledFirstOption).toBeFocused();",
        "await page.keyboard.press(\"Escape\");",
        "await page.reload();",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "command-dialog repeatable key-flow contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "toHaveScreenshot(",
        "toMatchSnapshot(",
        "waitForTimeout(",
        "setTimeout(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "command-dialog repeatable key-flow should avoid flaky/non-semantic token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dialog_e2e_check_script_covers_selector_contract() {
    assert!(
        path_exists("../../components/command-dialog/scripts/check-ui-e2e-command-dialog.sh"),
        "command-dialog e2e check script should exist.",
    );

    let script_source = load_source("../../components/command-dialog/scripts/check-ui-e2e-command-dialog.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
    ] {
        assert!(
            script_source.contains(needle),
            "command-dialog e2e selector check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_e2e_check_script_covers_selector_and_key_flow_contracts() {
    assert!(
        path_exists("../../components/command-dialog/scripts/check-ui-e2e-command-dialog.sh"),
        "command-dialog e2e check script should exist.",
    );

    let script_source = load_source("../../components/command-dialog/scripts/check-ui-e2e-command-dialog.sh");

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "command-dialog e2e check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_check2_marks_e2e_repeatable_key_flow_contract_complete() {
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "command_dialog_e2e_check_script_covers_selector_and_key_flow_contracts",
        "components/command-dialog/scripts/check-ui-e2e-command-dialog.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "command-dialog check2 should keep repeatable key-flow evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("../../components/command-dialog/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/command_dialog/view.rs");

    for needle in [
        "component_doc!(",
        "\"CommandDialog\"",
        "\"command-dialog\"",
        "collections_command::command_dialog",
    ] {
        assert!(
            pages_source.contains(needle),
            "CommandDialog docs page should stay in component coverage traversal via `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn command_dialog() -> AnyView",
        "title=\"CommandDialog\"",
        "slug=\"command-dialog\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "CommandDialog docs page should mount through ComponentPage contract `{needle}`.",
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget/probe wiring via `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf marker `{needle}`.",
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce perf guard `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-command-motion-source=move || root_state.get().command_motion_source_attr",
        "data-overlay-motion-source=move || root_state.get().overlay_motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view should expose perf triage marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
        "command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "CommandDialog check2 should include performance governance evidence token `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/command-dialog/src/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");
    let check2_source = load_source("../../components/command-dialog/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let semantics_source = load_source("tests/command_dialog_semantics.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-action-source=move || root_state.get().action_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-command-motion-source=move || root_state.get().command_motion_source_attr",
        "data-overlay-motion-source=move || root_state.get().overlay_motion_source_attr",
        "aria_label=aria_label_text.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view should keep semantics/data marker `{needle}`.",
        );
    }

    for needle in [
        "page.getByRole(\"button\", { name: \"Open CommandDialog\" }).focus()",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-ui-schema\", \"command-dialog\")",
        "toHaveAttribute(\"data-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "command-dialog e2e should keep focus/aria/data assertion marker `{needle}`.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "command-dialog e2e should avoid snapshot-only assertion token `{forbidden}`.",
        );
    }

    let perf_gate_needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking";
    assert!(
        perf_script_source.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`.",
    );

    let matrix_gate_needle = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement";
    assert!(
        perf_script_source.contains(matrix_gate_needle),
        "performance gate script should include `{matrix_gate_needle}`.",
    );

    assert!(
        semantics_source.contains(
            "fn command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking()"
        ),
        "command-dialog semantics suite should keep a dedicated blocking performance governance test.",
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up should keep `{needle}` marker.",
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "command_dialog_e2e_spec_covers_controlled_and_persistent_paths",
        "command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
    ] {
        assert!(
            check2_source.contains(needle),
            "CommandDialog check2 should keep semantics+performance evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_view_macro_complexity_is_bounded_and_semantically_partitioned() {
    let view_source = load_source("src/command_dialog/view.rs");

    assert!(
        view_source.contains("view! {"),
        "CommandDialog should keep explicit render block in view.rs.",
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "CommandDialog should avoid multi-mega macro expansion; keep one bounded root render block.",
    );
    assert!(
        view_source.lines().count() <= 260,
        "CommandDialog view.rs grew beyond complexity guardrail; split semantic sections into local helpers.",
    );

    for needle in [
        "use_presence(open)",
        "<Show",
        "<Modal",
        "<Command",
        "data-slot=move || root_state.get().slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view should keep semantic partition marker `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_prefers_functional_view_split_over_extra_local_components() {
    let view_source = load_source("src/command_dialog/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "CommandDialog should keep exactly one public component boundary; static/light fragments must stay as plain functions.",
    );

    for needle in [
        "fn render_dialog_view(",
        ") -> impl IntoView {",
        "render_dialog_view(",
        "let presence = use_presence(open);",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view split contract should include `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_static_fragment_constantization_is_not_applicable_for_lightweight_markup() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");

    for forbidden in ["<svg", "inner_html", "<footer"] {
        assert!(
            !view_source.contains(forbidden),
            "CommandDialog should not carry heavy static fragments `{forbidden}` in view.rs.",
        );
    }

    for needle in [
        "pub const DEFAULT_ID_BASE",
        "pub const DEFAULT_TITLE",
        "resolve_text_with_empty_default(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CommandDialog logic should centralize static fallback text contract via `{needle}`.",
        );
    }

    for needle in [
        "description=description_text.get_value()",
        "placeholder=placeholder_text.get_value()",
        "empty_label=empty_label_text.get_value()",
        "aria_label=aria_label_text.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog view should consume normalized text values via `{needle}`.",
        );
    }
}

#[test]
fn command_dialog_forbids_inner_html_injection_paths() {
    let view_source = load_source("src/command_dialog/view.rs");
    let logic_source = load_source("src/command_dialog/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "insert_adjacent_html",
        "dangerously_set_inner_html",
        "<script",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CommandDialog view should forbid html injection token `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "CommandDialog logic should forbid html injection token `{forbidden}`.",
        );
        assert!(
            !docs_source.contains(forbidden),
            "CommandDialog docs examples should forbid html injection token `{forbidden}`.",
        );
    }

    for forbidden in ["evaluate(", "innerHTML"] {
        assert!(
            !e2e_source.contains(forbidden),
            "CommandDialog e2e should avoid dynamic html/script execution token `{forbidden}`.",
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog should keep semantic marker `{needle}` while forbidding inner_html.",
        );
    }
}

#[test]
fn command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated() {
    let view_source = load_source("src/command_dialog/view.rs");
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_command_dialog.spec.mjs");
    let wasm_debug_script = load_source("../../scripts/check-ui-wasm-debug.sh");
    let check2_source = load_source("../../components/command-dialog/check2.md");

    for needle in [
        "let trace = use_ui_trace();",
        "let current = open_state.with_untracked(|state| state.is_open());",
        "UiTraceEventKind::OpenChange { open: next },",
        "trace.emit(",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CommandDialog should keep wasm-debug traceability marker `{needle}`.",
        );
    }

    for forbidden_feature in [
        "command-dialog-wasm-debug",
        "command_dialog-wasm-debug",
        "component-command_dialog-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "CommandDialog should not expose component-local wasm debug feature `{forbidden_feature}`.",
        );
    }
    for required_feature in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(required_feature),
            "ui should keep shared wasm-debug opt-in feature `{required_feature}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should keep dev-only wasm debug overlay entry `{needle}`.",
        );
    }

    for needle in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "UiTraceEventKind::OpenChange { open }",
        "UiTraceEventKind::Inspect { tag, data_slot }",
        "UiTraceEventKind::Note { message }",
        "ts_ms",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep replay/inspection timeline marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "headless trace contract should keep timestamped event model `{needle}`.",
        );
    }

    for needle in [
        "docs-app command-dialog controlled playground closes on action",
        "page.getByRole(\"button\", { name: \"Open CommandDialog\" }).focus()",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveCount(0);",
        "docs-app command-dialog marker playground stays open when close_on_action=false",
        "toHaveAttribute(\"data-state\", \"open\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "CommandDialog interaction path should stay reproducible/replayable via `{needle}`.",
        );
    }

    let gate_command = "cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated";
    assert!(
        wasm_debug_script.contains(gate_command),
        "wasm-debug gate script should include command-dialog contract command."
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "CommandDialog check2 should keep wasm-debug governance marker `{needle}`.",
        );
    }
}
