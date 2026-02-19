#[cfg(feature = "component-toggle_group")]
use super::{ToggleGroupItem, ToggleGroupState, ToggleGroupStateInput};
use super::{ToggleSize, ToggleState, ToggleStateInput, ToggleVariant};
#[cfg(feature = "component-toggle_group")]
use std::collections::BTreeSet;

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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
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
pub const DEFAULT_ARIA_LABEL: &str = "Toggle group";

#[cfg(feature = "component-toggle_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[cfg(feature = "component-toggle_group")]
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

#[cfg(feature = "component-toggle_group")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleGroupSelectionMode {
    #[default]
    Multiple,
    Single,
}

#[cfg(feature = "component-toggle_group")]
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

#[cfg(feature = "component-toggle_group")]
pub fn normalize_toggle_group_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

#[cfg(feature = "component-toggle_group")]
pub fn normalize_toggle_group_items(items: Vec<ToggleGroupItem>) -> Vec<ToggleGroupItem> {
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

#[cfg(feature = "component-toggle_group")]
pub fn collect_toggle_group_item_ids(items: &[ToggleGroupItem]) -> BTreeSet<String> {
    items.iter().map(|item| item.id.clone()).collect()
}

#[cfg(feature = "component-toggle_group")]
fn toggle_group_item_is_disabled(id: &str, items: &[ToggleGroupItem]) -> bool {
    items
        .iter()
        .find(|item| item.id == id)
        .map(|item| item.disabled)
        .unwrap_or(true)
}

#[cfg(feature = "component-toggle_group")]
pub fn sanitize_toggle_group_selected_ids(
    selected_ids: BTreeSet<String>,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
) -> BTreeSet<String> {
    let mut selected_ids: BTreeSet<String> = selected_ids
        .into_iter()
        .filter(|id| item_ids.contains(id) && !toggle_group_item_is_disabled(id, items))
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

#[cfg(feature = "component-toggle_group")]
pub fn toggle_toggle_group_selected_id(
    selected_ids: BTreeSet<String>,
    id: &str,
    item_ids: &BTreeSet<String>,
    items: &[ToggleGroupItem],
    selection_mode: ToggleGroupSelectionMode,
    next_selected: bool,
) -> BTreeSet<String> {
    if !item_ids.contains(id) || toggle_group_item_is_disabled(id, items) {
        return selected_ids;
    }

    match selection_mode {
        ToggleGroupSelectionMode::Single => {
            if next_selected {
                BTreeSet::from([id.into()])
            } else {
                BTreeSet::new()
            }
        }
        ToggleGroupSelectionMode::Multiple => {
            let mut next = selected_ids;
            if next_selected {
                next.insert(id.into());
            } else {
                next.remove(id);
            }
            next
        }
    }
}

#[cfg(feature = "component-toggle_group")]
pub fn resolve_toggle_group_state(input: ToggleGroupStateInput) -> ToggleGroupState {
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
mod tests {
    use super::*;

    #[test]
    fn state_and_interaction_attrs_follow_contract() {
        assert_eq!(state_attr_for_selected(true), "selected");
        assert_eq!(state_attr_for_selected(false), "unselected");

        assert_eq!(
            interaction_attr(true, false, false, false, false),
            "disabled"
        );
        assert_eq!(interaction_attr(false, true, true, true, true), "pressed");
        assert_eq!(interaction_attr(false, false, true, true, true), "hovered");
        assert_eq!(
            interaction_attr(false, false, false, true, true),
            "focus-visible"
        );
        assert_eq!(
            interaction_attr(false, false, false, false, true),
            "focused"
        );
        assert_eq!(interaction_attr(false, false, false, false, false), "idle");
    }

    #[test]
    fn variant_and_size_attrs_match_contract() {
        assert_eq!(variant_attr(ToggleVariant::Default), "default");
        assert_eq!(variant_attr(ToggleVariant::Accent), "accent");
        assert_eq!(variant_attr(ToggleVariant::Destructive), "destructive");
        assert_eq!(variant_attr(ToggleVariant::Outline), "outline");
        assert_eq!(variant_attr(ToggleVariant::Secondary), "secondary");
        assert_eq!(variant_attr(ToggleVariant::Ghost), "ghost");

        assert_eq!(size_attr(ToggleSize::Xs), "xs");
        assert_eq!(size_attr(ToggleSize::S), "s");
        assert_eq!(size_attr(ToggleSize::M), "m");
        assert_eq!(size_attr(ToggleSize::L), "l");
        assert_eq!(size_attr(ToggleSize::Xl), "xl");
        assert_eq!(size_attr(ToggleSize::IconXs), "icon-xs");
        assert_eq!(size_attr(ToggleSize::IconS), "icon-s");
        assert_eq!(size_attr(ToggleSize::IconM), "icon-m");
        assert_eq!(size_attr(ToggleSize::IconL), "icon-l");
        assert_eq!(size_attr(ToggleSize::IconXl), "icon-xl");

        assert_eq!(size_attr(ToggleSize::Default), "m");
        assert_eq!(size_attr(ToggleSize::Sm), "s");
        assert_eq!(size_attr(ToggleSize::Lg), "l");
        assert_eq!(size_attr(ToggleSize::Icon), "icon-m");
        assert_eq!(size_attr(ToggleSize::IconSm), "icon-s");
        assert_eq!(size_attr(ToggleSize::IconLg), "icon-l");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   \n".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-toggle  ".to_string())),
            Some("docs-toggle".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_source_and_interaction_markers() {
        let state = resolve_state(ToggleStateInput {
            selected: true,
            disabled: false,
            hovered: true,
            pressed_interaction: false,
            focused: true,
            focus_visible: true,
            variant: ToggleVariant::Outline,
            size: ToggleSize::Sm,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_aria_label: true,
            has_on_pressed_change: true,
        });

        assert_eq!(state.state_attr, "selected");
        assert_eq!(state.interaction_attr, "hovered");
        assert_eq!(state.variant_attr, "outline");
        assert_eq!(state.size_attr, "s");
        assert_eq!(state.variant_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.handler_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-toggle".to_string()),
            resolve_state(ToggleStateInput {
                selected: false,
                disabled: false,
                hovered: false,
                pressed_interaction: false,
                focused: false,
                focus_visible: false,
                variant: ToggleVariant::Outline,
                size: ToggleSize::Sm,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_aria_label: false,
                has_on_pressed_change: false,
            }),
        );

        for token in [
            "ui-toggle",
            "ui-toggle-button",
            "ui-toggle-button--variant-outline",
            "ui-toggle-button--size-s",
            "ui-toggle--custom-motion",
            "ui-toggle--custom-class",
            "docs-toggle",
        ] {
            assert!(
                class_name.contains(token),
                "toggle class name should include `{token}`"
            );
        }
    }

    #[cfg(feature = "component-toggle_group")]
    #[test]
    fn toggle_group_orientation_and_selection_mode_attrs_are_stable() {
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

    #[cfg(feature = "component-toggle_group")]
    #[test]
    fn normalize_and_sanitize_toggle_group_selected_ids_filter_unknown_and_disabled() {
        let items = normalize_toggle_group_items(vec![
            ToggleGroupItem::new("bold", "Bold"),
            ToggleGroupItem::new("italic", "Italic").disabled(true),
        ]);
        let item_ids = collect_toggle_group_item_ids(&items);

        let selected = BTreeSet::from([
            "bold".to_string(),
            "italic".to_string(),
            "missing".to_string(),
        ]);

        let selected = sanitize_toggle_group_selected_ids(
            selected,
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Multiple,
        );

        assert_eq!(selected, BTreeSet::from(["bold".to_string()]));
    }

    #[cfg(feature = "component-toggle_group")]
    #[test]
    fn toggle_toggle_group_selected_id_respects_selection_mode() {
        let items = normalize_toggle_group_items(vec![
            ToggleGroupItem::new("bold", "Bold"),
            ToggleGroupItem::new("italic", "Italic"),
        ]);
        let item_ids = collect_toggle_group_item_ids(&items);

        let selected = toggle_toggle_group_selected_id(
            BTreeSet::from(["bold".to_string()]),
            "italic",
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Single,
            true,
        );
        assert_eq!(selected, BTreeSet::from(["italic".to_string()]));

        let selected = toggle_toggle_group_selected_id(
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

        let selected = toggle_toggle_group_selected_id(
            BTreeSet::from(["bold".to_string(), "italic".to_string()]),
            "bold",
            &item_ids,
            &items,
            ToggleGroupSelectionMode::Multiple,
            false,
        );
        assert_eq!(selected, BTreeSet::from(["italic".to_string()]));
    }

    #[cfg(feature = "component-toggle_group")]
    #[test]
    fn compose_toggle_group_class_name_includes_state_and_custom_markers() {
        let state = resolve_toggle_group_state(ToggleGroupStateInput {
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

        let class_name =
            compose_toggle_group_class_name(Some("docs-toggle-group".to_string()), state);
        assert!(class_name.contains("ui-toggle-group"));
        assert!(class_name.contains("ui-toggle-group--vertical"));
        assert!(class_name.contains("ui-toggle-group--mode-single"));
        assert!(class_name.contains("ui-toggle-group--attached"));
        assert!(class_name.contains("ui-toggle-group--has-selection"));
        assert!(class_name.contains("ui-toggle-group--custom-class"));
        assert!(class_name.contains("docs-toggle-group"));
    }
}
