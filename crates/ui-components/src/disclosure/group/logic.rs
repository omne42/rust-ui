use super::{DisclosureGroupState, DisclosureGroupStateInput};
use std::collections::BTreeSet;

pub const DEFAULT_ARIA_LABEL: &str = "DisclosureGroup";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DisclosureGroupSelectionMode {
    Single,
    #[default]
    Multiple,
}

impl DisclosureGroupSelectionMode {
    pub fn class_name(self) -> &'static str {
        match self {
            DisclosureGroupSelectionMode::Single => "ui-disclosure-group--selection-single",
            DisclosureGroupSelectionMode::Multiple => "ui-disclosure-group--selection-multiple",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DisclosureGroupSelectionMode::Single => "single",
            DisclosureGroupSelectionMode::Multiple => "multiple",
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

pub fn normalize_expanded_indices(
    mode: DisclosureGroupSelectionMode,
    expanded: &BTreeSet<usize>,
    item_count: usize,
) -> BTreeSet<usize> {
    let mut next = expanded
        .iter()
        .copied()
        .filter(|&index| index < item_count)
        .collect::<BTreeSet<_>>();

    if mode == DisclosureGroupSelectionMode::Single && next.len() > 1 {
        let first = next.iter().copied().next();
        next.clear();
        if let Some(first) = first {
            next.insert(first);
        }
    }

    next
}

pub fn resolve_state(input: DisclosureGroupStateInput) -> DisclosureGroupState {
    let has_items = input.item_count > 0;

    let mut expanded_count = input.expanded_count.min(input.item_count);
    if input.selection_mode == DisclosureGroupSelectionMode::Single {
        expanded_count = expanded_count.min(1);
    }

    let has_expanded_items = expanded_count > 0;
    let has_multiple_expanded = expanded_count > 1;

    let data_state_attr = if !has_items {
        "empty"
    } else if input.disabled {
        "disabled"
    } else if has_multiple_expanded {
        "multiple-expanded"
    } else if has_expanded_items {
        "expanded"
    } else {
        "collapsed"
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

    DisclosureGroupState {
        selection_mode: input.selection_mode,
        selection_mode_class: input.selection_mode.class_name(),
        selection_mode_attr: input.selection_mode.as_attr(),
        item_count: input.item_count,
        expanded_count,
        is_empty: !has_items,
        has_items,
        has_expanded_items,
        has_multiple_expanded,
        is_disabled: input.disabled,
        has_disabled_items: input.has_disabled_items,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DisclosureGroupState) -> String {
    let mut classes = vec![
        "ui-disclosure-group".to_string(),
        state.selection_mode_class.to_string(),
    ];

    if state.is_empty {
        classes.push("ui-disclosure-group--empty".to_string());
    }

    if state.is_disabled {
        classes.push("ui-disclosure-group--disabled".to_string());
    }

    if state.has_multiple_expanded {
        classes.push("ui-disclosure-group--multiple-expanded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-disclosure-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::DisclosureGroupStateInput;
    use super::*;

    #[test]
    fn selection_mode_contracts_are_stable() {
        assert_eq!(
            DisclosureGroupSelectionMode::Single.class_name(),
            "ui-disclosure-group--selection-single"
        );
        assert_eq!(
            DisclosureGroupSelectionMode::Multiple.class_name(),
            "ui-disclosure-group--selection-multiple"
        );
        assert_eq!(DisclosureGroupSelectionMode::Single.as_attr(), "single");
        assert_eq!(DisclosureGroupSelectionMode::Multiple.as_attr(), "multiple");
    }

    #[test]
    fn normalize_aria_label_falls_back_to_default() {
        assert_eq!(
            normalize_aria_label(Some("  Custom Group  ".to_string())),
            ("Custom Group".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("  \n  ".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn normalize_expanded_indices_clamps_and_normalizes() {
        let indices = BTreeSet::from([0, 3, 10]);

        let multiple =
            normalize_expanded_indices(DisclosureGroupSelectionMode::Multiple, &indices, 4);
        assert_eq!(multiple, BTreeSet::from([0, 3]));

        let single = normalize_expanded_indices(DisclosureGroupSelectionMode::Single, &indices, 4);
        assert_eq!(single, BTreeSet::from([0]));
    }

    #[test]
    fn resolve_state_tracks_empty_disabled_and_expanded_flags() {
        let empty_state = resolve_state(DisclosureGroupStateInput {
            selection_mode: DisclosureGroupSelectionMode::Multiple,
            item_count: 0,
            expanded_count: 10,
            disabled: false,
            has_disabled_items: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        assert!(empty_state.is_empty);
        assert_eq!(empty_state.data_state_attr, "empty");
        assert_eq!(empty_state.expanded_count, 0);

        let disabled_state = resolve_state(DisclosureGroupStateInput {
            selection_mode: DisclosureGroupSelectionMode::Multiple,
            item_count: 3,
            expanded_count: 2,
            disabled: true,
            has_disabled_items: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert!(!disabled_state.is_empty);
        assert!(disabled_state.has_expanded_items);
        assert!(disabled_state.has_multiple_expanded);
        assert_eq!(disabled_state.data_state_attr, "disabled");
        assert_eq!(disabled_state.aria_source_attr, "custom");
        assert_eq!(disabled_state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_reflects_state_flags() {
        let state = resolve_state(DisclosureGroupStateInput {
            selection_mode: DisclosureGroupSelectionMode::Multiple,
            item_count: 3,
            expanded_count: 2,
            disabled: true,
            has_disabled_items: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name =
            compose_class_name(Some("docs-disclosure-group-custom".to_string()), state);

        assert!(class_name.contains("ui-disclosure-group"));
        assert!(class_name.contains("ui-disclosure-group--selection-multiple"));
        assert!(class_name.contains("ui-disclosure-group--multiple-expanded"));
        assert!(class_name.contains("ui-disclosure-group--disabled"));
        assert!(class_name.contains("ui-disclosure-group--custom-class"));
        assert!(class_name.contains("docs-disclosure-group-custom"));
    }
}
