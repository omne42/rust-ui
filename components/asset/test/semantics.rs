const ASSET_MOD_SOURCE: &str = include_str!("../src/mod.rs");
const ASSET_LOGIC_SOURCE: &str = include_str!("../src/logic.rs");
const ASSET_VIEW_SOURCE: &str = include_str!("../src/view.rs");
const ASSET_MOTION_SOURCE: &str = include_str!("../src/motion.rs");
const ASSET_PROTOCOL_SOURCE: &str = include_str!("../src/protocol.rs");
const ASSET_STYLES_SOURCE: &str = include_str!("../src/styles.rs");
const ASSET_COMPONENT_MANIFEST_SOURCE: &str = include_str!("../src/Component.toml");
const ASSET_RBI_SOURCE: &str = include_str!("../src/asset.rbi");
const ASSET_README_SOURCE: &str = include_str!("../README.md");
const ASSET_CARGO_SOURCE: &str = include_str!("../Cargo.toml");
const ASSET_PROTOCOL_TEST_SOURCE: &str = include_str!("../test/protocol.rs");
const THUMBNAIL_MOTION_SOURCE: &str = include_str!("../../thumbnail/src/motion.rs");
const ASSET_PRIMITIVES_SOURCE: &str =
    include_str!("../../../crates/ui-state-primitives/src/asset.rs");
const UI_COMPONENTS_LIB_SOURCE: &str = include_str!("../../../crates/ui-components/src/lib.rs");
const UI_COMPONENTS_CSS_SOURCE: &str = include_str!("../../../crates/ui-components/src/css.rs");
const UI_COMPONENTS_CARGO_SOURCE: &str = include_str!("../../../crates/ui-components/Cargo.toml");
const UI_COMPONENTS_ROOT_SOURCE: &str = include_str!("../../../crates/ui-components/src/root.rs");
const UI_VISUAL_PRIMITIVE_ACTIVE_HIGHLIGHT_SOURCE: &str =
    include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
const UI_HEADLESS_LIB_SOURCE: &str = include_str!("../../../crates/ui-headless/src/lib.rs");
const UI_MOTION_LIB_SOURCE: &str = include_str!("../../../crates/ui-motion/src/lib.rs");
const PLATFORM_CHECK_SCRIPT_SOURCE: &str =
    include_str!("../../../scripts/check-ui-components-platforms.sh");
const PERFORMANCE_CHECK_SCRIPT_SOURCE: &str =
    include_str!("../../../scripts/check-ui-components-performance.sh");
const WEB_DEMO_CARGO_SOURCE: &str = include_str!("../../../apps/web-demo/Cargo.toml");
const DOCS_PAGE_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages/display_extra_asset.rs");
const DOCS_PLAYGROUND_SOURCE: &str = include_str!("../../../apps/docs-app/src/playground.rs");
const DOCS_COMPONENTS_PAGES_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
const DOCS_COMPONENT_SHELL_SOURCE: &str =
    include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
const DOCS_PERF_PROBE_SOURCE: &str = include_str!("../../../apps/docs-app/src/perf_probe.rs");
const ASSET_E2E_SPEC_SOURCE: &str =
    include_str!("../../../e2e/tests/docs_app_asset_contract.spec.mjs");
const HEROUI_PARAMETER_STRATEGY_SOURCE: &str =
    include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
const TODO_PLAN_SOURCE: &str = include_str!("../../../docs/plan/TODO.md");
const DEV_DOCS_APP_SCRIPT_SOURCE: &str = include_str!("../../../scripts/dev-docs-app.sh");

#[test]
fn asset_does_not_expose_view_or_logic_modules() {
    for needle in ["pub mod view", "pub mod logic"] {
        assert!(
            !ASSET_MOD_SOURCE.contains(needle),
            "Asset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn asset_keeps_spec_rs_out_for_simple_component_scope() {
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !ASSET_STYLES_SOURCE.contains(forbidden),
            "Asset should not introduce spec module token `{forbidden}` for a simple component."
        );
    }
}

#[test]
fn asset_hyper_structure_builder_is_not_required_for_simple_component() {
    for forbidden in [
        "AssetSpec",
        "AssetSpec::new(",
        ".render()",
        "pub struct AssetSpec",
        "impl AssetSpec",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !ASSET_STYLES_SOURCE.contains(forbidden),
            "Asset is a simple component and should not expose Hyper-Structure builder token `{forbidden}`."
        );
    }
}

#[test]
fn asset_context_compression_manifest_and_rbi_projection_stay_current() {
    for required in [
        "schema_version = \"1\"",
        "name = \"Asset\"",
        "crate = \"ui-asset\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"schema_protocol_versioning\"",
        "name = \"variant\"",
        "name = \"size\"",
        "name = \"is_selected\"",
        "name = \"is_focused\"",
        "name = \"motion\"",
        "name = \"children\"",
    ] {
        assert!(
            ASSET_COMPONENT_MANIFEST_SOURCE.contains(required),
            "asset Component.toml should keep manifest marker `{required}`."
        );
    }

    for required in [
        "pub type AssetVariant = ui_state_primitives::asset::AssetVariant;",
        "pub type AssetSize = ui_state_primitives::thumbnail::ThumbnailSize;",
        "pub type AssetMotion = ui_thumbnail::ThumbnailMotion;",
        "pub fn Asset(",
        "variant: Option<AssetVariant>",
        "size: Option<AssetSize>",
        "is_selected: Option<bool>",
        "is_focused: Option<bool>",
        "motion: Option<AssetMotion>",
        "children: Option<leptos::children::Children>",
    ] {
        assert!(
            ASSET_RBI_SOURCE.contains(required),
            "asset RBI projection should keep API marker `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs", "AssetSpec::new(", ".render()"] {
        assert!(
            !ASSET_COMPONENT_MANIFEST_SOURCE.contains(forbidden)
                && !ASSET_RBI_SOURCE.contains(forbidden),
            "asset context-compression artifacts should not drift into forbidden marker `{forbidden}`."
        );
    }
}

#[test]
fn asset_version_deprecation_migration_is_not_applicable_without_major_breaking_upgrade() {
    assert!(
        ASSET_CARGO_SOURCE.contains("version = \"0.0.0\""),
        "asset crate version should stay pre-1.0 while API surface is still converging."
    );

    for required in [
        "pub enum AssetComponentSchemaVersion",
        "V1,",
        "pub struct AssetComponentSpec",
        "pub schema_version: AssetComponentSchemaVersion,",
        "name = \"schema_protocol_versioning\"",
    ] {
        assert!(
            ASSET_PROTOCOL_SOURCE.contains(required)
                || ASSET_COMPONENT_MANIFEST_SOURCE.contains(required),
            "asset should preserve schema versioning marker `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "deprecated_since",
        "remove_in",
        "codemod_rule",
        "schema_registry_entry",
    ] {
        assert!(
            !ASSET_PROTOCOL_SOURCE.contains(forbidden)
                && !ASSET_COMPONENT_MANIFEST_SOURCE.contains(forbidden)
                && !ASSET_RBI_SOURCE.contains(forbidden),
            "asset should not expose migration-registry token `{forbidden}` without a major breaking upgrade."
        );
    }
}

#[test]
fn asset_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    for required in [
        "pub const ASSET_AGENT_SCHEMA: &str = \"ui.asset.v1\";",
        "pub enum AssetAgentIntent",
        "pub enum AssetAgentAction",
        "pub enum AssetInteractionSource",
        "pub enum AssetMotionSource",
        "pub enum AssetStreamSupport",
        "pub enum AssetStreamFallback",
        "pub enum AssetOutputStatus",
        "pub const fn as_attr(self) -> &'static str",
        "pub const fn from_is_custom(is_custom: bool) -> Self",
    ] {
        assert!(
            ASSET_PROTOCOL_SOURCE.contains(required),
            "asset protocol schema should keep typed marker `{required}`."
        );
    }

    for required in [
        "data-ui-schema=protocol::ASSET_AGENT_SCHEMA",
        "data-ui-intent=protocol::AssetAgentIntent::Display.as_attr()",
        "data-ui-action=protocol::AssetAgentAction::StaticRender.as_attr()",
        "data-ui-state=state.data_state_attr",
        "data-ui-selection-source=protocol::AssetInteractionSource::ExternalProp.as_attr()",
        "data-ui-focus-source=protocol::AssetInteractionSource::ExternalProp.as_attr()",
        "data-ui-label-source=state.label_source_attr",
        "data-ui-class-source=state.class_source_attr",
        "data-ui-content-source=state.content_source_attr",
        "data-ui-motion-source=motion_source_attr",
        "data-ui-stream-support=protocol::AssetStreamSupport::Optional.as_attr()",
        "data-ui-stream-fallback=protocol::AssetStreamFallback::Snapshot.as_attr()",
        "data-ui-output-status=protocol::AssetOutputStatus::Verified.as_attr()",
        "protocol::AssetMotionSource::from_is_custom(is_custom_motion).as_attr()",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset view should keep agent-contract schema marker `{required}`."
        );
    }

    for required in [
        "name = \"agent-contract-schema\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"streaming_optional_snapshot_fallback\"",
        "data-ui-schema + data-ui-intent + data-ui-action",
    ] {
        assert!(
            ASSET_COMPONENT_MANIFEST_SOURCE.contains(required),
            "asset manifest should keep agent-contract declaration `{required}`."
        );
    }

    for required in [
        "pub const ASSET_AGENT_SCHEMA: &'static str;",
        "pub enum AssetAgentIntent",
        "pub enum AssetAgentAction",
        "pub enum AssetInteractionSource",
        "pub enum AssetMotionSource",
        "pub enum AssetStreamSupport",
        "pub enum AssetStreamFallback",
        "pub enum AssetOutputStatus",
    ] {
        assert!(
            ASSET_RBI_SOURCE.contains(required),
            "asset rbi should project schema token `{required}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "<script",
        "javascript:",
        "onload=",
        "onclick=",
        "eval(",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_PROTOCOL_SOURCE.contains(forbidden)
                && !ASSET_COMPONENT_MANIFEST_SOURCE.contains(forbidden)
                && !ASSET_RBI_SOURCE.contains(forbidden),
            "asset render/config pipeline should stay on whitelist boundary; forbidden token `{forbidden}` found."
        );
    }
}

