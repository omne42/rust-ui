use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn content_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/content/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Content internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn content_uses_logic_state_model() {
    let logic_source = load_source("src/content/logic.rs");
    let render_source = load_source("src/content/view.rs");

    for needle in [
        "pub enum ContentTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Content logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ContentStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Content render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn content_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/content/view.rs");

    for attr in [
        "data-slot=\"content\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-padded=move || state.get().is_padded.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Content should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn content_styles_include_tone_padding_and_custom_markers() {
    let source = load_source("src/content/styles.rs");

    for selector in [
        ".ui-content--tone-default",
        ".ui-content[data-tone=\"default\"]",
        ".ui-content--tone-muted",
        ".ui-content[data-tone=\"muted\"]",
        ".ui-content--padded",
        ".ui-content[data-padded=\"true\"]",
        ".ui-content--custom-class",
        ".ui-content[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Content styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn content_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn content() -> AnyView",
        "title=\"Content\"",
        "slug=\"content\"",
        "Playground title=\"Semantic Section + Tone\"",
        "Playground title=\"Padded + Custom Aria/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Content.",
        );
    }
}

#[test]
fn content_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic Section + Tone\"",
        "<Content>",
        "<Content tone=ContentTone::Muted>",
        "title=\"Padded + Custom Aria/Class\"",
        "padded=true",
        "aria_label=\"Dialog content\".to_string()",
        "class_name=\"docs-content-custom\".to_string()",
        "Verifies padding marker + custom class source contract.",
    ] {
        assert!(
            source.contains(needle),
            "content docs playgrounds should contain `{needle}`.",
        );
    }
}
