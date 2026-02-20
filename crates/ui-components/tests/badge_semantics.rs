use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_badge_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/badge").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

#[test]
fn ui_components_reexports_badge_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-badge\")]")
            && lib_source.contains("pub use ui_badge as badge;"),
        "ui-components should re-export the external ui-badge crate as `badge`.",
    );
    assert!(
        cargo_source.contains("component-badge = [\"dep:ui-badge\"]"),
        "component-badge feature should depend on dep:ui-badge after extraction.",
    );
    assert!(
        cargo_source.contains("ui-badge = { path = \"../../components/badge\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-badge dependency.",
    );
}

#[test]
fn badge_does_not_expose_logic_or_view_modules() {
    let source = load_badge_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Badge internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn badge_consumes_state_primitives_and_keeps_component_assembly_local() {
    let view_source = load_badge_component_source("src/view.rs");
    let logic_source = load_badge_component_source("src/logic.rs");
    let primitives_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/badge.rs");

    for needle in [
        "pub use ui_state_primitives::badge::{",
        "BadgeState, BadgeStateInput, BadgeVariant, normalize_optional_text, resolve_state,",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Badge logic should include `{needle}` to consume ui-state-primitives and keep only assembly logic."
        );
    }

    for needle in [
        "pub enum BadgeVariant",
        "pub struct BadgeStateInput",
        "pub struct BadgeState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: BadgeStateInput) -> BadgeState",
    ] {
        assert!(
            primitives_source.contains(needle),
            "badge primitive module should define `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(BadgeStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Badge view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn badge_emits_baseline_style_state_data_attributes() {
    let source = load_badge_component_source("src/view.rs");

    for attr in [
        "data-slot=\"badge\"",
        "data-variant=state.variant_attr",
        "data-fill=state.fill_attr",
        "data-state=state.fill_attr",
        "data-solid=state.is_solid.then_some(\"true\")",
        "data-outline=state.is_outline.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=agent_contract.class_source_attr",
        "lang=locale.lang",
        "dir=locale.dir",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state=agent_contract.state_attr",
        "data-ui-source=agent_contract.source_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            source.contains(attr),
            "Badge should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn badge_styles_include_variant_fill_and_custom_class_markers() {
    let source = load_badge_component_source("src/styles.rs");

    for selector in [
        ".ui-badge--variant-default",
        ".ui-badge[data-variant=\"accent\"]",
        ".ui-badge--variant-danger",
        ".ui-badge[data-variant=\"outline\"]",
        ".ui-badge--fill-solid",
        ".ui-badge[data-fill=\"solid\"]",
        ".ui-badge[data-state=\"outline\"]",
        ".ui-badge--custom-class",
        ".ui-badge[data-custom-class=\"true\"]",
        ".ui-badge[data-class-source=\"custom\"]",
        "font-size: var(--ui-font-size-100, 12px);",
    ] {
        assert!(
            source.contains(selector),
            "Badge styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn badge_docs_page_covers_primary_playgrounds() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn badge() -> AnyView",
        "title=\"Badge\"",
        "slug=\"badge\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Variant Matrix\"",
        "Playground title=\"Custom Class + Outline\"",
        "title=\"Badge Workbench (Display + Config + Code + CSS Test)\"",
        "test_source_path=\"components/badge/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Badge.",
        );
    }
}

#[test]
fn badge_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Hello World\"",
        "<Badge>\"New\"</Badge>",
        "title=\"Variant Matrix\"",
        "<Badge variant=BadgeVariant::Default>\"Default\"</Badge>",
        "<Badge variant=BadgeVariant::Accent>\"Accent\"</Badge>",
        "<Badge variant=BadgeVariant::Danger>\"Danger\"</Badge>",
        "<Badge variant=BadgeVariant::Outline>\"Outline\"</Badge>",
        "title=\"Custom Class + Outline\"",
        "variant=BadgeVariant::Accent class_name=\"docs-badge-custom\".to_string()",
        "variant=BadgeVariant::Outline class_name=\"docs-badge-custom\".to_string()",
        "data-slot=\"badge-workbench-controls\"",
        "data-slot=\"badge-workbench-compare\"",
        "\"Scenario compare\"",
        "Switch checked=workbench_custom_class set_checked=set_workbench_custom_class",
        "Switch checked=workbench_rtl set_checked=set_workbench_rtl",
    ] {
        assert!(
            source.contains(needle),
            "badge docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn badge_logic_resolves_agent_contract_and_locale_helpers() {
    let source = load_badge_component_source("src/logic.rs");
    let view_source = load_badge_component_source("src/view.rs");

    for needle in [
        "pub struct BadgeAgentContract",
        "pub fn resolve_agent_contract(state: BadgeState) -> BadgeAgentContract",
        "schema_attr: \"ui.badge.agent-contract\"",
        "stream_support_attr: \"unsupported\"",
        "stream_fallback_attr: \"snapshot\"",
        "stream_mode_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            source.contains(needle),
            "badge logic should expose `{needle}` machine-readable contract marker."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "let agent_contract = logic::resolve_agent_contract(state);",
    ] {
        assert!(
            view_source.contains(needle),
            "badge view should compose locale + agent contract via `{needle}`."
        );
    }
}

#[test]
fn badge_e2e_contract_file_exists_and_uses_semantic_selectors() {
    let rel = "../../e2e/tests/docs_app_badge_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "badge should provide docs-app e2e contract file: `{rel}`."
    );

    let source = load_ui_components_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "data-component=\"badge\"",
        "data-slot=\"badge\"",
        "data-ui-schema",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-class-source",
        "Show code|Hide code",
    ] {
        assert!(
            source.contains(needle),
            "badge e2e contract should include `{needle}`."
        );
    }
}