#[test]
fn asset_llm_streaming_snapshot_modes_are_not_applicable() {
    for forbidden in [
        "data-llm-output-mode",
        "data-streaming-state",
        "data-streaming-chunk",
        "data-snapshot-state",
        "on_chunk",
        "on_token",
        "flush_partial",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_PROTOCOL_SOURCE.contains(forbidden)
                && !ASSET_COMPONENT_MANIFEST_SOURCE.contains(forbidden)
                && !ASSET_RBI_SOURCE.contains(forbidden),
            "asset is not an LLM text-rendering surface; streaming/snapshot token `{forbidden}` should not appear."
        );
    }
}

#[test]
fn asset_snapshot_baseline_renders_complete_configuration_stably() {
    for required in [
        "pub fn Asset(",
        "#[prop(optional)] variant: AssetVariant",
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "#[prop(optional)] size: AssetSize",
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional)] motion: AssetMotion",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] children: Option<Children>",
        "let resolved = logic::resolve_view_state(logic::AssetResolvedInput {",
        "data-state=state.data_state_attr",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset should keep complete-config stable-render marker `{required}`."
        );
    }

    for forbidden in ["on_chunk", "on_token", "flush_partial", "incremental_patch"] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden) && !ASSET_LOGIC_SOURCE.contains(forbidden),
            "asset snapshot baseline should avoid incremental rendering token `{forbidden}`."
        );
    }
}

#[test]
fn asset_streaming_policy_is_optional_with_snapshot_fallback_and_status_marker() {
    for required in [
        "data-ui-stream-support=protocol::AssetStreamSupport::Optional.as_attr()",
        "data-ui-stream-fallback=protocol::AssetStreamFallback::Snapshot.as_attr()",
        "data-ui-output-status=protocol::AssetOutputStatus::Verified.as_attr()",
        "role=\"img\"",
        "aria-label=label.get_value()",
        "data-state=state.data_state_attr",
        "data-ui-state=state.data_state_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset should keep optional-streaming snapshot fallback marker `{required}`."
        );
    }

    for required in [
        "name = \"streaming_optional_snapshot_fallback\"",
        "data-ui-stream-support + data-ui-stream-fallback + data-ui-output-status",
    ] {
        assert!(
            ASSET_COMPONENT_MANIFEST_SOURCE.contains(required),
            "asset manifest should keep streaming policy declaration `{required}`."
        );
    }

    for required in [
        "pub enum AssetStreamSupport",
        "pub enum AssetStreamFallback",
        "pub enum AssetOutputStatus",
    ] {
        assert!(
            ASSET_RBI_SOURCE.contains(required),
            "asset rbi should project streaming policy token `{required}`."
        );
    }

    for forbidden in [
        "on_chunk",
        "on_token",
        "flush_partial",
        "reconnect_stream",
        "retry_stream",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_PROTOCOL_SOURCE.contains(forbidden),
            "asset should not implement upper-layer streaming/retry policy token `{forbidden}`."
        );
    }
}

#[test]
fn asset_rust_hygiene_forbids_unwrap_expect_let_underscore_and_string_clone_hotspots() {
    let rust_sources = [
        ASSET_MOD_SOURCE,
        ASSET_LOGIC_SOURCE,
        ASSET_VIEW_SOURCE,
        ASSET_STYLES_SOURCE,
        ASSET_MOTION_SOURCE,
        ASSET_PROTOCOL_SOURCE,
    ];

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            rust_sources
                .iter()
                .all(|source| !source.contains(forbidden)),
            "asset non-test code should not contain forbidden hygiene token `{forbidden}`."
        );
    }

    for forbidden in [".to_string()", ".to_owned()", "String::from("] {
        assert!(
            rust_sources
                .iter()
                .all(|source| !source.contains(forbidden)),
            "asset non-test code should avoid string clone hotspot `{forbidden}`; use borrowed static attrs or Cow<'static, str> when needed."
        );
    }
}

#[test]
fn asset_is_exported_from_module_and_ui_components_root() {
    assert!(
        ASSET_MOD_SOURCE.contains("pub use view::Asset;"),
        "asset module should export `Asset`."
    );
    assert!(
        UI_COMPONENTS_LIB_SOURCE
            .contains("pub use asset::{Asset, AssetMotion, AssetSize, AssetVariant};"),
        "ui-components crate root should re-export Asset contract."
    );
}

#[test]
fn asset_wraps_thumbnail_contract() {
    for needle in [
        "pub fn Asset(",
        "variant: AssetVariant",
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "logic::resolve_view_state(logic::AssetResolvedInput {",
        "has_children: children.is_some(),",
        "<Thumbnail",
        "lang=lang",
        "dir=dir",
        "data-slot=\"asset\"",
        "data-size=state.size_attr",
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-content-source=state.content_source_attr",
        "data-class-source=state.class_source_attr",
        "data-custom-class=if state.has_custom_class_name {",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=if motion != AssetMotion::default() {",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(needle),
            "Asset wrapper should preserve Thumbnail contract marker `{needle}`."
        );
    }
}

#[test]
fn asset_styles_include_variant_state_and_accessibility_markers() {
    for selector in [
        ".ui-asset--variant-file",
        ".ui-asset[data-variant=\"file\"]",
        ".ui-asset--variant-folder",
        ".ui-asset--variant-custom",
        ".ui-asset--selected",
        ".ui-asset[data-selected=\"true\"]",
        ".ui-asset--focused",
        ".ui-asset[data-focused=\"true\"]",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            ASSET_STYLES_SOURCE.contains(selector),
            "Asset styles should include `{selector}` for baseline-compatible state/accessibility contracts."
        );
    }
}

