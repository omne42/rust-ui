pub const DEFAULT_DECIMAL_SEPARATOR: &str = ".";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberFormatOptions<'a> {
    pub pad_start: bool,
    pub decimal_separator: &'a str,
    pub decimal_places: Option<u32>,
    pub thousand_separator: Option<&'a str>,
}

impl<'a> Default for NumberFormatOptions<'a> {
    fn default() -> Self {
        Self {
            pad_start: false,
            decimal_separator: DEFAULT_DECIMAL_SEPARATOR,
            decimal_places: None,
            thousand_separator: None,
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_decimal_separator(value: Option<String>) -> (String, bool) {
    if let Some(separator) = normalize_optional_text(value) {
        let is_custom = separator != DEFAULT_DECIMAL_SEPARATOR;
        return (separator, is_custom);
    }

    (DEFAULT_DECIMAL_SEPARATOR.into(), false)
}

pub fn resolve_thousand_separator(value: Option<String>) -> (Option<String>, bool) {
    let value = normalize_optional_text(value);
    let has_custom_thousand_separator = value.is_some();
    (value, has_custom_thousand_separator)
}

pub fn sanitize_decimal_places(value: Option<u32>) -> Option<u32> {
    value.map(|value| value.min(12))
}

pub fn sanitize_number(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberSign {
    Negative,
    Zero,
    Positive,
}

impl NumberSign {
    pub fn class_name(self) -> &'static str {
        match self {
            NumberSign::Negative => "ui-static-number--sign-negative",
            NumberSign::Zero => "ui-static-number--sign-zero",
            NumberSign::Positive => "ui-static-number--sign-positive",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            NumberSign::Negative => "negative",
            NumberSign::Zero => "zero",
            NumberSign::Positive => "positive",
        }
    }
}

pub fn resolve_sign(value: f64) -> NumberSign {
    let value = sanitize_number(value);
    if value < 0.0 {
        NumberSign::Negative
    } else if value > 0.0 {
        NumberSign::Positive
    } else {
        NumberSign::Zero
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaticNumberStateInput {
    pub value: f64,
    pub has_custom_decimal_separator: bool,
    pub has_custom_decimal_places: bool,
    pub has_custom_thousand_separator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticNumberState {
    pub sign: NumberSign,
    pub sign_class: &'static str,
    pub sign_attr: &'static str,
    pub has_custom_decimal_separator: bool,
    pub has_custom_decimal_places: bool,
    pub has_custom_thousand_separator: bool,
    pub has_custom_class_name: bool,
    pub decimal_separator_source_class: &'static str,
    pub decimal_places_source_class: &'static str,
    pub thousand_separator_source_class: &'static str,
    pub decimal_separator_source_attr: &'static str,
    pub decimal_places_source_attr: &'static str,
    pub thousand_separator_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_static_number_state(input: StaticNumberStateInput) -> StaticNumberState {
    let sign = resolve_sign(input.value);

    let (decimal_separator_source_class, decimal_separator_source_attr) =
        if input.has_custom_decimal_separator {
            ("ui-static-number--decimal-separator-custom", "custom")
        } else {
            ("ui-static-number--decimal-separator-default", "default")
        };

    let (decimal_places_source_class, decimal_places_source_attr) =
        if input.has_custom_decimal_places {
            ("ui-static-number--decimal-places-custom", "custom")
        } else {
            ("ui-static-number--decimal-places-auto", "auto")
        };

    let (thousand_separator_source_class, thousand_separator_source_attr) =
        if input.has_custom_thousand_separator {
            ("ui-static-number--thousand-separator-custom", "custom")
        } else {
            ("ui-static-number--thousand-separator-none", "none")
        };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    StaticNumberState {
        sign,
        sign_class: sign.class_name(),
        sign_attr: sign.as_str(),
        has_custom_decimal_separator: input.has_custom_decimal_separator,
        has_custom_decimal_places: input.has_custom_decimal_places,
        has_custom_thousand_separator: input.has_custom_thousand_separator,
        has_custom_class_name: input.has_custom_class_name,
        decimal_separator_source_class,
        decimal_places_source_class,
        thousand_separator_source_class,
        decimal_separator_source_attr,
        decimal_places_source_attr,
        thousand_separator_source_attr,
        class_source_attr,
    }
}

pub fn compose_static_number_class_name(
    base_class_name: Option<String>,
    state: StaticNumberState,
) -> String {
    let mut classes = vec![
        "ui-static-number".to_string(),
        state.sign_class.into(),
        state.decimal_separator_source_class.into(),
        state.decimal_places_source_class.into(),
        state.thousand_separator_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-static-number--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlidingNumberPhase {
    Animated,
    Static,
}

impl SlidingNumberPhase {
    pub fn class_name(self) -> &'static str {
        match self {
            SlidingNumberPhase::Animated => "ui-sliding-number--state-animated",
            SlidingNumberPhase::Static => "ui-sliding-number--state-static",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SlidingNumberPhase::Animated => "animated",
            SlidingNumberPhase::Static => "static",
        }
    }
}

pub fn resolve_sliding_phase(animate: bool) -> SlidingNumberPhase {
    if animate {
        SlidingNumberPhase::Animated
    } else {
        SlidingNumberPhase::Static
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlidingNumberStateInput {
    pub value: f64,
    pub animate: bool,
    pub has_custom_decimal_separator: bool,
    pub has_custom_decimal_places: bool,
    pub has_custom_thousand_separator: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlidingNumberState {
    pub sign: NumberSign,
    pub phase: SlidingNumberPhase,
    pub sign_class: &'static str,
    pub phase_class: &'static str,
    pub sign_attr: &'static str,
    pub phase_attr: &'static str,
    pub has_custom_decimal_separator: bool,
    pub has_custom_decimal_places: bool,
    pub has_custom_thousand_separator: bool,
    pub has_custom_motion: bool,
    pub has_custom_class_name: bool,
    pub is_animated: bool,
    pub is_static: bool,
    pub decimal_separator_source_class: &'static str,
    pub decimal_places_source_class: &'static str,
    pub thousand_separator_source_class: &'static str,
    pub motion_source_class: &'static str,
    pub decimal_separator_source_attr: &'static str,
    pub decimal_places_source_attr: &'static str,
    pub thousand_separator_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_sliding_number_state(input: SlidingNumberStateInput) -> SlidingNumberState {
    let sign = resolve_sign(input.value);
    let phase = resolve_sliding_phase(input.animate);

    let (decimal_separator_source_class, decimal_separator_source_attr) =
        if input.has_custom_decimal_separator {
            ("ui-sliding-number--decimal-separator-custom", "custom")
        } else {
            ("ui-sliding-number--decimal-separator-default", "default")
        };

    let (decimal_places_source_class, decimal_places_source_attr) =
        if input.has_custom_decimal_places {
            ("ui-sliding-number--decimal-places-custom", "custom")
        } else {
            ("ui-sliding-number--decimal-places-auto", "auto")
        };

    let (thousand_separator_source_class, thousand_separator_source_attr) =
        if input.has_custom_thousand_separator {
            ("ui-sliding-number--thousand-separator-custom", "custom")
        } else {
            ("ui-sliding-number--thousand-separator-none", "none")
        };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-sliding-number--motion-custom", "custom")
    } else {
        ("ui-sliding-number--motion-default", "default")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    SlidingNumberState {
        sign,
        phase,
        sign_class: match sign {
            NumberSign::Negative => "ui-sliding-number--sign-negative",
            NumberSign::Zero => "ui-sliding-number--sign-zero",
            NumberSign::Positive => "ui-sliding-number--sign-positive",
        },
        phase_class: phase.class_name(),
        sign_attr: sign.as_str(),
        phase_attr: phase.as_str(),
        has_custom_decimal_separator: input.has_custom_decimal_separator,
        has_custom_decimal_places: input.has_custom_decimal_places,
        has_custom_thousand_separator: input.has_custom_thousand_separator,
        has_custom_motion: input.has_custom_motion,
        has_custom_class_name: input.has_custom_class_name,
        is_animated: phase == SlidingNumberPhase::Animated,
        is_static: phase == SlidingNumberPhase::Static,
        decimal_separator_source_class,
        decimal_places_source_class,
        thousand_separator_source_class,
        motion_source_class,
        decimal_separator_source_attr,
        decimal_places_source_attr,
        thousand_separator_source_attr,
        motion_source_attr,
        class_source_attr,
    }
}

pub fn compose_sliding_number_class_name(
    base_class_name: Option<String>,
    state: SlidingNumberState,
) -> String {
    let mut classes = vec![
        "ui-sliding-number".to_string(),
        state.sign_class.into(),
        state.phase_class.into(),
        state.decimal_separator_source_class.into(),
        state.decimal_places_source_class.into(),
        state.thousand_separator_source_class.into(),
        state.motion_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-sliding-number--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

fn insert_thousand_separators(int_part: &str, sep: &str) -> String {
    if sep.is_empty() {
        return int_part.to_string();
    }

    let mut out = String::with_capacity(int_part.len() + int_part.len() / 3);
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();
    for (idx, ch) in chars.into_iter().enumerate() {
        out.push(ch);
        let remaining = len - idx - 1;
        if remaining > 0 && remaining.is_multiple_of(3) {
            out.push_str(sep);
        }
    }
    out
}

pub fn format_static_number(value: f64, options: NumberFormatOptions<'_>) -> String {
    let value = sanitize_number(value);
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    let mut number = if let Some(places) = options.decimal_places {
        format!("{:.*}", places as usize, abs_value)
    } else {
        abs_value.to_string()
    };

    if options.decimal_separator != DEFAULT_DECIMAL_SEPARATOR
        && let Some(dot) = number.find('.')
    {
        number.replace_range(dot..=dot, options.decimal_separator);
    }

    let (int_part, dec_part) = match number.split_once(options.decimal_separator) {
        Some((int_part, dec_part)) => (int_part, Some(dec_part)),
        None => (number.as_str(), None),
    };

    let int_part = if options.pad_start {
        int_part.to_string()
    } else {
        int_part.trim_start_matches('0').into()
    };
    let int_part = if int_part.is_empty() {
        "0".to_string()
    } else {
        int_part
    };

    let int_part = if let Some(sep) = options.thousand_separator {
        insert_thousand_separators(&int_part, sep)
    } else {
        int_part
    };

    let mut out = String::new();
    if is_negative {
        out.push('-');
    }
    out.push_str(&int_part);
    if let Some(dec_part) = dec_part
        && !dec_part.is_empty()
    {
        out.push_str(options.decimal_separator);
        out.push_str(dec_part);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-static-number  ".to_string())),
            Some("docs-static-number".to_string())
        );
    }

    #[test]
    fn resolve_decimal_separator_defaults_and_detects_custom_source() {
        assert_eq!(
            resolve_decimal_separator(None),
            (DEFAULT_DECIMAL_SEPARATOR.into(), false)
        );
        assert_eq!(
            resolve_decimal_separator(Some(" . ".to_string())),
            (DEFAULT_DECIMAL_SEPARATOR.into(), false)
        );
        assert_eq!(
            resolve_decimal_separator(Some(" , ".to_string())),
            (",".to_string(), true)
        );
    }

    #[test]
    fn resolve_thousand_separator_reports_source() {
        assert_eq!(resolve_thousand_separator(None), (None, false));
        assert_eq!(
            resolve_thousand_separator(Some("\n\t".to_string())),
            (None, false)
        );
        assert_eq!(
            resolve_thousand_separator(Some(" , ".to_string())),
            (Some(",".to_string()), true)
        );
    }

    #[test]
    fn sanitize_decimal_places_caps_at_twelve() {
        assert_eq!(sanitize_decimal_places(None), None);
        assert_eq!(sanitize_decimal_places(Some(2)), Some(2));
        assert_eq!(sanitize_decimal_places(Some(30)), Some(12));
    }

    #[test]
    fn sanitize_number_handles_non_finite_values() {
        assert_eq!(sanitize_number(42.0), 42.0);
        assert_eq!(sanitize_number(f64::NAN), 0.0);
        assert_eq!(sanitize_number(f64::INFINITY), 0.0);
    }

    #[test]
    fn resolve_sign_maps_sign_variants() {
        assert_eq!(resolve_sign(-1.0), NumberSign::Negative);
        assert_eq!(resolve_sign(0.0), NumberSign::Zero);
        assert_eq!(resolve_sign(1.0), NumberSign::Positive);
        assert_eq!(resolve_sign(f64::NAN), NumberSign::Zero);
    }

    #[test]
    fn resolve_static_state_tracks_source_contracts() {
        let state = resolve_static_number_state(StaticNumberStateInput {
            value: -12.3,
            has_custom_decimal_separator: true,
            has_custom_decimal_places: false,
            has_custom_thousand_separator: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.sign, NumberSign::Negative);
        assert_eq!(state.sign_class, "ui-static-number--sign-negative");
        assert_eq!(state.sign_attr, "negative");
        assert_eq!(
            state.decimal_separator_source_class,
            "ui-static-number--decimal-separator-custom"
        );
        assert_eq!(
            state.decimal_places_source_class,
            "ui-static-number--decimal-places-auto"
        );
        assert_eq!(
            state.thousand_separator_source_class,
            "ui-static-number--thousand-separator-custom"
        );
        assert_eq!(state.decimal_separator_source_attr, "custom");
        assert_eq!(state.decimal_places_source_attr, "auto");
        assert_eq!(state.thousand_separator_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_static_class_name_includes_state_markers() {
        let class_name = compose_static_number_class_name(
            Some("docs-static-number-custom".to_string()),
            resolve_static_number_state(StaticNumberStateInput {
                value: 0.0,
                has_custom_decimal_separator: false,
                has_custom_decimal_places: true,
                has_custom_thousand_separator: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-static-number",
            "ui-static-number--sign-zero",
            "ui-static-number--decimal-separator-default",
            "ui-static-number--decimal-places-custom",
            "ui-static-number--thousand-separator-custom",
            "ui-static-number--custom-class",
            "docs-static-number-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn sliding_phase_mappings_are_stable() {
        assert_eq!(
            SlidingNumberPhase::Animated.class_name(),
            "ui-sliding-number--state-animated"
        );
        assert_eq!(
            SlidingNumberPhase::Static.class_name(),
            "ui-sliding-number--state-static"
        );
        assert_eq!(SlidingNumberPhase::Animated.as_str(), "animated");
        assert_eq!(SlidingNumberPhase::Static.as_str(), "static");
    }

    #[test]
    fn resolve_sliding_state_tracks_source_contracts() {
        let state = resolve_sliding_number_state(SlidingNumberStateInput {
            value: -42.0,
            animate: true,
            has_custom_decimal_separator: true,
            has_custom_decimal_places: false,
            has_custom_thousand_separator: true,
            has_custom_motion: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.sign, NumberSign::Negative);
        assert_eq!(state.phase, SlidingNumberPhase::Animated);
        assert_eq!(state.sign_class, "ui-sliding-number--sign-negative");
        assert_eq!(state.phase_class, "ui-sliding-number--state-animated");
        assert_eq!(state.sign_attr, "negative");
        assert_eq!(state.phase_attr, "animated");
        assert!(state.is_animated);
        assert!(!state.is_static);
        assert_eq!(
            state.decimal_separator_source_class,
            "ui-sliding-number--decimal-separator-custom"
        );
        assert_eq!(
            state.decimal_places_source_class,
            "ui-sliding-number--decimal-places-auto"
        );
        assert_eq!(
            state.thousand_separator_source_class,
            "ui-sliding-number--thousand-separator-custom"
        );
        assert_eq!(
            state.motion_source_class,
            "ui-sliding-number--motion-custom"
        );
        assert_eq!(state.decimal_separator_source_attr, "custom");
        assert_eq!(state.decimal_places_source_attr, "auto");
        assert_eq!(state.thousand_separator_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_sliding_class_name_includes_state_markers() {
        let class_name = compose_sliding_number_class_name(
            Some("docs-sliding-number-custom".to_string()),
            resolve_sliding_number_state(SlidingNumberStateInput {
                value: 3.5,
                animate: false,
                has_custom_decimal_separator: true,
                has_custom_decimal_places: true,
                has_custom_thousand_separator: false,
                has_custom_motion: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-sliding-number",
            "ui-sliding-number--sign-positive",
            "ui-sliding-number--state-static",
            "ui-sliding-number--decimal-separator-custom",
            "ui-sliding-number--decimal-places-custom",
            "ui-sliding-number--thousand-separator-none",
            "ui-sliding-number--motion-custom",
            "ui-sliding-number--custom-class",
            "docs-sliding-number-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn formats_negative_and_decimals() {
        let out = format_static_number(
            -12.345,
            NumberFormatOptions {
                decimal_places: Some(2),
                ..Default::default()
            },
        );
        assert_eq!(out, "-12.35");
    }

    #[test]
    fn supports_thousand_separator() {
        let out = format_static_number(
            12345.0,
            NumberFormatOptions {
                thousand_separator: Some(","),
                ..Default::default()
            },
        );
        assert_eq!(out, "12,345");
    }

    #[test]
    fn supports_custom_decimal_separator() {
        let out = format_static_number(
            1.5,
            NumberFormatOptions {
                decimal_separator: ",",
                ..Default::default()
            },
        );
        assert_eq!(out, "1,5");
    }
}
