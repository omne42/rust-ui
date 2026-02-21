#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptySlot {
    Root,
    Header,
    Title,
    Description,
    Content,
    Media,
}

impl EmptySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            EmptySlot::Root => "empty",
            EmptySlot::Header => "empty-header",
            EmptySlot::Title => "empty-title",
            EmptySlot::Description => "empty-description",
            EmptySlot::Content => "empty-content",
            EmptySlot::Media => "empty-icon",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            EmptySlot::Root => "ui-empty",
            EmptySlot::Header => "ui-empty__header",
            EmptySlot::Title => "ui-empty__title",
            EmptySlot::Description => "ui-empty__description",
            EmptySlot::Content => "ui-empty__content",
            EmptySlot::Media => "ui-empty__media",
        }
    }

    pub fn state_attr(self) -> &'static str {
        match self {
            EmptySlot::Root => "root",
            EmptySlot::Header => "header",
            EmptySlot::Title => "title",
            EmptySlot::Description => "description",
            EmptySlot::Content => "content",
            EmptySlot::Media => "media",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyPartStateInput {
    pub slot: EmptySlot,
    pub media_variant: EmptyMediaVariant,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyPartState {
    pub slot: EmptySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub media_variant: EmptyMediaVariant,
    pub media_variant_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub variant_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: EmptyPartStateInput) -> EmptyPartState {
    EmptyPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: input.slot.state_attr(),
        media_variant: input.media_variant,
        media_variant_attr: input.media_variant.as_attr(),
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        variant_source_attr: if input.slot == EmptySlot::Media
            && input.media_variant != EmptyMediaVariant::Default
        {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: EmptyPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.slot == EmptySlot::Media && state.media_variant == EmptyMediaVariant::Icon {
        classes.push("ui-empty__media--icon".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-empty--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/empty.rs"]
mod tests;
