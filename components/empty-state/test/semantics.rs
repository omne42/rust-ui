use std::fs;
use std::path::Path;

fn load_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_workspace_source(rel_path_from_repo_root: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("../..").join(rel_path_from_repo_root);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn empty_state_component_files_are_split_by_responsibility() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        let path = manifest_dir.join(rel_path);
        assert!(path.exists(), "missing component contract file: {path:?}");
    }
}

#[test]
fn empty_state_component_files_keep_responsibility_boundaries() {
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "EmptyStateStrings",
        "pub use view::EmptyState;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep export boundary `{required}`."
        );
    }

    for forbidden in ["view! {", "NodeRef", "set_property(", "SpringAnimator::new"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for required in ["pub fn resolve_defaults(", "pub fn resolve_render_state("] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation entry `{required}`."
        );
    }

    for forbidden in [
        "NodeRef",
        "view! {",
        "set_property(",
        "--ui-empty-state-enter",
        ".ui-empty-state",
        "web_sys::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should avoid DOM/style details `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`."
        );
    }

    for forbidden in [
        "Nothing to show",
        "Try adjusting filters or refreshing data.",
        "Empty state",
        "view! {",
        "fn ",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not contain business copy or runtime logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "live_region_attrs(LiveRegionPriority::Polite)",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
        "logic::resolve_defaults(",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should mount structure/contract `{required}`."
        );
    }

    for forbidden in [
        "set_property(",
        "SpringAnimator::new(",
        "ui_motion::spring::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not embed motion engine detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn sanitize_motion(motion: EmptyStateMotion) -> EmptyStateMotion",
        "pub fn attach_motion<E>(",
        "ui_motion::spring::SpringAnimator::new(",
        "ui_motion::presets::spring_soft()",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should map semantics to shared motion contract via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "live_region_attrs(",
        "locale_attrs(",
        "resolve_defaults(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not take over view/headless/logic responsibilities `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_keeps_spec_rs_out_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "spec.rs should not exist for simple EmptyState component."
    );

    let mod_source = load_component_source("mod.rs");
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not expose optional spec layer for EmptyState: `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_hyper_structure_builder_spec_is_not_applicable() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple EmptyState should not introduce Hyper-Structure builder spec.rs."
    );

    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "pub mod spec;",
        "mod spec;",
        "pub use spec::",
        "struct EmptyStateSpec",
        "impl EmptyStateSpec",
        "Spec::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "simple component should avoid Hyper-Structure builder marker `{forbidden}`."
        );
    }

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`EmptyState` 为简单展示组件，不属于需要独立 Schema/Builder 固化的复杂配置域；保持无 `spec.rs`、无 `*Spec::new()...render()` API 可避免过度抽象与导出噪音。）",
        "empty_state_hyper_structure_builder_spec_is_not_applicable",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 Hyper-Structure builder evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_context_compression_manifest_and_rbi_are_present() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let manifest_source = load_component_source("Component.toml");
    let rbi_source = load_component_source("empty_state.rbi");

    for required in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"EmptyState\"",
        "crate = \"ui-empty-state\"",
        "[[capabilities]]",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent_contract_schema\"",
        "[[dependencies]]",
        "name = \"ui-state-primitives\"",
        "name = \"ui-headless\"",
        "name = \"ui-motion\"",
        "name = \"leptos\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub enum EmptyStateTone {",
        "pub enum EmptyStateAlign {",
        "pub struct EmptyStateMotion {",
        "pub struct EmptyStateStrings {",
        "pub const EMPTY_STATE_AGENT_SCHEMA_NAME: &str;",
        "pub const EMPTY_STATE_AGENT_SCHEMA_VERSION: &str;",
        "pub enum EmptyStateAgentIntent {",
        "pub enum EmptyStateAgentAction {",
        "pub enum EmptyStateAgentSource {",
        "pub struct EmptyStateAgentContract {",
        "pub fn resolve_agent_contract(",
        "pub fn EmptyState(",
        "icon: Option<leptos::children::ViewFn>",
        "actions: Option<leptos::children::ViewFn>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty_state.rbi should keep signature projection marker `{required}`."
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "empty_state_context_compression_manifest_and_rbi_are_present",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 Manifest+RBI evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_agent_contract_schema_is_typed_and_mounted() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let manifest_source = load_component_source("Component.toml");
    let rbi_source = load_component_source("empty_state.rbi");

    for required in [
        "pub const EMPTY_STATE_AGENT_SCHEMA_NAME: &str = \"ui-empty-state-agent-contract\";",
        "pub const EMPTY_STATE_AGENT_SCHEMA_VERSION: &str = \"1\";",
        "pub enum EmptyStateAgentIntent",
        "pub enum EmptyStateAgentAction",
        "pub enum EmptyStateAgentSource",
        "pub struct EmptyStateAgentContract",
        "pub fn resolve_agent_contract(",
        "pub agent_contract: EmptyStateAgentContract,",
        "let agent_contract = resolve_agent_contract(state, motion_source_attr);",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should define typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "data-ui-schema=move || state.get().agent_contract.schema_name",
        "data-ui-schema-version=move || state.get().agent_contract.schema_version",
        "data-ui-intent=move || state.get().agent_contract.intent.as_attr()",
        "data-ui-action=move || state.get().agent_contract.action.as_attr()",
        "data-ui-state=move || state.get().agent_contract.state",
        "data-ui-source=move || state.get().agent_contract.source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should mount typed agent-contract field `{required}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema\"",
        "data-ui-schema",
        "data-ui-schema-version",
        "data-ui-intent",
        "data-ui-action",
        "data-ui-state",
        "data-ui-source",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should include agent-contract schema field `{required}`."
        );
    }

    for required in [
        "pub const EMPTY_STATE_AGENT_SCHEMA_NAME: &str;",
        "pub const EMPTY_STATE_AGENT_SCHEMA_VERSION: &str;",
        "pub enum EmptyStateAgentIntent {",
        "pub enum EmptyStateAgentAction {",
        "pub enum EmptyStateAgentSource {",
        "pub struct EmptyStateAgentContract {",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty_state.rbi should project agent-contract signature `{required}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "empty_state_agent_contract_schema_is_typed_and_mounted",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 Agent Contract evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_llm_streaming_render_modes_are_not_applicable() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let manifest_source = load_component_source("Component.toml");
    let rbi_source = load_component_source("empty_state.rbi");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "is_streaming",
        "data-ui-stream-progress",
        "data-ui-token-count",
        "stream_delta",
        "token_delta",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "EmptyState should not expose LLM streaming render mode protocol `{forbidden}`."
        );
    }

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`EmptyState` 为通用空态展示组件，不是 LLM 正文输出渲染面；当前实现不引入 `Streaming/Snapshot` 模式切换协议，也不暴露 `AiRenderMode/AiOutputStatus` 语义字段，避免把非问题复杂化。若未来承担 LLM 正文渲染职责，再按该协议补齐双模式契约。）",
        "empty_state_llm_streaming_render_modes_are_not_applicable",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 LLM streaming definition evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_streaming_policy_is_optional_with_snapshot_fallback() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let manifest_source = load_component_source("Component.toml");
    let rbi_source = load_component_source("empty_state.rbi");

    for required in [
        "pub enum EmptyStateStreamingSupport",
        "pub enum EmptyStateRenderMode",
        "pub enum EmptyStateOutputStatus",
        "streaming_support: EmptyStateStreamingSupport::Optional",
        "render_mode: EmptyStateRenderMode::Snapshot",
        "fallback_mode: EmptyStateRenderMode::Snapshot",
        "output_status: EmptyStateOutputStatus::Validated",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should define optional-streaming snapshot fallback policy `{required}`."
        );
    }

    for required in [
        "data-ui-streaming=move || state.get().agent_contract.streaming_support.as_attr()",
        "data-ui-render-mode=move || state.get().agent_contract.render_mode.as_attr()",
        "data-ui-fallback=move || state.get().agent_contract.fallback_mode.as_attr()",
        "data-ui-output-status=move || state.get().agent_contract.output_status.as_attr()",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose readable streaming/fallback/output status marker `{required}`."
        );
    }

    for required in [
        "name = \"streaming_optional_snapshot_fallback\"",
        "data-ui-streaming",
        "data-ui-render-mode",
        "data-ui-fallback",
        "data-ui-output-status",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should declare streaming policy metadata `{required}`."
        );
    }

    for required in [
        "pub enum EmptyStateStreamingSupport {",
        "pub enum EmptyStateRenderMode {",
        "pub enum EmptyStateOutputStatus {",
        "pub streaming_support: EmptyStateStreamingSupport,",
        "pub render_mode: EmptyStateRenderMode,",
        "pub fallback_mode: EmptyStateRenderMode,",
        "pub output_status: EmptyStateOutputStatus,",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty_state.rbi should project streaming policy signature `{required}`."
        );
    }

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "fallback=snapshot",
        "empty_state_streaming_policy_is_optional_with_snapshot_fallback",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 streaming policy evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_snapshot_rendering_is_supported_as_default_baseline() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let manifest_source = load_component_source("Component.toml");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");

    for required in [
        "name = \"snapshot_rendering\"",
        "enabled = true",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should declare snapshot baseline capability `{required}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn EmptyState(",
        "logic::resolve_defaults(",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "data-state=move || state.get().state.data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should support stable snapshot render baseline via `{required}`."
        );
    }

    for required in [
        "pub struct EmptyStateStrings",
        "impl Default for EmptyStateStrings",
        "default_title: Cow::Borrowed(DEFAULT_TITLE)",
        "default_description: Cow::Borrowed(DEFAULT_DESCRIPTION)",
        "default_aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should provide full-result defaults for snapshot render `{required}`."
        );
    }

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "empty_state_snapshot_rendering_is_supported_as_default_baseline",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 snapshot baseline evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_component_directory_standard_file_layout_is_enforced() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        let path = manifest_dir.join(rel_path);
        assert!(
            path.exists(),
            "standard component directory should include {:?}.",
            path
        );
    }

    let mut rs_entries = fs::read_dir(&src_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {src_dir:?}: {e}"))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed: {e}"));
            entry.file_name().to_string_lossy().to_string()
        })
        .filter(|name| name.ends_with(".rs"))
        .collect::<Vec<_>>();
    rs_entries.sort();
    assert_eq!(
        rs_entries,
        vec!["logic.rs", "mod.rs", "motion.rs", "styles.rs", "view.rs"],
        "component src directory should only contain standard rust files."
    );

    let render_path = manifest_dir.join("src/render.rs");
    assert!(
        !render_path.exists(),
        "render.rs is forbidden; keep rendering in view.rs: {render_path:?}"
    );

    let spec_path = manifest_dir.join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "spec.rs is optional for complex components only: {spec_path:?}"
    );

    let mod_source = load_component_source("mod.rs");
    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "EmptyStateStrings",
        "pub use view::EmptyState;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep minimal stable export boundary `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod render;",
        "pub mod render;",
        "pub use logic::resolve_defaults",
        "pub use logic::resolve_render_state",
        "pub use logic::compose_class_name",
        "pub use motion::attach_motion",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should avoid over-exporting implementation detail `{forbidden}`."
        );
    }

    let logic_source = load_component_source("logic.rs");
    for required in [
        "pub fn resolve_defaults(",
        "pub fn resolve_render_state(input: EmptyStateRenderStateInput) -> EmptyStateRenderState",
        "ui_state_primitives::empty_state",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation contract `{required}`."
        );
    }
    for forbidden in ["web_sys::", "view! {", "set_property(", "NodeRef"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not absorb DOM/view/motion implementation `{forbidden}`."
        );
    }

    let styles_source = load_component_source("styles.rs");
    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should keep static CSS contract constant."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume ui token variables."
    );
    for forbidden in [
        "#fff", "#ffffff", "#000", "#000000", "rgb(", "rgba(", "hsl(", "hsla(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid hardcoded theme color literal `{forbidden}`."
        );
    }

    let view_source = load_component_source("view.rs");
    for required in [
        "view! {",
        "live_region_attrs(LiveRegionPriority::Polite)",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
        "logic::resolve_defaults(",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure + headless/logic mount `{required}`."
        );
    }
    for forbidden in ["mod render;", "pub mod render;"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not drift through render.rs indirection `{forbidden}`."
        );
    }

    let motion_source = load_component_source("motion.rs");
    for required in [
        "pub struct EmptyStateMotion",
        "pub fn attach_motion<E>(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract mapping entry `{required}`."
        );
    }
    for forbidden in ["view! {", "live_region_attrs(", "locale_attrs("] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not absorb view/headless semantics `{forbidden}`."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "empty_state_component_directory_standard_file_layout_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 standard-file-layout evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_follows_token_first_static_css_contract() {
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    let ui_root_source = load_workspace_source("crates/ui-components/src/root.rs");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "[data-tone=\"default\"]",
        "[data-motion-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static css marker `{required}`."
        );
    }

    for forbidden in ["@apply", "tw-", "tailwind", "styled(", "emotion"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid utility/css-in-rust leakage `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-empty_state\")]",
        "out.push_str(crate::empty_state::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css aggregator should include empty-state via `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(required),
            "UiRoot should inject aggregated component css through `{required}`."
        );
    }

    for forbidden in ["style=", "style:", "attr:style"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not host business inline style `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-empty-state-enter\","),
        "runtime style updates should stay limited to required css custom property."
    );
}

#[test]
fn empty_state_visual_baseline_uses_token_hierarchy_and_docs_entry() {
    let styles_source = load_component_source("styles.rs");
    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");

    for required in [
        "font-size: var(--ui-heading-h5-font-size, var(--ui-fallback-heading-h5-font-size));",
        "--ui-heading-h5-line-height,",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "gap: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "padding: var(--ui-space-xl, var(--ui-fallback-space-xl));",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "color-mix(in oklab,",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep visual hierarchy token baseline `{required}`."
        );
    }

    for forbidden in [
        "bootstrap",
        "--bs-",
        ".btn",
        ".card",
        ".panel",
        "form-control",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not regress to legacy utility/framework aesthetic `{forbidden}`."
        );
    }

    for required in [
        "pub(super) fn empty_state() -> AnyView",
        "title=\"EmptyState\"",
        "Playground title=\"Hello World (Default Path)\"",
        "Playground\n                title=\"State Matrix\"",
        "Playground title=\"Tone + Alignment + Actions\"",
        "Playground title=\"Compact + Bordered + Custom Class\"",
        "Playground\n                title=\"Controlled vs Uncontrolled (N/A)\"",
        "Playground\n                title=\"Streaming Optional / Snapshot\"",
        "Playground\n                title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=empty_state_imports.clone()",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app should keep EmptyState default-theme baseline entry `{required}`."
        );
    }
}

#[test]
fn empty_state_tree_shaking_feature_gates_are_wired() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let ui_components_cargo = load_workspace_source("crates/ui-components/Cargo.toml");
    let ui_components_lib = load_workspace_source("crates/ui-components/src/lib.rs");
    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    let web_demo_cargo = load_workspace_source("apps/web-demo/Cargo.toml");

    for required in [
        "component-empty_state = [\"dep:ui-empty-state\"]",
        "ui-empty-state = { path = \"../../components/empty-state\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components feature graph should keep empty-state tree-shaking gate `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-empty_state\")]",
        "pub use ui_empty_state as empty_state;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "lib.rs should guard empty-state export with feature gate `{required}`."
        );
    }

    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-empty_state\")]\n    out.push_str(crate::empty_state::styles::CSS);"
        ),
        "css.rs should guard empty-state CSS aggregation with component feature."
    );

    for required in [
        "features = [\"inject-css\", \"web-demo-components\"]",
        "default-features = false",
    ] {
        assert!(
            web_demo_cargo.contains(required),
            "web-demo should consume scoped feature set for tree-shaking `{required}`."
        );
    }

    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not implicitly pull ui-components all-components feature."
    );

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "empty_state_tree_shaking_feature_gates_are_wired",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 tree-shaking evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_ui_components_fixed_entry_points_are_wired() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let ui_components_lib = load_workspace_source("crates/ui-components/src/lib.rs");
    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    let ui_components_root = load_workspace_source("crates/ui-components/src/root.rs");
    let ui_visual_active_highlight =
        load_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable =
        load_workspace_source("crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_workspace_source("crates/ui-headless/src/presence.rs");
    let headless_a11y = load_workspace_source("crates/ui-headless/src/a11y.rs");

    for required in [
        "#[cfg(feature = \"component-empty_state\")]",
        "pub use ui_empty_state as empty_state;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components/lib.rs should keep fixed entry marker `{required}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components/lib.rs should not expose forbidden legacy entry `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-empty_state\")]",
        "out.push_str(crate::empty_state::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components/css.rs should keep feature-gated css entry `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui-components/root.rs should keep centralized root injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            ui_visual_active_highlight.contains(required),
            "ui-visual-primitive active_highlight contract should include `{required}`."
        );
    }

    for forbidden in ["Accordion", "Tabs", "Menu", "Popover", "EmptyState"] {
        assert!(
            !ui_visual_active_highlight.contains(forbidden),
            "active_highlight should stay generic, not component-business specific `{forbidden}`."
        );
    }

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
    ] {
        assert!(
            headless_controllable.contains(required),
            "ui-headless controllable_state entry should include `{required}`."
        );
    }
    assert!(
        headless_presence.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "ui-headless presence entry should expose use_presence."
    );
    for required in ["pub fn locale_attrs(", "pub fn live_region_attrs("] {
        assert!(
            headless_a11y.contains(required),
            "ui-headless a11y entry should expose `{required}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_components_src = manifest_dir.join("../..").join("crates/ui-components/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = ui_components_src.join(forbidden_file);
        assert!(
            !path.exists(),
            "ui-components fixed entry rule forbids file {:?}.",
            path
        );
    }

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "empty_state_ui_components_fixed_entry_points_are_wired",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 fixed-entry evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_type_system_and_semantic_markers_form_machine_readable_contract() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/empty_state.rs");

    for required in [
        "#[prop(optional)] tone: EmptyStateTone",
        "#[prop(optional)] align: EmptyStateAlign",
        "logic::resolve_defaults(",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
        "data-state=move || state.get().state.data_state_attr",
        "data-title-source=move || state.get().state.title_source_attr",
        "data-description-source=move || state.get().state.description_source_attr",
        "data-aria-source=move || state.get().state.aria_source_attr",
        "data-class-source=move || state.get().state.class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "type+semantic contract should expose `{required}` for machine-readable state."
        );
    }

    for required in [
        "pub struct EmptyStateRenderStateInput",
        "pub fn resolve_render_state(input: EmptyStateRenderStateInput) -> EmptyStateRenderState",
        "pub fn resolve_defaults(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should centralize invalid-state normalization via `{required}`."
        );
    }

    for required in [
        "pub data_state_attr: &'static str",
        "pub title_source_attr: &'static str",
        "pub description_source_attr: &'static str",
        "pub aria_source_attr: &'static str",
        "pub class_source_attr: &'static str",
    ] {
        assert!(
            primitive_source.contains(required),
            "state-primitives should expose typed marker field `{required}`."
        );
    }
}

