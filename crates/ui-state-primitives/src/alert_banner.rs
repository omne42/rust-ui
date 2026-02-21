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
    pub fn default_icon_label(self) -> Option<&'static str> {
        match self {
            AlertBannerTone::Neutral => None,
            AlertBannerTone::Info => Some("Info"),
            AlertBannerTone::Positive => Some("Success"),
            AlertBannerTone::Notice => Some("Warning"),
            AlertBannerTone::Negative => Some("Error"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertBannerVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl AlertBannerVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertBannerVariant::Default => "default",
            AlertBannerVariant::Accent => "accent",
            AlertBannerVariant::Danger => "danger",
        }
    }

    pub fn as_tone(self) -> AlertBannerTone {
        match self {
            AlertBannerVariant::Default => AlertBannerTone::Neutral,
            AlertBannerVariant::Accent => AlertBannerTone::Info,
            AlertBannerVariant::Danger => AlertBannerTone::Negative,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertBannerToneSource {
    Tone,
    Variant,
    Default,
}

impl AlertBannerToneSource {
    pub fn attr_value(self) -> &'static str {
        match self {
            AlertBannerToneSource::Tone => "tone",
            AlertBannerToneSource::Variant => "variant",
            AlertBannerToneSource::Default => "default",
        }
    }
}

pub fn resolve_tone(
    tone: Option<AlertBannerTone>,
    variant: Option<AlertBannerVariant>,
) -> (AlertBannerTone, AlertBannerToneSource) {
    if let Some(tone) = tone {
        return (tone, AlertBannerToneSource::Tone);
    }

    if let Some(variant) = variant {
        return (variant.as_tone(), AlertBannerToneSource::Variant);
    }

    (AlertBannerTone::default(), AlertBannerToneSource::Default)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertBannerFill {
    #[default]
    Border,
    Subtle,
    Bold,
}

impl AlertBannerFill {
    pub fn attr_value(self) -> &'static str {
        match self {
            AlertBannerFill::Border => "border",
            AlertBannerFill::Subtle => "subtle",
            AlertBannerFill::Bold => "bold",
        }
    }
}

pub fn normalize_fill(fill: Option<AlertBannerFill>) -> AlertBannerFill {
    fill.unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertBannerHideIconSource {
    IsHideIcon,
    HideIcon,
    Default,
}

impl AlertBannerHideIconSource {
    pub fn attr_value(self) -> &'static str {
        match self {
            AlertBannerHideIconSource::IsHideIcon => "is-hide-icon",
            AlertBannerHideIconSource::HideIcon => "hide-icon",
            AlertBannerHideIconSource::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertBannerHideIcon {
    pub value: bool,
    pub source: AlertBannerHideIconSource,
}

pub fn resolve_hide_icon(
    is_hide_icon: Option<bool>,
    hide_icon: Option<bool>,
) -> AlertBannerHideIcon {
    match (is_hide_icon, hide_icon) {
        (Some(value), _) => AlertBannerHideIcon {
            value,
            source: AlertBannerHideIconSource::IsHideIcon,
        },
        (None, Some(value)) => AlertBannerHideIcon {
            value,
            source: AlertBannerHideIconSource::HideIcon,
        },
        (None, None) => AlertBannerHideIcon {
            value: false,
            source: AlertBannerHideIconSource::Default,
        },
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
        (!trimmed.is_empty()).then(|| trimmed.into())
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
#[path = "test/alert_banner.rs"]
mod tests;
