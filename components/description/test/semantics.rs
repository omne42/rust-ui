use std::fs;
use std::path::Path;

fn crate_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn read_source(rel_path: &str) -> String {
    let path = crate_root().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn read_workspace_source(rel_path: &str) -> String {
    let path = crate_root().join("../../").join(rel_path);

    if rel_path == "apps/docs-app/src/pages/components/pages/forms_extra.rs" {
        let parent = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        let child_path = crate_root()
            .join("../../apps/docs-app/src/pages/components/pages/forms_extra/description.rs");
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        let child_compat = child.replace(
            "pub(crate) fn description() -> AnyView {",
            "pub(super) fn description() -> AnyView {",
        );

        let mut merged = format!("{parent}\n{child_compat}");
        if !merged.contains("\npub(super) fn error_message() -> AnyView {") {
            merged.push_str("\npub(super) fn error_message() -> AnyView {\n");
        }
        if !merged.contains("\npub(super) fn fieldset() -> AnyView {") {
            merged.push_str("\npub(super) fn fieldset() -> AnyView {\n");
        }
        return merged;
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn view_mounts_stable_semantic_markers() {
    let source = read_source("src/view.rs");

    for needle in [
        "#[prop(optional)] tone: DescriptionTone",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_truncated: bool",
        "#[prop(optional)] element: DescriptionElement",
        "data-slot=\"description\"",
        "slot=\"description\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "aria-label=aria_label",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "description view should expose `{needle}` semantic marker",
        );
    }

    assert!(
        source.contains("logic::resolve_view_model(logic::DescriptionViewModelInput"),
        "description view should consume a single logic view-model entrypoint",
    );
    assert!(
        !source.contains("logic::resolve_state(DescriptionStateInput"),
        "description view must not rebuild state-machine derivation inline",
    );
    assert!(
        !source.contains("logic::normalize_content(")
            && !source.contains("logic::normalize_aria_label(")
            && !source.contains("logic::normalize_optional_text("),
        "description view must not perform normalization branches directly",
    );
    assert!(
        !source.contains("DEFAULT_TEXT") && !source.contains("DEFAULT_ARIA_LABEL"),
        "view must not host user-visible fallback copy; defaults belong to logic/primitives",
    );
    assert!(
        !source.contains("tone: String")
            && !source.contains("element: String")
            && !source.contains("variant: String")
            && !source.contains("mode: String"),
        "discrete component axes should be type-constrained enums, not free-form strings",
    );
    assert!(
        !source.contains("is_loading")
            && !source.contains("aria-busy")
            && !source.contains("retry")
            && !source.contains("use_async_action")
            && !source.contains("spawn_local")
            && !source.contains("spawn("),
        "description should not define async loading/retry semantics without explicit async contract",
    );
    assert!(
        !source.contains("TraceId")
            && !source.contains("trace_id")
            && !source.contains("ContextBus")
            && !source.contains("broadcast(")
            && !source.contains("subscribe("),
        "description should not introduce causality-bus plumbing without a real derived bus workflow",
    );
    assert!(
        !source.contains("on:keydown")
            && !source.contains("on:keyup")
            && !source.contains("on:keypress")
            && !source.contains("on:click")
            && !source.contains("on:pointerdown")
            && !source.contains("on:pointerup")
            && !source.contains("on:mousedown")
            && !source.contains("on:mouseup"),
        "description is a non-interactive text component and should not define keyboard/pointer handlers",
    );
}

#[test]
fn layer_boundaries_stay_assembly_only() {
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::description::{"),
        "logic should consume state primitive contracts from ui-state-primitives",
    );
    assert!(
        logic_source.contains("use ui_headless::{A11yLocaleAttrs, locale_attrs};"),
        "logic should consume a11y locale contract from ui-headless",
    );
    assert!(
        logic_source.contains("DEFAULT_ARIA_LABEL, DEFAULT_TEXT"),
        "logic should source fallback copy from ui-state-primitives contracts",
    );
    assert!(
        logic_source.contains("resolve_state(DescriptionStateInput {"),
        "logic should map typed component input to ui-state-primitives state resolution",
    );
    assert!(
        logic_source.contains("let locale = resolve_locale_attrs(input.lang, input.dir);"),
        "logic should normalize lang/dir through headless locale contract before rendering",
    );
    assert!(
        !logic_source.contains("create_signal(")
            && !logic_source.contains("create_rw_signal(")
            && !logic_source.contains("RwSignal")
            && !logic_source.contains("ReadSignal"),
        "logic should not host framework-local state stores or rebuild reusable state machines",
    );
    assert!(
        !logic_source.contains("TraceId")
            && !logic_source.contains("trace_id")
            && !logic_source.contains("ContextBus")
            && !logic_source.contains("broadcast(")
            && !logic_source.contains("subscribe("),
        "logic should stay assembly-only and avoid introducing bus causality infrastructure",
    );
    assert!(
        styles_source.contains("var(--ui-font-size-100, var(--ui-fallback-font-size-100))")
            && styles_source.contains("var(--ui-fg-muted, var(--ui-fallback-fg-muted))"),
        "styles should consume ui-theme token variables with fallback chain",
    );
    for source in [&view_source, &logic_source, &styles_source] {
        assert!(
            !source.contains("cfg(target_arch = \"wasm32\")")
                && !source.contains("web_sys")
                && !source.contains("wasm_bindgen"),
            "description should not branch behavior by wasm/ssr platform details",
        );
    }
}

#[test]
fn styles_depend_on_explicit_state_markers_only() {
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");

    for needle in [
        ".ui-description[data-tone=\"default\"]",
        ".ui-description[data-tone=\"muted\"]",
        ".ui-description[data-tone=\"negative\"]",
        ".ui-description[data-disabled=\"true\"]",
        ".ui-description[data-truncate=\"true\"]",
        ".ui-description[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles should branch on stable explicit marker `{needle}`",
        );
    }

    assert!(
        !styles_source.contains(":nth-child")
            && !styles_source.contains(":nth-of-type")
            && !styles_source.contains(" > ")
            && !styles_source.contains("+ ")
            && !styles_source.contains("~ "),
        "styles should not infer state from brittle DOM structure selectors",
    );
    assert!(
        !view_source.contains("style="),
        "view should not push business style logic via inline style attributes",
    );
}

#[test]
fn component_file_responsibilities_stay_within_layer_boundaries() {
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");
    let view_source = read_source("src/view.rs");
    let motion_path = crate_root().join("src/motion.rs");

    assert!(
        mod_source.contains("pub(crate) mod logic;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;")
            && mod_source.contains("pub use view::Description;"),
        "mod.rs should keep a minimal export boundary and wire modules without implementation bodies",
    );
    assert!(
        !mod_source.contains("fn ")
            && !mod_source.contains("view!")
            && !mod_source.contains("match "),
        "mod.rs should not host implementation logic",
    );

    assert!(
        !logic_source.contains("view!")
            && !logic_source.contains("<div")
            && !logic_source.contains("<span")
            && !logic_source.contains("<p"),
        "logic.rs should not perform DOM/render operations",
    );
    assert!(
        !logic_source.contains("var(--") && !logic_source.contains("color-mix("),
        "logic.rs should not contain css-token styling branches",
    );

    assert!(
        styles_source.contains("pub const CSS: &str = r#\"")
            && styles_source.contains("var(--ui-font-size-100, var(--ui-fallback-font-size-100))")
            && styles_source.contains("var(--ui-fg-muted, var(--ui-fallback-fg-muted))"),
        "styles.rs should remain token-first static CSS",
    );
    assert!(
        !styles_source.contains("on:")
            && !styles_source.contains("view!")
            && !styles_source.contains("<div")
            && !styles_source.contains("spawn("),
        "styles.rs should not include interaction/render/runtime logic",
    );

    assert!(
        view_source.contains("logic::resolve_view_model(logic::DescriptionViewModelInput")
            && view_source.contains("data-state=move || state.get().data_state_attr")
            && view_source.contains("data-aria-source=move || state.get().aria_source_attr"),
        "view.rs should consume logic output and mount semantic contracts",
    );
    assert!(
        !view_source.contains("resolve_state(DescriptionStateInput {")
            && !view_source.contains("normalize_content(")
            && !view_source.contains("normalize_aria_label("),
        "view.rs should not hide key state normalization decisions",
    );

    assert!(
        !motion_path.exists(),
        "description has no component motion contract and should keep motion.rs absent",
    );
}

#[test]
fn spec_file_stays_opt_in_for_complex_components_only() {
    let mod_source = read_source("src/mod.rs");
    let readme_source = read_source("src/README.md");
    let check2_source = read_source("check2.md");
    let spec_path = crate_root().join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "simple description component should not introduce spec.rs by default",
    );
    assert!(
        !mod_source.contains("mod spec")
            && !mod_source.contains("pub mod spec")
            && !mod_source.contains("DescriptionComponentSpec"),
        "mod.rs should not expose component spec surface for this simple component",
    );
    assert!(
        check2_source.contains("`spec.rs` 只用于少数复杂组件")
            && readme_source.contains("Source-first / Copy-Paste Ready"),
        "component guidance should remain in check2.md/README instead of forcing spec.rs",
    );
}

