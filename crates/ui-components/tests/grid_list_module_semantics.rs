use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn grid_list_compat_module_is_removed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/grid_list/mod.rs");
    assert!(
        !path.exists(),
        "compat module `src/grid_list/mod.rs` should not exist."
    );
}

#[test]
fn crate_root_does_not_register_grid_list_compat_module() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("pub mod grid_list;"),
        "crate root should not include legacy `pub mod grid_list;`.",
    );
}

#[test]
fn docs_component_mapping_does_not_keep_grid_list_alias() {
    let source = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    assert!(
        !source.contains("\"grid-list\" =>"),
        "docs component module mapping should not keep `grid-list` compatibility alias.",
    );
}
