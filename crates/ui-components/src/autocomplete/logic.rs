pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        "Options".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| "Type…".to_string())
}

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
    fn normalize_label_trims_and_defaults() {
        assert_eq!(normalize_label("  City  ".to_string()), "City");
        assert_eq!(normalize_label("   ".to_string()), "Options");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Pick a city  ".to_string())),
            Some("Pick a city".to_string())
        );
    }

    #[test]
    fn resolve_placeholder_uses_fallback() {
        assert_eq!(
            resolve_placeholder(Some("  Search  ".to_string())),
            "Search"
        );
        assert_eq!(resolve_placeholder(Some("   ".to_string())), "Type…");
        assert_eq!(resolve_placeholder(None), "Type…");
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
