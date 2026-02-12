use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropzone_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/dropzone/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Dropzone internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn dropzone_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/dropzone/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Dropzone;"),
        "dropzone module should export `Dropzone`.",
    );
    assert!(
        crate_source.contains("pub use dropzone::Dropzone;"),
        "crate root should re-export `Dropzone`.",
    );
}

#[test]
fn dropzone_logic_exposes_state_helpers() {
    let source = load_source("src/dropzone/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(input: DropzoneStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: DropzoneState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Dropzone logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn dropzone_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/dropzone/view.rs");

    for needle in [
        "logic::resolve_label(label)",
        "logic::resolve_aria_label(&label, aria_label)",
        "logic::resolve_state(DropzoneStateInput {",
        "logic::compose_class_name(class_name, state)",
        "data-slot=\"dropzone\"",
        "data-state=state.state_attr",
        "data-label-source=state.label_source_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-drop-handler-source=state.drop_handler_source_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Dropzone view should include `{needle}` for stable marker contracts.",
        );
    }
}

#[test]
fn dropzone_styles_include_state_and_source_markers() {
    let source = load_source("src/dropzone/styles.rs");

    for selector in [
        ".ui-dropzone {",
        ".ui-dropzone[data-state=\"disabled\"]",
        ".ui-dropzone[data-label-source=\"custom\"]",
        ".ui-dropzone[data-aria-source=\"custom\"]",
        ".ui-dropzone[data-drop-handler-source=\"custom\"]",
        ".ui-dropzone[data-motion-source=\"custom\"]",
        ".ui-dropzone[data-custom-motion=\"true\"]",
        ".ui-dropzone--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Dropzone styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn dropzone_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::dropzone::styles::CSS);"),
        "ui-components css aggregator should include dropzone styles.",
    );
}

#[test]
fn dropzone_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/files_extra_dropzone.rs");

    for needle in [
        "pub(super) fn dropzone() -> AnyView",
        "title=\"Dropzone\"",
        "slug=\"dropzone\"",
        "State + Source Markers",
        "data-drop-handler-source",
    ] {
        assert!(
            source.contains(needle),
            "files_extra_dropzone docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn dropzone_docs_disabled_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/files_extra_dropzone.rs");

    for needle in [
        "title=\"Disabled\"",
        "label=\"Disabled\".to_string()",
        "disabled=true",
        "\"Dropzone disabled\"",
        "\"No pointer or drop interactions\"",
    ] {
        assert!(
            source.contains(needle),
            "Dropzone docs disabled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn dropzone_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/files_extra_dropzone.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "label=\"Asset upload\".to_string()",
        "aria_label=\"Asset upload area\".to_string()",
        "class_name=\"docs-dropzone-state\".to_string()",
        "let mut marker_motion = DropZoneMotion::default();",
        "marker_motion.hover_scale = 1.02",
        "marker_motion.drop_scale = 1.01",
        "motion=marker_motion",
        "on_drop_files=marker_on_drop_files",
        "\"Inspect root source/state markers\"",
        "Inspect root markers like `data-state`, `data-label-source`, `data-aria-source`, `data-drop-handler-source`, `data-class-source`, and `data-motion-source`.",
    ] {
        assert!(
            source.contains(needle),
            "Dropzone docs state/source playground should contain `{needle}`.",
        );
    }
}
