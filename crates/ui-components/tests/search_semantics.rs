use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/search/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Search internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn search_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/search/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Search;"),
        "search module should export `Search`.",
    );
    assert!(
        crate_source.contains("pub use search::Search;"),
        "crate root should re-export `Search`.",
    );
}

#[test]
fn search_logic_exposes_state_helpers() {
    let source = load_source("src/search/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(value: String)",
        "pub fn resolve_state(input: SearchStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: SearchState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Search logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn search_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/search/view.rs");

    for needle in [
        "logic::resolve_label(label)",
        "logic::resolve_state(SearchStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "data-slot=\"search\"",
        "data-state=move || state.get().state_attr",
        "data-value=move || state.get().value_attr",
        "data-requirement=move || state.get().requirement_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-submit-handler-source=move || state.get().submit_handler_source_attr",
        "data-clear-handler-source=move || state.get().clear_handler_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Search view should include `{needle}` for stable marker contracts.",
        );
    }
}

#[test]
fn search_styles_include_state_and_source_markers() {
    let source = load_source("src/search/styles.rs");

    for selector in [
        ".ui-search {",
        ".ui-search[data-state=\"disabled\"]",
        ".ui-search[data-state=\"invalid\"]",
        ".ui-search[data-state=\"readonly\"]",
        ".ui-search[data-value=\"filled\"]",
        ".ui-search[data-requirement=\"required\"]",
        ".ui-search[data-label-source=\"custom\"]",
        ".ui-search[data-description-source=\"custom\"]",
        ".ui-search[data-error-source=\"custom\"]",
        ".ui-search[data-placeholder-source=\"custom\"]",
        ".ui-search[data-submit-handler-source=\"custom\"]",
        ".ui-search[data-clear-handler-source=\"custom\"]",
        ".ui-search[data-motion-source=\"custom\"]",
        ".ui-search[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Search styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn search_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::search::styles::CSS);"),
        "ui-components css aggregator should include search styles.",
    );
}

#[test]
fn search_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "pub(super) fn search() -> AnyView",
        "title=\"Search\"",
        "slug=\"search\"",
        "State + Source Markers",
        "data-submit-handler-source",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_search docs page should contain `{needle}`.",
        );
    }
}