#[test]
fn asset_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    for required in [
        ".ui-asset--variant-file",
        ".ui-asset[data-variant=\"file\"]",
        ".ui-asset--variant-folder",
        ".ui-asset[data-variant=\"folder\"]",
        ".ui-asset--variant-custom",
        ".ui-asset[data-variant=\"custom\"]",
        ".ui-asset--selected .ui-asset__icon",
        ".ui-asset[data-selected=\"true\"] .ui-asset__icon",
        ".ui-asset--focused .ui-asset__icon",
        ".ui-asset[data-focused=\"true\"] .ui-asset__icon",
        ".ui-asset--size-500",
        ".ui-asset[data-size=\"500\"]",
        ".ui-asset--size-1000",
        ".ui-asset[data-size=\"1000\"]",
    ] {
        assert!(
            ASSET_STYLES_SOURCE.contains(required),
            "Asset styles should map visual states via explicit marker selector `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !ASSET_STYLES_SOURCE.contains(forbidden) && !ASSET_VIEW_SOURCE.contains(forbidden),
            "Asset should not rely on brittle DOM/style protocol token `{forbidden}`."
        );
    }
}

#[test]
fn asset_follows_token_first_static_style_contract() {
    for required in [
        "pub const CSS: &str",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "#[cfg(feature = \"component-asset\")]",
        "out.push_str(crate::asset::styles::CSS);",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ASSET_STYLES_SOURCE.contains(required)
                || UI_COMPONENTS_CSS_SOURCE.contains(required)
                || UI_COMPONENTS_ROOT_SOURCE.contains(required),
            "Asset token-first style contract should include `{required}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "style=",
        "css!(",
        "styled(",
        "styled_components",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_STYLES_SOURCE.contains(forbidden)
                && !ASSET_CARGO_SOURCE.contains(forbidden),
            "Asset should not adopt utility-first or CSS-in-Rust default token `{forbidden}`."
        );
    }
}

#[test]
fn asset_styles_use_defensive_variable_fallback_chains() {
    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-space-xl, var(--ui-fallback-space-xl))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
    ] {
        assert!(
            ASSET_STYLES_SOURCE.contains(required),
            "Asset styles should keep defensive fallback chain `{required}`."
        );
    }

    for forbidden in [
        "color: var(--ui-fg);",
        "var(--ui-bg);",
        "var(--ui-accent);",
        "2.25rem",
        "4.25rem",
        "0.25rem",
        "#fff",
        "#000",
    ] {
        assert!(
            !ASSET_STYLES_SOURCE.contains(forbidden),
            "Asset styles should not keep raw terminal style literal `{forbidden}`."
        );
    }
}

#[test]
fn asset_css_is_aggregated_under_ui_layer_without_plain_inline_styles() {
    assert!(
        UI_COMPONENTS_CSS_SOURCE.contains("out.push_str(\"\\n@layer ui {\\n\");"),
        "ui-components css registry should aggregate component styles under `@layer ui`."
    );
    assert!(
        UI_COMPONENTS_CSS_SOURCE.contains("#[cfg(feature = \"component-asset\")]")
            && UI_COMPONENTS_CSS_SOURCE.contains("out.push_str(crate::asset::styles::CSS);"),
        "asset styles should be feature-gated and injected through centralized `@layer ui` registry."
    );

    for line in ASSET_VIEW_SOURCE.lines() {
        let trimmed = line.trim_start();

        assert!(
            !trimmed.starts_with("style="),
            "asset view should not use plain inline `style=...`; found `{trimmed}`."
        );

        if trimmed.contains("style:") {
            assert!(
                trimmed.contains("style:--"),
                "asset runtime style mutation must use CSS custom properties only; found `{trimmed}`."
            );
        }
    }
}

#[test]
fn asset_tree_shaking_contract_is_feature_gated_for_package_and_style_layers() {
    for required in [
        "component-asset = [\"dep:ui-asset\"]",
        "ui-asset = { path = \"../../components/asset\", optional = true }",
        "#[cfg(feature = \"component-asset\")]\npub use ui_asset as asset;",
        "#[cfg(feature = \"component-asset\")]\n    out.push_str(crate::asset::styles::CSS);",
        "#[cfg(feature = \"inject-css\")]\npub fn push_components_css(out: &mut String) {",
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
    ] {
        assert!(
            UI_COMPONENTS_CARGO_SOURCE.contains(required)
                || UI_COMPONENTS_LIB_SOURCE.contains(required)
                || UI_COMPONENTS_CSS_SOURCE.contains(required)
                || WEB_DEMO_CARGO_SOURCE.contains(required),
            "Tree-shaking contract marker `{required}` should stay feature-gated."
        );
    }

    for forbidden in [
        "ui-components = { path = \"../../crates/ui-components\", features = [\"all-components\"] }",
        "ui-components = { path = \"../../crates/ui-components\", default-features = true",
    ] {
        assert!(
            !WEB_DEMO_CARGO_SOURCE.contains(forbidden),
            "web-demo should not pull full feature surface via `{forbidden}`."
        );
    }
}

#[test]
fn asset_ui_components_fixed_entry_files_are_correct() {
    for required in [
        "#[cfg(feature = \"component-asset\")]\npub use ui_asset as asset;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub fn push_components_css(out: &mut String) {\n    css::push_components_css(out);",
    ] {
        assert!(
            UI_COMPONENTS_LIB_SOURCE.contains(required),
            "ui-components lib.rs should keep fixed entry marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]\npub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-asset\")]",
        "out.push_str(crate::asset::styles::CSS);",
    ] {
        assert!(
            UI_COMPONENTS_CSS_SOURCE.contains(required),
            "ui-components css.rs should keep fixed css entry marker `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            UI_COMPONENTS_ROOT_SOURCE.contains(required),
            "ui-components root.rs should keep unified root entry marker `{required}`."
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_active_highlight_motion(",
    ] {
        assert!(
            UI_VISUAL_PRIMITIVE_ACTIVE_HIGHLIGHT_SOURCE.contains(required),
            "ui-visual-primitive active_highlight.rs should keep shared motion primitive marker `{required}`."
        );
    }

    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/ui-components/src"
        ))
        .join(forbidden);
        assert!(
            !path.exists(),
            "ui-components fixed entry layout should not include forbidden file `{forbidden}`."
        );
    }
}

#[test]
fn asset_type_system_and_semantic_markers_form_machine_readable_contract() {
    for required in [
        "variant: AssetVariant",
        "size: AssetSize",
        "pub variant: AssetVariant",
        "pub size: ThumbnailSize",
        "pub fn resolve_view_state(input: AssetResolvedInput) -> AssetResolvedViewState",
        "resolve_state(AssetStateInput {",
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required)
                || ASSET_LOGIC_SOURCE.contains(required)
                || ASSET_PRIMITIVES_SOURCE.contains(required),
            "Asset machine-readable state contract should include `{required}`."
        );
    }

    for forbidden in [
        "variant: String",
        "size: String",
        "status: String",
        "mode: String",
        "variant: Option<String>",
        "size: Option<String>",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_PRIMITIVES_SOURCE.contains(forbidden),
            "Asset should avoid string protocol drift token `{forbidden}`."
        );
    }

    for closed_set in [
        "AssetVariant::File => \"file\"",
        "AssetVariant::Folder => \"folder\"",
        "AssetVariant::Custom => \"custom\"",
        "\"selected\"",
        "\"focused\"",
        "\"default\"",
        "\"custom-slot\"",
        "\"fallback-icon\"",
        "\"builtin-icon\"",
    ] {
        assert!(
            ASSET_PRIMITIVES_SOURCE.contains(closed_set),
            "Primitive contract should keep enumerable closed-set token `{closed_set}`."
        );
    }
}

#[test]
fn asset_visual_baseline_uses_default_theme_tokens_without_bootstrap_like_regression() {
    for required in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "title=\"Hello World (Default Path)\"",
        "<Asset />",
    ] {
        assert!(
            ASSET_STYLES_SOURCE.contains(required) || DOCS_PAGE_SOURCE.contains(required),
            "Asset visual baseline should keep default-theme marker `{required}`."
        );
    }

    for forbidden in [
        "class=\"btn",
        "btn-primary",
        "form-control",
        "panel-default",
        "navbar-default",
        "bootstrap",
    ] {
        assert!(
            !ASSET_STYLES_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should avoid legacy/rough visual regression token `{forbidden}`."
        );
    }
}

