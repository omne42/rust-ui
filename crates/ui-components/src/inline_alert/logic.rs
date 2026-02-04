#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InlineAlertTone {
    #[default]
    Neutral,
    Info,
    Positive,
    Notice,
    Negative,
}

impl InlineAlertTone {
    pub fn class_name(self) -> &'static str {
        match self {
            InlineAlertTone::Neutral => "ui-inline-alert--tone-neutral",
            InlineAlertTone::Info => "ui-inline-alert--tone-info",
            InlineAlertTone::Positive => "ui-inline-alert--tone-positive",
            InlineAlertTone::Notice => "ui-inline-alert--tone-notice",
            InlineAlertTone::Negative => "ui-inline-alert--tone-negative",
        }
    }

    pub fn default_icon_label(self) -> Option<&'static str> {
        match self {
            InlineAlertTone::Neutral => None,
            InlineAlertTone::Info => Some("Info"),
            InlineAlertTone::Positive => Some("Success"),
            InlineAlertTone::Notice => Some("Warning"),
            InlineAlertTone::Negative => Some("Error"),
        }
    }

    pub fn aria_live(self) -> &'static str {
        match self {
            InlineAlertTone::Negative => "assertive",
            _ => "polite",
        }
    }

    pub fn role(self) -> &'static str {
        match self {
            InlineAlertTone::Negative => "alert",
            _ => "status",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InlineAlertFill {
    #[default]
    Border,
    Subtle,
    Bold,
}

impl InlineAlertFill {
    pub fn class_name(self) -> &'static str {
        match self {
            InlineAlertFill::Border => "ui-inline-alert--fill-border",
            InlineAlertFill::Subtle => "ui-inline-alert--fill-subtle",
            InlineAlertFill::Bold => "ui-inline-alert--fill-bold",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InlineAlertViewState {
    pub show_title: bool,
    pub show_description: bool,
    pub show_icon: bool,
}

pub fn resolve_view_state(
    tone: InlineAlertTone,
    title: Option<&str>,
    description: Option<&str>,
    hide_icon: bool,
) -> InlineAlertViewState {
    let show_title = title.is_some_and(|v| !v.trim().is_empty());
    let show_description = description.is_some_and(|v| !v.trim().is_empty());
    let show_icon = !hide_icon && tone.default_icon_label().is_some();

    InlineAlertViewState {
        show_title,
        show_description,
        show_icon,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_defaults_to_no_icon() {
        let state = resolve_view_state(InlineAlertTone::Neutral, None, None, false);
        assert!(!state.show_icon);
    }

    #[test]
    fn hide_icon_forces_icon_off() {
        let state = resolve_view_state(InlineAlertTone::Info, None, None, true);
        assert!(!state.show_icon);
    }

    #[test]
    fn title_and_description_flags_respect_trimmed_content() {
        let state = resolve_view_state(InlineAlertTone::Info, Some("  "), Some("ok"), false);
        assert!(!state.show_title);
        assert!(state.show_description);
    }
}
