use crate::toggle_group::{ToggleGroupItem, ToggleGroupState, ToggleGroupStateInput};
use std::collections::BTreeSet;

pub const DEFAULT_ARIA_LABEL: &str = "Toggle group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleGroupOrientation::Horizontal => "ui-toggle-group--horizontal",
            ToggleGroupOrientation::Vertical => "ui-toggle-group--vertical",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ToggleGroupOrientation::Horizontal => "horizontal",
            ToggleGroupOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupSelectionMode {
    #[default]
    Multiple,
    Single,
}

impl ToggleGroupSelectionMode {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleGroupSelectionMode::Multiple => "ui-toggle-group--mode-multiple",
            ToggleGroupSelectionMode::Single => "ui-toggle-group--mode-single",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ToggleGroupSelectionMode::Multiple => "multiple",
            ToggleGroupSelectionMode::Single => "single",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn normalize_items(items: Vec<ToggleGroupItem>) -> Vec<ToggleGroupItem> {
    items
        .into_iter()
        .enumerate()
        .map(|(index, mut item)| {
            let fallback_id = format!("toggle-{}", index + 1);
            item.id = normalize_optional_text(Some(item.id)).unwrap_or(fallback_id);
            item.label =
                normalize_optional_text(Some(item.label)).unwrap_or_else(|| item.id.clone());
            item
        })
        .collect()
}

pub fn collect_item_ids(items: &[ToggleGroupItem]) -> BTreeSet<String> {
    items.iter().map(|item| item.id.clone()).collect()
}

fn item_is_disabled(id: &str, items: &[ToggleGroupItem]) -> bool {
    items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.disabled)
        .unwrap_or(true)
}

pub fn sanitize_selected_ids(
    selected_ids: BTreeSet<String>,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
) -> BTreeSet<String> {
    let mut selected_ids: BTreeSet<String> = selected_ids
        .into_iter()
        .filter(|id| item_ids.contains(id) && !item_is_disabled(id, items))
        .collect();

    if matches!(selection_mode, ToggleGroupSelectionMode::Single) && selected_ids.len() > 1 {
        let first = selected_ids.iter().next().cloned();
        selected_ids.clear();
        if let Some(first) = first {
            selected_ids.insert(first);
        }
    }

    selected_ids
}

pub fn toggle_selected_id(
    selected_ids: BTreeSet<String>,
    id: &str,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
    next_selected: bool,
) -> BTreeSet<String> {
    if !item_ids.contains(id) || item_is_disabled(id, items) {
        return selected_ids;
    }

    match selection_mode {
        ToggleGroupSelectionMode::Single => {
            if next_selected {
                BTreeSet::from([id.to_string()])
            } else {
                BTreeSet::new()
            }
        }
        ToggleGroupSelectionMode::Multiple => {
            let mut next = selected_ids;
            if next_selected {
                next.insert(id.to_string());
            } else {
                next.remove(id);
            }
            next
        }
    }
}

pub fn resolve_state(input: ToggleGroupStateInput) -> ToggleGroupState {
    let has_selection = input.selected_count > 0;
    let is_empty = input.item_count == 0;
    let has_disabled_items = input.disabled_item_count > 0;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_empty {
        "empty"
    } else if has_selection {
        "selected"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ToggleGroupState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        selection_mode: input.selection_mode,
        selection_mode_class: input.selection_mode.class_name(),
        selection_mode_attr: input.selection_mode.as_attr(),
        is_disabled: input.disabled,
        is_attached: input.attached,
        item_count: input.item_count,
        selected_count: input.selected_count,
        disabled_item_count: input.disabled_item_count,
        has_selection,
        is_empty,
        has_disabled_items,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ToggleGroupState) -> String {
    let mut classes = vec![
        "ui-toggle-group".to_string(),
        state.orientation_class.to_string(),
        state.selection_mode_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-toggle-group--disabled".to_string());
    }
    if state.is_attached {
        classes.push("ui-toggle-group--attached".to_string());
    }
    if state.has_selection {
        classes.push("ui-toggle-group--has-selection".to_string());
    }
    if state.is_empty {
        classes.push("ui-toggle-group--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-toggle-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_and_selection_mode_attrs_are_stable() {
        assert_eq!(
            ToggleGroupOrientation::Horizontal.class_name(),
            "ui-toggle-group--horizontal"
        );
        assert_eq!(
            ToggleGroupOrientation::Vertical.class_name(),
            "ui-toggle-group--vertical"
        );
        assert_eq!(ToggleGroupOrientation::Horizontal.as_attr(), "horizontal");
        assert_eq!(ToggleGroupOrientation::Vertical.as_attr(), "vertical");

        assert_eq!(
            ToggleGroupSelectionMode::Multiple.class_name(),
            "ui-toggle-group--mode-multiple"
        );
        assert_eq!(
            ToggleGroupSelectionMode::Single.class_name(),
            "ui-toggle-group--mode-single"
        );
        assert_eq!(ToggleGroupSelectionMode::Multiple.as_attr(), "multiple");
        assert_eq!(ToggleGroupSelectionMode::Single.as_attr(), "single");
    }

    #[test]
    fn normalize_and_sanitize_selected_ids_filter_unknown_and_disabled() {
        let items = normalize_items(vec![
            ToggleGroupItem::new("bold", "Bold"),
            ToggleGroupItem::new("italic", "Italic").disabled(true),
        ]);
        let item_ids = collect_item_ids(&items);

        let selected = BTreeSet::from([
            "bold".to_string(),
            "italic".to_string(),
            "missing".to_string(),
        ]);

        let selected = sanitize_selected_ids(
            selected,
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Multiple,
        );

        assert_eq!(selected, BTreeSet::from(["bold".to_string()]));
    }

    #[test]
    fn toggle_selected_id_respects_selection_mode() {
        let items = normalize_items(vec![
            ToggleGroupItem::new("bold", "Bold"),
            ToggleGroupItem::new("italic", "Italic"),
        ]);
        let item_ids = collect_item_ids(&items);

        let selected = toggle_selected_id(
            BTreeSet::from(["bold".to_string()]),
            "italic",
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Single,
            true,
        );
        assert_eq!(selected, BTreeSet::from(["italic".to_string()]));

        let selected = toggle_selected_id(
            BTreeSet::from(["bold".to_string()]),
            "italic",
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Multiple,
            true,
        );
        assert_eq!(
            selected,
            BTreeSet::from(["bold".to_string(), "italic".to_string()])
        );

        let selected = toggle_selected_id(
            BTreeSet::from(["bold".to_string(), "italic".to_string()]),
            "bold",
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Multiple,
            false,
        );
        assert_eq!(selected, BTreeSet::from(["italic".to_string()]));
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(ToggleGroupStateInput {
            orientation: ToggleGroupOrientation::Vertical,
            selection_mode: ToggleGroupSelectionMode::Single,
            disabled: false,
            attached: true,
            item_count: 3,
            selected_count: 1,
            disabled_item_count: 1,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-toggle-group".to_string()), state);
        assert!(class_name.contains("ui-toggle-group"));
        assert!(class_name.contains("ui-toggle-group--vertical"));
        assert!(class_name.contains("ui-toggle-group--mode-single"));
        assert!(class_name.contains("ui-toggle-group--attached"));
        assert!(class_name.contains("ui-toggle-group--has-selection"));
        assert!(class_name.contains("ui-toggle-group--custom-class"));
        assert!(class_name.contains("docs-toggle-group"));
    }
}
