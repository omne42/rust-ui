use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    if rel_path == "../../apps/docs-app/src/pages/components/pages/layout.rs" {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let parent_path =
            manifest_dir.join("../../apps/docs-app/src/pages/components/pages/layout.rs");
        let child_path =
            manifest_dir.join("../../apps/docs-app/src/pages/components/pages/layout/footer.rs");
        let parent = fs::read_to_string(&parent_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {parent_path:?}: {e}"));
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        return format!("{parent}\n{child}").replace(
            "pub(crate) fn footer() -> AnyView {",
            "pub(super) fn footer() -> AnyView {",
        );
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn footer_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/footer/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Footer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn footer_uses_logic_state_model() {
    let logic_source = load_source("src/footer/logic.rs");
    let view_source = load_source("src/footer/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/footer.rs");

    for needle in [
        "pub use ui_state_primitives::footer::{",
        "FooterState",
        "FooterStateInput",
        "FooterTone",
        "compose_class_name",
        "normalize_aria_label",
        "normalize_optional_text",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "Footer logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "pub struct FooterStateInput",
        "pub struct FooterState",
        "pub fn resolve_state(input: FooterStateInput) -> FooterState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Footer primitive should include `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(FooterStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Footer view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn footer_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/footer/view.rs");

    for attr in [
        "data-slot=\"footer\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Footer should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn footer_styles_include_tone_border_and_custom_markers() {
    let source = load_source("src/footer/styles.rs");

    for selector in [
        ".ui-footer--tone-default",
        ".ui-footer[data-tone=\"default\"]",
        ".ui-footer--tone-muted",
        ".ui-footer[data-tone=\"muted\"]",
        ".ui-footer--bordered",
        ".ui-footer[data-bordered=\"true\"]",
        ".ui-footer--custom-class",
        ".ui-footer[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Footer styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn footer_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn footer() -> AnyView",
        "title=\"Footer\"",
        "slug=\"footer\"",
        "Playground title=\"Semantic Footer + Tone\"",
        "Playground title=\"Bordered + Custom Aria/Class\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Footer.",
        );
    }
}

#[test]
fn footer_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic Footer + Tone\"",
        "<Footer>",
        "<Footer tone=FooterTone::Muted>",
        "title=\"Bordered + Custom Aria/Class\"",
        "<Header bordered=true>",
        "<Content padded=true>",
        "tone=FooterTone::Muted",
        "bordered=true",
        "aria_label=\"Settings footer\".to_string()",
        "class_name=\"docs-footer-custom\".to_string()",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=footer_test_css_source",
        "test_config_signal=footer_actual_config",
        "controls=move || view! {",
        "FooterActualConfig",
        "\"comparison: configured(tone={}, bordered={}, custom_aria={}, custom_class={}) vs reference(default)\"",
    ] {
        assert!(
            source.contains(needle),
            "footer docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn footer_readme_documents_display_config_code_css_test_sections() {
    let source = load_source("src/footer/README.md");

    for needle in [
        "## Playground 展示区（展示 / config / code / css test）",
        "Workbench (Display + Config + Code + CSS Test)",
        "Config：Workbench test 面板输出 `FooterActualConfig`",
        "## 对比场景",
        "## Source-first / Copy-Paste Ready",
    ] {
        assert!(
            source.contains(needle),
            "footer README should include `{needle}`.",
        );
    }
}

#[test]
fn footer_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/footer/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-layout` 定义",
        "- [x] API 命名契约统一",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] 语义测试优先",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`Footer` 无远程请求与异步状态轴",
        "Streaming Optional",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "footer/check2.md should pin completion marker `{needle}`."
        );
    }
}

#[test]
fn footer_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/footer/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Footer check2.md should not keep unchecked checklist items after completion."
    );
}
