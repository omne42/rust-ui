fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "component_toml" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/avatar_group.rbi"),
        "manifest" => include_str!("../Cargo.toml"),
        "check2" => include_str!("../check2.md"),
        "platform_script" => include_str!("../../../scripts/check-ui-platforms.sh"),
        "ui_headless_manifest" => include_str!("../../../crates/ui-headless/Cargo.toml"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_motion_non_wasm_stub_test" => {
            include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs")
        }
        "docs_shell" => include_str!("../../../apps/docs-app/src/pages/components/shell.rs"),
        "perf_probe" => include_str!("../../../apps/docs-app/src/perf_probe.rs"),
        "coverage_e2e" => include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs"),
        "avatar_group_e2e_contract" => {
            include_str!("../../../e2e/tests/docs_app_avatar_group_contract.spec.mjs")
        }
        "todo_plan" => include_str!("../../../docs/plan/TODO.md"),
        "perf_script" => include_str!("../../../scripts/check-ui-performance.sh"),
        "ui_components_manifest" => include_str!("../../../crates/ui/Cargo.toml"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/avatar_group.rs"),
        "ui_components_root" => include_str!("../../../crates/ui/src/root.rs"),
        "headless_id_provider" => include_str!("../../../crates/ui-headless/src/id_provider.rs"),
        "readme" => include_str!("../src/README.md"),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn function_signature(source: &str, fn_name: &str) -> String {
    let start = source
        .find(&format!("pub fn {fn_name}("))
        .unwrap_or_else(|| panic!("missing function signature for `{fn_name}`"));
    let end = source[start..]
        .find(") -> impl IntoView {")
        .unwrap_or_else(|| panic!("missing IntoView return marker for `{fn_name}`"));
    source[start..start + end].to_string()
}

#[test]
fn avatar_group_module_boundary_is_minimal_and_wires_component_semantics_tests() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module.contains(required),
            "avatar-group module boundary should include `{required}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "avatar-group internals should stay private: `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_logic_view_styles_follow_ui_components_layered_responsibilities() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn compose_avatar_group_class_name(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/derivation helpers via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-",
        "labeled_group_attrs(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not carry render/headless detail `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
        "labeled_group_attrs(",
        "data-slot=\"avatar-group\"",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render and mount semantics via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-avatar-group[data-state=\"overflow\"]",
        "var(--ui-",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static styling via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:click", "labeled_group_attrs(", "logic::"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry runtime/view logic `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_styles_use_defensive_variable_fallback_chains_local() {
    let styles = load_source("styles");
    let theme_css = include_str!("../../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-avatar-size-sm, var(--ui-fallback-avatar-size-sm))",
        "var(--ui-avatar-size-md, var(--ui-fallback-avatar-size-md))",
        "var(--ui-avatar-size-lg, var(--ui-fallback-avatar-size-lg))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
    ] {
        assert!(
            styles.contains(required),
            "avatar-group styles should keep defensive fallback chain `{required}`."
        );
    }

    for required in [
        "  --ui-fallback-avatar-size-sm: 24px;",
        "  --ui-fallback-avatar-size-md: 32px;",
        "  --ui-fallback-avatar-size-lg: 40px;",
        "  --ui-fallback-bg: {};",
        "  --ui-fallback-bg-muted: {};",
        "  --ui-fallback-fg: {};",
        "  --ui-fallback-border-width: 1px;",
        "  --ui-fallback-accent: {};",
        "  --ui-fallback-accent-soft: {};",
        "  --ui-fallback-shadow-sm: {};",
        "  --ui-fallback-line-height-100: {}px;",
        "  --ui-fallback-font-size-100: {}px;",
        "  --ui-fallback-button-size-s-font-size: {}px;",
        "  --ui-fallback-space-xs: {}px;",
        "  --ui-fallback-space-sm: {}px;",
        "  --ui-fallback-space-md: {}px;",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme css should provide avatar-group fallback source `{required}`."
        );
    }

    for forbidden in [
        "--ui-avatar-group-size: 2rem;",
        "--ui-avatar-group-overlap: 10px;",
        "--ui-avatar-group-overflow-padding: 0.375rem;",
        "border-radius: 9999px;",
        "line-height: var(--ui-line-height-100, 16px);",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-button-size-s-font-size, 13px)",
        "border: 2px solid var(--ui-bg);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "avatar-group styles should not keep raw component terminal fallback `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_cascade_layer_and_runtime_style_contract_is_enforced_local() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css registry should keep cascade-layer marker `{required}`."
        );
    }

    for line in view.lines().chain(logic.lines()) {
        let trimmed = line.trim_start();
        assert!(
            !trimmed.starts_with("style="),
            "avatar-group should not use plain inline `style=...`; found `{trimmed}`."
        );
        if trimmed.contains("style:") {
            assert!(
                trimmed.contains("style:--"),
                "avatar-group runtime style mutation must use CSS custom properties only; found `{trimmed}`."
            );
        }
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "avatar-group should avoid plain inline style token `{forbidden}`."
        );
    }

    assert!(
        check2.contains("级联层覆盖（`@layer ui`）"),
        "avatar-group checklist should keep cascade-layer contract entry."
    );
}

#[test]
fn avatar_group_ui_components_entrypoints_and_headless_boundaries_are_correct_local() {
    let check2 = load_source("check2");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_root = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state =
        include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let headless_a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for required in [
        "#[cfg(feature = \"component-avatar_group\")]",
        "pub use ui_avatar_group as avatar_group;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep stable feature-gated entrypoint `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen::",
        "pub use leptos::html::",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep centralized feature-gated css aggregation `{required}`."
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
            "ui root.rs should keep centralized theme/i18n/css injection boundary `{required}`."
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringConfig",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight primitive should keep shared highlight capability `{required}`."
        );
    }

    for forbidden in ["AvatarGroup", "Accordion", "business"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight primitive should avoid component-business semantic coupling `{forbidden}`."
        );
    }

    for (path, label) in [
        (
            manifest_dir
                .join("../../crates/ui/src/overlay_open.rs")
                .display()
                .to_string(),
            "overlay_open.rs",
        ),
        (
            manifest_dir
                .join("../../crates/ui/src/presence.rs")
                .display()
                .to_string(),
            "presence.rs",
        ),
        (
            manifest_dir
                .join("../../crates/ui/src/a11y.rs")
                .display()
                .to_string(),
            "a11y.rs",
        ),
    ] {
        assert!(
            !std::path::Path::new(&path).exists(),
            "ui/src/{label} should stay absent; primitive belongs to ui-headless."
        );
    }

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            headless_controllable_state.contains(required)
                || headless_presence.contains(required)
                || headless_a11y.contains(required),
            "headless primitive boundary should provide `{required}`."
        );
    }

    assert!(
        check2.contains("`ui` 固定入口文件落点正确。"),
        "avatar-group checklist should track ui fixed-entrypoint contract."
    );
}

