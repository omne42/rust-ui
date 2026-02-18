use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn checkbox_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/checkbox_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CheckboxField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_uses_logic_state_model() {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let view_source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "pub struct CheckboxFieldStateInput",
        "pub struct CheckboxFieldState",
    ] {
        assert!(
            mod_source.contains(needle),
            "CheckboxField module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_label(label)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_aria_label(aria_label, &label.get_value())",
        "logic::resolve_state(CheckboxFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_composes_checkbox_with_label_slot() {
    let source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "<Checkbox",
        "checked=checked",
        "set_checked=set_checked",
        "variant=checkbox_variant",
        "class_name=checkbox_class",
        "data-slot=\"checkbox-field-label\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxField should compose Checkbox with stable contracts (`{needle}`)."
        );
    }
}

#[test]
fn checkbox_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/checkbox_field/view.rs");

    for attr in [
        "data-slot=\"checkbox-field\"",
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-indicator-placement=move || state.get().indicator_placement_attr",
        "data-description=move || state.get().description_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-slot=\"checkbox-field-description\"",
    ] {
        assert!(
            source.contains(attr),
            "CheckboxField should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn checkbox_field_styles_include_state_marker_contracts() {
    let source = load_source("src/checkbox_field/styles.rs");

    for selector in [
        ".ui-checkbox-field--indicator-end",
        ".ui-checkbox-field[data-indicator-placement=\"end\"]",
        ".ui-checkbox-field--tone-quiet",
        ".ui-checkbox-field[data-tone=\"default\"]",
        ".ui-checkbox-field--invalid .ui-checkbox-field__description",
        ".ui-checkbox-field[data-disabled=\"true\"]",
        ".ui-checkbox-field--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "CheckboxField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn checkbox_field_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "<Playground title=\"Controlled + Description\" code_signal=code>",
        "id_base=\"docs-checkbox-field-newsletter\".to_string()",
        "label=\"Subscribe to product updates\".to_string()",
        "description=\"Receive release notes and occasional best-practice tips.\".to_string()",
        "<Playground title=\"Indicator End + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "id_base=\"docs-checkbox-field-terms\".to_string()",
        "indicator_placement=CheckboxFieldIndicatorPlacement::End",
        "tone=CheckboxFieldTone::Quiet",
        "invalid=true",
        "class_name=\"docs-checkbox-field-custom\".to_string()",
        "id_base=\"docs-checkbox-field-read-only\".to_string()",
        "disabled=true",
        "aria_label=\"Maintenance alerts (read only)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_minimal_feature_gate_keeps_checkbox_dependency_wired() {
    let cargo_toml = load_source("Cargo.toml");
    let view_source = load_source("src/checkbox_field/view.rs");

    assert!(
        cargo_toml.contains("component-checkbox_field = [\"component-checkbox\"]"),
        "component-checkbox_field must depend on component-checkbox to keep minimal feature builds valid."
    );

    assert!(
        view_source.contains("use crate::checkbox::{Checkbox, CheckboxVariant};"),
        "checkbox_field view should import checkbox types from crate::checkbox module, not root re-exports."
    );
}

#[test]
fn checkbox_field_docs_include_interactive_playground_contract_panels() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"crates/ui-components/src/checkbox_field/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("src/checkbox_field/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "checkbox-field README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"checkbox-field\" => Some(CHECKBOX_FIELD_README_MD)"),
        "docs shell should map checkbox-field slug to CHECKBOX_FIELD_README_MD.",
    );
}
