use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icons_workflow_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon/workflow/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "IconsWorkflow internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icons_workflow_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icon/workflow/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconsWorkflow;"),
        "icons_workflow module should export `IconsWorkflow`."
    );
    assert!(
        module_source.contains("pub struct IconsWorkflowStateInput"),
        "icons_workflow module should expose `IconsWorkflowStateInput` contract."
    );
    assert!(
        crate_source.contains(
            "pub use icons_workflow::{IconsWorkflow, IconsWorkflowSize, IconsWorkflowTone};"
        ),
        "crate root should re-export `IconsWorkflow` contracts."
    );
}

#[test]
fn icons_workflow_logic_exposes_state_helpers() {
    let source = load_source("src/icon/workflow/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_icon_reference(icon: String)",
        "pub fn default_workflow_glyphs() -> Vec<IconsetGlyph>",
        "pub fn resolve_state(input: IconsWorkflowStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconsWorkflowState)",
    ] {
        assert!(
            source.contains(needle),
            "IconsWorkflow logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn icons_workflow_view_uses_logic_state_contracts() {
    let source = load_source("src/icon/workflow/view.rs");

    for needle in [
        "pub fn IconsWorkflow(",
        "logic::normalize_icon_reference(icon)",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(aria_label)",
        "logic::resolve_state(IconsWorkflowStateInput {",
        "logic::compose_class_name(class_name_for_wrapper, state)",
        "logic::default_workflow_glyphs()",
        "<Iconset",
        "iconset=\"workflow\".to_string()",
        "data-slot=\"icons-workflow\"",
        "data-state=state.state_attr",
        "data-icon-reference-source=state.icon_reference_source_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-glyph-source=state.glyph_source_attr",
        "data-size-source=state.size_source_attr",
        "data-tone-source=state.tone_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "IconsWorkflow view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn icons_workflow_styles_include_state_and_source_markers() {
    let source = load_source("src/icon/workflow/styles.rs");

    for selector in [
        ".ui-icons-workflow {",
        ".ui-icons-workflow[data-state=\"disabled\"]",
        ".ui-icons-workflow[data-state=\"decorative\"]",
        ".ui-icons-workflow[data-icon-reference-source=\"default\"]",
        ".ui-icons-workflow[data-icon-reference-source=\"explicit\"]",
        ".ui-icons-workflow[data-icon-reference-source=\"prefixed\"]",
        ".ui-icons-workflow[data-aria-source=\"custom\"]",
        ".ui-icons-workflow[data-class-source=\"custom\"]",
        ".ui-icons-workflow[data-glyph-source=\"custom\"]",
        ".ui-icons-workflow[data-size-source=\"custom\"]",
        ".ui-icons-workflow[data-tone-source=\"custom\"]",
        ".ui-icons-workflow--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "IconsWorkflow styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn icons_workflow_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::icons_workflow::styles::CSS);"),
        "ui-components css aggregator should include icons_workflow styles."
    );
}

#[test]
fn icons_workflow_docs_page_contains_state_source_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "pub(super) fn icons_workflow() -> AnyView",
        "title=\"IconsWorkflow\"",
        "slug=\"icons-workflow\"",
        "State + Source Markers",
        "data-tone-source",
        "<IconsWorkflow",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_workflow docs page should contain `{needle}`."
        );
    }
}

#[test]
fn icons_workflow_docs_default_and_custom_playgrounds_lock_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "<Playground title=\"Built-in Workflow Glyphs\" code_signal=defaults_code>",
        "icon=\"success\".to_string()",
        "icon=\"warning\".to_string()",
        "size=IconsWorkflowSize::Md",
        "tone=IconsWorkflowTone::Accent",
        "tone=IconsWorkflowTone::Danger",
        "decorative=false",
        "<Playground title=\"Custom Workflow Extension\" code_signal=custom_code>",
        "icon=\"workflow:deploy\".to_string()",
        "IconsetGlyph::new(\"workflow:deploy\", \"🚀\")",
        ".with_aria_label(\"Workflow Deploy\")",
        "size=IconsWorkflowSize::Lg",
        "tone=IconsWorkflowTone::Default",
        "class_name=\"docs-icons-workflow-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_workflow docs default/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_workflow_docs_state_source_playground_locks_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "title=\"State + Source Markers\"",
        "icon=\"success\".to_string()",
        "IconsetGlyph::new(\"workflow:success\", \"✓\")",
        ".with_aria_label(\"Workflow Success\")",
        "size=IconsWorkflowSize::Lg",
        "tone=IconsWorkflowTone::Muted",
        "decorative=false",
        "aria_label=\"Explicit workflow success icon\".to_string()",
        "class_name=\"docs-icons-workflow-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_workflow docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn icons_workflow_docs_page_covers_primary_playgrounds() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "pub(super) fn icons_workflow() -> AnyView",
        "title=\"IconsWorkflow\"",
        "slug=\"icons-workflow\"",
        "description=\"baseline-compatible icons-workflow wrapper with workflow namespace normalization, built-in workflow glyph defaults, and Iconset accessibility/source-state contracts.\"",
        "<Playground title=\"Built-in Workflow Glyphs\" code_signal=defaults_code>",
        "<Playground title=\"Custom Workflow Extension\" code_signal=custom_code>",
        "title=\"State + Source Markers\"",
        "<IconsWorkflow",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_icons_workflow docs should include `{needle}` for icons_workflow primary playground coverage.",
        );
    }
}

#[test]
fn icons_workflow_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra_icons_workflow.rs",
    );

    for needle in [
        "title=\"Built-in Workflow Glyphs\"",
        "icon=\"success\".to_string()",
        "icon=\"warning\".to_string()",
        "size=IconsWorkflowSize::Md",
        "tone=IconsWorkflowTone::Accent",
        "tone=IconsWorkflowTone::Danger",
        "title=\"Custom Workflow Extension\"",
        "icon=\"workflow:deploy\".to_string()",
        "IconsetGlyph::new(\"workflow:deploy\", \"🚀\")",
        ".with_aria_label(\"Workflow Deploy\")",
        "size=IconsWorkflowSize::Lg",
        "tone=IconsWorkflowTone::Default",
        "class_name=\"docs-icons-workflow-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "icon=\"success\".to_string()",
        "IconsetGlyph::new(\"workflow:success\", \"✓\")",
        ".with_aria_label(\"Workflow Success\")",
        "tone=IconsWorkflowTone::Muted",
        "aria_label=\"Explicit workflow success icon\".to_string()",
        "class_name=\"docs-icons-workflow-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icons_workflow docs playgrounds should contain `{needle}`.",
        );
    }
}
