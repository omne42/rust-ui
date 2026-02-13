#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

pub const BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL: &str = "Button";

impl ButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonVariant::Default => "ui-button--variant-default",
            ButtonVariant::Accent => "ui-button--variant-accent",
            ButtonVariant::Destructive => "ui-button--variant-destructive",
            ButtonVariant::Outline => "ui-button--variant-outline",
            ButtonVariant::Secondary => "ui-button--variant-secondary",
            ButtonVariant::Ghost => "ui-button--variant-ghost",
            ButtonVariant::Link => "ui-button--variant-link",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Xs,
    S,
    #[default]
    M,
    L,
    Xl,
    IconXs,
    IconS,
    IconM,
    IconL,
    IconXl,
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonSize::Xs => "ui-button--size-xs",
            ButtonSize::S => "ui-button--size-s",
            ButtonSize::M => "ui-button--size-m",
            ButtonSize::L => "ui-button--size-l",
            ButtonSize::Xl => "ui-button--size-xl",
            ButtonSize::IconXs => "ui-button--size-icon-xs",
            ButtonSize::IconS => "ui-button--size-icon-s",
            ButtonSize::IconM => "ui-button--size-icon-m",
            ButtonSize::IconL => "ui-button--size-icon-l",
            ButtonSize::IconXl => "ui-button--size-icon-xl",
            ButtonSize::Default => "ui-button--size-m",
            ButtonSize::Sm => "ui-button--size-s",
            ButtonSize::Lg => "ui-button--size-l",
            ButtonSize::Icon => "ui-button--size-icon-m",
            ButtonSize::IconSm => "ui-button--size-icon-s",
            ButtonSize::IconLg => "ui-button--size-icon-l",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonLoadingPlacement {
    #[default]
    Start,
    End,
    Center,
}

impl ButtonLoadingPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonLoadingPlacement::Start => "start",
            ButtonLoadingPlacement::End => "end",
            ButtonLoadingPlacement::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonLabelSource {
    Explicit,
    Fallback,
    None,
}

impl ButtonLabelSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonLabelSource::Explicit => "explicit",
            ButtonLabelSource::Fallback => "fallback",
            ButtonLabelSource::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonStateInput {
    pub disabled: bool,
    pub is_loading: bool,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub loading_placement: ButtonLoadingPlacement,
    pub is_icon_only: bool,
    pub full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonState {
    pub is_disabled: bool,
    pub is_loading: bool,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub loading_placement: ButtonLoadingPlacement,
    pub loading_placement_attr: &'static str,
    pub is_icon_only: bool,
    pub full_width: bool,
    pub has_start_content: bool,
    pub has_end_content: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(
    aria_label: Option<String>,
    is_icon_only: bool,
) -> (Option<String>, ButtonLabelSource) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (Some(label), ButtonLabelSource::Explicit);
    }

    if is_icon_only {
        return (
            Some(BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string()),
            ButtonLabelSource::Fallback,
        );
    }

    (None, ButtonLabelSource::None)
}