#[test]
fn asset_files_keep_component_layer_responsibilities() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::AssetMotion;",
        "pub use view::Asset;",
    ] {
        assert!(
            mod_source.contains(needle),
            "asset mod.rs should keep boundary export `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "fn ", "struct "] {
        assert!(
            !mod_source.contains(forbidden),
            "asset mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub struct AssetResolvedInput",
        "pub struct AssetResolvedViewState",
        "pub fn resolve_view_state",
        "normalize_optional_text",
        "resolve_label",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "asset logic.rs should keep normalization/derivation contract `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "view!", "<Thumbnail", "data-slot="] {
        assert!(
            !logic_source.contains(forbidden),
            "asset logic.rs should not include DOM/style/view details `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        ".ui-asset",
        "var(--ui-fg",
        "data-variant",
    ] {
        assert!(
            styles_source.contains(needle),
            "asset styles.rs should keep token-first static css contract `{needle}`."
        );
    }

    for forbidden in [
        "role=\"img\"",
        "aria-label",
        "Build Report",
        "Design Assets",
        "Featured Artwork",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "asset styles.rs should not carry behavior or business copy `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_view_state(logic::AssetResolvedInput {",
        "<Thumbnail",
        "lang=lang",
        "dir=dir",
        "data-state=state.data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "asset view.rs should keep structure + semantic mount contract `{needle}`."
        );
    }

    for forbidden in ["logic::resolve_state(", "logic::compose_class_name("] {
        assert!(
            !view_source.contains(forbidden),
            "asset view.rs should not re-implement kernel derivation `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: AssetMotion) -> AssetMotion",
        "ui_thumbnail::motion::sanitize_motion(motion)",
        "pub fn attach_motion(",
        "ui_thumbnail::motion::attach_motion(node_ref, active, sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "asset motion.rs should keep motion mapping/delegation contract `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator",
        "request_animation_frame",
        "set_timeout_with_callback",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "asset motion.rs should not re-implement runtime driver `{forbidden}`."
        );
    }
}

#[test]
fn asset_component_directory_keeps_standard_file_layout() {
    let component_src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src.join(required);
        assert!(
            path.exists(),
            "asset component directory should contain required file `{required}` at `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src.join(forbidden);
        assert!(
            !path.exists(),
            "asset component directory should not introduce `{forbidden}` for this simple component at `{}`.",
            path.display()
        );
    }

    assert!(
        ASSET_MOD_SOURCE.contains("pub use view::Asset;")
            && ASSET_MOD_SOURCE.contains("pub use motion::AssetMotion;"),
        "asset mod.rs should keep minimal stable export surface."
    );

    let mut rs_entries = std::collections::BTreeSet::new();
    for entry in std::fs::read_dir(&component_src).expect("asset src directory should be readable")
    {
        let entry = entry.expect("asset src directory entry should be readable");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            rs_entries.insert(entry.file_name().to_string_lossy().into_owned());
        }
    }

    let expected_entries = std::collections::BTreeSet::from([
        String::from("mod.rs"),
        String::from("logic.rs"),
        String::from("styles.rs"),
        String::from("view.rs"),
        String::from("motion.rs"),
        String::from("protocol.rs"),
    ]);
    assert_eq!(
        rs_entries, expected_entries,
        "asset src file placement should stay disciplined: core files + protocol sidecar only."
    );
}

#[test]
fn asset_semantic_contract_matrix_covers_applicable_branches_without_snapshot_dependency() {
    for required in [
        "role=\"img\"",
        "aria-label=label.get_value()",
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "Asset semantic contract marker `{required}` should be asserted instead of visual snapshot."
        );
    }

    for not_applicable_axis in [
        "is_disabled",
        "disabled=",
        "default_selected",
        "default_focused",
        "on_selected_change",
        "on_focused_change",
        "on:keydown",
        "on:keyup",
        "on:click",
        "on:pointerdown",
        "on:pointermove",
        "tabindex",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(not_applicable_axis)
                && !ASSET_LOGIC_SOURCE.contains(not_applicable_axis)
                && !ASSET_MOTION_SOURCE.contains(not_applicable_axis),
            "Asset should keep `{not_applicable_axis}` out of semantic matrix because this component is non-interactive (N/A axis)."
        );
    }

    for ssr_wasm_split in ["web_sys", "wasm_bindgen", "cfg(feature = \"ssr\")"] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(ssr_wasm_split)
                && !ASSET_LOGIC_SOURCE.contains(ssr_wasm_split)
                && !ASSET_MOTION_SOURCE.contains(ssr_wasm_split),
            "Asset should not add platform split token `{ssr_wasm_split}`; SSR/WASM branch is N/A for this static semantic primitive."
        );
    }

    for snapshot_dep in ["insta", "similar-asserts", "snapshot"] {
        assert!(
            !ASSET_CARGO_SOURCE.contains(snapshot_dep),
            "Asset semantic contract tests should not depend on snapshot-oriented dependency `{snapshot_dep}`."
        );
    }
}

#[test]
fn asset_semantics_tests_prioritize_contract_over_visual_snapshot() {
    for required in [
        "role=\"img\"",
        "aria-label=label.get_value()",
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset semantics must assert machine-readable contract marker `{required}`."
        );
    }

    for required_test in [
        "fn asset_semantic_contract_matrix_covers_applicable_branches_without_snapshot_dependency()",
        "fn asset_agent_contract_schema_is_typed_traceable_and_whitelisted()",
        "fn asset_snapshot_baseline_renders_complete_configuration_stably()",
    ] {
        assert!(
            include_str!("../test/semantics.rs").contains(required_test),
            "asset semantics suite should keep contract-focused regression `{required_test}`."
        );
    }

    for snapshot_dep in ["insta", "similar-asserts"] {
        assert!(
            !ASSET_CARGO_SOURCE.contains(snapshot_dep),
            "asset semantics should not regress into snapshot-first dependency `{snapshot_dep}`."
        );
    }
}

#[test]
fn asset_logic_tracks_label_content_and_class_sources() {
    for needle in [
        "pub use ui_state_primitives::asset::{",
        "AssetStateInput",
        "AssetState",
        "AssetResolvedInput",
        "AssetResolvedViewState",
        "DEFAULT_FILE_LABEL",
        "DEFAULT_FOLDER_LABEL",
        "DEFAULT_CUSTOM_LABEL",
        "resolve_label",
        "resolve_state",
        "resolve_view_state",
        "compose_class_name",
        "normalize_optional_text",
    ] {
        assert!(
            ASSET_LOGIC_SOURCE.contains(needle),
            "Asset logic should include `{needle}` to consume state primitives instead of re-implementing state kernels."
        );
    }
}

#[test]
fn asset_docs_page_exists() {
    for needle in [
        "pub(super) fn asset() -> AnyView",
        "title=\"Asset\"",
        "slug=\"asset\"",
        "<Asset",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(needle),
            "display_extra_asset docs page should contain `{needle}`."
        );
    }
}

#[test]
fn asset_heroui_strategy_doc_and_component_docs_entry_stay_synced() {
    for required in [
        "### Asset 同步记录（2026-02-20）",
        "`Asset` 保持 display primitive 定位",
        "component_doc!(\"Asset\", \"asset\", \"Display\", display_extra_asset::asset)",
        "components/asset/README.md",
        "apps/docs-app/src/pages/components/pages/display_extra_asset.rs",
        "data-slot=\"asset-source-first\"",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            HEROUI_PARAMETER_STRATEGY_SOURCE.contains(required),
            "HeroUI strategy doc should keep asset sync marker `{required}`."
        );
    }

    for required in [
        "mod display_extra_asset;",
        "component_doc!(\"Asset\", \"asset\", \"Display\", display_extra_asset::asset),",
    ] {
        assert!(
            DOCS_COMPONENTS_PAGES_SOURCE.contains(required),
            "docs catalog should keep asset index marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn asset() -> AnyView",
        "title=\"Asset\"",
        "slug=\"asset\"",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs page should keep accessible marker `{required}`."
        );
    }
}

