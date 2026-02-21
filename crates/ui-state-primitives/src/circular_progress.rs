pub const DEFAULT_ARIA_LABEL: &str = "Loading";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularProgressStateInput {
    pub size_px: Option<f64>,
    pub thickness_px: Option<f64>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircularProgressState {
    pub size_px: Option<f64>,
    pub thickness_px: Option<f64>,
    pub has_custom_size: bool,
    pub has_custom_thickness: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub size_source_attr: &'static str,
    pub thickness_source_attr: &'static str,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_aria_label(value: Option<String>, default_aria_label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        let is_custom = label != default_aria_label;
        return (label, is_custom);
    }

    (default_aria_label.into(), false)
}

pub fn sanitize_dimension(value: Option<f64>) -> Option<f64> {
    value.filter(|value| value.is_finite() && *value > 0.0)
}

pub fn resolve_state(input: CircularProgressStateInput) -> CircularProgressState {
    let size_px = sanitize_dimension(input.size_px);
    let thickness_px = sanitize_dimension(input.thickness_px);

    let has_custom_size = size_px.is_some();
    let has_custom_thickness = thickness_px.is_some();

    CircularProgressState {
        size_px,
        thickness_px,
        has_custom_size,
        has_custom_thickness,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        size_source_attr: if has_custom_size { "custom" } else { "default" },
        thickness_source_attr: if has_custom_thickness {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

#[cfg(test)]
#[path = "test/circular_progress.rs"]
mod tests;
