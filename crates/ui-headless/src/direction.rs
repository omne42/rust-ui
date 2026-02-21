use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::direction::DirectionMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DirectionHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionAttrs {
    pub lang: Option<String>,
    pub dir: &'static str,
    pub data_direction: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectionSemanticState {
    pub direction: DirectionMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionContract {
    pub attrs: DirectionAttrs,
    pub handlers: DirectionHandlers,
    pub state: DirectionSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionOptions {
    pub direction: DirectionMode,
    pub lang: Option<String>,
}

pub fn use_direction(options: DirectionOptions) -> DirectionContract {
    let a11y_dir = match options.direction {
        DirectionMode::Ltr => A11yDirection::Ltr,
        DirectionMode::Rtl => A11yDirection::Rtl,
    };
    let locale = locale_attrs(options.lang, Some(a11y_dir));

    DirectionContract {
        attrs: DirectionAttrs {
            lang: locale.lang,
            dir: options.direction.as_attr(),
            data_direction: options.direction.as_attr(),
        },
        handlers: DirectionHandlers,
        state: DirectionSemanticState {
            direction: options.direction,
        },
    }
}

#[cfg(test)]
#[path = "test/direction.rs"]
mod tests;