#[test]
fn asset_documentation_as_product_readme_is_beginner_friendly() {
    for required in [
        "# Asset",
        "## Documentation Entry",
        "apps/docs-app/src/pages/components/pages/display_extra_asset.rs",
        "## Hello World",
        "view! { <Asset /> }",
        "No `ui-state-primitives` or `ui-headless` wiring is required.",
        "## Common Usage",
        "AssetVariant::File",
        "AssetVariant::Folder",
        "## Start Simple, Then Go Advanced",
        "Default path first (`<Asset />`), then opt into advanced controls",
        "## Advanced Controls",
    ] {
        assert!(
            ASSET_README_SOURCE.contains(required),
            "asset README should keep beginner-friendly docs marker `{required}`."
        );
    }

    let hello_pos = ASSET_README_SOURCE
        .find("## Hello World")
        .expect("asset README should include Hello World section");
    let common_pos = ASSET_README_SOURCE
        .find("## Common Usage")
        .expect("asset README should include Common Usage section");
    let advanced_pos = ASSET_README_SOURCE
        .find("## Advanced Controls")
        .expect("asset README should include Advanced Controls section");
    assert!(
        hello_pos < common_pos && common_pos < advanced_pos,
        "asset README should document default path first, then common usage, then advanced controls."
    );
}

#[test]
fn asset_docs_page_covers_primary_playgrounds() {
    for needle in [
        "pub(super) fn asset() -> AnyView",
        "title=\"Asset\"",
        "slug=\"asset\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"File + Folder Variants\"",
        "title=\"Custom Image + Focused State\"",
        "title=\"State + Source Markers\"",
        "title=\"Controlled vs Uncontrolled (N/A Axis)\"",
        "title=\"Streaming Optional + Snapshot Fallback\"",
        "title=\"Interactive Playground (Props + State + Spec Preview)\"",
        "data-slot=\"asset-source-first\"",
        "data-slot=\"asset-source-first-paths\"",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(needle),
            "display-extra-asset docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn asset_docs_interactive_playground_supports_props_state_and_spec_preview() {
    for required in [
        "title=\"Interactive Playground (Props + State + Spec Preview)\"",
        "test_config_signal=interactive_spec_preview",
        "data-slot=\"asset-interactive-controls\"",
        "data-slot=\"asset-interactive-preview\"",
        "data-slot=\"asset-interactive-spec-preview\"",
        "set_interactive_variant_key.set(event_target_value(&ev))",
        "set_interactive_size_key.set(event_target_value(&ev))",
        "set_interactive_label.set(event_target_value(&ev))",
        "set_interactive_selected.set(event_target_checked(&ev))",
        "set_interactive_focused.set(event_target_checked(&ev))",
        "set_interactive_use_custom_slot.set(event_target_checked(&ev))",
        "set_interactive_custom_class.set(event_target_checked(&ev))",
        "Repeatable flow: Selected on -> Focused on -> Variant folder -> back to custom.",
        "AssetComponentSpecInput",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs interactive playground should keep marker `{required}`."
        );
    }
}

#[test]
fn asset_docs_playgrounds_lock_state_matrix_contract_values() {
    for needle in [
        "title=\"File + Folder Variants\"",
        "variant=AssetVariant::File",
        "variant=AssetVariant::Folder",
        "size=AssetSize::Size600",
        "label=\"Build Report\".to_string()",
        "label=\"Design Assets\".to_string()",
        "title=\"Custom Image + Focused State\"",
        "size=AssetSize::Size700",
        "is_selected=true",
        "is_focused=true",
        "title=\"State + Source Markers\"",
        "variant=AssetVariant::Custom",
        "size=AssetSize::Size800",
        "label=\"Featured Artwork\".to_string()",
        "lang=\"en\".to_string()",
        "dir=\"ltr\".to_string()",
        "class_name=\"docs-asset-state\".to_string()",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(needle),
            "asset docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn asset_docs_copy_paste_ready_playground_contract_is_complete() {
    for required in [
        "const ASSET_PLAYGROUND_IMPORTS: &str =",
        "use leptos::prelude::*;",
        "use ui_components::{Asset, AssetSize, AssetVariant};",
        "code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()",
        "title=\"Hello World (Default Path)\"",
        "title=\"State + Source Markers\"",
        "title=\"Controlled vs Uncontrolled (N/A Axis)\"",
        "title=\"Streaming Optional + Snapshot Fallback\"",
        "Uncontrolled axis: N/A",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs copy-paste playground contract should keep marker `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(required),
            "docs playground source-first copy pipeline should keep marker `{required}`."
        );
    }
}

#[test]
fn asset_source_first_docs_are_copy_paste_ready_and_point_to_real_source_files() {
    for required in [
        "data-slot=\"asset-source-first\"",
        "Source-first Copy-Paste",
        "<code>\"Show code\"</code>",
        "data-slot=\"asset-source-first-paths\"",
        "components/asset/src/mod.rs",
        "components/asset/src/view.rs",
        "components/asset/src/logic.rs",
        "components/asset/src/styles.rs",
        "components/asset/src/motion.rs",
        "components/asset/src/protocol.rs",
        "crates/ui-components/src/lib.rs",
        "data-slot=\"asset-source-first-prerequisites\"",
        "component-asset",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset source-first docs should keep marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"playground-toggle-code\"",
        "\"Show code\"",
        "<CodeBlock code=resolved_code.get() />",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(required),
            "docs playground copy-ready pipeline should keep marker `{required}`."
        );
    }

    let component_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative in [
        "src/mod.rs",
        "src/view.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/motion.rs",
        "src/protocol.rs",
        "../../crates/ui-components/src/lib.rs",
    ] {
        let path = component_root.join(relative);
        assert!(
            path.exists(),
            "asset source-first docs path should exist: `{}`.",
            path.display()
        );
    }
}

#[test]
fn asset_docs_examples_parameter_matrix_and_state_matrix_stay_synced_with_logic() {
    for required in [
        "data-slot=\"asset-state-matrix\"",
        "data-slot=\"asset-state-rows\"",
        "\"variant axis\"",
        "\"size axis\"",
        "\"data-state\"",
        "\"control mode\"",
        "\"disabled axis\"",
        "data-slot=\"asset-parameter-matrix\"",
        "data-slot=\"asset-parameter-rows\"",
        "\"variant: AssetVariant\"",
        "\"size: AssetSize\"",
        "\"is_selected / is_focused: bool\"",
        "\"label: Option&lt;String&gt;\"",
        "\"class_name / lang / dir: Option&lt;String&gt;\"",
        "\"motion: AssetMotion\"",
        "AssetVariant::Custom",
        "AssetSize::Size500",
        "AssetMotion::default()",
        "resolve_label",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs matrix should keep synced marker `{required}`."
        );
    }

    for required in [
        "DEFAULT_FILE_LABEL",
        "DEFAULT_FOLDER_LABEL",
        "DEFAULT_CUSTOM_LABEL",
        "resolve_label",
        "pub struct AssetResolvedInput",
        "pub fn resolve_view_state",
    ] {
        assert!(
            ASSET_LOGIC_SOURCE.contains(required),
            "asset logic contract should keep default/source marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] variant: AssetVariant",
        "#[prop(optional)] size: AssetSize",
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "#[prop(optional)] motion: AssetMotion",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset API surface marker `{required}` should stay in sync with docs matrices."
        );
    }
}

#[test]
fn asset_interactive_e2e_contract_is_semantic_and_repeatable() {
    for required in [
        "test(\"docs-app asset interactive playground updates props/state with semantic contracts\"",
        "[data-slot=\"asset-interactive-controls\"]",
        "[data-slot=\"asset-interactive-preview\"] [data-slot=\"asset\"]",
        "getByLabel(\"Variant\")",
        "getByLabel(\"Selected\")",
        "getByLabel(\"Focused\")",
        "getByLabel(\"Use custom slot\")",
        "toHaveAttribute(\"data-variant\", \"folder\")",
        "toHaveAttribute(\"data-content-source\", /(builtin-icon|fallback-icon)/)",
        "toHaveAttribute(\"data-content-source\", \"custom-slot\")",
    ] {
        assert!(
            ASSET_E2E_SPEC_SOURCE.contains(required),
            "asset interactive e2e contract should keep marker `{required}`."
        );
    }
}

#[test]
fn asset_source_first_e2e_contract_is_repeatable_with_semantic_markers() {
    for required in [
        "test(\"docs-app asset source-first docs expose copy-ready code and real source paths\"",
        "[data-slot=\"playground-toggle-code\"]",
        "[data-slot=\"playground-code\"]",
        "[data-slot=\"asset-source-first\"]",
        "[data-slot=\"asset-source-first-paths\"]",
        "components/asset/src/view.rs",
        "crates/ui-components/src/lib.rs",
        "[data-slot=\"asset-source-first-prerequisites\"]",
    ] {
        assert!(
            ASSET_E2E_SPEC_SOURCE.contains(required),
            "asset source-first e2e contract should keep marker `{required}`."
        );
    }
}

#[test]
fn asset_e2e_contract_uses_semantic_selectors_and_state_based_waits() {
    for required in [
        "await page.goto(\"/#/components/asset\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"asset\"]",
        "[data-slot=\"asset\"]",
        "data-ui-schema=\"ui.asset.v1\"",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
        "toHaveAttribute(\"data-selection-source\", \"external-prop\")",
        "toHaveAttribute(\"data-focus-source\", \"external-prop\")",
        "toHaveAttribute(\"role\", \"img\")",
        "toHaveAttribute(\"aria-label\", /.+/)",
        "await page.reload();",
    ] {
        assert!(
            ASSET_E2E_SPEC_SOURCE.contains(required),
            "asset e2e contract should keep semantic selector/wait marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        ":nth-child(",
        "xpath=",
    ] {
        assert!(
            !ASSET_E2E_SPEC_SOURCE.contains(forbidden),
            "asset e2e contract should avoid brittle selector/timer token `{forbidden}`."
        );
    }
}

#[test]
fn asset_repeatable_e2e_key_flow_is_registered_with_semantic_breakpoints() {
    for required in [
        "test(\"docs-app asset flow is repeatable via semantic ready/settled checkpoints\"",
        "const controlledAsset = docsRoot",
        "toHaveAttribute(\"data-state\", /(selected|focused)/);",
        "toHaveAttribute(\"data-ui-state\", /(selected|focused)/);",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\");",
        "await page.reload();",
        "const reloadedControlledAsset = page",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\");",
    ] {
        assert!(
            ASSET_E2E_SPEC_SOURCE.contains(required),
            "asset e2e key-flow suite should keep semantic breakpoint `{required}`."
        );
    }

    for forbidden in [
        "toHaveScreenshot(",
        "page.screenshot(",
        "snapshot.png",
        "page.content()",
    ] {
        assert!(
            !ASSET_E2E_SPEC_SOURCE.contains(forbidden),
            "asset e2e key-flow assertions should stay semantic and avoid opaque page-diff token `{forbidden}`."
        );
    }

    // Asset is display-only and has no overlay/focus-manager/async state machine path.
    for forbidden in [
        "Overlay",
        "focus_manager",
        "on:keydown",
        ".await",
        "spawn_local",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden) && !ASSET_LOGIC_SOURCE.contains(forbidden),
            "asset should not expose high-risk interaction axis token `{forbidden}`."
        );
    }
}

#[test]
fn asset_exposes_a11y_and_i18n_entrypoints_without_hardcoded_business_copy() {
    for required in [
        "role=\"img\"",
        "aria-label=label.get_value()",
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "lang=lang",
        "dir=dir",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "Asset should expose a11y/i18n entrypoint `{required}`."
        );
    }

    for forbidden in ["Build Report", "Design Assets", "Featured Artwork"] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden),
            "Asset view should not hardcode business-facing copy `{forbidden}`."
        );
    }
}

#[test]
fn asset_exposes_stable_observable_state_and_source_markers() {
    for required in [
        "data-slot=\"asset\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
        "data-custom-class=if state.has_custom_class_name {",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=if motion != AssetMotion::default() {",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "Asset should expose stable observable marker `{required}`."
        );
    }

    for closed_set in [
        "AssetVariant::File => \"file\"",
        "AssetVariant::Folder => \"folder\"",
        "AssetVariant::Custom => \"custom\"",
        "data_state_attr = if input.selected {",
        "} else if input.focused {",
        "\"selected\"",
        "\"focused\"",
        "\"default\"",
        "label_source_attr = if input.has_custom_label {",
        "class_source_attr = if input.has_custom_class_name {",
        "(AssetVariant::Custom, true) => \"custom-slot\"",
        "(AssetVariant::Custom, false) => \"fallback-icon\"",
        "_ => \"builtin-icon\"",
    ] {
        assert!(
            ASSET_PRIMITIVES_SOURCE.contains(closed_set),
            "Asset marker values should stay enumerable and closed via `{closed_set}`."
        );
    }
}

#[test]
fn asset_does_not_expose_controlled_uncontrolled_triplet_api() {
    for required in [
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "Asset should expose visual state input `{required}`."
        );
    }

    for forbidden in [
        "default_selected",
        "default_focused",
        "on_selected_change",
        "on_focused_change",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden),
            "Asset is stateless and must not expose controlled/uncontrolled triplet token `{forbidden}`."
        );
    }
}

#[test]
fn asset_view_keeps_state_kernel_derivation_in_logic() {
    for required in [
        "logic::resolve_view_state(logic::AssetResolvedInput {",
        "let state = resolved.state;",
        "let class_name = resolved.class_name;",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "Asset view should consume normalized state via `{required}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(",
        "logic::resolve_label(",
        "logic::normalize_optional_text(",
        "logic::compose_class_name(",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden),
            "Asset view should not re-derive state kernel in view layer: `{forbidden}`."
        );
    }
}

