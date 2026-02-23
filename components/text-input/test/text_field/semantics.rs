use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn text_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/text_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TextField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn text_field_component_files_keep_single_responsibility_boundaries() {
    let module_source = load_source("src/text_input/text_field/mod.rs");
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let styles_source = load_source("src/text_input/text_field/styles.rs");
    let motion_source = load_source("src/text_input/text_field/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::TextField;",
    ] {
        assert!(
            module_source.contains(needle),
            "TextField module boundary should include `{needle}`."
        );
    }

    assert!(
        !module_source.contains("spec.rs") && !module_source.contains("pub mod spec"),
        "TextField should not introduce a `spec.rs` module for this simple component."
    );

    for needle in [
        "pub struct TextFieldResolvedProps",
        "pub fn resolve_props(",
        "pub struct ValueAxisInput",
        "pub struct AccessibilityStateInput",
    ] {
        assert!(
            logic_source.contains(needle),
            "TextField logic should include `{needle}`."
        );
    }

    for forbidden in ["web_sys", "NodeRef<", "view!"] {
        assert!(
            !logic_source.contains(forbidden),
            "TextField logic should not include DOM/view details `{forbidden}`."
        );
    }

    for needle in [
        "use_text_field_contract",
        "use_text_field",
        "use_controllable_state",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view should mount logic/headless contracts via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "--ui-text-field-motion-duration",
        ".ui-text-field[data-state=\"invalid\"] .ui-text-field__input",
    ] {
        assert!(
            styles_source.contains(needle),
            "TextField styles should include `{needle}`."
        );
    }

    for needle in [
        "pub struct TextFieldMotion",
        "pub fn sanitize_motion(motion: TextFieldMotion) -> TextFieldMotion",
        "pub fn motion_style_vars(motion: TextFieldMotion) -> String",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "TextField motion module should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn text_field_component_directory_standard_files_follow_contracts() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let text_field_dir = manifest_dir.join("src/text_input/text_field");
    let mod_source = load_source("src/text_input/text_field/mod.rs");
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let styles_source = load_source("src/text_input/text_field/styles.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let motion_source = load_source("src/text_input/text_field/motion.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            text_field_dir.join(required).exists(),
            "TextField component directory should contain `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !text_field_dir.join(forbidden).exists(),
            "TextField should not introduce `{forbidden}` for this component scope."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_LABEL;",
        "pub use motion::TextFieldMotion;",
        "pub use view::TextField;",
    ] {
        assert!(
            mod_source.contains(needle),
            "TextField mod.rs should keep minimal stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod render;",
        "pub mod spec;",
        "pub use logic::*",
        "pub use styles::*",
        "pub use view::*",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "TextField mod.rs should avoid over-export marker `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ValueAxisInput",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput)",
        "pub fn resolve_props(",
    ] {
        assert!(
            logic_source.contains(needle),
            "TextField logic.rs should keep normalization/source-derivation marker `{needle}`."
        );
    }

    for forbidden in [
        "view!",
        "NodeRef<",
        "web_sys",
        "use_text_field(",
        "use_text_field_contract(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "TextField logic.rs should avoid render/headless mounting detail `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-text-field[data-state=\"invalid\"] .ui-text-field__input",
    ] {
        assert!(
            styles_source.contains(needle),
            "TextField styles.rs should keep static token-first CSS marker `{needle}`."
        );
    }

    for forbidden in ["on:input", "view!", "web_sys"] {
        assert!(
            !styles_source.contains(forbidden),
            "TextField styles.rs should avoid runtime/event/platform leakage `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "use_text_field_contract(TextFieldContractOptions {",
        "use_text_field(TextFieldOptions {",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view.rs should keep structure + headless mounting marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains("mod render;") && !view_source.contains("render.rs"),
        "TextField view.rs should not drift to `render.rs` split for this scope."
    );

    for needle in [
        "pub struct TextFieldMotion",
        "pub fn sanitize_motion(motion: TextFieldMotion) -> TextFieldMotion",
        "pub fn motion_style_vars(motion: TextFieldMotion) -> String",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "TextField motion.rs should keep motion contract marker `{needle}`."
        );
    }

    for forbidden in [
        "view!",
        "use_text_field_contract(",
        "use_text_field(",
        "ui_state_primitives::text_field",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "TextField motion.rs should avoid view/headless/state-machine concern `{forbidden}`."
        );
    }
}

#[test]
fn text_field_logic_owns_props_and_value_normalization_only() {
    let source = load_source("src/text_input/text_field/logic.rs");

    for needle in [
        "ui_state_primitives::text_field",
        "pub enum ValueControlMode",
        "pub enum ValueChangeSource",
        "impl ValueControlMode {",
        "impl ValueChangeSource {",
        "pub fn as_attr(self) -> &'static str",
        "pub struct ValueAxisInput",
        "pub struct ValueAxisState",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput)",
    ] {
        assert!(
            source.contains(needle),
            "TextField logic should include `{needle}` for central normalization."
        );
    }

    for forbidden in ["ui_headless", "use_text_field(", "use_text_field_contract("] {
        assert!(
            !source.contains(forbidden),
            "TextField logic should not implement headless contracts directly (`{forbidden}`)."
        );
    }
}

