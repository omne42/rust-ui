use std::borrow::Cow;

use crate::{
    CarouselItem, CarouselItemResolved, CarouselItemStatus, CarouselOrientation, CarouselPartState,
    CarouselPartStateInput, CarouselSlot,
};
use ui_state_primitives::carousel as carousel_primitives;

pub const DEFAULT_ID_BASE: &str = carousel_primitives::DEFAULT_ID_BASE;
pub const DEFAULT_ARIA_LABEL: &str = carousel_primitives::DEFAULT_ARIA_LABEL;
pub const DEFAULT_CONTROLS_ARIA_LABEL: &str = "Carousel controls";
pub const DEFAULT_INDICATORS_ARIA_LABEL: &str = "Carousel indicators";
pub const DEFAULT_PREVIOUS_LABEL: &str = "Previous";
pub const DEFAULT_NEXT_LABEL: &str = "Next";
pub const DEFAULT_INDICATOR_ARIA_LABEL_TEMPLATE: &str = "Go to {title}";
pub const DEFAULT_ORIENTATION: CarouselOrientation = CarouselOrientation::Horizontal;
pub const DEFAULT_LOOP_NAVIGATION: bool = true;

#[cfg(test)]
pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool) -> &'static str {
    carousel_primitives::state_attr(item_count, has_selection, has_focus)
}

#[cfg(test)]
pub fn item_attr(item_count: usize) -> &'static str {
    carousel_primitives::item_attr(item_count)
}

#[cfg(test)]
pub fn selected_attr(has_selection: bool) -> &'static str {
    carousel_primitives::selected_attr(has_selection)
}

#[cfg(test)]
pub fn focus_attr(has_focus: bool) -> &'static str {
    carousel_primitives::focus_attr(has_focus)
}

#[cfg(test)]
pub fn navigation_attr(loop_navigation: bool) -> &'static str {
    carousel_primitives::navigation_attr(loop_navigation)
}

#[cfg(test)]
pub fn selection_mode_attr(is_controlled: bool) -> &'static str {
    carousel_primitives::selection_mode_attr(is_controlled)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    carousel_primitives::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    carousel_primitives::normalize_id_base(id_base)
}

#[cfg(test)]
pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    carousel_primitives::resolve_aria_label(value)
}

pub fn resolve_aria_label_with_fallback(value: Option<String>, fallback: &str) -> (String, bool) {
    let value = normalize_optional_text(value);
    if let Some(value) = value {
        return (value, true);
    }

    (fallback.trim().into(), false)
}

pub fn resolve_label_with_fallback(value: Option<String>, fallback: &str) -> (String, bool) {
    let value = normalize_optional_text(value);
    if let Some(value) = value {
        return (value, true);
    }

    (fallback.trim().into(), false)
}

pub fn resolve_indicator_aria_label(template: &str, title: &str) -> String {
    if template.contains("{title}") {
        return template.replace("{title}", title);
    }

    format!("{template} {title}")
}

pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>) -> Vec<CarouselItemResolved> {
    let primitive_items = items
        .into_iter()
        .map(|item| carousel_primitives::CarouselItemInput {
            id: item.id,
            title: item.title,
            description: item.description,
            disabled: item.disabled,
        })
        .collect();

    carousel_primitives::resolve_items(id_base, primitive_items)
        .into_iter()
        .map(|item| CarouselItemResolved {
            id: item.id,
            slide_dom_id: item.slide_dom_id,
            dot_dom_id: item.dot_dom_id,
            title: item.title,
            description: item.description,
            disabled: item.disabled,
        })
        .collect()
}

#[cfg(test)]
pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    carousel_primitives::sanitize_index(index, item_count)
}

pub fn resolve_default_selected_index(
    default_selected_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_default_selected_index(default_selected_index, &disabled_flags)
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
    let resolved = carousel_primitives::resolve_item_state_attrs(
        index,
        selected_index,
        focused_index,
        is_disabled,
    );
    let status = match resolved.status {
        carousel_primitives::CarouselItemStatus::Idle => CarouselItemStatus::Idle,
        carousel_primitives::CarouselItemStatus::Focused => CarouselItemStatus::Focused,
        carousel_primitives::CarouselItemStatus::Selected => CarouselItemStatus::Selected,
        carousel_primitives::CarouselItemStatus::Disabled => CarouselItemStatus::Disabled,
    };

    CarouselItemStateAttrs {
        status,
        selected_attr: resolved.selected_attr,
        focused_attr: resolved.focused_attr,
        disabled_attr: resolved.disabled_attr,
        is_selected: resolved.is_selected,
    }
}

pub fn can_item_receive_selection(is_disabled: bool) -> bool {
    carousel_primitives::can_item_receive_selection(is_disabled)
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

#[cfg(test)]
pub fn sanitize_focused_index(
    focused_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::sanitize_enabled_index(focused_index, &disabled_flags)
}

#[cfg(test)]
pub fn first_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::first_enabled_index(&disabled_flags)
}