#[test]
fn asset_uses_typed_discrete_axes_instead_of_string_or_bool_protocols() {
    for required in [
        "variant: AssetVariant",
        "size: AssetSize",
        "pub variant: AssetVariant",
        "pub size: ThumbnailSize",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required) || ASSET_LOGIC_SOURCE.contains(required),
            "Asset should keep discrete axes typed via `{required}`."
        );
    }

    for forbidden in [
        "variant: String",
        "size: String",
        "variant: Option<String>",
        "size: Option<String>",
        "status: String",
        "mode: String",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden) && !ASSET_LOGIC_SOURCE.contains(forbidden),
            "Asset should not encode discrete axes as string protocols: `{forbidden}`."
        );
    }
}

#[test]
fn asset_consumes_state_primitives_without_business_store_binding() {
    for required in [
        "pub use ui_state_primitives::asset::{",
        "resolve_state",
        "AssetStateInput",
        "logic::resolve_view_state(logic::AssetResolvedInput {",
    ] {
        assert!(
            ASSET_LOGIC_SOURCE.contains(required) || ASSET_VIEW_SOURCE.contains(required),
            "Asset should consume state primitives contract via `{required}`."
        );
    }

    for forbidden in [
        "redux",
        "zustand",
        "mobx",
        "pinia",
        "Store<",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_CARGO_SOURCE.contains(forbidden),
            "Asset should not bind business store/state-machine protocol token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_async_loading_protocol_surface() {
    for forbidden in [
        "is_loading",
        "loading:",
        "loading=",
        "aria-busy",
        "error",
        "retry",
        "use_async_action",
        "spawn_local",
        ".await",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden) && !ASSET_LOGIC_SOURCE.contains(forbidden),
            "Asset should stay sync-only and must not expose async protocol token `{forbidden}`."
        );
    }

    for forbidden_dep in ["tokio", "async-std", "reqwest", "gloo-net"] {
        assert!(
            !ASSET_CARGO_SOURCE.contains(forbidden_dep),
            "Asset should not add runtime async dependency `{forbidden_dep}`."
        );
    }
}

#[test]
fn asset_dx_default_path_is_copy_paste_ready() {
    for required in [
        "title=\"Hello World (Default Path)\"",
        "code_signal=hello_code",
        "<Asset />",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "Asset docs should keep default-path DX marker `{required}`."
        );
    }

    for forbidden in [
        "<Asset state=",
        "ui_state_primitives",
        "ui_headless",
        "use_async_action",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden) && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset default DX path must not require low-level wiring token `{forbidden}`."
        );
    }
}

#[test]
fn asset_does_not_drift_into_collection_parallel_array_api() {
    for forbidden in [
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "items: Vec",
        "ItemSpec",
        "title_slots",
        "panel_slots",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset is a single-entity primitive; collection-style API token `{forbidden}` is not allowed."
        );
    }
}