#[test]
fn empty_state_view_mounts_headless_and_motion_contracts() {
    let source = load_component_source("view.rs");

    for needle in [
        "live_region_attrs(LiveRegionPriority::Polite)",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
        "logic::resolve_defaults(",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get().state)",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            source.contains(needle),
            "view.rs should mount contract `{needle}` from headless/primitives/motion."
        );
    }
}

#[test]
fn empty_state_has_a11y_i18n_l10n_contract_entry_points() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for required in [
        "use ui_headless::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<EmptyStateStrings>();",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-label=aria_label",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "strings.default_title.as_ref()",
        "strings.default_description.as_ref()",
        "strings.default_aria_label.as_ref()",
    ] {
        assert!(
            view_source.contains(required),
            "EmptyState should mount A11y/i18n contract entry `{required}`."
        );
    }

    for required in [
        "pub struct EmptyStateStrings",
        "impl Default for EmptyStateStrings",
        "default_title: Cow::Borrowed(DEFAULT_TITLE)",
        "default_description: Cow::Borrowed(DEFAULT_DESCRIPTION)",
        "default_aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should provide localizable defaults through `{required}`."
        );
    }

    for forbidden in ["fn live_region_attrs", "fn locale_attrs"] {
        assert!(
            !view_source.contains(forbidden),
            "component should consume shared ui-headless A11y tool, not redefine `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_rust_hygiene_contract_is_enforced() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let rbi_source = load_component_source("empty_state.rbi");

    for forbidden in [
        ".unwrap(",
        ".unwrap_err(",
        ".expect(",
        "let _ =",
        ".to_owned(",
        "String::from(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "non-test component source should satisfy rust-hygiene rule `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "pub default_title: Cow<'static, str>,",
        "pub default_description: Cow<'static, str>,",
        "pub default_aria_label: Cow<'static, str>,",
        "default_title: Cow::Borrowed(DEFAULT_TITLE),",
        "default_description: Cow::Borrowed(DEFAULT_DESCRIPTION),",
        "default_aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL),",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should centralize default copy via Cow<'static, str> marker `{required}`."
        );
    }

    for required in [
        "pub default_title: std::borrow::Cow<'static, str>,",
        "pub default_description: std::borrow::Cow<'static, str>,",
        "pub default_aria_label: std::borrow::Cow<'static, str>,",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty_state.rbi should project Cow-based string contract `{required}`."
        );
    }

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "empty_state_rust_hygiene_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 rust-hygiene evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_exposes_stable_observable_and_source_markers() {
    let view_source = load_component_source("view.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/empty_state.rs");

    for required in [
        "data-tone=move || state.get().state.tone_attr",
        "data-align=move || state.get().state.align_attr",
        "data-state=move || state.get().state.data_state_attr",
        "data-compact=move || state.get().state.is_compact.then_some(\"true\")",
        "data-bordered=move || state.get().state.is_bordered.then_some(\"true\")",
        "data-icon=move || state.get().state.has_icon.then_some(\"true\")",
        "data-actions=move || state.get().state.has_actions.then_some(\"true\")",
        "data-title-source=move || state.get().state.title_source_attr",
        "data-description-source=move || state.get().state.description_source_attr",
        "data-aria-source=move || state.get().state.aria_source_attr",
        "data-class-source=move || state.get().state.class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-label=aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose stable observable marker `{required}`."
        );
    }

    for required in [
        "pub tone_attr: &'static str",
        "pub align_attr: &'static str",
        "pub data_state_attr: &'static str",
        "pub title_source_attr: &'static str",
        "pub description_source_attr: &'static str",
        "pub aria_source_attr: &'static str",
        "pub class_source_attr: &'static str",
        "\"default\"",
        "\"muted\"",
        "\"accent\"",
        "\"start\"",
        "\"center\"",
        "\"rich\"",
        "\"actions\"",
        "\"icon\"",
        "\"plain\"",
        "\"custom\"",
    ] {
        assert!(
            primitive_source.contains(required),
            "state-primitives should define closed marker set with `{required}`."
        );
    }
}

#[test]
fn empty_state_styles_depend_on_explicit_state_markers() {
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for required in [
        "[data-tone=\"default\"]",
        "[data-tone=\"muted\"]",
        "[data-tone=\"accent\"]",
        "[data-align=\"start\"]",
        "[data-align=\"center\"]",
        "[data-compact=\"true\"]",
        "[data-bordered=\"true\"]",
        "[data-custom-class=\"true\"]",
        "[data-motion-source=\"custom\"]",
        "[data-custom-motion=\"true\"]",
        ".ui-empty-state--tone-default",
        ".ui-empty-state--tone-muted",
        ".ui-empty-state--tone-accent",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should branch on explicit state marker `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":has(", ":empty"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not infer state from fragile DOM structure `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "attr:style"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not carry inline business style logic `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-empty-state-enter\","),
        "motion.rs should only write necessary custom properties for runtime style updates."
    );
}

