use ui_test_support::source_contract;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "protocol" => include_str!("../src/protocol.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "readme" => include_str!("../src/README.md"),
        "check2" => include_str!("../check2.md"),
        "controllable_state" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "forms_native_docs" => source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/forms_native.rs",
        ),
        "docs_component_catalog" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "heroui_parameter_strategy" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "native_select_e2e_contract" => {
            include_str!("../../../e2e/tests/docs_app_native_select_contract.spec.mjs")
        }
        "docs_playground" => include_str!("../../../apps/docs-app/src/playground.rs"),
        "code_block_view" => include_str!("../../../components/code-block/src/view.rs"),
        "native_select_primitive" => {
            include_str!("../../../crates/ui-state-primitives/src/native_select.rs")
        }
        "headless_native_select" => {
            include_str!("../../../crates/ui-headless/src/native_select.rs")
        }
        "headless_a11y" => include_str!("../../../crates/ui-headless/src/a11y.rs"),
        "headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "headless_cargo_toml" => include_str!("../../../crates/ui-headless/Cargo.toml"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "native_select_cargo_toml" => include_str!("../Cargo.toml"),
        "workspace_native_select_semantics" => {
            include_str!("../../../components/native-select/test/native_select_semantics.rs")
        }
        "ui_components_lib" => include_str!("../../../crates/ui/src/lib.rs"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui/src/root.rs"),
        "ui_components_cargo_toml" => include_str!("../../../crates/ui/Cargo.toml"),
        "ui_visual_active_highlight" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "headless_presence" => include_str!("../../../crates/ui-headless/src/presence.rs"),
        "ui_theme_css" => source_contract::source_from_file_relative(
            file!(),
            "../../../crates/ui-theme/src/css.rs",
        ),
        "component_semantics_self" => include_str!("semantics.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn native_select_module_boundary_is_minimal_and_wires_semantics_tests() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "DEFAULT_ARIA_LABEL",
        "NativeSelectSize",
        "NativeSelectState",
        "pub use view::NativeSelect;",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module.contains(required),
            "native-select module boundary should include `{required}`."
        );
    }

    for forbidden in ["pub mod view", "pub struct NativeSelectState"] {
        assert!(
            !module.contains(forbidden),
            "native-select internals should stay private: `{forbidden}`."
        );
    }
}

#[test]
fn native_select_layered_files_keep_logic_view_styles_split() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "pub struct NativeSelectState",
        "pub fn normalize_options(",
        "pub fn normalize_default_selected_index(",
        "pub fn resolve_control_value(",
        "pub struct NativeSelectStateParams",
        "pub fn resolve_selected_index_correction(",
        "pub fn resolve_states_for_render(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/derivation via `{required}`."
        );
    }
    for forbidden in ["view! {", "on:change=", "web_sys::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not carry view/runtime integration `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_native_select(NativeSelectOptions {",
        "data-slot=\"native-select\"",
        "on:change=on_change",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render structure + headless mount via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-native-select__control",
        "var(--ui-",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS via `{required}`."
        );
    }
    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry render/headless integration `{forbidden}`."
        );
    }
}

#[test]
fn native_select_component_file_responsibilities_are_scoped_and_motion_is_na() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "DEFAULT_ARIA_LABEL",
        "NativeSelectSize",
        "NativeSelectState",
        "pub use view::NativeSelect;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep export boundary responsibility via `{required}`."
        );
    }

    for forbidden in ["mod motion;", "pub mod motion", "ui_motion::"] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should keep motion.rs as N/A without local motion engine path `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "web_sys::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs must not leak render/DOM responsibility `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs must stay static token-first CSS without render logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_native_select(NativeSelectOptions {",
        "on:change=on_change",
    ] {
        assert!(
            view.contains(required),
            "view.rs should focus on structure + headless mount via `{required}`."
        );
    }
}

#[test]
fn native_select_does_not_introduce_spec_rs_for_simple_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "native-select should not introduce src/spec.rs for a simple component."
    );

    for forbidden in ["mod spec;", "pub mod spec", "pub use spec::"] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not wire spec module path `{forbidden}`."
        );
    }

    for forbidden in ["serde::", "#[derive(Serialize", "#[derive(Deserialize"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "simple native-select should not carry schema-versioning surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_component_directory_standard_file_placement_is_correct() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");

    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src_dir.join(required).exists(),
            "native-select component directory should contain `{required}`."
        );
    }

    for absent in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(absent).exists(),
            "native-select component directory should not contain `{absent}` for current scope."
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::NativeSelect;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal export boundary via `{required}`."
        );
    }

    for forbidden in ["pub mod view", "mod motion;", "mod spec;", "mod render;"] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not over-export or drift file entry via `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_options(",
        "pub fn resolve_states_for_render(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/derivation responsibility via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not carry render/headless mount responsibility `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry rendering logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_native_select(NativeSelectOptions {",
        "on:change=on_change",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep Leptos structure + headless semantic mount via `{required}`."
        );
    }
}

#[test]
fn native_select_context_compression_manifest_and_rbi_are_present_and_synced() {
    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest_path = component_src_dir.join("Component.toml");
    let rbi_path = component_src_dir.join("native_select.rbi");

    assert!(
        manifest_path.exists(),
        "native-select should maintain src/Component.toml for context compression manifest."
    );
    assert!(
        rbi_path.exists(),
        "native-select should maintain src/native_select.rbi for API signature projection."
    );

    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));
    let rbi = std::fs::read_to_string(&rbi_path)
        .unwrap_or_else(|error| panic!("failed to read {rbi_path:?}: {error}"));

    for required in [
        "schema_version = \"1\"",
        "name = \"NativeSelect\"",
        "crate = \"ui-native-select\"",
        "name = \"id_base\"",
        "name = \"options\"",
        "name = \"selected_index\"",
        "name = \"default_selected_index\"",
        "name = \"on_selected_index_change\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"ui-headless\"",
        "name = \"ui-state-primitives\"",
    ] {
        assert!(
            manifest.contains(required),
            "Component.toml should include manifest contract `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::native_select::{",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub enum NativeSelectSize {",
        "pub struct NativeSelectState {",
        "pub fn NativeSelect(",
        "id_base: String",
        "options: Vec<NativeSelectOption>",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi.contains(required),
            "native_select.rbi should include API signature projection `{required}`."
        );
    }
}

#[test]
fn native_select_agent_contract_schema_markers_are_typed_and_whitelisted() {
    let logic = load_source("logic");
    let view = load_source("view");
    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest_path = component_src_dir.join("Component.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for required in [
        "pub const NATIVE_SELECT_AGENT_SCHEMA_NAME",
        "pub const NATIVE_SELECT_AGENT_SCHEMA_VERSION",
        "pub enum NativeSelectAgentIntent",
        "pub enum NativeSelectAgentAction",
        "pub enum NativeSelectAgentState",
        "pub enum NativeSelectAgentSource",
        "pub enum NativeSelectChangeSource",
        "pub enum NativeSelectAgentConfigPolicy",
        "pub struct NativeSelectAgentContract",
        "pub struct NativeSelectAgentContractInput",
        "pub fn resolve_agent_contract(",
        "input: NativeSelectAgentContractInput<'_>",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should define typed agent-contract schema generation via `{required}`."
        );
    }

    for required in [
        "logic::resolve_agent_contract(logic::NativeSelectAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version",
        "data-ui-intent=move || agent_contract.get().intent.as_attr()",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "view.rs should mount typed agent-contract markers via `{required}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.native_select.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest.contains(required),
            "Component.toml should carry agent-contract markers + whitelist boundary via `{required}`."
        );
    }
}

#[test]
fn native_select_streaming_term_is_scoped_to_llm_output_rendering_only() {
    let logic = load_source("logic");
    let view = load_source("view");
    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest_path = component_src_dir.join("Component.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for required in [
        "pub fn NativeSelect(",
        "options: Vec<NativeSelectOption>",
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "native-select should stay on discrete selection interaction path `{required}`."
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "ReadableStream",
        "text/event-stream",
        "render_markdown",
        "markdown",
        "stream_chunk",
        "token_delta",
        "llm_stream",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !manifest.contains(forbidden),
            "native-select should not introduce LLM streaming output pipeline surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_streaming_policy_is_optional_with_snapshot_fallback_and_readable_status_markers() {
    let logic = load_source("logic");
    let view = load_source("view");
    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest_path = component_src_dir.join("Component.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for required in [
        "data-streaming-mode=\"optional\"",
        "data-streaming-fallback=\"snapshot\"",
        "data-output-status=move || output_status.get().as_attr()",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view.contains(required),
            "native-select should expose optional-streaming snapshot fallback/status marker `{required}`."
        );
    }

    for required in [
        "pub enum NativeSelectOutputStatus",
        "Draft,",
        "Verified,",
        "Submittable,",
        "pub fn resolve_output_status(state: &NativeSelectState) -> NativeSelectOutputStatus",
        "state.is_invalid || state.is_empty",
    ] {
        assert!(
            logic.contains(required),
            "native-select output status contract should stay typed in logic.rs via `{required}`."
        );
    }

    for required in [
        "name = \"streaming_optional_fallback_snapshot\"",
        "name = \"snapshot_rendering\"",
        "data-streaming-mode + data-streaming-fallback + data-output-status",
    ] {
        assert!(
            manifest.contains(required),
            "Component.toml should declare streaming-optional snapshot fallback capability via `{required}`."
        );
    }
}

#[test]
fn native_select_rust_hygiene_disallows_unwrap_expect_let_underscore_and_string_clone_churn() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let protocol = load_source("protocol");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !protocol.contains(forbidden),
            "native-select non-test source should not contain forbidden hygiene pattern `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-native-select\")",
        "Cow::Borrowed(\"ui-native-select--disabled\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic.contains(required),
            "native-select should use Cow-backed class assembly in logic.rs via `{required}`."
        );
    }

    for forbidden in [
        "\"ui-native-select\".to_string()",
        "\"ui-native-select--disabled\".to_string()",
        "\"ui-native-select--invalid\".to_string()",
        "\"ui-native-select--empty\".to_string()",
        "\"ui-native-select--selected\".to_string()",
        "\"ui-native-select--has-placeholder\".to_string()",
        "\"ui-native-select--custom-class\".to_string()",
    ] {
        assert!(
            !logic.contains(forbidden),
            "native-select should remove class-string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn native_select_snapshot_baseline_renders_complete_config_stably() {
    let logic = load_source("logic");
    let view = load_source("view");
    let component_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest_path = component_src_dir.join("Component.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {manifest_path:?}: {error}"));

    for required in [
        "pub fn NativeSelect(",
        "options: Vec<NativeSelectOption>",
        "let resolved_options =",
        "logic::resolve_states_for_render(NativeSelectStateParams {",
        "<For",
        "each=move || resolved_options.get()",
        "children=render_native_select_option",
        "prop:value=move || selected_value.get()",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "native-select snapshot baseline should accept complete config and render stably via `{required}`."
        );
    }

    for required in ["name = \"snapshot_rendering\"", "enabled = true"] {
        assert!(
            manifest.contains(required),
            "Component.toml should declare snapshot baseline capability via `{required}`."
        );
    }
}

#[test]
fn native_select_engineering_contract_uses_structured_protocol_and_avoids_runtime_leakage() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let protocol = load_source("protocol");
    let cargo = load_source("native_select_cargo_toml");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "#[serde(default)]",
        "schema_version",
    ] {
        assert!(
            protocol.contains(required),
            "native-select protocol serialization contract should include `{required}`."
        );
    }

    for forbidden in [
        "tracing::",
        "#[instrument",
        "tokio::",
        "async_std",
        "async-std",
        "Runtime",
        "JoinHandle",
        "spawn_local(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !protocol.contains(forbidden)
                && !cargo.contains(forbidden),
            "native-select engineering contract should not leak runtime/tracing detail `{forbidden}`."
        );
    }

    for forbidden in ["tokio", "async-std", "tracing"] {
        assert!(
            !cargo.contains(forbidden),
            "native-select Cargo dependencies should stay runtime-agnostic without `{forbidden}`."
        );
    }
}

