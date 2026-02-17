use crate::accordion::{AccordionSelectionMode, AccordionVariant, logic};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const ACCORDION_COMPONENT_SCHEMA_NAME: &str = "ui.accordion.component-spec";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccordionComponentSchemaVersion {
    #[serde(rename = "1")]
    V1,
}

impl AccordionComponentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccordionSelectionModeSpec {
    Single,
    #[default]
    Multiple,
}

impl From<AccordionSelectionModeSpec> for AccordionSelectionMode {
    fn from(value: AccordionSelectionModeSpec) -> Self {
        match value {
            AccordionSelectionModeSpec::Single => AccordionSelectionMode::Single,
            AccordionSelectionModeSpec::Multiple => AccordionSelectionMode::Multiple,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccordionVariantSpec {
    #[default]
    Light,
    Shadow,
    Bordered,
    Splitted,
}

impl From<AccordionVariantSpec> for AccordionVariant {
    fn from(value: AccordionVariantSpec) -> Self {
        match value {
            AccordionVariantSpec::Light => AccordionVariant::Light,
            AccordionVariantSpec::Shadow => AccordionVariant::Shadow,
            AccordionVariantSpec::Bordered => AccordionVariant::Bordered,
            AccordionVariantSpec::Splitted => AccordionVariant::Splitted,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionComponentItemSpec {
    #[serde(default)]
    pub key: Option<usize>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_open: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionComponentSpec {
    pub schema_name: String,
    pub schema_version: AccordionComponentSchemaVersion,
    #[serde(default)]
    pub id_base: Option<String>,
    #[serde(default)]
    pub selection_mode: AccordionSelectionModeSpec,
    #[serde(default)]
    pub variant: AccordionVariantSpec,
    #[serde(default)]
    pub disallow_empty_selection: bool,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub items: Vec<AccordionComponentItemSpec>,
}

impl Default for AccordionComponentSpec {
    fn default() -> Self {
        Self {
            schema_name: ACCORDION_COMPONENT_SCHEMA_NAME.to_string(),
            schema_version: AccordionComponentSchemaVersion::V1,
            id_base: None,
            selection_mode: AccordionSelectionModeSpec::default(),
            variant: AccordionVariantSpec::default(),
            disallow_empty_selection: false,
            is_disabled: false,
            items: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedAccordionComponentItemSpec {
    pub key: usize,
    pub label: String,
    pub body: String,
    pub is_disabled: bool,
    pub is_open: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedAccordionComponentSpec {
    pub schema_name: String,
    pub schema_version: AccordionComponentSchemaVersion,
    pub id_base: Option<String>,
    pub selection_mode: AccordionSelectionMode,
    pub variant: AccordionVariant,
    pub disallow_empty_selection: bool,
    pub is_disabled: bool,
    pub items: Vec<ResolvedAccordionComponentItemSpec>,
    pub open_keys: BTreeSet<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccordionComponentSpecError {
    UnsupportedSchemaName {
        expected: &'static str,
        actual: String,
    },
}

impl AccordionComponentSpec {
    pub fn resolve(self) -> Result<ResolvedAccordionComponentSpec, AccordionComponentSpecError> {
        if self.schema_name.trim() != ACCORDION_COMPONENT_SCHEMA_NAME {
            return Err(AccordionComponentSpecError::UnsupportedSchemaName {
                expected: ACCORDION_COMPONENT_SCHEMA_NAME,
                actual: self.schema_name,
            });
        }

        let selection_mode = AccordionSelectionMode::from(self.selection_mode);
        let variant = AccordionVariant::from(self.variant);
        let id_base = logic::normalize_optional_text(self.id_base);
        let configured_keys = self.items.iter().map(|item| item.key).collect::<Vec<_>>();
        let resolved_keys = logic::assign_item_keys(&configured_keys);

        let mut items = self
            .items
            .into_iter()
            .zip(resolved_keys)
            .enumerate()
            .map(|(index, (item, key))| ResolvedAccordionComponentItemSpec {
                key,
                label: logic::resolve_item_label(item.label, index),
                body: item.body,
                is_disabled: item.is_disabled,
                is_open: item.is_open,
            })
            .collect::<Vec<_>>();

        let item_keys = items.iter().map(|item| item.key).collect::<Vec<_>>();
        let requested_open = items
            .iter()
            .filter_map(|item| item.is_open.then_some(item.key))
            .collect::<BTreeSet<_>>();
        let open_keys = logic::normalize_default_open_for_items(
            selection_mode,
            Some(&requested_open),
            &item_keys,
            self.disallow_empty_selection,
        );
        for item in &mut items {
            item.is_open = open_keys.contains(&item.key);
        }

        Ok(ResolvedAccordionComponentSpec {
            schema_name: ACCORDION_COMPONENT_SCHEMA_NAME.to_string(),
            schema_version: self.schema_version,
            id_base,
            selection_mode,
            variant,
            disallow_empty_selection: self.disallow_empty_selection,
            is_disabled: self.is_disabled,
            items,
            open_keys,
        })
    }
}

#[cfg(test)]
mod tests {
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
}