#[test]
fn asset_has_no_dragging_macro_micro_state_machine_surface() {
    for forbidden in [
        "Dragging",
        "DragEnd",
        "on:pointermove",
        "on:mousemove",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "Asset should not expose drag loop macro/micro state machine token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_two_pass_measurement_rectification_pipeline() {
    for forbidden in [
        "Intent -> Measure(view) -> Rectification(logic)",
        "Action::Measure",
        "Action::Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "Asset should not expose two-pass geometry pipeline token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_collection_registration_protocol_surface() {
    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose collection registration protocol token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_slot_projection_policy_surface() {
    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose slot projection policy token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_environment_stream_subscription_surface() {
    for forbidden in [
        "IntersectionObserver",
        "BreakpointChanged",
        "ThemeChanged",
        "add_event_listener",
        "match_media",
        "debounce",
        "throttle",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose environment stream token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_event_light_cone_batch_protocol_surface() {
    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose event light cone batch protocol token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_unified_causality_bus_trace_surface() {
    for forbidden in [
        "TraceId",
        "trace_id",
        "Causality Bus",
        "publish",
        "subscriber",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose unified causality bus token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_overlay_focus_stack_or_focus_restore_protocol_surface() {
    for forbidden in [
        "FallbackTo",
        "FocusManager",
        "focus_manager",
        "focus_stack",
        "restore_focus",
        "recover_focus",
        "document.body",
        "Overlay",
        "overlay_stack",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose overlay focus-stack protocol token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_foreign_zone_escape_hatch_surface() {
    for forbidden in [
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "third_party_instance",
        "imperative_instance",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !ASSET_CARGO_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not expose foreign-zone escape hatch token `{forbidden}`."
        );
    }
}

#[test]
fn asset_has_no_hydration_nondeterministic_id_or_time_surface() {
    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "js_sys::Date::now",
        "Uuid::new_v4",
        "crypto.randomUUID",
        "rand::",
        "thread_rng",
        "getrandom",
        "random_uuid",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !ASSET_CARGO_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset should not introduce hydration-nondeterministic source token `{forbidden}`."
        );
    }
}

#[test]
fn asset_cross_platform_contract_uses_feature_cfg_guards_and_keeps_non_wasm_browser_free() {
    for required in [
        "cargo check -p ui-components",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            PLATFORM_CHECK_SCRIPT_SOURCE.contains(required),
            "Platform compile-only script should keep `{required}`."
        );
    }

    assert!(
        UI_HEADLESS_LIB_SOURCE.contains(
            "#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");"
        ),
        "ui-headless should keep explicit feature mutex compile_error! guard."
    );

    for forbidden in [
        "web_sys",
        "web-sys",
        "js_sys",
        "wasm_bindgen",
        "window(",
        "document(",
        "HtmlElement",
        "Navigator",
        "performance",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_STYLES_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !ASSET_CARGO_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "Asset non-wasm sources should stay browser-object free; found `{forbidden}`."
        );
    }
}

#[test]
fn asset_headless_web_ssr_mutex_contract_is_enforced() {
    assert!(
        UI_HEADLESS_LIB_SOURCE.contains(
            "#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");"
        ),
        "ui-headless must keep web/ssr feature mutex compile_error! guard."
    );

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            PLATFORM_CHECK_SCRIPT_SOURCE.contains(required),
            "Platform guard script should keep ui-headless mutex contract token `{required}`."
        );
    }

    assert!(
        !ASSET_CARGO_SOURCE.contains("ui-headless"),
        "Asset should not add direct ui-headless dependency while keeping this component non-interactive."
    );
}

#[test]
fn asset_motion_non_wasm_stub_contract_is_preserved() {
    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn prefers_reduced_motion() -> bool {\n        true\n    }",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "non_wasm_web_backend_is_predictable_noop",
    ] {
        assert!(
            UI_MOTION_LIB_SOURCE.contains(required),
            "ui-motion should keep non-wasm no-op/stub contract marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]\npub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_active: leptos::prelude::Signal<bool>",
        "_motion: ThumbnailMotion",
    ] {
        assert!(
            THUMBNAIL_MOTION_SOURCE.contains(required),
            "thumbnail motion should keep wasm/non-wasm attach split marker `{required}`."
        );
    }

    for required in [
        "pub fn attach_motion(",
        "ui_thumbnail::motion::attach_motion(node_ref, active, sanitize_motion(motion));",
    ] {
        assert!(
            ASSET_MOTION_SOURCE.contains(required),
            "asset motion mapping should keep safe delegation marker `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
    ] {
        assert!(
            PLATFORM_CHECK_SCRIPT_SOURCE.contains(required),
            "platform script should keep ui-motion compile-only path `{required}`."
        );
    }

    for forbidden in ["panic!", "unwrap(", "expect("] {
        assert!(
            !UI_MOTION_LIB_SOURCE.contains(forbidden)
                && !THUMBNAIL_MOTION_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "motion non-wasm downgrade path should avoid hard-failure token `{forbidden}`."
        );
    }
}

#[test]
fn asset_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    for required in [
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "scale.set_target(target_scale);",
        "ring.set_target(target_ring);",
        "#[cfg(target_arch = \"wasm32\")]\npub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_active: leptos::prelude::Signal<bool>",
        "_motion: ThumbnailMotion",
    ] {
        assert!(
            THUMBNAIL_MOTION_SOURCE.contains(required),
            "reduced-motion/wasm/non-wasm branch marker `{required}` should stay in thumbnail motion."
        );
    }

    for required in [
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset semantics marker `{required}` should stay stable across SSR/wasm paths."
        );
    }

    for forbidden in ["cfg(target_arch = \"wasm32\")", "cfg(feature = \"ssr\")"] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden),
            "asset view semantics should not split by platform token `{forbidden}`."
        );
    }

    for required in [
        "asset_has_no_hydration_nondeterministic_id_or_time_surface",
        "asset_motion_non_wasm_stub_contract_is_preserved",
    ] {
        assert!(
            ASSET_MOD_SOURCE.contains("mod semantics_tests;")
                && include_str!("../test/semantics.rs").contains(required),
            "asset branch coverage should keep linked regression `{required}`."
        );
    }
}

#[test]
fn asset_motion_contract_is_parameterized_and_attached_with_safe_degrade() {
    for required in [
        "pub struct ThumbnailMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: 260.0,",
        "damping: 19.0,",
        "let stiffness = if spring.stiffness.is_finite() && spring.stiffness > 0.0 {",
        "let damping = if spring.damping.is_finite() && spring.damping > 0.0 {",
        "spring: ui_motion::spring::SpringConfig {",
        "stiffness,",
        "damping,",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(",
        "_motion: ThumbnailMotion",
    ] {
        assert!(
            THUMBNAIL_MOTION_SOURCE.contains(required),
            "thumbnail motion contract should preserve `{required}`."
        );
    }

    for required in [
        "pub type AssetMotion = ui_thumbnail::ThumbnailMotion;",
        "pub fn sanitize_motion(motion: AssetMotion) -> AssetMotion",
        "ui_thumbnail::motion::sanitize_motion(motion)",
        "pub fn attach_motion(",
        "ui_thumbnail::motion::attach_motion(node_ref, active, sanitize_motion(motion));",
    ] {
        assert!(
            ASSET_MOTION_SOURCE.contains(required),
            "asset motion bridge should preserve `{required}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "request_animation_frame",
        "set_timeout",
        "window(",
        "document(",
    ] {
        assert!(
            !ASSET_MOTION_SOURCE.contains(forbidden),
            "asset motion layer should not re-implement runtime driver token `{forbidden}`."
        );
    }
}

#[test]
fn asset_performance_governance_budget_is_equivalently_guarded_and_traceable() {
    for required in [
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            PERFORMANCE_CHECK_SCRIPT_SOURCE.contains(required),
            "performance gate script should keep blocking budget/follow-up path `{required}`."
        );
    }

    for required in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            DOCS_COMPONENT_SHELL_SOURCE.contains(required),
            "docs component shell should keep performance budget wiring marker `{required}`."
        );
    }

    for required in [
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            DOCS_PERF_PROBE_SOURCE.contains(required),
            "perf probe should expose repeatable performance marker `{required}`."
        );
    }

    assert!(
        TODO_PLAN_SOURCE.contains("render_count"),
        "performance governance should keep explicit render_count follow-up tracking in TODO plan."
    );

    for required in [
        "data-state=state.data_state_attr",
        "data-selected=if state.selected { \"true\" } else { \"false\" }",
        "data-focused=if state.focused { \"true\" } else { \"false\" }",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset should keep state/source markers `{required}` for performance attribution."
        );
    }

    for forbidden in [
        "Effect::new",
        "signal::<",
        "create_signal",
        "RwSignal",
        "Memo::new",
        "request_animation_frame",
        "set_interval",
        "set_timeout",
        "add_event_listener",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "asset should avoid local high-frequency runtime loop token `{forbidden}`."
        );
    }
}

