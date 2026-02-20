use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn well_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/well/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Well internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn well_directory_standard_files_and_module_exports_follow_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let well_dir = manifest_dir.join("src/well");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "i18n.rs"] {
        let path = well_dir.join(required);
        assert!(path.exists(), "Well directory should include `{required}`.");
    }

    let mod_source = load_source("src/well/mod.rs");
    for needle in [
        "mod i18n;",
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use i18n::WellStrings;",
        "pub use logic::{WellDensity, WellTone};",
        "pub use view::Well;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Well module entry should keep `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod motion;",
        "pub mod motion;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Well module entry should keep minimal export boundary; forbid `{forbidden}`."
        );
    }
}

#[test]
fn well_check2_documents_component_directory_standard_file_rules() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 组件目录标准文件落点正确。",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn well_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn well_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn well_agent_contract_markers_are_schema_like_and_machine_readable() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for marker in [
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=tone_source_attr",
        "data-density=move || state.get().density_attr",
        "data-density-source=density_source_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Well should expose agent-readable machine marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum WellTone",
        "pub enum WellDensity",
        "pub struct WellStateInput",
        "pub struct WellState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn source_attr_from_presence(is_present: bool) -> &'static str",
    ] {
        assert!(
            combined.contains(typed_source),
            "Well Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }
}

#[test]
fn well_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    // Well is non-interactive/simple, so schema-like markers are state/source-only.
    // `data-ui-schema` is optional and should not be faked by ad-hoc free-form strings.
    for forbidden in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "intent=\"",
        "action=\"",
        "format!(\"data-",
        "format!(\"{",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Well should avoid free-form/fake schema field token `{forbidden}` in non-interactive scope."
        );
    }

    for forbidden_interaction in ["on:click", "on:keydown", "on:pointerdown", "on:pointerup"] {
        assert!(
            !view_source.contains(forbidden_interaction),
            "Well has no interactive intent/action axis; token `{forbidden_interaction}` should remain absent."
        );
    }
}

#[test]
fn well_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let mod_source = load_source("src/well/mod.rs");
    let i18n_source = load_source("src/well/i18n.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{i18n_source}");

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
            "Well Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn well_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn well_streaming_semantics_are_not_required_for_snapshot_container_scope() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let mod_source = load_source("src/well/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "fallback=snapshot",
        "data-stream",
        "data-output-status",
        "data-draft",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Well is a snapshot-compatible container and should not mount streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn well_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn well_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for marker in [
        "children: Children",
        "{children()}",
        "#[prop(optional)] tone: Option<WellTone>",
        "#[prop(optional)] density: Option<WellDensity>",
        "#[prop(optional)] is_inset: Option<bool>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "logic::normalize_props(logic::WellNormalizeInput {",
        "let region = region_attrs(normalized.aria_label, lang, dir);",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
    ] {
        assert!(
            view_source.contains(marker),
            "Well snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub struct WellNormalizeInput",
        "pub struct WellNormalizedProps",
        "pub fn normalize_props(input: WellNormalizeInput) -> WellNormalizedProps",
        "pub fn resolve_state(input: WellStateInput) -> WellState",
    ] {
        assert!(
            logic_source.contains(marker) || primitives_source.contains(marker),
            "Well snapshot baseline should keep stable normalization/state marker `{marker}`."
        );
    }

    for marker in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Well>",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
        "aria_label=\"Selection summary\".to_string()",
        "class_name=\"docs-well-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "Well docs should include complete snapshot result marker `{marker}`."
        );
    }
}

#[test]
fn well_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn well_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/well/view.rs");

    for required in [
        "role=role_attr",
        "aria-label=aria_label",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-tone-source=tone_source_attr",
        "data-density-source=density_source_attr",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Well should keep continuous role/aria/data semantics via `{required}` in snapshot-only optional-streaming scope."
        );
    }

    for forbidden in [
        "data-ui-output-status",
        "data-output-status",
        "data-stream-status",
        "data-status=\"draft\"",
        "data-status=\"verified\"",
        "data-status=\"committed\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well should not mount fake streaming status field `{forbidden}` when stream protocol is N/A."
        );
    }
}

#[test]
fn well_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry",
        "reconnect",
        "backoff",
        "resume",
        "revalidate",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Well should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn well_component_directory_has_standard_file_layout() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let well_dir = manifest_dir.join("src/well");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "i18n.rs"] {
        assert!(
            well_dir.join(required).exists(),
            "Well component directory should include `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs", "motion.rs"] {
        assert!(
            !well_dir.join(forbidden).exists(),
            "Well should not include `{forbidden}` in current non-interactive/simple scope."
        );
    }
}

#[test]
fn well_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/well/mod.rs");

    for needle in [
        "mod i18n;",
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use i18n::WellStrings;",
        "pub use logic::{WellDensity, WellTone};",
        "pub use view::Well;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Well mod.rs should keep stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod i18n;",
        "pub use logic::*;",
        "pub use view::*;",
        "mod render;",
        "pub mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Well mod.rs should not over-export/introduce drift token `{forbidden}`."
        );
    }
}