#[test]
fn empty_state_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_component_source("styles.rs");
    let theme_css_source = load_workspace_source("crates/ui-theme/src/css.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xl, var(--ui-fallback-space-xl))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-heading-h5-font-size, var(--ui-fallback-heading-h5-font-size))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep defensive fallback chain token `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-xl:",
        "--ui-fallback-space-md:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-border-width:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-bg:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-border:",
        "--ui-fallback-accent:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-heading-h5-font-size:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for fallback token `{required}`."
        );
    }

    for forbidden in [
        "#fff",
        "#ffffff",
        "#000",
        "#000000",
        "max-width: 46ch;",
        "border: 1px solid transparent;",
        "border: 1px dashed",
        "outline: 1px solid",
        "var(--ui-space-sm)",
        "var(--ui-space-xl)",
        "var(--ui-radius-lg)",
        "var(--ui-bg-muted)",
        "var(--ui-fg)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not keep hardcoded terminal or single-layer token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "empty_state_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 defensive-variable evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let css_source = load_workspace_source("crates/ui-components/src/css.rs");
    let root_source = load_workspace_source("crates/ui-components/src/root.rs");
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-empty_state\")]",
        "out.push_str(crate::empty_state::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            css_source.contains(required) || root_source.contains(required),
            "cascade-layer contract should keep marker `{required}`."
        );
    }

    for forbidden in [
        "style=",
        "style:",
        "attr:style",
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"right\"",
        "set_property(\"bottom\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "runtime style contract should avoid non-variable inline style marker `{forbidden}`."
        );
    }

    for required in [
        "set_property(\"--ui-empty-state-enter\",",
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "empty_state_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            motion_source.contains(required) || check2_source.contains(required),
            "check2/runtime-style evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_non_interactive_matrix_axes_are_explicitly_not_applicable() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for forbidden in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] disabled: bool",
        "disabled=",
        "aria-disabled",
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:pointerdown",
        "on:pointerup",
        "on:pointermove",
        "on:click",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "EmptyState is non-interactive; matrix axis should remain N/A for `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_motion_contract_covers_wasm_and_ssr_paths() {
    let motion_source = load_component_source("motion.rs");

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion<E>(node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)",
        "pub fn attach_motion<E>(_node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should provide wasm/ssr contract branch `{required}`."
        );
    }
}