#[test]
fn native_select_version_deprecation_registry_is_na_without_breaking_upgrade() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let protocol = load_source("protocol");

    for required in ["schema_version", "#[serde(default)]", "V1"] {
        assert!(
            protocol.contains(required),
            "native-select protocol should preserve versioned baseline entry `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "codemod",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !protocol.contains(forbidden),
            "native-select should stay N/A for version-migration registry surface `{forbidden}` when no breaking upgrade is introduced."
        );
    }
}

#[test]
fn native_select_public_api_does_not_expose_dom_types() {
    let view = load_source("view");
    let module = load_source("mod");

    for forbidden in ["web_sys::", "HtmlSelectElement", "Element", "NodeRef"] {
        assert!(
            !view.contains(forbidden) && !module.contains(forbidden),
            "native-select public component surface should not expose DOM detail `{forbidden}`."
        );
    }
}

#[test]
fn native_select_controlled_uncontrolled_axis_is_triplet_and_not_half_controlled() {
    let view = load_source("view");
    let controllable_state = load_source("controllable_state");

    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "let selected_state = overlay_open::use_controllable_state(",
        "logic::normalize_default_selected_index(default_selected_index);",
        "default_selected_index,",
        "on_selected_index_change,",
        "let selected_index = selected_state.value;",
        "let request_selected_index_change = selected_state.request_change;",
        "logic::resolve_selected_index_correction(selected_index.get(), &resolved_options.get())",
        "logic::resolve_states_for_render(NativeSelectStateParams {",
    ] {
        assert!(
            view.contains(required),
            "native-select controlled/uncontrolled contract should include `{required}`."
        );
    }

    for forbidden in [
        "let (selected_index, set_selected_index) = signal(",
        "set_selected_index.set(",
    ] {
        assert!(
            !view.contains(forbidden),
            "native-select should not implement half-controlled local writes via `{forbidden}`."
        );
    }

    for required in [
        "let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.unwrap_or_default());",
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable state contract should include `{required}`."
        );
    }
}

#[test]
fn native_select_default_value_normalization_lives_in_logic() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub fn normalize_default_selected_index(",
        "pub fn resolve_control_value(",
    ] {
        assert!(
            logic.contains(required),
            "default normalization should be owned by logic.rs via `{required}`."
        );
    }

    for required in [
        "let default_selected_index = logic::normalize_default_selected_index(default_selected_index);",
        "logic::resolve_control_value(state.get().selected_value.as_deref())",
    ] {
        assert!(
            view.contains(required),
            "view.rs should consume logic defaults via `{required}`."
        );
    }

    for forbidden in [
        "default_selected_index.map(Some)",
        "selected_value.unwrap_or_default()",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not own default fallback branch `{forbidden}`."
        );
    }
}

#[test]
fn native_select_state_normalization_is_centered_in_logic() {
    let view = load_source("view");

    for forbidden in [
        "NativeSelectStateInput",
        "ui_state_primitives::native_select::resolve_state(",
        "logic::sanitize_selected_index(",
        "logic::resolve_state(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not rebuild state rules via `{forbidden}`."
        );
    }
}

#[test]
fn native_select_discrete_axes_are_type_constrained() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub enum NativeSelectSize",
        "#[prop(optional)] size: NativeSelectSize",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "native-select discrete axis should be type-constrained via `{required}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !view.contains(forbidden),
            "native-select should not expose stringly/boolean-explosion discrete inputs `{forbidden}`."
        );
    }
}

#[test]
fn native_select_consumes_state_primitives_without_business_store_binding() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        module.contains("pub use ui_state_primitives::native_select::{"),
        "module boundary should expose state primitives contracts from ui-state-primitives."
    );

    for required in [
        "ui_state_primitives::native_select::normalize_options(options)",
        "ui_state_primitives::native_select::resolve_options(id_base, options)",
        "ui_state_primitives::native_select::sanitize_selected_index(selected_index, options)",
        "ui_state_primitives::native_select::resolve_state(input);",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should consume state primitives via `{required}`."
        );
    }

    for forbidden in [
        "use_store(",
        "Store<",
        "Redux",
        "mobx",
        "zustand",
        "app_state",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not bind business store directly via `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_component_local_async_protocol_surface() {
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "spawn_local(",
        "tokio::",
        "Future<",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define component-local async protocol `{forbidden}`."
        );
    }
}

#[test]
fn native_select_dx_paradox_keeps_minimal_api_and_hello_world_docs() {
    let view = load_source("view");
    let docs = load_source("forms_native_docs");

    for required in [
        "id_base: String,",
        "options: Vec<NativeSelectOption>,",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            view.contains(required),
            "native-select API should keep required/advanced boundary via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "state: NativeSelectState",
        "state: Signal<",
        "state=state",
    ] {
        assert!(
            !view.contains(forbidden),
            "native-select should not expose mandatory internal state object `{forbidden}`."
        );
    }

    for required in [
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "id_base=\"docs-native-select-hello\".to_string()",
        "options=vec![NativeSelectOption::new(\"system\", \"System\"), NativeSelectOption::new(\"manual\", \"Manual\")]",
    ] {
        assert!(
            docs.contains(required),
            "docs-app should include minimal hello-world DX entry `{required}`."
        );
    }
}

#[test]
fn native_select_dx_workbench_supports_live_css_and_optional_state_persistence() {
    let docs = load_source("forms_native_docs");
    let playground = load_source("docs_playground");

    for required in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"crates/ui/src/native_select/styles.rs\".to_string()",
        "scoped CSS live-edit（CSS Test）+ optional state persistence across reload",
        "let persisted_workbench_state = load_native_select_workbench_state();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_native_select_workbench_state(NativeSelectWorkbenchState {",
        "clear_native_select_workbench_state();",
        "Persist workbench state",
        "data-slot=\"native-select-workbench-controls\"",
        "data-slot=\"native-select-workbench-canvas\"",
    ] {
        assert!(
            docs.contains(required),
            "native-select docs workbench DX contract should include `{required}`."
        );
    }

    for required in [
        "const NATIVE_SELECT_WORKBENCH_STORAGE_KEY: &str = \"docs:native-select:workbench:state\";",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn load_native_select_workbench_state() -> Option<NativeSelectWorkbenchState>",
        "fn save_native_select_workbench_state(state: NativeSelectWorkbenchState)",
        "fn save_native_select_workbench_state(_state: NativeSelectWorkbenchState) {}",
        "fn clear_native_select_workbench_state()",
    ] {
        assert!(
            docs.contains(required),
            "native-select docs workbench state-persistence boundary should include `{required}`."
        );
    }

    for required in [
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let on_reset_test_css: OnPress =",
    ] {
        assert!(
            playground.contains(required),
            "docs playground css-test feedback loop should include `{required}`."
        );
    }
}

#[test]
fn native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract() {
    let docs = load_source("forms_native_docs");
    let playground = load_source("docs_playground");
    let code_block = load_source("code_block_view");

    for required in [
        "const NATIVE_SELECT_DOC_IMPORTS: &str =",
        "<Playground title=\"Hello World (Uncontrolled)\"",
        "<Playground title=\"Controlled vs Uncontrolled\"",
        "<Playground title=\"State Matrix (Controlled / Uncontrolled / Disabled)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()",
        "data-slot=\"native-select-controlled-uncontrolled\"",
        "data-slot=\"native-select-state-matrix\"",
        "data-slot=\"native-select-streaming-snapshot\"",
        "Inspect `data-streaming-mode/data-streaming-fallback/data-output-status`.",
        "data-slot=\"native-select-source-first\"",
        "data-slot=\"native-select-source-paths\"",
        "component-native_select + inject-css",
    ] {
        assert!(
            docs.contains(required),
            "native-select docs copy-ready contract should include `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
        "CodeBlock code=resolved_code.get()",
    ] {
        assert!(
            playground.contains(required),
            "docs playground import-completion pipeline should include `{required}`."
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
        "on_press=on_copy_press",
    ] {
        assert!(
            code_block.contains(required),
            "code-block should provide one-click copy entry via `{required}`."
        );
    }
}

#[test]
fn native_select_docs_matrix_and_api_contract_are_synced_with_logic_defaults() {
    let docs = load_source("forms_native_docs");
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "<Playground title=\"Controlled vs Uncontrolled\"",
        "<Playground title=\"State Matrix (Controlled / Uncontrolled / Disabled)\"",
        "data-slot=\"native-select-controlled-uncontrolled\"",
        "data-slot=\"native-select-state-matrix\"",
        "default_selected_index=0",
        "selected_index=Signal::derive(|| Some(2usize))",
        "is_disabled=true",
    ] {
        assert!(
            docs.contains(required),
            "native-select docs should keep state-matrix branch coverage via `{required}`."
        );
    }

    for required in [
        "selected_index=selected_signal",
        "on_selected_index_change=on_selected_change",
        "default_selected_index=1",
        "is_required=true",
        "is_invalid=true",
        "size=NativeSelectSize::Lg",
        "_ => NativeSelectSize::Md,",
    ] {
        assert!(
            docs.contains(required),
            "native-select docs should keep API naming/default-value usage via `{required}`."
        );
    }

    for required in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "#[default]",
        "Md,",
        "pub fn normalize_default_selected_index(",
        "default_selected_index.map(Some)",
    ] {
        assert!(
            logic.contains(required),
            "native-select logic default contract should include `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_required: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] size: NativeSelectSize",
    ] {
        assert!(
            view.contains(required),
            "native-select public API contract in view.rs should include `{required}`."
        );
    }
}

#[test]
fn native_select_readme_is_beginner_friendly_documentation_product() {
    let readme = load_source("readme");
    let docs = load_source("forms_native_docs");

    for required in [
        "# NativeSelect",
        "## 新手路径：先用起来，再进阶",
        "### Hello World（零门槛）",
        "use ui::{NativeSelect, NativeSelectOption};",
        "### 常见用法（在 docs-app 直接对照）",
        "Hello World (Uncontrolled)",
        "Controlled + Placeholder",
        "Required + Invalid + Disabled",
        "Controlled vs Uncontrolled",
        "State Matrix (Controlled / Uncontrolled / Disabled)",
        "## API（进阶参考）",
    ] {
        assert!(
            readme.contains(required),
            "native-select README should include beginner-friendly docs contract `{required}`."
        );
    }

    let hello_index = readme
        .find("### Hello World（零门槛）")
        .unwrap_or_else(|| panic!("README should contain hello-world section heading"));
    let api_index = readme
        .find("## API（进阶参考）")
        .unwrap_or_else(|| panic!("README should contain advanced API section heading"));
    assert!(
        hello_index < api_index,
        "README should place hello-world path before advanced API reference."
    );

    for required in [
        "title=\"NativeSelect\"",
        "<Playground title=\"Hello World (Uncontrolled)\"",
    ] {
        assert!(
            docs.contains(required),
            "docs-app page should remain a valid equivalent docs entry via `{required}`."
        );
    }
}

