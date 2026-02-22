use super::*;
use std::path::Path;

#[test]
fn compose_copy_ready_code_prepends_imports_when_missing() {
    let code = compose_copy_ready_code(
        "<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant};",
    );

    assert!(code.contains("use ui::{Button, ButtonVariant};"));
    assert!(code.contains("<Button variant=ButtonVariant::Default>\"Button\"</Button>"));
}

#[test]
fn compose_copy_ready_code_keeps_existing_imports() {
    let code = compose_copy_ready_code(
        "use ui::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui::*;",
    );

    assert_eq!(
        code,
        "use leptos::prelude::*;\n\nuse ui::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
    );
}

#[test]
fn compose_copy_ready_code_does_not_duplicate_when_roots_exist() {
    let code = compose_copy_ready_code(
        "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui::*;",
    );

    assert_eq!(
        code,
        "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
    );
}

#[test]
fn compose_copy_ready_code_skips_imports_when_none_requested() {
    let code = compose_copy_ready_code("<Accordion />", "");
    assert_eq!(code, "<Accordion />");
}

#[test]
fn compose_scoped_css_wraps_plain_declarations() {
    let css = compose_scoped_css("[data-playground-scope=\"x\"]", "--ui-radius-md: 12px;");
    assert_eq!(
        css,
        "[data-playground-scope=\"x\"] {\n--ui-radius-md: 12px;\n}"
    );
}

#[test]
fn compose_scoped_css_replaces_scope_token_for_rule_blocks() {
    let css = compose_scoped_css(
        "[data-playground-scope=\"x\"]",
        ":scope .ui-button { border-radius: 18px; }",
    );
    assert_eq!(
        css,
        "[data-playground-scope=\"x\"] .ui-button { border-radius: 18px; }"
    );
}

#[test]
fn compose_original_css_source_contains_base_and_components_sections() {
    let css = compose_original_css_source();
    assert!(css.contains("apps/docs-app/app.css"));
    assert!(css.contains("ui aggregated css"));
    assert!(css.contains(".playground__preview"));
}

#[test]
fn playground_code_and_actual_config_use_reactive_code_block_binding() {
    let source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/playground.rs");
    let source = std::fs::read_to_string(&source_path)
        .unwrap_or_else(|err| panic!("failed to read {source_path:?}: {err}"));

    assert!(
        source.contains("let code = resolved_code.get();"),
        "playground code panel must rebuild CodeBlock from reactive resolved_code signal",
    );
    assert!(
        source.contains("let config = signal.get();"),
        "playground actual-config panel must rebuild CodeBlock from reactive test_config_signal",
    );
}
