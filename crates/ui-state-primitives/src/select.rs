use std::collections::HashSet;

pub const DEFAULT_ID_BASE: &str = "select";
pub const DEFAULT_PLACEHOLDER: &str = "Select…";

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectStateInput {
    pub disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub disabled_option_count: usize,
    pub is_open: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub trigger_disabled: bool,
    pub is_open: bool,
    pub is_closed: bool,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub has_disabled_options: bool,
    pub disabled_option_count: usize,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| DEFAULT_PLACEHOLDER.to_string())
}

pub fn resolve_disabled_option_count(
    disabled_indices: &HashSet<usize>,
    item_count: usize,
) -> usize {
    disabled_indices
        .iter()
        .filter(|index| **index < item_count)
        .count()
}

pub fn resolve_state(input: SelectStateInput) -> SelectState {
    let has_items = input.item_count > 0;
    let selected_index = input
        .selected_index
        .filter(|index| *index < input.item_count);
    let has_selection = selected_index.is_some();

    SelectState {
        item_count: input.item_count,
        is_empty: !has_items,
        has_items,
        is_disabled: input.disabled,
        trigger_disabled: resolve_trigger_disabled(input.disabled, input.item_count),
        is_open: input.is_open,
        is_closed: !input.is_open,
        selected_index,
        has_selection,
        selection_empty: !has_selection,
        has_disabled_options: input.disabled_option_count > 0,
        disabled_option_count: input.disabled_option_count,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SelectState) -> String {
    let mut classes = vec!["ui-select".to_string()];

    if state.is_open {
        classes.push("ui-select--open".to_string());
    } else {
        classes.push("ui-select--closed".to_string());
    }

    if state.trigger_disabled {
        classes.push("ui-select--disabled".to_string());
    }

    if state.is_empty {
        classes.push("ui-select--empty".to_string());
    }

    if state.has_selection {
        classes.push("ui-select--has-selection".to_string());
    }

    if state.has_disabled_options {
        classes.push("ui-select--has-disabled-options".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-select--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-select--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    disabled || item_count == 0
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
    fn resolve_state_tracks_empty_and_disabled() {
        let state = resolve_state(SelectStateInput {
            disabled: true,
            item_count: 0,
            selected_index: Some(0),
            disabled_option_count: 0,
            is_open: false,
            has_custom_class_name: false,
            has_custom_motion: false,
        });

        assert!(state.is_empty);
        assert!(state.trigger_disabled);
        assert!(!state.has_selection);
        assert_eq!(state.selected_index, None);
    }

    #[test]
    fn resolve_state_normalizes_selection_and_markers() {
        let state = resolve_state(SelectStateInput {
            disabled: false,
            item_count: 3,
            selected_index: Some(1),
            disabled_option_count: 2,
            is_open: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.has_items);
        assert!(state.has_selection);
        assert_eq!(state.selected_index, Some(1));
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_collects_state_classes() {
        let class_name = compose_class_name(
            Some("docs-select".to_string()),
            resolve_state(SelectStateInput {
                disabled: true,
                item_count: 0,
                selected_index: None,
                disabled_option_count: 0,
                is_open: true,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for token in [
            "ui-select",
            "ui-select--open",
            "ui-select--disabled",
            "ui-select--empty",
            "ui-select--custom-motion",
            "ui-select--custom-class",
            "docs-select",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }

    #[test]
    fn resolve_disabled_option_count_ignores_out_of_range_indices() {
        let disabled = HashSet::from([0_usize, 2, 8]);
        assert_eq!(resolve_disabled_option_count(&disabled, 3), 2);
        assert_eq!(resolve_disabled_option_count(&disabled, 2), 1);
    }

    #[test]
    fn resolve_horizontal_nav_target_skips_disabled_items() {
        let disabled = HashSet::from([1_usize, 3]);

        assert_eq!(
            resolve_horizontal_nav_target(Some(0), SelectHorizontalNav::Next, 5, &disabled),
            Some(2)
        );
        assert_eq!(
            resolve_horizontal_nav_target(Some(4), SelectHorizontalNav::Previous, 5, &disabled),
            Some(2)
        );
        assert_eq!(
            resolve_horizontal_nav_target(None, SelectHorizontalNav::Next, 5, &disabled),
            Some(0)
        );
    }

    #[test]
    fn typeahead_char_accepts_single_ascii_alphanumeric_only() {
        assert_eq!(typeahead_char("A"), Some('a'));
        assert_eq!(typeahead_char("7"), Some('7'));
        assert_eq!(typeahead_char("中"), None);
        assert_eq!(typeahead_char("ab"), None);
    }

    #[test]
    fn find_typeahead_match_wraps_and_skips_disabled_items() {
        let items = vec![
            "Apple".to_string(),
            "Apricot".to_string(),
            "Banana".to_string(),
            "Blueberry".to_string(),
        ];
        let disabled = HashSet::from([1_usize]);

        assert_eq!(find_typeahead_match("ap", 0, &items, &disabled), Some(0));
        assert_eq!(find_typeahead_match("bl", 2, &items, &disabled), Some(3));
        assert_eq!(find_typeahead_match("ap", 1, &items, &disabled), Some(0));
        assert_eq!(find_typeahead_match("zz", 0, &items, &disabled), None);
    }
}
