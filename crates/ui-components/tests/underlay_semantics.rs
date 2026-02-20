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
fn underlay_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/underlay/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Underlay internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn underlay_module_exposes_state_primitive_and_motion_contracts() {
    let source = load_source("src/underlay/mod.rs");

    for needle in [
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::UnderlayMotion;",
        "pub use view::Underlay;",
        "pub use ui_state_primitives::underlay::{",
        "UnderlaySlot",
        "UnderlayPartStateInput",
        "UnderlayPartState",
    ] {
        assert!(
            source.contains(needle),
            "Underlay module should include `{needle}` contract."
        );
    }
}

#[test]
fn underlay_logic_concentrates_normalization_and_consumes_state_primitives() {
    let source = load_source("src/underlay/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/underlay.rs");

    for needle in [
        "pub enum UnderlayOpenMode",
        "pub struct UnderlayOpenStateInput",
        "pub fn normalize_open_state(input: UnderlayOpenStateInput)",
        "pub struct UnderlayFlagsInput",
        "pub fn normalize_flags(input: UnderlayFlagsInput)",
        "pub struct UnderlayViewStateInput",
        "pub fn resolve_view_state(input: UnderlayViewStateInput)",
        "pub enum UnderlayAgentSchemaVersion",
        "pub enum UnderlayAgentIntent",
        "pub enum UnderlayAgentAction",
        "pub enum UnderlayAgentStateAxis",
        "pub enum UnderlayAgentSource",
        "pub enum UnderlayAgentStreamSupport",
        "pub enum UnderlayAgentStreamFallback",
        "pub struct UnderlayAgentContract",
        "pub fn resolve_agent_contract(state: UnderlayViewState)",
        "pub use ui_state_primitives::underlay::{",
        "resolve_state",
        "pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Underlay logic should include `{needle}`."
        );
    }

    for forbidden in [
        "pub const DEFAULT_OPEN: bool = false;",
        "pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState",
    ] {
        assert!(
            !source.contains(forbidden),
            "Underlay logic should avoid local primitive implementation `{forbidden}`."
        );
    }

    assert!(
        primitive_source
            .contains("pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState"),
        "Underlay state machine should stay in ui-state-primitives."
    );
}

#[test]
fn underlay_view_mounts_headless_and_motion_contracts() {
    let source = load_source("src/underlay/view.rs");

    for needle in [
        "use ui_ai_runtime::{AiOutputStatus, AiRenderMode, use_ai_space_state};",
        "use ui_headless::{",
        "use_controllable_open_state_traced",
        "use_underlay",
        "logic::normalize_open_state(logic::UnderlayOpenStateInput {",
        "logic::normalize_flags(logic::UnderlayFlagsInput {",
        "logic::resolve_view_state(logic::UnderlayViewStateInput {",
        "logic::resolve_agent_contract(state.get())",
        "motion::attach_motion(root_ref, open_signal, motion);",
    ] {
        assert!(
            source.contains(needle),
            "Underlay view should include `{needle}`."
        );
    }

    assert!(
        source.contains("on:click=move |_| underlay.handlers.on_click.run(())")
            || source.contains("on:click=move |_| on_click.run(())"),
        "Underlay view should mount headless click handler from typed contract."
    );
}

#[test]
fn underlay_view_exposes_state_and_source_markers() {
    let source = load_source("src/underlay/view.rs");

    for attr in [
        "data-state=move || state.get().part.state_attr",
        "data-open=move || state.get().part.open_attr",
        "data-transparent=move || state.get().part.transparent_attr",
        "data-disabled=move || state.get().part.disabled_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-open-source=move || state.get().open_source_attr",
        "data-open-change-source=move || state.get().open_change_source_attr",
        "data-open-prop-source=move || state.get().open_prop_source_attr",
        "data-transparent-prop-source=move || state.get().transparent_prop_source_attr",
        "data-disabled-prop-source=move || state.get().disabled_prop_source_attr",
        "data-controlled=move || {",
        "data-uncontrolled=move || {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || stream_mode.get().as_str()",
        "data-ui-output-status=move || output_status.get().as_str()",
        "data-ui-capability-dismiss=move || {",
        "data-ui-capability-external-sync=move || {",
    ] {
        assert!(
            source.contains(attr),
            "Underlay view should expose `{attr}`."
        );
    }
}

