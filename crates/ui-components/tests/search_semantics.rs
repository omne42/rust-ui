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

#[test]
fn search_docs_required_invalid_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "title=\"Required + Invalid\"",
        "id=\"docs-search-required\".to_string()",
        "label=\"Required query\".to_string()",
        "required=true",
        "invalid=required_invalid",
        "error=\"Query is required\".to_string()",
        "placeholder=\"Type a query\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Search docs required/invalid playground should contain `{needle}`.",
        );
    }
}

#[test]
fn search_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-search-markers\".to_string()",
        "label=\"Search runtime docs\".to_string()",
        "required=true",
        "invalid=Signal::derive(move || marker_invalid.get())",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Query is required\".to_string()",
        "placeholder=\"Try: spring\".to_string()",
        "class_name=\"docs-search-state\".to_string()",
        "let mut marker_motion = SearchFieldMotion::default();",
        "marker_motion.hidden_scale = 0.78",
        "marker_motion.hover_scale = 1.08",
        "marker_motion.tap_scale = 0.92",
        "motion=marker_motion",
        "Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, `data-submit-handler-source`, `data-clear-handler-source`, and `data-motion-source`.",
    ] {
        assert!(
            source.contains(needle),
            "Search docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn search_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "pub(super) fn search() -> AnyView",
        "title=\"Search\"",
        "slug=\"search\"",
        "description=\"Spectrum-compatible Search alias for upstream naming parity, preserving SearchField accessibility/state contracts and HeroUI-level clear-button spring motion.\"",
        "<Playground title=\"Submit + Clear\" code=basic_code>",
        "<Playground title=\"Required + Invalid\" code=validation_code>",
        "title=\"State + Source Markers\"",
        "code=markers_code",
        "<Search",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_search docs should include `{needle}` for search primary playground coverage.",
        );
    }
}

#[test]
fn search_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_search.rs");

    for needle in [
        "title=\"Submit + Clear\"",
        "id=\"docs-search-basic\".to_string()",
        "label=\"Search docs\".to_string()",
        "placeholder=\"Try: overlay\".to_string()",
        "on_submit=on_submit",
        "on_clear=on_clear",
        "Press Enter to submit; Escape to clear.",
        "title=\"Required + Invalid\"",
        "id=\"docs-search-required\".to_string()",
        "label=\"Required query\".to_string()",
        "required=true",
        "invalid=required_invalid",
        "error=\"Query is required\".to_string()",
        "placeholder=\"Type a query\".to_string()",
        "title=\"State + Source Markers\"",
        "id=\"docs-search-markers\".to_string()",
        "label=\"Search runtime docs\".to_string()",
        "required=true",
        "invalid=Signal::derive(move || marker_invalid.get())",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Query is required\".to_string()",
        "placeholder=\"Try: spring\".to_string()",
        "class_name=\"docs-search-state\".to_string()",
        "motion=marker_motion",
        "hidden_scale: 0.78",
        "hover_scale: 1.08",
        "tap_scale: 0.92",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_search docs playgrounds should contain `{needle}` for search contracts.",
        );
    }
}
