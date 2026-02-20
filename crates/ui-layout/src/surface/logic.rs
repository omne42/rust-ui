pub use ui_state_primitives::surface::{
    DEFAULT_ARIA_LABEL, SurfaceElevation, SurfaceState, SurfaceStateInput, SurfaceTone,
    normalize_aria_label, normalize_optional_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceControlInput {
    pub is_bordered: Option<bool>,
    pub is_padded: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceControlState {
    pub bordered: bool,
    pub padded: bool,
    pub bordered_source_attr: &'static str,
    pub padded_source_attr: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceRootInput {
    pub tone: SurfaceTone,
    pub elevation: SurfaceElevation,
    pub control: SurfaceControlInput,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceRootState {
    pub aria_label: String,
    pub class_name: Option<String>,
    pub state: SurfaceState,
    pub bordered_source_attr: &'static str,
    pub padded_source_attr: &'static str,
}

pub fn normalize_control_state(input: SurfaceControlInput) -> SurfaceControlState {
    let (bordered, bordered_source_attr) = input
        .is_bordered
        .map(|value| (value, "is-prop"))
        .unwrap_or((false, "default"));

    let (padded, padded_source_attr) = input
        .is_padded
        .map(|value| (value, "is-prop"))
        .unwrap_or((true, "default"));

    SurfaceControlState {
        bordered,
        padded,
        bordered_source_attr,
        padded_source_attr,
    }
}

pub fn normalize_root_state(input: SurfaceRootInput) -> SurfaceRootState {
    let control = normalize_control_state(input.control);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let state = resolve_state(SurfaceStateInput {
        tone: input.tone,
        elevation: input.elevation,
        bordered: control.bordered,
        padded: control.padded,
        has_custom_aria_label,
        has_custom_class_name,
    });

    SurfaceRootState {
        aria_label,
        class_name,
        state,
        bordered_source_attr: control.bordered_source_attr,
        padded_source_attr: control.padded_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SurfaceState) -> String {
    let mut classes = vec![
        "ui-surface".to_string(),
        state.tone_class.into(),
        state.elevation_class.into(),
    ];

    if state.is_bordered {
        classes.push("ui-surface--bordered".to_string());
    }

    if state.is_padded {
        classes.push("ui-surface--padded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-surface--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
