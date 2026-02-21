use super::*;
use crate::selection::{OnSingleSelectionChange, SelectedKey};
use std::sync::{Arc, Mutex};

fn keys(values: &[&str]) -> Vec<Key> {
    values.iter().map(|v| (*v).to_string()).collect()
}

#[test]
fn uncontrolled_select_next_and_prev_wraps() {
    let mut state = use_list_state(ListStateOptions {
        items: keys(&["a", "b", "c"]),
        selection: SingleSelectionStateOptions::default(),
    });

    assert_eq!(state.selected_key_str(), None);

    state.select_next();
    assert_eq!(state.selected_key_str(), Some("a"));

    state.select_next();
    assert_eq!(state.selected_key_str(), Some("b"));

    state.select_prev();
    assert_eq!(state.selected_key_str(), Some("a"));

    state.select_prev();
    assert_eq!(state.selected_key_str(), Some("c"));
}

#[test]
fn controlled_selection_does_not_update_internal() {
    let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);
    let on_selection_change: OnSingleSelectionChange = Arc::new(move |v| {
        let mut guard = match called2.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = Some(v);
    });

    let mut state = use_list_state(ListStateOptions {
        items: keys(&["a", "b"]),
        selection: SingleSelectionStateOptions {
            selected_key: Some(SelectedKey::key("a")),
            on_selection_change: Some(on_selection_change),
            ..Default::default()
        },
    });

    state.select_next();
    let called_value = match called.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    assert_eq!(called_value, Some(SelectedKey::key("b")));

    // Still controlled by input until synced.
    assert_eq!(state.selected_key_str(), Some("a"));
}

#[test]
fn accessible_name_prefers_aria_label_then_labelledby_then_default() {
    let named = resolve_accessible_name(
        Some("  Fruit choices ".to_string()),
        Some("trigger-id".to_string()),
    );
    assert_eq!(named.aria_label.as_deref(), Some("Fruit choices"));
    assert_eq!(named.aria_labelledby, None);

    let labelled = resolve_accessible_name(None, Some("  trigger-id ".to_string()));
    assert_eq!(labelled.aria_label, None);
    assert_eq!(labelled.aria_labelledby.as_deref(), Some("trigger-id"));

    let fallback = resolve_accessible_name(Some("  ".to_string()), Some(" ".to_string()));
    assert_eq!(fallback.aria_label.as_deref(), Some(DEFAULT_ARIA_LABEL));
    assert_eq!(fallback.aria_labelledby, None);
}

#[test]
fn list_view_state_sanitizes_selection_bounds() {
    let selected = resolve_view_state(ListViewStateInput {
        item_count: 3,
        selected_index: Some(1),
        has_disabled_options: true,
    });
    assert!(selected.has_items);
    assert!(selected.has_selection);
    assert!(!selected.is_empty);
    assert!(selected.has_disabled_options);

    let out_of_range = resolve_view_state(ListViewStateInput {
        item_count: 2,
        selected_index: Some(8),
        has_disabled_options: false,
    });
    assert!(out_of_range.has_items);
    assert!(!out_of_range.has_selection);
}

#[test]
fn item_state_and_indicator_contracts_are_stable() {
    assert_eq!(
        resolve_item_selection_indicator(true),
        ListItemSelectionIndicator::Checkmark
    );
    assert_eq!(
        resolve_item_selection_indicator(false),
        ListItemSelectionIndicator::Hidden
    );
    assert_eq!(
        ListItemSelectionIndicator::Checkmark.marker(true),
        Some("✓")
    );

    let state = resolve_item_state(ListItemStateInput {
        selected: true,
        focused: false,
        disabled: true,
        show_selection_indicator: true,
        has_divider: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "disabled-selected");
    assert_eq!(state.selection_indicator_attr, "checkmark");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn section_state_tracks_tone_and_sources() {
    let state = resolve_section_state(ListSectionStateInput {
        heading_tone: ListSectionHeadingTone::Quiet,
        item_count: 0,
        disabled: true,
        sticky_heading: true,
        show_divider: true,
        has_title: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.heading_tone_attr, "quiet");
    assert_eq!(state.heading_tone_class, "ui-listbox-section--tone-quiet");
    assert!(state.is_empty);
    assert_eq!(state.data_state_attr, "disabled-empty");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.title_source_attr, "custom");
}
