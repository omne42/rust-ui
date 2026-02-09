#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertBannerTone {
    #[default]
    Neutral,
    Info,
    Positive,
    Notice,
    Negative,
}

impl AlertBannerTone {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertBannerTone::Neutral => "ui-alert-banner--tone-neutral",
            AlertBannerTone::Info => "ui-alert-banner--tone-info",
            AlertBannerTone::Positive => "ui-alert-banner--tone-positive",
            AlertBannerTone::Notice => "ui-alert-banner--tone-notice",
            AlertBannerTone::Negative => "ui-alert-banner--tone-negative",
        }
    }

    pub fn default_icon_label(self) -> Option<&'static str> {
        match self {
            AlertBannerTone::Neutral => None,
            AlertBannerTone::Info => Some("Info"),
            AlertBannerTone::Positive => Some("Success"),
            AlertBannerTone::Notice => Some("Warning"),
            AlertBannerTone::Negative => Some("Error"),
        }
    }

    pub fn aria_live(self) -> &'static str {
        match self {
            AlertBannerTone::Negative => "assertive",
            _ => "polite",
        }
    }

    pub fn role(self) -> &'static str {
        match self {
            AlertBannerTone::Negative => "alert",
            _ => "status",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertBannerFill {
    #[default]
    Border,
    Subtle,
    Bold,
}

impl AlertBannerFill {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertBannerFill::Border => "ui-alert-banner--fill-border",
            AlertBannerFill::Subtle => "ui-alert-banner--fill-subtle",
            AlertBannerFill::Bold => "ui-alert-banner--fill-bold",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertBannerViewState {
    pub show_title: bool,
    pub show_description: bool,
    pub show_icon: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_view_state(
    tone: AlertBannerTone,
    title: Option<&str>,
    description: Option<&str>,
    hide_icon: bool,
) -> AlertBannerViewState {
    let show_title = title.is_some_and(|v| !v.trim().is_empty());
    let show_description = description.is_some_and(|v| !v.trim().is_empty());
    let show_icon = !hide_icon && tone.default_icon_label().is_some();

    AlertBannerViewState {
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
        let state = resolve_view_state(AlertBannerTone::Neutral, None, None, false);
        assert!(!state.show_icon);
    }

    #[test]
    fn hide_icon_forces_icon_off() {
        let state = resolve_view_state(AlertBannerTone::Info, None, None, true);
        assert!(!state.show_icon);
    }

    #[test]
    fn title_and_description_flags_respect_trimmed_content() {
        let state = resolve_view_state(AlertBannerTone::Info, Some("  "), Some("ok"), false);
        assert!(!state.show_title);
        assert!(state.show_description);
    }
}
