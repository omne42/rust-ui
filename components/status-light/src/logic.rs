pub use ui_state_primitives::status_light::{
    StatusLightRole, StatusLightState, StatusLightStateInput, StatusLightVariant,
    normalize_optional_text, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightRootInput {
    pub variant: Option<StatusLightVariant>,
    pub role: Option<StatusLightRole>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightRootState {
    pub state: StatusLightState,
    pub class_name: String,
}

pub fn compose_class_name(base_class_name: Option<String>, state: StatusLightState) -> String {
    let mut classes = vec![
        "ui-status-light".to_string(),
        state.variant_class.into(),
        state.state_class.into(),
        state.role_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-status-light--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn normalize_root_state(input: StatusLightRootInput) -> StatusLightRootState {
    let variant = input.variant.unwrap_or_default();
    let class_name = normalize_optional_text(input.class_name);
    let state = resolve_state(StatusLightStateInput {
        variant,
        role: input.role,
        has_custom_class_name: class_name.is_some(),
    });

    StatusLightRootState {
        state,
        class_name: compose_class_name(class_name, state),
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
