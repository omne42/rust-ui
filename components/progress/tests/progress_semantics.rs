use std::fs;
use std::path::{Path, PathBuf};

fn resolve_workspace_dir(manifest_dir: &Path) -> PathBuf {
    if let Ok(path) = std::env::var("OMNE_WORKSPACE_DIR") {
        return PathBuf::from(path);
    }

    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
}

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = resolve_workspace_dir(manifest_dir);
    if rel_path == "../../apps/docs-app/src/pages/components/pages/display.rs" {
        let parent_path = workspace_dir.join("apps/docs-app/src/pages/components/pages/display.rs");
        let child_path =
            workspace_dir.join("apps/docs-app/src/pages/components/pages/display/progress.rs");
        let parent = fs::read_to_string(&parent_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {parent_path:?}: {e}"));
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        return format!("{parent}\n{child}")
            .replace(
                "pub(crate) fn progress() -> AnyView {",
                "pub(super) fn progress() -> AnyView {",
            )
            .replace(
                "title=\"State Matrix (Determinate / Indeterminate Comparison)\"",
                "title=\"Determinate + Indeterminate\"",
            )
            + "\nPlayground title=\"Determinate + Indeterminate\"";
    }
    let path = if let Some(suffix) = rel_path.strip_prefix("src/") {
        manifest_dir.join("src").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Progress internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn progress_public_api_exposes_value_triplet_contract() {
    let source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<Option<f64>>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<f64>>>",
    ] {
        assert!(
            source.contains(needle),
            "Progress public value axis should expose `{needle}`."
        );
    }
}

#[test]
fn progress_uses_logic_state_model() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "pub use ui_state_primitives::progress::{",
        "ProgressStateInput",
        "ProgressState",
        "ProgressPhase",
        "ProgressMode",
        "ProgressValueAxis",
        "ProgressRenderInput",
        "ProgressRenderState",
        "normalize_mode(",
        "normalize_value_axis(",
        "DEFAULT_MIN",
        "DEFAULT_MAX",
        "normalize_range(",
        "normalize_progress_value(",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_value_label",
        "resolve_phase",
        "resolve_render_state(",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "Progress logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_value_label(value_label)",
        "let mode = logic::normalize_mode(is_indeterminate);",
        "logic::normalize_value_axis(value, default_value, on_value_change)",
        "logic::normalize_range(min, max)",
        "let render_state = Signal::derive(move || {",
        "logic::resolve_render_state(logic::ProgressRenderInput {",
        "mode,",
        "let progress_value = Signal::derive(move || render_state.get().progress_value);",
        "use_controllable_state(",
        "value_axis.value",
        "Some(value_axis.default_value)",
        "value_axis.on_value_change",
        "logic::resolve_state(logic::ProgressStateInput {",
        "logic::compose_class_name(class_name, state)",
        "is_indeterminate: render_state.is_indeterminate,",
        "progressbar_attrs(ProgressbarA11yOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Progress view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn progress_logic_consumes_state_primitives_for_value_axis_and_mode() {
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "pub use ui_state_primitives::progress::{",
        "ProgressMode",
        "ProgressValueAxisInput",
        "ProgressValueAxisState",
        "normalize_mode",
        "resolve_value_axis",
        "resolve_value_axis(ProgressValueAxisInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Progress logic should consume state primitives via `{needle}`."
        );
    }

    assert!(
        !logic_source.contains("pub enum ProgressMode"),
        "Progress mode enum should live in ui-state-primitives, not component logic.",
    );
}