#[test]
fn avatar_group_component_directory_standard_file_layout_is_correct_local() {
    let check2 = load_source("check2");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            manifest_dir.join("src").join(required).exists(),
            "avatar-group component directory should include `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !manifest_dir.join("src").join(forbidden).exists(),
            "avatar-group should not include `{forbidden}` in current scope."
        );
    }

    assert!(
        !manifest_dir.join("src/motion.rs").exists(),
        "avatar-group is non-interactive in current scope; `motion.rs` stays N/A and should remain absent."
    );

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep minimal stable exports via `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub mod motion",
        "mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should avoid over-export or render.rs drift token `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn compose_avatar_group_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation responsibilities via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-",
        "labeled_group_attrs(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry view/headless mount details `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-avatar-group[data-state=\"overflow\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static css contract `{required}`."
        );
    }

    for forbidden in ["rgb(", "hsl(", "view! {", "logic::", "labeled_group_attrs("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid hardcoded theme/runtime tokens `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
        "labeled_group_attrs(",
        "data-slot=\"avatar-group\"",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structural render + headless mount via `{required}`."
        );
    }

    for forbidden in ["mod render;", "include!(\"render.rs\")"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not drift into render.rs include pattern `{forbidden}`."
        );
    }

    assert!(
        check2.contains("组件目录标准文件落点正确。"),
        "avatar-group checklist should track component-directory standard layout contract."
    );
}

#[test]
fn avatar_group_context_compression_manifest_and_rbi_are_present_and_consistent_locally() {
    let check2 = load_source("check2");
    let component_manifest = load_source("component_toml");
    let component_rbi = load_source("rbi");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required_file in ["Component.toml", "avatar_group.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "avatar-group context-compression file should exist: `{required_file}`."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"AvatarGroup\"",
        "crate = \"ui-avatar-group\"",
        "name = \"items\"",
        "name = \"max\"",
        "name = \"size\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub struct AvatarGroupItem {",
        "pub fn AvatarGroup(",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "avatar_group.rbi should keep signature-projection marker `{required}`."
        );
    }

    assert!(
        check2.contains("上下文压缩协议（Manifest + RBI）"),
        "avatar-group checklist should keep context-compression entry."
    );
}

#[test]
fn avatar_group_agent_contract_schema_is_typed_traceable_and_whitelisted_locally() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let component_manifest = load_source("component_toml");

    for required in [
        "pub const AVATAR_GROUP_AGENT_SCHEMA: &str = \"ui.avatar-group.agent.v1\";",
        "pub enum AvatarGroupAgentIntent",
        "pub enum AvatarGroupAgentAction",
        "pub enum AvatarGroupAgentStateAxis",
        "pub enum AvatarGroupAgentSourceAxis",
        "pub struct AvatarGroupAgentContract",
        "pub fn resolve_avatar_group_agent_state_axis(",
        "pub fn resolve_avatar_group_agent_source_axis(",
        "pub fn resolve_avatar_group_agent_contract(",
        "intent: AvatarGroupAgentIntent::DisplayIdentityCollection",
    ] {
        assert!(
            logic.contains(required),
            "avatar-group logic should keep typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "let agent_contract = logic::resolve_avatar_group_agent_contract(state);",
        "data-ui-schema=agent_contract.schema",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view.contains(required),
            "avatar-group view should mount agent-contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "data-ui-schema=format!(",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "avatar-group agent-contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "name = \"whitelist_render_policy_no_script_injection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "avatar-group Component.toml should keep agent-contract capability `{required}`."
        );
    }

    assert!(
        check2.contains("语义标记统一升级为 Agent Contract（Schema 化）"),
        "avatar-group checklist should keep agent-contract governance entry."
    );
}

#[test]
fn avatar_group_streaming_definition_is_llm_output_only_with_two_modes_locally() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");

    for required in [
        "流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group checklist should keep streaming-definition marker `{required}`."
        );
    }

    for forbidden in [
        "AiSpace",
        "AiRenderMode",
        "is_streaming",
        "on_stream",
        "token_delta",
        "streaming_state",
        "data-ui-stream-mode",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !component_toml.contains(forbidden)
                && !rbi.contains(forbidden),
            "avatar-group is snapshot-only in current scope; streaming protocol token `{forbidden}` should stay absent."
        );
    }
}

#[test]
fn avatar_group_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status_locally()
 {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let component_toml = load_source("component_toml");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group checklist should keep streaming-optional policy marker `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupAgentStreamSupport {",
        "Required,",
        "Optional,",
        "Self::Required => \"required\"",
        "Self::Optional => \"optional\"",
        "pub enum AvatarGroupAgentStreamFallback {",
        "Self::Snapshot => \"snapshot\"",
        "pub enum AvatarGroupAgentOutputStatus {",
        "Draft,",
        "Verified,",
        "Submittable,",
        "Self::Draft => \"draft\"",
        "Self::Verified => \"verified\"",
        "Self::Submittable => \"submittable\"",
        "pub stream_support: AvatarGroupAgentStreamSupport,",
        "pub stream_fallback: AvatarGroupAgentStreamFallback,",
        "pub output_status: AvatarGroupAgentOutputStatus,",
        "stream_support: AvatarGroupAgentStreamSupport::Optional,",
        "stream_fallback: AvatarGroupAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic.contains(required),
            "avatar-group logic should keep typed streaming-policy marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"avatar-group\"",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "data-ui-stream-support=agent_contract.stream_support.as_str()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_str()",
        "data-ui-output-status=agent_contract.output_status.as_str()",
    ] {
        assert!(
            view.contains(required),
            "avatar-group view should mount streaming-policy semantic marker `{required}`."
        );
    }

    {
        let required =
            "name = \"streaming_optional_with_snapshot_fallback_and_output_status_markers\"";
        assert!(
            component_toml.contains(required),
            "avatar-group Component.toml should declare streaming-policy capability `{required}`."
        );
    }

    for forbidden in ["retry", "reconnect", "validation_error", "transport_error"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "avatar-group component should keep upper-layer resilience out of scope; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow_locally() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for forbidden in [
        ".unwrap(",
        ".unwrap_err(",
        ".expect(",
        "let _ =",
        ".to_owned(",
        "String::from(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group non-test source should satisfy rust-hygiene and avoid `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-avatar-group\")",
        "Cow::Owned(format!(\"ui-avatar-group--size-{}\", state.size_attr))",
        ".map(Cow::into_owned)",
        ".collect::<Vec<_>>()",
        ".join(\" \")",
    ] {
        assert!(
            logic.contains(required),
            "avatar-group logic should use Cow-based class assembly marker `{required}`."
        );
    }

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow_locally",
        "avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group check2 should keep rust-hygiene marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_snapshot_base_capability_accepts_complete_configuration_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");

    for required in [
        "`Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group checklist should keep snapshot-base marker `{required}`."
        );
    }

    for required in [
        "pub fn AvatarGroup(",
        "items: Vec<AvatarGroupItem>",
        "#[prop(optional)] max: Option<usize>",
        "#[prop(optional)] size: AvatarSize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let normalized = logic::normalize_avatar_group_input(",
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
    ] {
        assert!(
            view.contains(required),
            "avatar-group snapshot render path should support complete configuration marker `{required}`."
        );
    }

    for required in [
        "name = \"items\"",
        "name = \"max\"",
        "name = \"size\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Component.toml should expose snapshot-complete input marker `{required}`."
        );
    }

    for required in [
        "pub fn AvatarGroup(",
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: ui_avatar::AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
    ] {
        assert!(
            rbi.contains(required),
            "avatar_group.rbi should project snapshot-complete signature marker `{required}`."
        );
    }

    for forbidden in [
        "AiSpace",
        "AiRenderMode",
        "is_streaming",
        "on_stream",
        "token_delta",
        "streaming_state",
    ] {
        assert!(
            !view.contains(forbidden)
                && !component_toml.contains(forbidden)
                && !rbi.contains(forbidden),
            "avatar-group snapshot base capability should avoid streaming-only token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_component_consumes_primitives_and_headless_without_reimplementation() {
    let logic = load_source("logic");
    let view = load_source("view");
    let primitive = load_source("primitive");

    for required in [
        "pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};",
        "ui_state_primitives::avatar_group::resolve_render_state(input)",
        "use ui_headless::labeled_group_attrs;",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "avatar-group should consume primitive/headless contract via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "pub struct AvatarGroupRenderState",
    ] {
        assert!(
            primitive.contains(required),
            "ui-state-primitives should own avatar-group state model `{required}`."
        );
    }

    for forbidden in [
        "pub enum AvatarGroupVisualState {",
        "pub enum AvatarGroupAriaLabelSource {",
        "pub enum AvatarGroupClassSource {",
        "pub struct AvatarGroupRenderState {",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "component layer should not reimplement primitives `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy_local() {
    let view = load_source("view");
    let sig = function_signature(view, "AvatarGroup");
    let headless_a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let i18n_common = include_str!("../../../crates/ui-headless/src/i18n/common.rs");

    for required in [
        "use ui_headless::labeled_group_attrs;",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "common.avatar_group_aria_label.as_ref()",
        "common.avatar_group_overflow_aria_label_suffix.as_ref()",
        "let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            view.contains(required),
            "avatar-group should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in ["lang: Option<String>", "dir: Option<A11yDirection>"] {
        assert!(
            sig.contains(required),
            "avatar-group public API should expose locale direction hooks via `{required}`."
        );
    }

    for required in [
        "pub fn labeled_group_attrs(",
        "pub struct LabeledGroupA11yAttrs",
        "pub fn locale_attrs(",
    ] {
        assert!(
            headless_a11y.contains(required),
            "shared a11y utility should come from ui-headless via `{required}`."
        );
    }

    for required in [
        "avatar_group_aria_label",
        "avatar_group_overflow_aria_label_suffix",
    ] {
        assert!(
            i18n_common.contains(required),
            "i18n bundle should provide avatar-group string slot `{required}`."
        );
    }

    for forbidden in [
        "\"Avatar group\"",
        "\"more collaborators\"",
        "role=\"group\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view.contains(forbidden),
            "view should not hardcode user-visible copy or locale/a11y literal `{forbidden}`."
        );
    }

    for forbidden in ["on:click", "on:keydown", "on:keyup", "tabindex="] {
        assert!(
            !view.contains(forbidden),
            "avatar-group has no interactive focus model; unexpected token `{forbidden}` should stay absent."
        );
    }
}

#[test]
fn avatar_group_public_surface_does_not_leak_dom_or_platform_types() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen",
        "pub use leptos::html::",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "avatar-group public surface should not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_api_naming_contract_is_prefix_ready_without_alias_drift() {
    let view = load_source("view");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "avatar-group public signature should include `{required}`."
        );
    }

    // AvatarGroup currently has no bool callback/default-value axis.
    assert!(
        !sig.contains(": bool"),
        "if bool props are added later they must follow `is_*` naming."
    );
    assert!(
        !sig.contains("on_"),
        "avatar-group currently exposes no callbacks; future callbacks must follow `on_*`."
    );
    assert!(
        !sig.contains("default_"),
        "avatar-group currently exposes no default-value props; future defaults must follow `default_*`."
    );
}

