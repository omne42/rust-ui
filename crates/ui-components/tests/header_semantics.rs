use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn header_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/header/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Header internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn header_module_exposes_motion_contracts() {
    let source = load_source("src/header/mod.rs");

    for needle in [
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::HeaderMotion;",
        "pub use view::Header;",
    ] {
        assert!(
            source.contains(needle),
            "Header module should include `{needle}` contract."
        );
    }
}

#[test]
fn header_logic_delegates_state_primitives() {
    let logic_source = load_source("src/header/logic.rs");
    let view_source = load_source("src/header/view.rs");

    for needle in [
        "pub use ui_state_primitives::header::{",
        "DEFAULT_ARIA_LABEL",
        "HeaderState",
        "HeaderStateInput",
        "HeaderTone",
        "normalize_aria_label",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
        "pub struct HeaderAgentContract",
        "pub fn resolve_agent_contract(state: HeaderState)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Header logic should include `{needle}` for state-primitive delegation."
        );
    }

    for forbidden in [
        "pub enum HeaderTone {",
        "pub struct HeaderStateInput",
        "pub struct HeaderState",
        "pub fn normalize_aria_label(",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Header logic should avoid local primitive implementation `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(HeaderStateInput {",
        "logic::resolve_agent_contract(state.get())",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Header view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn header_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/header/view.rs");

    for attr in [
        "data-slot=\"header\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source",
        "data-custom-motion=(motion_source == \"custom\").then_some(\"true\")",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "Header should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn header_styles_include_tone_border_and_custom_markers() {
    let source = load_source("src/header/styles.rs");

    for selector in [
        "--ui-header-motion-duration",
        ".ui-header--tone-default",
        ".ui-header[data-tone=\"default\"]",
        ".ui-header--tone-strong",
        ".ui-header[data-tone=\"strong\"]",
        ".ui-header--bordered",
        ".ui-header[data-bordered=\"true\"]",
        ".ui-header--custom-class",
        ".ui-header[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Header styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn header_motion_contract_exposes_sanitization_and_style_vars() {
    let source = load_source("src/header/motion.rs");

    for needle in [
        "pub struct HeaderMotion",
        "pub fn sanitize_motion(motion: HeaderMotion) -> HeaderMotion",
        "pub fn source_attr(motion: HeaderMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: HeaderMotion) -> String",
        "--ui-header-motion-duration",
    ] {
        assert!(
            source.contains(needle),
            "Header motion should include `{needle}`."
        );
    }
}

#[test]
fn header_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn header() -> AnyView",
        "title=\"Header\"",
        "slug=\"header\"",
        "Playground title=\"Semantic Header + Tone\"",
        "Playground title=\"Bordered + Custom Aria/Class\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "\"Source-first / Copy-Paste Ready\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Header.",
        );
    }
}

#[test]
fn header_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic Header + Tone\"",
        "<Header>",
        "<Header tone=HeaderTone::Strong>",
        "title=\"Bordered + Custom Aria/Class\"",
        "tone=HeaderTone::Strong",
        "bordered=true",
        "aria_label=\"Settings header\".to_string()",
        "class_name=\"docs-header-custom\".to_string()",
        "Header above content, matching baseline container semantics.",
        "let (interactive_strong_tone, set_interactive_strong_tone) = signal(false);",
        "let (interactive_bordered, set_interactive_bordered) = signal(false);",
        "data-slot=\"header-interactive-controls\"",
        "data-action=\"toggle-tone\"",
        "data-action=\"toggle-bordered\"",
        "class_name=\"docs-header-interactive\".to_string()",
        "data-slot=\"header-interactive-summary\"",
        "data-slot=\"header-source-first\"",
        "class_name=\"docs-header-source-copy\".to_string()",
        "\"crates/ui-components/src/header/motion.rs\"",
        "\"component-header\"",
        "\"inject-css\"",
    ] {
        assert!(
            source.contains(needle),
            "header docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn header_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_header_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "#/components/header",
        "[data-slot=\"header\"]",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "[data-action=\"toggle-tone\"]",
        "[data-action=\"toggle-bordered\"]",
    ] {
        assert!(
            source.contains(needle),
            "header e2e should include `{needle}` semantic contract selector/wait."
        );
    }
}

#[test]
fn header_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/header/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`Header` 无远程请求与异步状态轴",
        "Streaming Optional",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "header/check2.md should pin completion marker `{needle}`."
        );
    }
}

#[test]
fn header_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/header/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Header check2.md should not keep unchecked checklist items after completion."
    );
}

#[test]
fn header_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### Header 同步记录（2026-02-19）",
        "`Header` 继续保持语义容器头部原语定位",
        "component_doc!(\"Header\", \"header\", \"Layout\", layout::header)",
        "`#/components/header` 可索引访问",
        "Interactive Playground (State + Source Markers)",
        "Source-first / Copy-Paste Ready",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI strategy doc should keep header sync token `{needle}`."
        );
    }

    for needle in ["\"Header\"", "\"header\"", "layout::header"] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep header docs entry `{needle}`."
        );
    }
}