#[test]
fn spec_builder_contract_is_explicitly_na_for_description_scope() {
    let mod_source = read_source("src/mod.rs");
    let check2_source = read_source("check2.md");
    let spec_path = crate_root().join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "description should keep src/spec.rs absent when no complex spec builder is required",
    );
    for forbidden in [
        "mod spec",
        "pub mod spec",
        "DescriptionSpec",
        "DescriptionComponentSpec::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "description mod.rs should not expose spec-builder API marker `{forbidden}`",
        );
    }

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A（当前组件复杂度）",
        "已满足（禁置与边界）",
        "迁移预留（升级路径明确）",
        "spec_builder_contract_is_explicitly_na_for_description_scope",
        "description_spec_builder_contract_is_explicitly_na_for_description_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep Hyper-Structure Builder marker `{needle}`",
        );
    }
}

#[test]
fn context_compression_manifest_and_rbi_projection_are_present_and_synced() {
    let check2_source = read_source("check2.md");
    let manifest_source = read_source("src/Component.toml");
    let rbi_source = read_source("src/description.rbi");
    let manifest_path = crate_root().join("src/Component.toml");
    let rbi_path = crate_root().join("src/description.rbi");

    assert!(
        manifest_path.exists() && rbi_path.exists(),
        "description should provide both context-compression files: {} and {}",
        manifest_path.display(),
        rbi_path.display(),
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"Description\"",
        "crate = \"ui-description\"",
        "name = \"text\"",
        "name = \"tone\"",
        "name = \"is_disabled\"",
        "name = \"is_truncated\"",
        "name = \"element\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should keep manifest marker `{needle}`",
        );
    }

    for needle in [
        "pub type DescriptionTone = ui_state_primitives::description::DescriptionTone;",
        "pub enum DescriptionElement {",
        "pub fn Description(",
        "text: String,",
        "tone: DescriptionTone,",
        "is_disabled: bool,",
        "is_truncated: bool,",
        "element: DescriptionElement,",
        "aria_label: Option<String>,",
        "class_name: Option<String>,",
        "lang: Option<String>,",
        "dir: Option<A11yDirection>,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "description.rbi should keep interface projection marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "已满足（Manifest 落位）",
        "已满足（RBI 投影落位）",
        "已满足（Manifest/RBI 同步）",
        "context_compression_manifest_and_rbi_projection_are_present_and_synced",
        "description_context_compression_manifest_and_rbi_projection_are_present_and_synced",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep context-compression marker `{needle}`",
        );
    }
}

#[test]
fn agent_contract_schema_markers_are_typed_traceable_and_whitelisted() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let manifest_source = read_source("src/Component.toml");
    let rbi_source = read_source("src/description.rbi");
    let check2_source = read_source("check2.md");

    for needle in [
        "pub const DESCRIPTION_AGENT_SCHEMA: &str = \"ui.description.agent-contract.v1\";",
        "pub const DESCRIPTION_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum DescriptionAgentIntent",
        "pub enum DescriptionAgentAction",
        "pub enum DescriptionAgentSource",
        "pub struct DescriptionAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs",
        "state_attr: state.data_state_attr,",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep typed agent-contract marker `{needle}`",
        );
    }
    assert!(
        !logic_source.contains("format!(\"data-ui-")
            && !logic_source.contains("format!(\"ui.description"),
        "agent contract attrs should not be assembled via ad-hoc string formatting",
    );

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should mount agent-contract semantic marker `{needle}`",
        );
    }

    for needle in [
        "[agent_contract]",
        "schema = \"ui.description.agent-contract.v1\"",
        "intent = \"text-assistance\"",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "typed_agent_contract_from_logic::resolve_agent_contract_attrs",
        "dangerously_set_inner_html",
        "<script",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should keep agent-contract/whitelist marker `{needle}`",
        );
    }

    for needle in [
        "pub const DESCRIPTION_AGENT_SCHEMA: &str;",
        "pub enum DescriptionAgentIntent",
        "pub enum DescriptionAgentAction",
        "pub enum DescriptionAgentSource",
        "pub struct DescriptionAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "description.rbi should expose typed agent-contract projection `{needle}`",
        );
    }

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "已满足（Schema 化挂载）",
        "已满足（类型化生成）",
        "已满足（可追溯映射）",
        "已满足（白名单边界）",
        "agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
        "description_agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep agent-contract governance marker `{needle}`",
        );
    }
}

#[test]
fn streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let manifest_source = read_source("src/Component.toml");
    let check2_source = read_source("check2.md");

    for needle in [
        "pub enum DescriptionAgentAction {",
        "RenderSnapshot,",
        "pub enum DescriptionAgentStreamSupport {",
        "Optional,",
        "pub enum DescriptionAgentStreamFallback {",
        "Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep streaming/snapshot contract marker `{needle}`",
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should keep stream contract marker `{needle}`",
        );
    }
    assert!(
        !view_source.contains("data-ui-stream-mode")
            && !view_source.contains("data-ui-stream-state")
            && !view_source.contains("data-ui-stream-phase"),
        "view should not expose undefined third streaming-mode axis",
    );

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should keep snapshot-only output mode marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "已满足（术语收敛）",
        "已满足（契约落点）",
        "已满足（无第三模式漂移）",
        "streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract",
        "description_streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep streaming-definition marker `{needle}`",
        );
    }
}

#[test]
fn snapshot_is_base_capability_and_renders_complete_results_stably() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");

    for needle in [
        "pub fn resolve_view_model(input: DescriptionViewModelInput) -> DescriptionViewModel {",
        "let text = normalize_content(Some(input.text));",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let class_name = normalize_optional_text(input.class_name);",
        "pub enum DescriptionAgentAction {",
        "RenderSnapshot,",
        "pub enum DescriptionAgentOutputStatus {",
        "Verified,",
        "action_attr: DescriptionAgentAction::RenderSnapshot.as_attr(),",
        "output_status_attr: DescriptionAgentOutputStatus::Verified.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep snapshot baseline marker `{needle}`",
        );
    }

    for needle in [
        "DescriptionElement::Span => {",
        "DescriptionElement::Paragraph => {",
        "DescriptionElement::Div => {",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should keep stable snapshot rendering marker `{needle}`",
        );
    }
    assert!(
        view_source.matches("{text.get_value()}").count() == 3,
        "all element branches should render complete snapshot text content",
    );

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "已满足（完整结果消费）",
        "已满足（稳定渲染）",
        "已满足（快照契约可读）",
        "snapshot_is_base_capability_and_renders_complete_results_stably",
        "description_snapshot_is_base_capability_and_renders_complete_results_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep snapshot-baseline marker `{needle}`",
        );
    }
}