#[test]
fn asset_view_macro_complexity_is_split_into_semantic_subrenders() {
    for required in [
        "fn render_file_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_folder_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_custom_fallback_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_custom_content(label: StoredValue<String>, children: Option<Children>) -> AnyView {",
        "fn resolve_icon_content(",
        "let content = resolve_icon_content(variant, label, children);",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset view decomposition should keep macro-splitting marker `{required}`."
        );
    }

    assert!(
        !ASSET_VIEW_SOURCE.contains("let content: AnyView = match variant {"),
        "asset should not keep giant inline match-driven view! block in component body."
    );
}

#[test]
fn asset_prefers_functional_subrender_helpers_over_local_components() {
    for required in [
        "fn render_file_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_folder_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_custom_fallback_icon(label: StoredValue<String>) -> AnyView {",
        "fn render_custom_content(label: StoredValue<String>, children: Option<Children>) -> AnyView {",
        "fn resolve_icon_content(",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset should keep function-first UI helper `{required}`."
        );
    }

    let component_count = ASSET_VIEW_SOURCE.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "asset view should keep a single public component and avoid local component noise."
    );

    for forbidden in [
        "#[component]\nfn render_file_icon",
        "#[component]\nfn render_folder_icon",
        "#[component]\nfn render_custom_fallback_icon",
        "#[component]\nfn render_custom_content",
        "#[component]\nfn resolve_icon_content",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden),
            "asset helper fragment should not be upgraded into local component `{forbidden}`."
        );
    }
}

#[test]
fn asset_static_svg_fragments_are_constantized_or_templated() {
    for required in [
        "const ICON_VIEW_BOX: &str = \"0 0 24 24\";",
        "const ICON_STROKE_WIDTH: &str = \"1.5\";",
        "const ICON_STROKE_LINE_JOIN: &str = \"round\";",
        "const FILE_PATH_BODY: &str = \"M6 2h8l4 4v16H6z\";",
        "const FILE_PATH_FOLD: &str = \"M14 2v4h4\";",
        "const FOLDER_PATH_BODY: &str = \"M3 6.5h6l2 2h10v9A2.5 2.5 0 0 1 18.5 20h-13A2.5 2.5 0 0 1 3 17.5z\";",
        "const FOLDER_PATH_DIVIDER: &str = \"M3 9h18\";",
        "const CUSTOM_FALLBACK_PATH_SCENE: &str =",
        "fn render_two_path_icon(",
        "render_two_path_icon(",
        "FILE_PATH_BODY",
        "FILE_PATH_FOLD",
        "FOLDER_PATH_BODY",
        "FOLDER_PATH_DIVIDER",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset static svg/template marker `{required}` should be centralized in view.rs."
        );
    }

    for required in [
        "role=\"img\"",
        "aria-label=label.get_value()",
        "\"asset-file\"",
        "\"asset-folder\"",
        "data-slot=\"asset-custom\"",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset static template should keep accessibility/slot marker `{required}`."
        );
    }
}

#[test]
fn asset_has_no_inner_html_injection_surface_without_whitelist_contract() {
    for forbidden in [
        "inner_html=",
        ".set_inner_html(",
        "innerHTML",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden)
                && !DOCS_PAGE_SOURCE.contains(forbidden),
            "asset is currently N/A for inner_html and should not introduce unreviewed html injection surface `{forbidden}`."
        );
    }
}

#[test]
fn asset_wasm_debug_contract_is_na_with_equivalent_traceability_and_no_api_pollution() {
    for required in [
        "title=\"State + Source Markers\"",
        "description=\"Inspect `data-state`, `data-label-source`, `data-content-source`, and `data-class-source`",
        "code_signal=state_code",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs should keep wasm-dev visual inspection entry marker `{required}`."
        );
    }

    for required in [
        "data-state=state.data_state_attr",
        "data-selection-source=\"external-prop\"",
        "data-focus-source=\"external-prop\"",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-content-source=state.content_source_attr",
    ] {
        assert!(
            ASSET_VIEW_SOURCE.contains(required),
            "asset should keep observable state/source marker `{required}` for equivalent traceability."
        );
    }

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "Action::Replay",
        "dispatch(",
        "Reducer",
        "Replay",
    ] {
        assert!(
            !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "asset is non-interactive and should not add replay pipeline token `{forbidden}`."
        );
    }

    assert!(
        ASSET_CARGO_SOURCE.contains("[features]\ndefault = []"),
        "asset should keep default-empty feature surface to avoid debug capability leaking into release artifact."
    );

    for forbidden in [
        "wasm-debug",
        "debug-replay",
        "devtools",
        "pub struct AssetDebug",
        "pub enum AssetDebug",
        "pub fn replay_",
        "pub fn trace_",
    ] {
        assert!(
            !ASSET_CARGO_SOURCE.contains(forbidden)
                && !ASSET_MOD_SOURCE.contains(forbidden)
                && !ASSET_VIEW_SOURCE.contains(forbidden)
                && !ASSET_LOGIC_SOURCE.contains(forbidden)
                && !ASSET_MOTION_SOURCE.contains(forbidden),
            "asset should not expose debug protocol/API pollution token `{forbidden}`."
        );
    }
}

#[test]
fn asset_dx_workbench_contract_keeps_hot_style_feedback_context_and_isolated_canvas() {
    for required in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/docs-app\"",
    ] {
        assert!(
            DEV_DOCS_APP_SCRIPT_SOURCE.contains(required),
            "docs dev script should keep fast-feedback local dev entry `{required}`."
        );
    }

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "\"Original CSS is loaded. Use :scope to target this playground only.\"",
    ] {
        assert!(
            DOCS_PLAYGROUND_SOURCE.contains(required),
            "playground workbench should keep DX marker `{required}`."
        );
    }

    for required in [
        "<Playground",
        "title=\"Hello World (Default Path)\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            DOCS_PAGE_SOURCE.contains(required),
            "asset docs should keep isolated playground/workbench entry `{required}`."
        );
    }
}

#[test]
fn asset_engineering_contract_stays_structured_and_runtime_agnostic() {
    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum AssetComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct AssetComponentSpec",
        "#[serde(default)]",
        "pub schema_version: AssetComponentSchemaVersion,",
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<AssetComponentSchemaVersion>();",
        "assert_serde::<AssetComponentSpec>();",
    ] {
        assert!(
            ASSET_PROTOCOL_SOURCE.contains(required)
                || ASSET_PROTOCOL_TEST_SOURCE.contains(required),
            "asset structured protocol marker `{required}` should be preserved."
        );
    }

    for forbidden_dep in ["tokio", "async-std", "tracing = ", "tracing-subscriber"] {
        assert!(
            !ASSET_CARGO_SOURCE.contains(forbidden_dep),
            "asset should not bind component crate to runtime/tracing stack token `{forbidden_dep}`."
        );
    }

    for forbidden_api in [
        "tokio::runtime",
        "async_std::task",
        "Runtime",
        "Handle",
        "use tracing",
        "tracing::",
        "#[instrument",
        "span!(",
        "event!(",
    ] {
        assert!(
            !ASSET_MOD_SOURCE.contains(forbidden_api)
                && !ASSET_LOGIC_SOURCE.contains(forbidden_api)
                && !ASSET_VIEW_SOURCE.contains(forbidden_api)
                && !ASSET_MOTION_SOURCE.contains(forbidden_api)
                && !ASSET_PROTOCOL_SOURCE.contains(forbidden_api),
            "asset should not leak runtime/tracing implementation token `{forbidden_api}` into component API surface."
        );
    }
}
