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
                next.insert(id.to_string());
            }
            next
        }
        ActionGroupSelectionMode::Multiple => {
            let mut next = selected_ids;
            if !next.insert(id.to_string()) {
                next.remove(id);
            }
            next
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_item_ids_deduplicates_ids() {
        let ids = collect_item_ids(["b", "a", "a", "c"]);
        assert_eq!(
            ids,
            BTreeSet::from(["a".to_string(), "b".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn sanitize_selected_ids_enforces_single_selection_and_known_ids() {
        let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
        let selected = sanitize_selected_ids(
            BTreeSet::from(["b".to_string(), "a".to_string(), "ghost".to_string()]),
            &item_ids,
            ActionGroupSelectionMode::Single,
        );
        assert_eq!(selected, BTreeSet::from(["a".to_string()]));
    }

    #[test]
    fn sanitize_selected_ids_clears_when_mode_is_none() {
        let item_ids = BTreeSet::from(["a".to_string()]);
        let selected = sanitize_selected_ids(
            BTreeSet::from(["a".to_string()]),
            &item_ids,
            ActionGroupSelectionMode::None,
        );
        assert!(selected.is_empty());
    }

    #[test]
    fn toggle_selected_id_respects_modes() {
        let item_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);

        let single = toggle_selected_id(
            BTreeSet::new(),
            "a",
            &item_ids,
            ActionGroupSelectionMode::Single,
        );
        assert_eq!(single, BTreeSet::from(["a".to_string()]));
        let single = toggle_selected_id(single, "a", &item_ids, ActionGroupSelectionMode::Single);
        assert!(single.is_empty());

        let multiple = toggle_selected_id(
            BTreeSet::new(),
            "a",
            &item_ids,
            ActionGroupSelectionMode::Multiple,
        );
        assert_eq!(multiple, BTreeSet::from(["a".to_string()]));
        let multiple =
            toggle_selected_id(multiple, "a", &item_ids, ActionGroupSelectionMode::Multiple);
        assert!(multiple.is_empty());

        let none = toggle_selected_id(
            BTreeSet::from(["a".to_string()]),
            "b",
            &item_ids,
            ActionGroupSelectionMode::None,
        );
        assert!(none.is_empty());
    }
}