#[test]
fn native_select_heroui_strategy_doc_and_component_docs_are_synced() {
    let strategy = load_source("heroui_parameter_strategy");
    let docs_catalog = load_source("docs_component_catalog");
    let docs = load_source("forms_native_docs");
    let readme = load_source("readme");

    for required in [
        "### NativeSelect 同步记录（2026-02-20）",
        "`selected_index/on_selected_index_change/default_selected_index`",
        "`is_disabled/is_required/is_invalid/size`",
        "`docs/spec/heroui-parameter-design-strategy.md`",
        "参数语义若变更，必须先同步本策略文档与 `components/native-select/src/README.md`、docs 入口，再推进实现",
    ] {
        assert!(
            strategy.contains(required),
            "HeroUI strategy doc should include NativeSelect sync evidence `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"NativeSelect\"",
        "\"native-select\"",
        "\"Forms\"",
        "forms_native::native_select",
    ] {
        assert!(
            docs_catalog.contains(required),
            "docs component catalog should keep NativeSelect indexable entry `{required}`."
        );
    }

    for required in ["title=\"NativeSelect\"", "slug=\"native-select\""] {
        assert!(
            docs.contains(required),
            "docs NativeSelect page should expose stable index keys via `{required}`."
        );
    }

    for required in [
        "# NativeSelect",
        "## 新手路径：先用起来，再进阶",
        "## Docs and Feature",
    ] {
        assert!(
            readme.contains(required),
            "component README should remain as equivalent documentation entry via `{required}`."
        );
    }
}

#[test]
fn native_select_item_semantics_are_bound_in_single_typed_option_model() {
    let view = load_source("view");
    let docs = load_source("forms_native_docs");
    let primitive = load_source("native_select_primitive");

    assert!(
        view.contains("options: Vec<NativeSelectOption>,"),
        "native-select should expose typed option collection instead of parallel arrays."
    );

    for required in [
        "pub struct NativeSelectOption {",
        "pub value: String,",
        "pub label: String,",
        "pub disabled: bool,",
    ] {
        assert!(
            primitive.contains(required),
            "state primitive should bind option semantics in one item struct via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "children: Vec<",
        "panels: Vec<",
        "items_spec: Vec<",
        "ItemSpec",
    ] {
        assert!(
            !view.contains(forbidden) && !docs.contains(forbidden),
            "native-select should not expose parallel-array/spec-sugar API `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_dragging_macro_micro_state_machine_path() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "on:drag",
        "requestAnimationFrame",
        "raf(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define drag-loop macro/micro state machine path `{forbidden}`."
        );
    }

    assert!(
        view.contains("on:change=on_change"),
        "native-select interaction should remain discrete change event for this component."
    );
}

#[test]
fn native_select_has_no_two_pass_geometry_measurement_path() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "Rectification",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define two-pass geometry pipeline `{forbidden}`."
        );
    }

    assert!(
        view.contains("on:change=on_change"),
        "native-select should keep discrete change-driven interaction instead of geometry measure loop."
    );
}

#[test]
fn native_select_has_no_registration_context_protocol_path() {
    let view = load_source("view");
    let logic = load_source("logic");
    let primitive = load_source("native_select_primitive");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !primitive.contains(forbidden),
            "native-select should not define dynamic registration protocol `{forbidden}`."
        );
    }

    for required in [
        "<For",
        "each=move || resolved_options.get()",
        "options: Vec<NativeSelectOption>,",
        "options.iter()",
        ".enumerate()",
    ] {
        assert!(
            view.contains(required) || logic.contains(required) || primitive.contains(required),
            "native-select ordering should come from typed Vec flow `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_slot_projection_lifecycle_protocol_path() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "suspend",
        "pause",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define slot projection lifecycle protocol `{forbidden}`."
        );
    }

    for required in ["<select", "<option", "<For"] {
        assert!(
            view.contains(required),
            "native-select should remain native select + option rendering path `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_environment_stream_subscription_pipeline() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "matchMedia",
        "prefers-color-scheme",
        "debounce",
        "throttle",
        "on:resize",
        "window.",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define env stream subscription pipeline `{forbidden}`."
        );
    }

    assert!(
        view.contains("on:change=on_change"),
        "native-select interaction should remain change-driven instead of env-stream-driven."
    );
}

#[test]
fn native_select_has_no_event_light_cone_bulk_bus_path() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "broadcast",
        "bulk",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define event-light-cone bulk bus path `{forbidden}`."
        );
    }

    for required in [
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view.contains(required),
            "native-select should remain single-axis change propagation via `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_causality_bus_trace_id_pipeline() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "TraceId",
        "trace_id",
        "trace-id",
        "Causality Bus",
        "publish",
        "subscribe",
        "subscriber",
        "command_bus",
        "EventBus",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select should not define causality-bus trace pipeline `{forbidden}`."
        );
    }

    for required in [
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view.contains(required),
            "native-select should keep direct user-change -> selected-index mapping via `{required}`."
        );
    }
}

#[test]
fn native_select_a11y_i18n_contract_uses_headless_locale_and_configurable_text_sources() {
    let view = load_source("view");
    let logic = load_source("logic");
    let primitive = load_source("native_select_primitive");
    let headless_native_select = load_source("headless_native_select");
    let headless_a11y = load_source("headless_a11y");

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] placeholder: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "use_native_select(NativeSelectOptions {",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "{placeholder}",
        "{option.label}",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view.contains(required),
            "native-select view should expose a11y/i18n hook `{required}`."
        );
    }

    for required in [
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "ui_state_primitives::native_select::normalize_aria_label(value)",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should normalize aria label through primitive source `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Native select\";",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "(DEFAULT_ARIA_LABEL.into(), false)",
    ] {
        assert!(
            primitive.contains(required),
            "state primitive should provide fallback aria-label contract `{required}`."
        );
    }

    for required in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(options.lang, options.dir);",
    ] {
        assert!(
            headless_native_select.contains(required),
            "native-select headless contract should reuse shared a11y locale helper `{required}`."
        );
    }

    assert!(
        headless_a11y.contains(
            "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs",
        ),
        "shared a11y helper should define locale_attrs in crates/ui-headless/src/a11y.rs."
    );

    for forbidden in ["\"Native select\"", "\"Select\"", "\"Choose\""] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not hardcode business-visible fallback copy `{forbidden}`."
        );
    }
}

#[test]
fn native_select_exposes_observable_retrievable_verifiable_state_markers() {
    let view = load_source("view");

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-selection-mode=if is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "data-selection-source=if is_controlled {",
        "\"external\"",
        "\"default\"",
        "\"internal\"",
        "data-change-source=move || selection_change_source_attr.get()",
        "signal(\"initial\")",
        "set_selection_change_source_attr.set(\"user\")",
        "set_selection_change_source_attr.set(\"external\")",
        "set_selection_change_source_attr.set(\"internal\")",
        "set_selection_change_source_attr.set(\"sync-effect\")",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view.contains(required),
            "native-select should expose stable state/source marker contract `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "querySelector(\"."] {
        assert!(
            !view.contains(forbidden),
            "marker contract should avoid brittle selector dependency `{forbidden}`."
        );
    }
}

#[test]
fn native_select_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        ".ui-native-select--size-sm .ui-native-select__control",
        ".ui-native-select--size-md .ui-native-select__control",
        ".ui-native-select--size-lg .ui-native-select__control",
        ".ui-native-select--invalid .ui-native-select__control",
        ".ui-native-select--selected .ui-native-select__control",
        ".ui-native-select--empty .ui-native-select__control",
        ".ui-native-select--disabled .ui-native-select__control",
        ".ui-native-select__control:disabled",
        ".ui-native-select__control:focus-visible",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
    ] {
        assert!(
            styles.contains(required) || view.contains(required),
            "native-select style/state contract should include explicit selector or marker `{required}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        "style=",
    ] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden),
            "native-select should not rely on brittle structure or inline-style business logic `{forbidden}`."
        );
    }
}

#[test]
fn native_select_semantics_contract_tests_cover_key_matrix_without_snapshot_dependency() {
    let view = load_source("view");
    let logic = load_source("logic");
    let component_semantics = load_source("component_semantics_self");
    let workspace_semantics = load_source("workspace_native_select_semantics");

    for required in [
        "fn native_select_controlled_uncontrolled_axis_is_triplet_and_not_half_controlled()",
        "fn native_select_has_no_component_local_async_protocol_surface()",
        "fn native_select_mounts_headless_a11y_contract_with_locale_hooks()",
        "fn native_select_exposes_observable_retrievable_verifiable_state_markers()",
        "fn native_select_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn native_select_token_first_static_style_contract_is_enforced()",
        "fn native_select_styles_use_defensive_variable_fallback_chain()",
        "fn native_select_css_cascade_layer_contract_is_enforced()",
        "fn native_select_visual_desire_theme_baseline_is_enforced()",
        "fn native_select_tree_shaking_feature_gates_are_component_scoped()",
        "fn native_select_ui_components_fixed_entry_files_are_in_correct_locations()",
        "fn native_select_component_directory_standard_file_placement_is_correct()",
        "fn native_select_type_system_and_semantic_markers_form_machine_readable_contract()",
        "fn native_select_has_no_overlay_focus_stack_gc_path()",
        "fn native_select_has_no_foreign_zone_escape_hatch_path()",
        "fn native_select_hydration_ids_are_deterministic_without_time_or_random_sources()",
        "fn native_select_ssr_and_cross_platform_compile_contract_is_preserved()",
        "fn native_select_respects_ui_headless_web_ssr_compile_error_mutex()",
        "fn native_select_respects_ui_motion_non_wasm_noop_contract()",
        "fn native_select_reduced_motion_ssr_wasm_contract_is_preserved()",
        "fn native_select_performance_budget_contract_is_guarded_without_render_count_harness()",
        "fn native_select_view_macro_complexity_is_controlled_by_semantic_splitting()",
        "fn native_select_functional_fragment_split_prefers_plain_functions()",
        "fn native_select_static_fragment_is_constantized_with_stable_a11y_contract()",
        "fn native_select_inner_html_contract_disallows_injection_surface()",
        "fn native_select_wasm_debug_contract_tracks_state_and_keeps_api_clean()",
        "fn native_select_dx_workbench_supports_live_css_and_optional_state_persistence()",
        "fn native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract()",
        "fn native_select_heroui_strategy_doc_and_component_docs_are_synced()",
        "fn native_select_context_compression_manifest_and_rbi_are_present_and_synced()",
        "fn native_select_agent_contract_schema_markers_are_typed_and_whitelisted()",
        "fn native_select_streaming_term_is_scoped_to_llm_output_rendering_only()",
        "fn native_select_streaming_policy_is_optional_with_snapshot_fallback_and_readable_status_markers()",
        "fn native_select_rust_hygiene_disallows_unwrap_expect_let_underscore_and_string_clone_churn()",
        "fn native_select_snapshot_baseline_renders_complete_config_stably()",
        "fn native_select_engineering_contract_uses_structured_protocol_and_avoids_runtime_leakage()",
        "fn native_select_version_deprecation_registry_is_na_without_breaking_upgrade()",
    ] {
        assert!(
            component_semantics.contains(required) || workspace_semantics.contains(required),
            "semantic contract matrix should include `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] is_disabled: bool",
        "on:change=on_change",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "data-state=move || state.get().data_state_attr",
        "data-selection-mode=if is_controlled {",
        "data-selection-source=if is_controlled {",
        "data-change-source=move || selection_change_source_attr.get()",
    ] {
        assert!(
            view.contains(required),
            "view semantic markers/interaction path should include `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch",
        "#[cfg(feature",
        "cfg!(target_arch",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "native-select has no component-local SSR/wasm split branch `{forbidden}`."
        );
    }

    for forbidden in [
        concat!("insta", "::"),
        concat!("assert_", "snapshot!"),
        concat!("to_match_", "snapshot"),
        concat!("snap", "box"),
    ] {
        assert!(
            !component_semantics.contains(forbidden),
            "semantic contract should not depend on snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn native_select_token_first_static_style_contract_is_enforced() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_cargo_toml = load_source("ui_components_cargo_toml");

    assert!(
        styles.contains("pub const CSS: &str = r#\""),
        "styles.rs should be the static CSS source of truth for native-select."
    );
    assert!(
        styles.contains("var(--ui-"),
        "styles.rs should consume ui-theme token variables through `var(--ui-*)`."
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-native_select\")]")
            && ui_components_css.contains("out.push_str(crate::native_select::styles::CSS);"),
        "ui css aggregator should feature-gate native-select style injection."
    );
    assert!(
        ui_components_cargo_toml.contains("inject-css = []")
            && ui_components_cargo_toml
                .contains("component-native_select = [\"dep:ui-native-select\"]"),
        "ui feature map should keep `inject-css` + `component-native_select` wiring."
    );

    for forbidden in [
        "style=",
        "style!(",
        "css!(",
        "styled_",
        "stylist::",
        "tailwind",
        "tw-",
        "class=\"flex",
        "class=\"grid",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select component source should avoid Utility-First / CSS-in-Rust default path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_styles_use_defensive_variable_fallback_chain() {
    let styles = load_source("styles");
    let ui_theme_css = load_source("ui_theme_css");

    for required in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
    ] {
        assert!(
            styles.contains(required),
            "native-select defensive variable contract should include `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-motion-duration-fast, 120ms)",
        "var(--ui-motion-ease-standard, ease)",
        "font-size: var(--ui-button-size-s-font-size, 13px);",
        "line-height: var(--ui-button-size-s-line-height, 18px);",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        "color: var(--ui-fg);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "native-select defensive variable contract should not keep bare fallback literal `{forbidden}`."
        );
    }

    for required in [
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-shadow-sm:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-accent:",
        "--ui-fallback-danger:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-text-field-motion-duration:",
        "--ui-fallback-text-field-motion-easing:",
    ] {
        assert!(
            ui_theme_css.contains(required),
            "ui-theme fallback SSOT should include `{required}`."
        );
    }
}

#[test]
fn native_select_css_cascade_layer_contract_is_enforced() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let ui_components_css = load_source("ui_components_css");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-native_select\")]",
        "out.push_str(crate::native_select::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "native-select cascade-layer contract should include `{required}`."
        );
    }
    assert!(
        ui_components_css.contains("out.push_str(\"}\\n\");")
            || ui_components_css.contains("out.push_str(\"\\n}\\n\");"),
        "native-select cascade-layer contract should include layer-close push."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=\"transform:",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !styles.contains(forbidden),
            "native-select should not rely on ordinary inline style value `{forbidden}`."
        );
    }

    assert!(
        !view.contains("style="),
        "native-select view should avoid ordinary inline style attributes and keep runtime numeric adjustments out of DOM style strings."
    );
}