#[test]
fn avatar_group_has_no_controllable_state_axis_and_avoids_half_controlled_api() {
    let view = load_source("view");
    let logic = load_source("logic");
    let sig = function_signature(view, "AvatarGroup");

    for forbidden in [" value:", "default_", "on_value_change", "on_open_change"] {
        assert!(
            !sig.contains(forbidden),
            "avatar-group should not expose partial controllable API marker `{forbidden}`."
        );
    }

    for forbidden in [
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
        "on_value_change",
        "on_open_change",
        "default_value",
        "default_open",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "avatar-group has no controllable state axis and should not include `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_defaults_are_normalized_in_logic_only() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "resolve_avatar_group_aria_label_with_fallback(",
        "name: name.unwrap_or_default()",
        "src: src.unwrap_or_default()",
        "alt: alt.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should centralize default normalization via `{required}`."
        );
    }

    for forbidden in [
        "unwrap_or_default()",
        "unwrap_or(",
        "resolve_avatar_group_aria_label_with_fallback(",
        "normalize_avatar_group_optional_text(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not perform default fallback via `{forbidden}`."
        );
    }

    assert!(
        view.contains("logic::normalize_avatar_group_input("),
        "view.rs should consume normalized defaults from logic.rs."
    );
}

#[test]
fn avatar_group_state_normalization_is_centralized_in_logic() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};",
        "pub fn resolve_avatar_group_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState",
        "ui_state_primitives::avatar_group::resolve_render_state(input)",
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should own typed state normalization/derivation via `{required}`."
        );
    }

    for required in [
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "logic::normalize_avatar_group_item_fields(",
    ] {
        assert!(
            view.contains(required),
            "view.rs should only consume normalized/derived state via `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::avatar_group::resolve_render_state(",
        "ui_state_primitives::avatar_group::normalize_",
        "on:",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not reimplement state-machine/event rules via `{forbidden}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should consume derived semantic markers via `{required}`."
        );
    }

    for forbidden in ["logic::", "AvatarGroupStateInput", "resolve_render_state("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should never derive state via `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_state_markers_are_observable_and_closed_set_contracts_local() {
    let view = load_source("view");
    let styles = load_source("styles");
    let primitive = load_source("primitive");

    for required in [
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
    ] {
        assert!(
            view.contains(required),
            "state/a11y marker should be observable from stable semantic attribute `{required}`."
        );
    }

    for required in [
        "Self::Stable => \"stable\"",
        "Self::Overflow => \"overflow\"",
        "Self::Empty => \"empty\"",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
    ] {
        assert!(
            primitive.contains(required),
            "marker value should come from enum closed set via `{required}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles.contains(required),
            "selector contract should target stable state/source marker `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-state=if",
        "data-aria-label-source=format!",
        ".ui-avatar-group:nth-child(",
        ".ui-avatar-group:nth-of-type(",
        ".ui-avatar-group > * > * > *",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "state contract should not depend on fragile/free-text marker `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_discrete_state_axes_are_type_safe_enums() {
    let primitive = load_source("primitive");
    let logic = load_source("logic");
    let view = load_source("view");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "size: AvatarSize",
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "Self::Stable => \"stable\"",
        "Self::Overflow => \"overflow\"",
        "Self::Empty => \"empty\"",
    ] {
        assert!(
            sig.contains(required) || primitive.contains(required),
            "discrete state axis should be enum-based via `{required}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "is_overflow: bool",
        "is_empty: bool",
    ] {
        assert!(
            !sig.contains(forbidden) && !primitive.contains(forbidden),
            "discrete state should avoid string/bool explosion marker `{forbidden}`."
        );
    }

    assert!(
        view.contains("data-state=state.visual_state.as_str()"),
        "view should expose enum-derived closed-set status marker."
    );
    assert!(
        logic.contains("pub use ui_avatar::AvatarSize;"),
        "logic should keep discrete size axis typed as AvatarSize enum."
    );
}

#[test]
fn avatar_group_state_primitive_source_boundary_is_enforced() {
    let logic = load_source("logic");
    let view = load_source("view");
    let primitive = load_source("primitive");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};",
        "pub fn resolve_avatar_group_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState",
        "ui_state_primitives::avatar_group::resolve_render_state(input)",
    ] {
        assert!(
            logic.contains(required),
            "component logic should consume state primitives via `{required}`."
        );
    }

    for required in [
        "pub struct AvatarGroupRenderState",
        "pub struct AvatarGroupStateInput",
        "pub fn resolve_render_state(",
    ] {
        assert!(
            primitive.contains(required),
            "state primitive contract should live in ui-state-primitives via `{required}`."
        );
    }

    for forbidden in [
        "use crate::store::",
        "use crate::state::",
        "global_store",
        "app_store",
        "redux",
        "zustand",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "component layer must not bind business store directly; found `{forbidden}`."
        );
    }

    for forbidden in [
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
        "Signal<",
        "Store<",
    ] {
        assert!(
            !sig.contains(forbidden),
            "public API should not expose framework/store state container `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_async_loading_protocol_and_keeps_sync_render_contract() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "on_retry",
        "error:",
        "async fn",
        "Future<",
        "spawn_local",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !sig.contains(forbidden),
            "avatar-group has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn avatar_group_dx_api_is_simple_and_docs_offer_minimal_hello_world() {
    let view = load_source("view");
    let sig = function_signature(view, "AvatarGroup");
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "public API should keep simple direct props via `{required}`."
        );
    }

    for forbidden in [
        " state:",
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
        "Signal<",
    ] {
        assert!(
            !sig.contains(forbidden),
            "public API should not force internal state wiring token `{forbidden}`."
        );
    }

    for required in [
        "let hello_code =",
        "r#\"<AvatarGroup items=empty_items.clone() />\"#.to_string()",
        "title=\"Hello World\" code_signal=hello_code",
        "<AvatarGroup items=empty_items.clone() />",
        "<AvatarGroup items=overflow_items.clone() max=3 size=AvatarSize::Md />",
        "class_name=\"docs-avatar-group-custom\".to_string()",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let code_imports =",
        "use ui::{AvatarGroup, AvatarGroupItem, AvatarSize};",
        "data-slot=\"avatar-group-copy-ready-hint\"",
    ] {
        assert!(
            docs.contains(required),
            "docs should provide minimal + advanced usage path via `{required}`."
        );
    }

    for forbidden in ["<AvatarGroup state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !docs.contains(forbidden),
            "docs hello path should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_composition_api_uses_typed_item_specs_and_rejects_parallel_arrays() {
    let view = load_source("view");
    let logic = load_source("logic");
    let sig = function_signature(view, "AvatarGroup");
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "data-slot=\"avatar-group-item\"",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "composition API should keep typed item-spec dimension via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "children: Vec<",
        "labels=",
        "titles=",
        "titles + panels",
        "labels + children",
    ] {
        assert!(
            !sig.contains(forbidden)
                && !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !docs.contains(forbidden),
            "component should reject parallel-array composition token `{forbidden}`."
        );
    }

    for required in ["<AvatarGroup", "items=vec![", "AvatarGroupItem {"] {
        assert!(
            docs.contains(required),
            "docs should keep typed ItemSpec composition sample via `{required}`."
        );
    }
}

#[test]
fn avatar_group_docs_parameter_and_state_matrix_match_logic_defaults() {
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");
    let logic = load_source("logic");
    let primitive = load_source("primitive");
    let sig = function_signature(load_source("view"), "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "AvatarGroup public API should expose `{required}` for docs alignment."
        );
    }

    for required in [
        "data-slot=\"avatar-group-state-matrix\"",
        "data-slot=\"avatar-group-state-rows\"",
        "data-slot=\"avatar-group-parameter-matrix\"",
        "data-slot=\"avatar-group-parameter-rows\"",
        "\"max: Option&lt;usize&gt;\"",
        "default = None -> normalize to 4",
        "\"size: AvatarSize\"",
        "default = AvatarSize::Md",
        "\"aria_label: Option&lt;String&gt;\"",
        "\"class_name: Option&lt;String&gt;, lang: Option&lt;String&gt;\"",
        "\"dir: Option&lt;A11yDirection&gt;\"",
    ] {
        assert!(
            docs.contains(required),
            "AvatarGroup docs should keep parameter/state matrix marker `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_MAX_VISIBLE: usize = 4;",
        "pub fn normalize_max_visible(value: Option<usize>) -> usize {",
        "value.unwrap_or(DEFAULT_MAX_VISIBLE)",
    ] {
        assert!(
            primitive.contains(required),
            "AvatarGroup primitive default contract should keep `{required}`."
        );
    }

    for required in [
        "let max_visible = normalize_avatar_group_max_visible(max);",
        "resolve_avatar_group_aria_label_with_fallback(aria_label, default_aria_label);",
        "lang: normalize_avatar_group_optional_text(lang),",
    ] {
        assert!(
            logic.contains(required),
            "AvatarGroup logic should keep docs-mapped normalization marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_docs_interactive_playground_supports_live_prop_controls_and_preview_feedback() {
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "let workbench_roster_options = vec![",
        "let workbench_size_options = vec![\"sm\".to_string(), \"md\".to_string(), \"lg\".to_string()];",
        "let workbench_max_options = vec![\"2\".to_string(), \"3\".to_string(), \"4\".to_string()];",
        "let (workbench_roster_index, set_workbench_roster_index) = signal(Some(2_usize));",
        "let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));",
        "let (workbench_max_index, set_workbench_max_index) = signal(Some(1_usize));",
        "let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
        "let (workbench_rtl, set_workbench_rtl) = signal(false);",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "test_config_signal=workbench_config",
        "data-slot=\"avatar-group-workbench-controls\"",
        "data-slot=\"avatar-group-workbench-preview\"",
        "data-slot=\"avatar-group-workbench-configured\"",
        "data-slot=\"avatar-group-workbench-state\"",
        "data-slot=\"avatar-group-spec-preview-na\"",
        "id_base=\"docs-avatar-group-workbench-roster\".to_string()",
        "id_base=\"docs-avatar-group-workbench-size\".to_string()",
        "id_base=\"docs-avatar-group-workbench-max\".to_string()",
        "<Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_rtl set_checked=set_workbench_rtl>",
        "expected: state={expected_state}, size={size_attr}, total={configured_total}, overflow={overflow}",
        "AI Spec input/preview linkage: N/A for AvatarGroup (non-spec component).",
    ] {
        assert!(
            docs.contains(required),
            "AvatarGroup interactive docs playground should include `{required}`."
        );
    }

    for forbidden in ["ui_state_primitives::", "ui_headless::", "state=..."] {
        assert!(
            !docs.contains(forbidden),
            "AvatarGroup interactive docs path should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_source_first_docs_are_copy_paste_ready_and_traceable_local() {
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e = load_source("avatar_group_e2e_contract");
    let check2 = load_source("check2");

    for required in [
        "data-slot=\"avatar-group-source-first\"",
        "data-slot=\"avatar-group-source-first-contract\"",
        "data-slot=\"avatar-group-source-prerequisites\"",
        "component-avatar-group",
        "inject-css",
        "UiRoot",
        "<Snippet",
        "text=source_first_code.get()",
        "label=\"Copy avatar-group starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-avatar-group-source-copy\".to_string()",
        "data-slot=\"avatar-group-source-paths\"",
        "components/avatar-group/src/mod.rs",
        "components/avatar-group/src/logic.rs",
        "components/avatar-group/src/view.rs",
        "components/avatar-group/src/styles.rs",
        "data-slot=\"avatar-group-source-sync-note\"",
    ] {
        assert!(
            docs.contains(required),
            "AvatarGroup source-first docs should include `{required}`."
        );
    }

    for required in [
        "docs-app avatar-group source-first section exposes copy-ready starter and source anchors",
        "[data-slot=\"avatar-group-source-first\"]",
        "[data-slot=\"snippet\"]",
        "[data-slot=\"snippet-copy-button\"]",
        "[data-slot=\"snippet-pre\"]",
        "[data-slot=\"avatar-group-source-paths\"]",
        "[data-slot=\"avatar-group-source-sync-note\"]",
    ] {
        assert!(
            e2e.contains(required),
            "AvatarGroup source-first e2e contract should include `{required}`."
        );
    }

    for required in [
        "Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮",
        "文档需指向真实源码落点并说明依赖前提",
        "文档代码与当前实现必须同步",
    ] {
        assert!(
            check2.contains(required),
            "AvatarGroup checklist should keep source-first copy-ready marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_heroui_alignment_docs_and_component_entry_are_synced_local() {
    let heroui = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let spectrum_heroui =
        include_str!("../../../docs/research/spectrum-heroui-style-interface-study.md");
    let pages_registry = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for required in [
        "### AvatarGroup 同步记录（2026-02-20）",
        "`AvatarGroup` 参数主轴保持 `items/max/size/aria_label/class_name/lang/dir`",
        "component_doc!(\"AvatarGroup\", \"avatar-group\", \"Display\", display::avatar_group)",
        "`#/components/avatar-group` 可索引访问",
        "`apps/docs-app/src/pages/components/pages/display.rs::avatar_group()` 覆盖 `Hello World`",
        "`Interactive Playground (Props + State + Preview)`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`component-avatar-group`、`UiRoot`、`inject-css`",
        "`components/avatar-group/src/{mod,logic,view,styles}.rs`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            heroui.contains(required),
            "AvatarGroup HeroUI alignment doc should include `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"AvatarGroup\"",
        "\"avatar-group\"",
        "display::avatar_group",
    ] {
        assert!(
            pages_registry.contains(required),
            "docs pages registry should keep AvatarGroup index entry via `{required}`."
        );
    }

    for required in [
        "slug=\"avatar-group\"",
        "pub(super) fn avatar_group() -> AnyView {",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_display.contains(required),
            "AvatarGroup docs page should keep synced docs entry token `{required}`."
        );
    }

    for required in ["#/components/avatar-group", "## docs-app 入口"] {
        assert!(
            readme.contains(required),
            "AvatarGroup README should expose docs-app entry token `{required}`."
        );
    }

    assert!(
        spectrum_heroui.contains("# Spectrum × HeroUI 样式与接口综合学习（v0）"),
        "spectrum-heroui research baseline doc should remain available."
    );

    for required in [
        "[x] HeroUI 对标文档与组件文档同步",
        "docs/spec/heroui-parameter-design-strategy.md",
        "docs/research/spectrum-heroui-style-interface-study.md",
        "仅代码更新无文档更新",
    ] {
        assert!(
            check2.contains(required),
            "AvatarGroup checklist should record HeroUI docs-sync contract via `{required}`."
        );
    }
}

#[test]
fn avatar_group_readme_is_beginner_friendly_with_default_path_before_advanced() {
    let readme = load_source("readme");
    let docs = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "# AvatarGroup",
        "## 先用起来（Quick Start / Hello World）",
        "不需要先理解分层架构",
        "use ui::{AvatarGroup, AvatarGroupItem};",
        "<AvatarGroup items=Vec::<AvatarGroupItem>::new() />",
        "## 常见用法（Common Usage）",
        "基础头像组 + overflow",
        "自定义 aria 与 class",
        "## 默认参数（Defaults）",
        "max: Option<usize>",
        "默认 `None`，归一化为 `4`",
        "size: AvatarSize",
        "默认 `AvatarSize::Md`",
        "## 进阶（Advanced，按需使用）",
        "先用上面的 Quick Start 和 Common Usage，再按需进入这些进阶能力。",
        "## docs-app 入口",
        "/#/components/avatar-group",
    ] {
        assert!(
            readme.contains(required),
            "AvatarGroup README should include beginner-friendly token `{required}`."
        );
    }

    let quick_start_pos = readme
        .find("## 先用起来（Quick Start / Hello World）")
        .expect("README should include quick-start section");
    let advanced_pos = readme
        .find("## 进阶（Advanced，按需使用）")
        .expect("README should include advanced section");
    assert!(
        quick_start_pos < advanced_pos,
        "README should present default quick-start path before advanced options."
    );

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::",
        "Signal<",
        "state=...",
    ] {
        assert!(
            !readme.contains(forbidden),
            "README starter path should not require internal layering token `{forbidden}`."
        );
    }

    for required in [
        "pub(super) fn avatar_group() -> AnyView",
        "slug=\"avatar-group\"",
        "Playground title=\"Hello World\"",
    ] {
        assert!(
            docs.contains(required),
            "docs-app should expose discoverable AvatarGroup doc entry via `{required}`."
        );
    }
}

#[test]
fn avatar_group_has_no_macro_micro_drag_state_machine_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/motion.rs")
            .exists(),
        "avatar-group has no drag interaction contract now; `src/motion.rs` should stay absent."
    );

    for forbidden in [
        "Dragging",
        "DragStart",
        "DragMove",
        "DragEnd",
        "Action::DragEnd",
        "request_animation_frame",
        "cancel_animation_frame",
        "pointermove",
        "mousemove",
        "on:pointermove",
        "on:mousemove",
        "on:pointerdown",
        "on:mousedown",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not contain macro/micro drag loop token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_two_pass_geometry_rectification_pipeline_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/motion.rs")
            .exists(),
        "avatar-group has no DOM-measure overlay contract now; `src/motion.rs` should stay absent."
    );

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ResizeObserver",
        "MutationObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "DOMRect",
        "Intent::Reposition",
        "Rectification",
        "tooltip",
        "popover",
        "menu-trigger",
        "placement",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include two-pass geometry marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_overlay_focus_stack_gc_contract_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/motion.rs")
            .exists(),
        "avatar-group has no layered overlay runtime now; `src/motion.rs` should stay absent."
    );

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should stay in static group-render path via `{required}`."
        );
    }

    for forbidden in [
        "NodeRef",
        "document.body",
        "document().body",
        "use_focus_trap",
        "focus_trap",
        "should_restore_focus",
        "restore_focus",
        "OverlayStack",
        "overlay_stack",
        "provide_overlay_stack",
        "use_overlay_stack",
        "data-ui-overlay-portal",
        "FallbackTo",
        "focus_manager",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not own overlay focus-stack/GC token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_foreign_zone_escape_hatch_integration_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    assert!(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/spec/foreign_zone_escape_hatches.md")
            .exists(),
        "foreign-zone governance spec should exist at docs/spec/foreign_zone_escape_hatches.md."
    );

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep plain declarative render path via `{required}`."
        );
    }

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "google.maps",
        "mapboxgl",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "yield_control",
        "cleanup_foreign",
        "chart_instance",
        "map_instance",
        "imperative_handle",
        "js_instance",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not carry foreign-zone escape-hatch token `{forbidden}` in current scope."
        );
    }

    for forbidden in [
        "chart:",
        "map:",
        "chart_instance",
        "map_instance",
        "imperative_handle",
        "web_sys::",
        "wasm_bindgen::JsValue",
    ] {
        assert!(
            !sig.contains(forbidden),
            "avatar-group public API should not leak imperative third-party handle token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let module = load_source("mod");
    let ui_components_root = load_source("ui_components_root");
    let headless_id_provider = load_source("headless_id_provider");

    // AvatarGroup does not allocate runtime IDs; hydration entropy axis stays N/A.
    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now",
        "now(",
        "Uuid::new_v4",
        "uuid::Uuid",
        "nanoid",
        "rand::",
        "thread_rng",
        "random::<",
        "random_uuid",
        "use_id(",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !module.contains(forbidden),
            "avatar-group should not introduce nondeterministic SSR/hydration entropy source `{forbidden}`."
        );
    }

    assert!(
        ui_components_root.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should remain the deterministic id-provider entrypoint."
    );
    assert!(
        ui_components_root.contains("#[prop(optional, default = 1)] id_seed: u64,"),
        "UiRoot should expose deterministic id seed prop for SSR/hydration alignment."
    );
    assert!(
        headless_id_provider.contains("pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider"),
        "headless id provider factory should remain available for deterministic ID wiring."
    );
}

