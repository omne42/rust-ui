use crate::step_list::{
    StepListItemState, StepListItemStateInput, StepListState, StepListStateInput,
};
use std::collections::BTreeSet;

pub const DEFAULT_ARIA_LABEL: &str = "Steps";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StepListOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl StepListOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            StepListOrientation::Horizontal => "ui-step-list--orientation-horizontal",
            StepListOrientation::Vertical => "ui-step-list--orientation-vertical",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            StepListOrientation::Horizontal => "horizontal",
            StepListOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StepListSize {
    S,
    #[default]
    M,
    L,
    Xl,
}

impl StepListSize {
    pub fn class_name(self) -> &'static str {
        match self {
            StepListSize::S => "ui-step-list--size-s",
            StepListSize::M => "ui-step-list--size-m",
            StepListSize::L => "ui-step-list--size-l",
            StepListSize::Xl => "ui-step-list--size-xl",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            StepListSize::S => "s",
            StepListSize::M => "m",
            StepListSize::L => "l",
            StepListSize::Xl => "xl",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepListItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl StepListItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn described(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn sanitize_item_id(value: &str, fallback: &str) -> String {
    let mut id = String::new();
    let mut last_dash = false;

    for ch in value.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            id.push(ch.to_ascii_lowercase());
            last_dash = false;
            continue;
        }

        if matches!(ch, '-' | '_' | ' ') && !last_dash {
            id.push('-');
            last_dash = true;
        }
    }

    while id.ends_with('-') {
        id.pop();
    }

    if id.is_empty() {
        return fallback.to_string();
    }

    id
}

fn canonical_item_id(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub fn normalize_items(items: Vec<StepListItem>) -> Vec<StepListItem> {
    let mut seen_ids = BTreeSet::new();
    let mut out = Vec::new();

    for (index, item) in items.into_iter().enumerate() {
        let fallback_id = format!("step-{}", index + 1);
        let id_value =
            normalize_optional_text(Some(item.id)).unwrap_or_else(|| fallback_id.clone());
        let id = sanitize_item_id(&id_value, &fallback_id);

        if !seen_ids.insert(canonical_item_id(&id)) {
            continue;
        }

        let label = normalize_optional_text(Some(item.label))
            .unwrap_or_else(|| format!("Step {}", index + 1));

        out.push(StepListItem {
            id,
            label,
            description: normalize_optional_text(item.description),
            disabled: item.disabled,
        });
    }

    out
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    index.filter(|index| *index < item_count)
}

pub fn resolve_selected_index(
    items: &[StepListItem],
    selected_index: Option<usize>,
) -> Option<usize> {
    let index = sanitize_index(selected_index, items.len())?;
    (!items[index].disabled).then_some(index)
}

pub fn normalize_completed_indices(
    item_count: usize,
    completed_indices: Vec<usize>,
) -> BTreeSet<usize> {
    completed_indices
        .into_iter()
        .filter(|index| *index < item_count)
        .collect()
}

pub fn first_enabled_index(items: &[StepListItem]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

pub fn last_enabled_index(items: &[StepListItem]) -> Option<usize> {
    items.iter().rposition(|item| !item.disabled)
}

pub fn next_enabled_index(
    items: &[StepListItem],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    if step == 0 || items.is_empty() {
        return None;
    }

    let mut index = current_index as isize;
    loop {
        index += step;
        if index < 0 || index >= items.len() as isize {
            return None;
        }

        let next_index = index as usize;
        if !items[next_index].disabled {
            return Some(next_index);
        }
    }
}

pub fn resolve_state(input: StepListStateInput) -> StepListState {
    let has_selection = input.selected_index.is_some();
    let is_empty = input.item_count == 0;
    let has_completed_steps = input.completed_count > 0;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_empty {
        "empty"
    } else if let Some(selected_index) = input.selected_index {
        if selected_index + 1 >= input.item_count {
            "complete"
        } else {
            "active"
        }
    } else if has_completed_steps {
        "active"
    } else {
        "default"
    };

    StepListState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        is_emphasized: input.emphasized,
        is_disabled: input.disabled,
        item_count: input.item_count,
        selected_index: input.selected_index,
        completed_count: input.completed_count,
        disabled_count: input.disabled_count,
        has_selection,
        has_completed_steps,
        is_empty,
        data_state_attr,
        emphasis_attr: if input.emphasized { "true" } else { "false" },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: StepListState) -> String {
    let mut classes = vec![
        "ui-step-list".to_string(),
        state.orientation_class.to_string(),
        state.size_class.to_string(),
    ];

    if state.is_emphasized {
        classes.push("ui-step-list--emphasized".to_string());
    }

    if state.is_disabled {
        classes.push("ui-step-list--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-step-list--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_item_state(input: StepListItemStateInput) -> StepListItemState {
    let is_current = !input.disabled && input.selected_index == Some(input.index);
    let is_completed = !input.disabled && !is_current && input.completed;
    let is_pending = !input.disabled && !is_current && !is_completed;
    let is_selectable = !input.disabled && !is_current;

    let (status_attr, status_class) = if input.disabled {
        ("disabled", "ui-step-list__item--disabled")
    } else if is_current {
        ("current", "ui-step-list__item--current")
    } else if is_completed {
        ("completed", "ui-step-list__item--completed")
    } else {
        ("pending", "ui-step-list__item--pending")
    };

    StepListItemState {
        index: input.index,
        marker_number: input.index + 1,
        is_current,
        is_completed,
        is_disabled: input.disabled,
        is_pending,
        is_selectable,
        status_attr,
        status_class,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_and_size_contracts_are_stable() {
        assert_eq!(
            StepListOrientation::Horizontal.class_name(),
            "ui-step-list--orientation-horizontal"
        );
        assert_eq!(StepListOrientation::Vertical.as_attr(), "vertical");

        assert_eq!(StepListSize::S.class_name(), "ui-step-list--size-s");
        assert_eq!(StepListSize::M.as_attr(), "m");
        assert_eq!(StepListSize::L.class_name(), "ui-step-list--size-l");
        assert_eq!(StepListSize::Xl.as_attr(), "xl");
    }

    #[test]
    fn normalize_items_filters_duplicate_ids_and_sanitizes_content() {
        let items = vec![
            StepListItem::new(" Setup ", " Setup "),
            StepListItem::new("setup", "Duplicate"),
            StepListItem::new("", "").described("  Upload documents  "),
            StepListItem::new("final review", "Final review").disabled(true),
        ];

        let normalized = normalize_items(items);
        assert_eq!(normalized.len(), 3);

        assert_eq!(normalized[0].id, "setup");
        assert_eq!(normalized[0].label, "Setup");

        assert_eq!(normalized[1].id, "step-3");
        assert_eq!(normalized[1].label, "Step 3");
        assert_eq!(
            normalized[1].description,
            Some("Upload documents".to_string())
        );

        assert_eq!(normalized[2].id, "final-review");
        assert!(normalized[2].disabled);
    }

    #[test]
    fn resolve_selected_and_completed_indices_are_bounded() {
        let items = normalize_items(vec![
            StepListItem::new("a", "A"),
            StepListItem::new("b", "B").disabled(true),
            StepListItem::new("c", "C"),
        ]);

        assert_eq!(resolve_selected_index(&items, Some(0)), Some(0));
        assert_eq!(resolve_selected_index(&items, Some(1)), None);
        assert_eq!(resolve_selected_index(&items, Some(9)), None);

        let completed = normalize_completed_indices(items.len(), vec![0, 2, 5]);
        assert_eq!(completed.len(), 2);
        assert!(completed.contains(&0));
        assert!(completed.contains(&2));
    }

    #[test]
    fn navigation_helpers_skip_disabled_items() {
        let items = normalize_items(vec![
            StepListItem::new("a", "A"),
            StepListItem::new("b", "B").disabled(true),
            StepListItem::new("c", "C"),
            StepListItem::new("d", "D"),
        ]);

        assert_eq!(first_enabled_index(&items), Some(0));
        assert_eq!(last_enabled_index(&items), Some(3));
        assert_eq!(next_enabled_index(&items, 0, 1), Some(2));
        assert_eq!(next_enabled_index(&items, 3, -1), Some(2));
        assert_eq!(next_enabled_index(&items, 0, -1), None);
    }

    #[test]
    fn resolve_state_and_item_state_track_contracts() {
        let state = resolve_state(StepListStateInput {
            orientation: StepListOrientation::Vertical,
            size: StepListSize::L,
            emphasized: true,
            disabled: false,
            item_count: 4,
            selected_index: Some(2),
            completed_count: 2,
            disabled_count: 1,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation_attr, "vertical");
        assert_eq!(state.size_attr, "l");
        assert!(state.is_emphasized);
        assert_eq!(state.data_state_attr, "active");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");

        let item_state = resolve_item_state(StepListItemStateInput {
            index: 1,
            selected_index: Some(2),
            completed: true,
            disabled: false,
        });

        assert_eq!(item_state.marker_number, 2);
        assert!(item_state.is_completed);
        assert_eq!(item_state.status_attr, "completed");
        assert_eq!(item_state.status_class, "ui-step-list__item--completed");
        assert!(item_state.is_selectable);

        let disabled = resolve_item_state(StepListItemStateInput {
            index: 2,
            selected_index: Some(2),
            completed: true,
            disabled: true,
        });
        assert_eq!(disabled.status_attr, "disabled");
        assert!(!disabled.is_selectable);
    }

    #[test]
    fn compose_class_name_includes_sources() {
        let state = resolve_state(StepListStateInput {
            orientation: StepListOrientation::Horizontal,
            size: StepListSize::M,
            emphasized: true,
            disabled: true,
            item_count: 1,
            selected_index: Some(0),
            completed_count: 0,
            disabled_count: 0,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-step-list".to_string()), state);
        assert!(class_name.contains("ui-step-list"));
        assert!(class_name.contains("ui-step-list--orientation-horizontal"));
        assert!(class_name.contains("ui-step-list--size-m"));
        assert!(class_name.contains("ui-step-list--emphasized"));
        assert!(class_name.contains("ui-step-list--disabled"));
        assert!(class_name.contains("ui-step-list--custom-class"));
        assert!(class_name.contains("docs-step-list"));
    }
}