#[test]
fn empty_state_depends_on_ui_motion_non_wasm_noop_contract() {
    let ui_motion_lib = load_workspace_source("crates/ui-motion/src/lib.rs");
    let component_motion = load_component_source("motion.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm noop/stub contract marker `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion<E>(_node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)",
        "std::hint::black_box(sanitize_motion(motion));",
        "let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);",
        "if let Some(animator) = spring_for_cleanup.get_value() {",
    ] {
        assert!(
            component_motion.contains(required),
            "empty-state motion should safely degrade/no-assume animation handle via `{required}`."
        );
    }

    for forbidden in ["panic!(", "expect(", "unwrap()"] {
        assert!(
            !component_motion.contains(forbidden),
            "component motion path should avoid panic-prone assumptions `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_reduced_motion_ssr_and_wasm_semantics_stay_consistent() {
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for required in [
        "if !motion.animate_in || ui_motion::web::prefers_reduced_motion() {",
        "@media (prefers-reduced-motion: reduce) {",
        "transform: none;",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required) || styles_source.contains(required),
            "reduced-motion/wasm contract should keep explicit branch marker `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().state.data_state_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(required),
            "SSR/wasm should share the same semantic markers in view.rs via `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if cfg!(target_arch = \"wasm32\")",
        "if cfg!(not(target_arch = \"wasm32\"))",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not split semantic contract by platform branch `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_motion_contract_is_parameterized_and_attached_via_ui_motion() {
    let motion_source = load_component_source("motion.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "const EMPTY_STATE_SPRING_STIFFNESS: f64 = 280.0;",
        "const EMPTY_STATE_SPRING_DAMPING: f64 = 20.0;",
        "const EMPTY_STATE_SPRING_MASS: f64 = 1.0;",
        "const EMPTY_STATE_SPRING_PRECISION: f64 = 0.001;",
        "fn empty_state_spring_contract() -> ui_motion::spring::SpringConfig {",
        "let fallback = ui_motion::presets::spring_soft();",
        "ui_motion::spring::sanitize_config(",
        "ui_motion::spring::SpringConfig {",
        "stiffness: EMPTY_STATE_SPRING_STIFFNESS,",
        "damping: EMPTY_STATE_SPRING_DAMPING,",
        "mass: EMPTY_STATE_SPRING_MASS,",
        "precision: EMPTY_STATE_SPRING_PRECISION,",
        "empty_state_spring_contract(),",
        "if !motion.animate_in || ui_motion::web::prefers_reduced_motion() {",
        "pub fn attach_motion<E>(_node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep component motion contract marker `{required}`."
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "empty_state_motion_contract_is_parameterized_and_attached_via_ui_motion",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 motion-contract evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_performance_governance_is_mount_only_traceable_and_blocking() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-performance.sh");
    let view_source = load_component_source("view.rs");

    for required in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`EmptyState` 为展示型非交互组件",
        "mount-only",
        "render_count",
        "等价证据",
        "渲染次数预算为 `1`",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 performance governance evidence should include `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test empty_state_semantics --no-default-features --features component-empty_state,inject-css empty_state_performance_governance_contract_is_mount_only_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );

    for required in [
        "data-state=move || state.get().state.data_state_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "EmptyState should expose attribution marker `{required}` for perf triage."
        );
    }
}

