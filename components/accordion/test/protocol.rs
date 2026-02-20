use super::*;

fn assert_serde<T>()
where
    T: Serialize + for<'de> Deserialize<'de>,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<AccordionComponentSpec>();
    assert_serde::<AccordionComponentItemSpec>();
    assert_serde::<AccordionSelectionModeSpec>();
    assert_serde::<AccordionVariantSpec>();
    assert_serde::<AccordionComponentSchemaVersion>();
}

#[test]
fn resolve_normalizes_keys_labels_and_open_state() {
    let spec = AccordionComponentSpec {
        schema_name: ACCORDION_COMPONENT_SCHEMA_NAME.to_string(),
        schema_version: AccordionComponentSchemaVersion::V1,
        id_base: Some(" docs-accordion ".to_string()),
        selection_mode: AccordionSelectionModeSpec::Single,
        variant: AccordionVariantSpec::Splitted,
        disallow_empty_selection: true,
        is_disabled: false,
        items: vec![
            AccordionComponentItemSpec {
                key: Some(2),
                label: "  ".to_string(),
                body: "A".to_string(),
                is_disabled: false,
                is_open: false,
            },
            AccordionComponentItemSpec {
                key: Some(2),
                label: "Details".to_string(),
                body: "B".to_string(),
                is_disabled: false,
                is_open: true,
            },
        ],
    };

    let resolved = spec.resolve().expect("spec should resolve");
    assert_eq!(resolved.id_base, Some("docs-accordion".to_string()));
    assert_eq!(resolved.items[0].key, 2);
    assert_eq!(resolved.items[1].key, 0);
    assert_eq!(resolved.items[0].label, "Section 1");
    assert_eq!(resolved.items[1].label, "Details");
    assert_eq!(resolved.open_keys, BTreeSet::from([0]));
    assert!(!resolved.items[0].is_open);
    assert!(resolved.items[1].is_open);
}

#[test]
fn resolve_rejects_unknown_schema_name() {
    let spec = AccordionComponentSpec {
        schema_name: "ui.accordion.unknown".to_string(),
        ..Default::default()
    };
    let error = spec.resolve().expect_err("schema should be rejected");
    assert_eq!(
        error,
        AccordionComponentSpecError::UnsupportedSchemaName {
            expected: ACCORDION_COMPONENT_SCHEMA_NAME,
            actual: "ui.accordion.unknown".to_string(),
        }
    );
}
