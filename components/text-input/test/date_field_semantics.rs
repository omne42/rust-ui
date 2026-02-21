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
fn date_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/date_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_field_uses_logic_state_model() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");

    for needle in [
        "pub enum DateFieldTone",
        "pub struct DateFieldIds",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_ids(",
        "pub fn normalize_year(",
        "pub fn normalize_month(",
        "pub fn days_in_month(",
        "pub fn normalize_day(",
        "pub fn parse_date_value(",
        "pub fn normalize_date_value(",
        "pub fn resolve_date_parts(",
        "pub fn update_year_from_input(",
        "pub fn update_month_from_input(",
        "pub fn update_day_from_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "placeholder_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change)",
        "logic::normalize_label(label)",
        "logic::normalize_placeholder(placeholder)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(DateFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/date_field/view.rs");

    for attr in [
        "data-slot=\"date-field\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-value=move || state.get().has_value.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"date-field-label\"",
        "data-slot=\"date-field-control\"",
        "data-slot=\"date-field-year\"",
        "data-slot=\"date-field-month\"",
        "data-slot=\"date-field-day\"",
        "data-slot=\"date-field-separator\"",
        "slot_name=\"date-field-clear\"",
        "role=\"group\"",
    ] {
        assert!(
            source.contains(attr),
            "DateField should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn date_field_styles_include_tone_value_and_source_markers() {
    let source = load_source("src/text_input/date_field/styles.rs");

    for selector in [
        ".ui-date-field--tone-default",
        ".ui-date-field[data-tone=\"default\"]",
        ".ui-date-field--tone-quiet",
        ".ui-date-field--tone-strong",
        ".ui-date-field--disabled",
        ".ui-date-field[data-disabled=\"true\"]",
        ".ui-date-field--has-value",
        ".ui-date-field[data-has-value=\"true\"] .ui-date-field__control",
        ".ui-date-field--custom-class",
        ".ui-date-field[data-custom-class=\"true\"]",
        ".ui-date-field__control",
        ".ui-date-field__input",
        ".ui-date-field__clear",
    ] {
        assert!(
            source.contains(selector),
            "DateField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn date_field() -> AnyView",
        "title=\"DateField\"",
        "slug=\"date-field\"",
        "description=\"Segmented date entry field with centralized year/month/day normalization and baseline-style state/source contracts.\"",
        "<Playground title=\"Controlled Value\" code_signal=code>",
        "<Playground title=\"Strong Tone + Custom Placeholder\" code_signal=states_code>",
        "<DateField",
        "tone=DateFieldTone::Strong",
        "default_value=\"2026-07-22\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for date_field primary coverage.",
        );
    }
}

#[test]
fn date_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "let (value, set_value) = signal(Some(\"2026-03-14\".to_string()));",
        "id_base=\"docs-date-field-controlled\".to_string()",
        "label=\"Invoice date\".to_string()",
        "value=value",
        "on_value_change=on_value_change",
        "\"value: \"",
        "id_base=\"docs-date-field-strong\".to_string()",
        "label=\"Ship date\".to_string()",
        "tone=DateFieldTone::Strong",
        "default_value=\"2026-07-22\".to_string()",
        "placeholder=\"year-month-day\".to_string()",
        "class_name=\"docs-date-field-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "date_field docs playgrounds should contain `{needle}`.",
        );
    }
}
