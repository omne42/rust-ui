use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn meter_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/meter/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Meter internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn meter_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/meter/mod.rs");
    let crate_source = load_source("src/lib.rs");
    let cargo_source = load_source("Cargo.toml");

    assert!(
        module_source.contains("pub use view::Meter;"),
        "meter module should export `Meter`.",
    );
    assert!(
        module_source.contains("pub use motion::MeterMotion;"),
        "meter module should export `MeterMotion`.",
    );
    assert!(
        crate_source.contains("pub use ui_meter as meter;"),
        "crate root should re-export external ui-meter crate as `meter`.",
    );
    assert!(
        crate_source.contains("pub use meter::{Meter, MeterMotion, MeterSize, MeterVariant};"),
        "crate root prelude should re-export meter public API.",
    );
    assert!(
        cargo_source.contains("component-meter = [\"dep:ui-meter\"]"),
        "component-meter feature should depend on dep:ui-meter after extraction.",
    );
    assert!(
        cargo_source.contains("ui-meter = { path = \"../../components/meter\", optional = true }"),
        "ui-components Cargo.toml should include optional ui-meter dependency.",
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn meter_uses_logic_state_model() {
    let view_source = load_source("src/meter/view.rs");
    let logic_source = load_source("src/meter/logic.rs");

    for needle in [
        "pub use ui_state_primitives::meter::{",
        "normalize_optional_text,",
        "resolve_aria_label,",
        "resolve_value_label,",
        "resolve_phase,",
        "resolve_state,",
        "compose_class_name,",
        "label_source_attr",
        "value_label_source_attr",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Meter logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(label)",
        "logic::resolve_aria_label(aria_label, label.clone())",
        "logic::resolve_value_label(value_label)",
        "logic::resolve_state(logic::MeterStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::resolve_phase(is_indeterminate.get())",
        "motion::attach_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "Meter view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn meter_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/meter/view.rs");

    for attr in [
        "data-slot=\"meter\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=move || phase.get().as_str()",
        "data-phase-class=move || phase.get().class_name()",
        "data-indeterminate=move || {",
        "data-determinate=move || {",
        "data-label-source=state.label_source_attr",
        "data-value-label-source=state.value_label_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-value-label=state.has_custom_value_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "role=\"meter\"",
        "aria-valuetext=move || value_label_text.get()",
    ] {
        assert!(
            source.contains(attr),
            "Meter should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn meter_styles_include_state_source_contracts() {
    let source = load_source("src/meter/styles.rs");

    for selector in [
        ".ui-meter--variant-default",
        ".ui-meter[data-variant=\"danger\"]",
        ".ui-meter--size-lg .ui-meter__track",
        ".ui-meter[data-size=\"sm\"] .ui-meter__track",
        ".ui-meter--label-custom .ui-meter__label",
        ".ui-meter[data-label-source=\"custom\"] .ui-meter__label",
        ".ui-meter--value-label-custom .ui-meter__value-label",
        ".ui-meter[data-value-label-source=\"custom\"] .ui-meter__value-label",
        ".ui-meter--motion-custom",
        ".ui-meter[data-motion-source=\"custom\"]",
        ".ui-meter--custom-class",
        ".ui-meter[data-custom-class=\"true\"]",
        ".ui-meter--state-indeterminate .ui-meter__indicator",
        ".ui-meter[data-state=\"indeterminate\"] .ui-meter__indicator",
        ".ui-meter--state-determinate .ui-meter__indicator",
        ".ui-meter[data-state=\"determinate\"] .ui-meter__indicator",
    ] {
        assert!(
            source.contains(selector),
            "Meter styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn meter_motion_uses_spring_animator() {
    let source = load_source("src/meter/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Meter motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn meter_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/meter/motion.rs");
    let view_source = load_source("src/meter/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: MeterMotion) -> MeterMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Meter motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "Meter view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn meter_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn meter() -> AnyView",
        "title=\"Meter\"",
        "slug=\"meter\"",
        "Playground title=\"Variant + Size Matrix\"",
        "Playground title=\"Custom Label + Motion + Class\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=test_css_source",
        "test_config_signal=actual_config",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Meter.",
        );
    }
}

#[test]
fn meter_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variant + Size Matrix\"",
        "id=\"docs-meter-default\".to_string()",
        "id=\"docs-meter-danger\".to_string()",
        "variant=MeterVariant::Danger",
        "size=MeterSize::Lg",
        "id=\"docs-meter-compact\".to_string()",
        "show_value_label=false",
        "on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 10).min(100)))",
        "title=\"Custom Label + Motion + Class\"",
        "id=\"docs-meter-custom\".to_string()",
        "aria_label=\"Background sync\".to_string()",
        "value_label=\"64 complete\".to_string()",
        "motion=ui_components::MeterMotion::fast()",
        "id=\"docs-meter-fallback\".to_string()",
        "label=\"   \".to_string()",
        "id=\"docs-meter-indeterminate\".to_string()",
        "value=Signal::derive(|| None)",
        "class_name=\"docs-meter-custom\".to_string()",
        "id=\"docs-meter-workbench\".to_string()",
        "id=\"docs-meter-workbench-contrast\".to_string()",
        "id=\"docs-meter-workbench-indeterminate\".to_string()",
        "data-slot=\"meter-workbench-controls\"",
    ] {
        assert!(
            source.contains(needle),
            "meter docs playgrounds should contain `{needle}`.",
        );
    }
}