#[test]
fn text_field_api_naming_and_value_triplet_are_present_with_migration_aliases() {
    let source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "value: Option<Signal<String>>",
        "default_value: Option<String>",
        "on_value_change: Option<Callback<String>>",
        "is_disabled: Option<bool>",
        "is_read_only: Option<bool>",
        "is_required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "has_controlled_value: value.is_some()",
        "has_on_value_change: on_value_change.is_some()",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let is_required = Signal::derive(move || match is_required_input {",
        "let is_invalid = Signal::derive(move || match is_invalid_input {",
    ] {
        assert!(
            source.contains(needle),
            "TextField API/normalization should include `{needle}`."
        );
    }
}

#[test]
fn text_field_uses_headless_a11y_contract_and_locale_passthrough() {
    let source = load_source("src/text_input/text_field/view.rs");
    let headless = load_source("../ui-headless/src/text_field.rs");

    for needle in [
        "use_text_field_contract(TextFieldContractOptions {",
        "use_text_field(TextFieldOptions {",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
        "lang=move || contract.attrs.lang.clone()",
        "dir=move || contract.attrs.dir",
    ] {
        assert!(
            source.contains(needle),
            "TextField view should include headless a11y contract `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "pub struct TextFieldContractAttrs",
        "pub fn use_text_field_contract(options: TextFieldContractOptions) -> TextFieldContract",
    ] {
        assert!(
            headless.contains(needle),
            "ui-headless text_field should include `{needle}`."
        );
    }
}

#[test]
fn text_field_emits_observable_state_and_source_markers() {
    let source = load_source("src/text_input/text_field/view.rs");

    for attr in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-value-controlled=value_axis.is_controlled.then_some(\"true\")",
        "data-value-uncontrolled=(!value_axis.is_controlled).then_some(\"true\")",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-has-value-change=value_axis.has_value_change_handler.then_some(\"true\")",
        "data-label-source=label_source_attr",
        "data-description-source=description_source_attr",
        "data-error-source=error_source_attr",
        "data-placeholder-source=placeholder_source_attr",
        "data-type-source=type_source_attr",
        "data-class-source=class_source_attr",
        "data-motion-source=if has_custom_motion {",
        "data-focus-visible",
        "data-disabled",
        "data-read-only",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "TextField should expose stable semantic marker `{attr}`."
        );
    }
}

#[test]
fn text_field_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "pub enum TextFieldAgentSchemaVersion",
        "pub enum TextFieldAgentIntent",
        "pub enum TextFieldAgentActionModel",
        "pub struct TextFieldAgentContract",
        "pub fn text_field_agent_contract() -> TextFieldAgentContract",
        "schema_attr: \"ui.text-field\"",
        "schema_version_attr: TextFieldAgentSchemaVersion::V1.as_attr()",
        "intent_attr: TextFieldAgentIntent::FormTextInput.as_attr()",
        "action_model_attr: TextFieldAgentActionModel::InputFocusBlurValidate.as_attr()",
        "state_axis_attr: \"state|value|requirement|disabled|readonly|focus-visible\"",
        "source_axis_attr: \"label|description|error|placeholder|type|class|motion|value-axis\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "TextField logic should derive typed Agent Contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = logic::text_field_agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-label-source=label_source_attr",
        "data-description-source=description_source_attr",
        "data-error-source=error_source_attr",
        "data-placeholder-source=placeholder_source_attr",
        "data-type-source=type_source_attr",
        "data-class-source=class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view should mount typed Agent Contract marker `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action-model=format!(",
        "data-ui-state-axis=format!(",
        "data-ui-source-axis=format!(",
        "inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TextField agent-contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn text_field_streaming_definition_is_llm_output_only_and_component_stays_non_streaming() {
    let checklist_source = load_source("src/text_input/text_field/check2.md");
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "TextField checklist should pin streaming-definition marker `{needle}`."
        );
    }

    let combined = format!("{logic_source}\n{view_source}\n{docs_source}");
    for forbidden in [
        "AiRenderMode",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-output-status",
        "fallback=snapshot",
        "streaming",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TextField is not an LLM output renderer and should not mount streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn text_field_snapshot_baseline_consumes_complete_configuration_and_renders_stably() {
    let checklist_source = load_source("src/text_input/text_field/check2.md");
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "TextField checklist should pin snapshot-baseline marker `{needle}`."
        );
    }

    for needle in [
        "id: String,",
        "label: String,",
        "value: Option<Signal<String>>",
        "default_value: Option<String>",
        "on_value_change: Option<Callback<String>>",
        "is_disabled: Option<bool>",
        "is_read_only: Option<bool>",
        "is_required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "description: Option<String>",
        "error: Option<String>",
        "placeholder: Option<String>",
        "input_type: Option<&'static str>",
        "motion: TextFieldMotion",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "node_ref: NodeRef<html::Input>",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "logic::resolve_props(",
        "prop:value=move || value.get()",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField snapshot baseline should keep complete-config render marker `{needle}`."
        );
    }

    for needle in [
        "pub struct ValueAxisState",
        "pub struct AccessibilityState",
        "pub struct TextFieldResolvedProps",
        "source_attr_from_presence",
        "resolve_input_type",
        "resolve_label",
    ] {
        assert!(
            logic_source.contains(needle),
            "TextField snapshot baseline should keep stable normalization marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Interactive Playground (State + Source Markers)\"",
        "value=marker_value",
        "on_value_change=Callback::new(move |next| set_marker_value.set(next))",
        "is_disabled=marker_disabled.get()",
        "is_required=Signal::derive(|| true)",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
        "is_read_only=marker_read_only.get()",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Email is required\".to_string()",
        "placeholder=\"release@omne.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "TextField docs should include complete snapshot-config marker `{needle}`."
        );
    }
}

#[test]
fn text_field_streaming_policy_is_optional_with_snapshot_fallback_and_semantic_continuity() {
    let checklist_source = load_source("src/text_input/text_field/check2.md");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "TextField checklist should keep streaming-policy marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
        "title=\"Interactive Playground (State + Source Markers)\"",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "TextField optional-streaming scope should keep semantic continuity marker `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{docs_source}");
    for forbidden in [
        "AiRenderMode::Streaming",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TextField is a non-LLM renderer and should not expose streaming protocol marker `{forbidden}`."
        );
    }
}

