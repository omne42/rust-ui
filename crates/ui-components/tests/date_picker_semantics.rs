use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/date_picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DatePicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_picker_uses_logic_state_model() {
    let logic_source = load_source("src/text_input/date_picker/logic.rs");
    let view_source = load_source("src/text_input/date_picker/view.rs");

    for needle in [
        "pub use ui_logic_calendar::date_picker::{",
        "DatePickerTone",
        "DatePickerIds",
        "DatePickerStateInput",
        "normalize_optional_text",
        "normalize_aria_label",
        "normalize_placeholder",
        "normalize_month",
        "normalize_selected_day",
        "resolve_ids",
        "resolve_state",
        "resolve_trigger_label",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DatePicker logic should include `{needle}` for primitives re-export and centralized assembly."
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state_traced(",
        "overlay_open::use_controllable_state(",
        "logic::normalize_placeholder(placeholder)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(DatePickerStateInput {",
        "has_custom_motion,",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "locale_attrs(logic::normalize_optional_text(lang), dir);",
        "motion: DatePickerMotion",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "DatePicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_picker_state_primitives_are_sourced_from_ui_state_primitives() {
    let primitive_source = load_source("../ui-logic-calendar/src/date_picker.rs");
    let logic_source = load_source("src/text_input/date_picker/logic.rs");

    for needle in [
        "pub struct DatePickerStateInput",
        "pub struct DatePickerState",
        "pub enum DatePickerTone",
        "pub fn resolve_state(input: DatePickerStateInput) -> DatePickerState",
        "pub fn resolve_ids(id_base: &str) -> DatePickerIds",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-logic-calendar::date_picker should include `{needle}`."
        );
    }

    for banned in ["use leptos", "web_sys", "wasm_bindgen"] {
        assert!(
            !primitive_source.contains(banned),
            "state primitives should stay platform-agnostic; found `{banned}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_logic_calendar::date_picker::{"),
        "date_picker logic should consume state primitives from ui-state-primitives."
    );
}

#[test]
fn date_picker_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/date_picker/view.rs");

    for attr in [
        "data-slot=\"date-picker\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-value=move || state.get().has_value.then_some(\"true\")",
        "data-selected-day=move || state.get().selected_day.map(|day| day.to_string())",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-slot=\"date-picker-trigger\"",
        "data-slot=\"date-picker-panel\"",
        "class_name=\"ui-date-picker__trigger\".to_string()",
        "class_name=\"ui-date-picker__calendar\".to_string()",
        "role=\"group\"",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "DatePicker should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn date_picker_styles_include_tone_open_value_and_source_markers() {
    let source = load_source("src/text_input/date_picker/styles.rs");

    for selector in [
        ".ui-date-picker--tone-default",
        ".ui-date-picker[data-tone=\"default\"]",
        ".ui-date-picker--tone-quiet",
        ".ui-date-picker--tone-strong",
        ".ui-date-picker--open",
        ".ui-date-picker[data-open=\"true\"]",
        ".ui-date-picker--disabled",
        ".ui-date-picker[data-disabled=\"true\"]",
        ".ui-date-picker--has-value",
        ".ui-date-picker[data-has-value=\"true\"]",
        ".ui-date-picker[data-class-source=\"custom\"]",
        ".ui-date-picker--custom-class",
        ".ui-date-picker[data-custom-class=\"true\"]",
        ".ui-date-picker[data-motion-source=\"custom\"]",
        ".ui-date-picker--custom-motion",
        ".ui-date-picker[data-custom-motion=\"true\"]",
        ".ui-date-picker__trigger",
        ".ui-date-picker__panel",
        ".ui-date-picker__calendar",
    ] {
        assert!(
            source.contains(selector),
            "DatePicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_picker_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/text_input/date_picker/mod.rs");
    let motion_source = load_source("src/text_input/date_picker/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::DatePickerMotion;",
        "pub struct DatePickerMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "DatePicker motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn date_picker_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/text_input/date_picker/motion.rs");
    let view_source = load_source("src/text_input/date_picker/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DatePickerMotion) -> DatePickerMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "DatePicker motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains(
            "let motion = crate::text_input::date_picker::motion::sanitize_motion(motion);"
        ),
        "DatePicker view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn date_picker_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn date_picker() -> AnyView",
        "title=\"DatePicker\"",
        "slug=\"date-picker\"",
        "description=\"Date picker trigger + popover calendar with centralized open/value/source state contracts and baseline-level popover motion handoff.\"",
        "<Playground title=\"Default + Outside Days\" code_signal=code>",
        "<Playground title=\"Monday First + Strong Tone\" code_signal=states_code>",
        "<DatePicker",
        "tone=DatePickerTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for date_picker primary coverage.",
        );
    }
}

#[test]
fn date_picker_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "id_base=\"docs-date-picker-release\".to_string()",
        "year=2026",
        "month=3",
        "default_selected_day=12",
        "tone=DatePickerTone::Default",
        "first_weekday=CalendarFirstWeekday::Sunday",
        "show_outside_days=true",
        "id_base=\"docs-date-picker-ship\".to_string()",
        "month=4",
        "default_selected_day=21",
        "tone=DatePickerTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
        "placeholder=\"Pick ship date\".to_string()",
        "class_name=\"docs-date-picker-custom\".to_string()",
        "motion=DatePickerMotion {",
        "popover: PopoverMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 10.0",
    ] {
        assert!(
            source.contains(needle),
            "date_picker docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn date_picker_docs_page_includes_workbench_css_test_and_comparison_matrix() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"展示 / Config / Code / CSS Test\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"crates/ui-components/src/text_input/date_picker/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"date-picker-workbench-controls\"",
        "data-slot=\"date-picker-workbench\"",
        "data-slot=\"date-picker-workbench-summary\"",
        "title=\"Comparison Matrix (Default / Quiet / Strong / Disabled)\"",
        "data-slot=\"date-picker-comparison-matrix\"",
        "id_base=\"docs-date-picker-compare-default\".to_string()",
        "id_base=\"docs-date-picker-compare-quiet\".to_string()",
        "id_base=\"docs-date-picker-compare-strong\".to_string()",
        "id_base=\"docs-date-picker-compare-disabled\".to_string()",
        "tone=DatePickerTone::Quiet",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "date_picker docs page should expose workbench/comparison marker `{needle}`.",
        );
    }
}

#[test]
fn date_picker_readme_is_whitelisted_in_docs_shell() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    for needle in [
        "const DATE_PICKER_README_MD: &str =",
        "include_str!(\"../../../../../crates/ui-components/src/text_input/date_picker/README.md\")",
        "\"date-picker\" => Some(DATE_PICKER_README_MD),",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should include date_picker readme whitelist marker `{needle}`.",
        );
    }
}

#[test]
fn date_picker_readme_includes_workbench_and_matrix_sections() {
    let readme_source = load_source("src/text_input/date_picker/README.md");

    for needle in [
        "## Docs Playground（展示 / Config / Code / CSS Test）",
        "展示区",
        "Config 区",
        "Code 区",
        "CSS Test 区",
        "Comparison Matrix (Default / Quiet / Strong / Disabled)",
    ] {
        assert!(
            readme_source.contains(needle),
            "date_picker README should include docs coverage marker `{needle}`.",
        );
    }
}

#[test]
fn date_picker_feature_gate_includes_required_dependencies() {
    let source = load_source("Cargo.toml");

    for needle in [
        "component-date_picker = [",
        "\"component-button\"",
        "\"component-calendar\"",
        "\"component-popover\"",
    ] {
        assert!(
            source.contains(needle),
            "component-date_picker feature gate should include `{needle}`."
        );
    }
}
