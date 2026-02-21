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
fn color_loupe_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-loupe/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorLoupe internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_loupe_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-loupe/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_loupe.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let view_source = load_source("../../components/color-loupe/src/view.rs");

    for needle in [
        "pub use ui_state_primitives::color_loupe::{",
        "ColorLoupeState",
        "ColorLoupeStateInput",
        "DEFAULT_COLOR",
        "DEFAULT_ARIA_LABEL",
        "sanitize_color",
        "normalize_aria_label",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorLoupe logic should bridge ui-state-primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub struct ColorLoupeStateInput",
        "pub struct ColorLoupeState",
        "pub fn sanitize_percent(",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn position_bucket(",
        "pub fn vertical_bucket(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ColorLoupe state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod color_loupe;"),
        "ui-state-primitives should export `color_loupe` module."
    );

    for needle in [
        "logic::resolve_component_state(ColorLoupeLogicInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorSwatch",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorLoupe view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_loupe_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-loupe/src/view.rs");

    for attr in [
        "data-slot=\"color-loupe\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-x-bucket=move || state.get().x_bucket_attr",
        "data-y-bucket=move || state.get().y_bucket_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"color-loupe-bubble\"",
        "data-slot=\"color-loupe-checker\"",
        "data-slot=\"color-loupe-fill\"",
        "data-slot=\"color-loupe-tail\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorLoupe should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_loupe_a11y_i18n_locale_entry_uses_headless_contract() {
    let source = load_source("../../components/color-loupe/src/view.rs");

    for needle in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "role=\"img\"",
        "aria-label=move || aria_label.get_value()",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "ColorLoupe a11y/i18n locale entry should keep `{needle}`.",
        );
    }
}

#[test]
fn color_loupe_styles_include_open_disabled_position_and_custom_contracts() {
    let source = load_source("../../components/color-loupe/src/styles.rs");

    for selector in [
        ".ui-color-loupe",
        ".ui-color-loupe__bubble",
        ".ui-color-loupe__fill",
        ".ui-color-loupe__tail",
        ".ui-color-loupe--x-start",
        ".ui-color-loupe--x-center",
        ".ui-color-loupe--x-end",
        ".ui-color-loupe--y-start",
        ".ui-color-loupe--y-center",
        ".ui-color-loupe--y-end",
        ".ui-color-loupe--open",
        ".ui-color-loupe[data-open=\"true\"]",
        ".ui-color-loupe[data-state=\"open\"]",
        ".ui-color-loupe--disabled",
        ".ui-color-loupe[data-disabled=\"true\"]",
        ".ui-color-loupe--custom-class",
        ".ui-color-loupe[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorLoupe styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_loupe_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_loupe() -> AnyView",
        "title=\"ColorLoupe\"",
        "slug=\"color-loupe\"",
        "title=\"Open + Position Buckets\"",
        "title=\"Disabled + Custom Label + Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "color-loupe docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_loupe_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Open + Position Buckets\" code_signal=basic_code>",
        "id_base=\"docs-color-loupe-start\".to_string()",
        "id_base=\"docs-color-loupe-center\".to_string()",
        "id_base=\"docs-color-loupe-end\".to_string()",
        "is_open=true",
        "<Playground title=\"Disabled + Custom Label + Custom Class\" code_signal=states_code>",
        "id_base=\"docs-color-loupe-disabled\".to_string()",
        "is_disabled=true",
        "id_base=\"docs-color-loupe-custom\".to_string()",
        "aria_label=\"Accent loupe\".to_string()",
        "class_name=\"docs-color-loupe-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-loupe docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_loupe_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("../../components/color-loupe/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("../../components/color-loupe/src/view.rs");
    let styles_source = load_source("../../components/color-loupe/src/styles.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"color-loupe\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget token `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based performance attribution token `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
        "Button`、`Input`",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorLoupe checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "animation:",
        "var(--ui-text-field-motion-duration",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            view_source.contains(needle) || styles_source.contains(needle),
            "ColorLoupe should expose performance attribution marker `{needle}`."
        );
    }
}

#[test]
fn color_loupe_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "cargo test -p ui-components --test color_loupe_semantics color_loupe_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn color_loupe_check2_marks_core_sections_complete() {
    let source = load_source("../../components/color-loupe/src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/color_loupe.rs",
        "components/color-loupe/test/color_loupe_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "ColorLoupe check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn color_loupe_check2_has_no_unchecked_checklist_items() {
    let source = load_source("../../components/color-loupe/src/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "color_loupe check2 should not keep unchecked checklist items"
    );
}
