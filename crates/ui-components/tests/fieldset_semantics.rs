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
fn fieldset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/fieldset/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Fieldset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn fieldset_consumes_state_primitives_and_keeps_component_assembly_local() {
    let logic_source = load_source("src/fieldset/logic.rs");
    let view_source = load_source("src/fieldset/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/fieldset.rs");

    for needle in [
        "pub use ui_state_primitives::fieldset::{",
        "FieldsetOrientation",
        "FieldsetTone",
        "FieldsetState",
        "FieldsetStateInput",
        "normalize_aria_label",
        "normalize_error_message",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Fieldset logic should include `{needle}` to consume ui-state-primitives and keep only assembly logic."
        );
    }

    for needle in [
        "pub enum FieldsetOrientation",
        "pub enum FieldsetTone",
        "pub struct FieldsetStateInput",
        "pub struct FieldsetState",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(input: FieldsetStateInput) -> FieldsetState",
    ] {
        assert!(
            primitives_source.contains(needle),
            "fieldset primitive module should define `{needle}`."
        );
    }

    for needle in [
        "let required = is_required.unwrap_or(required);",
        "let disabled = is_disabled.unwrap_or(disabled);",
        "let invalid = is_invalid.unwrap_or(invalid);",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, invalid)",
        "logic::resolve_state(FieldsetStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Fieldset view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn fieldset_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/fieldset/view.rs");

    for attr in [
        "style=move || motion_style.get_value()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-slot=\"fieldset\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-required-source=required_source_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-disabled-source=disabled_source_attr",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-invalid-source=invalid_source_attr",
        "data-has-legend=move || state.get().has_legend.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-has-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "data-slot=\"fieldset-field-group\"",
        "data-slot=\"fieldset-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "Fieldset should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn fieldset_styles_include_state_markers() {
    let source = load_source("src/fieldset/styles.rs");

    for selector in [
        ".ui-fieldset--orientation-vertical",
        ".ui-fieldset[data-orientation=\"horizontal\"]",
        ".ui-fieldset--tone-default",
        ".ui-fieldset[data-tone=\"muted\"]",
        ".ui-fieldset--required .ui-fieldset__legend",
        ".ui-fieldset[data-required=\"true\"] .ui-fieldset__legend",
        ".ui-fieldset--disabled",
        ".ui-fieldset[data-disabled=\"true\"]",
        ".ui-fieldset--invalid .ui-fieldset__group",
        ".ui-fieldset[data-invalid=\"true\"] .ui-fieldset__group",
        ".ui-fieldset--custom-class",
        ".ui-fieldset[data-custom-class=\"true\"]",
        ".ui-fieldset[data-class-source=\"custom\"]",
        "font-size: var(--ui-font-size-150, 0.875rem);",
        "font-size: var(--ui-font-size-100, 0.75rem);",
    ] {
        assert!(
            source.contains(selector),
            "Fieldset styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn fieldset_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn fieldset() -> AnyView",
        "title=\"Fieldset\"",
        "slug=\"fieldset\"",
        "description=\"baseline-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts.\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Legend + Description\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Invalid + Actions\" code_signal=invalid_code>",
        "title=\"Fieldset Workbench (Display + Config + Code + CSS Test)\"",
        "test_source_path=\"crates/ui-components/src/fieldset/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "is_invalid=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for fieldset primary coverage.",
        );
    }
}

#[test]
fn fieldset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World\"",
        "<Fieldset legend=\"Channels\".to_string() aria_label=\"Channel fieldset\".to_string()>",
        "title=\"Legend + Description\"",
        "legend=\"Notification channels\".to_string()",
        "description=\"Pick every channel you want to receive release updates from.\".to_string()",
        "required=true",
        "aria_label=\"Notification channel group\".to_string()",
        "<span>\"Email\"</span>",
        "<span>\"SMS\"</span>",
        "<span>\"Push\"</span>",
        "orientation=FieldsetOrientation::Horizontal",
        "tone=FieldsetTone::Muted",
        "is_invalid=true",
        "error_message=\"Pick at least one channel\".to_string()",
        "class_name=\"docs-fieldset-custom\".to_string()",
        "variant=ui_components::ButtonVariant::Secondary",
        "size=ui_components::ButtonSize::Sm",
        "\"Manage channels\"",
        "data-slot=\"fieldset-workbench-controls\"",
        "data-slot=\"fieldset-workbench-compare\"",
        "\"Scenario compare\"",
        "Switch checked=workbench_required set_checked=set_workbench_required",
        "Switch checked=workbench_invalid set_checked=set_workbench_invalid",
        "Switch checked=workbench_show_actions set_checked=set_workbench_show_actions",
    ] {
        assert!(
            source.contains(needle),
            "fieldset docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn fieldset_logic_resolves_agent_contract_and_locale_helpers() {
    let source = load_source("src/fieldset/logic.rs");
    let view_source = load_source("src/fieldset/view.rs");

    for needle in [
        "pub struct FieldsetAgentContract",
        "pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract",
        "schema_attr: \"ui.fieldset.agent-contract\"",
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            source.contains(needle),
            "fieldset logic should expose `{needle}` machine-readable contract marker."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "fieldset view should compose locale + agent contract via `{needle}`."
        );
    }
}

#[test]
fn fieldset_e2e_contract_file_exists_and_uses_semantic_selectors() {
    let rel = "../../e2e/tests/docs_app_fieldset_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "fieldset should provide docs-app e2e contract file: `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "data-component=\"fieldset\"",
        "data-slot=\"fieldset\"",
        "data-ui-schema",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-required-source",
        "Show code|Hide code",
    ] {
        assert!(
            source.contains(needle),
            "fieldset e2e contract should include `{needle}`."
        );
    }
}