#[test]
fn empty_state_semantic_and_performance_regression_contract_is_covered() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let view_source = load_component_source("view.rs");
    let ui_components_semantics =
        load_workspace_source("components/empty-state/test/empty_state_semantics.rs");

    for required in [
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-label=aria_label",
        "data-state=move || state.get().state.data_state_attr",
        "data-title-source=move || state.get().state.title_source_attr",
        "data-description-source=move || state.get().state.description_source_attr",
        "data-aria-source=move || state.get().state.aria_source_attr",
        "data-class-source=move || state.get().state.class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose semantic-regression marker `{required}`."
        );
    }

    for required in [
        "fn empty_state_performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "render_count",
        "Button/Input/Accordion",
    ] {
        assert!(
            ui_components_semantics.contains(required),
            "ui-components semantic/perf regression suite should contain `{required}`."
        );
    }

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "empty_state_semantic_and_performance_regression_contract_is_covered",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 semantic/perf regression evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_semantic_tests_prioritize_contract_over_visual_snapshot() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let view_source = load_component_source("view.rs");
    let component_semantics_source =
        load_workspace_source("components/empty-state/test/semantics.rs");
    let ui_components_semantics_source =
        load_workspace_source("components/empty-state/test/empty_state_semantics.rs");

    for required in [
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-label=aria_label",
        "data-state=move || state.get().state.data_state_attr",
        "data-title-source=move || state.get().state.title_source_attr",
        "data-description-source=move || state.get().state.description_source_attr",
        "data-aria-source=move || state.get().state.aria_source_attr",
        "data-class-source=move || state.get().state.class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "semantic-first regression requires contract marker `{required}`."
        );
    }

    for required in [
        "fn empty_state_has_a11y_i18n_l10n_contract_entry_points()",
        "fn empty_state_exposes_stable_observable_and_source_markers()",
        "fn empty_state_semantic_and_performance_regression_contract_is_covered()",
        "fn empty_state_semantic_tests_prioritize_contract_over_visual_snapshot()",
    ] {
        assert!(
            component_semantics_source.contains(required),
            "component semantic suite should retain contract regression `{required}`."
        );
    }

    for forbidden in [
        "assert_snapshot",
        "to_match_snapshot",
        "insta::assert_snapshot",
        "insta::assert_yaml_snapshot",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden)
                && !ui_components_semantics_source.contains(forbidden),
            "semantic suites should not regress into snapshot-only contract checks `{forbidden}`."
        );
    }

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "empty_state_semantic_tests_prioritize_contract_over_visual_snapshot",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 semantic-first evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_empty_state_contract.spec.mjs");

    for required in [
        "docs-app empty-state uses semantic selectors with wasm-stable ready waits",
        "docs-app empty-state flow is repeatable with semantic ready/settled breakpoints",
        "page.goto(\"/#/components/empty-state\")",
        "body:not(:has(#boot))",
        "[data-component=\"empty-state\"]",
        "[data-slot=\"empty-state\"][data-ui-schema=\"ui-empty-state-agent-contract\"]",
        "data-ui-output-status=\"validated\"",
        "data-ui-render-mode=\"snapshot\"",
        "data-ui-fallback=\"snapshot\"",
        "data-motion-source=\"default\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "empty-state e2e contract should contain semantic/settled marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child",
        "getByText(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "empty-state e2e should avoid fragile/non-semantic waiting selector `{forbidden}`."
        );
    }

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "empty_state_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 e2e selector stability evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_e2e_key_flow_is_repeatable_and_contract_breakpoints_are_addressable() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_empty_state_contract.spec.mjs");
    let view_source = load_component_source("view.rs");

    for required in [
        "docs-app empty-state flow is repeatable with semantic ready/settled breakpoints",
        "gotoEmptyStateDocsAndWaitSettled(page)",
        "assertSemanticReadySettledContracts(docsRoot)",
        "await page.reload();",
        "assertSemanticReadySettledContracts(reloadedRoot)",
    ] {
        assert!(
            e2e_source.contains(required),
            "empty-state e2e suite should keep repeatable key-flow marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-ui-render-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-fallback\", \"snapshot\")",
        "toHaveAttribute(\"role\", \"status\")",
        "toHaveAttribute(\"aria-live\", \"polite\")",
        "data-ui-output-status=\"validated\"",
        "data-motion-source=\"default\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "empty-state e2e flow should expose contract breakpoint `{required}` for failure diagnosis."
        );
    }

    for required in [
        "data-state=move || state.get().state.data_state_attr",
        "data-ui-render-mode=move || state.get().agent_contract.render_mode.as_attr()",
        "data-ui-fallback=move || state.get().agent_contract.fallback_mode.as_attr()",
        "data-ui-output-status=move || state.get().agent_contract.output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep semantic marker `{required}` used by repeatable e2e flow."
        );
    }

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "empty_state_e2e_key_flow_is_repeatable_and_contract_breakpoints_are_addressable",
        "高风险路径按适用范围为 N/A：`EmptyState` 为非交互展示组件",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 repeatable-flow evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let manifest_source = load_component_source("Component.toml");
    let rbi_source = load_component_source("empty_state.rbi");
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");

    for required in [
        "schema_version = \"1\"",
        "name = \"EmptyState\"",
        "crate = \"ui-empty-state\"",
        "pub fn EmptyState(",
        "title: Option<String>",
        "description: Option<String>",
        "tone: EmptyStateTone",
        "align: EmptyStateAlign",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "motion: EmptyStateMotion",
    ] {
        assert!(
            manifest_source.contains(required) || rbi_source.contains(required),
            "empty-state should keep stable v1 contract token `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "deprecation_window",
        "schema_version = \"2\"",
        "contract.v2",
        "codemod_rule",
        "schema_registry_entry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty-state should not introduce breaking-upgrade migration token `{forbidden}` in this change."
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "empty_state_version_deprecation_migration_is_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 version-migration evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_view_macro_complexity_is_bounded_and_semantically_split() {
    let view_source = load_component_source("view.rs");
    let view_lines = view_source.lines().count();
    let view_macro_count = view_source.match_indices("view! {").count();

    assert!(
        view_lines <= 160,
        "view.rs should stay compact for wasm macro expansion cost; got {view_lines} lines."
    );
    assert!(
        (1..=4).contains(&view_macro_count),
        "view.rs should avoid giant single macro or macro sprawl; found {view_macro_count} `view!` blocks."
    );

    for required in [
        "fn render_icon_slot(icon: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "fn render_actions_slot(actions: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "data-slot=\"empty-state-icon\"",
        "data-slot=\"empty-state-actions\"",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep semantic sub-block split marker `{required}`."
        );
    }

    for forbidden in ["for index in", ".enumerate()", "match index", "if index"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid index-driven structural assembly inside `view!` macro `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_component_source("view.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "fn render_icon_slot(icon: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "fn render_actions_slot(actions: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "{render_icon_slot(icon)}",
        "{render_actions_slot(actions)}",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep functional split helper marker `{required}`."
        );
    }

    let component_count = view_source.match_indices("#[component]").count();
    assert_eq!(
        component_count, 1,
        "view.rs should expose exactly one top-level component, found {component_count}."
    );

    for forbidden in [
        "#[component]\nfn render_icon_slot(",
        "#[component]\nfn render_actions_slot(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "lightweight render helpers must remain plain functions, not components `{forbidden}`."
        );
    }

    for required in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "render_icon_slot/render_actions_slot",
        "empty_state_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 functional-split evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_static_fragments_are_constantized_or_absent() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "fn render_icon_slot(icon: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "fn render_actions_slot(actions: Option<StoredValue<ViewFn>>) -> impl IntoView {",
        "data-slot=\"empty-state-icon\"",
        "data-slot=\"empty-state-actions\"",
        "data-slot=\"empty-state-title\"",
        "data-slot=\"empty-state-description\"",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep static fragment template marker `{required}`."
        );
    }

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html=",
        "Nothing to show",
        "Try adjusting filters",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid scattered heavy/static copy fragment `{forbidden}`."
        );
    }

    for required in [
        "pub struct EmptyStateStrings",
        "impl Default for EmptyStateStrings",
        "default_title: Cow::Borrowed(DEFAULT_TITLE),",
        "default_description: Cow::Borrowed(DEFAULT_DESCRIPTION),",
        "default_aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL),",
    ] {
        assert!(
            logic_source.contains(required),
            "default static copy should stay centralized in logic constants via `{required}`."
        );
    }

    for required in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A：`EmptyState` 当前无复杂 SVG/页脚/长静态说明",
        "empty_state_static_fragments_are_constantized_or_absent",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 static-fragment evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_inner_html_contract_stays_safe_and_semantic() {
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let styles_source = load_component_source("styles.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        ".set_inner_html(",
        "dangerously_set_inner_html",
        "web_sys::Element::set_inner_html",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "empty-state should forbid untrusted html injection path `{forbidden}`."
        );
    }

    for forbidden in [
        "remote_html",
        "user_html",
        "raw_html",
        "unsafe_html",
        "from_server_html",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "empty-state should not accept dynamic html payload marker `{forbidden}`."
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`EmptyState` 当前无 `inner_html` 注入节点",
        "empty_state_inner_html_contract_stays_safe_and_semantic",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 inner_html security evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_platform_paths_are_cfg_managed_and_non_wasm_safe() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml"))
        .unwrap_or_else(|e| panic!("read_to_string failed for component Cargo.toml: {e}"));
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for required in [
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "web-sys = { version = \"0.3.85\", features = [\"HtmlElement\"] }",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            cargo_toml.contains(required) || motion_source.contains(required),
            "cross-platform contract should keep explicit cfg/dependency boundary `{required}`."
        );
    }

    for forbidden in ["web_sys::", "js_sys::", "wasm_bindgen::JsCast"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "non-wasm component paths should not reference browser objects `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_respects_ui_headless_web_ssr_mutual_exclusion_contract() {
    let empty_state_cargo =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .unwrap_or_else(|e| panic!("read_to_string failed for empty-state Cargo.toml: {e}"));
    let headless_cargo = load_workspace_source("crates/ui-headless/Cargo.toml");
    let headless_lib = load_workspace_source("crates/ui-headless/src/lib.rs");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_lib.contains(required) || headless_cargo.contains(required),
            "ui-headless web/ssr mutual-exclusion contract marker missing `{required}`."
        );
    }

    {
        let required = "ui-headless = { path = \"../../crates/ui-headless\" }";
        assert!(
            empty_state_cargo.contains(required),
            "empty-state should depend on shared ui-headless contract via `{required}`."
        );
    }

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"] }",
        "features = [\"ssr\", \"web\"]",
    ] {
        assert!(
            !empty_state_cargo.contains(forbidden),
            "component dependency should not break ui-headless web/ssr mutual exclusion `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_defaults_are_not_derived_in_view() {
    let source = load_component_source("view.rs");

    for forbidden in [
        "logic::normalize_title(",
        "logic::normalize_description(",
        "logic::normalize_aria_label(",
        "logic::resolve_state(EmptyStateStateInput {",
        "if motion == EmptyStateMotion::default()",
    ] {
        assert!(
            !source.contains(forbidden),
            "default normalization should live in logic.rs, found `{forbidden}` in view.rs."
        );
    }
}

#[test]
fn empty_state_bool_props_use_is_prefix() {
    let source = load_component_source("view.rs");

    for required in [
        "#[prop(optional)] is_compact: bool",
        "#[prop(optional)] is_bordered: bool",
    ] {
        assert!(
            source.contains(required),
            "boolean prop naming should follow is_* contract; missing `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] compact: bool",
        "#[prop(optional)] bordered: bool",
    ] {
        assert!(
            !source.contains(forbidden),
            "boolean prop naming should avoid legacy alias `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_discrete_props_use_enum_types() {
    let source = load_component_source("view.rs");

    for required in [
        "#[prop(optional)] tone: EmptyStateTone",
        "#[prop(optional)] align: EmptyStateAlign",
    ] {
        assert!(
            source.contains(required),
            "discrete API input should be typed enum; missing `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "align: Option<String>",
        "tone: String",
        "align: String",
        "tone: &'static str",
        "align: &'static str",
        "Option<bool>",
    ] {
        assert!(
            !source.contains(forbidden),
            "discrete API input should avoid string/free-form bool composition; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_api_does_not_require_manual_state_wiring() {
    let source = load_component_source("view.rs");

    for forbidden in [
        "#[prop(into)] state:",
        "#[prop(optional, into)] state:",
        "state: EmptyStateState",
        "state: EmptyStateStateInput",
        "state: RwSignal",
        "state: ReadSignal",
        "state: WriteSignal",
    ] {
        assert!(
            !source.contains(forbidden),
            "EmptyState default API should not require manual state wiring; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_does_not_expose_parallel_collection_api_sugar() {
    let source = load_component_source("view.rs");

    for forbidden in [
        "ItemSpec",
        "items: Vec<",
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "#[prop(optional, into)] items:",
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional, into)] panels:",
    ] {
        assert!(
            !source.contains(forbidden),
            "EmptyState is not a parent/item composition component; found unsupported API sugar `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_logic_consumes_primitives_without_store_coupling() {
    let source = load_component_source("logic.rs");

    for required in [
        "pub use ui_state_primitives::empty_state::{",
        "resolve_state(EmptyStateStateInput {",
    ] {
        assert!(
            source.contains(required),
            "logic.rs should consume shared state primitives; missing `{required}`."
        );
    }

    for forbidden in [
        "leptos::",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "Signal<",
        "create_signal",
        "create_rw_signal",
        "use_context",
        "web_sys",
        "wasm_bindgen",
        "tokio::",
    ] {
        assert!(
            !source.contains(forbidden),
            "logic.rs should not bind framework/business store details; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_async_interaction_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "tokio::",
        "async fn",
        ".await",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "EmptyState has no async interaction contract; found `{forbidden}` in component source."
        );
    }
}

#[test]
fn empty_state_wasm_debug_requirements_are_not_applicable_and_do_not_leak_to_public_api() {
    let cargo_source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .unwrap_or_else(|e| panic!("read_to_string failed for empty-state Cargo.toml: {e}"));
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "[features]",
        "default = []",
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A：`EmptyState` 为非交互展示组件",
        "empty_state_wasm_debug_requirements_are_not_applicable_and_do_not_leak_to_public_api",
    ] {
        assert!(
            cargo_source.contains(required) || check2_source.contains(required),
            "wasm debug governance evidence should include `{required}`."
        );
    }

    for forbidden in [
        "pub fn debug_",
        "pub struct Debug",
        "pub enum Debug",
        "pub type Debug",
        "pub use debug",
        "pub mod debug",
        "feature = \"debug\"",
        "feature = \"devtools\"",
        "TraceReplay",
        "EventReplay",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "empty-state public surface should not leak wasm debug marker `{forbidden}`."
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:pointerdown",
        "on:pointerup",
        "on:pointermove",
        "on:click",
        "TraceId",
        "Replay",
        "timeline",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "empty-state should stay non-interactive without replay chain marker `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_dx_hot_reload_and_workbench_requirements_are_scoped_to_tooling() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_dev_script_source = load_workspace_source("scripts/dev-docs-app.sh");
    let web_dev_script_source = load_workspace_source("scripts/dev-web-demo.sh");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "pub(super) fn empty_state() -> AnyView {",
        "<Playground title=\"Hello World (Default Path)\" code_signal=hello_code>",
        "<Playground\n                title=\"State Matrix\"",
        "<Playground title=\"Tone + Alignment + Actions\" code_signal=tone_code>",
        "<Playground title=\"Compact + Bordered + Custom Class\" code_signal=state_code>",
        "<Playground\n                title=\"Controlled vs Uncontrolled (N/A)\"",
        "<Playground\n                title=\"Streaming Optional / Snapshot\"",
        "<Playground\n                title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=empty_state_imports.clone()",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs page should keep isolated empty-state playground entry `{required}`."
        );
    }

    for required in [
        "cd \"$ROOT_DIR/apps/docs-app\"",
        "cd \"$ROOT_DIR/apps/web-demo\"",
        "exec trunk serve --open true \"$@\"",
    ] {
        assert!(
            docs_dev_script_source.contains(required) || web_dev_script_source.contains(required),
            "dx dev scripts should keep fast trunk entry marker `{required}`."
        );
    }

    for forbidden in [
        "local_storage",
        "session_storage",
        "workbench",
        "Replay",
        "TraceId",
        "on:keydown",
        "on:pointerdown",
        "on:click",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "empty-state should remain non-interactive and avoid local state-persistence protocol `{forbidden}`."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "N/A：`EmptyState` 为非复杂交互展示组件",
        "empty_state_dx_hot_reload_and_workbench_requirements_are_scoped_to_tooling",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 DX governance evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_docs_are_copy_paste_ready_with_state_matrix_and_mode_sections() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");

    for required in [
        "pub(super) fn empty_state() -> AnyView {",
        "let empty_state_imports =",
        "<Playground title=\"Hello World (Default Path)\" code_signal=hello_code>",
        "<Playground\n                title=\"State Matrix\"",
        "<Playground\n                title=\"Controlled vs Uncontrolled (N/A)\"",
        "<Playground\n                title=\"Streaming Optional / Snapshot\"",
        "<Playground\n                title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=empty_state_imports.clone()",
        "code_imports=empty_state_imports",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs page should keep copy-paste-ready EmptyState marker `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "missing_import_lines",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground runtime should keep import auto-injection marker `{required}`."
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "empty_state_docs_are_copy_paste_ready_with_state_matrix_and_mode_sections",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 docs-product evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_source_first_docs_keep_copy_button_and_real_source_dependency_hints() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"empty-state-source-first-contract\"",
        "data-slot=\"empty-state-source-first-paths\"",
        "data-slot=\"empty-state-source-first-deps\"",
        "components/empty-state/src/mod.rs",
        "components/empty-state/src/logic.rs",
        "components/empty-state/src/view.rs",
        "components/empty-state/src/styles.rs",
        "components/empty-state/src/motion.rs",
        "component-empty_state",
        "inject-css",
        "UiRoot",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_page_source.contains(required),
            "empty_state source-first docs should keep marker `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "Show code",
        "Hide code",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground runtime should keep copy-ready marker `{required}`.",
        );
    }

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "empty_state_source_first_docs_keep_copy_button_and_real_source_dependency_hints",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 source-first evidence should include `{required}`.",
        );
    }
}

#[test]
fn empty_state_heroui_strategy_doc_and_component_docs_are_synced() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let strategy_source = load_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_workspace_source("apps/docs-app/src/pages/components/pages.rs");
    let readme_source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md"))
        .unwrap_or_else(|e| panic!("read_to_string failed for empty-state README.md: {e}"));

    for required in [
        "### EmptyState 同步记录（2026-02-20）",
        "`EmptyState` 维持 display primitive 定位",
        "component_doc!(\"EmptyState\", \"empty-state\", \"Display\", display_extra::empty_state)",
        "`#/components/empty-state` 可索引访问",
        "`apps/docs-app/src/pages/components/pages/display_extra.rs::empty_state()`",
        "`compose_copy_ready_code`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy should keep empty-state sync marker `{required}`."
        );
    }

    for required in [
        "component_doc!(\"EmptyState\", \"empty-state\", \"Display\", display_extra::empty_state)",
        "component_doc!(\"ErrorView\", \"error-view\", \"Display\", display_extra::error_view)",
    ] {
        assert!(
            docs_index_source.contains(required),
            "docs index should keep discoverable empty-state doc entry marker `{required}`."
        );
    }

    for required in [
        "## 文档入口",
        "docs-app: `/#/components/empty-state`",
        "页面源码：`apps/docs-app/src/pages/components/pages/display_extra.rs` 中 `empty_state()`",
    ] {
        assert!(
            readme_source.contains(required),
            "empty-state README should keep component-doc entry marker `{required}`."
        );
    }

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "empty_state_heroui_strategy_doc_and_component_docs_are_synced",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 heroui-doc-sync evidence should include `{required}`.",
        );
    }
}

#[test]
fn empty_state_docs_examples_and_matrices_stay_synced_with_logic_contract() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let logic_source = load_component_source("logic.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/empty_state.rs");

    for required in [
        "pub(super) fn empty_state() -> AnyView {",
        "<Playground title=\"Hello World (Default Path)\" code_signal=hello_code>",
        "<Playground\n                title=\"State Matrix\"",
        "<Playground title=\"Tone + Alignment + Actions\" code_signal=tone_code>",
        "<Playground title=\"Compact + Bordered + Custom Class\" code_signal=state_code>",
        "<Playground\n                title=\"Controlled vs Uncontrolled (N/A)\"",
        "<EmptyState />",
        "tone=EmptyStateTone::Muted",
        "align=EmptyStateAlign::Center",
        "tone=EmptyStateTone::Accent",
        "is_compact=true",
        "is_bordered=true",
        "class_name=\"docs-empty-state-custom\".to_string()",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs page should keep docs/examples/matrix marker `{required}` in sync.",
        );
    }

    for required in [
        "pub use ui_state_primitives::empty_state::{",
        "DEFAULT_TITLE",
        "DEFAULT_DESCRIPTION",
        "DEFAULT_ARIA_LABEL",
        "EmptyStateAlign",
        "EmptyStateTone",
        "pub fn resolve_defaults(",
        "pub fn resolve_render_state(input: EmptyStateRenderStateInput) -> EmptyStateRenderState",
    ] {
        assert!(
            logic_source.contains(required),
            "logic contract should still expose `{required}` for docs sync baseline.",
        );
    }

    for required in [
        "pub const DEFAULT_TITLE: &str = \"Nothing to show\";",
        "pub const DEFAULT_DESCRIPTION: &str = \"Try adjusting filters or refreshing data.\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Empty state\";",
        "pub enum EmptyStateTone",
        "pub enum EmptyStateAlign",
    ] {
        assert!(
            primitive_source.contains(required),
            "state primitive should keep default/API baseline `{required}` for docs sync.",
        );
    }

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "empty_state_docs_examples_and_matrices_stay_synced_with_logic_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 docs-sync evidence should include `{required}`.",
        );
    }
}

