#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchSize {
    Xs,
    S,
    #[default]
    M,
    L,
}

impl SwatchSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchSize::Xs => "ui-swatch--size-xs",
            SwatchSize::S => "ui-swatch--size-s",
            SwatchSize::M => "ui-swatch--size-m",
            SwatchSize::L => "ui-swatch--size-l",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchSize::Xs => "xs",
            SwatchSize::S => "s",
            SwatchSize::M => "m",
            SwatchSize::L => "l",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchBorder {
    #[default]
    Default,
    Light,
    None,
}

impl SwatchBorder {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchBorder::Default => "ui-swatch--border-default",
            SwatchBorder::Light => "ui-swatch--border-light",
            SwatchBorder::None => "ui-swatch--border-none",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchBorder::Default => "default",
            SwatchBorder::Light => "light",
            SwatchBorder::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchRounding {
    #[default]
    Default,
    None,
    Full,
}

impl SwatchRounding {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchRounding::Default => "ui-swatch--rounding-default",
            SwatchRounding::None => "ui-swatch--rounding-none",
            SwatchRounding::Full => "ui-swatch--rounding-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchRounding::Default => "default",
            SwatchRounding::None => "none",
            SwatchRounding::Full => "full",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchShape {
    #[default]
    Square,
    Rectangle,
}

impl SwatchShape {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchShape::Square => "ui-swatch--shape-square",
            SwatchShape::Rectangle => "ui-swatch--shape-rectangle",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchShape::Square => "square",
            SwatchShape::Rectangle => "rectangle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchStateInput {
    pub size: SwatchSize,
    pub border: SwatchBorder,
    pub rounding: SwatchRounding,
    pub shape: SwatchShape,
    pub has_color: bool,
    pub nothing: bool,
    pub mixed_value: bool,
    pub disabled: bool,
    pub decorative: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchState {
    pub size: SwatchSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub border: SwatchBorder,
    pub border_class: &'static str,
    pub border_attr: &'static str,
    pub rounding: SwatchRounding,
    pub rounding_class: &'static str,
    pub rounding_attr: &'static str,
    pub shape: SwatchShape,
    pub shape_class: &'static str,
    pub shape_attr: &'static str,
    pub has_color: bool,
    pub show_nothing: bool,
    pub show_mixed_value: bool,
    pub disabled: bool,
    pub decorative: bool,
    pub is_interactive: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(
    aria_label: Option<String>,
    label: Option<String>,
    color: Option<&str>,
    nothing: bool,
    mixed_value: bool,
) -> (String, &'static str) {
    if let Some(custom) = normalize_optional_text(aria_label) {
        return (custom, "custom");
    }

    if let Some(label) = normalize_optional_text(label) {
        return (label, "label");
    }

    if mixed_value {
        return ("Mixed".to_string(), "mixed");
    }

    if nothing {
        return ("No fill".to_string(), "nothing");
    }

    if let Some(color) = color.filter(|color| !color.trim().is_empty()) {
        return (color.to_string(), "color");
    }

    ("Swatch".to_string(), "default")
}

pub fn resolve_state(input: SwatchStateInput) -> SwatchState {
    let show_mixed_value = input.mixed_value;
    let show_nothing = !show_mixed_value && input.nothing;
    let has_color = input.has_color && !show_mixed_value && !show_nothing;
    let is_interactive = !input.disabled && !input.decorative && !show_mixed_value;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if show_mixed_value {
        "mixed"
    } else if show_nothing {
        "nothing"
    } else if has_color {
        "color"
    } else {
        "empty"
    };

    SwatchState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        border: input.border,
        border_class: input.border.class_name(),
        border_attr: input.border.as_attr(),
        rounding: input.rounding,
        rounding_class: input.rounding.class_name(),
        rounding_attr: input.rounding.as_attr(),
        shape: input.shape,
        shape_class: input.shape.class_name(),
        shape_attr: input.shape.as_attr(),
        has_color,
        show_nothing,
        show_mixed_value,
        disabled: input.disabled,
        decorative: input.decorative,
        is_interactive,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SwatchState) -> String {
    let mut classes = vec![
        "ui-swatch".to_string(),
        state.size_class.to_string(),
        state.border_class.to_string(),
        state.rounding_class.to_string(),
        state.shape_class.to_string(),
    ];

    if state.show_mixed_value {
        classes.push("ui-swatch--mixed".to_string());
    }

    if state.show_nothing {
        classes.push("ui-swatch--nothing".to_string());
    }

    if state.disabled {
        classes.push("ui-swatch--disabled".to_string());
    }

    if !state.is_interactive {
        classes.push("ui-swatch--static".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-swatch--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(color: Option<&str>) -> Option<String> {
    color.map(|color| format!("--ui-swatch-color: {color};"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_contracts_are_stable() {
        assert_eq!(SwatchSize::Xs.class_name(), "ui-swatch--size-xs");
        assert_eq!(SwatchSize::L.as_attr(), "l");

        assert_eq!(SwatchBorder::Light.class_name(), "ui-swatch--border-light");
        assert_eq!(SwatchBorder::None.as_attr(), "none");

        assert_eq!(
            SwatchRounding::Default.class_name(),
            "ui-swatch--rounding-default"
        );
        assert_eq!(SwatchRounding::Full.as_attr(), "full");

        assert_eq!(
            SwatchShape::Rectangle.class_name(),
            "ui-swatch--shape-rectangle"
        );
        assert_eq!(SwatchShape::Square.as_attr(), "square");
    }

    #[test]
    fn aria_label_prefers_custom_then_label_then_state_defaults() {
        let (label, source) = resolve_aria_label(
            Some("  Accent  ".to_string()),
            Some("Ignored".to_string()),
            Some("#ff0"),
            false,
            false,
        );
        assert_eq!(label, "Accent");
        assert_eq!(source, "custom");

        let (label, source) =
            resolve_aria_label(None, Some("  Brand  ".to_string()), None, false, false);
        assert_eq!(label, "Brand");
        assert_eq!(source, "label");

        let (label, source) = resolve_aria_label(None, None, None, false, true);
        assert_eq!(label, "Mixed");
        assert_eq!(source, "mixed");

        let (label, source) = resolve_aria_label(None, None, None, true, false);
        assert_eq!(label, "No fill");
        assert_eq!(source, "nothing");
    }

    #[test]
    fn state_resolves_interactive_and_state_markers() {
        let state = resolve_state(SwatchStateInput {
            size: SwatchSize::M,
            border: SwatchBorder::Default,
            rounding: SwatchRounding::Default,
            shape: SwatchShape::Square,
            has_color: true,
            nothing: false,
            mixed_value: false,
            disabled: false,
            decorative: false,
            has_custom_class_name: true,
        });
        assert!(state.has_color);
        assert!(state.is_interactive);
        assert_eq!(state.data_state_attr, "color");

        let state = resolve_state(SwatchStateInput {
            mixed_value: true,
            ..SwatchStateInput {
                size: SwatchSize::M,
                border: SwatchBorder::Default,
                rounding: SwatchRounding::Default,
                shape: SwatchShape::Square,
                has_color: true,
                nothing: true,
                mixed_value: false,
                disabled: false,
                decorative: false,
                has_custom_class_name: false,
            }
        });
        assert!(state.show_mixed_value);
        assert!(!state.has_color);
        assert!(!state.show_nothing);
        assert!(!state.is_interactive);
        assert_eq!(state.data_state_attr, "mixed");
    }

    #[test]
    fn class_and_style_composition_include_expected_tokens() {
        let state = resolve_state(SwatchStateInput {
            size: SwatchSize::L,
            border: SwatchBorder::Light,
            rounding: SwatchRounding::Full,
            shape: SwatchShape::Rectangle,
            has_color: false,
            nothing: true,
            mixed_value: false,
            disabled: true,
            decorative: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-swatch".to_string()), state);
        for token in [
            "ui-swatch",
            "ui-swatch--size-l",
            "ui-swatch--border-light",
            "ui-swatch--rounding-full",
            "ui-swatch--shape-rectangle",
            "ui-swatch--nothing",
            "ui-swatch--disabled",
            "ui-swatch--static",
            "ui-swatch--custom-class",
            "docs-swatch",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }

        assert_eq!(
            compose_inline_style(Some("#ff0000")),
            Some("--ui-swatch-color: #ff0000;".to_string())
        );
    }
}
