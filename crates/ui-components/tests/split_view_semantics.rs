use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn split_view_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/split_view/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SplitView internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn split_view_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/split_view/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::SplitView;"),
        "split_view module should export `SplitView`."
    );
    assert!(
        crate_source.contains("pub use split_view::SplitView;"),
        "crate root should re-export `SplitView`."
    );
}

#[test]
fn split_view_logic_exposes_state_helpers() {
    let source = load_source("src/split_view/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(value: Option<String>)",
        "pub fn resolve_state(input: SplitViewStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: SplitViewState)",
        "pub fn default_split_percent(value: Option<f64>) -> f64",
        "pub fn has_custom_bounds(min_split_percent: f64, max_split_percent: f64) -> bool",
        "DEFAULT_SPLIT_PERCENT",
    ] {
        assert!(
            source.contains(needle),
            "SplitView logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn split_view_view_uses_logic_state_contracts() {
    let source = load_source("src/split_view/view.rs");

    for needle in [
        "pub fn SplitView(",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::default_split_percent(default_split_percent)",
        "logic::has_custom_bounds(min_split_percent, max_split_percent)",
        "logic::resolve_state(SplitViewStateInput {",
        "logic::compose_class_name(class_name_for_wrapper.clone(), state.get())",
        "<Resizable",
        "split_percent: Option<Signal<f64>>",
        "data-slot=\"split-view\"",
        "data-state=move || state.get().state_attr",
        "data-orientation=move || state.get().orientation_attr",
        "data-split-mode=move || state.get().split_mode_attr",
        "data-handle=move || state.get().handle_attr",
        "data-default-split-source=move || state.get().default_split_source_attr",
        "data-bounds-source=move || state.get().bounds_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-handler-source=move || state.get().handler_source_attr",
        "data-custom-default=move || state.get().has_custom_default_split.then_some(\"true\")",
        "data-custom-bounds=move || state.get().has_custom_bounds.then_some(\"true\")",
        "data-custom-label=move || state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-handler=move || state.get().has_custom_change_handler.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "SplitView view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn split_view_styles_include_state_and_source_markers() {
    let source = load_source("src/split_view/styles.rs");

    for selector in [
        ".ui-split-view {",
        ".ui-split-view[data-orientation=\"horizontal\"]",
        ".ui-split-view[data-orientation=\"vertical\"]",
        ".ui-split-view[data-state=\"disabled\"]",
        ".ui-split-view[data-split-mode=\"controlled\"]",
        ".ui-split-view[data-split-mode=\"uncontrolled\"]",
        ".ui-split-view[data-handle=\"with-handle\"]",
        ".ui-split-view[data-handle=\"plain\"]",
        ".ui-split-view[data-default-split-source=\"custom\"]",
        ".ui-split-view[data-bounds-source=\"custom\"]",
        ".ui-split-view[data-label-source=\"custom\"]",
        ".ui-split-view[data-class-source=\"custom\"]",
        ".ui-split-view[data-handler-source=\"custom\"]",
        ".ui-split-view--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "SplitView styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn split_view_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::split_view::styles::CSS);"),
        "ui-components css aggregator should include split_view styles."
    );
}

#[test]
fn split_view_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_split_view.rs");

    for needle in [
        "pub(super) fn split_view() -> AnyView",
        "title=\"SplitView\"",
        "slug=\"split-view\"",
        "State + Source Markers",
        "data-handler-source",
        "<SplitView",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_split_view docs page should contain `{needle}`."
        );
    }
}