#[test]
fn text_field_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/text_field/semantics.rs");

    assert!(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/text_field/semantics.rs")
            .exists(),
        "TextField interactive component should provide dedicated `*_semantics.rs` coverage."
    );

    for required in [
        "fn text_field_emits_observable_state_and_source_markers()",
        "fn text_field_uses_headless_a11y_contract_and_locale_passthrough()",
        "fn text_field_agent_contract_schema_is_typed_traceable_and_whitelisted()",
        "fn text_field_snapshot_baseline_consumes_complete_configuration_and_renders_stably()",
        "fn text_field_streaming_policy_is_optional_with_snapshot_fallback_and_semantic_continuity()",
    ] {
        assert!(
            semantics_source.contains(required),
            "TextField semantic suite should include contract-focused coverage `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["image", "_snapshot"].concat(),
    ];
    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "TextField semantic suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn text_field_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/text_input/text_field/view.rs");
    let semantics_source = load_source("tests/text_field/semantics.rs");

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-label-source=label_source_attr",
        "data-description-source=description_source_attr",
        "data-error-source=error_source_attr",
        "data-placeholder-source=placeholder_source_attr",
        "data-type-source=type_source_attr",
        "data-class-source=class_source_attr",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
        "on:input=move |ev| contract.handlers.on_input.run(event_target_value(&ev))",
        "on:focus=move |_| contract.handlers.focus_ring.on_focus.run(())",
        "on:blur=move |_| contract.handlers.focus_ring.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(marker),
            "TextField view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "TextField semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn text_field_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_text_field_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "await page.goto(\"/#/components/text-field\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "locator('[data-slot=\"text-field\"]')",
        ".filter({ has: page.locator(\"#docs-text-field-markers\") })",
        "const controls = page.locator('[data-slot=\"text-field-marker-controls\"]').first();",
        "[data-slot=\"text-field-toggle-invalid\"] [data-slot=\"button\"]",
        "[data-slot=\"text-field-toggle-readonly\"] [data-slot=\"button\"]",
        "[data-slot=\"text-field-toggle-disabled\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.text-field\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
        "toHaveAttribute(\"data-value-control-mode\", \"controlled\")",
        "toHaveAttribute(\"data-default-value-source\", \"default\")",
        "toHaveAttribute(\"data-value-change-source\", \"on_value_change\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "toHaveAttribute(\"aria-required\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TextField e2e selector/wait contract should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"text-field-marker-controls\"",
        "data-slot=\"text-field-toggle-invalid\"",
        "data-slot=\"text-field-toggle-readonly\"",
        "data-slot=\"text-field-toggle-disabled\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TextField docs controls should expose semantic selector anchor `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "hasText:",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "TextField e2e should use semantic-ready waits and avoid brittle selector/wait API `{forbidden}`."
        );
    }
}

#[test]
fn text_field_e2e_ready_settled_flow_covers_keyboard_pointer_and_motion_related_states() {
    let e2e_source = load_source("../../e2e/tests/docs_app_text_field_contract.spec.mjs");

    for needle in [
        "docs-app text-field covers ready-settled pointer and keyboard flow via semantic markers",
        "await input.fill(\"qa@rustui.dev\");",
        "toHaveAttribute(\"data-value\", \"filled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "await toggleInvalid.click();",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-invalid\", \"true\")",
        "toHaveAttribute(\"aria-invalid\", \"true\")",
        "await toggleInvalid.focus();",
        "await expect(toggleInvalid).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await toggleReadOnly.click();",
        "toHaveAttribute(\"data-state\", \"readonly\")",
        "toHaveAttribute(\"data-read-only\", \"true\")",
        "toHaveAttribute(\"readonly\", \"\")",
        "await toggleReadOnly.focus();",
        "await expect(toggleReadOnly).toBeFocused();",
        "await toggleDisabled.click();",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"disabled\", \"\")",
        "await toggleDisabled.focus();",
        "await expect(toggleDisabled).toBeFocused();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TextField e2e ready/settled contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "TextField e2e ready/settled flow should avoid fixed-delay waits `{forbidden}`."
        );
    }
}

#[test]
fn text_field_e2e_repeatable_key_flow_is_in_regression_collection_with_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_text_field_contract.spec.mjs");
    let checklist_source = load_source("src/text_input/text_field/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "TextField checklist should keep repeatable-regression marker `{needle}`."
        );
    }

    for needle in [
        "test(\"docs-app text-field covers ready-settled pointer and keyboard flow via semantic markers\"",
        "await page.goto(\"/#/components/text-field\");",
        "await input.fill(\"qa@rustui.dev\");",
        "await toggleInvalid.click();",
        "await page.keyboard.press(\"Enter\");",
        "await toggleReadOnly.click();",
        "await expect(toggleReadOnly).toBeFocused();",
        "await toggleDisabled.click();",
        "await expect(toggleDisabled).toBeFocused();",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-invalid\", \"true\")",
        "toHaveAttribute(\"aria-invalid\", \"true\")",
        "toHaveAttribute(\"data-state\", \"readonly\")",
        "toHaveAttribute(\"data-read-only\", \"true\")",
        "toHaveAttribute(\"readonly\", \"\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"disabled\", \"\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TextField repeatable key-flow regression should include semantic breakpoint `{needle}`."
        );
    }

    let forbidden_terms = [
        "toHaveScreenshot(".to_string(),
        ["assert", "_snapshot"].concat(),
        ["image", "_snapshot"].concat(),
        "页面不一致".to_string(),
    ];
    for forbidden in forbidden_terms {
        assert!(
            !e2e_source.contains(forbidden.as_str()),
            "TextField repeatable e2e regression should avoid non-semantic failure mode `{forbidden}`."
        );
    }
}