#[test]
fn avatar_group_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean() {
    let manifest = load_source("manifest");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let platform_script = load_source("platform_script");
    let ui_headless_manifest = load_source("ui_headless_manifest");
    let ui_headless_lib = load_source("ui_headless_lib");
    let ui_components_manifest = load_source("ui_components_manifest");

    for required in [
        "cargo check -p ui",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-<your_component>,inject-css",
    ] {
        assert!(
            check2.contains(required),
            "checklist should keep explicit compile-only evidence command `{required}`."
        );
    }

    for required in [
        "cargo check -p ui --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            platform_script.contains(required),
            "platform check script should cover avatar-group cross-platform compile-only path `{required}`."
        );
    }

    assert!(
        manifest.contains(
            "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }"
        ),
        "avatar-group should keep explicit platform behavior via feature-gated leptos dependency."
    );
    assert!(
        ui_headless_manifest.contains("default = [\"web\"]")
            && ui_headless_manifest.contains("web = [\"leptos/csr\"]")
            && ui_headless_manifest.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless should keep web/ssr split under explicit feature management."
    );
    assert!(
        ui_headless_lib.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "ui-headless should guard invalid web+ssr co-enable path via compile_error."
    );
    assert!(
        ui_components_manifest.contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]")
            && ui_components_manifest.contains("web-sys = { version = \"0.3.85\""),
        "wasm-only browser dependency should stay behind explicit target cfg in ui."
    );

    for forbidden in [
        "web_sys::",
        "web-sys",
        "window.",
        "document.",
        "HtmlElement",
        "NodeRef",
        "js_sys::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "non-wasm avatar-group source should stay browser-object free; found `{forbidden}`."
        );
    }

    for forbidden in [
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(feature = \"ssr\")]",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should avoid accidental platform split marker `{forbidden}` in component layer."
        );
    }
}

