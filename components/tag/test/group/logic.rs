use super::*;
use leptos::prelude::*;

#[test]
fn normalize_group_input_centralizes_default_sources() {
    let normalized = normalize_group_input(None, None, None, None, None, None, None);

    assert_eq!(normalized.id_base, DEFAULT_ID_BASE);
    assert_eq!(normalized.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(normalized.class_name, "ui-tag-group");
    assert_eq!(normalized.label, None);
    assert_eq!(normalized.description, None);
    assert_eq!(normalized.error, None);
    assert_eq!(normalized.lang, None);
    assert_eq!(normalized.id_base_source.as_attr(), "default");
    assert_eq!(normalized.aria_label_source.as_attr(), "default");
    assert_eq!(normalized.class_name_source.as_attr(), "default");
    assert_eq!(normalized.lang_source.as_attr(), "missing");
}

#[test]
fn normalize_group_input_trims_user_values() {
    let normalized = normalize_group_input(
        Some("  group-id  ".to_string()),
        Some("  Label  ".to_string()),
        Some("  Desc  ".to_string()),
        Some("  Error  ".to_string()),
        Some("  Custom tags  ".to_string()),
        Some("  custom-class  ".to_string()),
        Some("  zh-CN  ".to_string()),
    );

    assert_eq!(normalized.id_base, "group-id");
    assert_eq!(normalized.label, Some("Label".to_string()));
    assert_eq!(normalized.description, Some("Desc".to_string()));
    assert_eq!(normalized.error, Some("Error".to_string()));
    assert_eq!(normalized.aria_label, "Custom tags");
    assert_eq!(normalized.class_name, "ui-tag-group custom-class");
    assert_eq!(normalized.lang, Some("zh-CN".to_string()));
    assert_eq!(normalized.id_base_source.as_attr(), "custom");
    assert_eq!(normalized.aria_label_source.as_attr(), "custom");
    assert_eq!(normalized.class_name_source.as_attr(), "custom");
    assert_eq!(normalized.lang_source.as_attr(), "provided");
}

#[test]
fn normalize_group_bool_input_uses_is_prefix_inputs() {
    let is_invalid = Signal::derive(|| true);
    let is_required = Signal::derive(|| false);

    let normalized = normalize_group_bool_input(Some(true), Some(is_invalid), Some(is_required));
    assert!(normalized.is_disabled);
    assert!(normalized.is_invalid.get());
    assert!(!normalized.is_required.get());

    let defaulted = normalize_group_bool_input(None, None, None);
    assert!(!defaulted.is_disabled);
    assert!(!defaulted.is_invalid.get());
    assert!(!defaulted.is_required.get());
}

#[test]
fn resolve_group_state_and_item_state_helpers_centralize_state_projection() {
    let tags = [
        Tag::new("tag-rust", "Rust"),
        Tag::disabled("tag-a11y", "A11y"),
    ];
    let root_state = resolve_group_state(
        &tags,
        TagGroupRootStateInput {
            is_disabled: false,
            has_remove_callback: true,
            is_invalid: false,
            is_required: true,
        },
    );
    assert!(root_state.has_items);
    assert_eq!(root_state.item_count, 2);
    assert!(root_state.has_disabled_tags);
    assert!(root_state.has_removable_tags);
    assert!(root_state.is_required);

    let item_state = resolve_group_item_state(TagGroupRenderableItemStateInput {
        is_group_disabled: true,
        has_remove_callback: true,
        is_tag_disabled: false,
    });
    assert!(item_state.is_disabled);
    assert!(!item_state.is_removable);
    assert_eq!(item_state.disabled_source_attr, "group");
    assert_eq!(item_state.removable_source_attr, "disabled");
}

#[test]
fn agent_contract_is_schema_typed_and_snapshot_fallback_is_explicit() {
    let state = resolve_state(
        &[
            Tag::new("tag-rust", "Rust"),
            Tag::new("tag-leptos", "Leptos"),
        ],
        false,
        true,
        false,
        false,
    );
    let contract = resolve_agent_contract(state, TagGroupAgentSource::RemovePointer, true);

    assert_eq!(contract.schema_name, "ui.tag-group.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "collection");
    assert_eq!(contract.action.as_str(), "remove-pointer");
    assert_eq!(contract.state.as_str(), "ready");
    assert_eq!(contract.source.as_str(), "remove-pointer");
    assert_eq!(contract.output_status.as_str(), "submittable");
    assert_eq!(contract.stream_support.as_str(), "unsupported");
    assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
    assert!(contract.capabilities.can_remove);
    assert!(contract.capabilities.can_validate);
    assert!(contract.capabilities.can_disable);
}
