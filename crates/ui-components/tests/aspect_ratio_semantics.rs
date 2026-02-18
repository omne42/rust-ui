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
fn aspect_ratio_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/aspect_ratio/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AspectRatio internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn aspect_ratio_uses_primitives_and_headless_contract_model() {
    let logic_source = load_source("src/aspect_ratio/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/aspect_ratio.rs");
    let headless_source = load_source("../ui-headless/src/aspect_ratio.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let view_source = load_source("src/aspect_ratio/view.rs");

    for needle in [
        "pub use ui_state_primitives::aspect_ratio::{",
        "AspectRatioPreset",
        "AspectRatioRadius",
        "AspectRatioState",
        "AspectRatioStateInput",
        "normalize_aria_label",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "AspectRatio logic should bridge primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum AspectRatioPreset",
        "pub enum AspectRatioRadius",
        "pub struct AspectRatioStateInput",
        "pub struct AspectRatioState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "AspectRatio state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub struct AspectRatioOptions",
        "pub struct AspectRatioAttrs",
        "pub struct AspectRatioContract",
        "pub fn use_aspect_ratio(options: AspectRatioOptions) -> AspectRatioContract",
        "region_attrs(options.aria_label, options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "AspectRatio headless contract should include `{needle}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod aspect_ratio;"),
        "ui-state-primitives should export `aspect_ratio` module."
    );
    assert!(
        headless_lib_source.contains("pub mod aspect_ratio;"),
        "ui-headless should export `aspect_ratio` module."
    );
    assert!(
        headless_lib_source.contains("use_aspect_ratio"),
        "ui-headless should re-export `use_aspect_ratio` contract."
    );

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(AspectRatioStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_aspect_ratio(AspectRatioOptions {",
        "state: state.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "AspectRatio view should compose logic/headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn aspect_ratio_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/aspect_ratio/view.rs");

    for attr in [
        "data-slot=\"aspect-ratio\"",
        "data-ratio=move || semantics.get().attrs.data_ratio",
        "data-radius=move || semantics.get().attrs.data_radius",
        "data-bordered=move || semantics.get().attrs.data_bordered",
        "data-fill=move || semantics.get().attrs.data_fill",
        "data-state=move || semantics.get().attrs.data_state",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-slot=\"aspect-ratio-inner\"",
    ] {
        assert!(
            source.contains(attr),
            "AspectRatio should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn aspect_ratio_component_files_follow_expected_layout_and_no_spec_file() {
    let mod_source = load_source("src/aspect_ratio/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::AspectRatio;",
    ] {
        assert!(
            mod_source.contains(needle),
            "aspect_ratio module should include `{needle}`."
        );
    }

    assert!(
        !path_exists("src/aspect_ratio/spec.rs"),
        "aspect_ratio should not introduce spec.rs for a simple layout primitive."
    );
}

#[test]
fn aspect_ratio_styles_include_ratio_and_frame_markers() {
    let source = load_source("src/aspect_ratio/styles.rs");

    for selector in [
        ".ui-aspect-ratio--ratio-square",
        ".ui-aspect-ratio[data-ratio=\"video\"]",
        ".ui-aspect-ratio--ratio-ultra-wide",
        ".ui-aspect-ratio--radius-md",
        ".ui-aspect-ratio[data-radius=\"full\"]",
        ".ui-aspect-ratio--bordered",
        ".ui-aspect-ratio[data-bordered=\"true\"]",
        ".ui-aspect-ratio--fill .ui-aspect-ratio__inner",
        ".ui-aspect-ratio[data-fill=\"true\"] .ui-aspect-ratio__inner > *",
        ".ui-aspect-ratio--custom-class",
        ".ui-aspect-ratio[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "AspectRatio styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn aspect_ratio_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn aspect_ratio() -> AnyView",
        "title=\"AspectRatio\"",
        "slug=\"aspect-ratio\"",
        "Playground title=\"Ratio Presets\"",
        "Playground title=\"Bordered + Fill + Custom Aria/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout-extra docs page should contain `{needle}` for AspectRatio.",
        );
    }
}

#[test]
fn aspect_ratio_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Ratio Presets\"",
        "ratio=AspectRatioPreset::Square",
        "ratio=AspectRatioPreset::Video",
        "ratio=AspectRatioPreset::Portrait",
        "radius=AspectRatioRadius::Sm",
        "radius=AspectRatioRadius::Md",
        "fill=true",
        "title=\"Bordered + Fill + Custom Aria/Class\"",
        "ratio=AspectRatioPreset::UltraWide",
        "radius=AspectRatioRadius::Lg",
        "bordered=true",
        "aria_label=\"Release trailer preview\".to_string()",
        "class_name=\"docs-aspect-ratio-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "aspect-ratio docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn aspect_ratio_check2_marks_core_sections_complete() {
    let source = load_source("src/aspect_ratio/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/aspect_ratio.rs",
        "ui-headless/src/aspect_ratio.rs",
        "crates/ui-components/tests/aspect_ratio_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "AspectRatio check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn aspect_ratio_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/aspect_ratio/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "aspect_ratio check2 should not keep unchecked checklist items"
    );
}