#[test]
fn empty_state_docs_interactive_playground_supports_live_preview_and_replayable_flow() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_empty_state_contract.spec.mjs");

    for required in [
        "title=\"Interactive Playground\"",
        "Interactive acceptance canvas: tune props/state and verify semantic markers in real time.",
        "let workbench_code = Signal::derive(move || {",
        "let workbench_actual_config = Signal::derive(move || {",
        "test_source_path=\"components/empty-state/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"empty-state-workbench-controls\"",
        "data-slot=\"empty-state-workbench-title\"",
        "data-slot=\"empty-state-workbench-description\"",
        "id_base=\"docs-empty-state-workbench-tone\".to_string()",
        "id_base=\"docs-empty-state-workbench-align\".to_string()",
        "data-slot=\"empty-state-workbench-toggle-compact\"",
        "data-slot=\"empty-state-workbench-toggle-bordered\"",
        "data-slot=\"empty-state-workbench-toggle-icon\"",
        "data-slot=\"empty-state-workbench-toggle-actions\"",
        "data-slot=\"empty-state-workbench-toggle-class\"",
        "data-slot=\"empty-state-workbench\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs interactive playground should keep marker `{required}`."
        );
    }

    for required in [
        "async function runInteractiveWorkbenchFlow(docsRoot) {",
        "docs-app empty-state interactive playground keeps live preview in sync",
        "docs-app empty-state flow is repeatable with semantic ready/settled breakpoints",
        "[data-slot=\"empty-state-workbench-controls\"]",
        "data-ui-output-status\", \"validated\"",
        "data-ui-render-mode\", \"snapshot\"",
        "data-ui-fallback\", \"snapshot\"",
        "data-state\", \"rich\"",
        "data-state\", \"plain\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "e2e interactive flow should keep contract breakpoint `{required}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "e2e interactive flow should avoid fragile wait marker `{forbidden}`."
        );
    }

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "empty_state_docs_interactive_playground_supports_live_preview_and_replayable_flow",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 interactive-playground evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_docs_readme_is_beginner_friendly_and_default_path_first() {
    let check2_source = load_workspace_source("components/empty-state/check2.md");
    let docs_page_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md"))
        .unwrap_or_else(|e| panic!("read_to_string failed for empty-state README.md: {e}"));

    for required in [
        "# EmptyState",
        "## 文档入口",
        "docs-app: `/#/components/empty-state`",
        "## 先用起来（Hello World）",
        "不需要先理解分层架构，先用默认 API：",
        "use ui_components::EmptyState;",
        "<EmptyState />",
        "## 常见用法（默认路径优先）",
        "tone=EmptyStateTone::Accent",
        "align=EmptyStateAlign::Center",
        "is_compact=true",
        "is_bordered=true",
        "## 进阶（需要时再看）",
        "### Architecture Layers",
        "### API (Quick Reference)",
    ] {
        assert!(
            readme_source.contains(required),
            "empty-state README should include beginner-friendly marker `{required}`."
        );
    }

    let hello_index = readme_source
        .find("## 先用起来（Hello World）")
        .expect("README should contain hello-world section");
    let common_index = readme_source
        .find("## 常见用法（默认路径优先）")
        .expect("README should contain common-usage section");
    let advanced_index = readme_source
        .find("## 进阶（需要时再看）")
        .expect("README should contain advanced section");

    assert!(
        hello_index < common_index && common_index < advanced_index,
        "README should keep default API path before advanced content."
    );

    for required in [
        "pub(super) fn empty_state() -> AnyView {",
        "title=\"EmptyState\"",
        "slug=\"empty-state\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "docs-app should keep discoverable empty-state doc entry marker `{required}`."
        );
    }

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "empty_state_docs_readme_is_beginner_friendly_and_default_path_first",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 docs-product evidence should include `{required}`."
        );
    }
}