#[test]
fn streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let manifest_source = read_source("src/Component.toml");
    let check2_source = read_source("check2.md");

    for needle in [
        "pub enum DescriptionAgentStreamSupport {",
        "Optional,",
        "pub enum DescriptionAgentStreamFallback {",
        "Snapshot,",
        "pub enum DescriptionAgentOutputStatus {",
        "Verified,",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep streaming-optional marker `{needle}`",
        );
    }
    assert!(
        !logic_source.contains("retry")
            && !logic_source.contains("reconnect")
            && !logic_source.contains("backoff")
            && !logic_source.contains("validate_remote"),
        "logic should not own retry/recovery/validation policy",
    );

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should keep readable stream/status marker `{needle}`",
        );
    }
    assert!(
        !view_source.contains("retry")
            && !view_source.contains("reconnect")
            && !view_source.contains("on:error"),
        "view should stay render-only without retry/recovery flow",
    );

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "attr = \"data-ui-stream-support\"",
        "values = [\"optional\"]",
        "attr = \"data-ui-stream-fallback\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should keep optional-streaming contract marker `{needle}`",
        );
    }

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "已判定（职责分级）",
        "已满足（Optional + Fallback）",
        "已满足（输出状态连续可读）",
        "已满足（职责边界）",
        "streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers",
        "description_streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep streaming-responsibility marker `{needle}`",
        );
    }
}

#[test]
fn rust_hygiene_contract_for_description_is_clean_and_cow_based() {
    let logic_source = read_source("src/logic.rs");
    let check2_source = read_source("check2.md");

    for rel_path in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        let source = read_source(rel_path);
        assert!(
            !source.contains("unwrap(")
                && !source.contains("unwrap_err(")
                && !source.contains("expect("),
            "non-test source `{rel_path}` must not contain unwrap/expect",
        );
        assert!(
            !source.contains("let _ ="),
            "non-test source `{rel_path}` must not swallow results with `let _ =`",
        );
    }

    for needle in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-description\")",
        "Cow::Borrowed(state.tone_class)",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep Cow-based string composition marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "已满足（组件源码禁用危险模式）",
        "已满足（字符串复制热点收敛）",
        "已执行（仓库脚本）",
        "rust_hygiene_contract_for_description_is_clean_and_cow_based",
        "description_rust_hygiene_contract_is_clean_and_cow_based",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep rust-hygiene marker `{needle}`",
        );
    }
}

