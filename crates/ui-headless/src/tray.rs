use crate::a11y::{A11yDirection, overlay_dialog_attrs};

#[derive(Clone)]
pub struct TrayA11yOptions {
    pub title_id: String,
    pub description_id: Option<String>,
    pub has_description: bool,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone, Default)]
pub struct TrayA11yHandlers;

#[derive(Clone)]
pub struct TrayA11yAttrs {
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayDescriptionA11yState {
    WithDescription,
    TitleOnly,
}

impl TrayDescriptionA11yState {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::WithDescription => "with-description",
            Self::TitleOnly => "title-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayA11yState {
    pub description_state: TrayDescriptionA11yState,
    pub has_description: bool,
}

#[derive(Clone)]
pub struct TrayA11yContract {
    pub attrs: TrayA11yAttrs,
    pub handlers: TrayA11yHandlers,
    pub state: TrayA11yState,
}

pub fn use_tray_a11y(options: TrayA11yOptions) -> TrayA11yContract {
    let TrayA11yOptions {
        title_id,
        description_id,
        has_description,
        lang,
        dir,
    } = options;

    let attrs = overlay_dialog_attrs(
        Some(title_id),
        has_description.then_some(description_id).flatten(),
        lang,
        dir,
    );
    let description_state = if attrs.aria_describedby.is_some() {
        TrayDescriptionA11yState::WithDescription
    } else {
        TrayDescriptionA11yState::TitleOnly
    };

    TrayA11yContract {
        attrs: TrayA11yAttrs {
            aria_labelledby: attrs.aria_labelledby,
            aria_describedby: attrs.aria_describedby,
            lang: attrs.lang,
            dir: attrs.dir,
        },
        handlers: TrayA11yHandlers,
        state: TrayA11yState {
            description_state,
            has_description: description_state == TrayDescriptionA11yState::WithDescription,
        },
    }
}

#[cfg(test)]
#[path = "test/tray.rs"]
mod tests;
