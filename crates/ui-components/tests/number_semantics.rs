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
fn number_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/number/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Number internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn number_module_exports_primitives_and_motion_contracts() {
    let source = load_source("src/text_input/number/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use logic::{NumberFormatOptions, format_static_number};",
        "pub use motion::SlidingNumberMotion;",
        "pub use view::{SlidingNumber, StaticNumber};",
    ] {
        assert!(
            source.contains(needle),
            "number module should expose `{needle}`.",
        );
    }

    for needle in [
        "pub mod text_input;",
        "pub use text_input::number::{",
        "NumberFormatOptions, SlidingNumber, SlidingNumberMotion, StaticNumber,",
    ] {
        assert!(
            crate_source.contains(needle),
            "crate root should include `{needle}` for number contracts.",
        );
    }
}

#[test]
fn number_logic_exposes_format_state_helpers() {
    let source = load_source("src/text_input/number/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_decimal_separator(",
        "pub fn resolve_thousand_separator(",
        "pub fn sanitize_decimal_places(",
        "pub fn sanitize_number(",
        "pub fn resolve_static_number_state(",
        "pub fn compose_static_number_class_name(",
        "pub fn resolve_sliding_number_state(",
        "pub fn compose_sliding_number_class_name(",
        "pub fn format_static_number(value: f64, options: NumberFormatOptions<'_>) -> String",
        "DEFAULT_DECIMAL_SEPARATOR",
    ] {
        assert!(
            source.contains(needle),
            "Number logic should include `{needle}` for centralized formatting/state derivation.",
        );
    }
}

#[test]
fn number_view_wires_motion_sanitization_and_state_markers() {
    let source = load_source("src/text_input/number/view.rs");

    for needle in [
        "let motion = crate::text_input::number::motion::sanitize_motion(motion);",
        "logic::resolve_static_number_state(logic::StaticNumberStateInput {",
        "logic::resolve_sliding_number_state(logic::SlidingNumberStateInput {",
        "data-slot=\"static-number\"",
        "data-slot=\"sliding-number\"",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-decimal-separator-source=state.decimal_separator_source_attr",
        "data-decimal-places-source=state.decimal_places_source_attr",
        "data-thousand-separator-source=state.thousand_separator_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Number view should include `{needle}` for stable state/motion marker contracts.",
        );
    }
}

#[test]
fn number_motion_contract_defaults_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/text_input/number/motion.rs");

    for needle in [
        "pub struct SlidingNumberMotion",
        "ui_motion::presets::spring_slide()",
        "animate: true",
        "motion.animate && !ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn supports_custom_spring_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Number motion contract should include `{needle}` for baseline-level spring/reduced-motion stability.",
        );
    }
}

#[test]
fn number_styles_and_css_aggregation_include_stable_selectors() {
    let styles_source = load_source("src/text_input/number/styles.rs");
    let css_source = load_source("src/css.rs");

    for selector in [
        ".ui-static-number",
        ".ui-static-number[data-sign=\"negative\"]",
        ".ui-static-number[data-decimal-separator-source=\"custom\"]",
        ".ui-static-number[data-thousand-separator-source=\"custom\"]",
        ".ui-sliding-number",
        ".ui-sliding-number[data-state=\"animated\"]",
        ".ui-sliding-number[data-motion-source=\"custom\"]",
        ".ui-sliding-number__roller",
        ".ui-sliding-number__stack",
    ] {
        assert!(
            styles_source.contains(selector),
            "Number styles should include `{selector}` as a stable marker.",
        );
    }

    assert!(
        css_source.contains("out.push_str(crate::number::styles::CSS);"),
        "ui-components css aggregator should include number styles.",
    );
}

#[test]
fn number_docs_page_contains_static_and_sliding_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "title=\"StaticNumber\"",
        "slug=\"static-number\"",
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"SlidingNumber\"",
        "slug=\"sliding-number\"",
        "<StaticNumber",
        "<SlidingNumber",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for number playground coverage.",
        );
    }
}

#[test]
fn number_docs_static_number_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "<Playground title=\"Formatting Matrix\" code_signal=matrix_code>",
        "number=12345.67",
        "number=-9876.5",
        "number=1000.0",
        "decimal_places=2",
        "decimal_places=1",
        "decimal_places=0",
        "thousand_separator=\",\".to_string()",
        "<Playground title=\"Custom Separators + Class\" code_signal=custom_code>",
        "number=42.123456789",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "number=f64::NAN",
        "class_name=\"docs-static-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number static docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_docs_sliding_number_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn sliding_number() -> AnyView",
        "<Playground title=\"Animated Matrix\" code_signal=matrix_code>",
        "number=number_signal",
        "decimal_places=2",
        "thousand_separator=\",\".to_string()",
        "decimal_places=0",
        "set_value.update(|v| *v += 250.0)",
        "set_value.update(|v| *v -= 100.0)",
        "<Playground title=\"Custom Separators + Motion + Class\" code_signal=custom_code>",
        "number=Signal::derive(|| 42123.456)",
        "decimal_separator=\",\".to_string()",
        "decimal_places=30",
        "thousand_separator=\" \".to_string()",
        "motion=ui_components::SlidingNumberMotion {",
        "animate: false,",
        "number=Signal::derive(|| f64::NAN)",
        "class_name=\"docs-sliding-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number sliding docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn number_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "title=\"StaticNumber\"",
        "slug=\"static-number\"",
        "<Playground title=\"Formatting Matrix\" code_signal=matrix_code>",
        "<Playground title=\"Custom Separators + Class\" code_signal=custom_code>",
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"SlidingNumber\"",
        "slug=\"sliding-number\"",
        "<Playground title=\"Animated Matrix\" code_signal=matrix_code>",
        "<Playground title=\"Custom Separators + Motion + Class\" code_signal=custom_code>",
        "<StaticNumber",
        "<SlidingNumber",
    ] {
        assert!(
            source.contains(needle),
            "display docs should include `{needle}` for number primary playground coverage.",
        );
    }
}

