fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "view" => include_str!("../src/view.rs"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/error_message.rs"),
        "headless" => include_str!("../../../crates/ui-headless/src/error_message.rs"),
        "legacy_semantics" => {
            include_str!("../../../components/error-message/test/error_message_semantics.rs")
        }
        "manifest" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/error_message.rbi"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn error_message_ui_components_export_surface_is_stable_and_platform_agnostic() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ErrorMessage;",
        "pub use motion::ErrorMessageMotion;",
    ] {
        assert!(
            module.contains(required),
            "error_message module boundary should contain `{required}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "error_message public surface should not expose `{forbidden}`."
        );
    }
}

#[test]
fn error_message_layer_files_follow_logic_view_styles_motion_split() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for required in [
        "pub use ui_state_primitives::error_message::{",
        "resolve_state,",
        "compose_class_name",
        "ErrorMessageStateFlagsInput,",
        "ErrorMessageStatus,",
        "ErrorMessageModelInput,",
        "ErrorMessageModel,",
        "normalize_state_flags,",
        "resolve_status,",
        "status_to_primitive_flags,",
        "resolve_model,",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep primitive mapping marker `{required}`."
        );
    }
    for forbidden in ["view! {", "use_error_message(", "web_sys::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain view/headless/platform details `{forbidden}`."
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, ErrorMessageOptions, use_error_message};",
        "logic::resolve_model(logic::ErrorMessageModelInput {",
        "let state = StoredValue::new(model.state);",
        "data-slot=\"error-message\"",
        "data-ui-schema=move || semantics.get().attrs.data_ui_schema",
        "data-ui-intent=move || semantics.get().attrs.data_ui_intent",
        "data-ui-stream-mode=move || semantics.get().attrs.data_ui_stream_mode",
        "data-stream-fallback=move || semantics.get().attrs.data_stream_fallback",
    ] {
        assert!(
            view.contains(required),
            "view.rs should assemble component + headless contract via `{required}`."
        );
    }
    for forbidden in [
        "pub struct ErrorMessageStateInput",
        "pub struct ErrorMessageState",
        "data-ui-schema=\"ui.error-message.agent-contract.v1\"",
        "data-ui-intent=\"form-validation-feedback\"",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not redefine primitives `{forbidden}`."
        );
    }

    assert!(
        styles.contains("var(--ui-"),
        "styles.rs should stay token-first and consume `--ui-*` variables."
    );
    for forbidden in ["web_sys::", "leptos::"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not contain runtime/platform code `{forbidden}`."
        );
    }

    for required in [
        "pub struct ErrorMessageMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should own motion contract + attach marker `{required}`."
        );
    }
    for forbidden in ["role=", "aria-live", "data-slot=\"error-message\""] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not contain semantic/view mounting detail `{forbidden}`."
        );
    }
}

#[test]
fn error_message_consumes_primitive_and_headless_without_reimplementation() {
    let logic = load_source("logic");
    let primitive = load_source("primitive");
    let headless = load_source("headless");

    for required in [
        "pub struct ErrorMessageStateInput",
        "pub struct ErrorMessageState",
        "pub fn resolve_state(input: ErrorMessageStateInput) -> ErrorMessageState",
    ] {
        assert!(
            primitive.contains(required),
            "state primitive contract should be centralized via `{required}`."
        );
    }

    for forbidden in [
        "pub struct ErrorMessageStateInput {",
        "pub struct ErrorMessageState {",
        "pub fn resolve_state(input: ErrorMessageStateInput) -> ErrorMessageState {",
    ] {
        assert!(
            !logic.contains(forbidden),
            "component logic should not reimplement primitive `{forbidden}`."
        );
    }

    for required in [
        "pub struct ErrorMessageContract",
        "pub struct ErrorMessageAttrs",
        "pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = \"ui.error-message.agent-contract.v1\";",
        "pub const ERROR_MESSAGE_AGENT_INTENT: &str = \"form-validation-feedback\";",
        "pub enum ErrorMessageAgentOutputMode",
        "pub fn use_error_message(options: ErrorMessageOptions) -> ErrorMessageContract",
    ] {
        assert!(
            headless.contains(required),
            "headless semantic contract should be centralized via `{required}`."
        );
    }
}

#[test]
fn error_message_has_local_semantics_test_and_keeps_legacy_semantics_bridge() {
    let local_semantics = include_str!("../test/semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");

    assert!(
        local_semantics.contains("error_message_layer_files_follow_logic_view_styles_motion_split"),
        "local `components/error-message/test/semantics.rs` should exist and assert layer split."
    );
    assert!(
        legacy_semantics.contains("fn error_message_uses_logic_state_model()"),
        "legacy semantics file should remain readable during migration bridge."
    );
}

