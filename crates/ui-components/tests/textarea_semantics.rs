use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn textarea_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/textarea/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Textarea internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn textarea_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/textarea/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Textarea;"),
        "textarea module should export `Textarea`.",
    );
    assert!(
        crate_source.contains("pub use textarea::Textarea;"),
        "crate root should re-export `Textarea`.",
    );
}

#[test]
fn textarea_logic_exposes_state_helpers() {
    let source = load_source("src/textarea/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(value: String)",
        "pub fn resolve_state(input: TextareaStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: TextareaState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Textarea logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn textarea_view_has_textfield_a11y_and_state_contracts() {
    let source = load_source("src/textarea/view.rs");

    for needle in [
        "use_focus_ring",
        "use_text_field",
        "logic::resolve_label(label)",
        "logic::resolve_state(TextareaStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "data-slot=\"textarea\"",
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
            "Textarea view should include `{needle}` to preserve stable contracts.",
        );
    }
}

#[test]
fn textarea_styles_include_state_and_source_selectors() {
    let source = load_source("src/textarea/styles.rs");

    for selector in [
        ".ui-textarea[data-state=\"disabled\"]",
        ".ui-textarea[data-state=\"invalid\"]",
        ".ui-textarea[data-state=\"readonly\"]",
        ".ui-textarea[data-value=\"filled\"]",
        ".ui-textarea[data-requirement=\"required\"]",
        ".ui-textarea[data-label-source=\"custom\"]",
        ".ui-textarea[data-description-source=\"custom\"]",
        ".ui-textarea[data-error-source=\"custom\"]",
        ".ui-textarea[data-placeholder-source=\"custom\"]",
        ".ui-textarea[data-rows-source=\"custom\"]",
        ".ui-textarea--custom-class",
        "prefers-reduced-motion: reduce",
        "transition: none;",
    ] {
        assert!(
            source.contains(selector),
            "Textarea styles should include `{selector}` selector.",
        );
    }
}

#[test]
fn textarea_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::textarea::styles::CSS);"),
        "ui-components css aggregator should include textarea styles.",
    );
}

#[test]
fn textarea_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "title=\"Textarea\"",
        "slug=\"textarea\"",
        "State + Source Markers",
        "data-rows-source",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should contain `{needle}` for textarea.",
        );
    }
}