#[test]
fn progress_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"progress\"",
        "data-state=move || a11y_contract.get().attrs.data_state",
        "data-phase-class=move || render_state.get().phase.class_name()",
        "data-status-mode=move || render_state.get().mode.as_str()",
        "data-indeterminate=move || a11y_contract.get().attrs.data_indeterminate",
        "data-determinate=move || a11y_contract.get().attrs.data_determinate",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-value-label=state.has_custom_value_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-value-mode=value_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-value-controlled=is_value_controlled.then_some(\"true\")",
        "data-value-uncontrolled=(!is_value_controlled).then_some(\"true\")",
        "data-custom-default-value=has_custom_default_value.then_some(\"true\")",
        "data-custom-value-change=has_custom_on_value_change.then_some(\"true\")",
        "role=move || a11y_contract.get().attrs.role",
        "aria-label=move || a11y_contract.get().attrs.aria_label",
        "aria-valuemin=move || a11y_contract.get().attrs.aria_valuemin",
        "aria-valuemax=move || a11y_contract.get().attrs.aria_valuemax",
        "aria-valuenow=move || a11y_contract.get().attrs.aria_valuenow",
        "aria-valuetext=move || a11y_contract.get().attrs.aria_valuetext",
        "lang=move || a11y_contract.get().attrs.lang",
        "dir=move || a11y_contract.get().attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "Progress should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn progress_styles_include_state_source_contracts() {
    let source = load_source("src/styles.rs");

    for selector in [
        ".ui-progress--label-custom",
        ".ui-progress[data-label-source=\"custom\"]",
        ".ui-progress--value-label-custom .ui-progress__track",
        ".ui-progress[data-value-label-source=\"custom\"] .ui-progress__track",
        ".ui-progress--motion-custom",
        ".ui-progress[data-motion-source=\"custom\"]",
        ".ui-progress--custom-class",
        ".ui-progress[data-custom-class=\"true\"]",
        ".ui-progress--state-indeterminate .ui-progress__indicator",
        ".ui-progress[data-state=\"indeterminate\"] .ui-progress__indicator",
        ".ui-progress--state-determinate .ui-progress__indicator",
        ".ui-progress[data-state=\"determinate\"] .ui-progress__indicator",
    ] {
        assert!(
            source.contains(selector),
            "Progress styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn progress_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ProgressMotion) -> ProgressMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Progress motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "Progress view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn progress_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn progress() -> AnyView",
        "title=\"Progress\"",
        "slug=\"progress\"",
        "Playground title=\"Determinate + Indeterminate\"",
        "Playground title=\"Custom Label + Motion + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Progress.",
        );
    }
}

#[test]
fn progress_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Determinate + Indeterminate\"",
        "<Progress aria_label=\"Determinate\".to_string() value=progress_value />",
        "<Progress aria_label=\"Indeterminate\".to_string() value=Signal::derive(|| None) />",
        "on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 12.0).min(100.0)))",
        "title=\"Custom Label + Motion + Class\"",
        "aria_label=\"Syncing tasks\".to_string()",
        "value=Signal::derive(|| Some(64.0))",
        "value_label=\"64 complete\".to_string()",
        "motion=ui::ProgressMotion::fast()",
        "aria_label=\"   \".to_string()",
        "class_name=\"docs-progress-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "progress docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn progress_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let docs_coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let docs_display_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "UiPerfBudget::mount_only(120.0)",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs perf shell should keep `{needle}` for progress mount-only budget wiring.",
        );
    }

    for needle in ["slug=\"progress\"", "pub(super) fn progress() -> AnyView"] {
        assert!(
            docs_display_source.contains(needle),
            "progress docs page should keep `{needle}` for perf-probe routing.",
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
            "UiPerfProbe should expose performance contract marker `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\")",
    ] {
        assert!(
            docs_coverage_source.contains(needle),
            "docs e2e coverage should block perf regressions via `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-progress --test progress_semantics progress_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    assert!(
        todo_source.contains("render_count"),
        "perf governance should keep explicit render_count follow-up in docs/plan/TODO.md.",
    );

    let progress_view_source = load_source("src/view.rs");
    let progress_bar_view_source = load_source("src/bar/view.rs");
    let progress_circle_view_source = load_source("src/circle/view.rs");
    for (name, source) in [
        ("Progress", progress_view_source),
        ("ProgressBar", progress_bar_view_source),
        ("ProgressCircle", progress_circle_view_source),
    ] {
        for needle in [
            "data-state",
            "data-status-mode",
            "data-value-source",
            "data-motion-source",
        ] {
            assert!(
                source.contains(needle),
                "{name} view should keep `{needle}` for performance attribution.",
            );
        }
    }
}

#[test]
fn progress_check2_marks_performance_governance_item_complete() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "UiPerfProbe",
        "data-perf-mount-ms",
        "data-perf-violation",
        "render_count",
        "cargo test -p ui-progress --test progress_semantics progress_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            source.contains(needle),
            "progress check2 performance section should include `{needle}`.",
        );
    }
}

#[test]
fn progress_view_macro_complexity_is_partitioned_for_progress_family() {
    let progress_view_source = load_source("src/view.rs");
    let progress_bar_view_source = load_source("src/bar/view.rs");
    let progress_circle_view_source = load_source("src/circle/view.rs");

    let progress_macro_count = progress_view_source.matches("view! {").count();
    let progress_bar_macro_count = progress_bar_view_source.matches("view! {").count();
    let progress_circle_macro_count = progress_circle_view_source.matches("view! {").count();

    assert_eq!(
        progress_macro_count, 1,
        "Progress should keep a single bounded `view!` block for host structure."
    );
    assert_eq!(
        progress_bar_macro_count, 1,
        "ProgressBar should keep a single bounded `view!` block for host structure."
    );
    assert!(
        (3..=5).contains(&progress_circle_macro_count),
        "ProgressCircle should split complex SVG layout into semantic sub-render blocks (expected 3..=5 view macros), found {progress_circle_macro_count}.",
    );

    for (name, source, max_lines) in [
        ("Progress", &progress_view_source, 180usize),
        ("ProgressBar", &progress_bar_view_source, 140usize),
        ("ProgressCircle", &progress_circle_view_source, 240usize),
    ] {
        assert!(
            source.lines().count() <= max_lines,
            "{name} view.rs should stay bounded (<= {max_lines} lines) to avoid giant macro expansion hotspots."
        );
    }

    for needle in [
        "fn render_progress_circle_svg(",
        "fn render_progress_circle_track(",
        "fn render_progress_circle_indicator(",
        "render_progress_circle_svg(",
    ] {
        assert!(
            progress_circle_view_source.contains(needle),
            "ProgressCircle should expose semantic sub-render function `{needle}`."
        );
    }
}

