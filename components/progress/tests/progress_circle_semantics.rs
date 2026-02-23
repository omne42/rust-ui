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
        let child_path = workspace_dir
            .join("apps/docs-app/src/pages/components/pages/display/progress_circle.rs");
        let parent = fs::read_to_string(&parent_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {parent_path:?}: {e}"));
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        return format!("{parent}\n{child}").replace(
            "pub(crate) fn progress_circle() -> AnyView {",
            "pub(super) fn progress_circle() -> AnyView {",
        );
    }
    let path = if let Some(suffix) = rel_path.strip_prefix("src/circle/") {
        manifest_dir.join("src/circle").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_circle_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/circle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ProgressCircle internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn progress_circle_public_api_exposes_value_triplet_contract() {
    let source = load_source("src/circle/view.rs");

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<Option<f64>>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<f64>>>",
    ] {
        assert!(
            source.contains(needle),
            "ProgressCircle public value axis should expose `{needle}`."
        );
    }
}

#[test]
fn progress_circle_uses_logic_state_model() {
    let view_source = load_source("src/circle/view.rs");
    let logic_source = load_source("src/circle/logic.rs");

    for needle in [
        "pub use ui_state_primitives::progress_circle::{",
        "ProgressCircleStateInput",
        "ProgressCircleState",
        "ProgressCircleMetricsInput",
        "ProgressCirclePhase",
        "ProgressCircleMode",
        "ProgressCircleValueAxis",
        "ProgressCircleKernelInput",
        "ProgressCircleKernelState",
        "ProgressCircleStrokeInput",
        "ProgressCircleStrokeState",
        "normalize_mode(",
        "normalize_value_axis(",
        "DEFAULT_MIN",
        "DEFAULT_MAX",
        "normalize_range(",
        "normalize_progress_value(",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_value_label",
        "sanitize_dimension",
        "resolve_metrics",
        "resolve_phase",
        "resolve_kernel_state(",
        "resolve_stroke_state(",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "ProgressCircle logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_value_label(value_label)",
        "let mode = logic::normalize_mode(is_indeterminate);",
        "logic::normalize_value_axis(value, default_value, on_value_change)",
        "logic::normalize_range(min, max)",
        "let kernel_state = Signal::derive(move || {",
        "logic::resolve_kernel_state(logic::ProgressCircleKernelInput {",
        "mode,",
        "let progress_value = Signal::derive(move || kernel_state.get().progress_value);",
        "let stroke_state = Signal::derive(move || {",
        "logic::resolve_stroke_state(logic::ProgressCircleStrokeInput {",
        "use_controllable_state(",
        "value_axis.value",
        "Some(value_axis.default_value)",
        "value_axis.on_value_change",
        "logic::resolve_metrics(logic::ProgressCircleMetricsInput {",
        "logic::resolve_state(logic::ProgressCircleStateInput {",
        "logic::compose_class_name(class_name, state)",
        "is_indeterminate: kernel_state.is_indeterminate,",
        "progressbar_attrs(ProgressbarA11yOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "ProgressCircle view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn progress_circle_logic_consumes_state_primitives_for_value_axis_and_mode() {
    let logic_source = load_source("src/circle/logic.rs");

    for needle in [
        "pub use ui_state_primitives::progress_circle::{",
        "ProgressCircleMode",
        "ProgressCircleValueAxisInput",
        "ProgressCircleValueAxisState",
        "normalize_mode",
        "resolve_value_axis",
        "resolve_value_axis(ProgressCircleValueAxisInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "ProgressCircle logic should consume state primitives via `{needle}`."
        );
    }

    assert!(
        !logic_source.contains("pub enum ProgressCircleMode"),
        "ProgressCircle mode enum should live in ui-state-primitives, not component logic.",
    );
}

#[test]
fn progress_circle_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/circle/view.rs");

    for attr in [
        "data-slot=\"progress-circle\"",
        "data-state=move || a11y_contract.get().attrs.data_state",
        "data-phase-class=move || kernel_state.get().phase.class_name()",
        "data-status-mode=move || kernel_state.get().mode.as_str()",
        "data-indeterminate=move || a11y_contract.get().attrs.data_indeterminate",
        "data-determinate=move || a11y_contract.get().attrs.data_determinate",
        "data-size-source=state.size_source_attr",
        "data-stroke-source=state.stroke_source_attr",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-size=state.has_custom_size.then_some(\"true\")",
        "data-custom-stroke=state.has_custom_stroke_width.then_some(\"true\")",
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
            "ProgressCircle should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn progress_circle_styles_include_state_source_contracts() {
    let source = load_source("src/circle/styles.rs");

    for selector in [
        ".ui-progress-circle--label-custom",
        ".ui-progress-circle[data-label-source=\"custom\"]",
        ".ui-progress-circle--size-custom",
        ".ui-progress-circle[data-size-source=\"custom\"]",
        ".ui-progress-circle--stroke-custom .ui-progress-circle__track",
        ".ui-progress-circle[data-stroke-source=\"custom\"] .ui-progress-circle__track",
        ".ui-progress-circle--motion-custom",
        ".ui-progress-circle[data-motion-source=\"custom\"]",
        ".ui-progress-circle--custom-class",
        ".ui-progress-circle[data-custom-class=\"true\"]",
        ".ui-progress-circle--state-indeterminate .ui-progress-circle__svg",
        ".ui-progress-circle[data-state=\"indeterminate\"] .ui-progress-circle__svg",
        ".ui-progress-circle--state-determinate .ui-progress-circle__svg",
        ".ui-progress-circle[data-state=\"determinate\"] .ui-progress-circle__svg",
    ] {
        assert!(
            source.contains(selector),
            "ProgressCircle styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn progress_circle_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/circle/motion.rs");
    let view_source = load_source("src/circle/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ProgressCircleMotion) -> ProgressCircleMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let motion = StoredValue::new(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "ProgressCircle motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::circle::motion::sanitize_motion(motion);"),
        "ProgressCircle view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn progress_circle_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn progress_circle() -> AnyView",
        "title=\"ProgressCircle\"",
        "slug=\"progress-circle\"",
        "Playground title=\"Determinate + Indeterminate\"",
        "Playground title=\"Custom Value Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for ProgressCircle.",
        );
    }
}

#[test]
fn progress_circle_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Determinate + Indeterminate\"",
        "<ProgressCircle aria_label=\"Determinate\".to_string() value=progress_value min=0.0 max=100.0 />",
        "<ProgressCircle aria_label=\"Indeterminate\".to_string() value=Signal::derive(|| None) />",
        "on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0)))",
        "title=\"Custom Value Label + Class\"",
        "aria_label=\"Sync progress\".to_string()",
        "value=Signal::derive(|| Some(64.0))",
        "size_px=40.0",
        "stroke_width_px=5.0",
        "value_label=\"64 done\".to_string()",
        "aria_label=\"   \".to_string()",
        "class_name=\"docs-progress-circle-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "progress-circle docs playgrounds should contain `{needle}`.",
        );
    }
}
