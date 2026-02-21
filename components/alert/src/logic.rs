use std::borrow::Cow;

use ui_headless::{LiveRegionPriority, live_region_attrs};
pub use ui_state_primitives::alert_banner::{
    AlertBannerFill as AlertFill, AlertBannerTone as AlertTone,
    AlertBannerVariant as AlertVariantPrimitive, normalize_fill as normalize_fill_primitive,
    normalize_optional_text, resolve_hide_icon as resolve_hide_icon_primitive,
    resolve_tone as resolve_tone_primitive, resolve_view_state,
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
        self.as_primitive().as_tone()
    }

    pub fn as_primitive(self) -> AlertVariantPrimitive {
        match self {
            AlertVariant::Default => AlertVariantPrimitive::Default,
            AlertVariant::Accent => AlertVariantPrimitive::Accent,
            AlertVariant::Danger => AlertVariantPrimitive::Danger,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertAgentSchema {
    #[default]
    V1,
}

impl AlertAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertAgentSchema::V1 => "alert.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertAgentIntent {
    #[default]
    StatusRegion,
}

impl AlertAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertAgentIntent::StatusRegion => "status-region",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertAgentAction {
    #[default]
    Announce,
}

impl AlertAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertAgentAction::Announce => "announce",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertAgentState {
    #[default]
    Snapshot,
}

impl AlertAgentState {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertAgentState::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertStreamingPolicy {
    #[default]
    Optional,
    Required,
}

impl AlertStreamingPolicy {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertStreamingPolicy::Optional => "optional",
            AlertStreamingPolicy::Required => "required",
        }
    }
}
const _: [AlertStreamingPolicy; 2] = [
    AlertStreamingPolicy::Optional,
    AlertStreamingPolicy::Required,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertStreamingFallback {
    #[default]
    Snapshot,
}

impl AlertStreamingFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertStreamingFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertOutputStatus {
    Draft,
    #[default]
    Verified,
    Committable,
}

impl AlertOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertOutputStatus::Draft => "draft",
            AlertOutputStatus::Verified => "verified",
            AlertOutputStatus::Committable => "committable",
        }
    }
}
const _: [AlertOutputStatus; 3] = [
    AlertOutputStatus::Draft,
    AlertOutputStatus::Verified,
    AlertOutputStatus::Committable,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertAgentSource {
    Tone,
    Variant,
    #[default]
    Default,
}

impl AlertAgentSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertAgentSource::Tone => "tone",
            AlertAgentSource::Variant => "variant",
            AlertAgentSource::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertMotionSource {
    #[default]
    Default,
    Custom,
}

impl AlertMotionSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertMotionSource::Default => "default",
            AlertMotionSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertIconLabelSource {
    Custom,
    ToneDefault,
    #[default]
    None,
}

impl AlertIconLabelSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            AlertIconLabelSource::Custom => "custom",
            AlertIconLabelSource::ToneDefault => "tone-default",
            AlertIconLabelSource::None => "none",
        }
    }
}

pub fn normalize_fill(fill: Option<AlertFill>) -> AlertFill {
    normalize_fill_primitive(fill)
}

pub fn normalize_layout(layout: Option<AlertLayout>) -> AlertLayout {
    layout.unwrap_or_default()
}

pub fn resolve_hide_icon(
    is_hide_icon: Option<bool>,
    hide_icon: Option<bool>,
) -> (bool, &'static str) {
    let resolved = resolve_hide_icon_primitive(is_hide_icon, hide_icon);
    (resolved.value, resolved.source.attr_value())
}

pub fn resolve_icon_label(
    icon_label: Option<String>,
    tone: AlertTone,
) -> (String, AlertIconLabelSource) {
    let (label, source) = if let Some(label) = normalize_optional_text(icon_label) {
        (Cow::Owned(label), AlertIconLabelSource::Custom)
    } else if let Some(label) = tone.default_icon_label() {
        (Cow::Borrowed(label), AlertIconLabelSource::ToneDefault)
    } else {
        (Cow::Borrowed(""), AlertIconLabelSource::None)
    };

    (label.into_owned(), source)
}

pub fn resolve_agent_source(variant_source_attr: &'static str) -> AlertAgentSource {
    match variant_source_attr {
        "tone" => AlertAgentSource::Tone,
        "variant" => AlertAgentSource::Variant,
        _ => AlertAgentSource::Default,
    }
}

pub fn resolve_motion_source(is_default_motion: bool) -> AlertMotionSource {
    if is_default_motion {
        AlertMotionSource::Default
    } else {
        AlertMotionSource::Custom
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

fn live_region_priority(tone: AlertTone) -> LiveRegionPriority {
    match tone {
        AlertTone::Negative => LiveRegionPriority::Assertive,
        _ => LiveRegionPriority::Polite,
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

pub fn resolve_state(input: AlertStateInput) -> AlertState {
    let (tone, variant_source) =
        resolve_tone_primitive(input.tone, input.variant.map(AlertVariant::as_primitive));
    let fill = normalize_fill(input.fill);
    let layout = normalize_layout(input.layout);
    let live_region = live_region_attrs(live_region_priority(tone));

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
        fill_attr: fill.attr_value(),
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
        role_attr: live_region.role,
        live_attr: live_region.aria_live,
        variant_source_attr: variant_source.attr_value(),
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AlertState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-alert"),
        Cow::Borrowed(state.layout_class),
        Cow::Borrowed(state.tone_class),
        Cow::Borrowed(state.fill_class),
    ];

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-alert--custom-class"));
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(Cow::Owned(base_class_name));
    }

    let mut iter = classes.iter();
    let Some(first) = iter.next() else {
        return String::new();
    };

    let mut class_name = String::from(first.as_ref());
    for class in iter {
        class_name.push(' ');
        class_name.push_str(class.as_ref());
    }
    class_name
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
