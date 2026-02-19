use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

#[test]
fn divider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/divider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Divider internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn divider_uses_primitives_and_headless_contract_model() {
    let view_source = load_source("src/divider/view.rs");
    let logic_source = load_source("src/divider/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/divider.rs");
    let headless_source = load_source("../ui-headless/src/divider.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "pub use ui_state_primitives::divider::{",
        "DividerOrientation",
        "DividerState",
        "DividerStateInput",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Divider logic should bridge primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum DividerOrientation",
        "pub struct DividerStateInput",
        "pub struct DividerState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Divider state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub struct DividerOptions",
        "pub struct DividerAttrs",
        "pub struct DividerContract",
        "pub fn use_divider(options: DividerOptions) -> DividerContract",
        "locale_attrs(options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "Divider headless contract should include `{needle}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod divider;"),
        "ui-state-primitives should export `divider` module."
    );
    assert!(
        headless_lib_source.contains("pub mod divider;"),
        "ui-headless should export `divider` module."
    );
    assert!(
        headless_lib_source.contains("use_divider"),
        "ui-headless should re-export `use_divider` contract."
    );

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(DividerStateInput {",
        "logic::compose_class_name(class_name, state)",
        "use_divider(DividerOptions { state, lang, dir })",
    ] {
        assert!(
            view_source.contains(needle),
            "Divider view should compose logic/headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn divider_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/divider/view.rs");

    for attr in [
        "data-slot=\"divider\"",
        "data-orientation=attrs.data_orientation",
        "data-state=attrs.data_state",
        "data-horizontal=attrs.data_horizontal",
        "data-vertical=attrs.data_vertical",
        "data-custom-class=attrs.data_custom_class",
        "role=attrs.role",
        "aria-orientation=attrs.aria_orientation",
        "lang=attrs.lang",
        "dir=attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "Divider should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn divider_styles_include_orientation_state_markers() {
    let source = load_source("src/divider/styles.rs");

    for selector in [
        ".ui-divider--horizontal",
        ".ui-divider[data-orientation=\"horizontal\"]",
        ".ui-divider[data-state=\"horizontal\"]",
        ".ui-divider[data-horizontal=\"true\"]",
        ".ui-divider--vertical",
        ".ui-divider[data-orientation=\"vertical\"]",
        ".ui-divider[data-state=\"vertical\"]",
        ".ui-divider[data-vertical=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Divider styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn divider_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn divider() -> AnyView",
        "title=\"Divider\"",
        "slug=\"divider\"",
        "Playground title=\"Orientation\"",
        "Playground title=\"Custom Class Marker\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Divider.",
        );
    }
}

#[test]
fn divider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Orientation\"",
        "<Divider />",
        "orientation=DividerOrientation::Vertical",
        "class_name=\"docs-divider-rail\".to_string()",
        "title=\"Custom Class Marker\"",
        "<Divider class_name=\"docs-divider-custom\".to_string() />",
        "class_name=\"docs-divider-custom docs-divider-rail\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "divider docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn divider_component_files_follow_expected_layout_and_no_spec_file() {
    let mod_source = load_source("src/divider/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "divider module should include `{needle}`."
        );
    }

    assert!(
        !path_exists("src/divider/spec.rs"),
        "divider should not introduce spec.rs for a simple layout primitive."
    );
}

#[test]
fn divider_check2_marks_core_sections_complete() {
    let source = load_source("src/divider/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-layout` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/divider.rs",
        "ui-headless/src/divider.rs",
        "crates/ui-layout/tests/divider_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "Divider check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn divider_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/divider/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "divider check2 should not keep unchecked checklist items"
    );
}
