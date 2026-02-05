use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn form_does_not_accept_an_unused_motion_prop() {
    let source = load_source("src/form/view.rs");

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
    let source = load_source("src/form/mod.rs");

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
        "`ui-components` should not re-export a placeholder `FormMotion` type."
    );
}
