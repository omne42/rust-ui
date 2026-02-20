use crate::{
    CommandFilterState, CommandGroup, CommandItem, CommandPartState, CommandPartStateInput,
    CommandSlot, FilteredCommandGroup, FilteredCommandItem,
};

pub const DEFAULT_ID_BASE: &str = "command";
pub const DEFAULT_PLACEHOLDER: &str = "Type a command or search...";
pub const DEFAULT_EMPTY_LABEL: &str = "No results found.";
pub const DEFAULT_ARIA_LABEL: &str = "Command menu";
pub const DEFAULT_DISABLED: bool = false;

pub fn state_attr(item_count: usize, is_disabled: bool, has_query: bool) -> &'static str {
    let is_empty = item_count == 0;

    if is_disabled && is_empty {
        "disabled-empty"
    } else if is_disabled {
        "disabled"
    } else if is_empty && has_query {
        "query-empty"
    } else if is_empty {
        "empty"
    } else if has_query {
        "query-results"
    } else {
        "default"
    }
}

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn group_attr(group_count: usize) -> &'static str {
    if group_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn query_attr(has_query: bool) -> &'static str {
    if has_query { "present" } else { "absent" }
}

pub fn disabled_attr(is_disabled: bool) -> &'static str {
    if is_disabled { "disabled" } else { "enabled" }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn resolve_placeholder(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_PLACEHOLDER.into(), false)
}

pub fn resolve_empty_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_EMPTY_LABEL.into(), false)
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: CommandPartStateInput) -> CommandPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    CommandPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.item_count, input.is_disabled, input.has_query),
        item_attr: item_attr(input.item_count),
        group_attr: group_attr(input.group_count),
        query_attr: query_attr(input.has_query),
        disabled_attr: disabled_attr(input.is_disabled),
        item_count: input.item_count,
        group_count: input.group_count,
        is_empty,
        has_items,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_query: input.has_query,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_empty_label: input.has_custom_empty_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_on_action: input.has_custom_on_action,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        placeholder_source_attr: source_attr(input.has_custom_placeholder),
        empty_label_source_attr: source_attr(input.has_custom_empty_label),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        action_source_attr: source_attr(input.has_custom_on_action),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(class_name: Option<String>, state: CommandPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, CommandSlot::Root) {
        if state.is_empty {
            classes.push("ui-command--empty".to_string());
        } else {
            classes.push("ui-command--has-items".to_string());
        }

        if state.is_disabled {
            classes.push("ui-command--disabled".to_string());
        } else {
            classes.push("ui-command--enabled".to_string());
        }

        if state.has_query {
            classes.push("ui-command--querying".to_string());
        } else {
            classes.push("ui-command--idle".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-command--custom-id".to_string());
        }

        if state.has_custom_placeholder {
            classes.push("ui-command--custom-placeholder".to_string());
        }

        if state.has_custom_empty_label {
            classes.push("ui-command--custom-empty-label".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-command--custom-aria-label".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-command--custom-disabled".to_string());
        }

        if state.has_custom_on_action {
            classes.push("ui-command--custom-action".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-command--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-command--custom-class".to_string());
            if let Some(class_name) = normalize_optional_text(class_name) {
                classes.push(class_name);
            }
        }
    } else if let Some(class_name) = normalize_optional_text(class_name) {
        classes.push(class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