#[test]
fn avatar_group_ui_headless_web_ssr_feature_mutex_contract_is_enforced() {
    let check2 = load_source("check2");
    let platform_script = load_source("platform_script");
    let ui_headless_manifest = load_source("ui_headless_manifest");
    let ui_headless_lib = load_source("ui_headless_lib");

    assert!(
        check2.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "checklist should explicitly track ui-headless web/ssr feature mutex contract."
    );

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless lib.rs should keep feature-mutex guard `{required}`."
        );
    }

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_manifest.contains(required),
            "ui-headless manifest should preserve split feature mapping `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "if cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script.contains(required),
            "platform check script should enforce ui-headless web/ssr mutex via `{required}`."
        );
    }
}

#[test]
fn avatar_group_ui_motion_non_wasm_noop_stub_contract_is_enforced() {
    let check2 = load_source("check2");
    let platform_script = load_source("platform_script");
    let ui_motion_lib = load_source("ui_motion_lib");
    let ui_motion_non_wasm_stub_test = load_source("ui_motion_non_wasm_stub_test");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    assert!(
        check2.contains("`ui-motion` 非 wasm 提供 no-op/stub"),
        "checklist should explicitly track ui-motion non-wasm no-op/stub contract."
    );

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm stub capability via `{required}`."
        );
    }

    for required in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "assert!(web::prefers_reduced_motion());",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test.contains(required),
            "ui-motion non-wasm stub regression should cover `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should protect ui-motion no-op/stub contract via `{required}`."
        );
    }

    for forbidden in ["ui_motion::", "attach_motion(", "SpringAnimator::new"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not assume runtime motion handles via `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_reduced_motion_ssr_wasm_branch_contract_is_explicitly_na_and_consistent() {
    let check2 = load_source("check2");
    let platform_script = load_source("platform_script");
    let ui_motion_lib = load_source("ui_motion_lib");
    let ui_motion_non_wasm_stub_test = load_source("ui_motion_non_wasm_stub_test");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    assert!(
        check2.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "checklist should explicitly track reduced-motion/SSR/wasm branch contract."
    );

    for required in [
        "cargo check -p ui --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract",
        "time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "scroll_area_reduced_motion_ssr_wasm_contract_is_consistent",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should lock reduced-motion/SSR/wasm coverage via `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm reduced-motion safe fallback `{required}`."
        );
    }

    for required in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test.contains(required),
            "ui-motion non-wasm stub test should cover `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::",
        "attach_motion(",
        "request_animation_frame",
        "animation:",
        "transition:",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should keep branch-neutral semantics without motion runtime token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_performance_governance_budget_is_defined_and_blocking() {
    let shell = load_source("docs_shell");
    let perf_probe = load_source("perf_probe");
    let coverage_e2e = load_source("coverage_e2e");
    let check2 = load_source("check2");
    let todo_plan = load_source("todo_plan");
    let perf_script = load_source("perf_script");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "\"avatar-group\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell.contains(required),
            "docs component shell should define avatar-group performance budget via `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe.contains(required),
            "UiPerfProbe should expose machine-readable perf marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_e2e.contains(required),
            "docs coverage should keep repeatable perf guard `{required}`."
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "Button",
        "Input",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group checklist should preserve performance-governance marker `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_plan.contains(required),
            "repo follow-up plan should keep performance governance TODO `{required}`."
        );
    }

    for required in [
        "cargo test -p ui --test avatar_group_semantics --no-default-features --features component-avatar_group,inject-css avatar_group_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(required),
            "performance gate script should include `{required}`."
        );
    }

    for required in [
        "logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "data-state=state.visual_state.as_str()",
        "data-visible-count=state.visible_count.to_string()",
        "data-overflow-count=state.overflow_count.to_string()",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view.contains(required),
            "avatar-group view should expose render/state attribution marker `{required}`."
        );
    }

    for forbidden in [
        "request_animation_frame",
        "set_interval(",
        "while ",
        "loop {",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should avoid uncontrolled runtime perf hazard `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_e2e_selectors_are_semantic_and_wasm_waits_are_stable_local() {
    let e2e = load_source("avatar_group_e2e_contract");
    let check2 = load_source("check2");

    for required in [
        "docs-app avatar-group uses semantic selectors with wasm-stable ready waits",
        "docs-app avatar-group keeps streaming/snapshot semantics readable and async-motion path explicitly N/A",
        "docs-app avatar-group key flow is repeatable with semantic checkpoints",
        "docs-app avatar-group interactive playground updates semantic state markers with live controls",
        "docs-app avatar-group source-first section exposes copy-ready starter and source anchors",
        "overflowBeforeReload",
        "overflowAfterReload",
        "await page.reload();",
        "toHaveAttribute(\"data-ui-state\", \"overflow\")",
        "/#/components/avatar-group",
        "body:not(:has(#boot))",
        "[data-component=\"avatar-group\"][data-slot=\"avatar-group\"]",
        "[data-slot=\"avatar-group\"][data-ui-schema=\"ui.avatar-group.agent.v1\"]",
        "[data-slot=\"avatar-group\"][data-state=\"overflow\"][data-has-overflow=\"true\"]",
        "[data-slot=\"avatar-group-overflow\"]",
        "[data-slot=\"avatar-group-item\"]",
        "[data-slot=\"avatar-group-streaming-policy\"]",
        "[data-slot=\"avatar-group-copy-ready-hint\"]",
        "[data-slot=\"avatar-group-source-first\"]",
        "[data-slot=\"snippet-copy-button\"]",
        "[data-slot=\"avatar-group-source-paths\"]",
        "[data-slot=\"avatar-group-source-sync-note\"]",
        "[data-slot=\"avatar-group-workbench-controls\"]",
        "[data-slot=\"avatar-group-workbench-configured\"] [data-slot=\"avatar-group\"]",
        "[data-slot=\"avatar-group-workbench-state\"]",
        "[data-slot=\"avatar-group-spec-preview-na\"]",
        "toHaveCount(0)",
    ] {
        assert!(
            e2e.contains(required),
            "avatar-group e2e contract should keep semantic selector/wait marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "section.playground",
        ".docs-page-title",
        "getByText(",
        "nth-child(",
    ] {
        assert!(
            !e2e.contains(forbidden),
            "avatar-group e2e contract should avoid fragile selector/wait token `{forbidden}`."
        );
    }

    for required in [
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记",
        "语义状态就绪而非固定 sleep",
        "关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程",
        "回归失败需可定位到具体语义契约断点",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2.contains(required),
            "avatar-group checklist should keep e2e selector stability marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_view_macro_complexity_is_bounded_and_semantically_split_for_items() {
    let view = load_source("view");

    assert!(
        view.contains("view! {"),
        "avatar-group view should keep explicit leptos render macro entry."
    );
    assert_eq!(
        view.matches("view! {").count(),
        2,
        "avatar-group should keep macro expansion bounded to root + item semantic fragments."
    );
    assert!(
        view.lines().count() <= 220,
        "avatar-group view.rs should stay compact; split semantic subrenders if it grows significantly."
    );

    for required in [
        "data-slot=\"avatar-group\"",
        "data-slot=\"avatar-group-item\"",
        "data-slot=\"avatar-group-overflow\"",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view.contains(required),
            "avatar-group should preserve semantic block split marker `{required}`."
        );
    }

    for forbidden in [
        "for item in",
        "match children",
        "<header",
        "<footer",
        "<article",
        "<section",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar-group should avoid heavy/expansion-prone view token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_prefers_functional_fragment_split_over_extra_component_defs_local() {
    let view = load_source("view");

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "avatar-group should keep a single public `#[component]` entry and avoid extra local components."
    );

    for required in [
        "fn render_avatar_group_item(",
        "index: usize,",
        "item: AvatarGroupItem,",
        "size: AvatarSize,",
        ") -> impl IntoView {",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "data-slot=\"avatar-group-item\"",
        "class_name=\"ui-avatar-group__avatar\"",
    ] {
        assert!(
            view.contains(required),
            "avatar-group should keep functional fragment split contract via `{required}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_avatar_group_item(",
        "#[component]\r\nfn render_avatar_group_item(",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar-group item fragment should remain a plain function, not a component `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_static_fragment_constantization_is_centralized_and_scope_bounded_local() {
    let view = load_source("view");

    for required in [
        "const OVERFLOW_VISIBLE_LABEL_PREFIX: &str = \"+\";",
        "fn render_avatar_group_overflow_label(overflow_count: usize) -> String {",
        "format!(\"{OVERFLOW_VISIBLE_LABEL_PREFIX}{overflow_count}\")",
        "let overflow_label = render_avatar_group_overflow_label(state.overflow_count);",
        "aria-label=overflow_aria_label.clone()",
    ] {
        assert!(
            view.contains(required),
            "avatar-group should centralize static overflow template path via `{required}`."
        );
    }

    for forbidden in [
        "format!(\"+{}\", state.overflow_count)",
        "<svg",
        "<footer",
        "inner_html=",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar-group should avoid scattered heavy static fragment token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_inner_html_contract_is_explicitly_na_and_user_input_injection_free_local() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for forbidden in [
        "inner_html=",
        "dangerouslySetInnerHTML",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "set_inner_html",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !sig.contains(forbidden),
            "avatar-group should stay free of unsafe HTML injection token `{forbidden}`."
        );
    }

    for required in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "aria-label=group_a11y.aria_label",
    ] {
        assert!(
            view.contains(required),
            "avatar-group should keep semantic text via typed/i18n path `{required}` instead of raw html injection."
        );
    }
}

#[test]
fn avatar_group_wasm_debug_contract_is_explicitly_na_and_feature_isolation_clean_local() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let manifest = load_source("manifest");
    let sig = function_signature(view, "AvatarGroup");

    for forbidden in [
        "tracing::",
        "trace!(",
        "debug!(",
        "console::",
        "console_log",
        "record_event",
        "event_log",
        "transition_log",
        "replay",
        "timeline",
        "devtools",
        "cfg(debug_assertions)",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !sig.contains(forbidden),
            "avatar-group should stay free of wasm debug/replay token `{forbidden}` in current scope."
        );
    }

    for required in [
        "default = []",
        "[features]",
        "items: Vec<AvatarGroupItem>",
        ".enumerate()",
    ] {
        assert!(
            manifest.contains(required) || view.contains(required) || sig.contains(required),
            "avatar-group should keep N/A debug baseline marker `{required}`."
        );
    }

    for forbidden_feature in [
        "wasm-debug",
        "avatar-group-wasm-debug",
        "avatar_group-wasm-debug",
    ] {
        assert!(
            !manifest.contains(forbidden_feature),
            "avatar-group crate should not expose production-facing debug feature `{forbidden_feature}`."
        );
    }
}

#[test]
fn avatar_group_dx_contract_prefers_playground_isolation_and_fast_style_feedback_local() {
    let view = load_source("view");
    let sig = function_signature(view, "AvatarGroup");
    let docs_display = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_playground = include_str!("../../../apps/docs-app/src/playground.rs");
    let dev_docs_script = include_str!("../../../scripts/dev-docs-app.sh");
    let dev_web_script = include_str!("../../../scripts/dev-web-demo.sh");

    for required in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/docs-app\"",
        "cd \"$ROOT_DIR/apps/web-demo\"",
    ] {
        assert!(
            dev_docs_script.contains(required) || dev_web_script.contains(required),
            "dev scripts should preserve fast feedback loop via `{required}`."
        );
    }

    for required in [
        "title=\"Hello World\" code_signal=hello_code",
        "title=\"Overflow Stack\" code_signal=overflow_code",
        "title=\"Custom Aria + Class\" code_signal=custom_code",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "<AvatarGroup items=empty_items.clone() />",
        "data-slot=\"avatar-group-streaming-policy\"",
        "data-slot=\"avatar-group-copy-ready-hint\"",
        "data-slot=\"avatar-group-source-first\"",
        "label=\"Copy avatar-group starter\".to_string()",
        "data-slot=\"avatar-group-source-prerequisites\"",
        "data-slot=\"avatar-group-source-paths\"",
    ] {
        assert!(
            docs_display.contains(required),
            "avatar-group docs should provide isolated demo/workbench entry via `{required}`."
        );
    }

    for required in [
        "<section class=section_class id=anchor_id data-slot=\"playground\">",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "prop:value=move || test_css.get()",
        "\"Restore original CSS\"",
    ] {
        assert!(
            docs_playground.contains(required),
            "docs playground should keep scoped style editing and context-preserving panel via `{required}`."
        );
    }

    for forbidden in [
        "signal(",
        "create_signal",
        "RwSignal<",
        "on:click",
        "on:input",
    ] {
        assert!(
            !view.contains(forbidden) && !sig.contains(forbidden),
            "avatar-group itself should remain stateless display component in current scope; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_engineering_unification_contract_is_explicitly_na_and_runtime_agnostic_local() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let manifest = load_source("manifest");
    let ui_components_manifest = load_source("ui_components_manifest");
    let sig = function_signature(view, "AvatarGroup");

    for forbidden in [
        "serde::",
        "Serialize",
        "Deserialize",
        "serde_json::",
        "tracing::",
        "trace!(",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "JoinHandle",
        "spawn_local",
        "async fn",
        ".await",
        "Future<",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !manifest.contains(forbidden)
                && !sig.contains(forbidden),
            "avatar-group should remain runtime-agnostic and avoid per-component infra token `{forbidden}`."
        );
    }

    for required in [
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\"]",
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "default = []",
    ] {
        assert!(
            ui_components_manifest.contains(required) || manifest.contains(required),
            "engineering unification baseline should stay centralized via `{required}`."
        );
    }

    for forbidden in [
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:serde\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:serde_json\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:tracing\"]",
    ] {
        assert!(
            !ui_components_manifest.contains(forbidden),
            "avatar-group feature chain should not leak infra coupling `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_dynamic_registration_protocol_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".enumerate()",
        "data-index=index",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep deterministic item order from typed Vec input via `{required}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "register_item",
        "unregister_item",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "IndexSet",
        "roving",
        "focus_next",
        "focus_prev",
        "tabs",
        "accordion",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include collection-registration protocol token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_slot_projection_lifecycle_protocol_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".take(state.visible_count)",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep direct eager rendering path via `{required}`."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "on_shown",
        "suspend_polling",
        "resume_polling",
        "pause_animation",
        "resume_animation",
        "set_interval",
        "set_timeout",
        "request_animation_frame",
        "cancel_animation_frame",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include slot-projection lifecycle token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_env_stream_subscription_pipeline_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep static prop-driven derivation path via `{required}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "matchMedia",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "debounce",
        "throttle",
        "on:resize",
        "window.add_event_listener",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include env-stream token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_event_light_cone_batch_protocol_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".take(state.visible_count)",
        ".enumerate()",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep direct list rendering path via `{required}`."
        );
    }

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "select_none",
        "selected_rows",
        "selected_columns",
        "row_selection",
        "column_selection",
        "Table",
        "Grid",
        "prop_drilling",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include event-light-cone token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_causality_bus_trace_pipeline_in_current_scope() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let sig = function_signature(view, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view.contains(required) || sig.contains(required),
            "avatar-group should keep direct local derivation path via `{required}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "Causality Bus",
        "CommandBus",
        "EventBus",
        "publish(",
        "broadcast(",
        "subscribe(",
        "subscriber",
        "dispatch_command",
        "derived_command",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "avatar-group should not include causality-bus token `{forbidden}` in current scope."
        );
    }
}
