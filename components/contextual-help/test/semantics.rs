fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "protocol" => include_str!("../src/protocol.rs"),
        "protocol_test" => include_str!("protocol.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "view" => include_str!("../src/view.rs"),
        "legacy_semantics" => {
            include_str!("../../../components/contextual-help/test/contextual_help_semantics.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn contextual_help_public_api_keeps_platform_details_internal() {
    let module = load_source("mod");

    for needle in [
        "pub use logic::ContextualHelpVariant;",
        "pub use motion::ContextualHelpMotion;",
        "pub use view::ContextualHelp;",
    ] {
        assert!(
            module.contains(needle),
            "ContextualHelp module should export `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys",
        "NodeRef",
        "HtmlElement",
        "pub mod view",
        "pub mod logic",
    ] {
        assert!(
            !module.contains(forbidden),
            "ContextualHelp public module should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_layer_files_follow_component_boundaries() {
    for required in [
        "components/contextual-help/src/mod.rs",
        "components/contextual-help/src/logic.rs",
        "components/contextual-help/src/styles.rs",
        "components/contextual-help/src/view.rs",
        "components/contextual-help/src/motion.rs",
    ] {
        assert!(
            std::path::Path::new(required).exists(),
            "required component file is missing: `{required}`.",
        );
    }
    for forbidden in [
        "components/contextual-help/src/render.rs",
        "components/contextual-help/src/spec.rs",
    ] {
        assert!(
            !std::path::Path::new(forbidden).exists(),
            "for this component, forbidden/unused file should not exist: `{forbidden}`.",
        );
    }

    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for needle in [
        "pub struct ContextualHelpStateInput",
        "pub struct ContextualHelpState",
        "pub struct ContextualHelpOpenStateInput",
        "pub struct ContextualHelpOpenStateConfig",
        "use ui_state_primitives::contextual_help as contextual_help_state;",
        "pub use contextual_help_state::{",
        "pub fn resolve_state(input: ContextualHelpStateInput) -> ContextualHelpState",
        "pub fn resolve_open_state_config(",
        "contextual_help_state::resolve_open_config(",
        "pub fn resolve_generated_id(provider_generated_id: Option<String>) -> String",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ContextualHelpState) -> String",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should own state normalization marker `{needle}`.",
        );
    }

    for forbidden in ["view!", "NodeRef<", "web_sys::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should stay pure and avoid UI/runtime detail `{forbidden}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state_traced(",
        "use_presence(open)",
        "ui_headless::aria_controls_when_open(open, panel_id.get_value())",
        "ui_headless::overlay_dialog_attrs(",
        "logic::resolve_state(ContextualHelpStateInput {",
        "data-slot=\"contextual-help\"",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should mount headless contracts via `{needle}`.",
        );
    }

    assert!(
        !view.contains("unwrap_or_else(|| \"ui-contextual-help-0\".to_string())"),
        "view.rs should not do fallback defaults directly; defaulting must stay in logic.rs.",
    );

    assert!(
        !view.contains("ui_state_primitives::"),
        "view.rs should not reimplement or directly wire status primitives.",
    );

    assert!(
        styles.contains("var(--ui-"),
        "styles.rs should remain token-first and consume ui-theme variables.",
    );
    assert!(
        !styles.contains("on:click"),
        "styles.rs must not contain interaction logic.",
    );

    for needle in [
        "pub struct ContextualHelpMotion",
        "pub fn sanitize_motion(motion: ContextualHelpMotion) -> ContextualHelpMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion.contains(needle),
            "motion.rs should map contract values via `{needle}`.",
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "requestAnimationFrame",
        "aria-",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not embed engine/a11y detail `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_hyper_structure_builder_is_na_for_non_complex_component() {
    let mod_rs = load_source("mod");
    let protocol = load_source("protocol");
    let readme = include_str!("../src/README.md");

    assert!(
        !std::path::Path::new("components/contextual-help/src/spec.rs").exists(),
        "non-complex component should not carry `spec.rs` by default.",
    );

    for forbidden in [
        "pub use spec::",
        "pub mod spec;",
        "ContextualHelpSpec::new(",
        ".render()",
    ] {
        assert!(
            !mod_rs.contains(forbidden),
            "public module should not expose Hyper-Structure builder entry `{forbidden}` for non-complex component.",
        );
    }

    for needle in ["schema_version", "Serialize", "Deserialize"] {
        assert!(
            protocol.contains(needle),
            "non-builder path should still keep machine-readable protocol contract marker `{needle}`.",
        );
    }

    for needle in [
        "Hello World（最小可用）",
        "<ContextualHelp",
        "受控模式使用 `open + on_open_change`",
    ] {
        assert!(
            readme.contains(needle),
            "README should keep direct composition usage marker `{needle}` for simple component path.",
        );
    }
}

#[test]
fn contextual_help_readme_is_beginner_friendly_and_progressive() {
    let readme = include_str!("../src/README.md");
    let check2 = include_str!("../check2.md");

    for needle in [
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 再进阶（受控 + 语义定制）",
        "先用起来：先走默认 API，确认语义与交互正常后再进入受控扩展。",
        "默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。",
        "受控模式使用 `open + on_open_change`",
        "`apps/docs-app/src/pages/components/pages/overlays.rs` 的 `contextual_help()` 页面。",
        "`Hello World (Default API)`",
        "`State Matrix`",
    ] {
        assert!(
            readme.contains(needle),
            "README should keep beginner-friendly marker `{needle}`.",
        );
    }

    let hello = readme
        .find("## Hello World（最小可用）")
        .expect("README should include Hello World section.");
    let common = readme
        .find("## 常见用法")
        .expect("README should include common usage section.");
    let advanced = readme
        .find("## 再进阶（受控 + 语义定制）")
        .expect("README should include advanced section.");

    assert!(
        hello < common && common < advanced,
        "README should keep progressive order: hello-world -> common usage -> advanced.",
    );

    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "check2 should mark beginner-friendly documentation item complete.",
    );
}

#[test]
fn contextual_help_manifest_and_rbi_are_kept_in_sync_for_ai_context_compression() {
    let manifest = include_str!("../src/Component.toml");
    let rbi = include_str!("../src/contextual_help.rbi");

    for needle in [
        "schema_version = \"1\"",
        "name = \"ContextualHelp\"",
        "crate = \"ui-contextual-help\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"open\"",
        "controlled = true",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"ui-headless\"",
        "name = \"ui-state-primitives\"",
        "name = \"serde\"",
    ] {
        assert!(
            manifest.contains(needle),
            "manifest contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub const COMPONENT_ID: &str;",
        "pub enum ContextualHelpVariant {",
        "pub struct ContextualHelpMotion {",
        "pub fn ContextualHelp(",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi.contains(needle),
            "rbi projection should keep signature marker `{needle}`.",
        );
    }

    for forbidden in ["ButtonSpec", "Accordion", "render.rs"] {
        assert!(
            !manifest.contains(forbidden) && !rbi.contains(forbidden),
            "manifest/rbi should avoid stale copy marker `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_has_component_level_semantics_regression_entry() {
    let legacy = load_source("legacy_semantics");
    let module = load_source("mod");

    assert!(
        module.contains("[path = \"../test/semantics.rs\"]"),
        "component-level semantics test should be wired from mod.rs.",
    );
    assert!(
        legacy.contains("fn contextual_help_styles_include_state_marker_contracts()"),
        "legacy ui semantics coverage should remain present during migration.",
    );
}

#[test]
fn contextual_help_api_naming_prefers_is_on_default_contract() {
    let view = load_source("view");
    let logic = load_source("logic");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp API should expose naming contract marker `{needle}`.",
        );
    }

    assert!(
        logic.contains(
            "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool"
        ),
        "logic.rs should centralize disabled naming migration and precedence.",
    );
}

#[test]
fn contextual_help_open_state_contract_is_fully_paired_and_not_semi_controlled() {
    let view = load_source("view");
    let headless_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let headless_state_tests =
        include_str!("../../../crates/ui-headless/src/test/controllable_state.rs");
    let logic = load_source("logic");

    for needle in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let open_state_config = logic::resolve_open_state_config(ContextualHelpOpenStateInput {",
        "let has_custom_open = open_state_config.has_custom_open;",
        "let is_controlled = open_state_config.is_controlled;",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"contextual-help\",",
        "open_state_config.open,",
        "open_state_config.default_open,",
        "open_state_config.on_open_change,",
        "let open = open_state.open;",
        "let request_open_change = open_state.request_open_change;",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp should keep full controlled/uncontrolled pair contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_open_state_config(",
        "contextual_help_state::resolve_open_config(",
        "default_open: primitive.default_open,",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should own explicit default-open priority rule `{needle}`.",
        );
    }

    for needle in [
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
        "fn controlled_open_does_not_update_internal_state()",
        "fn controlled_open_ignores_default_open_value()",
    ] {
        assert!(
            headless_state.contains(needle) || headless_state_tests.contains(needle),
            "Headless controllable primitive should enforce single source of truth contract `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_state_contract_is_type_bounded_and_machine_readable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let logic_tests = include_str!("logic.rs");

    for needle in [
        "pub enum ContextualHelpVariant",
        "pub struct ContextualHelpStateInput",
        "pub variant: ContextualHelpVariant,",
        "pub placement: PopoverPlacement,",
        "pub fn resolve_state(input: ContextualHelpStateInput) -> ContextualHelpState",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep key state axis type-bounded via `{needle}`.",
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-placement=state.placement_attr",
        "data-open-mode=state.open_mode_attr",
        "data-open-source=state.open_source_attr",
        "data-default-open-source=state.default_open_source_attr",
        "data-open-change-source=state.open_change_source_attr",
        "data-open-interaction-source=move || open_interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should expose stable machine-readable markers `{needle}`.",
        );
    }

    for needle in [
        "fn resolve_state_tracks_flags_and_attrs()",
        "fn resolve_open_state_config_makes_default_open_single_source_and_explicit()",
        "assert_eq!(state.open_mode_attr, \"controlled\");",
        "assert_eq!(state.default_open_source_attr, \"provided\");",
        "assert_eq!(state.open_change_source_attr, \"provided\");",
    ] {
        assert!(
            logic_tests.contains(needle),
            "logic tests should directly pinpoint state-contract regressions via `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_agent_contract_is_schema_driven_and_whitelisted() {
    let logic = load_source("logic");
    let view = load_source("view");
    let manifest = include_str!("../src/Component.toml");
    let rbi = include_str!("../src/contextual_help.rbi");

    for needle in [
        "pub const CONTEXTUAL_HELP_AGENT_SCHEMA: &str = \"ui.contextual-help.v1\";",
        "pub enum ContextualHelpAgentIntent",
        "pub enum ContextualHelpAgentAction",
        "pub enum ContextualHelpAgentState",
        "pub struct ContextualHelpAgentContract",
        "pub fn resolve_agent_contract(",
        "ContextualHelpOpenInteractionSource::TriggerPress => ContextualHelpAgentAction::ToggleOpen",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep typed agent schema marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = move || {",
        "logic::resolve_agent_contract(state.variant, open_interaction_source.get(), open.get())",
        "data-ui-schema=move || agent_contract().schema",
        "data-ui-intent=move || agent_contract().intent",
        "data-ui-action=move || agent_contract().action",
        "data-ui-state=move || agent_contract().state",
        "data-ui-source=move || agent_contract().source",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should mount agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_schema_contract\"",
        "name = \"semantic-markers\"",
    ] {
        assert!(
            manifest.contains(needle),
            "manifest should expose agent marker capability `{needle}`.",
        );
    }

    for needle in [
        "pub const CONTEXTUAL_HELP_AGENT_SCHEMA: &str;",
        "pub enum ContextualHelpAgentIntent {",
        "pub enum ContextualHelpAgentAction {",
        "pub enum ContextualHelpAgentState {",
        "pub struct ContextualHelpAgentContract {",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            rbi.contains(needle),
            "rbi should project typed agent contract signature `{needle}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=move || format!(",
        "data-ui-intent=format!(",
        "schema_json",
        "inner_html=",
        "<script",
        "eval(",
    ] {
        assert!(
            !view.contains(forbidden),
            "agent contract render path should stay whitelist-only and reject `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_llm_output_render_mode_contract_is_streaming_or_snapshot_only() {
    let logic = load_source("logic");
    let view = load_source("view");
    let manifest = include_str!("../src/Component.toml");
    let rbi = include_str!("../src/contextual_help.rbi");

    for needle in [
        "pub enum ContextualHelpLlmOutputMode",
        "Streaming",
        "Snapshot",
        "pub const CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE",
        "pub fn resolve_llm_output_mode(is_streaming: bool) -> ContextualHelpLlmOutputMode",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep llm output mode marker `{needle}`.",
        );
    }

    for needle in [
        "let llm_output_mode = logic::resolve_llm_output_mode(false);",
        "data-ui-output-mode=llm_output_mode.as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should mount explicit llm output mode marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"llm-output-mode\"",
        "ty = \"data-ui-output-mode=snapshot|streaming\"",
        "name = \"llm_output_snapshot_mode\"",
        "enabled = true",
        "name = \"llm_output_streaming_mode\"",
        "enabled = false",
    ] {
        assert!(
            manifest.contains(needle),
            "Component manifest should keep llm output mode contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum ContextualHelpLlmOutputMode {",
        "pub const CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE: ContextualHelpLlmOutputMode;",
        "pub fn resolve_llm_output_mode(is_streaming: bool) -> ContextualHelpLlmOutputMode;",
    ] {
        assert!(
            rbi.contains(needle),
            "RBI should project llm output mode contract marker `{needle}`.",
        );
    }

    for forbidden in [
        "data-ui-output-mode=move || format!(",
        "LLM chunk",
        "token stream",
    ] {
        assert!(
            !view.contains(forbidden),
            "render path should avoid ad-hoc output mode marker `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_streaming_requirement_is_optional_with_snapshot_fallback_and_status_marker() {
    let logic = load_source("logic");
    let view = load_source("view");
    let manifest = include_str!("../src/Component.toml");
    let rbi = include_str!("../src/contextual_help.rbi");

    for needle in [
        "pub enum ContextualHelpStreamingRequirement",
        "Required",
        "Optional",
        "pub struct ContextualHelpStreamingPolicy",
        "pub enum ContextualHelpLlmOutputStatus",
        "Draft",
        "Verified",
        "Submittable",
        "pub fn resolve_streaming_policy(is_reader_surface: bool) -> ContextualHelpStreamingPolicy",
        "pub fn resolve_llm_output_status(",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep streaming requirement + status marker `{needle}`.",
        );
    }

    for needle in [
        "let streaming_policy = logic::resolve_streaming_policy(false);",
        "let llm_output_status = logic::resolve_llm_output_status(llm_output_mode);",
        "data-ui-output-status=llm_output_status.as_attr()",
        "data-ui-streaming-requirement=streaming_policy.requirement.as_attr()",
        "data-ui-streaming-fallback=streaming_policy.fallback_mode.as_attr()",
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "data-ui-output-mode=llm_output_mode.as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should keep readable streaming/status/a11y marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"streaming-policy\"",
        "name = \"llm-output-status\"",
        "data-ui-output-status=draft|verified|submittable",
        "name = \"llm_output_snapshot_mode\"",
        "name = \"llm_output_streaming_mode\"",
        "enabled = false",
        "name = \"llm_output_status_marker\"",
        "name = \"llm_streaming_optional_fallback_snapshot\"",
    ] {
        assert!(
            manifest.contains(needle),
            "manifest should keep streaming optional contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum ContextualHelpStreamingRequirement {",
        "pub struct ContextualHelpStreamingPolicy {",
        "pub enum ContextualHelpLlmOutputStatus {",
        "pub fn resolve_streaming_policy(is_reader_surface: bool) -> ContextualHelpStreamingPolicy;",
        "pub fn resolve_llm_output_status(output_mode: ContextualHelpLlmOutputMode)",
    ] {
        assert!(
            rbi.contains(needle),
            "rbi should project streaming optional/status signature `{needle}`.",
        );
    }

    for forbidden in ["retry", "backoff", "fetch(", "reqwest::", "tokio::spawn"] {
        assert!(
            !view.contains(forbidden),
            "component view should not own upstream validation/retry concerns `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_rust_hygiene_contract_for_non_test_sources() {
    let mod_rs = load_source("mod");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let view = load_source("view");

    for source in [mod_rs, logic, motion, styles, view] {
        for forbidden in [
            ".unwrap(",
            ".unwrap_err(",
            ".expect(",
            "let _ =",
            ".to_string(",
            "String::from(",
            ".to_owned(",
        ] {
            assert!(
                !source.contains(forbidden),
                "non-test component source should keep rust hygiene marker `{forbidden}` out.",
            );
        }
    }
}

#[test]
fn contextual_help_focus_restore_delegates_to_headless_focus_manager() {
    let contextual_help_view = load_source("view");
    let popover_view = include_str!("../../popover/src/view.rs");
    let focus_trap = include_str!("../../../crates/ui-headless/src/focus_trap.rs");

    assert!(
        contextual_help_view.contains("let anchor_ref: NodeRef<html::Button> = NodeRef::new();"),
        "ContextualHelp should keep trigger NodeRef only for anchoring/positioning.",
    );
    assert!(
        contextual_help_view.contains("anchor_ref=anchor_ref"),
        "ContextualHelp should pass anchor_ref to Popover positioning contract.",
    );
    assert!(
        !contextual_help_view.contains("with_restore_policy("),
        "ContextualHelp must not set local focus-restore policy with private NodeRef.",
    );
    assert!(
        !contextual_help_view.contains(".focus()"),
        "ContextualHelp should not manually force focus restoration in component view.",
    );

    for needle in [
        "use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        "focus_trap.on_key_down.run((key.clone(), ev.shift_key()))",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should mount shared focus-trap contract `{needle}`.",
        );
    }

    for needle in [
        "thread_local! {",
        "FOCUS_MANAGER_STACK",
        "focus_manager_push_trap",
        "focus_manager_pop_trap",
        "RestorePolicy::FallbackTo",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap.contains(needle),
            "ui-headless focus manager should keep layered restore contract `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_id_generation_is_deterministic_for_ssr_hydration() {
    let view = load_source("view");
    let logic = load_source("logic");
    let logic_tests = include_str!("logic.rs");

    for needle in [
        "use_ui_id_provider().map(|provider| provider.next_prefixed_id(\"ui-contextual-help\"))",
        "let generated_id = logic::resolve_generated_id(",
        "let (id, has_custom_id) = logic::resolve_id(id, generated_id);",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should derive ids from injected id provider contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_generated_id(provider_generated_id: Option<String>) -> String",
        "provider_generated_id.unwrap_or_else(|| \"ui-contextual-help-0\".into())",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep deterministic id fallback contract `{needle}`.",
        );
    }

    for forbidden in [
        "SystemTime::now()",
        "Instant::now()",
        "Uuid::",
        "rand::",
        "js_sys::Date",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "ContextualHelp id path must not depend on nondeterministic source `{forbidden}`.",
        );
    }

    assert!(
        logic_tests.contains("fn resolve_generated_id_uses_provider_value_or_stable_fallback()"),
        "logic tests should keep explicit regression coverage for deterministic id resolution.",
    );
}

#[test]
fn contextual_help_platform_contract_keeps_ssr_wasm_boundaries_explicit() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let headless_lib = include_str!("../../../crates/ui-headless/src/lib.rs");
    let headless_focus_trap = include_str!("../../../crates/ui-headless/src/focus_trap.rs");

    assert!(
        headless_lib.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]")
            && headless_lib.contains(
                "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
            ),
        "ui-headless should keep explicit web/ssr mutual-exclusion compile guard.",
    );

    for needle in [
        "#[cfg(all(feature = \"web\", target_arch = \"wasm32\"))]",
        "#[cfg(not(all(feature = \"web\", target_arch = \"wasm32\")))]",
        "fn setup_focus_trap(_options: FocusTrapOptions) -> FocusTrapHandlers",
    ] {
        assert!(
            headless_focus_trap.contains(needle),
            "focus_trap should expose explicit wasm/non-wasm split contract `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen",
        "window()",
        "document()",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden),
            "ContextualHelp component layer should avoid browser-only api `{forbidden}` in non-wasm path.",
        );
    }
}

#[test]
fn contextual_help_keeps_ui_headless_web_ssr_mutex_contract() {
    let view = load_source("view");
    let headless_lib = include_str!("../../../crates/ui-headless/src/lib.rs");
    let platform_script = include_str!("../../../scripts/check-ui-platforms.sh");

    assert!(
        view.contains("use ui_headless as overlay_open;"),
        "ContextualHelp should keep consuming ui-headless contract from component layer.",
    );

    assert!(
        headless_lib.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]")
            && headless_lib.contains(
                "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
            ),
        "ui-headless must keep compile_error guard for web+ssr mutual exclusion.",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "mutually exclusive",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform gate script should keep mutex verification path `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_motion_contract_keeps_non_wasm_noop_stub_predictable() {
    let contextual_help_view = load_source("view");
    let popover_view = include_str!("../../popover/src/view.rs");
    let popover_motion = include_str!("../../popover/src/motion.rs");
    let ui_motion_lib = include_str!("../../../crates/ui-motion/src/lib.rs");
    let ui_motion_stub_tests = include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should keep non-wasm stub contract `{needle}`.",
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_stub_tests.contains(needle),
            "ui-motion should keep non-wasm stub regression tests `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(needle),
            "Popover motion should keep non-wasm no-op downgrade path `{needle}`.",
        );
    }

    assert!(
        popover_view.contains("motion::attach_motion(")
            && contextual_help_view.contains("motion=motion.popover"),
        "ContextualHelp -> Popover motion wiring should remain contract-based without runtime handles.",
    );
}

#[test]
fn contextual_help_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let contextual_help_view = load_source("view");
    let popover_motion = include_str!("../../popover/src/motion.rs");
    let ui_motion_spring = include_str!("../../../crates/ui-motion/src/spring.rs");
    let ui_motion_spring_tests = include_str!("../../../crates/ui-motion/tests/spring.rs");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should keep reduced-motion immediate-settle contract `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_tests.contains(needle),
            "ui-motion should keep reduced-motion regression test `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "scale.set_on_rest(move || on_exit_complete.run(()));",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(needle),
            "Popover motion should keep wasm/non-wasm convergence contract `{needle}`.",
        );
    }

    for needle in [
        "let presence = use_presence(open);",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
    ] {
        assert!(
            contextual_help_view.contains(needle),
            "ContextualHelp should keep stable open/closed semantics for SSR-hydration `{needle}`.",
        );
    }

    assert!(
        !contextual_help_view.contains("#[cfg("),
        "ContextualHelp view semantics should not split by platform cfg at component layer.",
    );
}

#[test]
fn contextual_help_performance_governance_has_budgeted_equivalent_evidence() {
    let view = load_source("view");
    let docs_shell = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let perf_script = include_str!("../../../scripts/check-ui-performance.sh");
    let accordion_semantics =
        include_str!("../../../components/accordion/test/accordion_semantics.rs");

    for needle in [
        "use crate::perf_probe::{UiPerfBudget, UiPerfProbe};",
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            docs_shell.contains(needle),
            "docs shell should keep repeatable perf budget/probe baseline via `{needle}`.",
        );
    }

    for needle in [
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(needle),
            "performance gate script should keep blocking contract `{needle}`.",
        );
    }

    for needle in [
        "data-open-source=state.open_source_attr",
        "data-open-change-source=state.open_change_source_attr",
        "data-motion-source=state.motion_source_attr",
        "overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp should expose traceable attribution marker `{needle}`.",
        );
    }

    for forbidden in [
        "requestAnimationFrame",
        "setInterval",
        "set_timeout",
        "performance.now(",
    ] {
        assert!(
            !view.contains(forbidden),
            "ContextualHelp view should avoid local perf-unbounded driver `{forbidden}`.",
        );
    }

    assert!(
        accordion_semantics.contains("perf_render_count_follow_up_is_tracked_in_plan")
            && accordion_semantics.contains("render_count"),
        "workspace should keep explicit render_count automation follow-up until runtime counters are available.",
    );
}

