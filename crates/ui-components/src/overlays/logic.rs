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

pub fn compose_root_class_name(
    base_class_name: Option<String>,
    state: OverlaysRootState,
) -> String {
    let mut classes = vec!["ui-overlays".to_string(), state.layer_kind_class.into()];

    if state.is_open {
        classes.push("ui-overlays--open".to_string());
    } else {
        classes.push("ui-overlays--closed".to_string());
    }

    if state.has_custom_id_base {
        classes.push("ui-overlays--custom-id".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-overlays--custom-class".to_string());
        if let Some(base_class_name) = normalize_optional_text(base_class_name) {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_id_base_uses_default_for_empty_values() {
        assert_eq!(
            normalize_id_base(Some("  docs-overlays  ".to_string())),
            ("docs-overlays".to_string(), true)
        );
        assert_eq!(
            normalize_id_base(Some(" \n\t ".to_string())),
            (DEFAULT_ID_BASE.into(), false)
        );
        assert_eq!(normalize_id_base(None), (DEFAULT_ID_BASE.into(), false));
    }

    #[test]
    fn resolve_root_state_tracks_state_and_sources() {
        let state = resolve_root_state(OverlaysRootStateInput {
            open: true,
            modal: true,
            has_custom_id_base: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.layer_kind_attr, "modal");
        assert_eq!(state.data_state_attr, "open");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert!(state.is_open);
        assert!(!state.is_closed);
    }

    #[test]
    fn compose_root_class_name_includes_markers() {
        let class_name = compose_root_class_name(
            Some("docs-overlays".to_string()),
            resolve_root_state(OverlaysRootStateInput {
                open: false,
                modal: false,
                has_custom_id_base: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-overlays",
            "ui-overlays--non-modal",
            "ui-overlays--closed",
            "ui-overlays--custom-id",
            "ui-overlays--custom-class",
            "docs-overlays",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
