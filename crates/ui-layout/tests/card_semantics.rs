use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_card_docs_source() -> String {
    let parent = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let child = load_source("../../apps/docs-app/src/pages/components/pages/layout/card.rs");
    format!("{parent}\n{child}").replace(
        "pub(crate) fn card() -> AnyView {",
        "pub(super) fn card() -> AnyView {",
    )
}

#[test]
fn card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Card internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn card_uses_logic_state_model() {
    let view_source = load_source("src/card/view.rs");
    let logic_source = load_source("src/card/logic.rs");

    for needle in [
        "pub struct CardStateInput",
        "pub struct CardState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Card logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CardStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Card view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn card_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/card/view.rs");

    for attr in [
        "data-slot=\"card\"",
        "data-variant=state.variant_attr",
        "data-state=if state.is_padded { \"padded\" } else { \"flush\" }",
        "data-padded=state.is_padded.then_some(\"true\")",
        "data-flush=state.is_flush.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Card should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn card_styles_include_variant_and_padding_markers() {
    let source = load_source("src/card/styles.rs");

    for selector in [
        ".ui-card--padded",
        ".ui-card[data-padded=\"true\"]",
        ".ui-card--no-padding",
        ".ui-card[data-flush=\"true\"]",
        ".ui-card--variant-default",
        ".ui-card[data-variant=\"muted\"]",
        ".ui-card--variant-outline",
    ] {
        assert!(
            source.contains(selector),
            "Card styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn card_docs_page_covers_primary_playgrounds() {
    let source = load_card_docs_source();

    for needle in [
        "pub(super) fn card() -> AnyView",
        "title=\"Card\"",
        "slug=\"card\"",
        "Playground title=\"Variants\"",
        "Playground title=\"Padding States\"",
        "Playground title=\"Custom Class\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Card.",
        );
    }
}

#[test]
fn card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_card_docs_source();

    for needle in [
        "title=\"Variants\"",
        "<Card variant=CardVariant::Default>",
        "<Card variant=CardVariant::Muted>",
        "<Card variant=CardVariant::Outline>",
        "title=\"Padding States\"",
        "<Card padded=true>",
        "<Card padded=false>",
        "title=\"Custom Class\"",
        "<Card class_name=\"docs-card-custom\".to_string()>",
        "Verifies `data-custom-class` + class merge.",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"card-workbench-controls\"",
        "CardActualConfig",
    ] {
        assert!(
            source.contains(needle),
            "card docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn card_check2_keeps_card_scope_and_na_rationale_explicit() {
    let source = load_source("src/card/check2.md");

    for needle in [
        "已核验（card，2026-02-18）：本组件为静态展示容器",
        "不承载可控状态机、异步流程、overlay 焦点链路与流式正文渲染职责",
        "相关条目按 N/A 语义核验并保持契约可追溯。",
    ] {
        assert!(
            source.contains(needle),
            "card check2 should keep scoped rationale marker `{needle}`."
        );
    }
}

#[test]
fn card_check2_marks_semantics_streaming_and_docs_contract_complete() {
    let source = load_source("src/card/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
    ] {
        assert!(
            source.contains(needle),
            "card check2 should mark semantics/docs governance item `{needle}` as complete."
        );
    }
}

#[test]
fn card_check2_marks_final_merge_gates_complete() {
    let source = load_source("src/card/check2.md");

    for needle in [
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
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "card check2 should keep final merge gate marker `{needle}`."
        );
    }
}

#[test]
fn card_check2_has_no_remaining_unchecked_items() {
    let source = load_source("src/card/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "card/check2.md should not keep unchecked checklist items once governance is complete."
    );
}

#[test]
fn card_readme_covers_display_config_code_css_test_and_comparisons() {
    let source = load_source("src/card/README.md");

    for needle in [
        "## Playground 展示区（Display / Config / Code / CSS Test）",
        "## 多场景对比展示",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "card README should include `{needle}`."
        );
    }
}