#[test]
fn contextual_help_view_macro_complexity_is_split_into_semantic_blocks() {
    let view = load_source("view");

    for needle in [
        "fn render_trigger_icon(variant: ContextualHelpVariant) -> impl IntoView",
        "let trigger_view = move || {",
        "let panel_view = move || {",
        "{trigger_view()}",
        "{move || panel_view()}",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp view should keep semantic split marker `{needle}` to avoid giant single `view!`.",
        );
    }

    assert!(
        !view.contains("{match state.variant {"),
        "variant icon rendering should stay extracted from the main `view!` block.",
    );
}

#[test]
fn contextual_help_prefers_functional_splits_over_local_components() {
    let view = load_source("view");

    assert!(
        view.contains("fn render_trigger_icon(variant: ContextualHelpVariant) -> impl IntoView"),
        "lightweight static fragment should be extracted as plain Rust function returning `impl IntoView`.",
    );

    assert!(
        view.contains("let trigger_view = move || {")
            && view.contains("let panel_view = move || {"),
        "local ui fragments should stay function-style splits instead of additional `#[component]` declarations.",
    );

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "ContextualHelp view should keep only one component entry point and avoid local component proliferation.",
    );
}

#[test]
fn contextual_help_static_svg_fragments_are_constantized_with_a11y_intact() {
    let view = load_source("view");

    for needle in [
        "const TRIGGER_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const HELP_ICON_OUTLINE_PATH: &str =",
        "const HELP_ICON_QUERY_PATH: &str =",
        "const INFO_ICON_STEM_PATH: &str = \"M10 9v5\";",
        "d=HELP_ICON_OUTLINE_PATH",
        "d=HELP_ICON_QUERY_PATH",
        "d=INFO_ICON_STEM_PATH",
        "viewBox=TRIGGER_ICON_VIEWBOX",
    ] {
        assert!(
            view.contains(needle),
            "static SVG fragment should be centralized via `{needle}`.",
        );
    }

    assert!(
        !view.contains("d=\"M10 17a7 7 0 1 1 0-14a7 7 0 0 1 0 14Z\""),
        "complex SVG path should not be scattered as inline literal attributes.",
    );

    assert!(
        view.contains("aria-hidden=\"true\""),
        "icon-only svg should keep explicit a11y marker after constantization.",
    );
}

