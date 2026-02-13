#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DragAndDropStateInput {
    pub disabled: bool,
    pub has_drop_handler: bool,
    pub has_pick_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DragAndDropState {
    pub is_disabled: bool,
    pub supports_drop: bool,
    pub supports_pick: bool,
    pub state_attr: &'static str,
}

pub fn resolve_state(input: DragAndDropStateInput) -> DragAndDropState {
    let state_attr = if input.disabled {
        "disabled"
    } else if input.has_drop_handler && input.has_pick_handler {
        "drop-and-pick"
    } else if input.has_drop_handler {
        "drop-only"
    } else if input.has_pick_handler {
        "pick-only"
    } else {
        "idle"
    };

    DragAndDropState {
        is_disabled: input.disabled,
        supports_drop: input.has_drop_handler,
        supports_pick: input.has_pick_handler,
        state_attr,
    }
}

pub fn compose_class_name(class_name: Option<String>) -> String {
    let mut classes = vec!["ui-drag-and-drop".to_string()];

    if let Some(class_name) = class_name
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        classes.push(class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_state_tracks_support_matrix() {
        let state = resolve_state(DragAndDropStateInput {
            disabled: false,
            has_drop_handler: true,
            has_pick_handler: true,
        });

        assert!(!state.is_disabled);
        assert!(state.supports_drop);
        assert!(state.supports_pick);
        assert_eq!(state.state_attr, "drop-and-pick");
    }

    #[test]
    fn compose_class_name_keeps_base_class() {
        assert_eq!(compose_class_name(None), "ui-drag-and-drop");
        assert_eq!(
            compose_class_name(Some(" docs-dnd ".to_string())),
            "ui-drag-and-drop docs-dnd"
        );
    }
}
