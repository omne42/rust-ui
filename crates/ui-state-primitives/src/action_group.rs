use std::collections::BTreeSet;

pub type ActionGroupItemId = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl ActionGroupItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionGroupSelectionMode {
    #[default]
    Single,
    Multiple,
    None,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_items(items: Vec<ActionGroupItem>) -> Vec<ActionGroupItem> {
    items
        .into_iter()
        .enumerate()
        .map(|(index, mut item)| {
            let fallback_id = format!("action-{}", index + 1);
            item.id = normalize_optional_text(Some(item.id)).unwrap_or(fallback_id);
            item.label =
                normalize_optional_text(Some(item.label)).unwrap_or_else(|| item.id.clone());
            item
        })
        .collect()
}

pub fn collect_item_ids<'a>(
    item_ids: impl IntoIterator<Item = &'a str>,
) -> BTreeSet<ActionGroupItemId> {
    item_ids.into_iter().map(str::to_string).collect()
}

pub fn sanitize_selected_ids(
    selected_ids: BTreeSet<ActionGroupItemId>,
    item_ids: &BTreeSet<ActionGroupItemId>,
    selection_mode: ActionGroupSelectionMode,
) -> BTreeSet<ActionGroupItemId> {
    let mut selected_ids: BTreeSet<ActionGroupItemId> = selected_ids
        .into_iter()
        .filter(|id| item_ids.contains(id))
        .collect();

    match selection_mode {
        ActionGroupSelectionMode::None => BTreeSet::new(),
        ActionGroupSelectionMode::Single => {
            if selected_ids.len() <= 1 {
                return selected_ids;
            }

            let first = selected_ids.iter().next().cloned();
            selected_ids.clear();
            if let Some(first) = first {
                selected_ids.insert(first);
            }
            selected_ids
        }
        ActionGroupSelectionMode::Multiple => selected_ids,
    }
}

pub fn toggle_selected_id(
    selected_ids: BTreeSet<ActionGroupItemId>,
    id: &str,
    item_ids: &BTreeSet<ActionGroupItemId>,
    selection_mode: ActionGroupSelectionMode,
) -> BTreeSet<ActionGroupItemId> {
    if !item_ids.contains(id) {
        return selected_ids;
    }

    match selection_mode {
        ActionGroupSelectionMode::None => BTreeSet::new(),
        ActionGroupSelectionMode::Single => {
            let mut next = BTreeSet::new();
            if !selected_ids.contains(id) {
                next.insert(id.into());
            }
            next
        }
        ActionGroupSelectionMode::Multiple => {
            let mut next = selected_ids;
            if !next.insert(id.into()) {
                next.remove(id);
            }
            next
        }
    }
}

#[cfg(test)]
#[path = "test/action_group.rs"]
mod tests;
