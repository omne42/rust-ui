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
    let css_source = load_ui_components_source("src/css.rs");

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
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("#[cfg(feature = \"component-badge\")]")
            && css_source.contains("out.push_str(crate::badge::styles::CSS);"),
        "ui-components css aggregation for badge should be guarded by inject-css + component-badge.",
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
        "pub fn resolve_render_state(",
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

    let needle = "let render_state = logic::resolve_render_state(variant, class_name);";
    assert!(
        view_source.contains(needle),
        "Badge view should derive wrapper state via logic helpers; missing `{needle}`."
    );
}

#[test]
fn badge_emits_baseline_style_state_data_attributes() {
    let source = load_badge_component_source("src/view.rs");

    for attr in [
        "data-slot=\"badge\"",
        "data-variant=render_state.state.variant_attr",
        "data-fill=render_state.state.fill_attr",
        "data-state=render_state.state.fill_attr",
        "data-solid=render_state.state.is_solid.then_some(\"true\")",
        "data-outline=render_state.state.is_outline.then_some(\"true\")",
        "data-custom-class=render_state.state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=render_state.agent_contract.source.as_attr()",
        "lang=locale.lang",
        "dir=locale.dir",
        "data-ui-schema=render_state.agent_contract.schema_name",
        "data-ui-schema-version=render_state.agent_contract.schema_version.as_attr()",
        "data-ui-intent=render_state.agent_contract.intent.as_attr()",
        "data-ui-action=render_state.agent_contract.action.as_attr()",
        "data-ui-state=render_state.agent_contract.state.as_attr()",
        "data-ui-source=render_state.agent_contract.source.as_attr()",
        "data-ui-stream-support=render_state.agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=render_state.agent_contract.stream_fallback.as_attr()",
        "data-ui-stream-mode=render_state.agent_contract.stream_mode.as_attr()",
        "data-ui-output-status=render_state.agent_contract.output_status.as_attr()",
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
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;",
        "background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
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
fn badge_docs_api_names_and_defaults_align_with_logic_contract() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let logic_source = load_badge_component_source("src/logic.rs");
    let view_source = load_badge_component_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] variant: Option<BadgeVariant>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "pub fn resolve_variant(variant: Option<BadgeVariant>) -> BadgeVariant {",
        "variant.unwrap_or_default()",
        "if variant != BadgeVariant::Default {",
        "lines.push(format!(\"  variant=BadgeVariant::{variant:?}\"));",
        "lines.push(\"  class_name=\\\"docs-badge-custom\\\".into()\".to_string());",
        "lines.push(format!(\"  lang=\\\"{lang}\\\".into()\"));",
        "lines.push(\"  dir=A11yDirection::Rtl\".to_string());",
        "variant.as_attr()",
        "variant.fill_attr()",
    ] {
        let present = docs_source.contains(needle)
            || logic_source.contains(needle)
            || view_source.contains(needle);
        assert!(
            present,
            "badge docs/api/default contract should include `{needle}`.",
        );
    }
}

#[test]
fn badge_readme_keeps_beginner_first_doc_path() {
    let source = load_badge_component_source("src/README.md");

    for needle in [
        "# Badge",
        "## 先用起来（Quick Start）",
        "零门槛最小示例（Hello World）：",
        "<Badge>\"New\"</Badge>",
        "## 常见用法",
        "BadgeVariant::Default",
        "BadgeVariant::Outline",
        "## 进阶（Workbench）",
        "Badge Workbench (Display + Config + Code + CSS Test)",
        "## Source-first（Copy-Paste Ready）",
        "use leptos::prelude::*;",
        "use ui_components::*;",
        "components/badge/src/view.rs",
        "components/badge/src/logic.rs",
        "components/badge/src/styles.rs",
        "## Architecture Layers",
        "apps/docs-app/src/pages/components/pages/display.rs` -> `badge()`",
    ] {
        assert!(
            source.contains(needle),
            "badge README should include `{needle}` for beginner-friendly docs.",
        );
    }

    let quick_start_index = source.find("## 先用起来（Quick Start）");
    let architecture_index = source.find("## Architecture Layers");
    assert!(
        matches!((quick_start_index, architecture_index), (Some(quick), Some(architecture)) if quick < architecture),
        "badge README should present quick-start usage before architecture internals.",
    );
}

#[test]
fn badge_logic_resolves_agent_contract_and_locale_helpers() {
    let source = load_badge_component_source("src/logic.rs");
    let view_source = load_badge_component_source("src/view.rs");

    for needle in [
        "pub const BADGE_AGENT_SCHEMA_NAME: &str = \"ui.badge.agent-contract\";",
        "pub enum BadgeAgentSchemaVersion",
        "pub enum BadgeAgentIntent",
        "pub enum BadgeAgentAction",
        "pub enum BadgeAgentStateAxis",
        "pub enum BadgeAgentSource",
        "pub struct BadgeAgentContract",
        "pub fn resolve_agent_contract(state: BadgeState) -> BadgeAgentContract",
        "schema_name: BADGE_AGENT_SCHEMA_NAME",
        "stream_support: BadgeAgentStreamSupport::Unsupported",
        "stream_fallback: BadgeAgentStreamFallback::Snapshot",
        "stream_mode: BadgeAgentStreamMode::Snapshot",
        "output_status: BadgeAgentOutputStatus::Verified",
    ] {
        assert!(
            source.contains(needle),
            "badge logic should expose `{needle}` machine-readable contract marker."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "let render_state = logic::resolve_render_state(variant, class_name);",
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
        "gotoBadgeDocsAndWaitSettled",
        "ensureWorkbenchControlsVisible",
        "docs-app badge workbench is interactive and updates preview semantics",
        "body:not(:has(#boot))",
        "data-component=\"badge\"",
        "data-slot=\"badge\"",
        "data-slot=\"playground\"",
        "data-slot=\"badge-workbench-controls\"",
        "data-slot=\"badge-workbench-compare\"",
        "data-slot=\"segmented-control\"",
        "data-slot=\"switch\"",
        "data-slot=\"segmented-control-option\"",
        "data-index=\"3\"",
        "data-ui-schema",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-class-source",
        "button[data-slot=\"button\"]:not([data-icon-only=\"true\"])",
        "use leptos::prelude::*;",
        "use ui_components::*;",
    ] {
        assert!(
            source.contains(needle),
            "badge e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn badge_heroui_strategy_and_docs_entry_stay_in_sync() {
    let strategy = load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_pages = load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### Badge 同步记录（2026-02-20）",
        "参数模型同步：`Badge` 参数继续收敛为 `variant/class_name/lang/dir`",
        "component_doc!(\"Badge\", \"badge\", \"Display\", display::badge)",
        "#/components/badge",
        "Badge Workbench (Display + Config + Code + CSS Test)",
    ] {
        let present = strategy.contains(needle) || docs_pages.contains(needle);
        assert!(
            present,
            "badge HeroUI strategy/docs contract should include `{needle}`.",
        );
    }
}
