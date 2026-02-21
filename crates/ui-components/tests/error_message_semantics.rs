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

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_error_message_component_source(rel_path: &str) -> String {
    let path = workspace_dir()
        .join("components/error-message")
        .join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_workspace_source(rel_path: &str) -> String {
    let path = workspace_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_error_message_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-error_message\")]")
            && lib_source.contains("pub use ui_error_message as error_message;"),
        "ui-components should re-export the external ui-error-message crate as `error_message`.",
    );
    assert!(
        cargo_source.contains("component-error_message = [\"dep:ui-error-message\"]"),
        "component-error_message feature should depend on dep:ui-error-message after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-error-message = { path = \"../../components/error-message\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-error-message dependency.",
    );
}

#[test]
fn error_message_does_not_expose_logic_or_view_modules() {
    let source = load_error_message_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ErrorMessage internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn error_message_uses_logic_state_model() {
    let logic_source = load_error_message_component_source("src/logic.rs");
    let view_source = load_error_message_component_source("src/view.rs");

    for needle in [
        "pub use ui_state_primitives::error_message::{",
        "ErrorMessageTone",
        "ErrorMessageElement",
        "ErrorMessageState",
        "ErrorMessageStateInput",
        "ErrorMessageStateFlagsInput",
        "ErrorMessageStateFlags",
        "ErrorMessageStatus",
        "ErrorMessageModelInput",
        "ErrorMessageModel",
        "normalize_optional_text",
        "normalize_message",
        "normalize_aria_label",
        "resolve_effective_tone",
        "resolve_state,",
        "normalize_state_flags,",
        "resolve_status,",
        "status_to_primitive_flags,",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorMessage logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, ErrorMessageOptions, use_error_message};",
        "logic::resolve_model(logic::ErrorMessageModelInput {",
        "text: Some(text),",
        "let state = StoredValue::new(model.state);",
        "use_error_message(ErrorMessageOptions {",
        "logic::compose_class_name(class_name.get_value(), state.get_value())",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorMessage view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn error_message_api_naming_uses_is_prefix_with_alias_compatibility() {
    let source = load_error_message_component_source("src/view.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] is_truncated: Option<bool>",
        "#[prop(optional)] truncate: Option<bool>",
        "logic::resolve_model(logic::ErrorMessageModelInput {",
        "text: Some(text),",
        "aria_label,",
        "class_name,",
        "is_disabled,",
        "is_truncated,",
        "let state = StoredValue::new(model.state);",
    ] {
        assert!(
            source.contains(needle),
            "ErrorMessage API naming contract should include `{needle}`."
        );
    }

    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/error_message.rs");
    assert!(
        !source.contains("unwrap_or(false)"),
        "view.rs should not own default fallback branches."
    );
    assert!(
        !source.contains("logic::resolve_state(logic::ErrorMessageStateInput {"),
        "view.rs should not build normalized state directly."
    );
    assert!(
        primitive_source
            .contains("disabled: input.is_disabled.or(input.disabled).unwrap_or(false),")
            && primitive_source
                .contains("truncate: input.is_truncated.or(input.truncate).unwrap_or(false),")
            && primitive_source.contains("ErrorMessageStatus::Disabled")
            && primitive_source.contains(
                "pub fn resolve_model(input: ErrorMessageModelInput) -> ErrorMessageModel"
            ),
        "logic.rs should centralize default fallback and alias priority."
    );
}

#[test]
fn error_message_emits_baseline_style_state_data_attributes() {
    let source = load_error_message_component_source("src/view.rs");

    for attr in [
        "data-slot=\"error-message\"",
        "slot=\"errorMessage\"",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-truncate=move || semantics.get().attrs.data_truncate",
        "data-message-source=move || semantics.get().attrs.data_message_source",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-ui-schema=move || semantics.get().attrs.data_ui_schema",
        "data-ui-intent=move || semantics.get().attrs.data_ui_intent",
        "data-ui-action=move || semantics.get().attrs.data_ui_action",
        "data-ui-stream-mode=move || semantics.get().attrs.data_ui_stream_mode",
        "data-stream-fallback=move || semantics.get().attrs.data_stream_fallback",
        "data-output-status=move || semantics.get().attrs.data_output_status",
    ] {
        assert!(
            source.contains(attr),
            "ErrorMessage should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn error_message_state_primitives_define_centralized_contract() {
    let source = load_workspace_source("crates/ui-state-primitives/src/error_message.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"ErrorMessage\";",
        "pub const DEFAULT_MESSAGE: &str = \"Invalid value\";",
        "pub enum ErrorMessageTone",
        "pub enum ErrorMessageElement",
        "pub struct ErrorMessageStateInput",
        "pub struct ErrorMessageState",
        "pub fn normalize_message(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_effective_tone(",
        "pub fn resolve_state(input: ErrorMessageStateInput) -> ErrorMessageState",
    ] {
        assert!(
            source.contains(needle),
            "error_message primitive contract should contain `{needle}`."
        );
    }
}

#[test]
fn error_message_headless_contract_maps_a11y_and_locale_attrs() {
    let source = load_workspace_source("crates/ui-headless/src/error_message.rs");

    for needle in [
        "pub struct ErrorMessageAttrs",
        "pub struct ErrorMessageContract",
        "pub struct ErrorMessageOptions",
        "pub fn use_error_message(options: ErrorMessageOptions) -> ErrorMessageContract",
        "pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = \"ui.error-message.agent-contract.v1\";",
        "pub const ERROR_MESSAGE_AGENT_INTENT: &str = \"form-validation-feedback\";",
        "pub enum ErrorMessageAgentOutputMode",
        "live_region_attrs(LiveRegionPriority::Assertive)",
        "locale_attrs(options.lang, options.dir)",
        "data_tone",
        "data_message_source",
        "data_aria_source",
        "data_class_source",
        "data_ui_schema",
        "data_ui_intent",
        "data_ui_action",
        "data_ui_stream_mode",
        "data_stream_fallback",
        "data_ui_output_status",
    ] {
        assert!(
            source.contains(needle),
            "error_message headless contract should include `{needle}`."
        );
    }
}

#[test]
fn error_message_styles_include_tone_state_and_markers() {
    let source = load_error_message_component_source("src/styles.rs");

    for selector in [
        ".ui-error-message--tone-auto",
        ".ui-error-message[data-tone=\"auto\"]",
        ".ui-error-message--tone-negative",
        ".ui-error-message[data-tone=\"negative\"]",
        ".ui-error-message--tone-neutral",
        ".ui-error-message[data-tone=\"neutral\"]",
        ".ui-error-message--disabled",
        ".ui-error-message[data-disabled=\"true\"]",
        ".ui-error-message--truncate",
        ".ui-error-message[data-truncate=\"true\"]",
        ".ui-error-message--custom-class",
        ".ui-error-message[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ErrorMessage styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn error_message_theme_token_pipeline_is_traceable_and_consumed() {
    let motion_source = load_error_message_component_source("src/motion.rs");
    let styles_source = load_error_message_component_source("src/styles.rs");
    let theme_tokens_source = load_workspace_source("crates/ui-theme/src/tokens.rs");
    let theme_map_source = load_workspace_source("crates/ui-theme/src/theme.rs");
    let theme_css_source = load_workspace_source("crates/ui-theme/src/css.rs");

    for needle in [
        "pub struct TextFieldMotionTokens",
        "pub fn default_text_field_motion_tokens() -> TextFieldMotionTokens",
        "--ui-text-field-motion-duration",
    ] {
        let found = theme_tokens_source.contains(needle)
            || theme_map_source.contains(needle)
            || theme_css_source.contains(needle);
        assert!(
            found,
            "ui-theme token pipeline should expose `{needle}` in tokens/theme/css layers."
        );
    }

    for needle in [
        "use ui_theme::default_text_field_motion_tokens;",
        "default_text_field_motion_tokens()",
        "var(--ui-font-size-100",
        "var(--ui-line-height-100",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "--ui-text-field-motion-duration",
    ] {
        let found = motion_source.contains(needle) || styles_source.contains(needle);
        assert!(
            found,
            "error_message should consume ui-theme variables/tokens via `{needle}`."
        );
    }

    assert!(
        !styles_source.contains("#"),
        "error_message styles should avoid hardcoded hex colors and rely on theme variables."
    );
    for forbidden in ["12px", "16px", "140ms", "ease))"] {
        assert!(
            !styles_source.contains(forbidden),
            "error_message styles should avoid component-local terminal fallback `{forbidden}`."
        );
    }
}

#[test]
fn error_message_docs_page_covers_primary_playgrounds() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn error_message() -> AnyView",
        "title=\"ErrorMessage\"",
        "slug=\"error-message\"",
        "description=\"baseline-style inline error primitive with centralized tone/is_disabled/is_truncated/source normalization and stable slot/data contracts.\"",
        "<Playground title=\"Hello World (Default API)\" code_signal=hello_world_code>",
        "<Playground title=\"Tone Variants\" code_signal=tone_code>",
        "<Playground title=\"Truncate + Disabled + Element + Custom Class\" code_signal=state_code>",
        "title=\"Display Comparisons (Tone / State / Element)\"",
        "title=\"Controlled / Uncontrolled (Input-Driven N/A)\"",
        "title=\"Streaming Optional + Snapshot Fallback\"",
        "title=\"Config + Code + CSS Test Workbench\"",
        "data-slot=\"error-message-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "<ErrorMessage",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for error_message primary playground coverage.",
        );
    }
}

#[test]
fn error_message_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World (Default API)\"",
        "text=\"Invalid email address\".to_string()",
        "title=\"Tone Variants\"",
        "aria_label=\"Email error\".to_string()",
        "text=\"Username contains unsupported characters.\".to_string()",
        "tone=ErrorMessageTone::Neutral",
        "text=\"Verification code expired, request a new one.\".to_string()",
        "tone=ErrorMessageTone::Negative",
        "title=\"Truncate + Disabled + Element + Custom Class\"",
        "text=\"A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.\".to_string()",
        "is_truncated=true",
        "class_name=\"docs-error-message-custom\".to_string()",
        "text=\"This error remains visible but marked as disabled for read-only states.\".to_string()",
        "is_disabled=true",
        "element=ErrorMessageElement::Div",
        "aria_label=\"Disabled error message\".to_string()",
        "title=\"Display Comparisons (Tone / State / Element)\"",
        "title=\"Controlled / Uncontrolled (Input-Driven N/A)\"",
        "No value/on_value_change/default_value triad is required for this component.",
        "title=\"Streaming Optional + Snapshot Fallback\"",
        "data-ui-stream-support/data-ui-stream-mode/data-stream-fallback",
        "title=\"Config + Code + CSS Test Workbench\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || {",
        "data-slot=\"error-message-source-first\"",
        "class_name=\"docs-error-message-source-copy\".to_string()",
        "data-slot=\"error-message-source-paths\"",
        "data-slot=\"error-message-source-prerequisites\"",
    ] {
        assert!(
            source.contains(needle),
            "error_message docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn error_message_heroui_strategy_doc_is_synced_with_docs_entry() {
    let strategy = load_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    let docs_index = load_workspace_source("apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### ErrorMessage 同步记录（2026-02-21）",
        "`ErrorMessage` 维持 form feedback primitive 定位",
        "`disabled/truncate` 仅作兼容别名",
        "`#/components/error-message` 可索引访问",
        "forms_extra.rs::error_message()",
        "`component-error_message`、`inject-css`",
    ] {
        assert!(
            strategy.contains(needle),
            "HeroUI strategy doc should include `{needle}` for error_message sync.",
        );
    }

    assert!(
        docs_index
            .contains("component_doc!(\n        \"ErrorMessage\",\n        \"error-message\",\n        \"Forms\",\n        forms_extra::error_message"),
        "docs-app component index should expose the error_message doc entry.",
    );
}

#[test]
fn error_message_readme_includes_display_config_code_css_test_sections() {
    let source = load_error_message_component_source("src/README.md");

    for needle in [
        "## display（展示区）",
        "## config（配置区）",
        "## code（代码区）",
        "## css test（样式测试区）",
        "## 多场景对比（Comparison Matrix）",
    ] {
        assert!(
            source.contains(needle),
            "error_message README should include `{needle}`."
        );
    }
}

#[test]
fn error_message_check2_has_no_unchecked_checklist_items() {
    let source = load_error_message_component_source("src/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "error_message check2 should not keep unchecked checklist items."
    );
}