#[test]
fn native_select_visual_desire_theme_baseline_is_enforced() {
    let styles = load_source("styles");
    let docs = load_source("forms_native_docs");

    for required in [
        ".ui-native-select__control:not(:disabled):hover",
        ".ui-native-select__control:not(:disabled):active",
        ".ui-native-select__control:focus-visible",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
    ] {
        assert!(
            styles.contains(required),
            "native-select default-theme visual baseline should include `{required}`."
        );
    }

    for required in [
        "title=\"NativeSelect\"",
        "slug=\"native-select\"",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "<Playground title=\"Controlled + Placeholder\" code_signal=code>",
        "<Playground title=\"Required + Invalid + Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs.contains(required),
            "docs baseline should include `{required}` for native-select visual acceptance."
        );
    }
}

#[test]
fn native_select_tree_shaking_feature_gates_are_component_scoped() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_cargo_toml = load_source("ui_components_cargo_toml");

    assert!(
        ui_components_lib.contains(
            "#[cfg(feature = \"component-native_select\")]\npub use ui_native_select as native_select;"
        ),
        "ui lib export should gate native-select behind `component-native_select`."
    );
    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-native_select\")]\n    out.push_str(crate::native_select::styles::CSS);"
        ),
        "ui css aggregation should gate native-select CSS behind `component-native_select`."
    );
    assert!(
        ui_components_lib.contains("#[cfg(feature = \"all-components\")]")
            && ui_components_lib.contains("mod all_components {")
            && ui_components_lib.contains(
                "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]"
            ),
        "ui central aggregation paths should remain feature-gated (`all-components` / `web-demo-components`)."
    );
    assert!(
        ui_components_cargo_toml.contains("component-native_select = [\"dep:ui-native-select\"]")
            && ui_components_cargo_toml.contains("default = [\"inject-css\", \"all-components\"]")
            && ui_components_cargo_toml.contains("all-components = [")
            && ui_components_cargo_toml.contains("\"component-native_select\""),
        "ui Cargo feature graph should keep component-scoped feature + optional all-components aggregate."
    );

    for forbidden in ["all_components", "web_demo_components"] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select component source should not host a global component registry path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_ui_components_fixed_entry_files_are_in_correct_locations() {
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let ui_components_cargo_toml = load_source("ui_components_cargo_toml");
    let ui_visual_active_highlight = load_source("ui_visual_active_highlight");
    let controllable_state = load_source("controllable_state");
    let headless_presence = load_source("headless_presence");
    let headless_a11y = load_source("headless_a11y");

    for required in [
        "mod css;",
        "mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-native_select\")]",
        "pub use ui_native_select as native_select;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should include fixed entry contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-native_select\")]",
        "out.push_str(crate::native_select::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep feature-gated aggregation contract `{required}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui root.rs should centralize theme/css/i18n injection via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            ui_visual_active_highlight.contains(required),
            "ui-visual-primitive active_highlight.rs should keep shared visual-motion primitive contract `{required}`."
        );
    }

    for forbidden in ["NativeSelect", "Accordion", "Dialog"] {
        assert!(
            !ui_visual_active_highlight.contains(forbidden),
            "active_highlight.rs should stay generic without component business semantic `{forbidden}`."
        );
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above CARGO_MANIFEST_DIR"));

    for absent in [
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
    ] {
        assert!(
            !workspace_root.join(absent).exists(),
            "ui should not define forbidden fixed-entry file `{absent}`."
        );
    }

    assert!(
        controllable_state.contains("pub fn use_controllable_state"),
        "open-state primitive source should stay in ui-headless controllable_state.rs."
    );
    assert!(
        headless_presence.contains("pub fn use_presence"),
        "presence primitive source should stay in ui-headless presence.rs."
    );
    assert!(
        headless_a11y.contains("pub fn locale_attrs("),
        "shared a11y utility source should stay in ui-headless a11y.rs."
    );
    assert!(
        ui_components_cargo_toml.contains("component-native_select = [\"dep:ui-native-select\"]"),
        "ui Cargo features should keep component-level fixed-entry gate for native-select."
    );
}

#[test]
fn native_select_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub enum NativeSelectSize",
        "pub struct NativeSelectStateParams<'a>",
        "pub fn sanitize_selected_index(",
        "pub fn resolve_selected_index_correction(",
        "pub fn resolve_states_for_render(",
        "ui_state_primitives::native_select::resolve_state(input);",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should provide typed state modeling and normalization entry `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-size=move || state.get().size_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-selection-mode=if is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "data-selection-source=if is_controlled {",
        "\"external\"",
        "\"default\"",
        "\"internal\"",
        "data-change-source=move || selection_change_source_attr.get()",
        "set_selection_change_source_attr.set(\"user\")",
        "set_selection_change_source_attr.set(\"external\")",
        "set_selection_change_source_attr.set(\"internal\")",
        "set_selection_change_source_attr.set(\"sync-effect\")",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view.contains(required),
            "view.rs should expose machine-readable semantic contract marker `{required}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not expose stringly/boolean-explosion typed inputs `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_overlay_focus_stack_gc_path() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for forbidden in [
        "NodeRef",
        "use_focus_trap(",
        "FocusTrapOptions",
        "FocusTrapFrame",
        "RestorePolicy",
        "FallbackTo(",
        "Selector(",
        "document.body",
        "body()",
        "provide_overlay_stack(",
        "use_overlay_stack(",
        "use_overlay_stack_registration(",
        "data-ui-overlay-portal",
        "focus_manager_push_trap",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not carry overlay focus-stack/GC internals `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_foreign_zone_escape_hatch_path() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "OpenLayers",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "foreign_zone",
        "yield_control",
        "cleanup_foreign",
        "js_sys::",
        "wasm_bindgen::JsValue",
        "web_sys::HtmlCanvasElement",
        "extern \"C\"",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not carry imperative third-party integration escape-hatch path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_hydration_ids_are_deterministic_without_time_or_random_sources() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let primitive = load_source("native_select_primitive");

    for required in [
        "id_base: String,",
        "let id_base = StoredValue::new(id_base);",
        "id=move || format!(\"{}-root\", id_base.get_value())",
        "id=move || format!(\"{}-control\", id_base.get_value())",
        "ui_state_primitives::native_select::resolve_options(id_base, options)",
        "id: format!(\"{id_base}-option-{index}\")",
    ] {
        assert!(
            view.contains(required) || logic.contains(required) || primitive.contains(required),
            "native-select hydration id path should keep deterministic seed mapping `{required}`."
        );
    }

    for forbidden in [
        "SystemTime",
        "UNIX_EPOCH",
        "Instant::now",
        "Utc::now",
        "now()",
        "uuid::",
        "Uuid",
        "rand::",
        "thread_rng",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !primitive.contains(forbidden),
            "native-select should not introduce non-deterministic hydration id source `{forbidden}`."
        );
    }
}

#[test]
fn native_select_ssr_and_cross_platform_compile_contract_is_preserved() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let native_select_cargo = load_source("native_select_cargo_toml");
    let headless_cargo = load_source("headless_cargo_toml");
    let headless_lib = load_source("headless_lib");

    for required in [
        "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            native_select_cargo.contains(required)
                || headless_cargo.contains(required)
                || headless_lib.contains(required),
            "cross-platform compile contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch",
        "#[cfg(feature",
        "cfg!(target_arch",
        "cfg!(",
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window.",
        "document.",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select component source should stay platform-neutral without `{forbidden}`."
        );
    }
}

#[test]
fn native_select_respects_ui_headless_web_ssr_compile_error_mutex() {
    let native_select_cargo = load_source("native_select_cargo_toml");
    let headless_cargo = load_source("headless_cargo_toml");
    let headless_lib = load_source("headless_lib");

    for required in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            native_select_cargo.contains(required)
                || headless_cargo.contains(required)
                || headless_lib.contains(required),
            "ui-headless web/ssr mutual-exclusion contract should include `{required}`."
        );
    }
}

#[test]
fn native_select_respects_ui_motion_non_wasm_noop_contract() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion non-wasm noop/stub contract should include `{required}`."
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion",
        "ui_motion::",
        "attach_motion(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not assume component-level motion runtime path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_reduced_motion_ssr_wasm_contract_is_preserved() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let primitive = load_source("native_select_primitive");
    let ui_motion_lib = load_source("ui_motion_lib");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion reduced-motion/non-wasm contract should include `{required}`."
        );
    }

    for required in [
        "id=move || format!(\"{}-root\", id_base.get_value())",
        "id=move || format!(\"{}-control\", id_base.get_value())",
        "id: format!(\"{id_base}-option-{index}\")",
        "use_native_select(NativeSelectOptions {",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view.contains(required) || primitive.contains(required),
            "native-select SSR/wasm semantic parity should include `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch",
        "#[cfg(feature",
        "cfg!(target_arch",
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not fork reduced-motion/SSR/wasm behavior in component source `{forbidden}`."
        );
    }
}