#[test]
fn well_component_file_responsibilities_remain_scoped() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");

    assert!(
        !manifest_dir.join("src/well/render.rs").exists(),
        "Well should keep render implementation in `view.rs` without `render.rs` drift."
    );
    assert!(
        !manifest_dir.join("src/well/spec.rs").exists(),
        "Well should not add `spec.rs` for simple component scope."
    );
    assert!(
        !manifest_dir.join("src/well/motion.rs").exists(),
        "Well should not add `motion.rs` when no interactive motion contract is required."
    );

    for forbidden in [
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
        "role=",
        "aria-",
        "var(--ui-",
        ".ui-well",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Well logic.rs should stay normalize/derive-only and avoid `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "Well styles.rs should keep static token-first css marker `{needle}`."
        );
    }
    for forbidden in [
        "Signal::derive(",
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Well styles.rs should avoid runtime/view logic token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Well(",
        "region_attrs(normalized.aria_label, lang, dir)",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Well view.rs should keep render + headless semantic mount marker `{required}`."
        );
    }
    for forbidden in ["pub const CSS", "mod logic;", "mod styles;", "unwrap_or("] {
        assert!(
            !view_source.contains(forbidden),
            "Well view.rs should not own styles/module/de-facto logic fallback token `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn well_spec_boundary_reuses_button_spec_without_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_spec_path = manifest_dir.join("../ui-components/src/button/spec.rs");
    let button_mod_source = load_source("../ui-components/src/button/mod.rs");
    let well_mod_source = load_source("src/well/mod.rs");

    assert!(
        button_spec_path.exists(),
        "button should keep canonical spec.rs boundary for complex schema contracts."
    );
    assert!(
        !manifest_dir.join("src/well/spec.rs").exists(),
        "Well should not introduce local spec.rs for a simple component contract."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec export `{needle}`."
        );
    }

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "WellSpec",
        "WellSchema",
    ] {
        assert!(
            !well_mod_source.contains(forbidden),
            "Well module should avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn well_check2_documents_spec_policy_for_simple_component_scope() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。",
        "简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。",
        "新增 `spec.rs` 必须同步给出契约测试与版本演进说明。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep spec governance guidance `{required}`."
        );
    }
}

#[test]
fn well_logic_styles_and_view_responsibilities_remain_separated() {
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");

    for forbidden in [
        "view! {",
        "#[component]",
        "data-slot=",
        "role=",
        "aria-",
        "on:click",
        "on:keydown",
        ".ui-well",
        "color-mix(",
        "var(--ui-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Well logic.rs should stay in normalize/derive/source-marker scope; found forbidden `{forbidden}`."
        );
    }

    for forbidden in [
        "#[component]",
        "pub fn Well(",
        "Signal::derive(",
        "StoredValue::new(",
        "normalize_props(",
        "resolve_state(",
        "if state.get()",
        "on:click",
        "on:keydown",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Well styles.rs should be static token-first CSS only; found forbidden `{forbidden}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        ".ui-well",
        "color-mix(",
        "var(--ui-",
        "pub fn normalize_tone(",
        "pub fn normalize_density(",
        "pub fn normalize_props(",
        "pub fn source_attr_from_presence(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well view.rs should focus on render + headless mount without owning logic/styles implementation `{forbidden}`."
        );
    }
}

#[test]
fn well_consumes_state_primitives_and_keeps_component_assembly_local() {
    let mod_source = load_source("src/well/mod.rs");
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");

    for needle in ["mod i18n;", "pub use i18n::WellStrings;"] {
        assert!(
            mod_source.contains(needle),
            "Well module should expose i18n entrypoint `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::well::{",
        "WellDensity",
        "WellNormalizeInput",
        "WellNormalizedProps",
        "WellState",
        "WellStateInput",
        "WellTone",
        "pub fn normalize_tone(value: Option<WellTone>) -> WellTone",
        "pub fn normalize_density(value: Option<WellDensity>) -> WellDensity",
        "pub fn normalize_is_inset(value: Option<bool>) -> bool",
        "pub fn normalize_aria_label_with_fallback(",
        "pub fn source_attr_from_presence(is_present: bool) -> &'static str",
        "fallback_aria_label: &str",
        "pub fn normalize_props(input: WellNormalizeInput) -> WellNormalizedProps",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "tone_source_attr",
        "density_source_attr",
        "inset_source_attr",
        "resolve_state,",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Well logic should include `{needle}` to consume ui-state-primitives and keep only assembly logic."
        );
    }

    for needle in [
        "pub enum WellTone",
        "pub enum WellDensity",
        "pub struct WellStateInput",
        "pub struct WellState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(input: WellStateInput) -> WellState",
    ] {
        assert!(
            primitives_source.contains(needle),
            "well primitive module should define `{needle}`."
        );
    }

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<WellStrings>();",
        "logic::normalize_props(logic::WellNormalizeInput {",
        "fallback_aria_label: strings.aria_label.as_ref().into(),",
        "let region = region_attrs(normalized.aria_label, lang, dir);",
        "let role_attr = region.role;",
        "let aria_label = region.aria_label;",
        "let locale_lang = region.lang;",
        "let locale_dir = region.dir;",
        "let state_input = StoredValue::new(normalized.state_input);",
        "logic::resolve_state(state_input.get_value())",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Well view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn well_logic_and_primitives_remain_framework_store_agnostic() {
    let logic_source = load_source("src/well/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");

    for forbidden in [
        "leptos::",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "web_sys",
        "wasm_bindgen",
        "GlobalStore",
        "AppStore",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Well logic should not bind framework/store type `{forbidden}`."
        );
        assert!(
            !primitives_source.contains(forbidden),
            "Well primitive should not bind framework/store type `{forbidden}`."
        );
    }
}

