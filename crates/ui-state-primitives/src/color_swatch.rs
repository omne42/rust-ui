pub const DEFAULT_ARIA_LABEL: &str = "No color selected";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorSwatchSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl ColorSwatchSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ColorSwatchSize::Xs => "ui-color-swatch--size-xs",
            ColorSwatchSize::Sm => "ui-color-swatch--size-sm",
            ColorSwatchSize::Md => "ui-color-swatch--size-md",
            ColorSwatchSize::Lg => "ui-color-swatch--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ColorSwatchSize::Xs => "xs",
            ColorSwatchSize::Sm => "sm",
            ColorSwatchSize::Md => "md",
            ColorSwatchSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorSwatchRounding {
    #[default]
    Default,
    None,
    Full,
}

impl ColorSwatchRounding {
    pub fn class_name(self) -> &'static str {
        match self {
            ColorSwatchRounding::Default => "ui-color-swatch--rounding-default",
            ColorSwatchRounding::None => "ui-color-swatch--rounding-none",
            ColorSwatchRounding::Full => "ui-color-swatch--rounding-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ColorSwatchRounding::Default => "default",
            ColorSwatchRounding::None => "none",
            ColorSwatchRounding::Full => "full",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorSwatchShape {
    #[default]
    Square,
    Wide,
}

impl ColorSwatchShape {
    pub fn class_name(self) -> &'static str {
        match self {
            ColorSwatchShape::Square => "ui-color-swatch--shape-square",
            ColorSwatchShape::Wide => "ui-color-swatch--shape-wide",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ColorSwatchShape::Square => "square",
            ColorSwatchShape::Wide => "wide",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ColorSwatchAlpha {
    #[default]
    None,
    Opaque,
    Translucent,
    Transparent,
}

impl ColorSwatchAlpha {
    pub fn class_name(self) -> &'static str {
        match self {
            ColorSwatchAlpha::None => "ui-color-swatch--alpha-none",
            ColorSwatchAlpha::Opaque => "ui-color-swatch--alpha-opaque",
            ColorSwatchAlpha::Translucent => "ui-color-swatch--alpha-translucent",
            ColorSwatchAlpha::Transparent => "ui-color-swatch--alpha-transparent",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ColorSwatchAlpha::None => "none",
            ColorSwatchAlpha::Opaque => "opaque",
            ColorSwatchAlpha::Translucent => "translucent",
            ColorSwatchAlpha::Transparent => "transparent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchBoolSource {
    IsProp,
    Default,
}

impl ColorSwatchBoolSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsProp => "is-prop",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchStateInput {
    pub size: ColorSwatchSize,
    pub rounding: ColorSwatchRounding,
    pub shape: ColorSwatchShape,
    pub bordered: bool,
    pub alpha: ColorSwatchAlpha,
    pub has_color: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchState {
    pub size: ColorSwatchSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub rounding: ColorSwatchRounding,
    pub rounding_class: &'static str,
    pub rounding_attr: &'static str,
    pub shape: ColorSwatchShape,
    pub shape_class: &'static str,
    pub shape_attr: &'static str,
    pub alpha: ColorSwatchAlpha,
    pub alpha_class: &'static str,
    pub alpha_attr: &'static str,
    pub is_bordered: bool,
    pub has_color: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_is_bordered(is_bordered: Option<bool>) -> (bool, ColorSwatchBoolSource) {
    if let Some(value) = is_bordered {
        return (value, ColorSwatchBoolSource::IsProp);
    }

    (true, ColorSwatchBoolSource::Default)
}

pub fn normalize_is_decorative(is_decorative: Option<bool>) -> (bool, ColorSwatchBoolSource) {
    if let Some(value) = is_decorative {
        return (value, ColorSwatchBoolSource::IsProp);
    }

    (false, ColorSwatchBoolSource::Default)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    crate::swatch::normalize_optional_text(value)
}

pub fn sanitize_color_value(value: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(value)
}

fn parse_hex_alpha(value: &str) -> Option<f32> {
    let text = value.trim();
    let hex = text.strip_prefix('#')?;

    match hex.len() {
        4 => {
            let a = hex.chars().nth(3)?.to_digit(16)? as u8;
            Some((a as f32 * 17.0) / 255.0)
        }
        8 => {
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(a as f32 / 255.0)
        }
        _ => None,
    }
}

fn parse_alpha_token(token: &str) -> Option<f32> {
    let token = token.trim().trim_end_matches(')').trim();
    if token.is_empty() {
        return None;
    }

    if let Some(percent) = token.strip_suffix('%') {
        let value = percent.trim().parse::<f32>().ok()?;
        return Some((value / 100.0).clamp(0.0, 1.0));
    }

    token.parse::<f32>().ok().map(|value| value.clamp(0.0, 1.0))
}

fn parse_function_alpha(value: &str) -> Option<f32> {
    let text = value.trim().to_ascii_lowercase();
    if !(text.starts_with("rgba(")
        || text.starts_with("hsla(")
        || text.starts_with("hwb(")
        || text.starts_with("rgb(")
        || text.starts_with("hsl("))
    {
        return None;
    }

    if let Some((_, tail)) = text.rsplit_once('/') {
        return parse_alpha_token(tail);
    }

    let (_, args) = text.split_once('(')?;
    let parts: Vec<&str> = args.split(',').collect();
    if parts.len() < 4 {
        return None;
    }

    parse_alpha_token(parts[3])
}

fn alpha_from_value(alpha: f32) -> ColorSwatchAlpha {
    if alpha <= 0.0 {
        ColorSwatchAlpha::Transparent
    } else if alpha >= 1.0 {
        ColorSwatchAlpha::Opaque
    } else {
        ColorSwatchAlpha::Translucent
    }
}

pub fn resolve_alpha(color: Option<&str>) -> ColorSwatchAlpha {
    let Some(color) = color.map(str::trim).filter(|value| !value.is_empty()) else {
        return ColorSwatchAlpha::None;
    };

    if let Some(alpha) = parse_hex_alpha(color) {
        return alpha_from_value(alpha);
    }

    if let Some(alpha) = parse_function_alpha(color) {
        return alpha_from_value(alpha);
    }

    ColorSwatchAlpha::Opaque
}

fn default_color_name(color: Option<&str>, alpha: ColorSwatchAlpha) -> String {
    match alpha {
        ColorSwatchAlpha::None => DEFAULT_ARIA_LABEL.into(),
        ColorSwatchAlpha::Transparent => "Transparent color".to_string(),
        ColorSwatchAlpha::Translucent => {
            if let Some(color) = color {
                format!("Translucent {color}")
            } else {
                "Translucent color".to_string()
            }
        }
        ColorSwatchAlpha::Opaque => {
            if let Some(color) = color {
                format!("Color {color}")
            } else {
                "Color swatch".to_string()
            }
        }
    }
}

pub fn normalize_aria_label(
    aria_label: Option<String>,
    color_name: Option<String>,
    color: Option<&str>,
    alpha: ColorSwatchAlpha,
) -> (String, bool) {
    let context_label = normalize_optional_text(aria_label);
    let custom_color_name = normalize_optional_text(color_name);

    let name = custom_color_name
        .clone()
        .unwrap_or_else(|| default_color_name(color, alpha));

    let has_custom_context = context_label.is_some();
    let has_custom_aria_label = custom_color_name.is_some();

    let label = if let Some(context_label) = context_label {
        format!("{name}, {context_label}")
    } else {
        name
    };

    (label, has_custom_aria_label || has_custom_context)
}

pub fn resolve_state(input: ColorSwatchStateInput) -> ColorSwatchState {
    let data_state_attr = match input.alpha {
        ColorSwatchAlpha::None => "empty",
        ColorSwatchAlpha::Transparent => "transparent",
        ColorSwatchAlpha::Translucent => "translucent",
        ColorSwatchAlpha::Opaque if input.bordered => "framed",
        ColorSwatchAlpha::Opaque => "default",
    };

    ColorSwatchState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        rounding: input.rounding,
        rounding_class: input.rounding.class_name(),
        rounding_attr: input.rounding.as_attr(),
        shape: input.shape,
        shape_class: input.shape.class_name(),
        shape_attr: input.shape.as_attr(),
        alpha: input.alpha,
        alpha_class: input.alpha.class_name(),
        alpha_attr: input.alpha.as_attr(),
        is_bordered: input.bordered,
        has_color: input.has_color,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
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
#[path = "test/color_swatch.rs"]
mod tests;
