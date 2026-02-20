use super::*;

#[test]
fn compose_copy_ready_code_prepends_imports_when_missing() {
    let code = compose_copy_ready_code(
        "<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};",
    );

    assert!(code.contains("use ui_components::{Button, ButtonVariant};"));
    assert!(code.contains("<Button variant=ButtonVariant::Default>\"Button\"</Button>"));
}

#[test]
fn compose_copy_ready_code_keeps_existing_imports() {
    let code = compose_copy_ready_code(
        "use ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui_components::*;",
    );

    assert_eq!(
        code,
        "use leptos::prelude::*;\n\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
    );
}

#[test]
fn compose_copy_ready_code_does_not_duplicate_when_roots_exist() {
    let code = compose_copy_ready_code(
        "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
        "use leptos::prelude::*;\nuse ui_components::*;",
    );

    assert_eq!(
        code,
        "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
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
    assert!(css.contains("ui-components aggregated css"));
    assert!(css.contains(".playground__preview"));
}
