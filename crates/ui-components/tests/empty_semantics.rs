use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_empty_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/empty").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_empty_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-empty\")]")
            && lib_source.contains("pub use ui_empty as empty;"),
        "ui-components should re-export the external ui-empty crate as `empty`.",
    );
    assert!(
        cargo_source.contains("component-empty = [\"dep:ui-empty\"]"),
        "component-empty feature should depend on dep:ui-empty after extraction.",
    );
    assert!(
        cargo_source.contains("ui-empty = { path = \"../../components/empty\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-empty dependency.",
    );
}

#[test]
fn empty_does_not_expose_logic_or_view_modules() {
    let source = load_empty_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Empty internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_is_exported_from_module_and_crate_root() {
    let module_source = load_empty_component_source("src/mod.rs");
    let crate_source = load_ui_components_source("src/lib.rs");

    assert!(
        module_source.contains("pub use logic::EmptyMediaVariant;"),
        "empty module should export `EmptyMediaVariant` from logic."
    );
    assert!(
        module_source.contains("pub use view::{"),
        "empty module should export Empty component family."
    );
    assert!(
        crate_source.contains("pub use empty::{"),
        "crate root should re-export Empty component contracts."
    );
}

#[test]
fn empty_logic_exposes_state_helpers() {
    let source = load_empty_component_source("src/logic.rs");

    for needle in [
        "pub enum EmptyMediaVariant",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: EmptyPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: EmptyPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Empty logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn empty_view_uses_logic_state_contracts() {
    let source = load_empty_component_source("src/view.rs");

    for needle in [
        "pub fn Empty(",
        "pub fn EmptyHeader(",
        "pub fn EmptyTitle(",
        "pub fn EmptyDescription(",
        "pub fn EmptyContent(",
        "pub fn EmptyMedia(",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(EmptyPartStateInput {",
        "logic::compose_class_name(class_name, state)",
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-variant-source=state.variant_source_attr",
        "data-variant=state.media_variant_attr",
    ] {
        assert!(
            source.contains(needle),
            "Empty view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn empty_styles_include_state_and_source_markers() {
    let source = load_empty_component_source("src/styles.rs");

    for selector in [
        ".ui-empty {",
        ".ui-empty[data-state=\"root\"]",
        ".ui-empty__header[data-state=\"header\"]",
        ".ui-empty__title[data-state=\"title\"]",
        ".ui-empty__description[data-state=\"description\"]",
        ".ui-empty__content[data-state=\"content\"]",
        ".ui-empty__media[data-state=\"media\"]",
        ".ui-empty__media[data-variant=\"icon\"]",
        ".ui-empty__media[data-variant-source=\"custom\"]",
        ".ui-empty--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Empty styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn empty_css_is_aggregated() {
    let source = load_ui_components_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::empty::styles::CSS);"),
        "ui-components css aggregator should include empty styles."
    );
}

#[test]
fn empty_docs_page_contains_state_source_playground() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
    );

    for needle in [
        "pub(super) fn empty() -> AnyView",
        "title=\"Empty\"",
        "slug=\"empty\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Header / Action / Source Markers)\"",
        "test_css_source=empty_test_css_source",
        "test_config_signal=workbench_config",
        "<Empty",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_empty docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_docs_default_playgrounds_lock_contract_values() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
    );

    for needle in [
        "id_base=\"docs-empty-media-variant\".to_string()",
        "\"Show content action\"",
        "\"Custom root class\"",
        "\"Custom slot classes\"",
        "class_name=\"docs-empty-custom\".to_string()",
        "class_name=\"docs-empty-header\".to_string()",
        "class_name=\"docs-empty-media\".to_string()",
        "<a href=\"#/components/search\">\"Open search\"</a>",
    ] {
        assert!(
            source.contains(needle),
            "empty docs default playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_docs_state_source_playground_locks_contract_values() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
    );

    for needle in [
        "title=\"Comparison Matrix (Header / Action / Source Markers)\"",
        "<span class=\"ui-muted\">\"Header + Icon\"</span>",
        "<span class=\"ui-muted\">\"Content Action\"</span>",
        "<span class=\"ui-muted\">\"State + Source Markers\"</span>",
        "\"No messages\"",
        "\"No deployments\"",
        "<Empty class_name=\"docs-empty-state\".to_string()>",
        "<EmptyHeader class_name=\"docs-empty-header\".to_string()>",
        "variant=EmptyMediaVariant::Icon",
        "class_name=\"docs-empty-media\".to_string()",
        "<EmptyTitle class_name=\"docs-empty-title\".to_string()>",
        "<EmptyDescription class_name=\"docs-empty-description\".to_string()>",
        "<EmptyContent class_name=\"docs-empty-content\".to_string()>",
        "<a href=\"#/components/search\">\"Open search\"</a>",
    ] {
        assert!(
            source.contains(needle),
            "empty docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_docs_page_covers_primary_playgrounds() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
    );

    for needle in [
        "pub(super) fn empty() -> AnyView",
        "title=\"Empty\"",
        "slug=\"empty\"",
        "description=\"baseline-compatible empty-state composition primitives (`Empty*`) with stable slot contracts for header/media/title/description/content layering.\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Header / Action / Source Markers)\"",
        "<Empty",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_empty docs page should include `{needle}` for empty primary playground coverage.",
        );
    }
}

#[test]
fn empty_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
    );

    for needle in [
        "id_base=\"docs-empty-media-variant\".to_string()",
        "title=\"Comparison Matrix (Header / Action / Source Markers)\"",
        "variant=EmptyMediaVariant::Icon",
        "\"No messages\"",
        "\"You're all caught up.\"",
        "class_name=\"docs-empty-custom\".to_string()",
        "\"No deployments\"",
        "\"Create your first release to populate this list.\"",
        "<a href=\"#/components/button\">\"Create deployment\"</a>",
        "class_name=\"docs-empty-state\".to_string()",
        "class_name=\"docs-empty-header\".to_string()",
        "class_name=\"docs-empty-media\".to_string()",
        "class_name=\"docs-empty-title\".to_string()",
        "class_name=\"docs-empty-description\".to_string()",
        "class_name=\"docs-empty-content\".to_string()",
        "<a href=\"#/components/search\">\"Open search\"</a>",
    ] {
        assert!(
            source.contains(needle),
            "empty docs playgrounds should contain `{needle}`.",
        );
    }
}
