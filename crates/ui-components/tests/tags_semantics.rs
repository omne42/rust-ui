use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tags_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tags/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tags internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tags_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/tags/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Tags;"),
        "tags module should export `Tags`."
    );
    assert!(
        crate_source.contains("pub use tags::Tags;"),
        "crate root should re-export `Tags`."
    );
}

#[test]
fn tags_logic_exposes_state_helpers() {
    let source = load_source("src/tags/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn derive_tag_flags(",
        "pub fn resolve_state(input: TagsStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: TagsState)",
    ] {
        assert!(
            source.contains(needle),
            "Tags logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn tags_view_uses_logic_state_contracts() {
    let source = load_source("src/tags/view.rs");

    for needle in [
        "pub fn Tags(",
        "logic::normalize_optional_text(id_base)",
        "logic::normalize_optional_text(aria_label)",
        "logic::derive_tag_flags(&tags, disabled, has_remove_handler)",
        "logic::resolve_state(TagsStateInput {",
        "logic::compose_class_name(class_name_for_wrapper.clone(), state.get())",
        "<TagGroup",
        "on_remove: Option<Callback<Tag>>",
        "data-slot=\"tags\"",
        "data-state=move || state.get().state_attr",
        "data-content=move || state.get().content_attr",
        "data-removal=move || state.get().removal_attr",
        "data-constraint=move || state.get().constraint_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-describedby-source=move || state.get().describedby_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-variant-source=move || state.get().variant_source_attr",
        "data-size-source=move || state.get().size_source_attr",
        "data-handler-source=move || state.get().handler_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tags view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn tags_styles_include_state_and_source_markers() {
    let source = load_source("src/tags/styles.rs");

    for selector in [
        ".ui-tags {",
        ".ui-tags[data-state=\"disabled\"]",
        ".ui-tags[data-state=\"empty\"]",
        ".ui-tags[data-content=\"filled\"]",
        ".ui-tags[data-removal=\"removable\"]",
        ".ui-tags[data-removal=\"static\"]",
        ".ui-tags[data-constraint=\"invalid\"]",
        ".ui-tags[data-constraint=\"required\"]",
        ".ui-tags[data-label-source=\"custom\"]",
        ".ui-tags[data-description-source=\"custom\"]",
        ".ui-tags[data-error-source=\"custom\"]",
        ".ui-tags[data-describedby-source=\"custom\"]",
        ".ui-tags[data-aria-source=\"custom\"]",
        ".ui-tags[data-class-source=\"custom\"]",
        ".ui-tags[data-variant-source=\"custom\"]",
        ".ui-tags[data-size-source=\"custom\"]",
        ".ui-tags[data-handler-source=\"custom\"]",
        ".ui-tags--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Tags styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn tags_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::tags::styles::CSS);"),
        "ui-components css aggregator should include tags styles."
    );
}

#[test]
fn tags_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_tags.rs");

    for needle in [
        "pub(super) fn tags() -> AnyView",
        "title=\"Tags\"",
        "slug=\"tags\"",
        "State + Source Markers",
        "data-handler-source",
        "<Tags",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_tags docs page should contain `{needle}`."
        );
    }
}

#[test]
fn tags_docs_default_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_tags.rs");

    for needle in [
        "<Playground title=\"Removable Tags\" code_signal=removable_code>",
        "<Tags",
        "tags=tags",
        "on_remove=on_remove",
        "label=\"Technologies\".to_string()",
        "description=\"Remove enabled tags; disabled tags remain.\".to_string()",
        "<Playground title=\"Disabled Tags\" code_signal=states_code>",
        "tags=static_tags",
        "disabled=true",
        "label=\"Disabled tags\".to_string()",
        "description=\"Read-only tag collection\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "tags docs default playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn tags_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_tags.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "variant=TagVariant::Surface",
        "size=TagSize::Sm",
        "id_base=\"docs-tags-markers\".to_string()",
        "label=\"Marker tags\".to_string()",
        "description=\"Inspect tags wrapper markers\".to_string()",
        "error=\"Keep at least two tags\".to_string()",
        "invalid=marker_invalid",
        "required=true",
        "aria_describedby=Signal::derive(move || Some(\"tags-hint\".to_string()))",
        "aria_label=\"Marker tag list\".to_string()",
        "class_name=\"docs-tags-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "tags docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn tags_docs_page_covers_primary_playgrounds() {
    tags_docs_page_contains_state_source_playground();
}

#[test]
fn tags_docs_playgrounds_lock_state_matrix_contract_values() {
    tags_docs_default_playgrounds_lock_contract_values();
    tags_docs_state_source_playground_locks_contract_values();
}
