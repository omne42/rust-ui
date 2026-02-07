#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonSize {
    XS,
    S,
    #[default]
    M,
    L,
    XL,
}

impl ActionButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonSize::XS => "ui-action-button--size-xs",
            ActionButtonSize::S => "ui-action-button--size-s",
            ActionButtonSize::M => "ui-action-button--size-m",
            ActionButtonSize::L => "ui-action-button--size-l",
            ActionButtonSize::XL => "ui-action-button--size-xl",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionButtonSize::XS => "xs",
            ActionButtonSize::S => "s",
            ActionButtonSize::M => "m",
            ActionButtonSize::L => "l",
            ActionButtonSize::XL => "xl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonLoadingPlacement {
    #[default]
    Center,
    Start,
    End,
}

impl ActionButtonLoadingPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ActionButtonLoadingPlacement::Center => "center",
            ActionButtonLoadingPlacement::Start => "start",
            ActionButtonLoadingPlacement::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonStateInput {
    pub disabled: bool,
    pub is_loading: bool,
    pub size: ActionButtonSize,
    pub loading_placement: ActionButtonLoadingPlacement,
    pub is_quiet: bool,
    pub is_icon_only: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonState {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub size: ActionButtonSize,
    pub size_attr: &'static str,
    pub loading_placement: ActionButtonLoadingPlacement,
    pub loading_placement_attr: &'static str,
    pub is_quiet: bool,
    pub is_icon_only: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: ActionButtonStateInput) -> ActionButtonState {
    ActionButtonState {
        is_disabled: input.disabled || input.is_loading,
        is_loading: input.is_loading,
        size: input.size,
        size_attr: input.size.as_attr(),
        loading_placement: input.loading_placement,
        loading_placement_attr: input.loading_placement.as_attr(),
        is_quiet: input.is_quiet,
        is_icon_only: input.is_icon_only,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ActionButtonState) -> String {
    let mut classes = vec![
        "ui-action-button".to_string(),
        state.size.class_name().to_string(),
        format!("ui-action-button--loading-{}", state.loading_placement_attr),
    ];

    if state.is_quiet {
        classes.push("ui-action-button--quiet".to_string());
    } else {
        classes.push("ui-action-button--filled".to_string());
    }

    if state.is_icon_only {
        classes.push("ui-action-button--icon-only".to_string());
    }
    if state.is_loading {
        classes.push("ui-action-button--loading".to_string());
    }
    if state.has_start_content {
        classes.push("ui-action-button--has-start".to_string());
    }
    if state.has_end_content {
        classes.push("ui-action-button--has-end".to_string());
    }
    if state.has_custom_press_handler {
        classes.push("ui-action-button--with-handler".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            ActionButtonSize::XS.class_name(),
            "ui-action-button--size-xs"
        );
        assert_eq!(ActionButtonSize::XS.as_attr(), "xs");
        assert_eq!(ActionButtonSize::M.class_name(), "ui-action-button--size-m");
        assert_eq!(
            ActionButtonSize::XL.class_name(),
            "ui-action-button--size-xl"
        );
        assert_eq!(ActionButtonSize::XL.as_attr(), "xl");
    }

    #[test]
    fn loading_placement_attrs_match_variants() {
        assert_eq!(ActionButtonLoadingPlacement::Center.as_attr(), "center");
        assert_eq!(ActionButtonLoadingPlacement::Start.as_attr(), "start");
        assert_eq!(ActionButtonLoadingPlacement::End.as_attr(), "end");
    }

    #[test]
    fn loading_forces_disabled() {
        assert!(
            !resolve_state(ActionButtonStateInput {
                disabled: false,
                is_loading: false,
                size: ActionButtonSize::M,
                loading_placement: ActionButtonLoadingPlacement::Center,
                is_quiet: false,
                is_icon_only: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_press_handler: false,
            })
            .is_disabled
        );
        assert!(
            resolve_state(ActionButtonStateInput {
                disabled: false,
                is_loading: true,
                size: ActionButtonSize::M,
                loading_placement: ActionButtonLoadingPlacement::Center,
                is_quiet: false,
                is_icon_only: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_press_handler: false,
            })
            .is_disabled
        );
        assert!(
            resolve_state(ActionButtonStateInput {
                disabled: true,
                is_loading: false,
                size: ActionButtonSize::M,
                loading_placement: ActionButtonLoadingPlacement::Center,
                is_quiet: false,
                is_icon_only: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_press_handler: false,
            })
            .is_disabled
        );
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Action  ".to_string())),
            Some("Action".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_state_tracks_visual_modifiers() {
        let state = resolve_state(ActionButtonStateInput {
            disabled: false,
            is_loading: true,
            size: ActionButtonSize::L,
            loading_placement: ActionButtonLoadingPlacement::End,
            is_quiet: true,
            is_icon_only: true,
            has_start_content: true,
            has_end_content: false,
            has_custom_class_name: true,
            has_custom_press_handler: true,
        });

        assert!(state.is_disabled);
        assert!(state.is_loading);
        assert_eq!(state.size_attr, "l");
        assert_eq!(state.loading_placement_attr, "end");
        assert!(state.is_quiet);
        assert!(state.is_icon_only);
        assert!(state.has_start_content);
        assert!(!state.has_end_content);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_press_handler);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ActionButtonStateInput {
                disabled: false,
                is_loading: true,
                size: ActionButtonSize::S,
                loading_placement: ActionButtonLoadingPlacement::Start,
                is_quiet: true,
                is_icon_only: true,
                has_start_content: true,
                has_end_content: true,
                has_custom_class_name: true,
                has_custom_press_handler: true,
            }),
        );

        for token in [
            "ui-action-button",
            "ui-action-button--quiet",
            "ui-action-button--size-s",
            "ui-action-button--icon-only",
            "ui-action-button--loading-start",
            "ui-action-button--loading",
            "ui-action-button--has-start",
            "ui-action-button--has-end",
            "ui-action-button--with-handler",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
