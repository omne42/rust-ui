use leptos::prelude::{Callback, ReadSignal, Signal};

pub use ui_state_primitives::tabs::{
    TabsKeyboardActivation, normalize_index_skipping_disabled, resolve_tabs_state,
};

pub const BASE_CLASS_NAME: &str = "ui-tabs";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsControlMode {
    Controlled,
    Uncontrolled,
}

impl TabsControlMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            TabsControlMode::Controlled => "controlled",
            TabsControlMode::Uncontrolled => "uncontrolled",
        }
    }

    pub const fn is_controlled(self) -> bool {
        matches!(self, TabsControlMode::Controlled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsDisabledSource {
    IsDisabled,
    Disabled,
}

impl TabsDisabledSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            TabsDisabledSource::IsDisabled => "is-disabled",
            TabsDisabledSource::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabsDisabledAxis {
    pub is_disabled: bool,
    pub source: TabsDisabledSource,
}

pub fn normalize_disabled_axis(is_disabled: Option<bool>, disabled: bool) -> TabsDisabledAxis {
    let source = if is_disabled.is_some() {
        TabsDisabledSource::IsDisabled
    } else {
        TabsDisabledSource::Disabled
    };
    let resolved_is_disabled = normalize_is_disabled(is_disabled, disabled);

    TabsDisabledAxis {
        is_disabled: resolved_is_disabled,
        source,
    }
}

#[derive(Clone)]
pub struct TabsSelectionAxisInput {
    pub selected_index: Option<ReadSignal<usize>>,
    pub default_selected_index: usize,
    pub on_selection_change: Option<Callback<usize>>,
}

#[derive(Clone)]
pub struct TabsSelectionAxis {
    pub selected_index: Option<Signal<usize>>,
    pub default_selected_index: usize,
    pub on_selection_change: Option<Callback<usize>>,
    pub control_mode: TabsControlMode,
}

pub fn normalize_selection_axis(input: TabsSelectionAxisInput) -> TabsSelectionAxis {
    let control_mode = if input.selected_index.is_some() {
        TabsControlMode::Controlled
    } else {
        TabsControlMode::Uncontrolled
    };

    TabsSelectionAxis {
        selected_index: input.selected_index.map(Into::into),
        default_selected_index: input.default_selected_index,
        on_selection_change: input.on_selection_change,
        control_mode,
    }
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn resolve_requested_selected_index(
    controlled_selected_index: Option<usize>,
    default_selected_index: usize,
) -> usize {
    controlled_selected_index.unwrap_or(default_selected_index)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn compose_class_name(class_name: Option<String>) -> String {
    if let Some(class_name) = normalize_optional_text(class_name) {
        format!("{BASE_CLASS_NAME} {class_name}")
    } else {
        BASE_CLASS_NAME.to_string()
    }
}

pub fn resolve_motion_source(has_custom_motion: bool) -> &'static str {
    if has_custom_motion {
        "custom"
    } else {
        "default"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_is_disabled_prefers_is_prefixed_value() {
        assert!(normalize_is_disabled(Some(true), false));
        assert!(!normalize_is_disabled(Some(false), true));
        assert!(normalize_is_disabled(None, true));
        assert!(!normalize_is_disabled(None, false));
    }

    #[test]
    fn normalize_disabled_axis_tracks_source() {
        let from_is_disabled = normalize_disabled_axis(Some(true), false);
        assert!(from_is_disabled.is_disabled);
        assert_eq!(from_is_disabled.source, TabsDisabledSource::IsDisabled);
        assert_eq!(from_is_disabled.source.as_attr(), "is-disabled");

        let from_disabled = normalize_disabled_axis(None, true);
        assert!(from_disabled.is_disabled);
        assert_eq!(from_disabled.source, TabsDisabledSource::Disabled);
        assert_eq!(from_disabled.source.as_attr(), "disabled");
    }

    #[test]
    fn normalize_selection_axis_tracks_control_mode() {
        let (selected, _set_selected) = leptos::prelude::signal(2_usize);
        let controlled = normalize_selection_axis(TabsSelectionAxisInput {
            selected_index: Some(selected),
            default_selected_index: 0,
            on_selection_change: None,
        });
        assert_eq!(controlled.control_mode, TabsControlMode::Controlled);
        assert!(controlled.control_mode.is_controlled());

        let uncontrolled = normalize_selection_axis(TabsSelectionAxisInput {
            selected_index: None,
            default_selected_index: 1,
            on_selection_change: None,
        });
        assert_eq!(uncontrolled.control_mode, TabsControlMode::Uncontrolled);
        assert!(!uncontrolled.control_mode.is_controlled());
    }

    #[test]
    fn resolve_requested_selected_index_uses_controlled_then_default() {
        assert_eq!(resolve_requested_selected_index(Some(3), 1), 3);
        assert_eq!(resolve_requested_selected_index(None, 1), 1);
    }

    #[test]
    fn compose_class_name_keeps_base_and_custom_suffix() {
        assert_eq!(compose_class_name(None), "ui-tabs");
        assert_eq!(
            compose_class_name(Some("  my-tabs ".to_string())),
            "ui-tabs my-tabs"
        );
        assert_eq!(compose_class_name(Some("   ".to_string())), "ui-tabs");
    }

    #[test]
    fn resolve_motion_source_maps_custom_flag() {
        assert_eq!(resolve_motion_source(false), "default");
        assert_eq!(resolve_motion_source(true), "custom");
    }
}
