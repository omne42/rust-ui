use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_code_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/code").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_code_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-code\")]")
            && lib_source.contains("pub use ui_code as code;"),
        "ui-components should re-export the external ui-code crate as `code`.",
    );
    assert!(
        cargo_source.contains("component-code = [\"dep:ui-code\"]"),
        "component-code feature should depend on dep:ui-code after extraction.",
    );
    assert!(
        cargo_source.contains("ui-code = { path = \"../../components/code\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-code dependency.",
    );
}

#[test]
fn code_does_not_expose_logic_or_view_modules() {
    let source = load_code_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Code internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn code_uses_logic_state_model() {
    let view_source = load_code_component_source("src/view.rs");
    let logic_source = load_code_component_source("src/logic.rs");

    for needle in [
        "pub struct CodeStateInput",
        "pub struct CodeState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: CodeStateInput)",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Code logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CodeStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Code view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn code_emits_baseline_style_state_data_attributes() {
    let source = load_code_component_source("src/view.rs");

    for attr in [
        "data-slot=\"code\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-inline=state.is_inline.then_some(\"true\")",
        "data-block=state.is_block.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Code should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn code_styles_include_variant_and_state_markers() {
    let source = load_code_component_source("src/styles.rs");

    for selector in [
        ".ui-code--variant-inline",
        ".ui-code[data-variant=\"inline\"]",
        ".ui-code--variant-block",
        ".ui-code[data-variant=\"block\"]",
        ".ui-code--state-inline",
        ".ui-code[data-state=\"block\"]",
        ".ui-code[data-block=\"true\"]",
        ".ui-code--custom-class",
        ".ui-code[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Code styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn code_keeps_code_element_slot_contract() {
    let source = load_code_component_source("src/view.rs");

    for needle in ["<code", "data-slot=\"code\""] {
        assert!(
            source.contains(needle),
            "Code view should retain `{needle}` markup contract."
        );
    }
}

#[test]
fn code_docs_page_covers_primary_playgrounds() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn code() -> AnyView",
        "title=\"Code\"",
        "slug=\"code\"",
        "Playground title=\"Variant Matrix\"",
        "Playground title=\"Custom Class + Block\"",
        "test_source_path=\"components/code/src/styles.rs\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Code.",
        );
    }
}

#[test]
fn code_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variant Matrix\"",
        "<Code variant=CodeVariant::Inline>\"cargo test -p ui-components\"</Code>",
        "<Code variant=CodeVariant::Block>",
        "cargo fmt --all",
        "cargo clippy -p ui-components -p docs-app --all-targets -- -D warnings",
        "title=\"Custom Class + Block\"",
        "<Code variant=CodeVariant::Inline class_name=\"docs-code-custom\".to_string()>",
        "\"--deny warnings\"",
        "<Code variant=CodeVariant::Block class_name=\"docs-code-custom\".to_string()>",
        "cargo test -p ui-components --test code_semantics",
    ] {
        assert!(
            source.contains(needle),
            "code docs playgrounds should contain `{needle}`.",
        );
    }
}
