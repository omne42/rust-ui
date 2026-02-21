use super::*;
use crate::{
    CommandCollectionAttr, CommandItem, CommandPartStateInput, CommandQueryAttr,
    CommandRootStateAttr, CommandSlot, CommandSourceAttr,
};

fn sample_groups() -> Vec<CommandGroup> {
    vec![
        CommandGroup::new(
            "Suggestions",
            vec![
                CommandItem::new("calendar", "Calendar")
                    .keywords(vec!["date".to_string(), "event".to_string()])
                    .shortcut("⌘K"),
                CommandItem::new("search", "Search Emoji")
                    .keywords(vec!["emoji".to_string(), "icon".to_string()]),
            ],
        ),
        CommandGroup::new(
            "Settings",
            vec![
                CommandItem::new("billing", "Billing").shortcut("⌘B"),
                CommandItem::new("admin", "Admin").disabled(true),
            ],
        ),
    ]
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Search  ".to_string())),
        Some("Search".to_string())
    );

    assert_eq!(
        normalize_id_base(" docs-command ".to_string()),
        "docs-command".to_string()
    );
    assert_eq!(
        normalize_id_base(" ".to_string()),
        DEFAULT_ID_BASE.to_string()
    );

    assert_eq!(
        resolve_placeholder(Some("  Find action  ".to_string()), Some("Buscar")),
        ("Find action".to_string(), CommandSourceAttr::Custom)
    );
    assert_eq!(
        resolve_placeholder(Some("".to_string()), Some("Buscar")),
        ("Buscar".to_string(), CommandSourceAttr::I18n)
    );
    assert_eq!(
        resolve_placeholder(None, None),
        (DEFAULT_PLACEHOLDER.into(), CommandSourceAttr::Default)
    );

    assert_eq!(
        resolve_empty_label(Some("  No match  ".to_string()), Some("Sin resultados")),
        ("No match".to_string(), CommandSourceAttr::Custom)
    );
    assert_eq!(
        resolve_empty_label(Some("".to_string()), Some("Sin resultados")),
        ("Sin resultados".to_string(), CommandSourceAttr::I18n)
    );
    assert_eq!(
        resolve_empty_label(None, None),
        (DEFAULT_EMPTY_LABEL.into(), CommandSourceAttr::Default)
    );

    assert_eq!(
        resolve_aria_label(Some("  Quick actions  ".to_string()), Some("Comandos")),
        ("Quick actions".to_string(), CommandSourceAttr::Custom)
    );
    assert_eq!(
        resolve_aria_label(Some("".to_string()), Some("Comandos")),
        ("Comandos".to_string(), CommandSourceAttr::I18n)
    );
    assert_eq!(
        resolve_aria_label(None, None),
        (DEFAULT_ARIA_LABEL.into(), CommandSourceAttr::Default)
    );

    assert_eq!(
        resolve_default_query(Some("  docs  ".to_string())),
        "docs".to_string()
    );
    assert_eq!(resolve_default_query(Some("".to_string())), DEFAULT_QUERY);
    assert_eq!(resolve_default_query(None), DEFAULT_QUERY);
    assert!(!has_query_text(" \n\t "));
    assert!(has_query_text(" cmd "));
}

#[test]
fn filter_groups_keeps_group_shape_and_matches_keywords() {
    let groups = sample_groups();

    let all = filter_groups(&groups, "");
    assert_eq!(all.items.len(), 4);
    assert_eq!(all.groups.len(), 2);
    assert_eq!(all.groups[0].item_indices, vec![0, 1]);
    assert_eq!(all.groups[1].item_indices, vec![2, 3]);

    let by_label = filter_groups(&groups, "cal");
    assert_eq!(by_label.items.len(), 1);
    assert_eq!(by_label.groups.len(), 1);
    assert_eq!(by_label.items[0].id, "calendar");

    let by_keyword = filter_groups(&groups, "icon");
    assert_eq!(by_keyword.items.len(), 1);
    assert_eq!(by_keyword.items[0].id, "search");

    let by_id = filter_groups(&groups, "bill");
    assert_eq!(by_id.items.len(), 1);
    assert_eq!(by_id.items[0].id, "billing");
}