#[test]
fn contextual_help_forbids_inner_html_injection_paths() {
    let mod_rs = load_source("mod");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let view = load_source("view");

    for source in [mod_rs, logic, motion, styles, view] {
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            ".set_inner_html(",
            "dangerously_set_inner_html",
        ] {
            assert!(
                !source.contains(forbidden),
                "ContextualHelp component files must forbid untrusted html injection path `{forbidden}`.",
            );
        }
    }

    for safe_marker in ["{children()}", "{heading}"] {
        assert!(
            view.contains(safe_marker),
            "user-visible content should keep template/text rendering path `{safe_marker}` instead of raw html injection.",
        );
    }
}

#[test]
fn contextual_help_wasm_debug_contract_is_traceable_replayable_and_dev_scoped() {
    let view = load_source("view");
    let mod_rs = load_source("mod");
    let docs_app = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let headless_trace = include_str!("../../../crates/ui-headless/src/trace.rs");

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "if previous_open_value != current_open_value {",
        "if let Some(trace) = trace {",
        "if trace.enabled() {",
        "UiTraceEventKind::Note",
        "open:{}->{} source={}",
        "sync.next_source.as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp should emit traceable transition marker `{needle}`.",
        );
    }

    for needle in [
        "pub ts_ms: u64,",
        "events: RwSignal<Vec<UiTraceEvent>>",
        "events.push(event);",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            headless_trace.contains(needle),
            "ui-headless trace bus should keep ordered replayable event buffer via `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app.contains(needle),
            "docs app should keep dev-only visual debug entry `{needle}`.",
        );
    }

    for needle in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        ".rev()",
        ".take(40)",
    ] {
        assert!(
            debug_overlay.contains(needle),
            "debug overlay should keep minimal replay view contract `{needle}`.",
        );
    }

    for forbidden in ["debug_overlay", "UiTrace", "trace"] {
        assert!(
            !mod_rs.contains(forbidden),
            "ContextualHelp public module should not leak debug API symbol `{forbidden}`.",
        );
    }
}