#[test]
fn well_logic_does_not_redefine_state_primitives() {
    let logic_source = load_source("src/well/logic.rs");

    for forbidden in [
        "pub enum WellTone {",
        "pub enum WellDensity {",
        "pub struct WellStateInput {",
        "pub struct WellState {",
        "pub fn resolve_state(input: WellStateInput) -> WellState",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Well logic should not redefine primitive contract `{forbidden}`."
        );
    }
}

#[test]
fn well_has_no_async_interaction_contract_and_is_explicitly_na() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "error",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well has no async interaction axis; N/A because component has no remote request or async state. Found `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Well has no async interaction axis; N/A because component has no remote request or async state. Found `{forbidden}` in logic.rs."
        );
    }
}

#[test]
fn well_public_bool_prop_uses_is_prefix() {
    let source = load_source("src/well/view.rs");

    assert!(
        source.contains("#[prop(optional)] is_inset: Option<bool>"),
        "Well public bool prop should follow `is_*` naming."
    );
    assert!(
        !source.contains("#[prop(optional)] inset: Option<bool>"),
        "Well should not expose legacy non-prefixed bool prop name `inset`."
    );

    for locale_prop in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            source.contains(locale_prop),
            "Well should expose locale passthrough prop `{locale_prop}`."
        );
    }
}

#[test]
fn well_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy() {
    let view_source = load_source("src/well/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let well_i18n_source = load_source("src/well/i18n.rs");

    for required in [
        "use ui_headless::i18n;",
        "use ui_headless::{A11yDirection, region_attrs};",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<WellStrings>();",
        "let region = region_attrs(normalized.aria_label, lang, dir);",
        "role=role_attr",
        "aria-label=aria_label",
        "lang=locale_lang.clone()",
        "dir=locale_dir",
    ] {
        assert!(
            view_source.contains(required),
            "Well should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in [
        "pub fn region_attrs(",
        "pub fn locale_attrs(",
        "pub struct RegionA11yAttrs",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "Well shared a11y utilities should come from ui-headless via `{required}`."
        );
    }

    for required in [
        "pub struct WellStrings",
        "pub aria_label: Arc<str>",
        "impl Default for WellStrings",
    ] {
        assert!(
            well_i18n_source.contains(required),
            "Well i18n entrypoint should expose `{required}`."
        );
    }

    for forbidden in [
        "\"Content well\"",
        "role=\"region\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well view should not hardcode user-facing copy/locale/a11y literal `{forbidden}`."
        );
    }
}

#[test]
fn well_public_api_does_not_require_internal_state_objects() {
    let source = load_source("src/well/view.rs");

    for forbidden in [
        "state: WellState",
        "state: WellStateInput",
        "state: Signal<WellState>",
        "state_input: WellStateInput",
    ] {
        assert!(
            !source.contains(forbidden),
            "Well API should not expose internal state object in public props: `{forbidden}`."
        );
    }
}

#[test]
fn well_is_not_a_composite_item_api_and_remains_explicit_children_composition() {
    let view_source = load_source("src/well/view.rs");

    assert!(
        view_source.contains("children: Children"),
        "Well should remain explicit children composition API (`<Well>...</Well>`)."
    );

    for forbidden in [
        "items: Vec<",
        "item_specs:",
        "labels:",
        "titles:",
        "panels:",
        "children_by_index",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well is not a composite item container; forbid parallel-slot API fragment `{forbidden}`."
        );
    }
}

#[test]
fn well_discrete_state_axes_are_type_constrained() {
    let view_source = load_source("src/well/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");

    for required in [
        "#[prop(optional)] tone: Option<WellTone>",
        "#[prop(optional)] density: Option<WellDensity>",
        "pub enum WellTone",
        "pub enum WellDensity",
    ] {
        assert!(
            view_source.contains(required) || primitives_source.contains(required),
            "Well discrete state should be enum-constrained; missing `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "density: Option<String>",
        "tone: Option<bool>",
        "density: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well should not model discrete state axis with `{forbidden}`."
        );
    }

    assert_eq!(
        view_source.matches("Option<bool>").count(),
        1,
        "Well should avoid bool explosion in public props; keep only independent boolean axis `is_inset`."
    );
}