#[cfg(test)]
pub fn last_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::last_enabled_index(&disabled_flags)
}

#[cfg(test)]
pub fn adjacent_enabled_index(
    items: &[CarouselItemResolved],
    current_index: usize,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::adjacent_enabled_index(&disabled_flags, current_index, step, should_loop)
}

#[cfg(test)]
pub fn resolve_initial_selected_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_initial_selected_index(&disabled_flags, selected_index)
}

#[cfg(test)]
const _: () = {
    let _ = sanitize_index as fn(Option<usize>, usize) -> Option<usize>;
    let _ = sanitize_focused_index as fn(Option<usize>, &[CarouselItemResolved]) -> Option<usize>;
    let _ = first_enabled_index as fn(&[CarouselItemResolved]) -> Option<usize>;
    let _ = last_enabled_index as fn(&[CarouselItemResolved]) -> Option<usize>;
    let _ =
        adjacent_enabled_index as fn(&[CarouselItemResolved], usize, isize, bool) -> Option<usize>;
    let _ = resolve_initial_selected_index
        as fn(&[CarouselItemResolved], Option<usize>) -> Option<usize>;
};

pub fn resolve_initial_focused_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_initial_focused_index(&disabled_flags, selected_index)
}

pub fn resolve_selected_index(
    selected_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_selected_index(&disabled_flags, selected_index)
}

pub fn resolve_focused_index(
    focused_index: Option<usize>,
    selected_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_focused_index(&disabled_flags, focused_index, selected_index)
}

pub fn step_selected_index(
    selected_index: Option<usize>,
    step: isize,
    loop_navigation: bool,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::step_selected_index(&disabled_flags, selected_index, step, loop_navigation)
}

pub fn edge_selected_index(pick_last: bool, items: &[CarouselItemResolved]) -> Option<usize> {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::edge_selected_index(&disabled_flags, pick_last)
}

pub fn can_step_selection(
    selected_index: Option<usize>,
    step: isize,
    loop_navigation: bool,
    items: &[CarouselItemResolved],
) -> bool {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::can_step_selection(&disabled_flags, selected_index, step, loop_navigation)
}

