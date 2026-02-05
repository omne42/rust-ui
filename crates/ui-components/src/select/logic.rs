use std::collections::HashSet;

pub struct SelectIds {
    pub trigger_id: String,
    pub listbox_id: String,
}

pub fn resolve_ids(id_base: &str) -> SelectIds {
    SelectIds {
        trigger_id: format!("{id_base}-trigger"),
        listbox_id: format!("{id_base}-listbox"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectOpenFocusStrategy {
    /// Default behavior: focus the selected option when opening.
    #[default]
    Selected,
    /// Focus the first enabled option when opening (keyboard "ArrowDown"/"Enter"/"Space" behavior).
    First,
    /// Focus the last enabled option when opening (keyboard "ArrowUp" behavior).
    Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectHorizontalNav {
    Previous,
    Next,
}

pub fn resolve_horizontal_nav_target(
    current: Option<usize>,
    direction: SelectHorizontalNav,
    item_count: usize,
    disabled: &HashSet<usize>,
) -> Option<usize> {
    if item_count == 0 {
        return None;
    }

    let is_enabled = |index: usize| !disabled.contains(&index);

    let current = current.filter(|&index| index < item_count);
    let Some(current) = current else {
        return (0..item_count).find(|&idx| is_enabled(idx));
    };

    match direction {
        SelectHorizontalNav::Previous => (0..current).rev().find(|&idx| is_enabled(idx)),
        SelectHorizontalNav::Next => ((current + 1)..item_count).find(|&idx| is_enabled(idx)),
    }
}

pub fn typeahead_char(key: &str) -> Option<char> {
    let mut chars = key.chars();
    let ch = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    if ch.is_ascii_alphanumeric() {
        Some(ch.to_ascii_lowercase())
    } else {
        None
    }
}

fn normalize_for_typeahead(text: &str) -> String {
    text.chars()
        .filter_map(|ch| {
            ch.is_ascii_alphanumeric()
                .then_some(ch.to_ascii_lowercase())
        })
        .collect()
}

pub fn find_typeahead_match(
    query: &str,
    start_index: usize,
    items: &[String],
    disabled: &HashSet<usize>,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }
    let query = query.trim().to_ascii_lowercase();
    if query.is_empty() {
        return None;
    }

    let count = items.len();
    let start = start_index.min(count.saturating_sub(1));

    for offset in 0..count {
        let index = (start + offset) % count;
        if disabled.contains(&index) {
            continue;
        }

        if normalize_for_typeahead(&items[index]).starts_with(&query) {
            return Some(index);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_nav_picks_first_enabled_when_no_selection() {
        let disabled = HashSet::from([0_usize]);
        assert_eq!(
            resolve_horizontal_nav_target(None, SelectHorizontalNav::Next, 3, &disabled),
            Some(1)
        );
        assert_eq!(
            resolve_horizontal_nav_target(None, SelectHorizontalNav::Previous, 3, &disabled),
            Some(1)
        );
    }

    #[test]
    fn horizontal_nav_moves_without_wrapping_and_skips_disabled() {
        let disabled = HashSet::from([1_usize]);
        assert_eq!(
            resolve_horizontal_nav_target(Some(0), SelectHorizontalNav::Next, 3, &disabled),
            Some(2)
        );
        assert_eq!(
            resolve_horizontal_nav_target(Some(2), SelectHorizontalNav::Next, 3, &disabled),
            None
        );
        assert_eq!(
            resolve_horizontal_nav_target(Some(2), SelectHorizontalNav::Previous, 3, &disabled),
            Some(0)
        );
        assert_eq!(
            resolve_horizontal_nav_target(Some(0), SelectHorizontalNav::Previous, 3, &disabled),
            None
        );
    }

    #[test]
    fn typeahead_char_accepts_single_ascii_alnum_and_lowercases() {
        assert_eq!(typeahead_char("A"), Some('a'));
        assert_eq!(typeahead_char("7"), Some('7'));
        assert_eq!(typeahead_char(" "), None);
        assert_eq!(typeahead_char("ab"), None);
        assert_eq!(typeahead_char("ArrowDown"), None);
    }

    #[test]
    fn typeahead_match_wraps_and_skips_disabled() {
        let items = vec![
            "Apple".to_string(),
            "Apricot".to_string(),
            "Banana".to_string(),
        ];
        let disabled = HashSet::from([0_usize]);

        // Starting from index 2, query "a" should wrap and pick Apricot (since Apple is disabled).
        assert_eq!(find_typeahead_match("a", 2, &items, &disabled), Some(1));
        assert_eq!(find_typeahead_match("ap", 2, &items, &disabled), Some(1));
    }

    #[test]
    fn typeahead_match_normalizes_whitespace() {
        let items = vec![
            "New South Wales".to_string(),
            "Northern Territory".to_string(),
        ];
        let disabled = HashSet::new();

        assert_eq!(
            find_typeahead_match("northernterr", 0, &items, &disabled),
            Some(1)
        );
    }

    #[test]
    fn ids_include_trigger_and_listbox_suffixes() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.trigger_id, "demo-trigger");
        assert_eq!(ids.listbox_id, "demo-listbox");
    }
}