#[test]
fn contextual_help_dx_workbench_keeps_css_hot_edit_context_and_isolated_canvas() {
    let docs_dev_script = include_str!("../../../scripts/dev-docs-app.sh");
    let overlays_page =
        include_str!("../../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground = include_str!("../../../apps/docs-app/src/playground.rs");

    for needle in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/docs-app\"",
    ] {
        assert!(
            docs_dev_script.contains(needle),
            "docs dev path should keep fast feedback loop marker `{needle}`.",
        );
    }

    for needle in [
        "<Playground\n                title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=test_css_source",
        "test_source_path=\"crates/ui/src/contextual_help/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "let (workbench_open_raw, set_workbench_open_raw) = signal(false);",
        "let (workbench_controlled, set_workbench_controlled) = signal(true);",
        "Switch checked=workbench_controlled set_checked=set_workbench_controlled",
        "\"Toggle workbench open\"",
    ] {
        assert!(
            overlays_page.contains(needle),
            "ContextualHelp docs workbench should keep DX contract marker `{needle}`.",
        );
    }

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "<div data-playground-scope=scope_id.clone()>",
        "on_press=on_reset_test_css",
    ] {
        assert!(
            playground.contains(needle),
            "Playground should keep scoped css hot-edit loop marker `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_docs_interactive_playground_supports_live_props_state_and_spec_linkage() {
    let overlays_page =
        include_str!("../../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_contextual_help_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "<Playground\n                title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_config_signal=actual_config",
        "controls=move || view! {",
        "Switch checked=workbench_disabled set_checked=set_workbench_disabled",
        "Switch checked=workbench_controlled set_checked=set_workbench_controlled",
        "Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria",
        "Switch checked=workbench_custom_class set_checked=set_workbench_custom_class",
        "\"Toggle workbench open\"",
        "\"mode: \"",
        "\" | open: \"",
        "open=workbench_open",
        "default_open=open",
        "ContextualHelpActualConfig {",
    ] {
        assert!(
            overlays_page.contains(needle),
            "contextual-help docs should keep interactive playground marker `{needle}`.",
        );
    }

    assert!(
        e2e_source
            .contains("docs-app contextual-help key flow is repeatable with semantic breakpoints"),
        "interactive playground acceptance flow should remain reproducible in e2e key-flow regression.",
    );

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "check2 should mark interactive playground item complete.",
    );
}

