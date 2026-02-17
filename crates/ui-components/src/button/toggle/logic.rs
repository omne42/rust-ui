use super::{ToggleSize, ToggleState, ToggleStateInput, ToggleVariant};

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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
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
        state.variant.class_name().to_string(),
        state.size.class_name().to_string(),
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
}
