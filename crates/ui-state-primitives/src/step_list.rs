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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListStateInput {
    pub orientation: StepListOrientation,
    pub size: StepListSize,
    pub emphasized: bool,
    pub disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub completed_count: usize,
    pub disabled_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListState {
    pub orientation: StepListOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub size: StepListSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub is_emphasized: bool,
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub completed_count: usize,
    pub disabled_count: usize,
    pub has_selection: bool,
    pub has_completed_steps: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub emphasis_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemStateInput {
    pub index: usize,
    pub selected_index: Option<usize>,
    pub completed: bool,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemState {
    pub index: usize,
    pub marker_number: usize,
    pub is_current: bool,
    pub is_completed: bool,
    pub is_disabled: bool,
    pub is_pending: bool,
    pub is_selectable: bool,
    pub status_attr: &'static str,
    pub status_class: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
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
        return fallback.into();
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

    (DEFAULT_ARIA_LABEL.into(), false)
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

pub fn is_completed_step(
    index: usize,
    selected_index: Option<usize>,
    completed_indices: &BTreeSet<usize>,
) -> bool {
    completed_indices.contains(&index) || selected_index.is_some_and(|selected| index < selected)
}

pub fn count_completed_steps(
    items: &[StepListItem],
    selected_index: Option<usize>,
    completed_indices: &BTreeSet<usize>,
) -> usize {
    items
        .iter()
        .enumerate()
        .filter(|(index, item)| {
            !item.disabled && is_completed_step(*index, selected_index, completed_indices)
        })
        .count()
}

pub fn count_disabled_steps(items: &[StepListItem]) -> usize {
    items.iter().filter(|item| item.disabled).count()
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
#[path = "test/step_list.rs"]
mod tests;