pub fn resolve_state(input: ButtonStateInput) -> ButtonState {
    let is_disabled = input.disabled || input.is_loading;
    let state_attr = if input.is_loading {
        "loading"
    } else if is_disabled {
        "disabled"
    } else {
        "ready"
    };

    ButtonState {
        is_disabled,
        is_loading: input.is_loading,
        variant: input.variant,
        size: input.size,
        loading_placement: input.loading_placement,
        loading_placement_attr: input.loading_placement.as_attr(),
        is_icon_only: input.is_icon_only,
        full_width: input.full_width,
        has_start_content: input.has_start_content,
        has_end_content: input.has_end_content,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ButtonState) -> String {
    let mut classes = vec![
        "ui-button".to_string(),
        state.variant.class_name().to_string(),
        state.size.class_name().to_string(),
        format!("ui-button--loading-{}", state.loading_placement_attr),
    ];

    if state.is_icon_only {
        classes.push("ui-button--icon-only".to_string());
    }
    if state.full_width {
        classes.push("ui-button--full-width".to_string());
    }
    if state.is_loading {
        classes.push("ui-button--loading".to_string());
    }
    if state.has_start_content {
        classes.push("ui-button--has-start".to_string());
    }
    if state.has_end_content {
        classes.push("ui-button--has-end".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-button--custom-motion".to_string());
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
    fn variant_class_names_are_stable() {
        assert_eq!(
            ButtonVariant::Default.class_name(),
            "ui-button--variant-default"
        );
        assert_eq!(
            ButtonVariant::Accent.class_name(),
            "ui-button--variant-accent"
        );
        assert_eq!(
            ButtonVariant::Destructive.class_name(),
            "ui-button--variant-destructive"
        );
        assert_eq!(
            ButtonVariant::Outline.class_name(),
            "ui-button--variant-outline"
        );
        assert_eq!(
            ButtonVariant::Secondary.class_name(),
            "ui-button--variant-secondary"
        );
        assert_eq!(
            ButtonVariant::Ghost.class_name(),
            "ui-button--variant-ghost"
        );
        assert_eq!(ButtonVariant::Link.class_name(), "ui-button--variant-link");
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(ButtonSize::Xs.class_name(), "ui-button--size-xs");
        assert_eq!(ButtonSize::S.class_name(), "ui-button--size-s");
        assert_eq!(ButtonSize::M.class_name(), "ui-button--size-m");
        assert_eq!(ButtonSize::L.class_name(), "ui-button--size-l");
        assert_eq!(ButtonSize::Xl.class_name(), "ui-button--size-xl");
        assert_eq!(ButtonSize::IconXs.class_name(), "ui-button--size-icon-xs");
        assert_eq!(ButtonSize::IconS.class_name(), "ui-button--size-icon-s");
        assert_eq!(ButtonSize::IconM.class_name(), "ui-button--size-icon-m");
        assert_eq!(ButtonSize::IconL.class_name(), "ui-button--size-icon-l");
        assert_eq!(ButtonSize::IconXl.class_name(), "ui-button--size-icon-xl");

        assert_eq!(ButtonSize::Default.class_name(), "ui-button--size-m");
        assert_eq!(ButtonSize::Sm.class_name(), "ui-button--size-s");
        assert_eq!(ButtonSize::Lg.class_name(), "ui-button--size-l");
        assert_eq!(ButtonSize::Icon.class_name(), "ui-button--size-icon-m");
        assert_eq!(ButtonSize::IconSm.class_name(), "ui-button--size-icon-s");
        assert_eq!(ButtonSize::IconLg.class_name(), "ui-button--size-icon-l");
    }

    #[test]
    fn loading_placement_attrs_match_variants() {
        assert_eq!(ButtonLoadingPlacement::Start.as_attr(), "start");
        assert_eq!(ButtonLoadingPlacement::End.as_attr(), "end");
        assert_eq!(ButtonLoadingPlacement::Center.as_attr(), "center");
    }

    #[test]
    fn label_source_attrs_are_stable() {
        assert_eq!(ButtonLabelSource::Explicit.as_attr(), "explicit");
        assert_eq!(ButtonLabelSource::Fallback.as_attr(), "fallback");
        assert_eq!(ButtonLabelSource::None.as_attr(), "none");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Button  ".to_string())),
            Some("Button".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_prefers_explicit_then_fallback() {
        assert_eq!(
            resolve_aria_label(Some(" Save ".to_string()), true),
            (Some("Save".to_string()), ButtonLabelSource::Explicit)
        );
        assert_eq!(
            resolve_aria_label(None, true),
            (
                Some(BUTTON_ICON_ONLY_FALLBACK_ARIA_LABEL.to_string()),
                ButtonLabelSource::Fallback,
            )
        );
        assert_eq!(
            resolve_aria_label(None, false),
            (None, ButtonLabelSource::None)
        );
    }

    #[test]
    fn resolve_state_tracks_visual_markers() {
        let state = resolve_state(ButtonStateInput {
            disabled: false,
            is_loading: true,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Icon,
            loading_placement: ButtonLoadingPlacement::End,
            is_icon_only: true,
            full_width: true,
            has_start_content: true,
            has_end_content: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.is_disabled);
        assert_eq!(state.state_attr, "loading");
        assert_eq!(state.loading_placement_attr, "end");
        assert!(state.is_icon_only);
        assert!(state.full_width);
        assert!(state.has_start_content);
        assert!(!state.has_end_content);
        assert!(state.has_custom_class_name);
        assert!(state.has_custom_motion);
    }

    #[test]
    fn loading_forces_disabled() {
        assert!(
            !resolve_state(ButtonStateInput {
                disabled: false,
                is_loading: false,
                variant: ButtonVariant::Default,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );

        assert!(
            resolve_state(ButtonStateInput {
                disabled: false,
                is_loading: true,
                variant: ButtonVariant::Default,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );

        assert!(
            resolve_state(ButtonStateInput {
                disabled: true,
                is_loading: false,
                variant: ButtonVariant::Default,
                size: ButtonSize::M,
                loading_placement: ButtonLoadingPlacement::Start,
                is_icon_only: false,
                full_width: false,
                has_start_content: false,
                has_end_content: false,
                has_custom_class_name: false,
                has_custom_motion: false,
            })
            .is_disabled
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-button".to_string()),
            resolve_state(ButtonStateInput {
                disabled: false,
                is_loading: true,
                variant: ButtonVariant::Outline,
                size: ButtonSize::S,
                loading_placement: ButtonLoadingPlacement::Center,
                is_icon_only: true,
                full_width: true,
                has_start_content: true,
                has_end_content: true,
                has_custom_class_name: true,
                has_custom_motion: true,
            }),
        );

        for needle in [
            "ui-button",
            "ui-button--variant-outline",
            "ui-button--size-s",
            "ui-button--loading-center",
            "ui-button--icon-only",
            "ui-button--full-width",
            "ui-button--loading",
            "ui-button--has-start",
            "ui-button--has-end",
            "ui-button--custom-motion",
            "docs-button",
        ] {
            assert!(
                class_name.contains(needle),
                "composed class name should contain `{needle}`"
            );
        }
    }
}