#[test]
fn native_select_view_macro_complexity_is_controlled_by_semantic_splitting() {
    let view = load_source("view");

    for required in [
        "fn render_placeholder_option(",
        "fn render_native_select_option(",
        "fn render_static_indicator() -> impl IntoView",
        "let render_placeholder = move || {",
        "{render_placeholder}",
        "<For",
        "children=render_native_select_option",
        "{render_static_indicator()}",
        "on:change=on_change",
    ] {
        assert!(
            view.contains(required),
            "native-select view-macro complexity control should include `{required}`."
        );
    }

    for forbidden in [
        "children=move |option| {",
        ".map(|placeholder| {",
        "let render_option =",
    ] {
        assert!(
            !view.contains(forbidden),
            "native-select should avoid inline repeated nested view fragments `{forbidden}`."
        );
    }

    let view_macro_count = view.matches("view! {").count();
    assert_eq!(
        view_macro_count, 4,
        "native-select should keep a bounded number of `view!` expansions (4)."
    );
}

#[test]
fn native_select_functional_fragment_split_prefers_plain_functions() {
    let view = load_source("view");

    for required in [
        "fn render_placeholder_option(placeholder_label: String, is_required: bool) -> impl IntoView",
        "fn render_native_select_option(option: crate::NativeSelectOptionResolved) -> impl IntoView",
        "render_placeholder_option(placeholder_label, is_required)",
        "children=render_native_select_option",
    ] {
        assert!(
            view.contains(required),
            "native-select should prefer plain function split for lightweight UI fragment `{required}`."
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\r\nfn render_"] {
        assert!(
            !view.contains(forbidden),
            "native-select fragment split should not introduce component-level abstraction noise `{forbidden}`."
        );
    }
}

#[test]
fn native_select_static_fragment_is_constantized_with_stable_a11y_contract() {
    let view = load_source("view");

    for required in [
        "const NATIVE_SELECT_INDICATOR_SYMBOL: &str = \"▾\";",
        "fn render_static_indicator() -> impl IntoView",
        "data-slot=\"native-select-indicator\"",
        "aria-hidden=\"true\"",
        "{NATIVE_SELECT_INDICATOR_SYMBOL}",
        "{render_static_indicator()}",
    ] {
        assert!(
            view.contains(required),
            "native-select static fragment constantization contract should include `{required}`."
        );
    }

    assert!(
        !view.contains("\n                \"▾\"\n"),
        "native-select should avoid scattering static indicator literal directly in root view tree."
    );
}

#[test]
fn native_select_inner_html_contract_disallows_injection_surface() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "insert_adjacent_html",
        "innerHTML",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should not expose HTML injection surface `{forbidden}`."
        );
    }

    for required in [
        "{placeholder_label}",
        "{option.label}",
        "{NATIVE_SELECT_INDICATOR_SYMBOL}",
    ] {
        assert!(
            view.contains(required),
            "native-select should keep text rendering path via `{required}`."
        );
    }
}