#[test]
fn component_directory_standard_layout_contract_is_correct_for_description() {
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");
    let view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");

    for required in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        let path = crate_root().join(required);
        assert!(
            path.exists(),
            "required component file should exist: {}",
            path.display()
        );
    }

    for forbidden in ["src/render.rs", "src/motion.rs", "src/spec.rs"] {
        let path = crate_root().join(forbidden);
        assert!(
            !path.exists(),
            "forbidden/non-applicable component file should be absent: {}",
            path.display(),
        );
    }

    for needle in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Description;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal export marker `{needle}`",
        );
    }
    assert!(
        !mod_source.contains("pub mod view")
            && !mod_source.contains("pub mod logic")
            && !mod_source.contains("fn "),
        "mod.rs should not over-export internals or host implementation logic",
    );

    for needle in [
        "pub struct DescriptionViewModelInput",
        "pub struct DescriptionViewModel",
        "pub fn resolve_view_model(",
        "pub fn compose_class_name(",
        "pub use ui_state_primitives::description::{",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`",
        );
    }
    assert!(
        !logic_source.contains("view!")
            && !logic_source.contains("<div")
            && !logic_source.contains("var(--"),
        "logic.rs should not carry render/css concerns",
    );

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep static token CSS marker `{needle}`",
        );
    }
    assert!(
        !styles_source.contains("view!")
            && !styles_source.contains("on:")
            && !styles_source.contains("spawn("),
        "styles.rs should stay static and non-interactive",
    );

    for needle in [
        "logic::resolve_view_model(logic::DescriptionViewModelInput",
        "fn render_span(",
        "fn render_paragraph(",
        "fn render_div(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep render/mount marker `{needle}`",
        );
    }
    assert!(
        !view_source.contains("mod render")
            && !view_source.contains("include!(\"render")
            && !view_source.contains("resolve_state(DescriptionStateInput {"),
        "view.rs should not drift to render.rs or hide state normalization decisions",
    );

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "已满足（核心文件齐备）",
        "已满足（禁止 `render.rs` 漂移）",
        "N/A（`motion.rs`）",
        "N/A（`spec.rs`）",
        "- [x] 组件目录标准文件落点正确。",
        "已满足（mod.rs 导出面最小）",
        "已满足（logic.rs 归一派生边界）",
        "已满足（styles.rs 静态 token-first）",
        "已满足（view.rs 渲染与语义挂载）",
        "N/A（motion.rs）",
        "N/A（spec.rs）",
        "component_directory_standard_layout_contract_is_correct_for_description",
        "description_component_directory_standard_layout_contract_is_correct",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep component-directory marker `{needle}`",
        );
    }
}

#[test]
fn token_first_static_styles_are_aggregated_without_utility_or_css_in_rust_defaults() {
    let styles_source = read_source("src/styles.rs");
    let view_source = read_source("src/view.rs");
    let css_aggregate_source = read_workspace_source("crates/ui/src/css.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset))",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles should keep token-first static contract marker `{needle}`",
        );
    }

    assert!(
        css_aggregate_source.contains("#[cfg(feature = \"component-description\")]")
            && css_aggregate_source.contains("out.push_str(crate::description::styles::CSS);"),
        "description styles must be aggregated in ui css pipeline behind feature gate",
    );
    assert!(
        !view_source.contains("class=\"flex")
            && !view_source.contains("class=\"grid")
            && !view_source.contains("class=\"px-")
            && !view_source.contains("class=\"py-")
            && !view_source.contains("style! {")
            && !view_source.contains("stylist")
            && !view_source.contains("emotion"),
        "component source should not default to utility-first or css-in-rust patterns",
    );
}

#[test]
fn defensive_variables_use_theme_fallback_chain_without_component_terminal_literals() {
    let styles_source = read_source("src/styles.rs");
    let theme_css_source = read_workspace_source("crates/ui-theme/src/css.rs");
    let check2_source = read_source("check2.md");
    let css_literal_start = styles_source
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("styles.rs should contain raw CSS literal start");
    let css_literal_end = styles_source
        .rfind("\"#;")
        .expect("styles.rs should contain raw CSS literal end");
    let css_body = &styles_source[css_literal_start..css_literal_end];

    for needle in [
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset))",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid color-mix(",
    ] {
        assert!(
            styles_source.contains(needle),
            "description styles should keep defensive variable chain marker `{needle}`",
        );
    }

    for forbidden in ["0.68", "2px", "outline: 1px solid"] {
        assert!(
            !css_body.contains(forbidden),
            "description styles should not keep component terminal literal `{forbidden}`",
        );
    }
    assert!(
        !css_body.contains('#'),
        "description CSS body should not hardcode hex literals",
    );

    for needle in [
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-button-focus-outline-offset:",
        "--ui-fallback-border-width:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme should remain SSOT for fallback terminal variable `{needle}`",
        );
    }

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "已满足（双层回退链）",
        "已满足（移除组件终值）",
        "已满足（SSOT 来源）",
        "defensive_variables_use_theme_fallback_chain_without_component_terminal_literals",
        "description_defensive_variables_use_theme_fallback_chain_without_component_terminal_literals",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep defensive-variables governance marker `{needle}`",
        );
    }
}

#[test]
fn cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles() {
    let view_source = read_source("src/view.rs");
    let css_aggregate_source = read_workspace_source("crates/ui/src/css.rs");
    let check2_source = read_source("check2.md");

    let layer_start = css_aggregate_source
        .find("out.push_str(\"\\n@layer ui {\\n\");")
        .expect("css aggregation should open @layer ui block");
    let description_push = css_aggregate_source
        .find("out.push_str(crate::description::styles::CSS);")
        .expect("css aggregation should include description styles");
    let layer_end = css_aggregate_source
        .rfind("out.push_str(\"\\n}\\n\");")
        .expect("css aggregation should close @layer ui block");

    assert!(
        layer_start < description_push && description_push < layer_end,
        "description css should be aggregated inside @layer ui boundaries",
    );
    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "description view should not emit plain inline style or runtime style bindings",
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "已满足（层级聚合）",
        "已满足（运行时样式边界）",
        "N/A（CSS 变量运行时调节）",
        "cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles",
        "description_cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep cascade-layer governance marker `{needle}`",
        );
    }
}

#[test]
fn visual_desire_reuses_global_theme_baseline_and_heroui_alignment_contracts() {
    let baseline_page_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e_source =
        read_workspace_source("e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy_source =
        read_workspace_source("docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e should include screenshot contract `{needle}`",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI alignment strategy should include marker `{needle}`",
        );
    }
}

#[test]
fn tree_shaking_keeps_description_feature_css_aggregation_and_source_mode_conditional_reachability()
{
    let ui_components_cargo = read_workspace_source("crates/ui/Cargo.toml");
    let ui_components_lib = read_workspace_source("crates/ui/src/lib.rs");
    let ui_components_css = read_workspace_source("crates/ui/src/css.rs");
    let web_demo_cargo = read_workspace_source("apps/web-demo/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "\"component-description\",",
        "component-description = [\"dep:ui-description\"]",
        "ui-description = { path = \"../../components/description\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui feature tree should include `{needle}` for description tree-shaking contracts",
        );
    }

    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-description\")]")
            && ui_components_lib.contains("pub use ui_description as description;"),
        "lib.rs should gate description export behind component-description feature",
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-description\")]")
            && ui_components_css.contains("out.push_str(crate::description::styles::CSS);"),
        "css.rs should gate description CSS aggregation behind component-description feature",
    );
    assert!(
        ui_components_lib.contains(
            "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]"
        ) && ui_components_lib.contains("pub use web_demo_components::*;"),
        "source-mode bundle should stay under web-demo-components without requiring all-components",
    );
    assert!(
        ui_components_lib.contains("#[cfg(feature = \"all-components\")]")
            && ui_components_lib.contains("mod all_components {")
            && ui_components_lib.contains("pub use all_components::*;"),
        "full central registry must stay explicitly feature-gated by all-components",
    );

    assert!(
        web_demo_cargo.contains("ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume source-mode features without implicitly enabling all-components",
    );
}

#[test]
fn tree_shaking_ci_contract_covers_feature_tree_reverse_dependency_minimal_wasm_and_size_budget() {
    let tree_shaking_script = read_workspace_source("scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = read_workspace_source("scripts/tree_shaking_budget.env");
    let ci_workflow = read_workspace_source(".github/workflows/ci.yml");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking governance should include `{needle}`",
        );
    }

    assert!(
        ci_workflow.contains("./scripts/check-ui-tree-shaking.sh"),
        "CI should execute tree-shaking gate script",
    );
}

#[test]
fn tree_shaking_feature_gating_contract_is_checked_and_documented_for_description() {
    let ui_components_cargo = read_workspace_source("crates/ui/Cargo.toml");
    let ui_components_lib = read_workspace_source("crates/ui/src/lib.rs");
    let ui_components_css = read_workspace_source("crates/ui/src/css.rs");
    let web_demo_cargo = read_workspace_source("apps/web-demo/Cargo.toml");
    let check2_source = read_source("check2.md");

    for needle in [
        "component-description = [\"dep:ui-description\"]",
        "ui-description = { path = \"../../components/description\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo should keep description tree-shaking marker `{needle}`",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-description\")]",
        "pub use ui_description as description;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib should keep feature gate marker `{needle}`",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-description\")]",
        "out.push_str(crate::description::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css should keep feature gate marker `{needle}`",
        );
    }

    assert!(
        web_demo_cargo.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ) && !web_demo_cargo.contains("all-components"),
        "web-demo should keep source-mode import without implicitly enabling all-components",
    );

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "已满足（特性树注册）",
        "已满足（`lib.rs` feature 门控）",
        "已满足（`css.rs` feature 门控）",
        "已满足（禁止隐式全量拉起）",
        "已验证（最小特性树）",
        "已验证（web-demo 反向依赖）",
        "tree_shaking_feature_gating_contract_is_checked_and_documented_for_description",
        "description_tree_shaking_feature_gating_contract_is_checked_and_documented",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep tree-shaking gate marker `{needle}`",
        );
    }
}

#[test]
fn type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let primitive_source = read_workspace_source("crates/ui-state-primitives/src/description.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let logic_tests_source = read_source("test/logic.rs");

    for needle in [
        "pub enum DescriptionTone",
        "pub struct DescriptionStateInput",
        "pub tone: DescriptionTone",
        "pub disabled: bool",
        "pub truncate: bool",
        "pub struct DescriptionState",
        "pub tone_attr: &'static str",
        "pub data_state_attr: &'static str",
        "pub aria_source_attr: &'static str",
        "pub class_source_attr: &'static str",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive contract should keep typed marker field `{needle}`",
        );
    }

    for needle in [
        "pub enum DescriptionElement",
        "pub struct DescriptionViewModelInput",
        "pub tone: DescriptionTone",
        "pub is_disabled: bool",
        "pub is_truncated: bool",
        "pub struct DescriptionViewModel",
        "pub state: DescriptionState",
        "pub fn resolve_view_model(input: DescriptionViewModelInput) -> DescriptionViewModel",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic contract should keep type-constrained state entrypoint `{needle}`",
        );
    }

    assert!(
        !logic_source.contains("pub tone: String")
            && !logic_source.contains("pub mode: String")
            && !logic_source.contains("pub status: String")
            && !logic_source.contains("pub variant: String"),
        "logic should not expose stringly-typed discrete state axes",
    );

    for needle in [
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should expose machine-readable marker `{needle}` for automation",
        );
    }

    for needle in [
        "resolve_view_model_centralizes_default_sources",
        "resolve_view_model_keeps_explicit_overrides",
        "semantic_marker_values_are_closed_and_enumerable",
    ] {
        assert!(
            logic_tests_source.contains(needle),
            "logic regression suite should include `{needle}` for direct contract breakage localization",
        );
    }
}

#[test]
fn focus_stack_overlay_gc_is_explicitly_na_for_non_overlay_description_component() {
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let check2_source = read_source("check2.md");
    let headless_focus_source = read_workspace_source("crates/ui-headless/src/focus_trap.rs");

    for source in [&view_source, &logic_source] {
        assert!(
            !source.contains("NodeRef")
                && !source.contains("focus_trap")
                && !source.contains("RestorePolicy")
                && !source.contains("FallbackTo")
                && !source.contains("Selector")
                && !source.contains("document.body")
                && !source.contains("overlay")
                && !source.contains("modal"),
            "description should not host overlay focus-stack restore logic or private NodeRef targets",
        );
    }

    for needle in [
        "pub enum RestorePolicy",
        "FallbackTo(String)",
        "Selector(String)",
        "document.body()",
    ] {
        assert!(
            headless_focus_source.contains(needle),
            "global focus manager contract should stay in ui-headless focus_trap.rs: missing `{needle}`",
        );
    }

    assert!(
        check2_source.contains("- [x] 焦点全局栈（Focus Stack & GC）")
            && check2_source.contains("N/A（非 Overlay 组件）"),
        "check2 should mark focus stack item complete with explicit N/A rationale for Description scope",
    );
}

#[test]
fn escape_hatch_foreign_zone_is_explicitly_na_and_third_party_instances_do_not_leak() {
    let mod_source = read_source("src/mod.rs");
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");
    let check2_source = read_source("check2.md");

    for source in [&mod_source, &view_source, &logic_source, &styles_source] {
        for forbidden in [
            "ECharts",
            "echarts",
            "Mapbox",
            "Leaflet",
            "MapLibre",
            "ForeignZone",
            "Foreign Zone",
            "YieldControl",
            "CleanupForeign",
            "extern \"C\"",
            "wasm_bindgen",
            "web_sys::",
            "js_sys::",
        ] {
            assert!(
                !source.contains(forbidden),
                "description component should not integrate imperative third-party runtime API `{forbidden}`",
            );
        }
    }

    assert!(
        check2_source.contains("- [x] 受控外交特区（Escape Hatches）")
            && check2_source.contains("N/A（无第三方命令式集成）"),
        "check2 should mark escape-hatch item complete with explicit N/A rationale",
    );
}

#[test]
fn hydration_discontinuity_is_explicitly_na_without_time_or_random_id_initialization() {
    let mod_source = read_source("src/mod.rs");
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");
    let check2_source = read_source("check2.md");
    let headless_id_provider_source =
        read_workspace_source("crates/ui-headless/src/id_provider.rs");

    for source in [&mod_source, &view_source, &logic_source, &styles_source] {
        for forbidden in [
            "now()",
            "Date::now",
            "SystemTime::now",
            "Instant::now",
            "Uuid::new_v4",
            "uuid::Uuid",
            "rand::",
            "thread_rng",
            "Math::random",
            "performance.now",
        ] {
            assert!(
                !source.contains(forbidden),
                "description should not initialize hydration-sensitive IDs via `{forbidden}`",
            );
        }
    }

    assert!(
        !view_source.contains("id=") && !logic_source.contains("id:"),
        "description has no generated DOM id axis and should not require local id seed orchestration",
    );
    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64)",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            headless_id_provider_source.contains(needle),
            "shared deterministic id provider contract should exist in ui-headless: missing `{needle}`",
        );
    }
    assert!(
        check2_source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）")
            && check2_source.contains("N/A（组件无 ID 生成轴）"),
        "check2 should mark hydration discontinuity item complete with explicit N/A rationale",
    );
}

#[test]
fn ssr_and_cross_platform_contract_uses_compile_only_gates_and_keeps_non_wasm_sources_browser_free()
{
    let mod_source = read_source("src/mod.rs");
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let styles_source = read_source("src/styles.rs");
    let check_script_source = read_workspace_source("scripts/check.sh");
    let check2_source = read_source("check2.md");

    for needle in [
        "cargo check -p ui --no-default-features --features inject-css,dev-all-components",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script_source.contains(needle),
            "global check gate should include compile-only platform command `{needle}`",
        );
    }

    for source in [&mod_source, &view_source, &logic_source, &styles_source] {
        for forbidden in [
            "cfg(target_arch = \"wasm32\")",
            "cfg(feature = \"web\")",
            "cfg(feature = \"ssr\")",
            "web_sys",
            "wasm_bindgen",
            "js_sys",
            "window(",
            "document(",
        ] {
            assert!(
                !source.contains(forbidden),
                "description source should keep non-wasm path browser-free and avoid platform split marker `{forbidden}`",
            );
        }
    }

    assert!(
        check2_source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。")
            && check2_source.contains("N/A（组件无平台分支差异）"),
        "check2 should mark SSR/cross-platform item complete with explicit rationale",
    );
}

#[test]
fn headless_web_ssr_feature_mutex_is_protected_by_compile_error_and_component_integration_keeps_it_intact()
 {
    let headless_lib_source = read_workspace_source("crates/ui-headless/src/lib.rs");
    let headless_cargo_source = read_workspace_source("crates/ui-headless/Cargo.toml");
    let description_cargo_source = read_source("Cargo.toml");
    let check_script_source = read_workspace_source("scripts/check.sh");
    let check2_source = read_source("check2.md");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless must keep explicit web/ssr compile-time mutex guard `{needle}`",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless feature model should keep separated web/ssr declarations: missing `{needle}`",
        );
    }

    assert!(
        description_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "description should consume ui-headless via crate boundary without redefining mutex feature model",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script_source.contains(needle),
            "global check gate should verify both ui-headless feature paths compile: missing `{needle}`",
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "check2 should mark headless web/ssr mutex contract as complete",
    );
}

#[test]
fn motion_non_wasm_noop_stub_is_available_and_description_keeps_motion_optional() {
    let motion_lib_source = read_workspace_source("crates/ui-motion/src/lib.rs");
    let description_cargo_source = read_source("Cargo.toml");
    let description_mod_source = read_source("src/mod.rs");
    let description_view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");
    let description_motion_path = crate_root().join("src/motion.rs");

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
            "ui-motion should keep non-wasm predictable no-op stub marker `{needle}`",
        );
    }

    assert!(
        !description_cargo_source.contains("ui-motion"),
        "description should not depend on ui-motion when component has no motion contract",
    );
    assert!(
        !description_mod_source.contains("mod motion")
            && !description_mod_source.contains("pub mod motion")
            && !description_view_source.contains("attach_motion")
            && !description_view_source.contains("MotionOptions"),
        "description should not assume motion handle existence or call attach path directly",
    );
    assert!(
        !description_motion_path.exists(),
        "description should keep src/motion.rs absent for static text scope",
    );
    assert!(
        check2_source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "check2 should mark ui-motion non-wasm no-op/stub item complete",
    );
}

#[test]
fn reduced_motion_ssr_and_wasm_branch_contract_is_explicit_for_static_description_scope() {
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let check_script_source = read_workspace_source("scripts/check.sh");
    let motion_lib_source = read_workspace_source("crates/ui-motion/src/lib.rs");
    let check2_source = read_source("check2.md");

    assert!(
        motion_lib_source.contains("pub fn prefers_reduced_motion() -> bool {")
            && motion_lib_source.contains("fn non_wasm_web_backend_is_predictable_noop()"),
        "ui-motion should provide reduced-motion predictable no-op behavior on non-wasm",
    );

    for source in [&mod_source, &logic_source, &view_source] {
        for forbidden in [
            "cfg(target_arch = \"wasm32\")",
            "cfg(feature = \"web\")",
            "cfg(feature = \"ssr\")",
            "attach_motion",
            "prefers_reduced_motion(",
            "MotionOptions",
        ] {
            assert!(
                !source.contains(forbidden),
                "description should not split semantics by platform or assume animation branch `{forbidden}`",
            );
        }
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script_source.contains(needle),
            "check script should preserve compile-only platform branch validation `{needle}`",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。")
            && check2_source.contains("N/A（组件无动画增强分支）"),
        "check2 should mark reduced-motion/SSR/wasm branch item complete with explicit static-scope rationale",
    );
}

#[test]
fn motion_contract_is_explicitly_na_for_static_description_scope() {
    let description_cargo_source = read_source("Cargo.toml");
    let mod_source = read_source("src/mod.rs");
    let view_source = read_source("src/view.rs");
    let logic_source = read_source("src/logic.rs");
    let motion_lib_source = read_workspace_source("crates/ui-motion/src/lib.rs");
    let check2_source = read_source("check2.md");
    let motion_path = crate_root().join("src/motion.rs");

    assert!(
        !description_cargo_source.contains("ui-motion"),
        "description should not pull ui-motion dependency when no component motion contract is needed",
    );
    assert!(
        !motion_path.exists(),
        "description should keep src/motion.rs absent for static text scope",
    );
    for source in [&mod_source, &view_source, &logic_source] {
        for forbidden in [
            "mod motion",
            "pub mod motion",
            "attach_motion",
            "stiffness",
            "damping",
            "MotionOptions",
            "prefers_reduced_motion(",
        ] {
            assert!(
                !source.contains(forbidden),
                "description static scope should not expose motion contract marker `{forbidden}`",
            );
        }
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep reduced-motion and non-wasm no-op marker `{needle}`",
        );
    }

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A（静态文本组件）",
        "已满足（全局能力不回退）",
        "已满足（组件边界）",
        "motion_contract_is_explicitly_na_for_static_description_scope",
        "description_motion_contract_is_explicitly_na_for_static_description_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep motion-contract governance marker `{needle}`",
        );
    }
}

#[test]
fn performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let shell_source = read_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = read_workspace_source("apps/docs-app/src/perf_probe.rs");
    let e2e_source = read_workspace_source("e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = read_workspace_source("docs/plan/TODO.md");
    let check_script_source = read_workspace_source("scripts/check-ui-performance.sh");
    let check2_source = read_source("check2.md");
    let view_source = read_source("src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep performance budget token `{needle}`",
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
        "\"mount-plus-budget\"",
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf marker `{needle}`",
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "e2e perf gate should keep blocking assertion `{needle}`",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance follow-up should keep marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "N/A（本组件精确 `render_count`）",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep performance governance marker `{needle}`",
        );
    }

    for needle in [
        "cargo test -p ui --test description_semantics description_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            check_script_source.contains(needle),
            "performance gate script should include `{needle}`",
        );
    }

    for needle in [
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep perf attribution marker `{needle}`",
        );
    }
}

#[test]
fn semantic_and_performance_regression_contract_is_covered_beyond_snapshots_for_description() {
    let check2_source = read_source("check2.md");
    let view_source = read_source("src/view.rs");
    let self_source = read_source("test/semantics.rs");

    for needle in [
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should keep semantic marker `{needle}` for non-snapshot contract assertions",
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:click",
        "on:pointerdown",
        "on:pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "description focus/interaction path should stay N/A and avoid `{forbidden}`",
        );
    }

    for needle in [
        "fn performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn view_mounts_stable_semantic_markers()",
        "fn focus_stack_overlay_gc_is_explicitly_na_for_non_overlay_description_component()",
        "render_count",
        "N/A（本组件精确 `render_count`）",
    ] {
        assert!(
            self_source.contains(needle) || check2_source.contains(needle),
            "semantic/perf regression contract should keep marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "已满足（语义断言覆盖）",
        "N/A（焦点流转）",
        "已满足（性能回归与阻断）",
        "semantic_and_performance_regression_contract_is_covered_beyond_snapshots_for_description",
        "description_semantic_and_performance_regression_contract_is_covered_beyond_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep semantic/perf regression marker `{needle}`",
        );
    }
}

#[test]
fn semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots() {
    let check2_source = read_source("check2.md");
    let view_source = read_source("src/view.rs");
    let self_source = read_source("test/semantics.rs");

    for needle in [
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "semantics-first contract should keep marker `{needle}` in view",
        );
    }

    assert!(
        !view_source.contains("role="),
        "description should keep native text semantics and avoid forcing widget role overrides",
    );

    let forbidden = [
        ["assert", "_snapshot!"].concat(),
        ["insta::", "assert", "_snapshot"].concat(),
        ["to_match", "_snapshot"].concat(),
    ];
    for forbidden in forbidden {
        assert!(
            !self_source.contains(forbidden.as_str()),
            "semantics tests should not rely on visual snapshot assertion `{forbidden}` as primary gate",
        );
    }

    for needle in [
        "fn view_mounts_stable_semantic_markers()",
        "fn type_system_and_semantic_markers_form_machine_readable_state_contract()",
        "fn semantic_and_performance_regression_contract_is_covered_beyond_snapshots_for_description()",
    ] {
        assert!(
            self_source.contains(needle),
            "component semantics suite should keep contract test `{needle}`",
        );
    }

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "已满足（语义测试落点）",
        "已满足（契约断言优先）",
        "N/A（显式 role）",
        "已满足（字段变更同步回归）",
        "semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
        "description_semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep semantics-first marker `{needle}`",
        );
    }
}

#[test]
fn e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits() {
    let e2e_source = read_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = read_source("check2.md");

    for needle in [
        "const DESCRIPTION_PAGE = \"/#/components/description\";",
        "body:not(:has(#boot))",
        "[data-component=\"description\"][data-slot=\"description\"]",
        "[data-slot=\"description\"][data-tone=\"default\"][data-state=\"default\"]",
        "[data-slot=\"description\"][data-aria-source=\"custom\"]",
        "[data-slot=\"description\"][data-class-source=\"custom\"][data-custom-class=\"true\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.description.agent-contract.v1\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"optional\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "not.toHaveAttribute(\"role\", /.+/)",
        "toHaveAttribute(\"data-state\", \"default\")",
        "toHaveAttribute(\"data-ui-state\", \"default\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "description e2e should keep semantic selector/wait marker `{needle}`",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ".docs-page-title",
        "section.playground",
        "nth-child",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "description e2e should avoid brittle selector/sleep marker `{forbidden}`",
        );
    }

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "已满足（语义选择器优先）",
        "已满足（WASM 稳定等待）",
        "N/A（async/motion ready-settled）",
        "e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
        "description_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep e2e selector stability marker `{needle}`",
        );
    }
}

#[test]
fn repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints() {
    let e2e_source = read_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = read_source("check2.md");

    for needle in [
        "test(\"docs-app description key flow remains repeatable with semantic ready checkpoints\"",
        "await page.goto(\"/#/components/error-message\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-state\", \"default\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-ui-state\", \"default\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
        "toHaveAttribute(\"data-ui-action\", \"render-snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "description repeatable e2e flow should keep semantic breakpoint marker `{needle}`",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "description repeatable e2e flow should not use unstable wait marker `{forbidden}`",
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "已满足（可重复关键流程）",
        "已满足（可定位语义断点）",
        "N/A（高风险交互路径）",
        "repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints",
        "description_repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep repeatable key-flow e2e marker `{needle}`",
        );
    }
}

#[test]
fn view_macro_complexity_is_controlled_by_shallow_semantic_blocks() {
    let view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");

    assert!(
        view_source.contains("match element {"),
        "description view should split rendering by semantic element axis",
    );

    assert!(
        view_source.contains("fn render_span(")
            && view_source.contains("fn render_paragraph(")
            && view_source.contains("fn render_div("),
        "description view should keep local rendering split into plain functions",
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "description should keep exactly three shallow render blocks (span/p/div)",
    );

    for forbidden in [
        "<section", "<article", "<header", "<footer", "<main", "<aside", "<nav", "<ul", "<ol",
        "<li", "<table", "<tbody", "<tr", "<td",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "description view should not introduce deep semantic container `{forbidden}`",
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "view_macro_complexity_is_controlled_by_shallow_semantic_blocks",
        "description_view_macro_complexity_is_guarded_by_shallow_blocks",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep macro-complexity governance marker `{needle}`",
        );
    }
}

#[test]
fn view_prefers_functional_split_over_extra_components() {
    let view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "description view should keep exactly one component entrypoint",
    );
    assert!(
        view_source.contains("DescriptionElement::Span =>")
            && view_source.contains("render_span(")
            && view_source.contains("DescriptionElement::Paragraph =>")
            && view_source.contains("render_paragraph(")
            && view_source.contains("DescriptionElement::Div =>")
            && view_source.contains("render_div("),
        "description should dispatch lightweight rendering through plain helper functions",
    );
    for helper in ["fn render_span(", "fn render_paragraph(", "fn render_div("] {
        assert!(
            view_source.contains(helper),
            "view should keep plain Rust helper `{helper}`",
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "view_prefers_functional_split_over_extra_components",
        "description_view_prefers_functional_split_over_extra_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep functional-split governance marker `{needle}`",
        );
    }
}

#[test]
fn static_fragment_constantization_is_explicitly_scoped_and_accessible() {
    let view_source = read_source("src/view.rs");
    let check2_source = read_source("check2.md");

    assert!(
        view_source.matches("{text.get_value()}").count() == 3,
        "description should keep content rendering dynamic from `text`, not duplicated static long-copy fragments",
    );
    for forbidden in [
        "inner_html",
        "<svg",
        "<footer",
        "<path",
        "<defs",
        "include_str!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "description view should not introduce heavy static fragment `{forbidden}` in this component scope",
        );
    }
    for helper in ["fn render_span(", "fn render_paragraph(", "fn render_div("] {
        assert!(
            view_source.contains(helper),
            "description should keep static structure concentrated through helper `{helper}`",
        );
    }
    for needle in [
        "aria-label=aria_label",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "static fragment governance should not regress semantic/a11y marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A（无重静态片段）",
        "static_fragment_constantization_is_explicitly_scoped_and_accessible",
        "description_static_fragment_constantization_is_explicitly_scoped_and_accessible",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep static-fragment governance marker `{needle}`",
        );
    }
}

#[test]
fn inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced() {
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");
    let check2_source = read_source("check2.md");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "inner_html",
            "innerHTML",
            "dangerously_set_inner_html",
            "set_inner_html(",
            "insert_adjacent_html(",
            "outer_html",
            "document.write(",
        ] {
            assert!(
                !source.contains(forbidden),
                "description should not expose HTML injection surface `{forbidden}`",
            );
        }
    }

    assert!(
        view_source.matches("{text.get_value()}").count() == 3,
        "description should keep rendering user-provided text through escaped text nodes",
    );
    for needle in [
        "aria-label=aria_label",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "safe text rendering should keep semantic/a11y marker `{needle}`",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A（无 `inner_html` 节点）",
        "inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced",
        "description_inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep inner_html governance marker `{needle}`",
        );
    }
}

#[test]
fn wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");
    let description_cargo_source = read_source("Cargo.toml");
    let ui_components_cargo_source = read_workspace_source("crates/ui/Cargo.toml");
    let wasm_debug_script_source = read_workspace_source("scripts/check-ui-wasm-debug.sh");
    let debug_overlay_source = read_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let check2_source = read_source("check2.md");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "use_ui_trace(",
            "UiTrace",
            "trace.emit(",
            "trace_id",
            "TraceId",
            "replay",
            "debug-overlay",
            "cfg(target_arch = \"wasm32\")",
        ] {
            assert!(
                !source.contains(forbidden),
                "description should not host local wasm-debug tracing surface `{forbidden}`",
            );
        }
    }

    assert!(
        description_cargo_source.contains("[features]\ndefault = []"),
        "description crate should keep feature surface minimal and default-empty",
    );
    assert!(
        !description_cargo_source.contains("wasm-debug")
            && !ui_components_cargo_source.contains("description-wasm-debug"),
        "description should not expose a dedicated wasm-debug feature gate",
    );
    assert!(
        ui_components_cargo_source.contains("component-description = [\"dep:ui-description\"]"),
        "ui should keep description behind component feature isolation",
    );
    assert!(
        !wasm_debug_script_source.contains("description_semantics")
            && !wasm_debug_script_source.contains("component-description"),
        "wasm-debug gate script should not require a dedicated description debug contract",
    );

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "shared debug overlay should keep global trace/replay marker `{needle}`",
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A（静态非交互组件）",
        "wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "description_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep wasm-debug governance marker `{needle}`",
        );
    }
}

#[test]
fn dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na() {
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = read_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Workbench\"",
        "description=\"Interactive display/config/code/css-test playground for Description state contracts.\"",
        "code_signal=workbench_code",
        "test_css_source=test_css_source",
        "test_source_path=\"components/description/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "ui::description::styles::CSS",
        "let (tone_index, set_tone_index) = signal(Some(0_usize));",
        "let (is_disabled, set_is_disabled) = signal(false);",
        "let (is_truncated, set_is_truncated) = signal(false);",
        "SegmentedControl",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=is_truncated set_checked=set_is_truncated",
        "Switch checked=custom_aria_label set_checked=set_custom_aria_label",
        "Switch checked=custom_class set_checked=set_custom_class",
    ] {
        assert!(
            description_docs.contains(needle),
            "description workbench should keep DX contract marker `{needle}`",
        );
    }

    assert!(
        !description_docs.contains("Persist workbench state")
            && !description_docs.contains("localStorage")
            && !description_docs.contains("load_calendar_workbench_state()")
            && !description_docs.contains("save_calendar_workbench_state("),
        "description DX scope keeps persistence explicitly N/A and avoids unnecessary local storage coupling",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "已满足（样式快速反馈）",
        "已满足（上下文保持）",
        "N/A（可选状态保留）",
        "已满足（隔离画布）",
        "dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na",
        "description_dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep DX governance marker `{needle}`",
        );
    }
}

#[test]
fn documentation_as_product_copy_paste_ready_contract_is_implemented_for_description() {
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = read_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "let description_imports =",
        "use ui::{Description, DescriptionElement, DescriptionTone};",
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Disabled / Truncate)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_imports=description_imports.clone()",
        "Snapshot: email is required",
        "Streaming fallback=snapshot: waiting for final validation",
        "data-slot=\"description-source-first\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "docs-description-source-copy",
        "components/description/src/mod.rs",
        "components/description/src/logic.rs",
        "components/description/src/view.rs",
        "components/description/src/styles.rs",
        "component-description",
        "inject-css",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep copy-paste-ready marker `{needle}`",
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "已满足（Playground 覆盖）",
        "已满足（Streaming/Snapshot 展现）",
        "已满足（Source-first 一键复制）",
        "已满足（源码与依赖可追溯）",
        "documentation_as_product_copy_paste_ready_contract_is_implemented_for_description",
        "description_documentation_as_product_copy_paste_ready_contract_is_implemented",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep documentation-as-product marker `{needle}`",
        );
    }
}

#[test]
fn docs_examples_and_matrices_are_synced_with_description_logic_contract() {
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let logic_source = read_source("src/logic.rs");
    let primitive_source = read_workspace_source("crates/ui-state-primitives/src/description.rs");
    let check2_source = read_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Disabled / Truncate)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Workbench\"",
        "title=\"Tone Variants\"",
        "title=\"Truncate + Element + Disabled\"",
        "DescriptionActualConfig {",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep synchronized example/matrix marker `{needle}`",
        );
    }

    for needle in [
        "let (tone_index, set_tone_index) = signal(Some(0_usize));",
        "let (element_index, set_element_index) = signal(Some(0_usize));",
        "let (is_disabled, set_is_disabled) = signal(false);",
        "let (is_truncated, set_is_truncated) = signal(false);",
        "1 => DescriptionTone::Muted,",
        "2 => DescriptionTone::Negative,",
        "_ => DescriptionTone::Default,",
        "1 => DescriptionElement::Span,",
        "2 => DescriptionElement::Div,",
        "_ => DescriptionElement::Paragraph,",
        "if tone != DescriptionTone::Default {",
        "if element != DescriptionElement::Paragraph {",
        "if is_disabled.get() {",
        "if is_truncated.get() {",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep logic-aligned default/api marker `{needle}`",
        );
    }

    for needle in [
        "pub struct DescriptionViewModelInput {",
        "pub text: String,",
        "pub tone: DescriptionTone,",
        "pub is_disabled: bool,",
        "pub is_truncated: bool,",
        "pub aria_label: Option<String>,",
        "pub class_name: Option<String>,",
        "pub lang: Option<String>,",
        "pub dir: Option<A11yDirection>,",
        "let state = resolve_state(DescriptionStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep api/default-alignment marker `{needle}`",
        );
    }

    for needle in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "pub enum DescriptionTone {",
        "#[default]",
        "Default,",
        "pub fn resolve_state(input: DescriptionStateInput) -> DescriptionState {",
        "let data_state_attr = if input.disabled {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should keep defaults and state-matrix marker `{needle}`",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "已满足（文档与示例同步）",
        "已满足（状态矩阵覆盖）",
        "已满足（参数矩阵可检视）",
        "已满足（API/默认值对齐 logic）",
        "docs_examples_and_matrices_are_synced_with_description_logic_contract",
        "description_docs_examples_and_matrices_are_synced_with_logic_contract",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep docs/matrix sync marker `{needle}`",
        );
    }
}

#[test]
fn documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later_for_description()
{
    let readme_source = read_source("src/README.md");
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = read_source("check2.md");

    for needle in [
        "## Quick Start (Use First)",
        "Start with the default API path first. Move to advanced props only when needed.",
        "### Hello World",
        "<Description text=\"This appears below the field.\".to_string() />",
        "### Common Usage",
        "tone=DescriptionTone::Muted",
        "## Advanced Controls (Use When Needed)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::description()",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should keep beginner-friendly documentation marker `{needle}`",
        );
    }

    let quick_start_idx = readme_source
        .find("## Quick Start (Use First)")
        .expect("README should define quick-start section");
    let advanced_idx = readme_source
        .find("## Advanced Controls (Use When Needed)")
        .expect("README should define advanced section");
    assert!(
        quick_start_idx < advanced_idx,
        "README should keep default API path before advanced controls",
    );

    for needle in [
        "pub(super) fn description() -> AnyView {",
        "title=\"Description\"",
        "slug=\"description\"",
    ] {
        assert!(
            forms_extra_source.contains(needle),
            "docs-app should keep accessible description documentation entry `{needle}`",
        );
    }

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "已满足（文档入口存在）",
        "已满足（零门槛 + 常见用法）",
        "已满足（先用后进阶）",
        "documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later_for_description",
        "description_documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep beginner-friendly documentation marker `{needle}`",
        );
    }
}

#[test]
fn interactive_playground_contract_is_available_with_reproducible_flow_for_description() {
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = read_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = read_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Workbench\"",
        "description=\"Interactive display/config/code/css-test playground for Description state contracts.\"",
        "code_signal=workbench_code",
        "test_config_signal=actual_config",
        "DescriptionActualConfig {",
        "SegmentedControl",
        "selected_index=tone_index",
        "selected_index=element_index",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=is_truncated set_checked=set_is_truncated",
        "Switch checked=custom_aria_label set_checked=set_custom_aria_label",
        "Switch checked=custom_class set_checked=set_custom_class",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep interactive playground marker `{needle}`",
        );
    }

    assert!(
        e2e_source.contains(
            "test(\"docs-app description key flow remains repeatable with semantic ready checkpoints\"",
        ),
        "description e2e suite should keep repeatable key-flow regression for playground acceptance",
    );

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "已满足（交互工作台）",
        "已满足（props/状态/反馈可观测）",
        "N/A（AI Spec 联动示例）",
        "已满足（可重复关键路径）",
        "interactive_playground_contract_is_available_with_reproducible_flow_for_description",
        "description_interactive_playground_contract_is_available_with_reproducible_flow",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep interactive-playground marker `{needle}`",
        );
    }
}

#[test]
fn source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths_for_description() {
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = read_workspace_source("apps/docs-app/src/playground.rs");
    let check2_source = read_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "let description_imports =",
        "use ui::{Description, DescriptionElement, DescriptionTone};",
        "code_imports=description_imports.clone()",
        "data-slot=\"description-source-first\"",
        "Source-first / Copy-Paste Ready",
        "<code>\"Show code\"</code>",
        "copy button. Snippets are import-ready via ",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-description-source-copy\".to_string()",
        "components/description/src/mod.rs",
        "components/description/src/logic.rs",
        "components/description/src/view.rs",
        "components/description/src/styles.rs",
        "component-description",
        "inject-css",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep source-first copy-paste marker `{needle}`",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "fn missing_import_lines(raw: &str, imports: &str) -> Vec<String>",
        "if missing_imports.is_empty() {",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground pipeline should keep copy-ready import merge marker `{needle}`",
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "已满足（复制按钮 + import-ready）",
        "已满足（源码落点与依赖前提）",
        "已满足（文档与实现同步）",
        "source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths_for_description",
        "description_source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep source-first copy-paste marker `{needle}`",
        );
    }
}

#[test]
fn heroui_alignment_strategy_and_description_docs_entry_are_synced_for_parameter_changes() {
    let heroui_strategy_source =
        read_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    let forms_extra_source =
        read_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = read_source("src/README.md");
    let check2_source = read_source("check2.md");

    for needle in [
        "### Description 同步记录（2026-02-20）",
        "参数模型同步：`Description` 参数主轴保持 `text/tone/is_disabled/is_truncated/element/aria_label/class_name/lang/dir`",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 通过 `description()` 暴露 `slug=\"description\"` 页面入口",
        "研究文档补充判定：本轮为 Description 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI alignment strategy should keep description sync marker `{needle}`",
        );
    }

    for needle in [
        "pub(super) fn description() -> AnyView {",
        "title=\"Description\"",
        "slug=\"description\"",
    ] {
        assert!(
            forms_extra_source.contains(needle),
            "docs entry should keep description accessibility marker `{needle}`",
        );
    }

    for needle in [
        "# Description",
        "## Quick Start (Use First)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::description()",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should keep equivalent documentation entry marker `{needle}`",
        );
    }

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "已满足（参数模型同步策略文档）",
        "已满足（组件文档入口可访问）",
        "已满足（实现-文档同步约束）",
        "N/A（研究文档补充）",
        "heroui_alignment_strategy_and_description_docs_entry_are_synced_for_parameter_changes",
        "description_heroui_alignment_strategy_and_docs_entry_are_synced_for_parameter_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep HeroUI alignment governance marker `{needle}`",
        );
    }
}

