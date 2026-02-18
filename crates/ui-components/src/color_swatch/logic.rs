use crate::color_swatch::{ColorSwatchState, ColorSwatchStateInput};

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
    IsPrefixed,
    LegacyAlias,
    Default,
}

impl ColorSwatchBoolSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsPrefixed => "is-prefixed",
            Self::LegacyAlias => "legacy-alias",
            Self::Default => "default",
        }
    }
}

pub fn normalize_is_bordered(
    is_bordered: Option<bool>,
    bordered: Option<bool>,
) -> (bool, ColorSwatchBoolSource) {
    if let Some(value) = is_bordered {
        return (value, ColorSwatchBoolSource::IsPrefixed);
    }

    if let Some(value) = bordered {
        return (value, ColorSwatchBoolSource::LegacyAlias);
    }

    (true, ColorSwatchBoolSource::Default)
}

pub fn normalize_is_decorative(
    is_decorative: Option<bool>,
    decorative: Option<bool>,
) -> (bool, ColorSwatchBoolSource) {
    if let Some(value) = is_decorative {
        return (value, ColorSwatchBoolSource::IsPrefixed);
    }

    if let Some(value) = decorative {
        return (value, ColorSwatchBoolSource::LegacyAlias);
    }

    (false, ColorSwatchBoolSource::Default)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::swatch::normalize_optional_text(value)
}

pub fn sanitize_color_value(value: Option<String>) -> Option<String> {
    ui_state_primitives::swatch::sanitize_color_value(value)
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
        ColorSwatchAlpha::None => DEFAULT_ARIA_LABEL.to_string(),
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

    let label = if let Some(context_label) = context_label {
        format!("{name}, {context_label}")
    } else {
        name
    };

    let has_custom_aria_label = custom_color_name.is_some();

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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorSwatchState) -> String {
    let mut classes = vec![
        "ui-color-swatch".to_string(),
        state.size_class.to_string(),
        state.rounding_class.to_string(),
        state.shape_class.to_string(),
        state.alpha_class.to_string(),
    ];

    if state.is_bordered {
        classes.push("ui-color-swatch--bordered".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-swatch--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(color: Option<&str>) -> Option<String> {
    color.map(|color| format!("--ui-color-swatch-color: {color};"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_contracts_are_stable() {
        assert_eq!(ColorSwatchSize::Xs.class_name(), "ui-color-swatch--size-xs");
        assert_eq!(ColorSwatchSize::Md.as_attr(), "md");
        assert_eq!(
            ColorSwatchRounding::Default.class_name(),
            "ui-color-swatch--rounding-default"
        );
        assert_eq!(ColorSwatchRounding::Full.as_attr(), "full");
        assert_eq!(
            ColorSwatchShape::Square.class_name(),
            "ui-color-swatch--shape-square"
        );
        assert_eq!(ColorSwatchShape::Wide.as_attr(), "wide");
        assert_eq!(
            ColorSwatchAlpha::Translucent.class_name(),
            "ui-color-swatch--alpha-translucent"
        );
        assert_eq!(ColorSwatchAlpha::Transparent.as_attr(), "transparent");
    }

    #[test]
    fn normalize_and_sanitize_helpers_drop_invalid_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  #ff0000  ".to_string())),
            Some("#ff0000".to_string())
        );

        assert_eq!(
            sanitize_color_value(Some("  #ff0000  ".to_string())),
            Some("#ff0000".to_string())
        );
        assert_eq!(
            sanitize_color_value(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn is_prefixed_boolean_props_take_precedence_with_legacy_alias_fallback() {
        assert_eq!(
            normalize_is_bordered(Some(false), Some(true)),
            (false, ColorSwatchBoolSource::IsPrefixed)
        );
        assert_eq!(
            normalize_is_bordered(None, Some(false)),
            (false, ColorSwatchBoolSource::LegacyAlias)
        );
        assert_eq!(
            normalize_is_bordered(None, None),
            (true, ColorSwatchBoolSource::Default)
        );

        assert_eq!(
            normalize_is_decorative(Some(true), Some(false)),
            (true, ColorSwatchBoolSource::IsPrefixed)
        );
        assert_eq!(
            normalize_is_decorative(None, Some(true)),
            (true, ColorSwatchBoolSource::LegacyAlias)
        );
        assert_eq!(
            normalize_is_decorative(None, None),
            (false, ColorSwatchBoolSource::Default)
        );
    }

    #[test]
    fn alpha_resolution_supports_hex_and_functional_colors() {
        assert_eq!(resolve_alpha(Some("#FF0000")), ColorSwatchAlpha::Opaque);
        assert_eq!(
            resolve_alpha(Some("#FF000080")),
            ColorSwatchAlpha::Translucent
        );
        assert_eq!(resolve_alpha(Some("#F000")), ColorSwatchAlpha::Transparent);
        assert_eq!(
            resolve_alpha(Some("rgba(255, 0, 0, 0.25)")),
            ColorSwatchAlpha::Translucent
        );
        assert_eq!(
            resolve_alpha(Some("hsl(0 100% 50% / 0%)")),
            ColorSwatchAlpha::Transparent
        );
        assert_eq!(resolve_alpha(None), ColorSwatchAlpha::None);
    }

    #[test]
    fn aria_label_uses_color_name_and_context_when_provided() {
        let (label, is_custom) = normalize_aria_label(
            Some("Background".to_string()),
            Some("Fire truck red".to_string()),
            Some("#f00"),
            ColorSwatchAlpha::Opaque,
        );
        assert_eq!(label, "Fire truck red, Background");
        assert!(is_custom);

        let (label, is_custom) = normalize_aria_label(
            None,
            None,
            Some("rgba(255, 0, 0, 0.4)"),
            ColorSwatchAlpha::Translucent,
        );
        assert_eq!(label, "Translucent rgba(255, 0, 0, 0.4)");
        assert!(!is_custom);
    }

    #[test]
    fn state_and_class_composition_track_markers() {
        let state = resolve_state(ColorSwatchStateInput {
            size: ColorSwatchSize::Lg,
            rounding: ColorSwatchRounding::Full,
            shape: ColorSwatchShape::Wide,
            bordered: true,
            alpha: ColorSwatchAlpha::Transparent,
            has_color: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "transparent");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");

        let class_name = compose_class_name(Some("docs-color-swatch".to_string()), state);
        for token in [
            "ui-color-swatch",
            "ui-color-swatch--size-lg",
            "ui-color-swatch--rounding-full",
            "ui-color-swatch--shape-wide",
            "ui-color-swatch--alpha-transparent",
            "ui-color-swatch--bordered",
            "ui-color-swatch--custom-class",
            "docs-color-swatch",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }

        assert_eq!(
            compose_inline_style(Some("#ff0000")),
            Some("--ui-color-swatch-color: #ff0000;".to_string())
        );
    }
}