#[test]
fn number_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Formatting Matrix\"",
        "number=12345.67",
        "number=-9876.5",
        "decimal_places=2",
        "thousand_separator=\",\".to_string()",
        "title=\"Custom Separators + Class\"",
        "decimal_separator=\",\".to_string()",
        "thousand_separator=\" \".to_string()",
        "class_name=\"docs-static-number-custom\".to_string()",
        "title=\"Animated Matrix\"",
        "number=number_signal",
        "set_value.update(|v| *v += 250.0)",
        "set_value.update(|v| *v -= 100.0)",
        "title=\"Custom Separators + Motion + Class\"",
        "motion=ui_components::SlidingNumberMotion {",
        "animate: false,",
        "class_name=\"docs-sliding-number-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "number docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn number_docs_page_includes_button_style_workbench_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn static_number() -> AnyView",
        "pub(super) fn sliding_number() -> AnyView",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "description=\"Button-style playground with display/config/code/css-test panels for number formatting contracts.\"",
        "description=\"Button-style playground with display/config/code/css-test panels for sliding number motion and format contracts.\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"static-number-workbench-controls\"",
        "data-slot=\"sliding-number-workbench-controls\"",
        "\"展示区 · 对比矩阵\"",
    ] {
        assert!(
            source.contains(needle),
            "display number docs should include `{needle}` for button-style workbench coverage.",
        );
    }
}

#[test]
fn number_readme_covers_display_config_code_css_test_sections_and_comparisons() {
    let rel = "src/text_input/number/README.md";
    assert!(path_exists(rel), "number README should exist at `{rel}`.");
    let source = load_source(rel);

    for needle in [
        "# Number",
        "## Playground 展示区（Display / Config / Code / CSS Test）",
        "## 对比场景（多种情况）",
        "Display",
        "Config",
        "Code",
        "CSS Test",
        "StaticNumber",
        "SlidingNumber",
    ] {
        assert!(
            source.contains(needle),
            "number README should include `{needle}` for docs contract completeness.",
        );
    }
}

#[test]
fn number_feature_dependency_chain_supports_minimal_component_builds() {
    let cargo_toml = load_source("Cargo.toml");

    for needle in [
        "component-number = []",
        "component-number_field = [\"component-button\"]",
    ] {
        assert!(
            cargo_toml.contains(needle),
            "Number feature dependency chain should include `{needle}` for minimal-feature builds."
        );
    }
}

#[test]
fn number_view_mounts_locale_and_headless_a11y_contracts() {
    let source = load_source("src/text_input/number/view.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-slot=\"static-number\"",
        "data-slot=\"sliding-number\"",
    ] {
        assert!(
            source.contains(needle),
            "Number view should include `{needle}` for locale/a11y contract coverage."
        );
    }
}

#[test]
fn number_tree_shaking_boundaries_stay_feature_gated() {
    let lib_source = load_source("src/lib.rs");
    let domain_mod_source = load_source("src/text_input/mod.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        lib_source.contains("pub mod text_input;"),
        "ui-components lib boundary should expose `text_input` domain module."
    );
    assert!(
        lib_source.contains("pub use text_input::number::{")
            && lib_source
                .contains("NumberFormatOptions, SlidingNumber, SlidingNumberMotion, StaticNumber,"),
        "ui-components lib boundary should re-export number contracts from text_input domain."
    );
    assert!(
        domain_mod_source.contains("#[cfg(feature = \"component-number\")]")
            && domain_mod_source.contains("pub mod number;"),
        "text_input domain module should feature-gate `number`."
    );

    for needle in [
        "#[cfg(feature = \"component-number\")]",
        "out.push_str(crate::number::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css boundary should include `{needle}` for number feature gating."
        );
    }
}

#[test]
fn number_e2e_contract_uses_semantic_selectors_and_settled_waits() {
    let rel = "../../e2e/tests/docs_app_number_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "number E2E contract file should exist at `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"static-number\"]",
        "[data-component=\"sliding-number\"]",
        "data-slot=\"static-number\"",
        "data-slot=\"sliding-number\"",
        "data-slot=\"sliding-number-a11y-value\"",
    ] {
        assert!(
            source.contains(needle),
            "number E2E contract should include semantic selector/wait marker `{needle}`.",
        );
    }
}

#[test]
fn number_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source() {
    let source = load_source("../../e2e/tests/docs_app_number_contract.spec.mjs");

    for needle in [
        "page.keyboard.press(\"Tab\")",
        "await page.reload();",
        "Show code|Hide code",
        "data-copyable",
        "Copy to clipboard",
    ] {
        assert!(
            source.contains(needle),
            "number E2E contract should include `{needle}` for key-flow and source-copy coverage.",
        );
    }
}

#[test]
fn number_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/text_input/number/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] 语义测试优先",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`Number` 当前仅处理同步数值格式化与渲染状态标记",
        "`Number` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "number/check2.md should pin completion marker `{needle}`.",
        );
    }
}

#[test]
fn number_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/text_input/number/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "number/check2.md should mark anti-pattern guard `{needle}` as complete.",
        );
    }
}

#[test]
fn number_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/text_input/number/check2.md");

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
            "number/check2.md should keep final merge-gate marker `{needle}`.",
        );
    }
}

#[test]
fn number_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/text_input/number/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Number check2.md should not keep unchecked checklist items after completion."
    );
}