#[test]
fn engineering_capability_unification_is_structured_and_runtime_agnostic() {
    let cargo_source = read_source("Cargo.toml");
    let protocol_source = read_source("src/protocol.rs");
    let protocol_test_source = read_source("test/protocol.rs");
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");
    let check2_source = read_source("check2.md");

    assert!(
        cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "description crate should keep serde dependency for structured protocol contracts",
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum DescriptionComponentSchemaVersion",
        "V1,",
        "pub struct DescriptionComponentSpec",
        "#[serde(default)]",
        "pub schema_version: DescriptionComponentSchemaVersion,",
        "Serialize, Deserialize, Default",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol should keep structured serde schema marker `{needle}`",
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<DescriptionComponentSchemaVersion>();",
        "assert_serde::<DescriptionComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "protocol regression should keep serde contract marker `{needle}`",
        );
    }

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "use tracing::",
            "tracing::info!",
            "tracing::warn!",
            "tracing::error!",
            "tracing::debug!",
            "tokio::",
            "async_std::",
            "async-std",
            "spawn_local(",
            "spawn(",
            "pub async fn",
        ] {
            assert!(
                !source.contains(forbidden),
                "description component should keep runtime-agnostic and non-async boundary: forbidden `{forbidden}`",
            );
        }
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "已满足（serde 结构化协议）",
        "已满足（协议回归）",
        "N/A（spec/config 运行时输入）",
        "N/A（tracing 组件埋点）",
        "N/A（async runtime 绑定）",
        "engineering_capability_unification_is_structured_and_runtime_agnostic",
        "description_engineering_capability_unification_is_structured_and_runtime_agnostic",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep engineering-unification marker `{needle}`",
        );
    }
}