#[test]
fn text_field_styles_use_explicit_state_selectors_and_runtime_css_vars_only() {
    let source = load_source("src/text_input/text_field/styles.rs");

    for selector in [
        "--ui-text-field-label-font-size: var(--ui-font-size-150);",
        "--ui-text-field-meta-font-size: var(--ui-font-size-100);",
        "--ui-text-field-focus-outline-width: var(--ui-button-focus-outline-width);",
        "--ui-text-field-focus-outline-offset: var(--ui-button-focus-outline-offset);",
        "--ui-text-field-control-bg: var(--ui-bg);",
        "--ui-text-field-control-bg-hover: color-mix(in oklab, var(--ui-bg-muted) 38%, var(--ui-bg) 62%);",
        "--ui-text-field-control-border: var(--ui-border);",
        "--ui-text-field-control-border-hover: color-mix(in oklab, var(--ui-border) 62%, var(--ui-fg) 38%);",
        "--ui-text-field-control-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-border) 74%, transparent);",
        "prefers-reduced-motion: reduce",
        "--ui-text-field-motion-duration",
        "--ui-text-field-motion-easing",
        "font-size: var(--ui-text-field-label-font-size);",
        "font-size: var(--ui-text-field-meta-font-size);",
        "border: 1px solid var(--ui-text-field-control-border);",
        "background: var(--ui-text-field-control-bg);",
        "box-shadow: var(--ui-text-field-control-shadow);",
        "outline: var(--ui-text-field-focus-outline-width) solid var(--ui-focus-ring);",
        "outline-offset: var(--ui-text-field-focus-outline-offset);",
        ".ui-text-field__input:hover:not(:disabled):not([readonly])",
        "border-color: color-mix(in oklab, var(--ui-focus-ring) 32%, var(--ui-text-field-control-border) 68%);",
        ".ui-text-field[data-state=\"disabled\"] .ui-text-field__input",
        ".ui-text-field[data-state=\"invalid\"] .ui-text-field__input",
        ".ui-text-field[data-state=\"readonly\"] .ui-text-field__input",
        ".ui-text-field[data-value=\"filled\"]",
        ".ui-text-field[data-requirement=\"required\"]",
        ".ui-text-field[data-value-control-mode=\"controlled\"]",
        ".ui-text-field[data-default-value-source=\"custom\"]",
        ".ui-text-field[data-value-change-source=\"on_value_change\"]",
        ".ui-text-field[data-value-change-source=\"set_value\"]",
    ] {
        assert!(
            source.contains(selector),
            "TextField styles should include explicit selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", "div > div >"] {
        assert!(
            !source.contains(forbidden),
            "TextField styles should avoid brittle structural selector `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn text_field_css_is_aggregated_and_ui_root_injects_components_css() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-text_field\")]\n#[path = \"text_input/text_field/mod.rs\"]\npub mod text_field;"
        ),
        "ui lib should gate `text_field` module with `component-text_field` feature."
    );

    assert!(
        css_source.contains("out.push_str(crate::text_field::styles::CSS);"),
        "ui css aggregator should include text_field styles."
    );

    let text_field_css_push = css_source
        .find("out.push_str(crate::text_field::styles::CSS);")
        .expect("text_field css aggregation entry should exist");
    let text_field_css_cfg = css_source[..text_field_css_push]
        .rfind("#[cfg(feature = \"component-text_field\")]")
        .expect("text_field css aggregation should be feature-gated");
    assert!(
        text_field_css_push.saturating_sub(text_field_css_cfg) < 120,
        "text_field css aggregation should be directly guarded by `component-text_field` cfg."
    );

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should include `{needle}` for aggregated component CSS injection."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn text_field_ui_components_fixed_entry_files_follow_layer_contracts() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_source = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "#[cfg(feature = \"component-text_field\")]\n#[path = \"text_input/text_field/mod.rs\"]\npub mod text_field;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui fixed entry should include `{needle}` in lib.rs."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "mod overlay_open;",
        "pub mod presence;",
        "mod presence;",
        "pub mod a11y;",
        "mod a11y;",
        "pub mod observability;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
        "pub use wasm_bindgen::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui public entry should not leak internal/platform detail `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-active_highlight\")]\n    out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(feature = \"component-text_field\")]\n    out.push_str(crate::text_field::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry contract should include feature-gated aggregation marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "if let Some(overrides) = semantic_overrides.get_value() {",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should include centralized theme/injection/i18n marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared capability should include `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "TextField",
        "Toast",
        "Menu",
        "data-slot=\"text-field\"",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should avoid component business semantic coupling `{forbidden}`."
        );
    }

    for forbidden_path in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden_path).exists(),
            "ui should not define forbidden entry file `{forbidden_path}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String)",
    ] {
        assert!(
            headless_controllable_source.contains(needle)
                || headless_presence_source.contains(needle)
                || headless_a11y_source.contains(needle),
            "headless fixed primitive entry should provide `{needle}`."
        );
    }
}

#[test]
fn text_field_motion_module_exposes_sanitized_contract_and_attach_api() {
    let source = load_source("src/text_input/text_field/motion.rs");

    for needle in [
        "pub struct TextFieldMotion",
        "default_text_field_motion_tokens",
        "pub fn sanitize_motion(motion: TextFieldMotion) -> TextFieldMotion",
        "pub fn motion_style_vars(motion: TextFieldMotion) -> String",
        "pub fn attach_motion(",
        "ui_motion::web::animate(",
    ] {
        assert!(
            source.contains(needle),
            "TextField motion module should include `{needle}`.",
        );
    }
}