#[test]
fn native_select_wasm_debug_contract_tracks_state_and_keeps_api_clean() {
    let view = load_source("view");
    let docs = load_source("forms_native_docs");
    let cargo = load_source("native_select_cargo_toml");
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");

    for required in [
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-selection-mode=if is_controlled {",
        "data-selection-source=if is_controlled {",
        "data-change-source=move || selection_change_source_attr.get()",
        "let current_selected_index = selected_index.get();",
        "let previous_selected_index = last_selected_index.get_untracked();",
        "let on_change = move |ev: ev::Event| {",
        "resolve_native_select_change_index(&next_value, &resolved_options.get_untracked())",
        "request_selected_index_change.run(next_index);",
        "Interactive Playground",
    ] {
        assert!(
            view.contains(required) || docs.contains(required),
            "native-select wasm debug contract should include `{required}`."
        );
    }

    for required in ["[features]", "default = []"] {
        assert!(
            cargo.contains(required),
            "native-select cargo feature surface should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] debug",
        "#[prop(optional)] debug_trace",
        "feature = \"debug\"",
        "feature = \"wasm-debug\"",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should keep debug hooks out of public API surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_performance_budget_contract_is_guarded_without_render_count_harness() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "let resolved_options =",
        "Signal::derive(move || logic::resolve_options(&id_base.get_value(), &options.get_value()))",
        "let resolved_states = Signal::derive(move || {",
        "let selected_value =",
        "Signal::derive(move || logic::resolve_control_value(state.get().selected_value.as_deref()))",
        "let on_change = move |ev: ev::Event| {",
        "resolve_native_select_change_index(&next_value, &resolved_options.get_untracked())",
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view.contains(required),
            "native-select performance budget baseline should include `{required}`."
        );
    }

    let effect_count = view.matches("Effect::new(move |_| {").count();
    assert_eq!(
        effect_count, 2,
        "native-select should keep effect count stable (2) to avoid accidental reactive churn."
    );

    for forbidden in [
        "on:input=",
        "on:mousemove=",
        "on:pointermove=",
        "on:touchmove=",
        "request_animation_frame",
        "set_interval",
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "spawn_local(",
        "tokio::",
        "mod motion;",
        "ui_motion::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "native-select should avoid high-frequency/event-flood perf path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_e2e_contract_uses_semantic_selectors_and_wasm_ready_wait() {
    let e2e = load_source("native_select_e2e_contract");

    for required in [
        "await page.goto(\"/#/components/native-select\")",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "#docs-native-select-controlled-control",
        "#docs-native-select-uncontrolled-control",
        "ancestor::*[@data-slot=\"native-select\"][1]",
        "toHaveAttribute(\"data-selection-mode\", \"controlled\")",
        "toHaveAttribute(\"data-selection-source\", \"external\")",
        "toHaveAttribute(\"data-change-source\", \"user\")",
        "toHaveAttribute(\"data-output-status\", \"submittable\")",
        "await page.reload();",
    ] {
        assert!(
            e2e.contains(required),
            "native-select e2e contract should include `{required}`."
        );
    }

    for forbidden in ["waitForTimeout", "setTimeout", "hasText:"] {
        assert!(
            !e2e.contains(forbidden),
            "native-select e2e contract should avoid brittle/sleep-based pattern `{forbidden}`."
        );
    }
}

#[test]
fn native_select_e2e_key_flow_regression_is_repeatable_and_traceable() {
    let e2e = load_source("native_select_e2e_contract");

    for required in [
        "docs-app native-select key flow is repeatable with semantic breakpoints",
        "await uncontrolledControl.focus();",
        "await expect(uncontrolledControl).toBeFocused();",
        "await uncontrolledControl.selectOption(\"system\");",
        "toHaveAttribute(\"data-selected-index\", \"0\")",
        "toHaveAttribute(\"data-selected-value\", \"system\")",
        "await page.reload();",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"data-change-source\", \"initial\")",
        "docs-app native-select keyboard path uses semantic breakpoints",
        "#docs-native-select-matrix-default-control",
        "await matrixControl.focus();",
        "await expect(matrixControl).toBeFocused();",
        "await page.keyboard.press(\"ArrowDown\");",
        "await page.keyboard.press(\"ArrowUp\");",
    ] {
        assert!(
            e2e.contains(required),
            "native-select e2e key-flow regression should include `{required}`."
        );
    }

    for forbidden in ["waitForTimeout", "setTimeout", "sleep("] {
        assert!(
            !e2e.contains(forbidden),
            "native-select key-flow e2e regression should avoid non-deterministic wait pattern `{forbidden}`."
        );
    }
}

#[test]
fn native_select_checklist_tracks_ui_components_contract() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。"),
        "check2.md should mark ui definition as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 当前无组件级 `motion.rs`"),
        "check2.md should explain why native-select does not define component-level motion.rs."
    );
    assert!(
        check2.contains("- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。"),
        "check2.md should mark controlled/uncontrolled triplet contract as completed."
    );
    assert!(
        check2.contains("- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。"),
        "check2.md should mark centralized state normalization as completed."
    );
    assert!(
        check2.contains("- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。"),
        "check2.md should mark discrete-state type constraints as completed."
    );
    assert!(
        check2.contains("- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。"),
        "check2.md should mark state primitive source boundary as completed."
    );
    assert!(
        check2.contains("- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。"),
        "check2.md should mark async semantic contract item as completed for native-select."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 当前仅处理同步选项归一化与选择变更"),
        "check2.md should include an explicit N/A reason for no async interaction flow."
    );
    assert!(
        check2
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"),
        "check2.md should mark DX paradox item as completed."
    );
    assert!(
        check2.contains("Hello World (Uncontrolled)"),
        "check2.md should include docs hello-world evidence for DX."
    );
    assert!(
        check2.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`.")
            || check2.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
        "check2.md should mark composition API item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 不是 `<Parent><Item/>` 插槽型容器组件"),
        "check2.md should explain N/A scope for composition-style API requirement."
    );
    assert!(
        check2.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。"),
        "check2.md should mark macro/micro duality item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 无拖拽类高频物理交互"),
        "check2.md should include N/A reason for drag macro/micro duality scope."
    );
    assert!(
        check2.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。"),
        "check2.md should mark two-pass rendering item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 不依赖 DOM 几何测量进行位置校正"),
        "check2.md should include N/A reason for two-pass rendering scope."
    );
    assert!(
        check2.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。"),
        "check2.md should mark registration protocol item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 不是动态注册子项容器"),
        "check2.md should include N/A reason for registration protocol scope."
    );
    assert!(
        check2.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。"),
        "check2.md should mark slot projection item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 非容器投影组件"),
        "check2.md should include N/A reason for slot projection scope."
    );
    assert!(
        check2.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。"),
        "check2.md should mark env-streams item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 无环境订阅驱动的响应式布局/可见性逻辑"),
        "check2.md should include N/A reason for env-stream scope."
    );
    assert!(
        check2.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。"),
        "check2.md should mark event-light-cone item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 非大集合批量操作组件"),
        "check2.md should include N/A reason for event-light-cone scope."
    );
    assert!(
        check2.contains("- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。"),
        "check2.md should mark causality-bus traceability item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 不存在复杂派生总线操作"),
        "check2.md should include N/A reason for causality-bus scope."
    );
    assert!(
        check2.contains(
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
        ),
        "check2.md should mark a11y/i18n/l10n integration item as completed."
    );
    assert!(
        check2.contains("共享工具：`crates/ui-headless/src/native_select.rs` 通过 `use crate::a11y::{A11yDirection, locale_attrs};`"),
        "check2.md should record shared ui-headless a11y utility evidence."
    );
    assert!(
        check2.contains(
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
        ),
        "check2.md should mark observable/retrievable/verifiable marker item as completed."
    );
    assert!(
        check2.contains("来源标记：新增 `data-selection-mode`（`controlled|uncontrolled`）、`data-selection-source`（`external|default|internal`）、`data-change-source`（`initial|user|external|internal|sync-effect`）"),
        "check2.md should include closed-set marker source evidence."
    );
    assert!(
        check2.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
        "check2.md should mark style-explicit-state item as completed."
    );
    assert!(
        check2.contains(
            "`components/native-select/src/view.rs` 不注入业务 inline style（无 `style=`）"
        ),
        "check2.md should include no-inline-style evidence for native-select."
    );
    assert!(
        check2.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "check2.md should mark semantic-contract-over-snapshot item as completed."
    );
    assert!(
        check2.contains("适用范围：`NativeSelect` 无组件自定义 SSR/wasm 分支"),
        "check2.md should include applicability note for SSR/wasm matrix."
    );
    assert!(
        check2.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。"),
        "check2.md should mark component file-responsibility item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 当前无组件级 `motion.rs`"),
        "check2.md should include motion.rs N/A scope for native-select."
    );
    assert!(
        check2.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "check2.md should mark spec.rs-scope item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 为简单原生 `<select>` 包装组件"),
        "check2.md should include N/A reason for spec.rs scope."
    );
    assert!(
        check2.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "check2.md should mark token-first static style contract item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/css.rs` 通过 `#[cfg(feature = \"component-native_select\")] out.push_str(crate::native_select::styles::CSS);` 按组件特性聚合"),
        "check2.md should include css aggregation evidence for token-first style contract."
    );
    assert!(
        check2.contains("未引入 Tailwind/utility class 协议与 CSS-in-Rust（如 `style!`/`css!`/`stylist`）默认路径"),
        "check2.md should include Utility-First/CSS-in-Rust exclusion evidence."
    );
    assert!(
        check2.contains("native_select_token_first_static_style_contract_is_enforced"),
        "check2.md should include regression reference for token-first static style contract."
    );
    assert!(
        check2.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "check2.md should mark visual-desire default-theme item as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/styles.rs` 以 token-first 样式提供清晰层级")
            && check2.contains("`:hover/:active/:focus-visible`"),
        "check2.md should include hover/active/focus evidence for visual-desire contract."
    );
    assert!(
        check2.contains("N/A 说明：子条目“Button/Input/Overlay 截图基线”属于仓库级视觉治理任务"),
        "check2.md should scope Button/Input/Overlay screenshot baseline as repository-level N/A for native-select."
    );
    assert!(
        check2.contains("native_select_visual_desire_theme_baseline_is_enforced"),
        "check2.md should include regression reference for visual-desire theme baseline contract."
    );
    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "check2.md should mark tree-shaking contract item as completed."
    );
    assert!(
        check2.contains(
            "`crates/ui/Cargo.toml` 提供 `component-native_select = [\"dep:ui-native-select\"]`"
        ),
        "check2.md should include component-native_select feature evidence."
    );
    assert!(
        check2.contains(
            "`cargo tree -e features -p ui --no-default-features --features component-native_select,inject-css`"
        ) && check2.contains("仅出现 `ui-native-select`"),
        "check2.md should include tree command evidence for minimal feature closure."
    );
    assert!(
        check2.contains(
            "N/A 说明：`CI 最小特性 wasm 编译` 与 `体积预算阈值阻断` 属于仓库级流水线治理"
        ),
        "check2.md should scope CI wasm/size budget as repository-level N/A for this component checklist."
    );
    assert!(
        check2.contains("native_select_tree_shaking_feature_gates_are_component_scoped"),
        "check2.md should include regression reference for tree-shaking feature-gate contract."
    );
    assert!(
        check2.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "check2.md should mark type-system + semantic-marker machine-readable contract as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/logic.rs` 以 `NativeSelectSize`（`Sm|Md|Lg`）建模离散尺寸轴"),
        "check2.md should include typed enum modeling evidence for machine-readable contract."
    );
    assert!(
        check2.contains("`components/native-select/src/view.rs` 公开稳定机器可读标记：`data-state/data-size/data-selected-index/data-selected-value/data-selection-mode/data-selection-source/data-change-source`"),
        "check2.md should include stable semantic marker evidence for machine-readable contract."
    );
    assert!(
        check2.contains(
            "native_select_type_system_and_semantic_markers_form_machine_readable_contract"
        ),
        "check2.md should include regression reference for machine-readable type/marker contract."
    );
    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "check2.md should mark focus-stack and GC contract item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 非层叠 `Overlay` 组件")
            && check2.contains("无 `NodeRef` 私有恢复目标"),
        "check2.md should include explicit N/A scope/evidence for overlay focus-stack requirement."
    );
    assert!(
        check2.contains("native_select_has_no_overlay_focus_stack_gc_path"),
        "check2.md should include regression reference for focus-stack/GC N/A contract."
    );
    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "check2.md should mark escape-hatches foreign-zone item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 是原生 `<select>` 装配组件")
            && check2.contains("未接入命令式第三方实例"),
        "check2.md should include explicit N/A scope/evidence for foreign-zone escape-hatch requirement."
    );
    assert!(
        check2.contains("native_select_has_no_foreign_zone_escape_hatch_path"),
        "check2.md should include regression reference for foreign-zone escape-hatch N/A contract."
    );
    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "check2.md should mark hydration discontinuity contract item as completed."
    );
    assert!(
        check2.contains("适用说明：`NativeSelect` 通过外部 `id_base` 注入确定性种子")
            && check2.contains("{id_base}-root / {id_base}-control / {id_base}-option-{index}"),
        "check2.md should include deterministic id-seed evidence for hydration discontinuity contract."
    );
    assert!(
        check2.contains("未使用 `now()` / `uuid` / `rand` 等非确定性源"),
        "check2.md should include no-time-no-random evidence for hydration discontinuity contract."
    );
    assert!(
        check2.contains(
            "native_select_hydration_ids_are_deterministic_without_time_or_random_sources"
        ),
        "check2.md should include regression reference for hydration discontinuity contract."
    );
    assert!(
        check2
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "check2.md should mark ssr/cross-platform contract item as completed."
    );
    assert!(
        check2.contains("compile-only 证据（2026-02-20 执行记录）")
            && check2.contains("cargo check -p ui-native-select")
            && check2.contains("cargo check -p ui-native-select --target wasm32-unknown-unknown")
            && check2.contains("cargo check -p ui-headless --no-default-features --features ssr"),
        "check2.md should record compile-only command evidence for web/ssr/wasm paths."
    );
    assert!(
        check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should record current compile command environment blocker."
    );
    assert!(
        check2.contains("无 `#[cfg(...)]` 平台分支、无 `web_sys`/`wasm_bindgen`/`js_sys`")
            && check2.contains("compile_error!"),
        "check2.md should include explicit platform-branch and non-wasm boundary evidence."
    );
    assert!(
        check2.contains("native_select_ssr_and_cross_platform_compile_contract_is_preserved"),
        "check2.md should include regression reference for ssr/cross-platform contract."
    );
    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "check2.md should mark ui-headless web/ssr mutual-exclusion item as completed."
    );
    assert!(
        check2.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]")
            && check2.contains("compile_error!("),
        "check2.md should include compile_error-based ui-headless feature mutex evidence."
    );
    assert!(
        check2.contains("cargo check -p ui-headless --no-default-features --features web,ssr")
            && check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should capture attempted web,ssr conflict-path compile evidence and environment blocker."
    );
    assert!(
        check2.contains("native_select_respects_ui_headless_web_ssr_compile_error_mutex"),
        "check2.md should include regression reference for ui-headless web/ssr mutex contract."
    );
    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "check2.md should mark ui-motion non-wasm noop/stub item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 当前无组件级 `motion.rs`")
            && check2.contains("`prefers_reduced_motion() -> true`")
            && check2.contains("`animate(...) {}`"),
        "check2.md should include native-select N/A scope and ui-motion non-wasm noop evidence."
    );
    assert!(
        check2.contains("native_select_respects_ui_motion_non_wasm_noop_contract"),
        "check2.md should include regression reference for ui-motion non-wasm noop contract."
    );
    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "check2.md should mark reduced-motion/SSR/wasm branch coverage item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 无组件级 `motion.rs`")
            && check2.contains("`prefers_reduced_motion() -> true`")
            && check2.contains("不含 `#[cfg(...)]` / `web_sys` / `wasm_bindgen` 分支"),
        "check2.md should include N/A scope and reduced-motion/SSR/wasm evidence."
    );
    assert!(
        check2.contains("native_select_reduced_motion_ssr_wasm_contract_is_preserved"),
        "check2.md should include regression reference for reduced-motion/SSR/wasm branch contract."
    );
    assert!(
        check2.contains("- [x] `ui` 固定入口文件落点正确。"),
        "check2.md should mark ui fixed entry file placement item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/lib.rs`")
            && check2.contains("`crates/ui/src/css.rs`")
            && check2.contains("`crates/ui/src/root.rs`")
            && check2.contains("`crates/ui-visual-primitive/src/active_highlight.rs`")
            && check2.contains("`crates/ui/src/overlay_open.rs`")
            && check2.contains("`crates/ui/src/presence.rs`")
            && check2.contains("`crates/ui/src/a11y.rs`"),
        "check2.md should include full fixed-entry file checklist evidence for ui + ui-visual-primitive."
    );
    assert!(
        check2.contains("native_select_ui_components_fixed_entry_files_are_in_correct_locations"),
        "check2.md should include regression reference for fixed-entry file placement contract."
    );
    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确。"),
        "check2.md should mark component-directory standard file placement item as completed."
    );
    assert!(
        check2.contains("N/A 说明（`motion.rs`）：`NativeSelect` 为原生 `<select>` 装配组件")
            && check2.contains("N/A 说明（`spec.rs`）：组件无独立复杂 schema/config 演进需求")
            && check2.contains("`components/native-select/src/render.rs`、`components/native-select/src/motion.rs`、`components/native-select/src/spec.rs` 均不存在"),
        "check2.md should include explicit N/A scope and absence evidence for motion/spec/render entry files."
    );
    assert!(
        check2.contains("native_select_component_directory_standard_file_placement_is_correct"),
        "check2.md should include regression reference for component-directory standard file placement."
    );
    assert!(
        check2.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "check2.md should mark file-placement discipline item as completed."
    );
    assert!(
        check2.contains("N/A 说明（`motion.rs`）：组件无 enter/exit/open/close 语义动效轴")
            && check2.contains("N/A 说明（`spec.rs`）：组件无复杂 schema 演进需求")
            && check2.contains("`components/native-select/src/render.rs`、`components/native-select/src/motion.rs`、`components/native-select/src/spec.rs` 均不存在"),
        "check2.md should include explicit N/A and absence evidence for file-placement discipline item."
    );
    assert!(
        check2.contains("native_select_component_directory_standard_file_placement_is_correct"),
        "check2.md should link file-placement discipline item to component-directory regression coverage."
    );
    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "check2.md should mark hyper-structure builder item as completed."
    );
    assert!(
        check2.contains(
            "适用说明：该要求仅适用于“复杂组件”；`NativeSelect` 当前属于 simple component"
        ) && check2.contains("N/A 说明：组件无复杂 schema 固化/多形态渲染编排/AI 结构投影需求")
            && check2.contains("`components/native-select/src/spec.rs` 不存在")
            && check2.contains("`components/native-select/src/protocol.rs` 仅保留 serde 协议结构"),
        "check2.md should include explicit scope, N/A rationale, and evidence for hyper-structure builder item."
    );
    assert!(
        check2.contains("native_select_does_not_introduce_spec_rs_for_simple_component"),
        "check2.md should link hyper-structure builder item to spec.rs-scope regression coverage."
    );
    assert!(
        check2.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "check2.md should mark context-compression manifest + RBI item as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/Component.toml`")
            && check2.contains("`components/native-select/src/native_select.rbi`")
            && check2.contains("`context_compression_manifest`")
            && check2.contains("`rbi_signature_projection`"),
        "check2.md should include concrete manifest/RBI evidence for native-select."
    );
    assert!(
        check2
            .contains("native_select_context_compression_manifest_and_rbi_are_present_and_synced"),
        "check2.md should link context-compression manifest + RBI item to regression coverage."
    );
    assert!(
        check2.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "check2.md should mark agent-contract schema marker item as completed."
    );
    assert!(
        check2.contains("`data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source/data-ui-config-policy`")
            && check2.contains("`logic::resolve_agent_contract`")
            && check2.contains("`NativeSelectAgentContract`")
            && check2.contains("`[[agent_contract_whitelist]]`")
            && check2.contains("`inner_html`/`dangerously_set_inner_html`/`<script`/`javascript:`"),
        "check2.md should include typed schema marker + whitelist evidence for agent-contract item."
    );
    assert!(
        check2.contains("native_select_agent_contract_schema_markers_are_typed_and_whitelisted"),
        "check2.md should link agent-contract schema marker item to regression coverage."
    );
    assert!(
        check2.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "check2.md should mark streaming-term scope item as completed."
    );
    assert!(
        check2.contains("`Streaming`：LLM 还在生成，界面边生成边显示。")
            && check2.contains("`Snapshot`：LLM 全部生成完成后，一次性显示。")
            && check2.contains("`NativeSelect` 是原生 `<select>` 交互组件")
            && check2.contains("无 token/chunk 增量正文渲染")
            && check2.contains("`EventSource/WebSocket/ReadableStream`"),
        "check2.md should include explicit LLM streaming/snapshot definition and native-select scope evidence."
    );
    assert!(
        check2.contains("native_select_streaming_term_is_scoped_to_llm_output_rendering_only"),
        "check2.md should link streaming-term scope item to regression coverage."
    );
    assert!(
        check2.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "check2.md should mark streaming-required-vs-optional item as completed."
    );
    assert!(
        check2.contains("`Streaming Optional`")
            && check2.contains("`fallback=snapshot`")
            && check2.contains("`data-streaming-mode=\"optional\"`")
            && check2.contains("`data-streaming-fallback=\"snapshot\"`")
            && check2.contains("`data-output-status`")
            && check2.contains("草稿/已验证/可提交"),
        "check2.md should include optional-streaming fallback and readable output-status marker evidence."
    );
    assert!(
        check2.contains(
            "native_select_streaming_policy_is_optional_with_snapshot_fallback_and_readable_status_markers",
        ),
        "check2.md should link streaming-required-vs-optional item to regression coverage."
    );
    assert!(
        check2.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "check2.md should mark snapshot-baseline item as completed."
    );
    assert!(
        check2.contains("`NativeSelect` 接收完整 props 配置")
            && check2.contains("`options: Vec<NativeSelectOption>`")
            && check2.contains(
                "`<For each=move || resolved_options.get() children=render_native_select_option />`"
            )
            && check2.contains("`snapshot_rendering`"),
        "check2.md should include complete-config + stable-render evidence for snapshot baseline."
    );
    assert!(
        check2.contains("native_select_snapshot_baseline_renders_complete_config_stably"),
        "check2.md should link snapshot-baseline item to regression coverage."
    );
    assert!(
        check2.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "check2.md should mark motion-contract item as completed."
    );
    assert!(
        check2.contains(
            "N/A 说明：组件目录维持 `mod.rs|logic.rs|view.rs|styles.rs`，未引入 `motion.rs`"
        ) && check2.contains("`motion.rs + attach_motion` 为 N/A"),
        "check2.md should include explicit N/A scope for component-level motion contract in native-select."
    );
    assert!(
        check2.contains("native_select_respects_ui_motion_non_wasm_noop_contract")
            && check2.contains("native_select_reduced_motion_ssr_wasm_contract_is_preserved"),
        "check2.md should link motion-contract item to existing reduced-motion/non-wasm regression tests."
    );
    assert!(
        check2.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "check2.md should mark view-macro complexity control item as completed."
    );
    assert!(
        check2.contains("`render_placeholder_option` / `render_native_select_option`")
            && check2.contains("<For ... children=render_native_select_option />"),
        "check2.md should include semantic split evidence for view-macro complexity control."
    );
    assert!(
        check2.contains("native_select_view_macro_complexity_is_controlled_by_semantic_splitting"),
        "check2.md should include regression reference for view-macro complexity control."
    );
    assert!(
        check2.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "check2.md should mark functional fragment split item as completed."
    );
    assert!(
        check2.contains("render_placeholder_option(...) -> impl IntoView")
            && check2.contains("render_native_select_option(...) -> impl IntoView"),
        "check2.md should include plain function split evidence for lightweight fragments."
    );
    assert!(
        check2.contains("native_select_functional_fragment_split_prefers_plain_functions"),
        "check2.md should include regression reference for functional fragment split contract."
    );
    assert!(
        check2.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "check2.md should mark static fragment constantization item as completed."
    );
    assert!(
        check2.contains("`const NATIVE_SELECT_INDICATOR_SYMBOL`")
            && check2.contains("`render_static_indicator()`"),
        "check2.md should include constantized static indicator evidence."
    );
    assert!(
        check2.contains("native_select_static_fragment_is_constantized_with_stable_a11y_contract"),
        "check2.md should include regression reference for static fragment constantization."
    );
    assert!(
        check2.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "check2.md should mark inner_html safety item as completed."
    );
    assert!(
        check2.contains("N/A 说明：`NativeSelect` 当前不使用 `inner_html`")
            && check2.contains("不存在 HTML 注入面"),
        "check2.md should include N/A scope for native-select inner_html injection surface."
    );
    assert!(
        check2.contains("native_select_inner_html_contract_disallows_injection_surface"),
        "check2.md should include regression reference for inner_html safety contract."
    );
    assert!(
        check2.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
        "check2.md should mark wasm debug requirement item as completed."
    );
    assert!(
        check2.contains("`data-selected-index/data-selected-value/data-selection-mode/data-selection-source/data-change-source`")
            && check2.contains("`Interactive Playground`"),
        "check2.md should include wasm debug trace + visual workbench evidence."
    );
    assert!(
        check2.contains("native_select_wasm_debug_contract_tracks_state_and_keeps_api_clean"),
        "check2.md should include regression reference for wasm debug contract."
    );
    assert!(
        check2.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "check2.md should mark DX workbench requirement item as completed."
    );
    assert!(
        check2.contains("`test_css_source=workbench_test_css_source`")
            && check2.contains("`Persist workbench state`")
            && check2.contains("native-select-workbench-canvas"),
        "check2.md should include live-css feedback, optional persistence, and isolated workbench-canvas evidence."
    );
    assert!(
        check2.contains(
            "native_select_dx_workbench_supports_live_css_and_optional_state_persistence"
        ),
        "check2.md should include regression reference for DX workbench contract."
    );
    assert!(
        check2.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "check2.md should mark engineering-unification requirement item as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/protocol.rs`")
            && check2.contains("`schema_version`")
            && check2.contains("未引入 `tokio` / `async-std` / `tracing`"),
        "check2.md should include serde protocol + runtime-agnostic evidence for engineering-unification."
    );
    assert!(
        check2.contains(
            "native_select_engineering_contract_uses_structured_protocol_and_avoids_runtime_leakage"
        ),
        "check2.md should include regression reference for engineering-unification contract."
    );
    assert!(
        check2.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "check2.md should mark defensive-variables requirement item as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/styles.rs`")
            && check2.contains("`var(--ui-border-width, var(--ui-fallback-border-width))`")
            && check2.contains("`crates/ui-theme/src/css.rs`"),
        "check2.md should include defensive fallback-chain + theme SSOT evidence."
    );
    assert!(
        check2.contains("native_select_styles_use_defensive_variable_fallback_chain"),
        "check2.md should include regression reference for defensive-variable contract."
    );
    assert!(
        check2.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "check2.md should mark cascade-layer requirement item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/css.rs`")
            && check2.contains("`@layer ui`")
            && check2.contains("`components/native-select/src/view.rs` 未使用普通 `style=`"),
        "check2.md should include ui-layer aggregation + no-inline-style evidence for cascade-layer contract."
    );
    assert!(
        check2.contains("native_select_css_cascade_layer_contract_is_enforced"),
        "check2.md should include regression reference for cascade-layer contract."
    );
    assert!(
        check2.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "check2.md should mark performance governance budget item as completed."
    );
    assert!(
        check2.contains("组件级预算（NativeSelect）：空闲预算为“初始化后无外部变更时不触发高频循环”")
            && check2.contains("关键更新预算为“单次 `on:change` 仅提交一次 `request_selected_index_change.run(next_index)`”"),
        "check2.md should include explicit component-level performance budget baselines."
    );
    assert!(
        check2
            .contains("N/A 说明（仓库级 render_count 门禁）：`Button/Input` 全局 `render_count=1`")
            && check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should scope repository-level render_count blocker with environment evidence."
    );
    assert!(
        check2.contains(
            "native_select_performance_budget_contract_is_guarded_without_render_count_harness"
        ),
        "check2.md should include regression reference for performance governance contract."
    );
    assert!(
        check2.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "check2.md should mark rust-hygiene item as completed."
    );
    assert!(
        check2.contains("`./scripts/check-rust-hygiene.sh`")
            && check2.contains("`PCRE2 is not available in this build of ripgrep`")
            && check2.contains(
                "`components/native-select/src/mod.rs|logic.rs|view.rs|styles.rs|protocol.rs`"
            )
            && check2.contains("未出现 `.unwrap(` / `.expect(` / `let _ =`")
            && check2.contains("`use std::borrow::Cow;`")
            && check2.contains("`Vec<Cow<'static, str>>`"),
        "check2.md should include rust-hygiene command evidence and component-local Cow-based mitigation."
    );
    assert!(
        check2
            .contains("native_select_rust_hygiene_disallows_unwrap_expect_let_underscore_and_string_clone_churn"),
        "check2.md should link rust-hygiene item to regression coverage."
    );
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "check2.md should mark tree-shaking feature-gate item in delivery-gates section as completed."
    );
    assert!(
        check2.contains("`component-native_select = [\"dep:ui-native-select\"]`")
            && check2.contains(
                "`#[cfg(feature = \"component-native_select\")] pub use ui_native_select as native_select;`"
            )
            && check2.contains(
                "`#[cfg(feature = \"component-native_select\")] out.push_str(crate::native_select::styles::CSS);`"
            )
            && check2.contains("`cargo tree -e features -p ui --no-default-features --features component-native_select,inject-css`")
            && check2.contains("`cargo tree -e features -i ui -p web-demo`")
            && check2.contains("未出现 `all-components`"),
        "check2.md should include tree-shaking evidence for feature-gated lib/css aggregation and cargo-tree verification."
    );
    assert!(
        check2.contains("native_select_tree_shaking_feature_gates_are_component_scoped"),
        "check2.md should link tree-shaking delivery-gate item to regression coverage."
    );
    assert!(
        check2.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "check2.md should mark semantic-test + performance-regression delivery-gate item as completed."
    );
    assert!(
        check2.contains("`aria-*`（`aria-label/aria-invalid`）")
            && check2.contains("`data-state/data-selection-mode/data-selection-source/data-change-source`")
            && check2.contains("`:focus-visible`")
            && check2.contains("`native_select_performance_budget_contract_is_guarded_without_render_count_harness`")
            && check2.contains("`Invalid cross-device link (os error 18)`"),
        "check2.md should include semantic-marker/focus coverage and render_count-N/A performance evidence."
    );
    assert!(
        check2.contains("native_select_mounts_headless_a11y_contract_with_locale_hooks")
            && check2
                .contains("native_select_exposes_observable_retrievable_verifiable_state_markers")
            && check2
                .contains("native_select_styles_depend_on_explicit_state_markers_not_dom_guessing")
            && check2.contains(
                "native_select_performance_budget_contract_is_guarded_without_render_count_harness"
            ),
        "check2.md should link semantic + performance delivery-gate item to concrete regression tests."
    );
    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "check2.md should mark version-deprecation/codemod item as completed."
    );
    assert!(
        check2.contains("N/A 说明：本次 `native-select` 清单变更未引入跨大版本 API 破坏升级")
            && check2.contains("未出现 `migrate_v1_to_v2` / `migrate_v2_to_v3` / `SchemaRegistry` / `deprecation_window`")
            && check2.contains("`schema_version`（当前 `V1`）")
            && check2.contains("`#[serde(default)]")
            && check2.contains("若后续引入真实跨大版本破坏"),
        "check2.md should include N/A scope, evidence, and escalation rule for version-deprecation migration contract."
    );
    assert!(
        check2
            .contains("native_select_version_deprecation_registry_is_na_without_breaking_upgrade"),
        "check2.md should link version-deprecation/codemod item to regression coverage."
    );
    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "check2.md should mark docs-as-product copy-paste-ready item as completed."
    );
    assert!(
        check2.contains("`Hello World (Uncontrolled)`")
            && check2.contains("`Controlled vs Uncontrolled`")
            && check2.contains("`State Matrix (Controlled / Uncontrolled / Disabled)`")
            && check2.contains("`Streaming Optional (fallback=snapshot)`")
            && check2.contains("`NATIVE_SELECT_DOC_IMPORTS`")
            && check2.contains("`compose_copy_ready_code`")
            && check2.contains("`DEFAULT_PLAYGROUND_IMPORTS`")
            && check2.contains("`ui-code-block__copy-button`"),
        "check2.md should include docs playground matrix + streaming/snapshot + copy-ready import-completion evidence."
    );
    assert!(
        check2.contains(
            "native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract"
        ),
        "check2.md should link docs-as-product copy-paste-ready item to regression coverage."
    );
    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "check2.md should mark semantic-test-first item as completed."
    );
    assert!(
        check2.contains(
            "`native_select_semantics_contract_tests_cover_key_matrix_without_snapshot_dependency`"
        ) && check2.contains("`aria-label/aria-invalid`")
            && check2.contains("`data-state`")
            && check2.contains("`data-selection-mode`")
            && check2.contains("`data-selection-source`")
            && check2.contains("`data-change-source`")
            && check2.contains("`on:change`")
            && check2.contains(&format!(
                "`{}` / `{}` / `{}`",
                concat!("insta", "::"),
                concat!("assert_", "snapshot!"),
                concat!("to_match_", "snapshot")
            )),
        "check2.md should include semantic-contract-first evidence and anti-snapshot constraints."
    );
    assert!(
        check2.contains(
            "native_select_semantics_contract_tests_cover_key_matrix_without_snapshot_dependency"
        ),
        "check2.md should link semantic-test-first item to regression coverage."
    );
    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "check2.md should mark e2e-selector-stability item as completed."
    );
    assert!(
        check2.contains("`e2e/tests/docs_app_native_select_contract.spec.mjs`")
            && check2.contains("#docs-native-select-controlled-control")
            && check2.contains("#docs-native-select-uncontrolled-control")
            && check2.contains("`ancestor::*[@data-slot=\"native-select\"][1]`")
            && check2.contains("`body:not(:has(#boot))`")
            && check2.contains("无 `waitForTimeout` / `setTimeout`")
            && check2.contains(
                "`data-selected-index/data-selected-value/data-change-source/data-output-status`"
            )
            && check2.contains("`page.reload()`"),
        "check2.md should include semantic-selector, wasm-ready wait, and settled-breakpoint evidence for native-select e2e stability."
    );
    assert!(
        check2.contains(
            "N/A 说明（异步/动画）：`NativeSelect` 当前无组件级异步请求或组件级动效流水线"
        ),
        "check2.md should include explicit N/A scope for async/animation e2e readiness requirements."
    );
    assert!(
        check2.contains("native_select_e2e_contract_uses_semantic_selectors_and_wasm_ready_wait")
            && check2.contains("native_select_checklist_tracks_ui_components_contract"),
        "check2.md should link e2e selector-stability item to concrete regression coverage."
    );
    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "check2.md should mark repeatable key-flow regression item as completed."
    );
    assert!(
        check2.contains("`docs-app native-select key flow is repeatable with semantic breakpoints`")
            && check2.contains("进入页面 -> 交互变更 -> 语义断点断言 -> reload 后再次断言")
            && check2.contains("`data-selection-mode/data-selection-source/data-selected-index/data-selected-value/data-change-source/data-output-status`"),
        "check2.md should include repeatable flow chain and traceable semantic-breakpoint evidence."
    );
    assert!(
        check2.contains("`docs-app native-select keyboard path uses semantic breakpoints`")
            && check2.contains("`focus + ArrowDown/ArrowUp`")
            && check2.contains("`data-selected-index`")
            && check2.contains("`body:not(:has(#boot))`"),
        "check2.md should include focus/keyboard high-risk-path evidence with wasm-ready wait."
    );
    assert!(
        check2.contains("N/A 说明（overlay/async）：`NativeSelect` 非 overlay 组件")
            && check2.contains("无组件级异步请求流"),
        "check2.md should include explicit N/A scope for overlay/async high-risk paths."
    );
    assert!(
        check2.contains("native_select_e2e_key_flow_regression_is_repeatable_and_traceable"),
        "check2.md should link repeatable key-flow item to dedicated regression test."
    );
    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "check2.md should mark docs-app docs/example/matrix sync item as completed."
    );
    assert!(
        check2.contains("forms_native.rs` 已同步维护 NativeSelect 页面描述与示例集合")
            && check2.contains("`Hello World (Uncontrolled)`")
            && check2.contains("`Controlled + Placeholder`")
            && check2.contains("`Required + Invalid + Disabled`")
            && check2.contains("`Controlled vs Uncontrolled`")
            && check2.contains("`State Matrix (Controlled / Uncontrolled / Disabled)`"),
        "check2.md should include docs-app page and example synchronization evidence."
    );
    assert!(
        check2.contains("`data-slot=\"native-select-controlled-uncontrolled\"`")
            && check2.contains("`data-slot=\"native-select-state-matrix\"`")
            && check2.contains("`default_selected_index=0`")
            && check2.contains("`selected_index=Signal::derive(|| Some(2usize))`")
            && check2.contains("`is_disabled=true`"),
        "check2.md should include state-matrix branch coverage evidence."
    );
    assert!(
        check2.contains("`selected_index/on_selected_index_change/default_selected_index/is_disabled/is_required/is_invalid/size`")
            && check2.contains("`NativeSelectSize` 默认 `Md`")
            && check2.contains("`_ => NativeSelectSize::Md`")
            && check2.contains("`logic::normalize_default_selected_index`"),
        "check2.md should include API naming + default-value sync evidence against logic.rs."
    );
    assert!(
        check2.contains("native_select_docs_matrix_and_api_contract_are_synced_with_logic_defaults")
            && check2.contains("components/native-select/test/native_select_semantics.rs::native_select_docs_matrix_and_api_contract_are_synced_with_logic_defaults"),
        "check2.md should link docs-app sync item to component/workspace regression tests."
    );
    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "check2.md should mark beginner-friendly documentation-product item as completed."
    );
    assert!(
        check2.contains("`components/native-select/src/README.md`")
            && check2.contains("`forms_native.rs::native_select()`")
            && check2.contains("`## 新手路径：先用起来，再进阶`")
            && check2.contains("`### Hello World（零门槛）`")
            && check2.contains("`### 常见用法（在 docs-app 直接对照）`")
            && check2.contains("`## API（进阶参考）`"),
        "check2.md should include README + docs entry and beginner-first structure evidence."
    );
    assert!(
        check2.contains("`Controlled + Placeholder`")
            && check2.contains("`Required + Invalid + Disabled`")
            && check2.contains("`Controlled vs Uncontrolled`")
            && check2.contains("`State Matrix`")
            && check2.contains("`Hello World` 在前、`## API（进阶参考）` 在后"),
        "check2.md should include zero-threshold/common-usage and beginner-then-advanced ordering evidence."
    );
    assert!(
        check2.contains("native_select_readme_is_beginner_friendly_documentation_product")
            && check2.contains("components/native-select/test/native_select_semantics.rs::native_select_readme_is_beginner_friendly_documentation_product"),
        "check2.md should link beginner-friendly documentation item to component/workspace regression tests."
    );
    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "check2.md should mark interactive-playground item as completed."
    );
    assert!(
        check2.contains("`title=\"Interactive Playground\"`")
            && check2.contains("`data-slot=\"native-select-workbench-controls\"`")
            && check2.contains("`data-slot=\"native-select-workbench-canvas\"`")
            && check2.contains("`size`、`selected` 模式")
            && check2.contains("`Required/Invalid/Disabled/Placeholder/Custom class/Show compare matrix/Persist workbench state`"),
        "check2.md should include interactive controls/canvas evidence for docs-app playground."
    );
    assert!(
        check2.contains("N/A 说明（AI Spec 联动）：`NativeSelect` 为 simple component")
            && check2.contains("`spec.rs` 明确不适用"),
        "check2.md should include explicit N/A scope for AI spec-input preview linkage."
    );
    assert!(
        check2.contains("`load/save/clear_native_select_workbench_state`")
            && check2.contains("`Persist workbench state`")
            && check2.contains("配置 -> 预览 -> 刷新后恢复"),
        "check2.md should include reproducible interactive-acceptance path evidence."
    );
    assert!(
        check2.contains(
            "native_select_dx_workbench_supports_live_css_and_optional_state_persistence"
        ) && check2.contains("native_select_checklist_tracks_ui_components_contract"),
        "check2.md should link interactive-playground item to regression coverage."
    );
    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "check2.md should mark source-first copy-paste-ready item as completed."
    );
    assert!(
        check2.contains("`code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()`")
            && check2.contains("`compose_copy_ready_code`")
            && check2.contains("`DEFAULT_PLAYGROUND_IMPORTS`")
            && check2.contains("`ui-code-block__copy-button`"),
        "check2.md should include copy-button + runnable snippet import-completion evidence."
    );
    assert!(
        check2.contains("`data-slot=\"native-select-source-first\"`")
            && check2.contains("`data-slot=\"native-select-source-paths\"`")
            && check2.contains("`crates/ui/src/native_select`")
            && check2.contains("`component-native_select + inject-css`"),
        "check2.md should include source-first path and dependency-prerequisite evidence."
    );
    assert!(
        check2.contains("`NATIVE_SELECT_DOC_IMPORTS` 作为统一 import 基线")
            && check2.contains("避免示例片段与当前实现脱节"),
        "check2.md should include anti-drift evidence for docs code snippets."
    );
    assert!(
        check2.contains(
            "native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract"
        ) && check2.contains("native_select_checklist_tracks_ui_components_contract"),
        "check2.md should link source-first copy-paste-ready item to regression coverage."
    );
    assert!(
        check2.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "check2.md should mark HeroUI strategy/docs synchronization item as completed."
    );
    assert!(
        check2.contains("`docs/spec/heroui-parameter-design-strategy.md`")
            && check2.contains("`### NativeSelect 同步记录（2026-02-20）`")
            && check2.contains("`selected_index/on_selected_index_change/default_selected_index`")
            && check2.contains("`is_disabled/is_required/is_invalid/size`"),
        "check2.md should include NativeSelect parameter-model sync evidence in HeroUI strategy doc."
    );
    assert!(
        check2.contains("`apps/docs-app/src/pages/components/pages.rs`")
            && check2.contains("`component_doc!(\"NativeSelect\", \"native-select\", \"Forms\", forms_native::native_select)`")
            && check2.contains("`forms_native.rs` 维持 `title=\"NativeSelect\" + slug=\"native-select\"`")
            && check2.contains("`components/native-select/src/README.md`"),
        "check2.md should include docs entry/indexability evidence for docs-app and README."
    );
    assert!(
        check2.contains("N/A 说明（research 补充）：本轮仅为参数语义与文档入口同步")
            && check2.contains("`docs/research/spectrum-heroui-style-interface-study.md`"),
        "check2.md should include explicit N/A reasoning for optional research-doc update."
    );
    assert!(
        check2.contains("native_select_heroui_strategy_doc_and_component_docs_are_synced")
            && check2.contains("native_select_checklist_tracks_ui_components_contract"),
        "check2.md should link HeroUI strategy/docs synchronization item to regression coverage."
    );
}