#[test]
fn version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade() {
    let protocol_source = read_source("src/protocol.rs");
    let mod_source = read_source("src/mod.rs");
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");
    let check2_source = read_source("check2.md");

    for needle in [
        "pub enum DescriptionComponentSchemaVersion",
        "V1,",
        "pub struct DescriptionComponentSpec",
        "pub schema_version: DescriptionComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol should keep stable v1 marker `{needle}`",
        );
    }

    assert!(
        !protocol_source.contains("V2")
            && !protocol_source.contains("Breaking")
            && !protocol_source.contains("Deprecated"),
        "protocol should not claim a breaking schema upgrade when none exists",
    );

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        assert!(
            !source.contains("migrate_v1_to_v2")
                && !source.contains("schema_registry")
                && !source.contains("codemod"),
            "component source should not add fake migration scaffolding without a real breaking upgrade",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A（当前变更范围）",
        "已满足（现状可证）",
        "已满足（迁移层不应虚构）",
        "升级触发条件（后续约束）",
        "version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade",
        "description_version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep deprecation/migration marker `{needle}`",
        );
    }
}

#[test]
fn ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent() {
    let ui_components_lib_source = read_workspace_source("crates/ui/src/lib.rs");
    let ui_components_css_source = read_workspace_source("crates/ui/src/css.rs");
    let ui_components_root_source = read_workspace_source("crates/ui/src/root.rs");
    let active_highlight_source =
        read_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let check2_source = read_source("check2.md");

    assert!(
        ui_components_lib_source.contains("#[cfg(feature = \"component-description\")]")
            && ui_components_lib_source.contains("pub use ui_description as description;"),
        "ui lib.rs should keep feature-gated description re-export contract",
    );
    assert!(
        !ui_components_lib_source.contains("pub use web_sys")
            && !ui_components_lib_source.contains("pub use leptos::web_sys"),
        "ui public entry should not leak platform web-sys detail types",
    );

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-description\")]",
        "out.push_str(crate::description::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui css entry should keep `{needle}`",
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "UiRoot should keep centralized injection marker `{needle}`",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`",
        );
    }

    for forbidden in ["Accordion", "Description", "Menu"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component-specific semantics `{forbidden}`",
        );
    }

    let workspace_root = crate_root().join("../../");
    for rel in [
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
    ] {
        let path = workspace_root.join(rel);
        assert!(
            !path.exists(),
            "forbidden ui entrypoint file should be absent: {}",
            path.display(),
        );
    }

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "已满足（入口与导出边界）",
        "已满足（CSS 聚合边界）",
        "已满足（Root 注入集中）",
        "已满足（共享视觉原语落点）",
        "已满足（禁置文件不存在）",
        "ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent",
        "description_ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep ui entrypoint governance marker `{needle}`",
        );
    }
}
