#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightStateInput {
    pub animate_height: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightState {
    pub overflow_hidden: bool,
    pub animate_height: bool,
    pub is_static: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn resolve_state(input: AutoHeightStateInput) -> AutoHeightState {
    AutoHeightState {
        overflow_hidden: true,
        animate_height: input.animate_height,
        is_static: !input.animate_height,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
    }
}

#[cfg(test)]
#[path = "test/auto_height.rs"]
mod tests;