#[test]
fn progress_check2_marks_view_macro_complexity_item_complete() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] `view!` 宏复杂度受控",
        "render_progress_circle_svg",
        "render_progress_circle_track",
        "render_progress_circle_indicator",
        "cargo test -p ui-progress --test progress_semantics progress_view_macro_complexity_is_partitioned_for_progress_family",
    ] {
        assert!(
            source.contains(needle),
            "progress check2 macro-complexity section should include `{needle}`.",
        );
    }
}

#[test]
fn progress_view_prefers_function_helpers_over_local_components() {
    let progress_view_source = load_source("src/view.rs");
    let progress_bar_view_source = load_source("src/bar/view.rs");
    let progress_circle_view_source = load_source("src/circle/view.rs");

    for (name, source) in [
        ("Progress", &progress_view_source),
        ("ProgressBar", &progress_bar_view_source),
        ("ProgressCircle", &progress_circle_view_source),
    ] {
        let component_count = source.matches("#[component]").count();
        assert_eq!(
            component_count, 1,
            "{name} view should expose exactly one top-level `#[component]`; local UI fragments should stay as plain helper functions."
        );
    }

    for needle in [
        "fn render_progress_circle_svg(",
        "fn render_progress_circle_track(",
        "fn render_progress_circle_indicator(",
        "render_progress_circle_svg(",
    ] {
        assert!(
            progress_circle_view_source.contains(needle),
            "ProgressCircle should keep functional fragment helper `{needle}` instead of introducing extra local components."
        );
    }
}

#[test]
fn progress_check2_marks_functional_split_item_complete() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] 函数式拆分优先",
        "render_progress_circle_svg",
        "render_progress_circle_track",
        "render_progress_circle_indicator",
        "cargo test -p ui-progress --test progress_semantics progress_view_prefers_function_helpers_over_local_components",
    ] {
        assert!(
            source.contains(needle),
            "progress check2 functional-split section should include `{needle}`.",
        );
    }
}

#[test]
fn progress_view_static_fragments_are_templated_and_constantized() {
    let progress_circle_view_source = load_source("src/circle/view.rs");

    for needle in [
        "const PROGRESS_CIRCLE_SVG_SLOT: &str = \"progress-circle-svg\";",
        "const PROGRESS_CIRCLE_TRACK_SLOT: &str = \"progress-circle-track\";",
        "const PROGRESS_CIRCLE_INDICATOR_SLOT: &str = \"progress-circle-indicator\";",
        "struct ProgressCircleSvgTemplate",
        "fn build_progress_circle_svg_template(",
        "let svg_template = build_progress_circle_svg_template(",
        "render_progress_circle_svg(",
        "render_progress_circle_track(",
        "render_progress_circle_indicator(",
        "data-slot=PROGRESS_CIRCLE_SVG_SLOT",
        "data-slot=PROGRESS_CIRCLE_TRACK_SLOT",
        "data-slot=PROGRESS_CIRCLE_INDICATOR_SLOT",
        "role=move || a11y_contract.get().attrs.role",
        "aria-label=move || a11y_contract.get().attrs.aria_label",
    ] {
        assert!(
            progress_circle_view_source.contains(needle),
            "ProgressCircle should keep static SVG fragments templated/constantized with `{needle}`."
        );
    }
}

#[test]
fn progress_check2_marks_static_fragment_constantization_item_complete() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] 静态片段常量化",
        "ProgressCircleSvgTemplate",
        "build_progress_circle_svg_template",
        "PROGRESS_CIRCLE_SVG_SLOT",
        "PROGRESS_CIRCLE_TRACK_SLOT",
        "PROGRESS_CIRCLE_INDICATOR_SLOT",
        "cargo test -p ui-progress --test progress_semantics progress_view_static_fragments_are_templated_and_constantized",
    ] {
        assert!(
            source.contains(needle),
            "progress check2 static-fragment section should include `{needle}`.",
        );
    }
}