#[test]
fn contextual_help_source_first_docs_are_copy_paste_ready_and_synced() {
    let overlays_page =
        include_str!("../../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "data-slot=\"contextual-help-source-first\"",
        "<Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "text=\"use leptos::prelude::*;\\nuse ui::*;",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"contextual-help-source-paths\"",
        "components/contextual-help/src/mod.rs",
        "components/contextual-help/src/logic.rs",
        "components/contextual-help/src/view.rs",
        "components/contextual-help/src/styles.rs",
        "components/contextual-help/src/motion.rs",
        "data-slot=\"contextual-help-source-prerequisites\"",
        "component-contextual_help",
        "inject-css",
    ] {
        assert!(
            overlays_page.contains(needle),
            "source-first docs section should keep marker `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy pipeline should keep import-ready marker `{needle}`.",
        );
    }

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "check2 should mark source-first copy-paste-ready item complete.",
    );
}

#[test]
fn contextual_help_heroui_strategy_and_component_docs_stay_synchronized() {
    let heroui_strategy = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let readme = include_str!("../src/README.md");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "### ContextualHelp 同步记录（2026-02-20）",
        "open + on_open_change + default_open",
        "is_disabled",
        "disabled",
        "component_doc!(\"ContextualHelp\", \"contextual-help\", \"Overlays\", overlays::contextual_help)",
        "apps/docs-app/src/pages/components/pages/overlays.rs::contextual_help()",
        "compose_copy_ready_code",
        "研究文档补充判定：本轮为参数语义命名与文档验收面同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "HeroUI strategy doc should keep contextual-help sync marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ContextualHelp\"",
        "\"contextual-help\"",
        "overlays::contextual_help",
    ] {
        assert!(
            docs_registry.contains(needle),
            "docs registry should keep contextual-help index marker `{needle}`.",
        );
    }

    assert!(
        readme.contains(
            "`apps/docs-app/src/pages/components/pages/overlays.rs` 的 `contextual_help()` 页面。"
        ),
        "README should keep contextual-help docs entry marker.",
    );

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "check2 should mark HeroUI strategy/doc-sync item complete.",
    );
}

