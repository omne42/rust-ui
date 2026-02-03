use leptos::prelude::*;

#[path = "../src/text_field.rs"]
mod text_field;

use text_field::{TextFieldOptions, use_text_field};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

#[test]
fn describedby_combines_description_error_and_extra() {
    init_executor();

    let (invalid, set_invalid) = signal(true);

    let aria = use_text_field(TextFieldOptions {
        id: "email".to_string(),
        has_description: true,
        has_error: true,
        aria_describedby: Some("external-help email-description".to_string()).into(),
        is_invalid: invalid.into(),
        is_required: false.into(),
    });

    assert_eq!(aria.input.id, "email");
    assert_eq!(aria.label.for_attr, "email");
    assert_eq!(aria.description.id, "email-description");
    assert_eq!(aria.error.id, "email-error");
    assert_eq!(
        aria.input.aria_describedby.get_untracked(),
        Some("email-description email-error external-help".to_string())
    );
    assert_eq!(aria.input.aria_invalid.get_untracked(), Some("true"));
    assert_eq!(aria.input.aria_required.get_untracked(), None);

    set_invalid.set(false);
    assert_eq!(
        aria.input.aria_describedby.get_untracked(),
        Some("email-description external-help".to_string())
    );
    assert_eq!(aria.input.aria_invalid.get_untracked(), None);
}

#[test]
fn describedby_is_none_without_description_or_invalid_error() {
    init_executor();

    let aria = use_text_field(TextFieldOptions {
        id: "name".to_string(),
        has_description: false,
        has_error: true,
        aria_describedby: None.into(),
        is_invalid: false.into(),
        is_required: false.into(),
    });

    assert_eq!(aria.input.aria_describedby.get_untracked(), None);
    assert_eq!(aria.input.aria_invalid.get_untracked(), None);
}

#[test]
fn required_sets_aria_required() {
    init_executor();

    let (required, set_required) = signal(false);

    let aria = use_text_field(TextFieldOptions {
        id: "country".to_string(),
        has_description: false,
        has_error: false,
        aria_describedby: None.into(),
        is_invalid: false.into(),
        is_required: required.into(),
    });

    assert_eq!(aria.input.aria_required.get_untracked(), None);
    set_required.set(true);
    assert_eq!(aria.input.aria_required.get_untracked(), Some("true"));
}
