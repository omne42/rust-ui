use crate::empty::{EmptyPartState, EmptyPartStateInput, EmptySlot};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: EmptyPartStateInput) -> EmptyPartState {
    EmptyPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: input.slot.state_attr(),
        media_variant: input.media_variant,
        media_variant_attr: input.media_variant.as_attr(),
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        variant_source_attr: if input.slot == EmptySlot::Media
            && input.media_variant != EmptyMediaVariant::Default
        {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: EmptyPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.slot == EmptySlot::Media && state.media_variant == EmptyMediaVariant::Icon {
        classes.push("ui-empty__media--icon".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-empty--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_variant_attr_contract_is_stable() {
        assert_eq!(EmptyMediaVariant::Default.as_attr(), "default");
        assert_eq!(EmptyMediaVariant::Icon.as_attr(), "icon");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-empty  ".to_string())),
            Some("docs-empty".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_slot_and_source_markers() {
        let state = resolve_state(EmptyPartStateInput {
            slot: EmptySlot::Media,
            media_variant: EmptyMediaVariant::Icon,
            has_custom_class_name: true,
        });

        assert_eq!(state.slot_attr, "empty-icon");
        assert_eq!(state.base_class, "ui-empty__media");
        assert_eq!(state.state_attr, "media");
        assert_eq!(state.media_variant_attr, "icon");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.variant_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-empty-custom".to_string()),
            resolve_state(EmptyPartStateInput {
                slot: EmptySlot::Media,
                media_variant: EmptyMediaVariant::Icon,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-empty__media",
            "ui-empty__media--icon",
            "ui-empty--custom-class",
            "docs-empty-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
