use crate::command::{CommandGroup, CommandItem, filter_groups, normalize_selected_index};

fn sample_groups() -> Vec<CommandGroup> {
    vec![
        CommandGroup::new(
            "Suggestions",
            vec![
                CommandItem::new("calendar", "Calendar")
                    .keywords(vec!["date".to_string(), "event".to_string()])
                    .shortcut("Ctrl+K"),
                CommandItem::new("search", "Search Emoji")
                    .keywords(vec!["emoji".to_string(), "icon".to_string()]),
            ],
        ),
        CommandGroup::new(
            "Settings",
            vec![
                CommandItem::new("billing", "Billing").shortcut("Ctrl+B"),
                CommandItem::new("admin", "Admin").disabled(true),
            ],
        ),
    ]
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
fn normalize_selected_index_preserves_in_range_or_resets() {
    assert_eq!(normalize_selected_index(Some(1), 2), Some(1));
    assert_eq!(normalize_selected_index(Some(3), 2), Some(0));
    assert_eq!(normalize_selected_index(None, 2), Some(0));
    assert_eq!(normalize_selected_index(Some(0), 0), None);
    assert_eq!(normalize_selected_index(None, 0), None);
}
