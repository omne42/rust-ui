use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn shared_element_transition_module_reexports_view_contracts() {
    let source = load_source("src/shared_element_transition/mod.rs");

    for needle in [
        "pub use crate::view::View as SharedElementTransition;",
        "pub use crate::view::ViewElement as SharedElementTransitionElement;",
        "pub use crate::view::ViewRadius as SharedElementTransitionRadius;",
    ] {
        assert!(
            source.contains(needle),
            "shared_element_transition module should expose `{needle}` for react-aria-components SharedElementTransition compatibility.",
        );
    }
}

#[test]
fn crate_root_registers_shared_element_transition_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod shared_element_transition;",
        "pub use shared_element_transition::{",
        "SharedElementTransition, SharedElementTransitionElement, SharedElementTransitionRadius,",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for shared-element-transition compatibility.",
        );
    }
}

#[test]
fn shared_element_transition_compatibility_reuses_view_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in ["title=\"View\"", "slug=\"view\"", "<View"] {
        assert!(
            source.contains(needle),
            "layout docs should contain `{needle}` for shared-element-transition compatibility coverage.",
        );
    }
}
