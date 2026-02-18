use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn breadcrumb_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/breadcrumb/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Breadcrumb internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_is_exported_from_module_and_registered_in_crate() {
    let module_source = load_source("src/breadcrumb/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{"),
        "breadcrumb module should export breadcrumb primitive family.",
    );
    assert!(
        crate_source.contains("pub mod breadcrumb;"),
        "crate root should register breadcrumb module.",
    );
}

#[test]
fn breadcrumb_logic_exposes_state_helpers() {
    let source = load_source("src/breadcrumb/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_href(",
        "pub fn resolve_root_state(",
        "pub fn resolve_slot_state(",
        "pub fn resolve_link_state(",
        "pub fn resolve_separator_state(",
        "pub fn compose_class_name(",
        "pub fn compose_link_class_name(",
        "pub fn compose_separator_class_name(",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb logic should include `{needle}` for centralized state/source contracts.",
        );
    }
}

#[test]
fn breadcrumb_state_primitives_are_sourced_from_ui_state_primitives() {
    let source = load_source("src/breadcrumb/logic.rs");

    for needle in [
        "use ui_state_primitives::breadcrumb as breadcrumb_state;",
        "pub const DEFAULT_ARIA_LABEL: &str = breadcrumb_state::DEFAULT_ARIA_LABEL;",
        "breadcrumb_state::normalize_optional_text(",
        "breadcrumb_state::normalize_aria_label(",
        "breadcrumb_state::normalize_href(",
        "breadcrumb_state::resolve_root_state(",
        "breadcrumb_state::resolve_slot_state(",
        "breadcrumb_state::resolve_link_state(",
        "breadcrumb_state::resolve_separator_state(",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb state primitives should delegate to ui-state-primitives via `{needle}`.",
        );
    }

    for forbidden in [
        "if let Some(label) = normalize_optional_text(value) {",
        "state_attr: match (input.has_href, input.has_custom_class_name) {",
        "state_attr: match (input.has_custom_content, input.has_custom_class_name) {",
    ] {
        assert!(
            !source.contains(forbidden),
            "Breadcrumb should not reimplement state primitive logic in ui-components: `{forbidden}`.",
        );
    }
}

#[test]
fn breadcrumb_view_uses_logic_state_contracts() {
    let source = load_source("src/breadcrumb/view.rs");

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_root_state(BreadcrumbRootStateInput {",
        "logic::resolve_link_state(BreadcrumbLinkStateInput {",
        "logic::resolve_separator_state(BreadcrumbSeparatorStateInput {",
        "data-slot=\"breadcrumb\"",
        "data-state=state.state_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-href-state=state.href_state_attr",
        "data-content-source=state.content_source_attr",
        "data-label-source=\"default\"",
        "aria-current=\"page\"",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb view should expose stable marker contract `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_page_current_marker_stays_non_interactive() {
    let source = load_source("src/breadcrumb/view.rs");

    for disallowed in ["role=\"link\"", "aria-disabled=\"true\""] {
        assert!(
            !source.contains(disallowed),
            "Breadcrumb current page should be non-interactive text semantics; found `{disallowed}`.",
        );
    }
}

#[test]
fn breadcrumb_styles_include_state_and_accessibility_markers() {
    let source = load_source("src/breadcrumb/styles.rs");

    for selector in [
        ".ui-breadcrumb {",
        ".ui-breadcrumb[data-aria-source=\"custom\"]",
        ".ui-breadcrumb__list {",
        ".ui-breadcrumb__link--placeholder",
        ".ui-breadcrumb__link[data-href-state=\"absent\"]",
        ".ui-breadcrumb__separator--custom-content",
        ".ui-breadcrumb__separator[data-content-source=\"custom\"]",
        ".ui-breadcrumb__ellipsis-label",
        ".ui-breadcrumb--custom-class",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            source.contains(selector),
            "Breadcrumb styles should include `{selector}` as stable style markers.",
        );
    }
}

#[test]
fn breadcrumb_css_is_aggregated_in_component_layer() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::breadcrumb::styles::CSS);"),
        "ui-components css aggregator should include breadcrumb styles.",
    );
}

#[test]
fn breadcrumb_docs_page_contains_state_source_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb_primitives() -> AnyView",
        "title=\"BreadcrumbList\"",
        "slug=\"breadcrumb-list\"",
        "State + Source Markers",
        "data-aria-source",
    ] {
        assert!(
            source.contains(needle),
            "collections_breadcrumb_primitives docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_docs_overflow_playground_locks_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs",
    );

    for needle in [
        "title=\"Ellipsis Overflow\"",
        "<BreadcrumbEllipsis />",
        "<BreadcrumbLink href=\"/\">\"Home\"</BreadcrumbLink>",
        "<BreadcrumbPage>",
        "\"Current\"",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb docs overflow playground should contain `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_docs_state_source_playground_locks_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs",
    );

    for needle in [
        "title=\"State + Source Markers\"",
        "aria_label=\"Documentation navigation\".to_string()",
        "class_name=\"docs-breadcrumb-state\".to_string()",
        "<BreadcrumbList class_name=\"docs-breadcrumb-list\".to_string()>",
        "<BreadcrumbLink class_name=\"docs-breadcrumb-link\".to_string()>",
        "<BreadcrumbPage class_name=\"docs-breadcrumb-page\".to_string()>",
        "<span>\"→\"</span>",
        "Inspect root/link/separator markers like `data-state`, `data-aria-source`, `data-class-source`, `data-href-state`, and `data-content-source` for baseline-compatible breadcrumb contracts.",
    ] {
        assert!(
            source.contains(needle),
            "Breadcrumb docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs");

    for needle in [
        "pub(super) fn breadcrumb() -> AnyView",
        "title=\"Breadcrumb\"",
        "slug=\"breadcrumb\"",
        "title=\"Trail\"",
        "title=\"Label-Only + Empty\"",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumb docs page should contain `{needle}`.",
        );
    }

    let primitives_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb_primitives() -> AnyView",
        "title=\"BreadcrumbList\"",
        "slug=\"breadcrumb-list\"",
        "title=\"Link + Current Page\"",
        "title=\"Ellipsis Overflow\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            primitives_source.contains(needle),
            "breadcrumb primitives docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs");

    for needle in [
        "aria_label=\"Label-only trail\".to_string()",
        "aria_label=\"Empty trail\".to_string()",
        "\"all labels (no links)\"",
        "\"empty trail (0 items)\"",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumb docs playground should contain `{needle}`.",
        );
    }

    let primitives_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs",
    );

    for needle in [
        "title=\"State + Source Markers\"",
        "aria_label=\"Documentation navigation\".to_string()",
        "class_name=\"docs-breadcrumb-state\".to_string()",
        "<BreadcrumbList class_name=\"docs-breadcrumb-list\".to_string()>",
        "<BreadcrumbLink class_name=\"docs-breadcrumb-link\".to_string()>",
        "<BreadcrumbPage class_name=\"docs-breadcrumb-page\".to_string()>",
        "<span>\"→\"</span>",
    ] {
        assert!(
            primitives_source.contains(needle),
            "breadcrumb state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/breadcrumb/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_check2_marks_semantics_first_testing_complete() {
    let check2_source = load_source("src/breadcrumb/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/breadcrumb/check2.md");

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
            "breadcrumb/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn breadcrumb_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/breadcrumb/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "breadcrumb/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
