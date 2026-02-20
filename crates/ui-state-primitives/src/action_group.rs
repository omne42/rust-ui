use std::collections::BTreeSet;

pub type ActionGroupItemId = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionGroupSelectionMode {
    #[default]
    Single,
    Multiple,
    None,
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