#[test]
fn well_has_no_controlled_uncontrolled_state_axis() {
    let source = load_source("src/well/view.rs");

    for required in [
        "#[prop(optional)] tone: Option<WellTone>",
        "#[prop(optional)] density: Option<WellDensity>",
        "#[prop(optional)] is_inset: Option<bool>",
        "logic::normalize_props(logic::WellNormalizeInput {",
        "let state_input = StoredValue::new(normalized.state_input);",
    ] {
        assert!(
            source.contains(required),
            "Well should keep explicit static input prop `{required}`."
        );
    }

    for forbidden in [
        "default_inset",
        "on_inset_change",
        "default_tone",
        "on_tone_change",
        "default_density",
        "on_density_change",
    ] {
        assert!(
            !source.contains(forbidden),
            "Well should not introduce half-controlled API fragment `{forbidden}` without full controlled/uncontrolled axis design."
        );
    }
}

#[test]
fn well_has_no_disabled_axis_and_marks_it_explicitly_na() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");

    for forbidden in [
        "is_disabled",
        "disabled:",
        "disabled=",
        "default_disabled",
        "on_disabled_change",
        "aria-disabled",
        "data-disabled",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well disabled branch is N/A (component has no disabled state axis); found `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Well disabled branch is N/A (component has no disabled state axis); found `{forbidden}` in logic.rs."
        );
    }
}

#[test]
fn well_has_no_keyboard_or_pointer_interaction_branches_and_marks_them_na() {
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:click",
        "on:mousedown",
        "on:mouseup",
        "on:pointerdown",
        "on:pointerup",
        "on:pointerenter",
        "on:pointerleave",
        "on_press",
        "use_press",
        "use_hover",
        "use_focus_visible",
        "tabindex",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well keyboard/pointer branch is N/A (component exposes no interactive input contract); found `{forbidden}` in view.rs."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Well keyboard/pointer branch is N/A (component exposes no interactive input contract); found `{forbidden}` in logic.rs."
        );
    }
}

#[test]
fn well_does_not_define_component_motion_runtime_when_no_motion_contract_needed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let mod_source = load_source("src/well/mod.rs");

    assert!(
        !manifest_dir.join("src/well/motion.rs").exists(),
        "Well should not define `src/well/motion.rs` when no reusable runtime motion contract is needed."
    );

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "attach_motion(",
        "request_animation_frame",
        "cancel_animation_frame",
        "SpringAnimator::new",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Well mod.rs should stay motion-module free for current N/A motion scope; found `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Well component layer should stay motion-runtime free when motion contract is N/A; found `{forbidden}`."
        );
    }

    for forbidden_css in ["transition:", "animation:"] {
        assert!(
            !styles_source.contains(forbidden_css),
            "Well styles should stay static without runtime motion declaration `{forbidden_css}`."
        );
    }
}

#[test]
fn well_ssr_and_wasm_contract_has_no_component_local_platform_split() {
    let mod_source = load_source("src/well/mod.rs");
    let view_source = load_source("src/well/view.rs");

    for forbidden in [
        "#[cfg(",
        "cfg(",
        "target_arch = \"wasm32\"",
        "web_sys",
        "wasm_bindgen",
        "window()",
        "document()",
        "js_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Well should keep SSR/wasm behavior unified without local platform split; found `{forbidden}` in mod.rs."
        );
        assert!(
            !view_source.contains(forbidden),
            "Well should keep SSR/wasm behavior unified without local platform split; found `{forbidden}` in view.rs."
        );
    }
}

#[test]
fn well_platform_guards_keep_non_wasm_files_web_sys_free() {
    let mod_source = load_source("src/well/mod.rs");
    let i18n_source = load_source("src/well/i18n.rs");
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");

    let forbidden = "web_sys";
    assert!(
        !mod_source.contains(forbidden)
            && !i18n_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden),
        "non-wasm Well files should stay browser-object free; found `{forbidden}` outside optional wasm-only modules."
    );
}

