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
