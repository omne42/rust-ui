pub fn filter_indices(items: &[String], query: &str, has_typed: bool) -> Vec<usize> {
    if !has_typed {
        return (0..items.len()).collect();
    }

    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return (0..items.len()).collect();
    }

    items
        .iter()
        .enumerate()
        .filter_map(|(idx, label)| label.to_ascii_lowercase().contains(&q).then_some(idx))
        .collect()
}

pub fn map_selected_to_filtered(
    selected_original: Option<usize>,
    filtered_original_indices: &[usize],
) -> Option<usize> {
    let selected = selected_original?;
    filtered_original_indices
        .iter()
        .position(|&idx| idx == selected)
}

pub fn map_filtered_to_original(
    filtered_index: usize,
    filtered_original_indices: &[usize],
) -> Option<usize> {
    filtered_original_indices.get(filtered_index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<String> {
        vec!["Apple".into(), "Banana".into(), "Apricot".into()]
    }

    #[test]
    fn filter_returns_all_when_not_typed_or_query_empty() {
        let items = items();
        assert_eq!(filter_indices(&items, "ap", false), vec![0, 1, 2]);
        assert_eq!(filter_indices(&items, " ", true), vec![0, 1, 2]);
    }

    #[test]
    fn filter_is_case_insensitive_contains() {
        let items = items();
        assert_eq!(filter_indices(&items, "ap", true), vec![0, 2]);
        assert_eq!(filter_indices(&items, "BAN", true), vec![1]);
    }

    #[test]
    fn selected_index_maps_to_filtered_position() {
        let filtered = vec![2, 0];
        assert_eq!(map_selected_to_filtered(Some(0), &filtered), Some(1));
        assert_eq!(map_selected_to_filtered(Some(2), &filtered), Some(0));
        assert_eq!(map_selected_to_filtered(Some(1), &filtered), None);
        assert_eq!(map_selected_to_filtered(None, &filtered), None);
    }

    #[test]
    fn filtered_to_original_maps_by_lookup() {
        let filtered = vec![2, 0];
        assert_eq!(map_filtered_to_original(0, &filtered), Some(2));
        assert_eq!(map_filtered_to_original(1, &filtered), Some(0));
        assert_eq!(map_filtered_to_original(2, &filtered), None);
    }
}
