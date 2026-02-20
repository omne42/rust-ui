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
#[path = "../test/logic.rs"]
mod tests;
