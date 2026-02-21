#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OverlaysLayerKind {
    #[default]
    Stack,
    Modal,
    NonModal,
}

impl OverlaysLayerKind {
    pub fn class_name(self) -> &'static str {
        match self {
            OverlaysLayerKind::Stack => "ui-overlays--stack",
            OverlaysLayerKind::Modal => "ui-overlays--modal",
            OverlaysLayerKind::NonModal => "ui-overlays--non-modal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            OverlaysLayerKind::Stack => "stack",
            OverlaysLayerKind::Modal => "modal",
            OverlaysLayerKind::NonModal => "non-modal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlaysRootStateInput {
    pub open: bool,
    pub modal: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlaysRootState {
    pub layer_kind: OverlaysLayerKind,
    pub layer_kind_attr: &'static str,
    pub layer_kind_class: &'static str,
    pub is_open: bool,
    pub is_closed: bool,
    pub data_state_attr: &'static str,
    pub id_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
}

pub const DEFAULT_ID_BASE: &str = "overlays-root";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> (String, bool) {
    if let Some(id_base) = normalize_optional_text(value) {
        return (id_base, true);
    }

    (DEFAULT_ID_BASE.into(), false)
}

pub fn resolve_root_state(input: OverlaysRootStateInput) -> OverlaysRootState {
    let layer_kind = if input.modal {
        OverlaysLayerKind::Modal
    } else {
        OverlaysLayerKind::NonModal
    };

    OverlaysRootState {
        layer_kind,
        layer_kind_attr: layer_kind.as_attr(),
        layer_kind_class: layer_kind.class_name(),
        is_open: input.open,
        is_closed: !input.open,
        data_state_attr: if input.open { "open" } else { "closed" },
        id_source_attr: if input.has_custom_id_base {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_id_base: input.has_custom_id_base,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "test/overlays.rs"]
mod tests;
