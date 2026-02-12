use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TextArea internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn text_area_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/text_area/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::TextArea;"),
        "text_area module should export `TextArea`.",
    );
    assert!(
        crate_source.contains("pub use text_area::TextArea;"),
        "crate root should re-export `TextArea`.",
    );
}

#[test]
fn text_area_logic_exposes_state_helpers() {
    let source = load_source("src/text_area/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(value: String)",
        "pub fn resolve_state(input: TextAreaStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: TextAreaState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "TextArea logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn text_area_view_uses_logic_state_and_a11y_contracts() {
    let source = load_source("src/text_area/view.rs");

    for needle in [
        "use_focus_ring",
        "use_text_field",
        "logic::resolve_label(label)",
        "logic::resolve_state(TextAreaStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "data-slot=\"text-area\"",
        "data-state=move || state.get().state_attr",
        "data-value=move || state.get().value_attr",
        "data-requirement=move || state.get().requirement_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-rows-source=move || state.get().rows_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            source.contains(needle),
            "TextArea view should include `{needle}` for stable marker + a11y contracts.",
        );
    }
}

#[test]
fn text_area_styles_include_state_source_and_reduced_motion_markers() {
    let source = load_source("src/text_area/styles.rs");

    for selector in [
        ".ui-text-area[data-state=\"disabled\"]",
        ".ui-text-area[data-state=\"invalid\"]",
        ".ui-text-area[data-state=\"readonly\"]",
        ".ui-text-area[data-value=\"filled\"]",
        ".ui-text-area[data-requirement=\"required\"]",
        ".ui-text-area[data-label-source=\"custom\"]",
        ".ui-text-area[data-description-source=\"custom\"]",
        ".ui-text-area[data-error-source=\"custom\"]",
        ".ui-text-area[data-placeholder-source=\"custom\"]",
        ".ui-text-area[data-rows-source=\"custom\"]",
        ".ui-text-area--custom-class",
        "prefers-reduced-motion: reduce",
        "transition: none;",
    ] {
        assert!(
            source.contains(selector),
            "TextArea styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn text_area_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::text_area::styles::CSS);"),
        "ui-components css aggregator should include text_area styles.",
    );
}

#[test]
fn text_area_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn text_area() -> AnyView",
        "title=\"TextArea\"",
        "slug=\"text-area\"",
        "State + Source Markers",
        "data-rows-source",
    ] {
        assert!(
            source.contains(needle),
            "forms docs page should contain `{needle}` for text-area.",
        );
    }
}

#[test]
fn text_area_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-text-area-markers\".to_string()",
        "label=\"Release notes\".to_string()",
        "required=true",
        "invalid=Signal::derive(move || invalid.get())",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Release notes are required\".to_string()",
        "placeholder=\"Write release notes…\".to_string()",
        "rows=6",
        "class_name=\"docs-text-area-state\".to_string()",
        "Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-rows-source`.",
    ] {
        assert!(
            source.contains(needle),
            "TextArea docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn text_area_docs_page_covers_primary_playgrounds() {
    text_area_docs_page_contains_state_source_playground();
}

#[test]
fn text_area_docs_playgrounds_lock_state_matrix_contract_values() {
    text_area_docs_state_source_playground_locks_contract_values();
}