#[test]
fn resolve_state_and_class_contracts_are_stable() {
    let state = resolve_state(CommandPartStateInput {
        slot: CommandSlot::Root,
        item_count: 0,
        group_count: 0,
        is_disabled: true,
        has_query: true,
        has_custom_id_base: true,
        has_custom_placeholder: true,
        has_i18n_placeholder: false,
        has_custom_empty_label: true,
        has_i18n_empty_label: false,
        has_custom_aria_label: true,
        has_i18n_aria_label: false,
        has_custom_class_name: true,
        has_custom_disabled: true,
        has_custom_on_action: true,
        has_custom_motion: true,
        is_query_controlled: false,
        has_custom_default_query: false,
        has_custom_query_change_handler: false,
    });

    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(state.is_disabled);
    assert!(state.has_query);
    assert_eq!(state.state_attr, CommandRootStateAttr::DisabledEmpty);
    assert_eq!(state.query_attr, CommandQueryAttr::Present);
    assert_eq!(state.item_attr, CommandCollectionAttr::Empty);
    assert_eq!(state.disabled_source_attr, CommandSourceAttr::Custom);
    assert_eq!(state.aria_label_source_attr, CommandSourceAttr::Custom);
    assert_eq!(
        state.query_control_attr.as_attr(),
        "uncontrolled",
        "query control marker should expose closed-set `uncontrolled`."
    );
    assert_eq!(
        state.query_default_source_attr.as_attr(),
        "empty",
        "query default source marker should expose closed-set `empty`."
    );
    assert_eq!(
        state.query_change_source_attr.as_attr(),
        "none",
        "query change source marker should expose closed-set `none`."
    );

    let class_name = compose_class_name(Some("docs-command".to_string()), state);

    for needle in [
        "ui-command",
        "ui-command--empty",
        "ui-command--disabled",
        "ui-command--querying",
        "ui-command--custom-id",
        "ui-command--custom-placeholder",
        "ui-command--custom-empty-label",
        "ui-command--custom-aria-label",
        "ui-command--custom-class",
        "ui-command--custom-disabled",
        "ui-command--custom-action",
        "ui-command--custom-motion",
        "docs-command",
    ] {
        assert!(
            class_name.contains(needle),
            "composed class list should include `{needle}`"
        );
    }
}

#[test]
fn resolve_root_state_centralizes_query_presence_derivation() {
    let state = resolve_root_state(CommandRootStateInput {
        item_count: 1,
        group_count: 1,
        is_disabled: false,
        query: "   ",
        has_custom_id_base: false,
        has_custom_placeholder: false,
        has_i18n_placeholder: false,
        has_custom_empty_label: false,
        has_i18n_empty_label: false,
        has_custom_aria_label: false,
        has_i18n_aria_label: false,
        has_custom_class_name: false,
        has_custom_disabled: false,
        has_custom_on_action: false,
        has_custom_motion: false,
        is_query_controlled: false,
        has_custom_default_query: false,
        has_custom_query_change_handler: false,
    });

    assert!(!state.has_query);
    assert_eq!(state.query_attr, CommandQueryAttr::Absent);
    assert_eq!(state.state_attr, CommandRootStateAttr::Default);

    let state = resolve_root_state(CommandRootStateInput {
        item_count: 1,
        group_count: 1,
        is_disabled: false,
        query: "docs",
        has_custom_id_base: false,
        has_custom_placeholder: false,
        has_i18n_placeholder: false,
        has_custom_empty_label: false,
        has_i18n_empty_label: false,
        has_custom_aria_label: false,
        has_i18n_aria_label: false,
        has_custom_class_name: false,
        has_custom_disabled: false,
        has_custom_on_action: false,
        has_custom_motion: false,
        is_query_controlled: false,
        has_custom_default_query: false,
        has_custom_query_change_handler: false,
    });

    assert!(state.has_query);
    assert_eq!(state.query_attr, CommandQueryAttr::Present);
    assert_eq!(state.state_attr, CommandRootStateAttr::QueryResults);
}

#[test]
fn resolve_root_state_exposes_query_control_source_markers() {
    let state = resolve_root_state(CommandRootStateInput {
        item_count: 1,
        group_count: 1,
        is_disabled: false,
        query: "docs",
        has_custom_id_base: false,
        has_custom_placeholder: false,
        has_i18n_placeholder: false,
        has_custom_empty_label: false,
        has_i18n_empty_label: false,
        has_custom_aria_label: false,
        has_i18n_aria_label: false,
        has_custom_class_name: false,
        has_custom_disabled: false,
        has_custom_on_action: false,
        has_custom_motion: false,
        is_query_controlled: true,
        has_custom_default_query: true,
        has_custom_query_change_handler: true,
    });

    assert_eq!(state.query_control_attr.as_attr(), "controlled");
    assert_eq!(state.query_default_source_attr.as_attr(), "provided");
    assert_eq!(state.query_change_source_attr.as_attr(), "provided");
}

#[test]
fn invalid_input_state_is_normalized_in_logic_layer() {
    assert_eq!(
        normalize_selected_index(Some(3), 2),
        Some(0),
        "out-of-range selected index should be normalized to first selectable item."
    );
    assert_eq!(
        normalize_selected_index(None, 2),
        Some(0),
        "missing selected index should normalize to first selectable item when items exist."
    );
    assert_eq!(
        normalize_selected_index(Some(0), 0),
        None,
        "selected index must normalize to none when item set is empty."
    );
    assert_eq!(
        normalize_selected_index(None, 0),
        None,
        "missing selected index stays none when item set is empty."
    );
}