pub fn resolve_active_index(
    selected_index: Option<usize>,
    focused_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> usize {
    let disabled_flags = disabled_flags(items);
    carousel_primitives::resolve_active_index(&disabled_flags, selected_index, focused_index)
}

pub fn resolve_state(input: CarouselPartStateInput) -> CarouselPartState {
    let core =
        carousel_primitives::resolve_state_core(carousel_primitives::CarouselStateCoreInput {
            item_count: input.item_count,
            selected_index: input.selected_index,
            focused_index: input.focused_index,
            loop_navigation: input.loop_navigation,
            is_controlled: input.is_controlled,
            has_custom_id_base: input.has_custom_id_base,
            has_custom_aria_label: input.has_custom_aria_label,
            has_custom_class_name: input.has_custom_class_name,
            has_custom_orientation: input.has_custom_orientation,
            has_custom_loop_navigation: input.has_custom_loop_navigation,
            has_custom_selected_index: input.has_custom_selected_index,
            has_custom_default_selected_index: input.has_custom_default_selected_index,
            has_custom_on_selected_index_change: input.has_custom_on_selected_index_change,
            has_custom_motion: input.has_custom_motion,
        });

    CarouselPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: core.state_attr,
        item_attr: core.item_attr,
        selected_attr: core.selected_attr,
        focus_attr: core.focus_attr,
        orientation: input.orientation,
        orientation_attr: input.orientation.attr(),
        navigation_attr: core.navigation_attr,
        selection_mode_attr: core.selection_mode_attr,
        loop_attr: input.loop_navigation.then_some("true"),
        bounded_attr: (!input.loop_navigation).then_some("true"),
        item_count: input.item_count,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        is_empty: core.is_empty,
        has_items: core.has_items,
        has_selection: core.has_selection,
        has_focus: core.has_focus,
        has_disabled_items: input.has_disabled_items,
        loop_navigation: input.loop_navigation,
        is_controlled: core.is_controlled,
        is_uncontrolled: core.is_uncontrolled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_orientation: input.has_custom_orientation,
        has_custom_loop_navigation: input.has_custom_loop_navigation,
        has_custom_selected_index: input.has_custom_selected_index,
        has_custom_default_selected_index: input.has_custom_default_selected_index,
        has_custom_on_selected_index_change: input.has_custom_on_selected_index_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: core.id_source_attr,
        aria_label_source_attr: core.aria_label_source_attr,
        class_source_attr: core.class_source_attr,
        orientation_source_attr: core.orientation_source_attr,
        loop_navigation_source_attr: core.loop_navigation_source_attr,
        selected_index_source_attr: core.selected_index_source_attr,
        default_selected_index_source_attr: core.default_selected_index_source_attr,
        selected_index_change_source_attr: core.selected_index_change_source_attr,
        motion_source_attr: core.motion_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CarouselPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if matches!(state.slot, CarouselSlot::Root) {
        classes.push(Cow::Borrowed(state.orientation.class_name()));

        if state.is_empty {
            classes.push(Cow::Borrowed("ui-carousel--empty"));
        } else {
            classes.push(Cow::Borrowed("ui-carousel--has-items"));
        }

        if state.has_selection {
            classes.push(Cow::Borrowed("ui-carousel--selected"));
        } else {
            classes.push(Cow::Borrowed("ui-carousel--unselected"));
        }

        if state.has_focus {
            classes.push(Cow::Borrowed("ui-carousel--focused"));
        }

        if state.loop_navigation {
            classes.push(Cow::Borrowed("ui-carousel--loop"));
        } else {
            classes.push(Cow::Borrowed("ui-carousel--bounded"));
        }

        if state.has_disabled_items {
            classes.push(Cow::Borrowed("ui-carousel--has-disabled-items"));
        }

        if state.is_controlled {
            classes.push(Cow::Borrowed("ui-carousel--controlled"));
        } else {
            classes.push(Cow::Borrowed("ui-carousel--uncontrolled"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-carousel--custom-id"));
        }

        if state.has_custom_aria_label {
            classes.push(Cow::Borrowed("ui-carousel--custom-aria-label"));
        }

        if state.has_custom_orientation {
            classes.push(Cow::Borrowed("ui-carousel--custom-orientation"));
        }

        if state.has_custom_loop_navigation {
            classes.push(Cow::Borrowed("ui-carousel--custom-loop-navigation"));
        }

        if state.has_custom_selected_index {
            classes.push(Cow::Borrowed("ui-carousel--custom-selected-index"));
        }

        if state.has_custom_default_selected_index {
            classes.push(Cow::Borrowed("ui-carousel--custom-default-selected-index"));
        }

        if state.has_custom_on_selected_index_change {
            classes.push(Cow::Borrowed("ui-carousel--custom-selected-index-change"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-carousel--custom-motion"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-carousel--custom-class"));
            if let Some(base_class_name) = normalize_optional_text(base_class_name) {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(Cow::Owned(base_class_name));
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentSchemaVersion {
    V1,
}

impl CarouselAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentIntent {
    NavigateSlides,
}

impl CarouselAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NavigateSlides => "navigate-slides",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentAction {
    Navigate,
    Hold,
}

impl CarouselAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Navigate => "navigate",
            Self::Hold => "hold",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentStateAxis {
    Empty,
    Idle,
    Focused,
    Selected,
}

impl CarouselAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Idle => "idle",
            Self::Focused => "focused",
            Self::Selected => "selected",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentSourceAxis {
    ControlledExternal,
    UncontrolledDefault,
    UncontrolledImplicitDefault,
}

impl CarouselAgentSourceAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledExternal => "controlled-external",
            Self::UncontrolledDefault => "uncontrolled-default",
            Self::UncontrolledImplicitDefault => "uncontrolled-implicit-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentConfigPolicy {
    WhitelistOnly,
}

impl CarouselAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WhitelistOnly => "whitelist-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselAgentOutputStatus {
    Verified,
}

impl CarouselAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CarouselAgentSchemaVersion,
    pub intent: CarouselAgentIntent,
    pub action: CarouselAgentAction,
    pub state: CarouselAgentStateAxis,
    pub source: CarouselAgentSourceAxis,
    pub config_policy: CarouselAgentConfigPolicy,
    pub output_status: CarouselAgentOutputStatus,
}

pub fn resolve_agent_state_axis(state: CarouselPartState) -> CarouselAgentStateAxis {
    if state.is_empty {
        CarouselAgentStateAxis::Empty
    } else if state.has_selection {
        CarouselAgentStateAxis::Selected
    } else if state.has_focus {
        CarouselAgentStateAxis::Focused
    } else {
        CarouselAgentStateAxis::Idle
    }
}

pub fn resolve_agent_action(state: CarouselPartState) -> CarouselAgentAction {
    if state.has_items {
        CarouselAgentAction::Navigate
    } else {
        CarouselAgentAction::Hold
    }
}

pub fn resolve_agent_source_axis(state: CarouselPartState) -> CarouselAgentSourceAxis {
    if state.is_controlled {
        CarouselAgentSourceAxis::ControlledExternal
    } else if state.has_custom_default_selected_index {
        CarouselAgentSourceAxis::UncontrolledDefault
    } else {
        CarouselAgentSourceAxis::UncontrolledImplicitDefault
    }
}

pub fn resolve_agent_contract(state: CarouselPartState) -> CarouselAgentContract {
    CarouselAgentContract {
        schema_name: "ui.carousel.agent",
        schema_version: CarouselAgentSchemaVersion::V1,
        intent: CarouselAgentIntent::NavigateSlides,
        action: resolve_agent_action(state),
        state: resolve_agent_state_axis(state),
        source: resolve_agent_source_axis(state),
        config_policy: CarouselAgentConfigPolicy::WhitelistOnly,
        output_status: CarouselAgentOutputStatus::Verified,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
