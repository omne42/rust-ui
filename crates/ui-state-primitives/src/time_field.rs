pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldDisabledStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_disabled_state(input: TimeFieldDisabledStateInput) -> bool {
    input.is_disabled.unwrap_or(input.disabled)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_value_change_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldValueAxisState {
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn resolve_value_axis_state(input: TimeFieldValueAxisInput) -> TimeFieldValueAxisState {
    TimeFieldValueAxisState {
        is_controlled: input.is_controlled,
        control_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        default_value_source_attr: source_attr_from_presence(input.has_default_value),
        value_change_source_attr: if input.has_value_change_handler {
            "on_value_change"
        } else {
            "none"
        },
        has_value_change_handler: input.has_value_change_handler,
    }
}

#[cfg(test)]
#[path = "test/time_field.rs"]
mod tests;