#[test]
fn underlay_styles_are_token_first_and_state_driven() {
    let source = load_source("src/underlay/styles.rs");

    for needle in [
        "var(--ui-underlay-scrim-alpha",
        "var(--ui-underlay-backdrop-blur",
        "var(--ui-underlay-z-index",
        "var(--ui-underlay-runtime-duration",
        "var(--ui-underlay-transition-duration",
        "var(--ui-underlay-transition-easing",
        ".ui-underlay[data-state=\"open\"]",
        ".ui-underlay[data-close-mode=\"interactive\"]",
        ".ui-underlay[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(needle),
            "Underlay styles should include `{needle}` token/state contract."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn underlay_motion_contract_has_non_wasm_noop_path() {
    let source = load_source("src/underlay/motion.rs");

    for needle in [
        "pub struct UnderlayMotion",
        "pub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "UnderlayMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "Underlay motion should include `{needle}`."
        );
    }
}

#[test]
fn underlay_css_is_feature_gated_and_aggregated() {
    let css_source = load_source("src/css.rs");
    let lib_source = load_source("src/lib.rs");

    assert!(
        css_source.contains("out.push_str(crate::underlay::styles::CSS);"),
        "ui-components css aggregator should include underlay styles."
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"component-underlay\")]"),
        "underlay module should stay feature-gated."
    );
}

#[test]
fn underlay_docs_show_recommended_is_prefixed_api_and_source_markers() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn underlay() -> AnyView",
        "title=\"Underlay\"",
        "State + Source Markers",
        "is_open=",
        "is_transparent=true",
        "is_disabled=true",
        "on_open_change=Callback::new",
        "data-open-mode",
        "data-open-source",
        "AiSpace",
        "AiRenderMode::Streaming",
        "AiRenderMode::Snapshot",
        "AiOutputStatus::Draft",
        "AiOutputStatus::Verified",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "test_source_path=\"components/underlay/src/view.rs\".to_string()",
        "Copy-ready snippets auto-include `use leptos::prelude::*; use ui_components::*;`",
        "requires `ui-components` with `component-underlay`",
    ] {
        assert!(
            source.contains(needle),
            "underlay docs should include `{needle}`."
        );
    }
}

#[test]
fn underlay_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_underlay_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "#docs-underlay-ai[data-slot=\"underlay\"]",
        "[data-slot=\"underlay-ai-controls\"]",
        "[data-action=\"open\"]",
        "[data-action=\"toggle-mode\"]",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            source.contains(needle),
            "underlay e2e should include `{needle}` semantic contract selector/wait."
        );
    }
}

#[test]
fn underlay_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let check2_source = load_source("src/underlay/check2.md");
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let view_source = load_source("src/underlay/view.rs");

    for needle in [
        "HeroUI 对标文档与组件文档同步",
        "参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "underlay checklist should keep HeroUI-alignment governance marker `{needle}`."
        );
    }

    for needle in [
        "### Underlay 同步记录（2026-02-17）",
        "`Underlay` 维持 overlay primitive 定位",
        "`is_open/open/default_open/on_open_change`",
        "component_doc!(\"Underlay\", \"underlay\", \"Overlays\", overlays_extra::underlay)",
        "`#/components/underlay` 可索引访问",
        "`Scrim + Click To Close`、`Transparent + Disabled + Custom Class`、`State + Source Markers`、`LLM Render Modes (Snapshot + Streaming)`",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI strategy doc should keep underlay sync token `{needle}`."
        );
    }

    for needle in ["\"Underlay\"", "\"underlay\"", "overlays_extra::underlay"] {
        assert!(
            pages_source.contains(needle),
            "docs catalog entry should expose underlay token `{needle}`."
        );
    }

    for needle in [
        "slug=\"underlay\"",
        "title=\"Underlay\"",
        "Scrim + Click To Close",
        "LLM Render Modes (Snapshot + Streaming)",
        "is_open=",
        "on_open_change=",
    ] {
        assert!(
            docs_source.contains(needle),
            "underlay docs page should keep token `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_close: Option<OnPress>",
    ] {
        assert!(
            view_source.contains(needle),
            "underlay public API should keep token `{needle}` for docs/runtime sync."
        );
    }
}