#[test]
fn text_field_cross_platform_compile_contract_has_explicit_cfg_and_no_non_wasm_web_sys_usage() {
    let view_source = load_source("src/text_input/text_field/view.rs");
    let motion_source = load_source("src/text_input/text_field/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "pub mod web;",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            motion_source.contains(needle) || ui_motion_source.contains(needle),
            "TextField cross-platform contract should include explicit cfg/no-op motion path `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "window().",
        "document().",
        "leptos::web_sys",
        "wasm_bindgen::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TextField view should avoid browser-only APIs in non-wasm paths: `{forbidden}`."
        );
    }

    let non_wasm_motion = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("text_field motion should include non-wasm cfg branch");
    for forbidden in [
        "web_sys::",
        "window().",
        "document().",
        "leptos::web_sys",
        "wasm_bindgen::",
        "unchecked_into",
    ] {
        assert!(
            !non_wasm_motion.contains(forbidden),
            "TextField non-wasm motion branch should avoid browser-only API `{forbidden}`."
        );
    }
}

#[test]
fn text_field_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "use ui_headless::A11yDirection;",
        "use ui_headless::text_field::{",
        "use_text_field_contract",
        "use_text_field",
    ] {
        assert!(
            headless_lib.contains(needle) || view_source.contains(needle),
            "text_field should preserve ui-headless web/ssr mutex contract via `{needle}`."
        );
    }
}

#[test]
fn text_field_motion_dependency_exposes_non_wasm_noop_stub_contract() {
    let motion_source = load_source("src/text_input/text_field/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "ui_motion::web::animate(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            ui_motion_source.contains(needle) || motion_source.contains(needle),
            "text_field motion dependency should keep predictable non-wasm no-op contract via `{needle}`."
        );
    }

    let non_wasm_branch = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("text_field motion should include non-wasm attach branch");
    for forbidden in ["panic!(", "unreachable!(", ".expect(", ".unwrap("] {
        assert!(
            !non_wasm_branch.contains(forbidden),
            "text_field non-wasm motion branch should avoid panic-prone assumption `{forbidden}`."
        );
    }
}

#[test]
fn text_field_reduced_motion_ssr_wasm_paths_stay_semantically_consistent() {
    let view_source = load_source("src/text_input/text_field/view.rs");
    let motion_source = load_source("src/text_input/text_field/motion.rs");
    let styles_source = load_source("src/text_input/text_field/styles.rs");

    for needle in [
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "let duration_ms = if motion.enabled {",
        "--ui-text-field-motion-duration: {}ms;",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(sanitize_motion(motion));",
        "@media (prefers-reduced-motion: reduce)",
        "transition: none;",
    ] {
        assert!(
            motion_source.contains(needle) || styles_source.contains(needle),
            "text_field should include reduced-motion/wasm-ssr branch contract `{needle}`."
        );
    }

    for needle in [
        "style=inline_style.get_value().unwrap_or_default()",
        "motion::attach_motion(root_ref, is_active, motion);",
        "lang=move || contract.attrs.lang.clone()",
        "dir=move || contract.attrs.dir",
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "text_field view should keep hydration-safe semantic contract `{needle}` across platforms."
        );
    }

    assert!(
        !view_source.contains("cfg(target_arch"),
        "text_field view should avoid wasm/ssr semantic split in rendered contract."
    );
}

#[test]
fn text_field_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "const SLOT_ROOT: &str = \"text-field\";",
        "const SLOT_LABEL: &str = \"text-field-label\";",
        "const SLOT_INPUT: &str = \"text-field-input\";",
        "const SLOT_DESCRIPTION: &str = \"text-field-description\";",
        "const SLOT_ERROR: &str = \"text-field-error\";",
        "const CLASS_LABEL: &str = \"ui-text-field__label\";",
        "const CLASS_INPUT: &str = \"ui-text-field__input\";",
        "const CLASS_DESCRIPTION: &str = \"ui-text-field__description\";",
        "const CLASS_ERROR: &str = \"ui-text-field__error\";",
        "const MOTION_SOURCE_CUSTOM: &str = \"custom\";",
        "const MOTION_SOURCE_DEFAULT: &str = \"default\";",
        "data-slot=SLOT_ROOT",
        "data-slot=SLOT_LABEL",
        "data-slot=SLOT_INPUT",
        "data-slot=SLOT_DESCRIPTION",
        "data-slot=SLOT_ERROR",
        "class=CLASS_LABEL",
        "class=CLASS_INPUT",
        "class=CLASS_DESCRIPTION",
        "class=CLASS_ERROR",
        "MOTION_SOURCE_CUSTOM",
        "MOTION_SOURCE_DEFAULT",
    ] {
        assert!(
            view_source.contains(needle),
            "text_field view should keep static fragment constants via `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("SLOT_DESCRIPTION").count(),
        2,
        "text_field description slot should have one constant source and one use."
    );
    assert_eq!(
        view_source.matches("SLOT_ERROR").count(),
        2,
        "text_field error slot should have one constant source and one use."
    );
}

#[test]
fn text_field_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "fn render_description(description: Option<String>, description_id: String) -> impl IntoView",
        "fn render_error(",
        "is_invalid: Signal<bool>,",
        ") -> impl IntoView {",
        "let description_view = render_description(description, aria.description.id.clone());",
        "let error_view = render_error(error, aria.error.id.clone(), is_invalid);",
        "{description_view}",
        "{error_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "text_field view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_description(",
        "#[component]\nfn render_error(",
        "#[component]\r\nfn render_description(",
        "#[component]\r\nfn render_error(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "text_field local fragments should stay plain functions, not local components `{forbidden}`."
        );
    }
}

