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
fn form_does_not_accept_an_unused_motion_prop() {
    let source = load_source("../../components/form/src/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "Form should not ignore a motion contract. If Form ever grows motion, it must attach a driver."
    );
    assert!(
        !source.contains("motion:"),
        "Form should not accept a `motion` prop unless it has a real motion contract."
    );
}

#[test]
fn form_module_does_not_export_a_placeholder_motion_contract() {
    let source = load_source("../../components/form/src/mod.rs");

    assert!(
        !source.contains("pub mod motion"),
        "Form should not expose a placeholder `motion` module."
    );
    assert!(
        !source.contains("FormMotion"),
        "Form should not export a placeholder `FormMotion` contract."
    );
}

#[test]
fn ui_components_does_not_reexport_form_motion() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("FormMotion"),
        "`ui` should not re-export a placeholder `FormMotion` type."
    );
}

#[test]
fn form_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn form() -> AnyView",
        "title=\"Form\"",
        "slug=\"form\"",
        "description=\"A context provider for form-wide disabled/required/label layout.\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
        "test_css_source=form_test_css_source",
        "test_config_signal=workbench_config",
        "<Form",
    ] {
        assert!(
            source.contains(needle),
            "forms docs page should include `{needle}` for form primary playground coverage.",
        );
    }
}

#[test]
fn form_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "id_base=\"docs-form-label-position\".to_string()",
        "id_base=\"docs-form-label-align\".to_string()",
        "is_required=true",
        "is_disabled=true",
        "is_read_only=true",
        "label_position=FormLabelPosition::Left",
        "label_align=FormLabelAlign::End",
        "class_name=\"docs-form-custom\".to_string()",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
        "id=\"docs-form-matrix-default\".to_string()",
        "id=\"docs-form-matrix-required\".to_string()",
        "id=\"docs-form-matrix-disabled\".to_string()",
        "id=\"docs-form-matrix-readonly\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "form docs playground should contain `{needle}`."
        );
    }
}
