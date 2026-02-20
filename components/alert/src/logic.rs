pub use ui_state_primitives::alert_banner::{
    AlertBannerFill as AlertFill, AlertBannerTone as AlertTone, normalize_optional_text,
    resolve_view_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertLayout {
    #[default]
    Banner,
    Inline,
}

impl AlertLayout {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertLayout::Banner => "ui-alert--layout-banner",
            AlertLayout::Inline => "ui-alert--layout-inline",
        }
    }

    pub fn attr_value(self) -> &'static str {
        match self {
            AlertLayout::Banner => "banner",
            AlertLayout::Inline => "inline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl AlertVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Accent => "accent",
            AlertVariant::Danger => "danger",
        }
    }

    pub fn as_tone(self) -> AlertTone {
        match self {
            AlertVariant::Default => AlertTone::Neutral,
            AlertVariant::Accent => AlertTone::Info,
            AlertVariant::Danger => AlertTone::Negative,
        }
    }
}

pub fn normalize_fill(fill: Option<AlertFill>) -> AlertFill {
    fill.unwrap_or_default()
}

pub fn normalize_layout(layout: Option<AlertLayout>) -> AlertLayout {
    layout.unwrap_or_default()
}

pub fn resolve_hide_icon(
    is_hide_icon: Option<bool>,
    hide_icon: Option<bool>,
) -> (bool, &'static str) {
    match (is_hide_icon, hide_icon) {
        (Some(value), _) => (value, "is-hide-icon"),
        (None, Some(value)) => (value, "hide-icon"),
        (None, None) => (false, "default"),
    }
}

fn tone_attr(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Neutral => "neutral",
        AlertTone::Info => "info",
        AlertTone::Positive => "positive",
        AlertTone::Notice => "notice",
        AlertTone::Negative => "negative",
    }
}

fn tone_class_name(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Neutral => "ui-alert--tone-neutral",
        AlertTone::Info => "ui-alert--tone-info",
        AlertTone::Positive => "ui-alert--tone-positive",
        AlertTone::Notice => "ui-alert--tone-notice",
        AlertTone::Negative => "ui-alert--tone-negative",
    }
}

fn fill_attr(fill: AlertFill) -> &'static str {
    match fill {
        AlertFill::Border => "border",
        AlertFill::Subtle => "subtle",
        AlertFill::Bold => "bold",
    }
}

fn fill_class_name(fill: AlertFill) -> &'static str {
    match fill {
        AlertFill::Border => "ui-alert--fill-border",
        AlertFill::Subtle => "ui-alert--fill-subtle",
        AlertFill::Bold => "ui-alert--fill-bold",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateInput {
    pub tone: Option<AlertTone>,
    pub variant: Option<AlertVariant>,
    pub layout: Option<AlertLayout>,
    pub fill: Option<AlertFill>,
    pub has_title: bool,
    pub has_description: bool,
    pub hide_icon: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertState {
    pub tone: AlertTone,
    pub tone_attr: &'static str,
    pub tone_class: &'static str,
    pub fill: AlertFill,
    pub fill_attr: &'static str,
    pub fill_class: &'static str,
    pub layout: AlertLayout,
    pub layout_attr: &'static str,
    pub layout_class: &'static str,
    pub show_title: bool,
    pub title_attr: &'static str,
    pub show_description: bool,
    pub description_attr: &'static str,
    pub show_icon: bool,
    pub icon_attr: &'static str,
    pub role_attr: &'static str,
    pub live_attr: &'static str,
    pub variant_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

fn resolve_tone(
    tone: Option<AlertTone>,
    variant: Option<AlertVariant>,
) -> (AlertTone, &'static str) {
    if let Some(tone) = tone {
        return (tone, "tone");
    }

    if let Some(variant) = variant {
        return (variant.as_tone(), "variant");
    }

    (AlertTone::default(), "default")
}

pub fn resolve_state(input: AlertStateInput) -> AlertState {
    let (tone, variant_source_attr) = resolve_tone(input.tone, input.variant);
    let fill = normalize_fill(input.fill);
    let layout = normalize_layout(input.layout);

    let view_state = resolve_view_state(
        tone,
        input.has_title.then_some("present"),
        input.has_description.then_some("present"),
        input.hide_icon,
    );

    AlertState {
        tone,
        tone_attr: tone_attr(tone),
        tone_class: tone_class_name(tone),
        fill,
        fill_attr: fill_attr(fill),
        fill_class: fill_class_name(fill),
        layout,
        layout_attr: layout.attr_value(),
        layout_class: layout.class_name(),
        show_title: view_state.show_title,
        title_attr: if view_state.show_title {
            "present"
        } else {
            "absent"
        },
        show_description: view_state.show_description,
        description_attr: if view_state.show_description {
            "present"
        } else {
            "absent"
        },
        show_icon: view_state.show_icon,
        icon_attr: if view_state.show_icon {
            "visible"
        } else {
            "hidden"
        },
        role_attr: tone.role(),
        live_attr: tone.aria_live(),
        variant_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AlertState) -> String {
    let mut classes = vec![
        "ui-alert".to_string(),
        state.layout_class.to_string(),
        state.tone_class.to_string(),
        state.fill_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-alert--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
