use super::*;

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(FieldLabelStateInput {
        tone: FieldLabelTone::Muted,
        required: true,
        disabled: true,
        has_for_id: true,
        has_custom_text: true,
        has_custom_indicator: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-field-label-custom".to_string()), state);

    for token in [
        "ui-field-label",
        "ui-field-label--tone-muted",
        "ui-field-label--required",
        "ui-field-label--disabled",
        "ui-field-label--for",
        "ui-field-label--text-custom",
        "ui-field-label--indicator-custom",
        "ui-field-label--aria-custom",
        "ui-field-label--custom-class",
        "docs-field-label-custom",
    ] {
        assert!(class_name.contains(token), "class should contain `{token}`");
    }
}
