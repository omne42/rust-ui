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
fn checkbox_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/checkbox-group/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CheckboxGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_uses_logic_state_model() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");

    for needle in [
        "pub struct CheckboxGroupState",
        "pub fn resolve_checkbox_group_state(",
        "pub is_disabled: bool",
        "pub is_invalid: bool",
        "pub shows_error: bool",
        "pub has_messages: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_checkbox_group_state(",
        "state.get().shows_error",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_resolves_ids_and_normalizes_text_inputs() {
    let view_source = load_source("../../components/checkbox-group/src/view.rs");
    let logic_source = load_source("../../components/checkbox-group/src/logic.rs");

    for needle in [
        "resolve_checkbox_group_ids",
        "normalize_checkbox_group_label",
        "normalize_checkbox_group_optional_text",
        "aria-labelledby=legend_id.get_value()",
        "id=legend_id.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxGroup should wire `{needle}` for stable labeling and normalized text content."
        );
    }

    assert!(
        logic_source.contains("\"Options\".to_string()"),
        "CheckboxGroup label normalization should default empty labels to a stable fallback."
    );
}

#[test]
fn checkbox_group_uses_headless_text_field_contract() {
    let source = load_source("../../components/checkbox-group/src/logic.rs");

    for needle in [
        "use_text_field",
        "CheckboxGroupFieldsetAttrs",
        "aria_describedby",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should delegate describedby/invalid/required modeling via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_emits_baseline_state_data_attributes() {
    let source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "data-slot=\"checkbox-group\"",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-valid=move || state.get().is_valid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-optional=move || state.get().is_optional.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error.then_some(\"true\")",
        "data-shows-error=move || state.get().shows_error.then_some(\"true\")",
        "data-has-messages=move || state.get().has_messages.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should expose `{needle}` for baseline-style state styling and inspection."
        );
    }
}

#[test]
fn checkbox_group_only_renders_error_slot_when_invalid() {
    let source = load_source("../../components/checkbox-group/src/view.rs");

    for needle in [
        "<Show when=move || state.get().shows_error>",
        "data-slot=\"checkbox-group-error\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxGroup should guard error rendering via `{needle}`."
        );
    }
}

#[test]
fn checkbox_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn checkbox_group() -> AnyView",
        "title=\"CheckboxGroup\"",
        "slug=\"checkbox-group\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "<Playground title=\"Validation + Required\" code_signal=code>",
        "id=\"docs-checkbox-group\".to_string()",
        "label=\"Fruits\".to_string()",
        "required=required",
        "invalid=invalid",
        "aria_describedby=aria_describedby",
        "id=\"docs-checkbox-group-extra\"",
        "\"Clear selections\"",
        "<Playground title=\"Disabled + Optional\" code_signal=states_code>",
        "id=\"docs-checkbox-group-disabled\".to_string()",
        "disabled=true",
        "id=\"docs-checkbox-group-optional\".to_string()",
        "\"optional selected count: \"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_docs_include_interactive_playground_contract_panels() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"components/checkbox-group/src/styles.rs\".to_string()",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Optional\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-group interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_group_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("../../components/checkbox-group/src/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "checkbox-group README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"checkbox-group\" => Some(CHECKBOX_GROUP_README_MD)"),
        "docs shell should map checkbox-group slug to CHECKBOX_GROUP_README_MD.",
    );
}

#[test]
fn checkbox_group_breaking_migration_removes_legacy_namespace_and_path_shim() {
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "checkbox::group::CheckboxGroup",
        "#[path = \"checkbox_field/checkbox/mod.rs\"]",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "checkbox breaking migration should not keep legacy compatibility token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_group_css_aggregation_uses_new_top_level_contract() {
    let css_source = load_source("src/css.rs");

    assert!(
        css_source.contains("out.push_str(crate::checkbox_group::styles::CSS);"),
        "css aggregation should use top-level checkbox_group css constant.",
    );
    assert!(
        !css_source.contains("out.push_str(crate::checkbox::styles::CHECKBOX_GROUP_CSS);"),
        "css aggregation should not keep merged checkbox::styles::CHECKBOX_GROUP_CSS path.",
    );
}