#[test]
fn well_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui-layout",
        "echo \"[platform] compile-only: well native path\"",
        "cargo check -p ui-layout --no-default-features --features component-well,inject-css",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: well wasm path\"",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-well,inject-css",
        "echo \"[platform] source guard: non-wasm well files must not reference web_sys\"",
        "crates/ui-layout/src/well/view.rs",
        "if rg -n \"web_sys\" \"$file\" >/dev/null; then",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn well_ui_headless_feature_mutex_contract_is_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless feature mutex should be guarded in lib.rs by `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex and dual compile paths via `{needle}`."
        );
    }
}

#[test]
fn well_ui_motion_non_wasm_noop_stub_contract_is_guarded() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let motion_non_wasm_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");
    let well_mod_source = load_source("src/well/mod.rs");
    let well_view_source = load_source("src/well/view.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm no-op/stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            motion_non_wasm_test_source.contains(needle),
            "ui-motion non-wasm stub regression test should include `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should keep ui-motion non-wasm compile/tooling guards via `{needle}`."
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "attach_motion(",
    ] {
        assert!(
            !well_mod_source.contains(forbidden) && !well_view_source.contains(forbidden),
            "Well should not assume motion handle/runtime exists when motion is N/A; found `{forbidden}`."
        );
    }
}

#[test]
fn well_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");
    let mod_source = load_source("src/well/mod.rs");
    let view_source = load_source("src/well/view.rs");
    let styles_source = load_source("src/well/styles.rs");

    // Well has no component-local motion runtime; reduced-motion is handled by shared ui-motion no-op/stub.
    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "attach_motion(",
        "prefers_reduced_motion(",
        "request_animation_frame",
        "cancel_animation_frame",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Well should keep reduced-motion branch as explicit N/A without local motion runtime `{forbidden}`."
        );
    }
    for forbidden_css in ["transition:", "animation:"] {
        assert!(
            !styles_source.contains(forbidden_css),
            "Well should not rely on component-local runtime motion CSS `{forbidden_css}`."
        );
    }

    // SSR/wasm semantic contract must stay unified; no platform cfg split in view/mod.
    for forbidden in [
        "#[cfg(",
        "target_arch = \"wasm32\"",
        "web_sys",
        "wasm_bindgen",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Well semantic contract should not split by platform token `{forbidden}`."
        );
    }
    for semantic_marker in [
        "role=role_attr",
        "aria-label=aria_label",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "Well should keep stable semantic marker across SSR/wasm `{semantic_marker}`."
        );
    }

    // Toolchain compile evidence remains required for native + wasm + non-wasm motion stubs.
    for needle in [
        "cargo check -p ui-layout --no-default-features --features component-well,inject-css",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-well,inject-css",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "pub fn prefers_reduced_motion() -> bool {",
    ] {
        assert!(
            script_source.contains(needle) || motion_lib_source.contains(needle),
            "reduced-motion/SSR/wasm coverage guard should keep `{needle}`."
        );
    }
}

#[test]
fn well_wasm_debug_capability_stays_feature_isolated_and_non_polluting() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let mod_source = load_source("src/well/mod.rs");
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");
    let i18n_source = load_source("src/well/i18n.rs");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-layout should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-layout Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("well-wasm-debug"),
        "Well should not expose a dedicated wasm-debug feature because it is a non-interactive surface component."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{i18n_source}");
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
            !combined.contains(forbidden),
            "Well production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }
}

#[test]
fn well_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events() {
    let view_source = load_source("src/well/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for marker in [
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=tone_source_attr",
        "data-density=move || state.get().density_attr",
        "data-density-source=density_source_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Well should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
        "request_replay",
        "data-slot=\"button-debug-replay\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well has no interactive replay path; non-applicable interaction token `{forbidden}` should remain absent."
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
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }
}

#[test]
fn well_check2_documents_wasm_debug_governance_contract() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep wasm-debug governance rule `{required}`."
        );
    }
}

#[test]
fn well_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn well() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Tone + Density + Inset\" code_signal=tone_code>",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Well docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let checklist_source = load_source("src/well/check2.md");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for forbidden in [
        "WELL_WORKBENCH_STORAGE_KEY",
        "load_well_workbench_state(",
        "save_well_workbench_state(",
        "clear_well_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Well is non-interactive; optional persisted state is N/A for this component scope, so `{forbidden}` should remain absent."
        );
    }

    for required in [
        "- [ ] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn well_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-dx.sh");

    for needle in [
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/well/mod.rs");
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");
    let styles_source = load_source("src/well/styles.rs");
    let i18n_source = load_source("src/well/i18n.rs");
    let checklist_source = load_source("src/well/check2.md");

    assert!(
        !manifest_dir.join("src/well/spec.rs").exists(),
        "Well should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-well = []"),
        "Well feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-well = [\"dep:serde\"")
            && !cargo_source.contains("component-well = [\"dep:serde_json\""),
        "Well should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{i18n_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Well engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("../ui-components/src/button/view.rs");
    let combined = [
        load_source("src/well/mod.rs"),
        load_source("src/well/logic.rs"),
        load_source("src/well/view.rs"),
        load_source("src/well/styles.rs"),
        load_source("src/well/i18n.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_layout::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("well-wasm-debug"),
        "Well should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_layout::well::",
        "const WELL_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Well should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn well_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/well/mod.rs");
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");
    let styles_source = load_source("src/well/styles.rs");
    let i18n_source = load_source("src/well/i18n.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &i18n_source,
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
                "Well engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Well public module boundary should not leak web_sys types."
    );
}

