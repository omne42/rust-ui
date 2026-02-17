use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn shared_element_transition_compat_module_is_removed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/shared_element_transition/mod.rs");
    assert!(!path.exists(), "compat module  should not exist.",);
}

#[test]
fn crate_root_does_not_register_shared_element_transition_compat_module() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("pub mod shared_element_transition;"),
        "crate root should not include legacy.",
    );
}
