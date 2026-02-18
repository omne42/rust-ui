use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn error_message_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/error_message/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ErrorMessage internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn error_message_uses_logic_state_model() {
    let logic_source = load_source("src/error_message/logic.rs");
    let view_source = load_source("src/error_message/view.rs");

    for needle in [
        "pub use ui_state_primitives::error_message::{",
        "ErrorMessageTone",
        "ErrorMessageElement",
        "ErrorMessageState",
        "ErrorMessageStateInput",
        "normalize_optional_text",
        "normalize_message",
        "normalize_aria_label",
        "resolve_effective_tone",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ErrorMessage logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, ErrorMessageOptions, use_error_message};",
        "logic::normalize_message(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(logic::ErrorMessageStateInput {",
        "use_error_message(ErrorMessageOptions {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ErrorMessage view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn error_message_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/error_message/view.rs");

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
        "data-ui-schema=\"ui.error-message.agent-contract.v1\"",
        "data-ui-intent=\"form-validation-feedback\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-stream-fallback=\"snapshot\"",
        "data-output-status=move || ui_output_status.get()",
    ] {
        assert!(
            source.contains(attr),
            "ErrorMessage should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn error_message_state_primitives_define_centralized_contract() {
    let source = load_source("../../crates/ui-state-primitives/src/error_message.rs");

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
    let source = load_source("../../crates/ui-headless/src/error_message.rs");

    for needle in [
        "pub struct ErrorMessageAttrs",
        "pub struct ErrorMessageContract",
        "pub struct ErrorMessageOptions",
        "pub fn use_error_message(options: ErrorMessageOptions) -> ErrorMessageContract",
        "live_region_attrs(LiveRegionPriority::Assertive)",
        "locale_attrs(options.lang, options.dir)",
        "data_tone",
        "data_message_source",
        "data_aria_source",
        "data_class_source",
    ] {
        assert!(
            source.contains(needle),
            "error_message headless contract should include `{needle}`."
        );
    }
}

#[test]
fn error_message_styles_include_tone_state_and_markers() {
    let source = load_source("src/error_message/styles.rs");

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
fn error_message_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn error_message() -> AnyView",
        "title=\"ErrorMessage\"",
        "slug=\"error-message\"",
        "description=\"baseline-style inline error primitive with centralized tone/disabled/truncate/source normalization and stable slot/data contracts.\"",
        "<Playground title=\"Tone Variants\" code_signal=tone_code>",
        "<Playground title=\"Truncate + Disabled + Element + Custom Class\" code_signal=state_code>",
        "title=\"Display Comparisons (Tone / State / Element)\"",
        "title=\"Config + Code + CSS Test Workbench\"",
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
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Tone Variants\"",
        "text=\"Invalid email address\".to_string()",
        "aria_label=\"Email error\".to_string()",
        "text=\"Username contains unsupported characters.\".to_string()",
        "tone=ErrorMessageTone::Neutral",
        "text=\"Verification code expired, request a new one.\".to_string()",
        "tone=ErrorMessageTone::Negative",
        "title=\"Truncate + Disabled + Element + Custom Class\"",
        "text=\"A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.\".to_string()",
        "truncate=true",
        "class_name=\"docs-error-message-custom\".to_string()",
        "text=\"This error remains visible but marked as disabled for read-only states.\".to_string()",
        "disabled=true",
        "element=ErrorMessageElement::Div",
        "aria_label=\"Disabled error message\".to_string()",
        "title=\"Display Comparisons (Tone / State / Element)\"",
        "title=\"Config + Code + CSS Test Workbench\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || {",
    ] {
        assert!(
            source.contains(needle),
            "error_message docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn error_message_readme_includes_display_config_code_css_test_sections() {
    let source = load_source("src/error_message/README.md");

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
    let source = load_source("src/error_message/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "error_message check2 should not keep unchecked checklist items."
    );
}
