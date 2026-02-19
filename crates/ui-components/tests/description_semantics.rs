use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn description_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_form/description/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Description internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn description_uses_logic_state_model() {
    let logic_source = load_source("src/field_form/description/logic.rs");
    let view_source = load_source("src/field_form/description/view.rs");

    for needle in [
        "pub enum DescriptionTone",
        "pub enum DescriptionElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_content(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Description logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_content(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(DescriptionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Description view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn description_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/field_form/description/view.rs");

    for attr in [
        "data-slot=\"description\"",
        "slot=\"description\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Description should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn description_styles_include_tone_state_and_markers() {
    let source = load_source("src/field_form/description/styles.rs");

    for selector in [
        ".ui-description--tone-default",
        ".ui-description[data-tone=\"default\"]",
        ".ui-description--tone-muted",
        ".ui-description[data-tone=\"muted\"]",
        ".ui-description--tone-negative",
        ".ui-description[data-tone=\"negative\"]",
        ".ui-description--disabled",
        ".ui-description[data-disabled=\"true\"]",
        ".ui-description--truncate",
        ".ui-description[data-truncate=\"true\"]",
        ".ui-description--custom-class",
        ".ui-description[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Description styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn description_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn description() -> AnyView",
        "title=\"Description\"",
        "slug=\"description\"",
        "description=\"baseline-style form description primitive with centralized tone/state/source contracts and stable slot semantics.\"",
        "<Playground title=\"Tone Variants\" code_signal=tone_code>",
        "<Playground title=\"Truncate + Element + Disabled\" code_signal=truncate_code>",
        "<Description",
        "DescriptionTone::Negative",
        "DescriptionElement::Span",
        "truncate=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for description primary coverage.",
        );
    }
}

#[test]
fn description_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "text=\"This appears below the field as guidance.\".to_string()",
        "tone=DescriptionTone::Default",
        "aria_label=\"Name helper\".to_string()",
        "text=\"Optional details are only visible to admins.\".to_string()",
        "tone=DescriptionTone::Muted",
        "text=\"Two-factor code expired. Request a new one.\".to_string()",
        "tone=DescriptionTone::Negative",
        "text=\"A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.\".to_string()",
        "element=DescriptionElement::Span",
        "truncate=true",
        "class_name=\"docs-description-custom\".to_string()",
        "text=\"Disabled helper text\".to_string()",
        "disabled=true",
        "class=\"docs-stack docs-description-limit\"",
    ] {
        assert!(
            source.contains(needle),
            "description docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/field_form/description/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_semantics_first_testing_complete() {
    let check2_source = load_source("src/field_form/description/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/field_form/description/check2.md");

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
            check2_source.contains(needle),
            "description/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/field_form/description/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "description/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