#[test]
fn text_field_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let guarded_files = [
        "src/text_input/text_field/mod.rs",
        "src/text_input/text_field/logic.rs",
        "src/text_input/text_field/styles.rs",
        "src/text_input/text_field/motion.rs",
        "src/text_input/text_field/view.rs",
        "../../apps/docs-app/src/pages/components/pages/forms_text_field.rs",
    ];

    for rel_path in guarded_files {
        let source = load_source(rel_path);
        let normalized = source.to_ascii_lowercase();

        for forbidden in ["inner_html", "<script", "javascript:"] {
            assert!(
                !normalized.contains(forbidden),
                "text_field source `{rel_path}` should not contain untrusted html injection marker `{forbidden}`."
            );
        }
    }
}

#[test]
fn text_field_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let text_field_mod_source = load_source("src/text_input/text_field/mod.rs");
    let text_field_logic_source = load_source("src/text_input/text_field/logic.rs");
    let text_field_motion_source = load_source("src/text_input/text_field/motion.rs");
    let text_field_styles_source = load_source("src/text_input/text_field/styles.rs");
    let text_field_view_source = load_source("src/text_input/text_field/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root_source.contains(needle),
            "ui should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("text_field-wasm-debug")
            && !cargo_source.contains("textfield-wasm-debug"),
        "TextField should not expose a dedicated wasm-debug feature; it should reuse global ui-trace overlay."
    );

    let text_field_combined = format!(
        "{text_field_mod_source}\n{text_field_logic_source}\n{text_field_motion_source}\n{text_field_styles_source}\n{text_field_view_source}"
    );
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !text_field_combined.contains(forbidden),
            "TextField production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for marker in [
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
    ] {
        assert!(
            text_field_view_source.contains(marker),
            "TextField should expose stable source markers for debug attribution via `{marker}`."
        );
    }

    for needle in [
        "on:input=move |ev| contract.handlers.on_input.run(event_target_value(&ev))",
        "on:focus=move |_| contract.handlers.focus_ring.on_focus.run(())",
        "on:blur=move |_| contract.handlers.focus_ring.on_blur.run(())",
    ] {
        assert!(
            text_field_view_source.contains(needle),
            "TextField interaction chain should stay reproducible via `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep wasm dev visual-entry gate `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "ui_headless::UiTraceEventKind::Inspect",
        "ui_headless::UiTraceEventKind::Note { message }",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep timeline rendering marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Note {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped event marker `{needle}`."
        );
    }
}

#[test]
fn text_field_dx_contract_uses_docs_playground_for_hot_css_iteration_and_context_preservation() {
    let docs_text_field_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");
    let docs_playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (marker_value, set_marker_value) = signal(\"release@omne.rs\".to_string());",
        "let (marker_invalid, set_marker_invalid) = signal(false);",
        "let (marker_read_only, set_marker_read_only) = signal(false);",
        "let (marker_disabled, set_marker_disabled) = signal(false);",
        "<Playground title=\"Label + placeholder\" code_signal=code>",
        "title=\"Interactive Playground (State + Source Markers)\"",
    ] {
        assert!(
            docs_text_field_source.contains(needle),
            "TextField docs should keep context-preserving playground state marker `{needle}`."
        );
    }

    for needle in [
        "fn compose_scoped_css(scope_selector: &str, raw: &str) -> String",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Restore original CSS\"",
        "<UiPerfProbe name=format!(\"Playground::{title}\")>",
        "data-playground-scope=scope_id.clone()",
    ] {
        assert!(
            docs_playground_source.contains(needle),
            "docs Playground should keep DX hot-style/workbench contract marker `{needle}`."
        );
    }
}

#[test]
fn text_field_engineering_contract_is_spec_free_tracing_aligned_and_runtime_agnostic() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let text_field_mod_source = load_source("src/text_input/text_field/mod.rs");
    let text_field_logic_source = load_source("src/text_input/text_field/logic.rs");
    let text_field_motion_source = load_source("src/text_input/text_field/motion.rs");
    let text_field_view_source = load_source("src/text_input/text_field/view.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");

    assert!(
        cargo_source.contains("component-text_field = []"),
        "TextField feature should stay lightweight and avoid implicit engineering dependency fan-out."
    );

    for forbidden in [
        "component-text_field = [\"dep:serde\"",
        "component-text_field = [\"dep:serde_json\"",
        "component-text_field = [\"dep:tracing\"",
        "component-text_field = [\"dep:tokio\"",
        "component-text_field = [\"dep:async-std\"",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "TextField feature should not pin serde/tracing/runtime deps directly: `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir
            .join("src/text_input/text_field/spec.rs")
            .exists(),
        "TextField simple component scope should keep spec/config serde migration path as N/A without local spec.rs."
    );

    let text_field_combined = format!(
        "{text_field_mod_source}\n{text_field_logic_source}\n{text_field_motion_source}\n{text_field_view_source}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "tokio::",
        "async_std::",
        "async-std",
    ] {
        assert!(
            !text_field_combined.contains(forbidden),
            "TextField engineering path should avoid local serde/tracing/runtime coupling `{forbidden}`."
        );
    }

    for needle in [
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "TextField tracing semantics should remain aligned with shared ui-headless trace contract `{needle}`."
        );
    }
}

#[test]
fn text_field_docs_page_covers_dx_and_state_source_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "pub(super) fn text_field() -> AnyView",
        "title=\"TextField\"",
        "slug=\"text-field\"",
        "description=\"A compact field wrapper built on headless text field semantics with explicit state/source marker contracts.\"",
        "<Playground title=\"Label + placeholder\" code_signal=code>",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "data-slot=\"text-field-marker-controls\"",
        "<TextField",
    ] {
        assert!(
            source.contains(needle),
            "forms_text_field docs should include `{needle}`.",
        );
    }
}

