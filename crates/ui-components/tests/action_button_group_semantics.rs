use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_button_group_does_not_expose_logic_module() {
    let source = load_source("src/action_button_group/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "ActionButtonGroup's `logic` module should stay non-public to avoid leaking implementation details into the public API."
    );
}

#[test]
fn action_button_group_provides_context_for_child_buttons() {
    let source = load_source("src/action_button_group/view.rs");

    for needle in ["provide_context", "ActionButtonGroupContextValue"] {
        assert!(
            source.contains(needle),
            "ActionButtonGroup should provide a context value via `{needle}` so child ActionButton instances can inherit group config."
        );
    }
}

#[test]
fn action_button_group_emits_toolbar_semantics_and_state_attributes() {
    let source = load_source("src/action_button_group/view.rs");

    for needle in [
        "data-slot=\"action-button-group\"",
        "data-disabled",
        "role=\"toolbar\"",
        "aria-orientation",
        "aria-disabled",
    ] {
        assert!(
            source.contains(needle),
            "ActionButtonGroup should set `{needle}` to align with Spectrum toolbar semantics and enable state-driven styling."
        );
    }
}
