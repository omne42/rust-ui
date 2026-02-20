use std::collections::BTreeSet;

use crate::{
    CarouselItem, CarouselItemResolved, CarouselOrientation, CarouselPartState,
    CarouselPartStateInput, CarouselSlot,
};
use ui_state_primitives::carousel as carousel_primitives;

pub const DEFAULT_ID_BASE: &str = "carousel";
pub const DEFAULT_ARIA_LABEL: &str = "Carousel";
pub const DEFAULT_ORIENTATION: CarouselOrientation = CarouselOrientation::Horizontal;
pub const DEFAULT_LOOP_NAVIGATION: bool = true;

pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool) -> &'static str {
    if item_count == 0 {
        "empty"
    } else if has_selection {
        "selected"
    } else if has_focus {
        "focused"
    } else {
        "idle"
    }
}

pub fn item_attr(item_count: usize) -> &'static str {
    if item_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn selected_attr(has_selection: bool) -> &'static str {
    if has_selection { "present" } else { "absent" }
}

pub fn focus_attr(has_focus: bool) -> &'static str {
    if has_focus { "present" } else { "absent" }
}

pub fn navigation_attr(loop_navigation: bool) -> &'static str {
    if loop_navigation { "loop" } else { "bounded" }
}

pub fn selection_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
            continue;
        }

        if (ch == '-' || ch == '_' || ch == ' ') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        return fallback.into();
    }

    out
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>) -> Vec<CarouselItemResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::new();

    for (index, item) in items.into_iter().enumerate() {
        let fallback_id = format!("slide-{}", index + 1);
        let raw_id = normalize_optional_text(Some(item.id)).unwrap_or_else(|| fallback_id.clone());
        let base_id = sanitize_token(&raw_id, &fallback_id);

        let mut unique_id = base_id.clone();
        let mut suffix = 2;
        while seen_ids.contains(&unique_id) {
            unique_id = format!("{base_id}-{suffix}");
            suffix += 1;
        }
        seen_ids.insert(unique_id.clone());

        let title = normalize_optional_text(Some(item.title))
            .unwrap_or_else(|| format!("Slide {}", index + 1));

        resolved.push(CarouselItemResolved {
            slide_dom_id: format!("{id_base}-{unique_id}-slide"),
            dot_dom_id: format!("{id_base}-{unique_id}-dot"),
            id: unique_id,
            title,
            description: normalize_optional_text(item.description),
            disabled: item.disabled,
        });
    }

    resolved
}

pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    carousel_primitives::sanitize_index(index, item_count)
}

fn disabled_flags(items: &[CarouselItemResolved]) -> Vec<bool> {
    items.iter().map(|item| item.disabled).collect()
}

pub fn sanitize_selected_index(
    selected_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::sanitize_enabled_index(selected_index, &disabled_flags)
}

pub fn sanitize_focused_index(
    focused_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::sanitize_enabled_index(focused_index, &disabled_flags)
}

pub fn first_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::first_enabled_index(&disabled_flags)
}

pub fn last_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::last_enabled_index(&disabled_flags)
}

pub fn adjacent_enabled_index(
    items: &[CarouselItemResolved],
    current_index: usize,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::adjacent_enabled_index(&disabled_flags, current_index, step, should_loop)
}

pub fn resolve_initial_selected_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_initial_selected_index(&disabled_flags, selected_index)
}

pub fn resolve_initial_focused_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_initial_focused_index(&disabled_flags, selected_index)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: CarouselPartStateInput) -> CarouselPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    CarouselPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.item_count, has_selection, has_focus),
        item_attr: item_attr(input.item_count),
        selected_attr: selected_attr(has_selection),
        focus_attr: focus_attr(has_focus),
        orientation: input.orientation,
        orientation_attr: input.orientation.attr(),
        navigation_attr: navigation_attr(input.loop_navigation),
        selection_mode_attr: selection_mode_attr(input.is_controlled),
        loop_attr: input.loop_navigation.then_some("true"),
        bounded_attr: (!input.loop_navigation).then_some("true"),
        item_count: input.item_count,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        is_empty,
        has_items,
        has_selection,
        has_focus,
        has_disabled_items: input.has_disabled_items,
        loop_navigation: input.loop_navigation,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_orientation: input.has_custom_orientation,
        has_custom_loop_navigation: input.has_custom_loop_navigation,
        has_custom_selected_index: input.has_custom_selected_index,
        has_custom_default_selected_index: input.has_custom_default_selected_index,
        has_custom_on_selected_index_change: input.has_custom_on_selected_index_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        orientation_source_attr: source_attr(input.has_custom_orientation),
        loop_navigation_source_attr: source_attr(input.has_custom_loop_navigation),
        selected_index_source_attr: source_attr(input.has_custom_selected_index),
        default_selected_index_source_attr: source_attr(input.has_custom_default_selected_index),
        selected_index_change_source_attr: source_attr(input.has_custom_on_selected_index_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CarouselPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, CarouselSlot::Root) {
        classes.push(state.orientation.class_name().into());

        if state.is_empty {
            classes.push("ui-carousel--empty".to_string());
        } else {
            classes.push("ui-carousel--has-items".to_string());
        }

        if state.has_selection {
            classes.push("ui-carousel--selected".to_string());
        } else {
            classes.push("ui-carousel--unselected".to_string());
        }

        if state.has_focus {
            classes.push("ui-carousel--focused".to_string());
        }

        if state.loop_navigation {
            classes.push("ui-carousel--loop".to_string());
        } else {
            classes.push("ui-carousel--bounded".to_string());
        }

        if state.has_disabled_items {
            classes.push("ui-carousel--has-disabled-items".to_string());
        }

        if state.is_controlled {
            classes.push("ui-carousel--controlled".to_string());
        } else {
            classes.push("ui-carousel--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-carousel--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-carousel--custom-aria-label".to_string());
        }

        if state.has_custom_orientation {
            classes.push("ui-carousel--custom-orientation".to_string());
        }

        if state.has_custom_loop_navigation {
            classes.push("ui-carousel--custom-loop-navigation".to_string());
        }

        if state.has_custom_selected_index {
            classes.push("ui-carousel--custom-selected-index".to_string());
        }

        if state.has_custom_default_selected_index {
            classes.push("ui-carousel--custom-default-selected-index".to_string());
        }

        if state.has_custom_on_selected_index_change {
            classes.push("ui-carousel--custom-selected-index-change".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-carousel--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-carousel--custom-class".to_string());
            if let Some(base_class_name) = normalize_optional_text(base_class_name) {
                classes.push(base_class_name);
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
