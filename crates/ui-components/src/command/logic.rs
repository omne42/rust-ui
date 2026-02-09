#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub keywords: Vec<String>,
    pub shortcut: Option<String>,
    pub disabled: bool,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            keywords: Vec::new(),
            shortcut: None,
            disabled: false,
        }
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroup {
    pub heading: String,
    pub items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(heading: impl Into<String>, items: Vec<CommandItem>) -> Self {
        Self {
            heading: heading.into(),
            items,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilteredCommandItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilteredCommandGroup {
    pub heading: String,
    pub item_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommandFilterState {
    pub items: Vec<FilteredCommandItem>,
    pub groups: Vec<FilteredCommandGroup>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandStateInput {
    pub item_count: usize,
    pub group_count: usize,
    pub is_disabled: bool,
    pub has_query: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandState {
    pub item_count: usize,
    pub group_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_query: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_placeholder(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| "Type a command or search...".to_string())
}

pub fn normalize_empty_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| "No results found.".to_string())
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| "Command menu".to_string())
}

fn normalize_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn matches_query(item: &CommandItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let normalized = query.to_ascii_lowercase();

    item.label.to_ascii_lowercase().contains(&normalized)
        || item.id.to_ascii_lowercase().contains(&normalized)
        || item
            .keywords
            .iter()
            .any(|keyword| keyword.to_ascii_lowercase().contains(&normalized))
}

pub fn filter_groups(groups: &[CommandGroup], query: &str) -> CommandFilterState {
    let query = normalize_query(query);
    let mut state = CommandFilterState::default();

    for group in groups {
        let mut indices = Vec::new();

        for item in &group.items {
            if !matches_query(item, &query) {
                continue;
            }

            let index = state.items.len();
            state.items.push(FilteredCommandItem {
                id: item.id.clone(),
                label: item.label.clone(),
                shortcut: item.shortcut.clone(),
                disabled: item.disabled,
            });
            indices.push(index);
        }

        if indices.is_empty() {
            continue;
        }

        state.groups.push(FilteredCommandGroup {
            heading: group.heading.clone(),
            item_indices: indices,
        });
    }

    state
}

pub fn resolve_state(input: CommandStateInput) -> CommandState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    let data_state_attr = if input.is_disabled && is_empty {
        "disabled-empty"
    } else if input.is_disabled {
        "disabled"
    } else if is_empty && input.has_query {
        "query-empty"
    } else if is_empty {
        "empty"
    } else if input.has_query {
        "query-results"
    } else {
        "default"
    };

    CommandState {
        item_count: input.item_count,
        group_count: input.group_count,
        is_empty,
        has_items,
        is_disabled: input.is_disabled,
        has_query: input.has_query,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: CommandState) -> String {
    let mut classes = vec!["ui-command".to_string()];

    if state.is_empty {
        classes.push("ui-command--empty".to_string());
    }

    if state.is_disabled {
        classes.push("ui-command--disabled".to_string());
    }

    if state.has_query {
        classes.push("ui-command--querying".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-command--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

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
            normalize_placeholder(Some("  Find action  ".to_string())),
            "Find action".to_string()
        );
        assert_eq!(
            normalize_placeholder(Some("".to_string())),
            "Type a command or search...".to_string()
        );

        assert_eq!(
            normalize_empty_label(Some("  No match  ".to_string())),
            "No match".to_string()
        );
        assert_eq!(
            normalize_empty_label(Some("".to_string())),
            "No results found.".to_string()
        );

        assert_eq!(
            normalize_aria_label(Some("  Quick actions  ".to_string())),
            "Quick actions".to_string()
        );
        assert_eq!(
            normalize_aria_label(Some("".to_string())),
            "Command menu".to_string()
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
        let state = resolve_state(CommandStateInput {
            item_count: 0,
            group_count: 0,
            is_disabled: true,
            has_query: true,
            has_custom_class_name: true,
        });

        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(state.is_disabled);
        assert!(state.has_query);
        assert_eq!(state.data_state_attr, "disabled-empty");

        let class_name = compose_class_name(Some("docs-command".to_string()), state);

        for needle in [
            "ui-command",
            "ui-command--empty",
            "ui-command--disabled",
            "ui-command--querying",
            "ui-command--custom-class",
            "docs-command",
        ] {
            assert!(
                class_name.contains(needle),
                "composed class list should include `{needle}`"
            );
        }
    }
}
