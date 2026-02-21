use std::{fs, path::PathBuf};

fn load(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path:?}: {err}"))
}

#[test]
fn fieldset_component_depends_on_layered_kernel_shell_crates() {
    let manifest = load("Cargo.toml");

    for dep in [
        "ui-state-primitives",
        "ui-headless",
        "ui-motion",
        "ui-theme",
    ] {
        assert!(
            manifest.contains(dep),
            "fieldset component should wire layered dependency `{dep}`."
        );
    }
}

#[test]
fn fieldset_public_api_surface_is_stable_and_not_dom_leaky() {
    let mod_source = load("src/mod.rs");

    for expected in ["pub use motion::FieldsetMotion;", "pub use view::Fieldset;"] {
        assert!(
            mod_source.contains(expected),
            "fieldset stable public API should include `{expected}`."
        );
    }

    for forbidden in ["pub mod view", "pub mod logic", "web_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "fieldset public API should not leak `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_view_assembles_logic_headless_and_motion_without_reimplementing_kernels() {
    let view_source = load("src/view.rs");

    for expected in [
        "logic::resolve_view_state(logic::FieldsetViewStateInput {",
        "let a11y = Memo::new(move |_| {",
        "fieldset_attrs(",
        "let motion_style = StoredValue::new(Some(crate::motion::attach_motion(motion)));",
    ] {
        assert!(
            view_source.contains(expected),
            "fieldset view should assemble layered contracts via `{expected}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "ui_state_primitives::fieldset::resolve_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "fieldset view should not directly leak low-level/runtime-specific detail `{forbidden}`."
        );
    }
}

#[test]
fn fieldset_component_has_local_semantics_test_file() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    assert!(
        path.exists(),
        "fieldset should keep semantics regression test in component-local `test/semantics.rs`."
    );
}
