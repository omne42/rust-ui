use std::borrow::Cow;

use crate::{IconSlotKind, IconState, IconStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Icon";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl IconSize {
    pub fn class_name(self) -> &'static str {
        match self {
            IconSize::Sm => "ui-icon--size-sm",
            IconSize::Md => "ui-icon--size-md",
            IconSize::Lg => "ui-icon--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            IconSize::Sm => "sm",
            IconSize::Md => "md",
            IconSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconTone {
    #[default]
    Default,
    Muted,
    Accent,
    Danger,
}

impl IconTone {
    pub fn class_name(self) -> &'static str {
        match self {
            IconTone::Default => "ui-icon--tone-default",
            IconTone::Muted => "ui-icon--tone-muted",
            IconTone::Accent => "ui-icon--tone-accent",
            IconTone::Danger => "ui-icon--tone-danger",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            IconTone::Default => "default",
            IconTone::Muted => "muted",
            IconTone::Accent => "accent",
            IconTone::Danger => "danger",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_slot_kind(slot: Option<&str>) -> IconSlotKind {
    match slot {
        None => IconSlotKind::None,
        Some(slot) if slot.eq_ignore_ascii_case("label") => IconSlotKind::Label,
        Some(slot) if slot.eq_ignore_ascii_case("description") => IconSlotKind::Description,
        Some(slot) if slot.eq_ignore_ascii_case("icon") => IconSlotKind::Icon,
        Some(_) => IconSlotKind::Custom,
    }
}

pub fn resolve_state(input: IconStateInput) -> IconState {
    let has_accessible_name = !input.decorative;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.decorative {
        "decorative"
    } else if has_accessible_name {
        "labeled"
    } else {
        "default"
    };

    IconState {
        size: input.size,
        tone: input.tone,
        size_class: input.size.class_name(),
        tone_class: input.tone.class_name(),
        size_attr: input.size.as_attr(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_accessible_name,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        slot_kind: input.slot_kind,
        has_named_slot: input.has_named_slot,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-icon"),
        Cow::Borrowed(state.size_class),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-icon--disabled"));
    }

    if state.is_decorative {
        classes.push(Cow::Borrowed("ui-icon--decorative"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-icon--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
