use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn provider_module_reexports_root_ui_root_as_provider() {
    let source = load_source("src/provider/mod.rs");

    assert!(
        source.contains("pub use crate::root::UiRoot as Provider;"),
        "provider module should expose `Provider` as an alias of `UiRoot`."
    );
}

#[test]
fn crate_root_registers_provider_module_and_alias() {
    let source = load_source("src/lib.rs");

    for needle in ["pub mod provider;", "pub use provider::Provider;"] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for react-spectrum Provider compatibility."
        );
    }
}

#[test]
fn provider_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
    ] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for Provider compatibility coverage."
        );
    }
}