#[test]
fn text_field_docs_playgrounds_lock_prefixed_api_and_source_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");

    for needle in [
        "title=\"Label + placeholder\"",
        "id=\"docs-text-field\".to_string()",
        "label=\"Name\".to_string()",
        "placeholder=\"Jane\".to_string()",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "id=\"docs-text-field-markers\".to_string()",
        "label=\"Email\".to_string()",
        "value=marker_value",
        "on_value_change=Callback::new(move |next| set_marker_value.set(next))",
        "is_disabled=marker_disabled.get()",
        "is_required=Signal::derive(|| true)",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
        "is_read_only=marker_read_only.get()",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Email is required\".to_string()",
        "placeholder=\"release@omne.rs\".to_string()",
        "input_type=\"email\"",
        "class_name=\"docs-text-field-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "forms_text_field docs should contain `{needle}`.",
        );
    }

    assert!(
        !source.contains(" set_value="),
        "forms_text_field docs should prefer `on_value_change` over legacy `set_value=` examples."
    );
}

#[test]
fn text_field_docs_page_syncs_api_matrix_state_matrix_and_source_first_contracts() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"text-field-api-matrix\"",
        "<h3>\"API Matrix\"</h3>",
        "data-slot=\"text-field-api-rows\"",
        "ui::text_input::text_field::DEFAULT_LABEL",
        "value + on_value_change + default_value",
        "data-slot=\"text-field-state-matrix\"",
        "<h3>\"State Matrix\"</h3>",
        "data-slot=\"text-field-state-rows\"",
        "data-value-control-mode",
        "data-state",
        "data-slot=\"text-field-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "text=\"use leptos::prelude::*;\\nuse ui::*;",
        "id=\\\"email\\\".into()",
        "label=\\\"Email\\\".into()",
        "data-slot=\"text-field-source-paths\"",
        "components/text-input/src/text_field/view.rs",
        "data-slot=\"text-field-source-prerequisites\"",
        "component-text_field",
        "inject-css",
    ] {
        assert!(
            docs_source.contains(needle),
            "forms_text_field docs should include matrix/source-first marker `{needle}`."
        );
    }

    assert!(
        docs_source.contains("compose_copy_ready_code"),
        "TextField docs should state the copy-ready code path explicitly."
    );
    assert!(
        playground_source
            .contains("fn compose_copy_ready_code(raw: &str, imports: &str) -> String"),
        "docs playground should keep copy-ready import composition contract."
    );
}

#[test]
fn text_field_docs_entry_is_indexed_and_beginner_friendly() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_text_field.rs");
    let catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "component_doc!(",
        "\"TextField\"",
        "\"text-field\"",
        "forms_text_field::text_field",
        "<Playground title=\"Label + placeholder\" code_signal=code>",
        "<TextField id=\\\"name\\\".into()",
    ] {
        assert!(
            docs_source.contains(needle) || catalog_source.contains(needle),
            "TextField docs/index should include `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::text_field",
        "use_text_field_contract(",
        "use_controllable_state(",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "TextField docs beginner path should not require internal wiring `{forbidden}`."
        );
    }
}

#[test]
fn text_field_heroui_alignment_doc_records_text_field_sync() {
    let source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "### TextField 同步记录",
        "docs-app/src/pages/components/pages/forms_text_field.rs",
        "#/components/text-field",
        "API Matrix + State Matrix",
        "Source-first / Copy-Paste Ready",
        "value + on_value_change + default_value",
    ] {
        assert!(
            source.contains(needle),
            "HeroUI strategy doc should record TextField sync marker `{needle}`."
        );
    }
}

