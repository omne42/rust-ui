pub use ui_state_primitives::visually_hidden::VisuallyHiddenState;
use ui_state_primitives::visually_hidden::{
    VisuallyHiddenStateInput, normalize_optional_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VisuallyHiddenFocusMode {
    #[default]
    Hidden,
    Focusable,
}

impl VisuallyHiddenFocusMode {
    pub fn is_focusable(self) -> bool {
        matches!(self, Self::Focusable)
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Focusable => "focusable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FocusPropSource {
    #[default]
    Default,
    IsFocusable,
    FocusableAlias,
}

impl FocusPropSource {
    pub fn resolve(
        is_focusable: Option<bool>,
        focusable: Option<bool>,
    ) -> (VisuallyHiddenFocusMode, Self) {
        if let Some(value) = is_focusable {
            let mode = if value {
                VisuallyHiddenFocusMode::Focusable
            } else {
                VisuallyHiddenFocusMode::Hidden
            };
            return (mode, Self::IsFocusable);
        }

        if let Some(value) = focusable {
            let mode = if value {
                VisuallyHiddenFocusMode::Focusable
            } else {
                VisuallyHiddenFocusMode::Hidden
            };
            return (mode, Self::FocusableAlias);
        }

        (VisuallyHiddenFocusMode::Hidden, Self::Default)
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::IsFocusable => "is_focusable",
            Self::FocusableAlias => "focusable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClassNameSource {
    #[default]
    Default,
    Custom,
}

impl ClassNameSource {
    pub fn from_class_name(class_name: &Option<String>) -> Self {
        if class_name.is_some() {
            Self::Custom
        } else {
            Self::Default
        }
    }

    pub fn has_custom_class_name(self) -> bool {
        matches!(self, Self::Custom)
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenLogicInput {
    pub is_focusable: Option<bool>,
    pub focusable: Option<bool>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisuallyHiddenLogicState {
    pub focus_mode: VisuallyHiddenFocusMode,
    pub focus_prop_source: FocusPropSource,
    pub class_name_source: ClassNameSource,
    pub class_name: Option<String>,
    pub primitive_state: VisuallyHiddenState,
}

pub fn normalize_props(input: VisuallyHiddenLogicInput) -> VisuallyHiddenLogicState {
    let (focus_mode, focus_prop_source) =
        FocusPropSource::resolve(input.is_focusable, input.focusable);
    let class_name = normalize_optional_text(input.class_name);
    let class_name_source = ClassNameSource::from_class_name(&class_name);
    let primitive_state = resolve_state(VisuallyHiddenStateInput {
        is_focusable: focus_mode.is_focusable(),
        has_custom_class_name: class_name_source.has_custom_class_name(),
    });

    VisuallyHiddenLogicState {
        focus_mode,
        focus_prop_source,
        class_name_source,
        class_name,
        primitive_state,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: VisuallyHiddenState) -> String {
    let mut class = "ui-visually-hidden".to_string();
    if let Some(focusable_class) = state.focusable_class {
        class.push(' ');
        class.push_str(focusable_class);
    }
    if state.has_custom_class_name
        && let Some(custom_class_name) = class_name
    {
        class.push(' ');
        class.push_str(&custom_class_name);
    }
    class
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_mode_source_and_attr_mappings_are_stable() {
        assert_eq!(VisuallyHiddenFocusMode::Hidden.as_attr(), "hidden");
        assert_eq!(VisuallyHiddenFocusMode::Focusable.as_attr(), "focusable");

        assert_eq!(FocusPropSource::Default.as_attr(), "default");
        assert_eq!(FocusPropSource::IsFocusable.as_attr(), "is_focusable");
        assert_eq!(FocusPropSource::FocusableAlias.as_attr(), "focusable");

        assert_eq!(
            FocusPropSource::resolve(None, None),
            (VisuallyHiddenFocusMode::Hidden, FocusPropSource::Default)
        );
        assert_eq!(
            FocusPropSource::resolve(Some(true), None),
            (
                VisuallyHiddenFocusMode::Focusable,
                FocusPropSource::IsFocusable
            )
        );
        assert_eq!(
            FocusPropSource::resolve(Some(false), Some(true)),
            (
                VisuallyHiddenFocusMode::Hidden,
                FocusPropSource::IsFocusable
            )
        );
        assert_eq!(
            FocusPropSource::resolve(None, Some(true)),
            (
                VisuallyHiddenFocusMode::Focusable,
                FocusPropSource::FocusableAlias
            )
        );
    }

    #[test]
    fn class_name_source_uses_normalized_class_name() {
        let default_state = normalize_props(VisuallyHiddenLogicInput {
            is_focusable: None,
            focusable: None,
            class_name: Some("   ".to_string()),
        });
        assert_eq!(default_state.class_name_source, ClassNameSource::Default);
        assert_eq!(default_state.class_name, None);

        let custom_state = normalize_props(VisuallyHiddenLogicInput {
            is_focusable: None,
            focusable: None,
            class_name: Some(" docs-hidden ".to_string()),
        });
        assert_eq!(custom_state.class_name_source, ClassNameSource::Custom);
        assert_eq!(ClassNameSource::Default.as_attr(), "default");
        assert_eq!(ClassNameSource::Custom.as_attr(), "custom");
        assert_eq!(custom_state.class_name.as_deref(), Some("docs-hidden"));
    }

    #[test]
    fn normalize_props_delegates_state_machine_to_ui_state_primitives() {
        let state = normalize_props(VisuallyHiddenLogicInput {
            is_focusable: Some(true),
            focusable: Some(false),
            class_name: Some("docs-hidden".to_string()),
        });

        assert_eq!(state.focus_mode, VisuallyHiddenFocusMode::Focusable);
        assert_eq!(state.focus_prop_source, FocusPropSource::IsFocusable);
        assert_eq!(state.class_name_source, ClassNameSource::Custom);
        assert!(state.primitive_state.is_focusable);
        assert_eq!(
            state.primitive_state.focusable_class,
            Some("ui-visually-hidden--focusable")
        );
        assert_eq!(state.primitive_state.custom_class_attr, Some("true"));
    }

    #[test]
    fn compose_class_name_tracks_focus_and_custom_class() {
        let state = normalize_props(VisuallyHiddenLogicInput {
            is_focusable: Some(true),
            focusable: None,
            class_name: Some("docs-hidden".to_string()),
        });
        let class = compose_class_name(state.class_name, state.primitive_state);
        for token in [
            "ui-visually-hidden",
            "ui-visually-hidden--focusable",
            "docs-hidden",
        ] {
            assert!(
                class.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
