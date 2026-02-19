use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn iconset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon/set/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Iconset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn iconset_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icon/set/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Iconset;"),
        "iconset module should export `Iconset`."
    );
    assert!(
        module_source.contains("pub struct IconsetGlyph"),
        "iconset module should expose `IconsetGlyph` data contract."
    );
    assert!(
        crate_source
            .contains("pub use iconset::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};"),
        "crate root should re-export Iconset contracts."
    );
}

#[test]
fn iconset_logic_exposes_state_helpers() {
    let source = load_source("src/icon/set/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn parse_icon_reference(icon: &str)",
        "pub fn resolve_iconset_namespace(",
        "pub fn glyph_matches(candidate_name: &str, iconset: &str, icon_name: &str)",
        "pub fn resolve_registry_glyph(",
        "pub fn resolve_accessible_label(",
        "pub fn resolve_state(input: IconsetStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconsetState)",
        "DEFAULT_ICONSET_NAMESPACE",
    ] {
        assert!(
            source.contains(needle),
            "Iconset logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn iconset_view_uses_logic_state_contracts() {
    let source = load_source("src/icon/set/view.rs");

    for needle in [
        "pub fn Iconset(",
        "logic::parse_icon_reference(&icon)",
        "logic::resolve_iconset_namespace(iconset_from_prop, iconset_from_icon)",
        "logic::resolve_registry_glyph(glyphs, &resolved_iconset, &icon_name)",
        "logic::resolve_state(IconsetStateInput {",
        "logic::resolve_accessible_label(decorative, custom_aria_label, registry_label, &icon_name)",
        "logic::compose_class_name(class_name, state)",
        "<Icon",
        "data-slot=\"iconset\"",
        "data-state=state.state_attr",
        "data-icon-source=state.icon_source_attr",
        "data-iconset-source=state.iconset_source_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-size-source=state.size_source_attr",
        "data-tone-source=state.tone_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Iconset view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn iconset_styles_include_state_and_source_markers() {
    let source = load_source("src/icon/set/styles.rs");

    for selector in [
        ".ui-iconset {",
        ".ui-iconset[data-state=\"disabled\"]",
        ".ui-iconset[data-state=\"decorative\"]",
        ".ui-iconset[data-state=\"fallback\"]",
        ".ui-iconset[data-icon-source=\"registry\"]",
        ".ui-iconset[data-icon-source=\"fallback\"]",
        ".ui-iconset[data-iconset-source=\"prop\"]",
        ".ui-iconset[data-iconset-source=\"icon\"]",
        ".ui-iconset[data-iconset-source=\"default\"]",
        ".ui-iconset[data-label-source=\"custom\"]",
        ".ui-iconset[data-label-source=\"registry\"]",
        ".ui-iconset[data-label-source=\"fallback\"]",
        ".ui-iconset[data-class-source=\"custom\"]",
        ".ui-iconset[data-size-source=\"custom\"]",
        ".ui-iconset[data-tone-source=\"custom\"]",
        ".ui-iconset--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Iconset styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn iconset_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::iconset::styles::CSS);"),
        "ui-components css aggregator should include iconset styles."
    );
}

#[test]
fn iconset_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "pub(super) fn iconset() -> AnyView",
        "title=\"Iconset\"",
        "slug=\"iconset\"",
        "State + Source Markers",
        "data-label-source",
        "<Iconset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_iconset docs page should contain `{needle}`."
        );
    }
}

#[test]
fn iconset_docs_default_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "<Playground title=\"Registry Namespace Resolution\" code_signal=registry_code>",
        "icon=\"workflow:check\".to_string()",
        "icon=\"workflow:alert\".to_string()",
        "glyphs=workflow_glyphs.clone()",
        "size=IconsetSize::Md",
        "tone=IconsetTone::Accent",
        "tone=IconsetTone::Danger",
        "decorative=false",
        "<Playground title=\"Fallback + Source State\" code_signal=fallback_code>",
        "icon=\"ui:unknown\".to_string()",
        "iconset=\"ui\".to_string()",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Muted",
        "class_name=\"docs-iconset-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs default playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn iconset_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "icon=\"workflow:check\".to_string()",
        "iconset=\"workflow\".to_string()",
        "glyphs=vec![",
        "IconsetGlyph::new(\"workflow:check\", \"✓\")",
        ".with_aria_label(\"Registry Check\")",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Danger",
        "aria_label=\"Explicit workflow check\".to_string()",
        "class_name=\"docs-iconset-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn iconset_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "pub(super) fn iconset() -> AnyView",
        "title=\"Iconset\"",
        "slug=\"iconset\"",
        "description=\"baseline-compatible Iconset registry wrapper for namespace + icon-name resolution, composed on Icon accessibility contracts with stable source markers.\"",
        "<Playground title=\"Registry Namespace Resolution\" code_signal=registry_code>",
        "<Playground title=\"Fallback + Source State\" code_signal=fallback_code>",
        "title=\"State + Source Markers\"",
        "<Iconset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_iconset docs should include `{needle}` for iconset primary playground coverage.",
        );
    }
}

#[test]
fn iconset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "title=\"Registry Namespace Resolution\"",
        "icon=\"workflow:check\".to_string()",
        "icon=\"workflow:alert\".to_string()",
        "glyphs=workflow_glyphs.clone()",
        "size=IconsetSize::Md",
        "tone=IconsetTone::Accent",
        "tone=IconsetTone::Danger",
        "title=\"Fallback + Source State\"",
        "icon=\"ui:unknown\".to_string()",
        "iconset=\"ui\".to_string()",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Muted",
        "class_name=\"docs-iconset-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "icon=\"workflow:check\".to_string()",
        "iconset=\"workflow\".to_string()",
        "IconsetGlyph::new(\"workflow:check\", \"✓\")",
        ".with_aria_label(\"Registry Check\")",
        "aria_label=\"Explicit workflow check\".to_string()",
        "class_name=\"docs-iconset-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn iconset_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/icon/set/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "iconset/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn iconset_check2_marks_semantics_first_testing_complete() {
    let check2_source = load_source("src/icon/set/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "iconset/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn iconset_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/icon/set/check2.md");

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
            "iconset/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn iconset_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/icon/set/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "iconset/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
