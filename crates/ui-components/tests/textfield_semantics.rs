use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn textfield_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/textfield/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Textfield internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn textfield_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/textfield/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Textfield;"),
        "textfield module should export `Textfield`.",
    );
    assert!(
        crate_source.contains("pub use textfield::Textfield;"),
        "crate root should re-export `Textfield`.",
    );
}

#[test]
fn textfield_logic_exposes_state_helpers() {
    let source = load_source("src/textfield/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(value: String)",
        "pub fn resolve_input_type(value: Option<&'static str>)",
        "pub fn resolve_state(input: TextfieldStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: TextfieldState)",
        "DEFAULT_LABEL",
        "DEFAULT_INPUT_TYPE",
    ] {
        assert!(
            source.contains(needle),
            "Textfield logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn textfield_view_uses_logic_state_contracts() {
    let source = load_source("src/textfield/view.rs");

    for needle in [
        "logic::resolve_label(label)",
        "logic::resolve_input_type(input_type)",
        "logic::resolve_state(TextfieldStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "data-slot=\"textfield\"",
        "data-state=move || state.get().state_attr",
        "data-value=move || state.get().value_attr",
        "data-requirement=move || state.get().requirement_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-type-source=move || state.get().type_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Textfield view should include `{needle}` for stable marker contracts.",
        );
    }
}

#[test]
fn textfield_styles_include_state_and_source_markers() {
    let source = load_source("src/textfield/styles.rs");

    for selector in [
        ".ui-textfield {",
        ".ui-textfield[data-state=\"disabled\"]",
        ".ui-textfield[data-state=\"invalid\"]",
        ".ui-textfield[data-state=\"readonly\"]",
        ".ui-textfield[data-value=\"filled\"]",
        ".ui-textfield[data-requirement=\"required\"]",
        ".ui-textfield[data-label-source=\"custom\"]",
        ".ui-textfield[data-description-source=\"custom\"]",
        ".ui-textfield[data-error-source=\"custom\"]",
        ".ui-textfield[data-placeholder-source=\"custom\"]",
        ".ui-textfield[data-type-source=\"custom\"]",
        ".ui-textfield--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Textfield styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn textfield_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::textfield::styles::CSS);"),
        "ui-components css aggregator should include textfield styles.",
    );
}

#[test]
fn textfield_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_textfield.rs");

    for needle in [
        "pub(super) fn textfield() -> AnyView",
        "title=\"Textfield\"",
        "slug=\"textfield\"",
        "State + Source Markers",
        "data-type-source",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_textfield docs page should contain `{needle}`.",
        );
    }
}