#[test]
fn underlay_forbidden_antipatterns_are_guarded() {
    let primitive_source = load_source("../ui-state-primitives/src/underlay.rs");
    let headless_source = load_source("../ui-headless/src/underlay.rs");
    let logic_source = load_source("src/underlay/logic.rs");
    let view_source = load_source("src/underlay/view.rs");
    let mod_source = load_source("src/underlay/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "leptos",
        "web_sys",
        "NodeRef",
        "view!",
        "<div",
        "var(--ui-",
        ".ui-underlay",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives underlay must stay DOM/style free; found `{forbidden}`."
        );
    }

    for forbidden in [
        ".ui-underlay",
        "var(--ui-",
        "UnderlayMotion",
        "ui_motion",
        "keyframe",
        "spring",
        "transition-duration",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless underlay must not include visual/motion orchestration; found `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_open_state(logic::UnderlayOpenStateInput {",
        "logic::normalize_flags(logic::UnderlayFlagsInput {",
        "logic::resolve_view_state(logic::UnderlayViewStateInput {",
        "use_underlay(UnderlayOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "underlay view should consume normalized logic/headless contracts via `{needle}`."
        );
    }

    for forbidden in [
        "resolve_state(UnderlayPartStateInput {",
        "pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState",
        "pub const DEFAULT_OPEN: bool = false;",
        "pub const DEFAULT_TRANSPARENT: bool = false;",
        "pub const DEFAULT_DISABLED: bool = false;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "underlay view should not re-implement primitive state logic; found `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_transparent: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "data-open-prop-source=move || state.get().open_prop_source_attr",
        "data-transparent-prop-source=move || state.get().transparent_prop_source_attr",
        "data-disabled-prop-source=move || state.get().disabled_prop_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "underlay API naming + source contract should include `{needle}`."
        );
    }

    for forbidden in [
        "labels + children",
        "titles + panels",
        "Vec<String>, Vec<AnyView>",
        "zip(labels",
        "zip(titles",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "underlay docs/API should avoid parallel-array implicit contracts; found `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden),
            "underlay public API must not leak platform-private types; found `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::underlay::{")
            && logic_source.contains("resolve_state,"),
        "underlay logic should consume reusable primitive contract from ui-state-primitives."
    );
    assert!(
        !logic_source
            .contains("pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState"),
        "underlay logic must not keep a local copy of the reusable primitive state machine."
    );
}

#[test]
fn underlay_merge_gate_verdicts_are_explicit_and_fully_completed() {
    let check2_source = load_source("src/underlay/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
    ] {
        assert!(
            check2_source.contains(needle),
            "underlay merge-gate checklist should include `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。"),
        "merge gate should be fully completed."
    );
}

#[test]
fn underlay_checklist_sections_one_to_nine_have_no_unchecked_items() {
    let check2_source = load_source("src/underlay/check2.md");

    for needle in [
        "### 1. 大骨架（架构边界与层职责）",
        "### 2. 小骨架（API 设计检查 + 状态管理检查）",
        "### 3. 实现细节（A11y / i18n-l10n / 可观测 / 样式与动效）",
        "### 4. SSR / 跨平台 / WASM / 性能 / 工程能力",
        "### 5. 文件落点检查（必须提及）",
        "### 6. AI 原生能力（Agent Contract + 流式）",
        "### 7. 测试与文档（验证闭环）",
        "### 8. 明确禁止的反模式",
        "### 9. 合并门禁（最终裁决）",
    ] {
        assert!(
            check2_source.contains(needle),
            "underlay checklist should include section `{needle}`."
        );
    }

    assert!(
        !check2_source.contains("- [ ]"),
        "underlay checklist must not keep unchecked items after completion."
    );
}