#[test]
fn error_message_api_naming_prefers_is_prefix_and_keeps_alias_bridge() {
    let view = load_source("view");
    let _logic = load_source("logic");
    let primitive = load_source("primitive");

    for required in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] is_truncated: Option<bool>",
        "#[prop(optional)] truncate: Option<bool>",
        "logic::resolve_model(logic::ErrorMessageModelInput {",
        "text: Some(text),",
        "aria_label,",
        "class_name,",
        "let state = StoredValue::new(model.state);",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep naming migration marker `{required}`."
        );
    }

    assert!(
        !view.contains("unwrap_or(false)"),
        "view.rs should not perform default fallback; defaults must be centralized in logic.rs."
    );
    assert!(
        !view.contains("logic::resolve_state(logic::ErrorMessageStateInput {"),
        "view.rs should not rebuild state input directly."
    );
    assert!(
        primitive.contains("disabled: input.is_disabled.or(input.disabled).unwrap_or(false),")
            && primitive
                .contains("truncate: input.is_truncated.or(input.truncate).unwrap_or(false),")
            && primitive.contains("ErrorMessageStatus::Disabled")
            && primitive.contains(
                "pub fn resolve_model(input: ErrorMessageModelInput) -> ErrorMessageModel"
            ),
        "logic.rs should be the single source of default/priority fallback."
    );
}

#[test]
fn error_message_does_not_use_inner_html_sink() {
    for (name, source) in [
        ("view.rs", load_source("view")),
        ("logic.rs", load_source("logic")),
        ("styles.rs", load_source("styles")),
        ("motion.rs", load_source("motion")),
    ] {
        assert!(
            !source.contains("inner_html="),
            "{name} should not use `inner_html`; error-message must render text via typed props/headless contract."
        );
    }
}

#[test]
fn error_message_styles_use_defensive_variable_fallback_chain() {
    let styles = load_source("styles");

    for required in [
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep defensive variable fallback contract `{required}`."
        );
    }

    for forbidden in ["12px", "16px", "140ms", "ease))"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should avoid component-local terminal fallback `{forbidden}`."
        );
    }
}

#[test]
fn error_message_has_context_compression_manifest_and_rbi_projection() {
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");

    for required in [
        "schema_version = \"1\"",
        "name = \"ErrorMessage\"",
        "crate = \"ui-error-message\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "output_mode_axis = [\"streaming\", \"snapshot\"]",
        "name = \"stream_mode\"",
        "schema = \"ui.error-message.agent-contract.v1\"",
    ] {
        assert!(
            manifest.contains(required),
            "Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = \"ui.error-message.agent-contract.v1\";",
        "pub enum ErrorMessageAgentIntent",
        "pub enum ErrorMessageAgentAction",
        "pub enum ErrorMessageAgentStreamMode",
        "Streaming,",
        "Snapshot,",
        "pub fn ErrorMessage(",
    ] {
        assert!(
            rbi.contains(required),
            "error_message.rbi should keep signature projection marker `{required}`."
        );
    }
}

#[test]
fn error_message_semantics_tests_cover_aria_and_data_contracts() {
    let view = load_source("view");

    for required in [
        "role=move || semantics.get().attrs.role",
        "aria-live=move || semantics.get().attrs.aria_live",
        "aria-label=move || semantics.get().attrs.aria_label.clone()",
        "aria-disabled=move || semantics.get().attrs.aria_disabled",
        "data-state=move || semantics.get().attrs.data_state",
        "data-message-source=move || semantics.get().attrs.data_message_source",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-ui-state=move || semantics.get().state.state",
        "data-ui-source=move || semantics.get().state.message_source",
        "data-output-status=move || semantics.get().attrs.data_output_status",
    ] {
        assert!(
            view.contains(required),
            "error_message semantic contract should include `{required}`."
        );
    }
}

#[test]
fn error_message_is_non_interactive_leaf_without_focus_or_high_frequency_loops() {
    let view = load_source("view");
    let motion = load_source("motion");
    let headless = load_source("headless");

    for forbidden in [
        "tabindex=",
        "autofocus",
        "on:focus",
        "on:blur",
        "on:keydown",
        "on:keyup",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
    ] {
        assert!(
            !view.contains(forbidden),
            "error_message should remain non-focusable/non-interactive; found `{forbidden}`."
        );
    }

    for forbidden in [
        "request_animation_frame",
        "AnimationFrame",
        "set_interval",
        "set_timeout",
        "on:pointermove",
        "on:mousemove",
    ] {
        assert!(
            !motion.contains(forbidden) && !view.contains(forbidden),
            "error_message should not introduce frame-loop/high-frequency path `{forbidden}`."
        );
    }

    assert!(
        headless.contains("live_region_attrs(LiveRegionPriority::Assertive)"),
        "error_message focus semantics are modeled as non-interactive live-region announcement."
    );
}

#[test]
fn error_message_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let view = load_source("view");
    let headless = load_source("headless");
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");

    for required in [
        "pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = \"ui.error-message.agent-contract.v1\";",
        "pub const ERROR_MESSAGE_AGENT_SCHEMA_VERSION: &str = \"1\";",
        "schema_version = \"1\"",
        "schema = \"ui.error-message.agent-contract.v1\"",
        "pub fn ErrorMessage(",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] is_truncated: Option<bool>",
        "#[prop(optional)] truncate: Option<bool>",
    ] {
        assert!(
            headless.contains(required)
                || manifest.contains(required)
                || rbi.contains(required)
                || view.contains(required),
            "error_message should keep stable v1 contract/public API token `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "deprecated_since",
        "deprecation_window",
        "schema_version = \"2\"",
        "contract.v2",
        "V2",
    ] {
        assert!(
            !headless.contains(forbidden)
                && !manifest.contains(forbidden)
                && !rbi.contains(forbidden)
                && !view.contains(forbidden),
            "error_message should not introduce major-upgrade migration token `{forbidden}`."
        );
    }
}
