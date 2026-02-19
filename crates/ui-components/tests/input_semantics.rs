use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_clears_on_escape_when_clearable_and_not_empty() {
    let source = load_source("src/text_input/input/view.rs");

    assert!(
        source.contains("use_clearable_text_field"),
        "Input should consume a headless clearable text-field contract for Escape semantics."
    );
    assert!(
        source.contains("ClearableTextFieldOptions"),
        "Input should configure clearable text-field semantics through typed headless options."
    );
    assert!(
        source.contains("is_clearable,"),
        "Input should forward `is_clearable` into the headless keyboard contract."
    );
    assert!(
        source.contains("is_empty: Signal::derive(move || is_empty.get())"),
        "Input should forward emptiness state into the headless clearability contract."
    );
    assert!(
        source.contains("clearable.handlers.on_key_down.run(ev.key())"),
        "Input should delegate Escape key handling to the headless handler."
    );
}

#[test]
fn input_escape_clear_stops_propagation() {
    let source = load_source("src/text_input/input/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "Input should stop Escape propagation when clearing (baseline parity: Escape clears without dismissing parent overlays)."
    );
}

#[test]
fn input_mounts_locale_attrs_from_headless_a11y_helpers() {
    let source = load_source("src/text_input/input/view.rs");

    for needle in [
        "A11yDirection",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "Input should include `{needle}` to expose lang/dir via headless a11y locale attrs."
        );
    }
}

#[test]
fn input_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/text_input/input/view.rs");

    assert!(
        source.contains("exclude_from_tab_order=true"),
        "Input clear button should be excluded from tab order to avoid extra Tab stops."
    );
}

#[test]
fn input_clear_button_is_presence_safe() {
    let source = load_source("src/text_input/input/view.rs");

    assert!(
        source.contains("is_visible=Signal::derive(move || view_state.get().show_clear)"),
        "Input should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || view_state.get().show_clear"),
        "Input should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn input_attaches_clear_motion_driver() {
    let source = load_source("src/text_input/input/view.rs");

    assert!(
        source.contains("attach_clear_button_motion"),
        "Input should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn input_styles_include_motion_marker_contracts() {
    let source = load_source("src/text_input/input/styles.rs");

    for selector in [
        ".ui-input[data-motion-source=\"custom\"]",
        ".ui-input[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Input styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn input_styles_define_clear_motion_css_vars() {
    let source = load_source("src/text_input/input/styles.rs");

    assert!(
        source.contains("--ui-input-clear-opacity"),
        "Input styles should define `--ui-input-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-input-clear-scale"),
        "Input styles should define `--ui-input-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn input_motion_uses_spring_animator() {
    let source = load_source("src/text_input/input/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Input motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn input_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/input/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-read-only",
        "data-required",
        "data-motion-source",
        "data-custom-motion",
    ] {
        assert!(
            source.contains(attr),
            "Input should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn input_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/text_input/input/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: InputMotion) -> InputMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hidden_scale:",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "Input motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn input_state_primitives_are_sourced_from_ui_state_primitives() {
    let logic_source = load_source("src/text_input/input/logic.rs");

    for needle in [
        "pub use ui_state_primitives::input::{",
        "InputLogicState",
        "resolve_clear_aria_label",
        "resolve_view_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "Input logic should consume `{needle}` from ui-state-primitives instead of reimplementing a local state primitive."
        );
    }
}

#[test]
fn input_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn input() -> AnyView",
        "title=\"Input\"",
        "slug=\"input\"",
        "description=\"baseline-style text input with label, description/error, and clear button.\"",
        "<Playground title=\"Clearable + validation\" code_signal=code>",
        "<Input",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for input primary playground coverage.",
        );
    }
}

#[test]
fn input_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Clearable + validation\"",
        "id=\"docs-input\".to_string()",
        "label=\"Name\".to_string()",
        "is_clearable=true",
        "invalid=Signal::derive(move || invalid.get())",
        "description=\"Try toggling invalid.\".to_string()",
        "error=\"This field is invalid.\".to_string()",
        "size=InputSize::Md",
        "variant=InputVariant::Bordered",
        "on_press=Callback::new(move |_| set_invalid.update(|v| *v = !*v))",
    ] {
        assert!(
            source.contains(needle),
            "input docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn input_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("src/text_input/input/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/text_input/input/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
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
        "\"mount-plus-budget\"",
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
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Input checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_view_state(",
        "motion::attach_clear_button_motion(",
        "data-focused=move || is_focused.get().then_some(\"true\")",
        "data-filled=move || view_state.get().is_filled.then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "Input view should expose state/render/style/motion attribution marker `{needle}`."
        );
    }
}

#[test]
fn input_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn text_input_family_label_meta_text_metrics_use_typography_tokens() {
    let style_files = [
        "src/text_input/input/styles.rs",
        "src/text_input/text_field/styles.rs",
        "src/text_input/search_field/styles.rs",
        "src/text_input/textarea/styles.rs",
        "src/text_input/text_area/styles.rs",
    ];

    for style_file in style_files {
        let source = load_source(style_file);

        assert!(
            source.contains("var(--ui-line-height-150)"),
            "{style_file} should use `--ui-line-height-150` for control/label text metrics."
        );
        assert!(
            source.contains("var(--ui-line-height-100)"),
            "{style_file} should use `--ui-line-height-100` for meta text metrics."
        );
        assert!(
            !source.contains("line-height: 1.2;"),
            "{style_file} should not hardcode `line-height: 1.2;`."
        );
        assert!(
            !source.contains("line-height: 1.3;"),
            "{style_file} should not hardcode `line-height: 1.3;`."
        );
    }
}

#[test]
fn input_check2_has_no_unchecked_items_after_verification() {
    let source = load_source("src/text_input/input/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "input/check2.md should not keep unchecked checklist items after completion."
    );
}