#[test]
fn contextual_help_e2e_contract_uses_semantic_selectors_and_wasm_safe_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_contextual_help_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "body:not(:has(#boot))",
        "[data-slot=\"contextual-help\"][data-open-mode=\"controlled\"][data-class-source=\"custom\"]",
        "[data-slot=\"contextual-help-panel\"][data-open-mode=\"controlled\"]",
        "button[aria-haspopup=\"dialog\"]",
        "page.keyboard.press(\"Escape\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveCount(0, { timeout: 6000 })",
    ] {
        assert!(
            e2e_source.contains(required),
            "contextual-help e2e spec should keep semantic ready/settled selector token `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep(", "getByText("] {
        assert!(
            !e2e_source.contains(forbidden),
            "contextual-help e2e spec must avoid unstable sleep/text selector `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "contextual-help check2 should mark e2e selector stability item complete.",
    );
}

#[test]
fn contextual_help_e2e_key_flow_is_repeatable_and_semantic_breakpointed() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_contextual_help_contract.spec.mjs");
    let check2_source = include_str!("../check2.md");

    for required in [
        "docs-app contextual-help key flow is repeatable with semantic breakpoints",
        "[data-slot=\"contextual-help\"][data-open-mode=\"controlled\"][data-class-source=\"custom\"]",
        "button[aria-haspopup=\"dialog\"]",
        "page.keyboard.press(\"Escape\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveAttribute(\"role\", \"dialog\")",
        "toHaveCount(0, { timeout: 6000 })",
        "toBeFocused()",
    ] {
        assert!(
            e2e_source.contains(required),
            "contextual-help e2e key-flow regression should keep semantic breakpoint token `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "contextual-help e2e key-flow should avoid fixed-sleep wait `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "contextual-help check2 should mark key-flow regression item complete.",
    );
}

#[test]
fn contextual_help_docs_sync_keeps_examples_and_matrixes_aligned_with_logic() {
    let docs_source = include_str!("../../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let logic_source = load_source("logic");
    let check2_source = include_str!("../check2.md");

    for required in [
        "<Playground title=\"Hello World (Default API)\" code_signal=semantic_code>",
        "<Playground title=\"Info Variant + Controlled\" code_signal=controlled_code>",
        "<Playground title=\"State Matrix\" code_signal=comparison_code>",
        "data-slot=\"contextual-help-api-matrix\"",
        "data-slot=\"contextual-help-state-matrix\"",
        "ContextualHelpVariant::default()",
        "ui_headless::PopoverPlacement::default()",
        "default path = uncontrolled (open absent); `default_open` omitted => internal false",
        "compatibility alias for `is_disabled`; precedence = is_disabled -> disabled -> false",
        "is_disabled=true",
        "on_open_change=on_controlled_open_change",
        "default_open=open",
        "\"size axis\"",
        "N/A (ContextualHelp trigger is fixed ButtonSize::IconSm)",
    ] {
        assert!(
            docs_source.contains(required),
            "docs page should keep contextual-help sync marker `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool",
        "pub fn resolve_open_state_config(",
        "contextual_help_state::resolve_open_config(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should remain the single source for documented API/default marker `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "check2 should mark docs/api/state matrix synchronization item complete.",
    );
}

#[test]
fn contextual_help_engineering_contract_uses_structured_serde_trace_and_runtime_agnostic_api() {
    let mod_rs = load_source("mod");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let protocol = load_source("protocol");
    let protocol_test = load_source("protocol_test");
    let view = load_source("view");
    let trace = include_str!("../../../crates/ui-headless/src/trace.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "Serialize, Deserialize",
        "schema_version",
    ] {
        assert!(
            protocol.contains(needle),
            "spec/protocol serialization should stay structured via `{needle}`.",
        );
    }

    for needle in [
        "use serde::de::DeserializeOwned;",
        "fn assert_serde<T>()",
        "assert_serde::<",
    ] {
        assert!(
            protocol_test.contains(needle),
            "protocol contract should keep explicit serde regression marker `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_open_state_traced(",
        "ui_headless::UiTraceEventKind::Note",
        "\"open:{}->{} source={}\"",
    ] {
        assert!(
            view.contains(needle),
            "ContextualHelp trace semantics should use shared contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum UiTraceEventKind",
        "OpenChange {",
        "Note {",
        "pub struct UiTraceEvent",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace.contains(needle),
            "shared trace bus should keep canonical event semantics marker `{needle}`.",
        );
    }

    assert!(
        controllable_state.contains("use_controllable_open_state_traced(")
            && controllable_state.contains("trace.emit(component, UiTraceEventKind::OpenChange"),
        "headless traced-state primitive should keep unified open-change emission contract.",
    );

    for source in [mod_rs, logic, motion, view] {
        for forbidden in [
            "tokio::",
            "async_std::",
            "runtime::",
            "#[tokio::main]",
            "#[tokio::test]",
            "pub async fn",
        ] {
            assert!(
                !source.contains(forbidden),
                "component public/runtime boundary should not leak runtime detail `{forbidden}`.",
            );
        }
    }
}

#[test]
fn contextual_help_styles_use_defensive_variable_fallback_chain() {
    let styles = load_source("styles");
    let theme_css = include_str!("../../../crates/ui-theme/src/css.rs");

    let has_css_hex_literal = styles.as_bytes().windows(2).enumerate().any(|(idx, pair)| {
        if pair[0] != b'#' {
            return false;
        }
        let Some(next) = styles.as_bytes().get(idx + 1) else {
            return false;
        };
        next.is_ascii_hexdigit()
    });

    for required in [
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-border, var(--ui-fallback-border))",
    ] {
        assert!(
            styles.contains(required),
            "ContextualHelp styles should keep defensive variable chain `{required}`.",
        );
    }

    for forbidden in ["18px", "240px"] {
        assert!(
            !styles.contains(forbidden),
            "ContextualHelp styles should avoid hardcoded terminal value `{forbidden}`.",
        );
    }

    assert!(
        !has_css_hex_literal,
        "ContextualHelp styles should avoid hardcoded CSS hex literals; use theme tokens/fallbacks instead.",
    );

    for required_fallback in [
        "--ui-fallback-icon-size-200:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-border:",
    ] {
        assert!(
            theme_css.contains(required_fallback),
            "ui-theme should remain SSOT for fallback terminal `{required_fallback}`.",
        );
    }
}

#[test]
fn contextual_help_cascade_layer_and_runtime_style_contract_is_enforced() {
    let view = load_source("view");
    let css_registry = include_str!("../../../crates/ui/src/css.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-contextual_help\")]",
        "out.push_str(crate::contextual_help::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_registry.contains(needle),
            "components css registry should keep cascade-layer contract marker `{needle}`.",
        );
    }

    for forbidden in [
        "style=",
        "style:top",
        "style:left",
        "style:width",
        "style:height",
    ] {
        assert!(
            !view.contains(forbidden),
            "ContextualHelp view should avoid plain inline runtime style token `{forbidden}`.",
        );
    }

    assert!(
        !view.contains("style:"),
        "ContextualHelp has no runtime style payload; style attributes should remain absent in view layer.",
    );
}