#[test]
fn well_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-layout-engineering.sh");

    for needle in [
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn well_check2_documents_ui_layout_entrypoint_rules() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] `ui-layout` 固定入口文件落点正确。",
        "`crates/ui-layout/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-layout/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-layout/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-layout/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-layout/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-layout/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep ui-layout entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn well_ui_layout_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-well\")]",
        "pub mod well;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib entry should keep marker `{needle}`."
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
            "ui-layout lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn well_ui_layout_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-well\")]",
        "out.push_str(crate::well::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn well_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn well_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn well_ui_layout_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-layout forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn well_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-entrypoints.sh");

    let needle = "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_ui_layout_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn well_component_files_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-component-files.sh");

    for needle in [
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_component_directory_has_standard_file_layout",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn well_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_markers_are_schema_like_and_machine_readable",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn well_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-streaming.sh");

    for needle in [
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_semantics_are_not_required_for_snapshot_container_scope",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn well_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("src/well/view.rs");
    let script_source = load_source("../../scripts/check-ui-layout-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "Well should keep a single explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Well should keep one small `view!` block; no giant macro split needed for current scope."
    );
    assert!(
        view_source.lines().count() <= 120,
        "Well view.rs should stay compact; if this grows significantly, split into semantic subrenders."
    );

    for forbidden in [
        "{children()}\n        </section>\n    }\n}\n\n#[component]",
        "for item in",
        "collect::<Vec<_>>()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well view should avoid patterns that usually indicate giant macro/loop-heavy rendering `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn well_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("src/well/view.rs");
    let script_source = load_source("../../scripts/check-ui-layout-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Well should keep a single public component boundary for current simple layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn well_",
        "pub fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well should not introduce extra local components/render API noise for simple static layout `{forbidden}`."
        );
    }

    for needle in ["children: Children", "{children()}"] {
        assert!(
            view_source.contains(needle),
            "Well should keep explicit simple composition marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-layout --test well_semantics --no-default-features --features component-well,inject-css well_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn well_static_fragments_are_constantized_with_stable_a11y_semantics() {
    let view_source = load_source("src/well/view.rs");

    for needle in [
        "const SLOT_WELL: &str = \"well\";",
        "const STATE_INSET: &str = \"inset\";",
        "const STATE_DEFAULT: &str = \"default\";",
        "const BOOL_TRUE: &str = \"true\";",
        "fn inset_state_attr(is_inset: bool) -> &'static str",
        "data-slot=SLOT_WELL",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset=move || state.get().is_inset.then_some(BOOL_TRUE)",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)",
        "role=role_attr",
        "aria-label=aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Well static fragment contract should include `{needle}`."
        );
    }

    for forbidden in [
        "data-slot=\"well\"",
        "data-state=move || if state.get().is_inset { \"inset\" } else { \"default\" }",
        "data-inset=move || state.get().is_inset.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well should avoid scattered static literal fragment `{forbidden}` after constantization."
        );
    }
}

#[test]
fn well_check2_documents_static_fragment_constantization_policy() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep static-fragment constantization rule `{required}`."
        );
    }
}

#[test]
fn well_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("src/well/mod.rs");
    let i18n_source = load_source("src/well/i18n.rs");
    let logic_source = load_source("src/well/logic.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");
    let checklist_source = load_source("src/well/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !i18n_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Well should not use html injection path `{forbidden}`; this component has no trusted static-html requirement (N/A).",
        );
    }

    for required in [
        "- [ ] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep inner_html safety governance rule `{required}`."
        );
    }
}

#[test]
fn well_view_does_not_apply_local_default_fallbacks() {
    let source = load_source("src/well/view.rs");

    for forbidden in ["unwrap_or(", "unwrap_or_else("] {
        assert!(
            !source.contains(forbidden),
            "Well view should not apply local default fallback `{forbidden}`; defaults must be normalized in logic.rs."
        );
    }
}

#[test]
fn well_view_does_not_rebuild_state_machine_inputs() {
    let source = load_source("src/well/view.rs");

    for forbidden in [
        "WellStateInput {",
        "has_custom_label:",
        "has_custom_class_name:",
    ] {
        assert!(
            !source.contains(forbidden),
            "Well view should not rebuild state-machine input fragment `{forbidden}`; normalize in logic.rs."
        );
    }
}

