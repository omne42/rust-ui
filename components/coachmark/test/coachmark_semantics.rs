use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/coachmark/") {
        let path = workspace_dir()
            .join("components/coachmark/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn coachmark_does_not_expose_view_module() {
    let source = load_source("src/coachmark/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Coachmark internals should stay private; found `pub mod view`."
    );

    assert!(
        !source.contains("pub mod logic"),
        "Coachmark `logic` module should stay private to avoid leaking internal state helpers."
    );
}

#[test]
fn coachmark_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/coachmark/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Coachmark;"),
        "coachmark module should export `Coachmark`."
    );
    assert!(
        crate_source.contains("pub use coachmark::{"),
        "crate root should re-export `Coachmark` contracts."
    );
}

#[test]
fn coachmark_wraps_contextual_help_contract() {
    let source = load_source("src/coachmark/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, PopoverPlacement};",
        "pub fn Coachmark(",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "let view_model = logic::resolve_view_model(logic::CoachmarkViewModelInput {",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let state = view_model.state;",
        "<ContextualHelp",
        "disabled=is_disabled",
        "lang=lang.get_value()",
        "dir=dir.get_value()",
        "let class_name = StoredValue::new(view_model.class_name);",
        "primary_cta: Option<String>",
        "asset_variant: Option<CoachmarkAssetVariant>",
        "footer=move || footer_view.get_value().run()",
        "data-slot=\"coachmark-content\"",
        "data-asset=state.asset_attr",
        "data-asset-source=state.asset_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark wrapper should preserve ContextualHelp contract marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_relies_on_contextual_help_headless_a11y_contract() {
    let source = load_source("../../components/contextual-help/src/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, PopoverPlacement};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let panel_a11y = ui_headless::overlay_dialog_attrs(",
        "lang=lang.clone()",
        "dir=dir",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
    ] {
        assert!(
            source.contains(needle),
            "ContextualHelp should expose ui-headless a11y contract marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_logic_reexports_state_primitives_contract() {
    let source = load_source("src/coachmark/logic.rs");

    for needle in [
        "pub use ui_state_primitives::coachmark::{",
        "CoachmarkState",
        "CoachmarkStateInput",
        "DEFAULT_ASSET_LABEL",
        "DEFAULT_TITLE",
        "compose_class_name",
        "compose_heading",
        "compose_step_label",
        "normalize_modifier_keys",
        "normalize_optional_text",
        "resolve_cta_mode",
        "resolve_asset_source",
        "resolve_state",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark logic should re-export `{needle}` from ui-state-primitives."
        );
    }
}

#[test]
fn coachmark_state_primitives_track_heading_steps_and_source_contracts() {
    let source = load_source("../../crates/ui-state-primitives/src/coachmark.rs");

    for needle in [
        "pub const DEFAULT_TITLE: &str = \"Coachmark\";",
        "pub const DEFAULT_ASSET_LABEL: &str = \"Coachmark asset\";",
        "pub enum CoachmarkCtaMode {",
        "pub enum CoachmarkAssetSource {",
        "pub struct CoachmarkStateInput",
        "pub struct CoachmarkState",
        "pub fn compose_heading(",
        "pub fn compose_step_label(",
        "pub fn resolve_cta_mode(",
        "pub fn resolve_asset_source(",
        "pub fn resolve_state(input: CoachmarkStateInput) -> CoachmarkState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: CoachmarkState)",
        "pub cta_mode: CoachmarkCtaMode,",
        "pub asset_source: CoachmarkAssetSource,",
        "variant_attr",
        "placement_attr",
        "asset_source_attr",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark state primitive should include `{needle}` for centralized state/source normalization."
        );
    }
}

#[test]
fn coachmark_styles_include_variant_state_and_accessibility_markers() {
    let source = load_source("src/coachmark/styles.rs");

    for selector in [
        ".ui-coachmark--variant-help",
        ".ui-coachmark[data-variant=\"info\"]",
        ".ui-coachmark--state-disabled",
        ".ui-coachmark[data-state=\"enabled\"]",
        ".ui-coachmark[data-cta=\"none\"] .ui-coachmark__actions",
        ".ui-coachmark__actions-extra",
        ".ui-coachmark[data-motion-source=\"custom\"]",
        ".ui-coachmark[data-custom-motion=\"true\"]",
        "@media (forced-colors: active)",
    ] {
        assert!(
            source.contains(selector),
            "Coachmark styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn coachmark_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    for needle in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
        "<Coachmark",
        "State + Source Markers",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs page should contain `{needle}` for Coachmark."
        );
    }
}

#[test]
fn coachmark_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    for needle in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Step + CTA + Asset Variant\"",
        "title=\"Controlled + Image Asset + Actions\"",
        "title=\"State + Source Markers\"",
        "title=\"Display Comparisons (Help / Info / Disabled)\"",
        "title=\"Config + Code + CSS Test Workbench\"",
    ] {
        assert!(
            source.contains(needle),
            "coachmark docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn coachmark_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>",
        "<Coachmark title=\"Welcome\".to_string() default_open=true>",
        "<div>\"Tour copy\"</div>",
        "<Playground title=\"Step + CTA + Asset Variant\" code_signal=basic_code>",
        "default_open=true",
        "primary_cta=\"Next\".to_string()",
        "asset_variant=CoachmarkAssetVariant::Folder",
        "on_primary=on_primary",
        "<Playground title=\"Controlled + Image Asset + Actions\" code_signal=controlled_code>",
        "<Button variant=ButtonVariant::Secondary on_press=toggle_controlled>",
        "open=controlled_open",
        "asset_src=\"https://picsum.photos/420/260\".to_string()",
        "actions=move || {",
        "is_disabled=true",
        "title=\"State + Source Markers\"",
        "aria_label=\"Coachmark help\".to_string()",
        "class_name=\"docs-coachmark-state\".to_string()",
        "title=\"Display Comparisons (Help / Info / Disabled)\"",
        "title=\"Config + Code + CSS Test Workbench\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || {",
    ] {
        assert!(
            source.contains(needle),
            "coachmark docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn coachmark_docs_hello_world_code_path_stays_under_five_lines_and_no_state_object() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    let hello_world_snippet = r#"<Coachmark title=\"Welcome\".into() default_open=true>
  <div>Tour copy</div>
</Coachmark>"#;

    assert!(
        source.contains(hello_world_snippet),
        "coachmark docs should keep a minimal runnable hello-world code path."
    );

    let line_count = hello_world_snippet.lines().count();
    assert!(
        line_count <= 5,
        "coachmark hello-world snippet must stay within 5 lines, got {line_count}."
    );

    let view_source = load_source("src/coachmark/view.rs");
    assert!(
        !view_source.contains("#[prop(optional)] state:"),
        "coachmark public API must not require an internal `state` object."
    );
}

#[test]
fn coachmark_readme_includes_display_config_code_css_test_sections() {
    let source = load_source("src/coachmark/README.md");

    for needle in [
        "## display（展示区）",
        "## config（配置区）",
        "## code（代码区）",
        "## css test（样式测试区）",
        "## 多场景对比（Comparison Matrix）",
    ] {
        assert!(
            source.contains(needle),
            "coachmark README should include `{needle}`."
        );
    }
}

#[test]
fn coachmark_exposes_agent_contract_and_snapshot_stream_markers() {
    let source = load_source("src/coachmark/view.rs");

    for needle in [
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-state-source=agent_contract.state_source",
        "data-ui-action-source=agent_contract.action_source",
        "data-ui-render-path=agent_contract.render_path",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=agent_contract.output_status.as_str()",
        "data-stream-mode=\"snapshot\"",
        "data-stream-fallback=\"snapshot\"",
        "data-output-status=agent_contract.output_status.as_str()",
    ] {
        assert!(
            source.contains(needle),
            "coachmark view should expose `{needle}` for agent-contract + stream snapshot markers."
        );
    }
}

#[test]
fn coachmark_feature_chain_includes_required_dependencies() {
    let source = load_source("Cargo.toml");

    for needle in [
        "component-contextual_help = [\"component-button\", \"component-popover\"]",
        "component-coachmark = [",
        "\"component-asset\"",
        "\"component-button\"",
        "\"component-contextual_help\"",
    ] {
        assert!(
            source.contains(needle),
            "coachmark feature dependency chain should include `{needle}`."
        );
    }
}

#[test]
fn coachmark_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/coachmark/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "coachmark check2 checklist should not keep unchecked items."
    );
}