#[test]
fn contextual_help_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let contextual_help_motion = load_source("motion");
    let contextual_help_view = load_source("view");
    let popover_motion = include_str!("../../popover/src/motion.rs");
    let popover_view = include_str!("../../popover/src/view.rs");
    let ui_motion_spring = include_str!("../../../crates/ui-motion/src/spring.rs");

    for needle in [
        "pub struct ContextualHelpMotion {",
        "pub popover: crate::popover::PopoverMotion,",
        "pub fn sanitize_motion(motion: ContextualHelpMotion) -> ContextualHelpMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            contextual_help_motion.contains(needle),
            "ContextualHelp motion contract should keep component-scoped marker `{needle}`.",
        );
    }

    for needle in [
        "SpringConfig {",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            popover_motion.contains(needle),
            "Popover motion backend should keep contract marker `{needle}`.",
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should preserve reduced-motion settlement marker `{needle}`.",
        );
    }

    for needle in [
        "motion::attach_motion(",
        "motion=motion.popover",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_view.contains(needle)
                || contextual_help_view.contains(needle)
                || popover_motion.contains(needle),
            "motion attach/non-wasm no-op chain should keep marker `{needle}`.",
        );
    }
}

#[test]
fn contextual_help_ui_components_entrypoint_layout_contract_is_stable() {
    let ui_components_cargo_toml = include_str!("../../../crates/ui/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_root = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "component-contextual_help = [\"component-button\", \"component-popover\"]",
        "all-components = [",
        "web-demo-components = [",
        "\"component-contextual_help\",",
    ] {
        assert!(
            ui_components_cargo_toml.contains(needle),
            "ui feature tree should keep contextual-help registration marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-contextual_help\")]",
        "pub mod contextual_help;",
        "#[path = \"../../../components/contextual-help/src/mod.rs\"]",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib entry should keep feature-gated contextual-help export `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib should not expose forbidden legacy module `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-contextual_help\")]",
        "out.push_str(crate::contextual_help::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css entry should keep feature-gated layer aggregation marker `{needle}`.",
        );
    }

    for needle in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(needle),
            "UiRoot entry should keep centralized theme/injection marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active_highlight shared primitive should keep generic motion/style marker `{needle}`.",
        );
    }

    for forbidden in ["contextual-help", "business", "copy"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight shared primitive should avoid component/business-specific marker `{forbidden}`.",
        );
    }

    for path in [
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
    ] {
        assert!(
            !std::path::Path::new(path).exists(),
            "forbidden compatibility file should not exist: `{path}`.",
        );
    }
}