#[test]
fn well_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/well/view.rs");

    for attr in [
        "data-slot=SLOT_WELL",
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=tone_source_attr",
        "data-density=move || state.get().density_attr",
        "data-density-source=density_source_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset=move || state.get().is_inset.then_some(BOOL_TRUE)",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)",
        "data-class-source=move || state.get().class_source_attr",
        "role=role_attr",
        "aria-label=aria_label",
        "lang=locale_lang.clone()",
        "dir=locale_dir",
    ] {
        assert!(
            source.contains(attr),
            "Well should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn well_state_source_markers_are_observable_and_closed_set() {
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");

    for required in [
        "pub fn source_attr_from_presence(is_present: bool) -> &'static str",
        "if is_present { \"prop\" } else { \"default\" }",
        "data-tone-source=tone_source_attr",
        "data-density-source=density_source_attr",
        "data-inset-source=inset_source_attr",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "Well should keep observable state-source marker contract `{required}`.",
        );
    }

    for forbidden in [
        "data-tone-source=move ||",
        "data-density-source=move ||",
        "format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Well source markers should be closed-set, not free-form/runtime-generated: `{forbidden}`."
        );
    }
}

#[test]
fn well_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/well_semantics.rs");

    for required in [
        "well_emits_baseline_style_state_data_attributes",
        "well_state_source_markers_are_observable_and_closed_set",
        "role=role_attr",
        "aria-label=aria_label",
        "data-state=move || inset_state_attr(state.get().is_inset)",
    ] {
        assert!(
            semantics_source.contains(required),
            "Well semantic test suite should assert contract marker `{required}`."
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
            "Well semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn well_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/well/view.rs");
    let semantics_source = load_source("tests/well_semantics.rs");

    for marker in [
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=tone_source_attr",
        "data-density=move || state.get().density_attr",
        "data-density-source=density_source_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "role=role_attr",
        "aria-label=aria_label",
    ] {
        assert!(
            view_source.contains(marker),
            "Well view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Well semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn well_token_first_styles_are_static_and_aggregated_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/well/styles.rs");
    let view_source = load_source("src/well/view.rs");
    let logic_source = load_source("src/well/logic.rs");
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "#[cfg(feature = \"component-well\")]",
        "out.push_str(crate::well::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "Component CSS aggregation should include well styles via `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] inject_components_css: bool",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should stay as centralized CSS injection boundary via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-radius-lg)",
        "var(--ui-border)",
        "var(--ui-fg)",
        "var(--ui-bg-muted)",
        "var(--ui-bg)",
        "var(--ui-accent-soft)",
        "var(--ui-accent)",
        "var(--ui-common-white)",
        "var(--ui-shadow-sm)",
    ] {
        assert!(
            styles_source.contains(required),
            "Well styles should remain token-first/static via `{required}`."
        );
    }

    for forbidden in [
        "--well-",
        "@apply",
        "tailwind",
        "tw-",
        "styled(",
        "stylex",
        "emotion",
        "css!(",
        "style!(",
        "view! {",
        "Callback::new",
        "format!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Well styles should not adopt utility-first, CSS-in-Rust, or runtime composition token `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "class=\"w-",
        "class=\"h-",
        "class=\"gap-",
        "tailwind",
        "tw!",
        "css!(",
        "style!(",
        "styled!(",
        "emotion",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Well component layer should not depend on utility-first/CSS-in-Rust default marker `{forbidden}`."
        );
    }

    for required in [
        "样式规则统一落在 `styles.rs`，由 `crates/ui-layout/src/css.rs` 聚合并通过 `UiRoot` 注入。",
        "Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。",
        "CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep token-first style governance guidance `{required}`."
        );
    }
}

#[test]
fn well_visual_desire_reuses_theme_visual_baseline_gate() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let well_docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "component_doc!(\n        \"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "theme visual baseline docs gate should include `{needle}`."
        );
    }

    for needle in [
        "E2E_VISUAL_BASELINE",
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression gate should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn well() -> AnyView",
        "title=\"Well\"",
        "slug=\"well\"",
        "description=\"Inset container surface for grouped content with centralized tone/density/label state contracts.\"",
    ] {
        assert!(
            well_docs_source.contains(needle),
            "Well docs entry should stay under default-theme quality gate `{needle}`."
        );
    }
}

