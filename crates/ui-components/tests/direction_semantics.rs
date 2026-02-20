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
fn direction_does_not_expose_view_module() {
    let source = load_source("../../components/direction/src/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Direction internals should stay private; found `pub mod view`."
    );
}

#[test]
fn direction_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/direction/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{DirectionMode, DirectionProvider};"),
        "ui-direction module should export `DirectionMode` and `DirectionProvider`."
    );
    assert!(
        crate_source.contains("pub use ui_direction as direction;"),
        "crate root should re-export ui-direction as `direction` module."
    );
    assert!(
        crate_source.contains("pub use direction::{DirectionMode, DirectionProvider};"),
        "crate root should re-export direction contracts."
    );
}

#[test]
fn direction_provider_exposes_slot_and_dir_contracts() {
    let source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "pub fn DirectionProvider(",
        "DirectionMode",
        "dir=direction.as_attr()",
        "data-slot=\"direction-provider\"",
        "data-direction=direction.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "DirectionProvider should include `{needle}` for stable contract checks."
        );
    }
}

#[test]
fn direction_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "pub(super) fn direction_provider() -> AnyView",
        "title=\"DirectionProvider\"",
        "slug=\"direction-provider\"",
        "<DirectionProvider",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_direction docs page should contain `{needle}`."
        );
    }
}

#[test]
fn direction_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "pub(super) fn direction_provider() -> AnyView",
        "title=\"DirectionProvider\"",
        "slug=\"direction-provider\"",
        "description=\"baseline/Radix-compatible direction context wrapper with normalized `direction`/`dir` props and stable slot + data-direction contracts.\"",
        "<Playground title=\"LTR Direction\" code_signal=ltr_code>",
        "<Playground title=\"RTL Direction + Class\" code_signal=rtl_code>",
        "<DirectionProvider",
        "DirectionMode::Ltr",
        "DirectionMode::Rtl",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_direction docs page should include `{needle}` for direction primary coverage.",
        );
    }
}

#[test]
fn direction_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "<DirectionProvider direction=DirectionMode::Ltr>",
        "\"Name → Value\"",
        "direction=DirectionMode::Rtl",
        "class_name=\"docs-direction-rtl\".to_string()",
        "\"الاسم ← القيمة\"",
    ] {
        assert!(
            source.contains(needle),
            "direction docs playgrounds should contain `{needle}`.",
        );
    }
}
