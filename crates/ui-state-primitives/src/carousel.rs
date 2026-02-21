use std::collections::BTreeSet;

pub use crate::button::normalize_optional_text;

pub const DEFAULT_ID_BASE: &str = "carousel";
pub const DEFAULT_ARIA_LABEL: &str = "Carousel";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItemInput {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItemResolved {
    pub id: String,
    pub slide_dom_id: String,
    pub dot_dom_id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselStateCoreInput {
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub loop_navigation: bool,
    pub is_controlled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_orientation: bool,
    pub has_custom_loop_navigation: bool,
    pub has_custom_selected_index: bool,
    pub has_custom_default_selected_index: bool,
    pub has_custom_on_selected_index_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselStateCore {
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub selected_attr: &'static str,
    pub focus_attr: &'static str,
    pub navigation_attr: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_focus: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub id_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub orientation_source_attr: &'static str,
    pub loop_navigation_source_attr: &'static str,
    pub selected_index_source_attr: &'static str,
    pub default_selected_index_source_attr: &'static str,
    pub selected_index_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

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

pub fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
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

pub fn resolve_items(id_base: &str, items: Vec<CarouselItemInput>) -> Vec<CarouselItemResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::with_capacity(items.len());

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

pub fn resolve_state_core(input: CarouselStateCoreInput) -> CarouselStateCore {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    CarouselStateCore {
        state_attr: state_attr(input.item_count, has_selection, has_focus),
        item_attr: item_attr(input.item_count),
        selected_attr: selected_attr(has_selection),
        focus_attr: focus_attr(has_focus),
        navigation_attr: navigation_attr(input.loop_navigation),
        selection_mode_attr: selection_mode_attr(input.is_controlled),
        is_empty,
        has_items,
        has_selection,
        has_focus,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
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

pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    index.filter(|index| *index < item_count)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselItemStatus {
    Idle,
    Focused,
    Selected,
    Disabled,
}

impl CarouselItemStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Focused => "focused",
            Self::Selected => "selected",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselItemStateAttrs {
    pub status: CarouselItemStatus,
    pub selected_attr: Option<&'static str>,
    pub focused_attr: Option<&'static str>,
    pub disabled_attr: Option<&'static str>,
    pub is_selected: bool,
}

pub fn resolve_item_state_attrs(
    index: usize,
    selected_index: Option<usize>,
    focused_index: Option<usize>,
    is_disabled: bool,
) -> CarouselItemStateAttrs {
    let is_selected = selected_index == Some(index);
    let is_focused = focused_index == Some(index);

    let status = if is_disabled {
        CarouselItemStatus::Disabled
    } else if is_selected {
        CarouselItemStatus::Selected
    } else if is_focused {
        CarouselItemStatus::Focused
    } else {
        CarouselItemStatus::Idle
    };

    CarouselItemStateAttrs {
        status,
        selected_attr: is_selected.then_some("true"),
        focused_attr: is_focused.then_some("true"),
        disabled_attr: is_disabled.then_some("true"),
        is_selected,
    }
}

pub fn can_item_receive_selection(is_disabled: bool) -> bool {
    !is_disabled
}

pub fn sanitize_enabled_index(index: Option<usize>, disabled_flags: &[bool]) -> Option<usize> {
    let index = sanitize_index(index, disabled_flags.len())?;
    (!disabled_flags[index]).then_some(index)
}

pub fn resolve_default_selected_index(
    default_selected_index: Option<usize>,
    disabled_flags: &[bool],
) -> Option<usize> {
    let requested_index = sanitize_index(default_selected_index, disabled_flags.len());
    resolve_initial_selected_index(disabled_flags, requested_index)
}

pub fn first_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().position(|disabled| !disabled)
}

pub fn last_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().rposition(|disabled| !disabled)
}

pub fn adjacent_enabled_index(
    disabled_flags: &[bool],
    current_index: usize,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    if disabled_flags.is_empty() || step == 0 {
        return None;
    }

    if should_loop {
        let len = disabled_flags.len() as isize;
        let mut cursor = current_index as isize;

        for _ in 0..disabled_flags.len().saturating_sub(1) {
            cursor = (cursor + step).rem_euclid(len);
            let index = cursor as usize;
            if !disabled_flags[index] {
                return Some(index);
            }
        }

        return None;
    }

    let mut cursor = current_index as isize;
    loop {
        cursor += step;
        if cursor < 0 || cursor >= disabled_flags.len() as isize {
            return None;
        }

        let index = cursor as usize;
        if !disabled_flags[index] {
            return Some(index);
        }
    }
}

pub fn resolve_initial_selected_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn resolve_initial_focused_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn resolve_selected_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn resolve_focused_index(
    disabled_flags: &[bool],
    focused_index: Option<usize>,
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(focused_index, disabled_flags)
        .or_else(|| sanitize_enabled_index(selected_index, disabled_flags))
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn step_selected_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    if step == 0 {
        return None;
    }

    let current_index = sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags));

    if let Some(current_index) = current_index {
        return adjacent_enabled_index(disabled_flags, current_index, step, should_loop);
    }

    if step > 0 {
        first_enabled_index(disabled_flags)
    } else {
        last_enabled_index(disabled_flags)
    }
}

pub fn edge_selected_index(disabled_flags: &[bool], pick_last: bool) -> Option<usize> {
    if pick_last {
        last_enabled_index(disabled_flags)
    } else {
        first_enabled_index(disabled_flags)
    }
}

pub fn can_step_selection(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
    step: isize,
    should_loop: bool,
) -> bool {
    step_selected_index(disabled_flags, selected_index, step, should_loop).is_some()
}

pub fn resolve_active_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
    focused_index: Option<usize>,
) -> usize {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| sanitize_enabled_index(focused_index, disabled_flags))
        .or_else(|| first_enabled_index(disabled_flags))
        .unwrap_or(0)
}

#[cfg(test)]
#[path = "test/carousel.rs"]
mod tests;
