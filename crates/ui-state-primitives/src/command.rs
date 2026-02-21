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

pub fn normalize_selected_index(selected_index: Option<usize>, item_count: usize) -> Option<usize> {
    match selected_index {
        Some(index) if index < item_count => Some(index),
        _ if item_count == 0 => None,
        _ => Some(0),
    }
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

#[cfg(test)]
#[path = "test/command.rs"]
mod tests;
