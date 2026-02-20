use super::*;
use crate::{CommandPartStateInput, CommandSlot};

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
        resolve_placeholder(Some("  Find action  ".to_string())),
        ("Find action".to_string(), true)
    );
    assert_eq!(
        resolve_placeholder(Some("".to_string())),
        (DEFAULT_PLACEHOLDER.into(), false)
    );

    assert_eq!(
        resolve_empty_label(Some("  No match  ".to_string())),
        ("No match".to_string(), true)
    );
    assert_eq!(
        resolve_empty_label(Some("".to_string())),
        (DEFAULT_EMPTY_LABEL.into(), false)
    );

    assert_eq!(
        resolve_aria_label(Some("  Quick actions  ".to_string())),
        ("Quick actions".to_string(), true)
    );
    assert_eq!(
        resolve_aria_label(Some("".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
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
        has_custom_empty_label: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_disabled: true,
        has_custom_on_action: true,
        has_custom_motion: true,
    });

    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(state.is_disabled);
    assert!(state.has_query);
    assert_eq!(state.state_attr, "disabled-empty");
    assert_eq!(state.query_attr, "present");
    assert_eq!(state.disabled_source_attr, "custom");
    assert_eq!(state.aria_label_source_attr, "custom");

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
