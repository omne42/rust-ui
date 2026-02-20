use super::*;

#[test]
fn intent_variant_mapping_is_stable() {
    assert_eq!(ButtonIntent::Primary.into_variant(), ButtonVariant::Default);
    assert_eq!(
        ButtonIntent::Destructive.into_variant(),
        ButtonVariant::Destructive
    );
    assert_eq!(ButtonIntent::Link.into_variant(), ButtonVariant::Link);
}

#[test]
fn schema_json_is_machine_readable() {
    let json = ButtonSchema::new(
        "btn_del_01",
        ButtonIntent::Destructive,
        "delete_record(id: u32)",
    )
    .requires_confirmation(true)
    .to_json();

    assert!(json.contains("\"schema_version\":1"));
    assert!(json.contains("\"element_id\":\"btn_del_01\""));
    assert!(json.contains("\"intent\":\"destructive\""));
    assert!(json.contains("\"action_signature\":\"delete_record(id: u32)\""));
    assert!(json.contains("\"requires_confirmation\":true"));
}

#[test]
fn schema_version_normalization_is_stable() {
    let normalized = ButtonSchema::new("btn_save", ButtonIntent::Primary, "save()")
        .schema_version(0)
        .to_json();
    let upgraded = ButtonSchema::new("btn_save", ButtonIntent::Primary, "save()")
        .schema_version(2)
        .to_json();

    assert!(normalized.contains("\"schema_version\":0"));
    assert!(upgraded.contains("\"schema_version\":2"));
}

#[test]
fn schema_to_json_result_and_from_json_roundtrip() {
    let original =
        ButtonSchema::new("btn_sync", ButtonIntent::Accent, "sync()").requires_confirmation(true);
    let encoded = original
        .to_json_result()
        .expect("button schema should serialize");
    let decoded = ButtonSchema::from_json(&encoded).expect("button schema should deserialize");

    assert_eq!(decoded, original);
}

#[test]
fn schema_from_json_rejects_missing_or_zero_version() {
    let legacy_missing_version = r#"{"element_id":"btn_a","intent":"primary","action_signature":"save()","requires_confirmation":false}"#;
    let legacy_zero_version = r#"{"schema_version":0,"element_id":"btn_b","intent":"secondary","action_signature":"publish()","requires_confirmation":true}"#;

    let missing_error = ButtonSchema::from_json(legacy_missing_version)
        .expect_err("missing schema_version should be rejected");
    let zero_error = ButtonSchema::from_json(legacy_zero_version)
        .expect_err("zero schema_version should be rejected");

    assert_eq!(missing_error.kind, ButtonSchemaErrorKind::Deserialize);
    assert_eq!(missing_error.code, "button_schema_deserialize_failed");
    assert_eq!(zero_error.kind, ButtonSchemaErrorKind::UnsupportedVersion);
    assert_eq!(zero_error.code, "button_schema_unsupported_version");
    assert_eq!(zero_error.schema_version, Some(0));
}

#[test]
fn schema_from_json_reports_structured_error_for_unsupported_version() {
    let unsupported = format!(
        "{{\"schema_version\":{},\"element_id\":\"btn_x\",\"intent\":\"primary\",\"action_signature\":\"noop()\",\"requires_confirmation\":false}}",
        BUTTON_SCHEMA_VERSION + 1
    );
    let error = ButtonSchema::from_json(&unsupported).expect_err("future schema should fail");

    assert_eq!(error.kind, ButtonSchemaErrorKind::UnsupportedVersion);
    assert_eq!(error.code, "button_schema_unsupported_version");
    assert_eq!(error.schema_version, Some(BUTTON_SCHEMA_VERSION + 1));
    assert_eq!(error.supported_schema_version, BUTTON_SCHEMA_VERSION);
}

#[test]
fn button_text_resolves_static_and_dynamic_values() {
    let (value, set_value) = signal("Count: 1".to_string());
    let static_text = ButtonText::static_text("Save");
    assert_eq!(static_text.resolve(), "Save");

    let dynamic_text = ButtonText::dynamic(value.into());
    assert_eq!(dynamic_text.resolve(), "Count: 1");

    set_value.set("Count: 2".to_string());
    assert_eq!(dynamic_text.resolve(), "Count: 2");
}
