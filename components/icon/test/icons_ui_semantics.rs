use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_ui_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon/ui/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "IconsUi internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icons_ui_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icon/ui/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconsUi;"),
        "icons_ui module should export `IconsUi`."
    );
    assert!(
        module_source.contains("pub struct IconsUiStateInput"),
        "icons_ui module should expose `IconsUiStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use icons_ui::{IconsUi, IconsUiSize, IconsUiTone};"),
        "crate root should re-export `IconsUi` contracts."
    );
}

#[test]
fn icons_ui_logic_exposes_state_helpers() {
    let source = load_source("src/icon/ui/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_icon_reference(icon: String)",
        "pub fn default_ui_glyphs() -> Vec<IconsetGlyph>",
        "pub fn resolve_state(input: IconsUiStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconsUiState)",
    ] {
        assert!(
            source.contains(needle),
            "IconsUi logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn icons_ui_view_uses_logic_state_contracts() {
    let source = load_source("src/icon/ui/view.rs");

    for needle in [
        "pub fn IconsUi(",
        "logic::normalize_icon_reference(icon)",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(aria_label)",
        "logic::resolve_state(IconsUiStateInput {",
        "logic::compose_class_name(class_name_for_wrapper, state)",
        "logic::default_ui_glyphs()",
        "<Iconset",
        "iconset=\"ui\".to_string()",
        "data-slot=\"icons-ui\"",
        "data-state=state.state_attr",
        "data-icon-reference-source=state.icon_reference_source_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-glyph-source=state.glyph_source_attr",
        "data-size-source=state.size_source_attr",
        "data-tone-source=state.tone_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "IconsUi view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn icons_ui_styles_include_state_and_source_markers() {
    let source = load_source("src/icon/ui/styles.rs");

    for selector in [
        ".ui-icons-ui {",
        ".ui-icons-ui[data-state=\"disabled\"]",
        ".ui-icons-ui[data-state=\"decorative\"]",
        ".ui-icons-ui[data-icon-reference-source=\"default\"]",
        ".ui-icons-ui[data-icon-reference-source=\"explicit\"]",
        ".ui-icons-ui[data-icon-reference-source=\"prefixed\"]",
        ".ui-icons-ui[data-aria-source=\"custom\"]",
        ".ui-icons-ui[data-class-source=\"custom\"]",
        ".ui-icons-ui[data-glyph-source=\"custom\"]",
        ".ui-icons-ui[data-size-source=\"custom\"]",
        ".ui-icons-ui[data-tone-source=\"custom\"]",
        ".ui-icons-ui--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "IconsUi styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn icons_ui_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::icons_ui::styles::CSS);"),
        "ui css aggregator should include icons_ui styles."
    );
}

#[test]
fn icons_ui_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "pub(super) fn icons_ui() -> AnyView",
        "title=\"IconsUi\"",
        "slug=\"icons-ui\"",
        "State + Source Markers",
        "data-tone-source",
        "<IconsUi",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_ui docs page should contain `{needle}`."
        );
    }
}

#[test]
fn icons_ui_docs_default_and_custom_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "<Playground title=\"Built-in UI Glyphs\" code_signal=defaults_code>",
        "icon=\"check\".to_string()",
        "icon=\"close\".to_string()",
        "size=IconsUiSize::Md",
        "tone=IconsUiTone::Accent",
        "tone=IconsUiTone::Danger",
        "decorative=false",
        "<Playground title=\"Custom Registry Extension\" code_signal=custom_code>",
        "icon=\"ui:save\".to_string()",
        "IconsetGlyph::new(\"ui:save\", \"💾\")",
        ".with_aria_label(\"UI Save\")",
        "size=IconsUiSize::Lg",
        "tone=IconsUiTone::Default",
        "class_name=\"docs-icons-ui-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_ui docs default/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_ui_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "icon=\"help\".to_string()",
        "IconsetGlyph::new(\"ui:help\", \"?\")",
        ".with_aria_label(\"UI Help\")",
        "size=IconsUiSize::Lg",
        "tone=IconsUiTone::Muted",
        "decorative=false",
        "aria_label=\"Explicit UI help icon\".to_string()",
        "class_name=\"docs-icons-ui-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_ui docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_ui_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "pub(super) fn icons_ui() -> AnyView",
        "title=\"IconsUi\"",
        "slug=\"icons-ui\"",
        "description=\"baseline-compatible icons-ui wrapper with built-in UI icon registry defaults, namespace normalization, and Iconset accessibility/source-state contracts.\"",
        "<Playground title=\"Built-in UI Glyphs\" code_signal=defaults_code>",
        "<Playground title=\"Custom Registry Extension\" code_signal=custom_code>",
        "title=\"State + Source Markers\"",
        "<IconsUi",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_ui docs should include `{needle}` for icons_ui primary playground coverage.",
        );
    }
}

#[test]
fn icons_ui_docs_interactive_playground_exposes_config_code_css_test_sections() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view! {",
        "data-slot=\"icons-ui-workbench-controls\"",
        "id_base=\"docs-icons-ui-workbench-icon\".to_string()",
        "id_base=\"docs-icons-ui-workbench-size\".to_string()",
        "id_base=\"docs-icons-ui-workbench-tone\".to_string()",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/icon/ui/styles.rs\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_ui interactive docs playground should include `{needle}`.",
        );
    }
}

#[test]
fn icons_ui_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_icons_ui.rs");

    for needle in [
        "title=\"Built-in UI Glyphs\"",
        "icon=\"check\".to_string()",
        "icon=\"close\".to_string()",
        "size=IconsUiSize::Md",
        "tone=IconsUiTone::Accent",
        "tone=IconsUiTone::Danger",
        "title=\"Custom Registry Extension\"",
        "icon=\"ui:save\".to_string()",
        "IconsetGlyph::new(\"ui:save\", \"💾\")",
        ".with_aria_label(\"UI Save\")",
        "size=IconsUiSize::Lg",
        "tone=IconsUiTone::Default",
        "class_name=\"docs-icons-ui-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "icon=\"help\".to_string()",
        "IconsetGlyph::new(\"ui:help\", \"?\")",
        ".with_aria_label(\"UI Help\")",
        "tone=IconsUiTone::Muted",
        "aria_label=\"Explicit UI help icon\".to_string()",
        "class_name=\"docs-icons-ui-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_ui docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_ui_check2_marks_core_sections_complete() {
    let source = load_source("src/icon/ui/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "### 10. IconsUi 本轮验收证据",
        "component-icons_ui -> component-iconset",
        "component-iconset -> component-icon",
        "crates/ui/src/icon/ui/view.rs",
        "crates/ui/src/icon/set/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "IconsUi check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn icons_ui_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/icon/ui/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "icons_ui check2 should not keep unchecked checklist items"
    );
}
