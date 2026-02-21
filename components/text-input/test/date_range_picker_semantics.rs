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
fn date_range_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/date_range_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateRangePicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn date_range_picker_uses_logic_state_model() {
    let logic_source = load_source("src/text_input/date_range_picker/logic.rs");
    let view_source = load_source("src/text_input/date_range_picker/view.rs");

    for needle in [
        "pub use ui_logic_calendar::date_range_picker::{",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_month(",
        "pub fn days_in_month(",
        "pub fn normalize_day(",
        "pub fn is_range_invalid(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_text_state(",
        "DateRangePickerTextInput",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateRangePicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_aria_label(aria_label)",
        "logic::is_range_invalid(",
        "logic::resolve_state(DateRangePickerStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::resolve_text_state(DateRangePickerTextInput {",
        "text_state.get_value().start_label",
        "text_state.get_value().invalid_range_message",
        "<DatePicker",
    ] {
        assert!(
            view_source.contains(needle),
            "DateRangePicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_range_picker_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/date_range_picker/view.rs");

    for attr in [
        "data-slot=\"date-range-picker\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-start-value=move || state.get().has_start_value.then_some(\"true\")",
        "data-has-end-value=move || state.get().has_end_value.then_some(\"true\")",
        "data-has-full-value=move || state.get().has_full_value.then_some(\"true\")",
        "data-partial=move || state.get().is_partial.then_some(\"true\")",
        "data-invalid-range=move || state.get().is_invalid_range.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"date-range-picker-fields\"",
        "data-slot=\"date-range-picker-start\"",
        "data-slot=\"date-range-picker-end\"",
        "data-slot=\"date-range-picker-start-label\"",
        "data-slot=\"date-range-picker-end-label\"",
        "data-slot=\"date-range-picker-hint\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "DateRangePicker should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn date_range_picker_styles_include_tone_partial_invalid_and_source_markers() {
    let source = load_source("src/text_input/date_range_picker/styles.rs");

    for selector in [
        ".ui-date-range-picker--tone-default",
        ".ui-date-range-picker[data-tone=\"default\"]",
        ".ui-date-range-picker--tone-quiet",
        ".ui-date-range-picker--tone-strong",
        ".ui-date-range-picker--disabled",
        ".ui-date-range-picker[data-disabled=\"true\"]",
        ".ui-date-range-picker--partial",
        ".ui-date-range-picker[data-state=\"partial\"]",
        ".ui-date-range-picker--has-full-value",
        ".ui-date-range-picker[data-has-full-value=\"true\"]",
        ".ui-date-range-picker--invalid-range",
        ".ui-date-range-picker[data-invalid-range=\"true\"]",
        ".ui-date-range-picker--custom-class",
        ".ui-date-range-picker[data-custom-class=\"true\"]",
        ".ui-date-range-picker__fields",
        ".ui-date-range-picker__picker",
        ".ui-date-range-picker__hint",
    ] {
        assert!(
            source.contains(selector),
            "DateRangePicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_range_picker_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn date_range_picker() -> AnyView",
        "title=\"DateRangePicker\"",
        "slug=\"date-range-picker\"",
        "description=\"Two DatePicker composition with centralized range validity/value-shape derivation and baseline-style state/source contracts.\"",
        "<Playground title=\"Controlled + Shared Month\" code_signal=code>",
        "<Playground title=\"Strong Tone + Invalid Range Hint\" code_signal=states_code>",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=test_css_source",
        "test_config_signal=actual_config",
        "<DateRangePicker",
        "tone=DateRangePickerTone::Strong",
        "default_start_day=20",
        "default_end_day=12",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for date_range_picker primary coverage.",
        );
    }
}

#[test]
fn date_range_picker_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "let (start_day, set_start_day) = signal(Some(8_u8));",
        "let (end_day, set_end_day) = signal(Some(19_u8));",
        "id_base=\"docs-date-range-picker-controlled\".to_string()",
        "start_year=2026",
        "start_month=6",
        "end_year=2026",
        "end_month=6",
        "start_day=start_day",
        "end_day=end_day",
        "on_start_day_change=on_start_day_change",
        "on_end_day_change=on_end_day_change",
        "\"start: \"",
        "\" · end: \"",
        "id_base=\"docs-date-range-picker-strong\".to_string()",
        "start_month=7",
        "end_month=7",
        "default_start_day=20",
        "default_end_day=12",
        "tone=DateRangePickerTone::Strong",
        "class_name=\"docs-date-range-picker-custom\".to_string()",
        "id_base=\"docs-date-range-picker-workbench\".to_string()",
        "id_base=\"docs-date-range-picker-compare-valid\".to_string()",
        "id_base=\"docs-date-range-picker-compare-invalid\".to_string()",
        "data-slot=\"date-range-picker-workbench-controls\"",
    ] {
        assert!(
            source.contains(needle),
            "date_range_picker docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn date_range_picker_feature_gate_includes_required_dependencies() {
    let source = load_source("Cargo.toml");

    for needle in [
        "component-date_range_picker = [",
        "\"component-date_picker\"",
    ] {
        assert!(
            source.contains(needle),
            "component-date_range_picker feature gate should include `{needle}`."
        );
    }
}