#[test]
fn empty_state_engineering_capabilities_are_unified_without_runtime_or_serde_leakage() {
    let cargo_source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .unwrap_or_else(|e| panic!("read_to_string failed for empty-state Cargo.toml: {e}"));
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let check2_source = load_workspace_source("components/empty-state/check2.md");

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`EmptyState` 无 spec/config 序列化输入与异步边界",
        "empty_state_engineering_capabilities_are_unified_without_runtime_or_serde_leakage",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 engineering-governance evidence should include `{required}`."
        );
    }

    for forbidden in ["serde", "tracing", "tokio", "async-std", "async_std"] {
        assert!(
            !cargo_source.contains(forbidden),
            "component manifest should not pull runtime/serde/tracing dependency `{forbidden}`."
        );
    }

    for forbidden in [
        "serde::",
        "Serialize",
        "Deserialize",
        "tracing::",
        "span!(",
        "event!(",
        "#[instrument",
        "tokio::",
        "async_std::",
        "async fn",
        ".await",
        "Runtime",
        "JoinHandle",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "empty-state source should avoid runtime/serde/tracing leakage marker `{forbidden}`."
        );
    }

    for required in [
        "EmptyStateStrings",
        "pub use view::EmptyState;",
        "pub use motion::EmptyStateMotion;",
    ] {
        assert!(
            mod_source.contains(required),
            "public API should stay UI-contract focused via `{required}`."
        );
    }
}

