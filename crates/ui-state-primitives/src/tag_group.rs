pub use crate::button::normalize_optional_text;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl Tag {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_disabled_tags: bool,
    pub has_removable_tags: bool,
    pub is_invalid: bool,
    pub is_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupItemStateInput {
    pub group_disabled: bool,
    pub supports_removal: bool,
    pub tag_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupItemState {
    pub is_disabled: bool,
    pub is_removable: bool,
    pub disabled_source: TagGroupItemDisabledSource,
    pub removable_source: TagGroupItemRemovableSource,
    pub disabled_source_attr: &'static str,
    pub removable_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupItemDisabledSource {
    None,
    Group,
    Item,
    GroupAndItem,
}

impl TagGroupItemDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            TagGroupItemDisabledSource::None => "none",
            TagGroupItemDisabledSource::Group => "group",
            TagGroupItemDisabledSource::Item => "item",
            TagGroupItemDisabledSource::GroupAndItem => "group-and-item",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupItemRemovableSource {
    Removable,
    Disabled,
    Unsupported,
}

impl TagGroupItemRemovableSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            TagGroupItemRemovableSource::Removable => "removable",
            TagGroupItemRemovableSource::Disabled => "disabled",
            TagGroupItemRemovableSource::Unsupported => "unsupported",
        }
    }
}

pub fn resolve_state(
    tags: &[Tag],
    is_disabled: bool,
    supports_removal: bool,
    is_invalid: bool,
    is_required: bool,
) -> TagGroupState {
    let item_count = tags.len();
    let has_items = item_count > 0;
    let has_disabled_tags = has_items && (is_disabled || tags.iter().any(|tag| tag.disabled));
    let has_removable_tags =
        supports_removal && has_items && tags.iter().any(|tag| !is_disabled && !tag.disabled);

    TagGroupState {
        item_count,
        is_empty: !has_items,
        has_items,
        is_disabled,
        has_disabled_tags,
        has_removable_tags,
        is_invalid,
        is_required,
    }
}

pub fn resolve_item_state(input: TagGroupItemStateInput) -> TagGroupItemState {
    let disabled_source = match (input.group_disabled, input.tag_disabled) {
        (false, false) => TagGroupItemDisabledSource::None,
        (true, false) => TagGroupItemDisabledSource::Group,
        (false, true) => TagGroupItemDisabledSource::Item,
        (true, true) => TagGroupItemDisabledSource::GroupAndItem,
    };
    let is_disabled = disabled_source != TagGroupItemDisabledSource::None;

    let removable_source = if !input.supports_removal {
        TagGroupItemRemovableSource::Unsupported
    } else if is_disabled {
        TagGroupItemRemovableSource::Disabled
    } else {
        TagGroupItemRemovableSource::Removable
    };
    let is_removable = removable_source == TagGroupItemRemovableSource::Removable;

    TagGroupItemState {
        is_disabled,
        is_removable,
        disabled_source,
        removable_source,
        disabled_source_attr: disabled_source.as_attr(),
        removable_source_attr: removable_source.as_attr(),
    }
}

pub fn merge_describedby_ids(
    external: Option<String>,
    description_id: Option<&str>,
    error_id: Option<&str>,
) -> Option<String> {
    let mut ids = Vec::new();

    if let Some(external) = normalize_optional_text(external) {
        ids.push(external);
    }

    if let Some(description_id) = description_id {
        ids.push(description_id.into());
    }

    if let Some(error_id) = error_id {
        ids.push(error_id.into());
    }

    (!ids.is_empty()).then(|| ids.join(" "))
}

#[cfg(test)]
#[path = "test/tag_group.rs"]
mod tests;
