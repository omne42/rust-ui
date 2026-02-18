use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn legend_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/legend/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Legend internals should stay private; found `{needle}`."
        );
    }

    assert!(
        source.contains("pub use logic::{"),
        "Legend module should re-export state contracts through logic boundary."
    );
}

#[test]
fn legend_uses_state_primitive_and_headless_contracts() {
    let primitive_source = load_source("../ui-state-primitives/src/legend.rs");
    let headless_source = load_source("../ui-headless/src/legend.rs");
    let logic_source = load_source("src/legend/logic.rs");
    let view_source = load_source("src/legend/view.rs");

    for needle in [
        "pub enum LegendTone",
        "pub struct LegendStateInput",
        "pub struct LegendState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Legend state primitive should include `{needle}`."
        );
    }

    for needle in [
        "pub struct LegendOptions",
        "pub struct LegendAttrs",
        "pub struct LegendContract",
        "pub fn use_legend(",
    ] {
        assert!(
            headless_source.contains(needle),
            "Legend headless contract should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::legend::{",
        "pub fn normalize_required_state(",
        "pub fn normalize_accessibility_state(",
        "pub fn resolve_agent_contract(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, LegendOptions, use_legend};",
        "logic::normalize_required_state(is_required, required)",
        "logic::normalize_accessibility_state(is_disabled, disabled)",
        "let semantics = use_legend(LegendOptions {",
        "data-ui-schema=agent_contract.schema_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Legend view should mount state/headless/agent contracts; missing `{needle}`."
        );
    }
}

#[test]
fn legend_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/legend/view.rs");

    for attr in [
        "data-slot=\"legend\"",
        "data-tone=legend_data_tone",
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-required-source=required_state.required_source_attr",
        "data-disabled-source=accessibility_state.disabled_source_attr",
        "data-text-source=legend_data_text_source",
        "data-indicator-source=legend_data_indicator_source",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
        "data-ui-state=legend_data_state",
        "data-slot=\"legend-text\"",
        "data-slot=\"legend-required\"",
    ] {
        assert!(
            source.contains(attr),
            "Legend should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn legend_styles_include_state_marker_contracts() {
    let source = load_source("src/legend/styles.rs");

    for selector in [
        ".ui-legend--tone-default",
        ".ui-legend[data-tone=\"strong\"]",
        ".ui-legend--required",
        ".ui-legend[data-disabled=\"true\"]",
        ".ui-legend--text-custom",
        ".ui-legend[data-indicator-source=\"custom\"]",
        ".ui-legend--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Legend styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn legend_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn legend() -> AnyView",
        "title=\"Legend\"",
        "slug=\"legend\"",
        "<Playground title=\"Required Legend\" code_signal=required_code>",
        "<Playground title=\"Tone + Custom Indicator + Disabled\" code_signal=states_code>",
    ] {
        assert!(
            source.contains(needle),
            "legend docs should include `{needle}`.",
        );
    }
}

#[test]
fn legend_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "<Legend text=\"Notification settings\".to_string() is_required=true />",
        "text=\"Billing preferences\".to_string()",
        "tone=LegendTone::Muted",
        "is_required=true",
        "required_indicator=\"(required)\".to_string()",
        "class_name=\"docs-legend-custom\".to_string()",
        "text=\"Read-only group\".to_string()",
        "tone=LegendTone::Strong",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "legend docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn legend_readme_is_copy_paste_ready() {
    let source = load_source("src/legend/README.md");

    for needle in [
        "# Legend",
        "## Hello World",
        "<Legend",
        "text=\"Notification settings\".to_string()",
        "## API 约定",
        "is_required",
        "is_disabled",
    ] {
        assert!(
            source.contains(needle),
            "Legend README should include `{needle}`.",
        );
    }
}