#[test]
fn empty_state_has_no_macro_micro_dragging_state_machine() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on_drag",
        "on:drag",
        "dragstart",
        "dragend",
        "pointermove",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement drag macro/micro state machine; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_two_pass_geometry_rendering_pipeline() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
        "Popover",
        "Tooltip",
        "Menu",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement two-pass geometry rendering; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_registration_protocol_for_dynamic_items() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "EmptyState should not implement dynamic-item registration protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_slot_projection_strategy_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot projection",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement slot projection strategy protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_environment_stream_subscription_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "on_resize",
        "on:resize",
        "matchMedia",
        "debounce",
        "throttle",
        "ThemeChanged",
        "IntersectionChanged",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "EmptyState should not implement env-stream subscription protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_event_light_cone_batch_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "select_all",
        "batch_select",
        "Table",
        "Grid",
        "prop drilling",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "EmptyState should not implement event-light-cone batch protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_causality_bus_trace_propagation_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality bus",
        "bus.broadcast",
        "broadcast(",
        "subscribe(",
        "Subscriber",
        "derived command",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement causality-bus trace propagation protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_focus_stack_overlay_restore_protocol() {
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "FocusManager",
        "FocusStack",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "overlay stack",
        "document.body",
        "focus_gc",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement overlay focus-stack restore protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_foreign_zone_escape_hatch_protocol() {
    let mod_source = load_component_source("mod.rs");
    let view_source = load_component_source("view.rs");
    let logic_source = load_component_source("logic.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "google.maps",
        "Leaflet",
        "mapbox",
        "third-party instance",
        "imperative instance",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should not implement foreign-zone escape hatch protocol; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_has_no_hydration_discontinuity_seed_usage() {
    let mod_source = load_component_source("mod.rs");
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");

    for forbidden in [
        "now()",
        "Date::now",
        "js_sys::Date",
        "SystemTime::now",
        "UNIX_EPOCH",
        "Uuid::",
        "uuid::",
        "rand::",
        "thread_rng",
        "getrandom",
        "nanoid",
        "ulid::",
        "create_id(",
        "use_id(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "EmptyState should avoid hydration-unstable seed/id initialization `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_public_api_does_not_leak_dom_types() {
    let source = load_component_source("mod.rs");

    for forbidden in ["web_sys", "wasm_bindgen", "NodeRef", "HtmlElement"] {
        assert!(
            !source.contains(forbidden),
            "public API surface should not leak DOM detail `{forbidden}`."
        );
    }
}
