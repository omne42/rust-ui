use super::*;

#[test]
fn compose_class_name_includes_markers() {
    let class_name = compose_class_name(
        Some("docs-error-message".to_string()),
        resolve_state(ErrorMessageStateInput {
            tone: ErrorMessageTone::Negative,
            disabled: true,
            truncate: true,
            has_custom_message: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-error-message",
        "ui-error-message--tone-negative",
        "ui-error-message--disabled",
        "ui-error-message--truncate",
        "ui-error-message--custom-class",
        "docs-error-message",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