#[test]
fn text_field_heroui_sync_guard_keeps_catalog_and_parameter_contract_in_step() {
    let view_source = load_source("src/text_input/text_field/view.rs");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let checklist_source = load_source("src/text_input/text_field/check2.md");

    for needle in [
        "value + on_value_change + default_value",
        "is_disabled/is_read_only/is_required/is_invalid",
        "component_doc!(\"TextField\", \"text-field\", \"Forms\", forms_text_field::text_field)",
        "### TextField 同步记录（2026-02-18）",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_catalog_source.contains(needle),
            "TextField HeroUI/doc sync guard should include `{needle}`."
        );
    }

    for needle in [
        "on_value_change: Option<Callback<String>>",
        "default_value: Option<String>",
        "is_disabled: Option<bool>",
        "is_read_only: Option<bool>",
        "is_required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view API surface should include `{needle}` for doc-sync validation."
        );
    }

    assert!(
        checklist_source.contains(
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`"
        ),
        "TextField checklist should mark HeroUI/doc sync gate as completed with explicit evidence."
    );
}

#[test]
fn text_field_rejects_cross_layer_anti_patterns() {
    let primitives_source = load_source("../ui-state-primitives/src/text_field.rs");
    let headless_source = load_source("../ui-headless/src/text_field.rs");
    let logic_source = load_source("src/text_input/text_field/logic.rs");
    let view_source = load_source("src/text_input/text_field/view.rs");
    let mod_source = load_source("src/text_input/text_field/mod.rs");

    for forbidden in ["view!", "data-slot=", "web_sys", "leptos::"] {
        assert!(
            !primitives_source.contains(forbidden),
            "ui-state-primitives text_field should stay pure POJO logic without `{forbidden}`."
        );
    }

    for forbidden in [
        "ui-text-field__",
        "var(--ui-",
        ".ui-text-field",
        "TextFieldMotion",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless text_field should avoid visual/motion coupling `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "document.", "window.", "HtmlElement"] {
        assert!(
            !logic_source.contains(forbidden),
            "TextField logic should avoid platform specifics `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "use_text_field_contract(TextFieldContractOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view should consume normalized/headless outputs via `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<web_sys::", "HtmlInputElement"] {
        assert!(
            !mod_source.contains(forbidden),
            "TextField public module boundary should not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn text_field_architecture_foundation_layers_are_checked_individually() {
    let primitives_source = load_source("../ui-state-primitives/src/text_field.rs");
    let headless_source = load_source("../ui-headless/src/text_field.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let component_logic_source = load_source("src/text_input/text_field/logic.rs");
    let component_view_source = load_source("src/text_input/text_field/view.rs");
    let component_styles_source = load_source("src/text_input/text_field/styles.rs");
    let checklist_source = load_source("src/text_input/text_field/check2.md");

    for forbidden in ["web_sys", "view!", "data-slot=", "class:ui-text-field"] {
        assert!(
            !primitives_source.contains(forbidden),
            "status-primitives text_field must stay pure; found `{forbidden}`."
        );
    }

    for forbidden in [
        "ui-text-field__",
        ".ui-text-field",
        "TextFieldMotion",
        "var(--ui-",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless text_field must stay interaction/A11y-only; found `{forbidden}`."
        );
    }

    for needle in [
        "pub mod keyframes;",
        "pub mod options;",
        "pub mod spring;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should expose generic engine/backend capability token `{needle}`."
        );
    }

    for forbidden in ["data-slot", "view!", "on:input", "on:blur"] {
        assert!(
            !component_logic_source.contains(forbidden),
            "text_field logic.rs should stay assembly-only and avoid `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "use_text_field_contract(TextFieldContractOptions {",
        "motion::attach_motion(root_ref, is_active, motion);",
    ] {
        assert!(
            component_view_source.contains(needle),
            "text_field view.rs should compose layers via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "data-state=\"invalid\"",
    ] {
        assert!(
            component_styles_source.contains(needle),
            "text_field styles.rs should consume theme tokens/state markers via `{needle}`."
        );
    }

    for needle in [
        "- [x] `status-primitives` 定义：",
        "- [x] `ui-headless` 定义：",
        "- [x] `ui-motion` 定义：",
        "- [x] `ui-theme` 定义：",
        "- [x] `ui` 定义：",
    ] {
        assert!(
            checklist_source.contains(needle),
            "check2 should mark architecture foundation item `{needle}` as completed."
        );
    }
}

#[test]
fn text_field_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/text_input/text_field/check2.md");
    let view_source = load_source("src/text_input/text_field/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget baseline token `{needle}`."
        );
    }

    for needle in [
        "\"TextField\",",
        "\"text-field\",",
        "forms_text_field::text_field",
    ] {
        assert!(
            pages_source.contains(needle),
            "TextField docs page should stay in docs coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose perf contract marker `{needle}`."
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
            "coverage e2e should keep perf observability assertion `{needle}`."
        );
    }

    for needle in ["render_count", "替换当前 mount-only 等价证据"] {
        assert!(
            todo_source.contains(needle),
            "perf plan should keep render_count follow-up token `{needle}`."
        );
    }

    for needle in [
        "data-state=move || contract.state.resolved.get().state_attr",
        "data-value=move || contract.state.resolved.get().value_attr",
        "data-requirement=move || contract.state.resolved.get().requirement_attr",
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-class-source=class_source_attr",
        "data-motion-source=if has_custom_motion {",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view should expose triage marker `{needle}`."
        );
    }

    for needle in ["- [x] 性能治理：关键路径有预算", "render_count", "等价证据"] {
        assert!(
            check2_source.contains(needle),
            "check2 performance governance evidence should include `{needle}`."
        );
    }
}

#[test]
fn text_field_view_macro_complexity_is_bounded_with_semantic_subblocks() {
    let view_source = load_source("src/text_input/text_field/view.rs");
    let check2_source = load_source("src/text_input/text_field/check2.md");

    assert!(
        view_source.contains("view! {"),
        "TextField should keep explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "TextField should keep one primary view block and two local semantic subblocks."
    );
    assert!(
        view_source.lines().count() <= 280,
        "TextField view.rs should stay bounded; split semantic subrenders if it grows."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        ".fold(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TextField view should avoid expansion-heavy macro token `{forbidden}`."
        );
    }

    for needle in [
        "fn render_description(description: Option<String>, description_id: String) -> impl IntoView",
        "fn render_error(",
        "{description_view}",
        "{error_view}",
        "<Show when=move || is_invalid.get()>",
    ] {
        assert!(
            view_source.contains(needle),
            "TextField view should keep semantic subblock token `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `view!` 宏复杂度受控："),
        "check2 should mark view-macro complexity governance as completed."
    );
}

#[test]
fn text_field_gate_completion_records_full_responsible_chain_commands() {
    let check2_source = load_source("src/text_input/text_field/check2.md");

    assert!(
        check2_source.contains("- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。"),
        "check2 should mark gate completion as checked."
    );

    for needle in [
        "$HOME/.cargo/bin/rustfmt --edition 2024 --check",
        "$HOME/.cargo/bin/cargo clippy -p ui --no-default-features --features component-text_field,inject-css --lib --test text_field_semantics -- -D warnings",
        "$HOME/.cargo/bin/cargo test -p ui --test text_field_semantics --no-default-features --features component-text_field,inject-css",
        "$HOME/.cargo/bin/cargo check -p ui --no-default-features --features component-text_field,inject-css",
        "$HOME/.cargo/bin/cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-text_field,inject-css",
        "TMPDIR=/root/autodl-tmp/zjj/p/rust-ui/.codex-tmp CARGO_TARGET_DIR=target-codex-textfield-smoke bash ./scripts/smoke-csr.sh apps/docs-app \"body:not(:has(#boot))\"",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 gate evidence should include executed command `{needle}`."
        );
    }
}
