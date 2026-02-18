use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn circular_progress_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/circular_progress/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CircularProgress internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn circular_progress_uses_logic_state_model() {
    let view_source = load_source("src/circular_progress/view.rs");
    let logic_source = load_source("src/circular_progress/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/circular_progress.rs");

    for needle in [
        "pub use ui_state_primitives::circular_progress::{",
        "CircularProgressStateInput",
        "DEFAULT_ARIA_LABEL",
        "compose_class_name",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "CircularProgress logic should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub struct CircularProgressStateInput",
        "pub struct CircularProgressState",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Loading\";",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(value: Option<String>, default_aria_label: &str)",
        "pub fn sanitize_dimension(",
        "pub fn compose_style_vars(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "size_source_attr",
        "thickness_source_attr",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "CircularProgress primitives should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label, common.loading_aria_label.as_ref())",
        "logic::resolve_state(CircularProgressStateInput {",
        "logic::compose_class_name(class_name, &state)",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn circular_progress_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/circular_progress/view.rs");

    for attr in [
        "data-slot=\"circular-progress\"",
        "data-state=\"indeterminate\"",
        "data-motion=\"spin\"",
        "data-size-source=state.size_source_attr",
        "data-thickness-source=state.thickness_source_attr",
        "data-label-source=state.label_source_attr",
        "data-custom-size=state.has_custom_size.then_some(\"true\")",
        "data-custom-thickness=state.has_custom_thickness.then_some(\"true\")",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "role=\"progressbar\"",
        "aria-valuemin=\"0\"",
        "aria-valuemax=\"100\"",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "CircularProgress should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn circular_progress_styles_include_state_marker_contracts() {
    let source = load_source("src/circular_progress/styles.rs");

    for selector in [
        ".ui-circular-progress--state-indeterminate",
        ".ui-circular-progress[data-motion=\"spin\"]",
        ".ui-circular-progress--size-custom",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress--thickness-custom",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress--label-custom",
        ".ui-circular-progress[data-label-source=\"custom\"]",
        ".ui-circular-progress--custom-class",
        ".ui-circular-progress[data-custom-class=\"true\"]",
        "--ui-cp-rotation-duration",
        "--ui-button-spinner-duration",
        "prefers-reduced-motion: reduce",
        "animation-duration: 1ms;",
    ] {
        assert!(
            source.contains(selector),
            "CircularProgress styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn circular_progress_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"CircularProgress\"",
        "slug=\"circular-progress\"",
        "Playground title=\"Size + Thickness Matrix\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for CircularProgress.",
        );
    }
}

#[test]
fn circular_progress_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Size + Thickness Matrix\"",
        "<CircularProgress aria_label=\"Loading\".to_string() />",
        "<CircularProgress aria_label=\"Syncing mail\".to_string() size_px=24.0 />",
        "<CircularProgress aria_label=\"Syncing mail\".to_string() thickness_px=3.0 />",
        "size_px=30.0",
        "thickness_px=4.0",
        "title=\"Custom Label + Class\"",
        "aria_label=\"Background refresh\".to_string()",
        "size_px=28.0",
        "thickness_px=3.5",
        "aria_label=\"   \".to_string()",
        "class_name=\"docs-circular-progress-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "circular-progress docs playgrounds should contain `{needle}`.",
        );
    }
}
