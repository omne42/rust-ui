use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("".to_string())), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  hello  ".to_string())),
        Some("hello".to_string())
    );
}

#[test]
fn resolve_label_falls_back_to_default_when_blank() {
    assert_eq!(
        resolve_label("".to_string()),
        (DEFAULT_LABEL.into(), "default")
    );
    assert_eq!(
        resolve_label("   ".to_string()),
        (DEFAULT_LABEL.into(), "default")
    );
    assert_eq!(
        resolve_label(" Name ".to_string()),
        ("Name".to_string(), "custom")
    );
}

#[test]
fn resolve_input_type_tracks_default_and_custom() {
    assert_eq!(resolve_input_type(None), (DEFAULT_INPUT_TYPE, "default"));
    assert_eq!(
        resolve_input_type(Some("")),
        (DEFAULT_INPUT_TYPE, "default")
    );
    assert_eq!(
        resolve_input_type(Some("  ")),
        (DEFAULT_INPUT_TYPE, "default")
    );
    assert_eq!(
        resolve_input_type(Some("text")),
        (DEFAULT_INPUT_TYPE, "default")
    );
    assert_eq!(resolve_input_type(Some("email")), ("email", "custom"));
    assert_eq!(resolve_input_type(Some("  email  ")), ("email", "custom"));
}

#[test]
fn source_attr_from_presence_tracks_custom_flag() {
    assert_eq!(source_attr_from_presence(false), "default");
    assert_eq!(source_attr_from_presence(true), "custom");
}

#[test]
fn resolve_state_derives_state_value_and_requirement_attrs() {
    let state = resolve_state(TextFieldStateInput {
        is_disabled: false,
        is_invalid: true,
        is_read_only: false,
        value: "abc",
        is_required: true,
    });

    assert_eq!(state.state_attr, "invalid");
    assert_eq!(state.value_attr, "filled");
    assert_eq!(state.requirement_attr, "required");
}

#[test]
fn state_attr_prioritizes_disabled_then_invalid_then_readonly() {
    assert_eq!(resolve_state_attr(false, false, false), "ready");
    assert_eq!(resolve_state_attr(false, false, true), "readonly");
    assert_eq!(resolve_state_attr(false, true, true), "invalid");
    assert_eq!(resolve_state_attr(true, true, true), "disabled");
}

#[test]
fn value_and_requirement_attrs_map_to_closed_sets() {
    assert_eq!(resolve_value_attr(""), "empty");
    assert_eq!(resolve_value_attr("   "), "empty");
    assert_eq!(resolve_value_attr("value"), "filled");
    assert_eq!(resolve_requirement_attr(false), "optional");
    assert_eq!(resolve_requirement_attr(true), "required");
}
