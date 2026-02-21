#[cfg(feature = "component-toggle_group")]
use super::{ToggleGroupItem, ToggleGroupState, ToggleGroupStateInput};
use super::{ToggleSize, ToggleState, ToggleStateInput, ToggleVariant};
#[cfg(feature = "component-toggle_group")]
use std::collections::BTreeSet;
use ui_state_primitives::button::normalize_optional_text as normalize_optional_text_primitive;
#[cfg(feature = "component-toggle_group")]
use ui_state_primitives::toggle_button as toggle_group_state;

#[cfg(feature = "component-toggle_group")]
pub const DEFAULT_ARIA_LABEL: &str = toggle_group_state::DEFAULT_TOGGLE_GROUP_ARIA_LABEL;
#[cfg(feature = "component-toggle_group")]
pub use toggle_group_state::{ToggleGroupOrientation, ToggleGroupSelectionMode};

pub fn state_attr_for_selected(selected: bool) -> &'static str {
    if selected { "selected" } else { "unselected" }
}

pub fn interaction_attr(
    disabled: bool,
    pressed: bool,
    hovered: bool,
    focus_visible: bool,
    focused: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if pressed {
        "pressed"
    } else if hovered {
        "hovered"
    } else if focus_visible {
        "focus-visible"
    } else if focused {
        "focused"
    } else {
        "idle"
    }
}

pub fn variant_attr(variant: ToggleVariant) -> &'static str {
    match variant {
        ToggleVariant::Default => "default",
        ToggleVariant::Accent => "accent",
        ToggleVariant::Destructive => "destructive",
        ToggleVariant::Outline => "outline",
        ToggleVariant::Secondary => "secondary",
        ToggleVariant::Ghost => "ghost",
    }
}

pub fn size_attr(size: ToggleSize) -> &'static str {
    match size {
        ToggleSize::Xs => "xs",
        ToggleSize::S => "s",
        ToggleSize::M => "m",
        ToggleSize::L => "l",
        ToggleSize::Xl => "xl",
        ToggleSize::IconXs => "icon-xs",
        ToggleSize::IconS => "icon-s",
        ToggleSize::IconM => "icon-m",
        ToggleSize::IconL => "icon-l",
        ToggleSize::IconXl => "icon-xl",
        ToggleSize::Default => "m",
        ToggleSize::Sm => "s",
        ToggleSize::Lg => "l",
        ToggleSize::Icon => "icon-m",
        ToggleSize::IconSm => "icon-s",
        ToggleSize::IconLg => "icon-l",
    }
}

pub fn resolve_state(input: ToggleStateInput) -> ToggleState {
    ToggleState {
        is_selected: input.selected,
        is_disabled: input.disabled,
        is_hovered: input.hovered,
        is_pressed: input.pressed_interaction,
        is_focused: input.focused,
        is_focus_visible: input.focus_visible,
        variant: input.variant,
        size: input.size,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_aria_label: input.has_custom_aria_label,
        has_on_pressed_change: input.has_on_pressed_change,
        state_attr: state_attr_for_selected(input.selected),
        interaction_attr: interaction_attr(
            input.disabled,
            input.pressed_interaction,
            input.hovered,
            input.focus_visible,
            input.focused,
        ),
        variant_attr: variant_attr(input.variant),
        size_attr: size_attr(input.size),
        variant_source_attr: if input.variant == ToggleVariant::default() {
            "default"
        } else {
            "custom"
        },
        size_source_attr: if input.size == ToggleSize::default() {
            "default"
        } else {
            "custom"
        },
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
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_on_pressed_change {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    normalize_optional_text_primitive(value)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ToggleState) -> String {
    let mut classes = vec![
        "ui-toggle".to_string(),
        "ui-toggle-button".to_string(),
        state.variant.class_name().into(),
        state.size.class_name().into(),
    ];

    if state.has_custom_motion {
        classes.push("ui-toggle--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-toggle--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(feature = "component-toggle_group")]
pub fn normalize_toggle_group_aria_label(value: Option<String>) -> (String, bool) {
    toggle_group_state::normalize_toggle_group_aria_label(value)
}

#[cfg(feature = "component-toggle_group")]
pub fn normalize_toggle_group_items(items: Vec<ToggleGroupItem>) -> Vec<ToggleGroupItem> {
    toggle_group_state::normalize_toggle_group_items(items)
}

#[cfg(feature = "component-toggle_group")]
pub fn collect_toggle_group_item_ids(items: &[ToggleGroupItem]) -> BTreeSet<String> {
    toggle_group_state::collect_toggle_group_item_ids(items)
}

#[cfg(feature = "component-toggle_group")]
pub fn sanitize_toggle_group_selected_ids(
    selected_ids: BTreeSet<String>,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
) -> BTreeSet<String> {
    toggle_group_state::sanitize_toggle_group_selected_ids(
        selected_ids,
        item_ids,
        items,
        selection_mode,
    )
}

#[cfg(feature = "component-toggle_group")]
pub fn normalize_toggle_group_default_selected_ids(
    default_selected_ids: Option<BTreeSet<String>>,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
) -> BTreeSet<String> {
    sanitize_toggle_group_selected_ids(
        default_selected_ids.unwrap_or_default(),
        item_ids,
        items,
        selection_mode,
    )
}

#[cfg(feature = "component-toggle_group")]
pub fn toggle_toggle_group_selected_id(
    selected_ids: BTreeSet<String>,
    id: &str,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
    next_selected: bool,
) -> BTreeSet<String> {
    toggle_group_state::toggle_toggle_group_selected_id(
        selected_ids,
        id,
        item_ids,
        items,
        selection_mode,
        next_selected,
    )
}

#[cfg(feature = "component-toggle_group")]
pub fn resolve_toggle_group_state(input: ToggleGroupStateInput) -> ToggleGroupState {
    toggle_group_state::resolve_toggle_group_state(input)
}

#[cfg(feature = "component-toggle_group")]
pub fn compose_toggle_group_class_name(
    base_class_name: Option<String>,
    state: ToggleGroupState,
) -> String {
    let mut classes = vec![
        "ui-toggle-group".to_string(),
        state.orientation_class.into(),
        state.selection_mode_class.into(),
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
#[path = "../../test/toggle/logic.rs"]
mod tests;