#[test]
fn well_visual_desire_heroui_alignment_targets_experience_not_api_copy() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let well_docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let checklist_source = load_source("src/well/check2.md");

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should include alignment constraint `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Well>",
        "<Playground title=\"Tone + Density + Inset\" code_signal=tone_code>",
        "<Well tone=WellTone::Strong is_inset=true>",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
    ] {
        assert!(
            well_docs_source.contains(needle),
            "Well docs should keep simple-first + advanced-on-demand progression token `{needle}`."
        );
    }

    for needle in [
        "默认主题美学质量达标（Visual Desire）",
        "禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。",
        "HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Well checklist should keep visual desire governance rule `{needle}`."
        );
    }

    for forbidden in ["Bootstrap", "btn-default", "panel-default", "form-control"] {
        assert!(
            !well_docs_source.contains(forbidden),
            "Well docs should avoid legacy visual-regression token `{forbidden}`."
        );
    }
}

#[test]
fn well_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_layout_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-well = []",
        "inject-css = []",
    ] {
        assert!(
            ui_layout_cargo.contains(needle),
            "ui-layout Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-well\")]\npub mod well;"),
        "lib.rs should feature-gate well module export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-well\")]")
            && css_source.contains("out.push_str(crate::well::styles::CSS);"),
        "css.rs should gate well CSS aggregation behind component-well feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-layout via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn well_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-layout-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-layout -p ui-layout --no-default-features --features",
        "cargo tree -e features -i ui-layout -p web-demo",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-layout --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn well_check2_documents_type_system_and_machine_readable_state_contract() {
    let checklist_source = load_source("src/well/check2.md");

    for required in [
        "- [ ] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。",
        "无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。",
        "关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。",
        "编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Well checklist should keep type-system + semantic-marker governance rule `{required}`."
        );
    }
}

#[test]
fn well_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/well.rs");
    let logic_source = load_source("src/well/logic.rs");
    let view_source = load_source("src/well/view.rs");

    for required in [
        "pub enum WellTone",
        "pub enum WellDensity",
        "pub struct WellStateInput",
        "pub struct WellState",
        "pub fn resolve_state(input: WellStateInput) -> WellState",
        "pub fn normalize_props(input: WellNormalizeInput) -> WellNormalizedProps",
        "source_attr_from_presence",
    ] {
        assert!(
            primitives_source.contains(required) || logic_source.contains(required),
            "Well state contract should stay type-first and normalized via `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "density: Option<String>",
        "tone: String",
        "density: String",
        "tone_attr: String",
        "density_attr: String",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitives_source.contains(forbidden),
            "Well should avoid string protocol drift for discrete state axis `{forbidden}`."
        );
    }

    for marker in [
        "data-tone=move || state.get().tone_attr",
        "data-tone-source=tone_source_attr",
        "data-density=move || state.get().density_attr",
        "data-density-source=density_source_attr",
        "data-state=move || inset_state_attr(state.get().is_inset)",
        "data-inset-source=inset_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Well machine-readable semantic contract should expose marker `{marker}`."
        );
    }
}

#[test]
fn well_styles_include_tone_density_and_source_markers() {
    let source = load_source("src/well/styles.rs");

    for selector in [
        ".ui-well--density-comfortable",
        ".ui-well[data-density=\"comfortable\"]",
        ".ui-well--density-compact",
        ".ui-well[data-density=\"compact\"]",
        ".ui-well--tone-default",
        ".ui-well[data-tone=\"default\"]",
        ".ui-well--tone-quiet",
        ".ui-well--tone-strong",
        ".ui-well--inset",
        ".ui-well[data-inset=\"true\"]",
        ".ui-well--label-custom",
        ".ui-well[data-label-source=\"custom\"]",
        ".ui-well--custom-class",
        ".ui-well[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Well styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn well_styles_do_not_guess_state_from_dom_structure() {
    let source = load_source("src/well/styles.rs");

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        ":only-child",
        ".ui-well .",
        ".ui-well >",
        ".ui-well +",
        ".ui-well ~",
    ] {
        assert!(
            !source.contains(forbidden),
            "Well styles should not infer state from brittle DOM structure selector `{forbidden}`."
        );
    }
}

#[test]
fn well_view_keeps_runtime_styles_out_of_inline_business_logic() {
    let source = load_source("src/well/view.rs");

    for forbidden in ["style=", "style:"] {
        assert!(
            !source.contains(forbidden),
            "Well view should not carry business style logic inline via `{forbidden}`; rely on data/class markers and static CSS."
        );
    }
}

#[test]
fn well_styles_consume_theme_tokens_without_private_color_constants() {
    let source = load_source("src/well/styles.rs");

    for needle in [
        "var(--ui-radius-lg)",
        "var(--ui-border)",
        "var(--ui-fg)",
        "var(--ui-bg-muted)",
        "var(--ui-bg)",
        "var(--ui-accent-soft)",
        "var(--ui-accent)",
        "var(--ui-common-white)",
        "var(--ui-shadow-sm)",
    ] {
        assert!(
            source.contains(needle),
            "Well styles should consume theme token `{needle}`."
        );
    }

    assert!(
        !source.contains("color-mix(in oklab, white"),
        "Well styles should not hardcode `white`; consume theme token variables instead."
    );
}

#[test]
fn well_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn well() -> AnyView",
        "title=\"Well\"",
        "slug=\"well\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Tone + Density + Inset\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Well.",
        );
    }
}

#[test]
fn well_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Hello World\"",
        "<Well>\n  <div>\"Default well\"</div>\n</Well>",
        "title=\"Tone + Density + Inset\"",
        "<Well tone=WellTone::Default>",
        "<Well tone=WellTone::Quiet density=WellDensity::Compact>",
        "<Well tone=WellTone::Strong is_inset=true>",
        "title=\"Custom Label + Class\"",
        "aria_label=\"Selection summary\".to_string()",
        "class_name=\"docs-well-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "well docs playgrounds should contain `{needle}`.",
        );
    }
}
