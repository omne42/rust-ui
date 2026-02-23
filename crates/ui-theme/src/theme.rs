use crate::tokens::{
    ACCORDION_MOTION_TOKENS_LARGE, ACCORDION_MOTION_TOKENS_MEDIUM, AccordionMotionTokens,
    BUTTON_LAYOUT_TOKENS_LARGE, BUTTON_LAYOUT_TOKENS_MEDIUM, BUTTON_MOTION_TOKENS_LARGE,
    BUTTON_MOTION_TOKENS_MEDIUM, ButtonLayoutTokens, ButtonMotionTokens,
    CHECKBOX_GROUP_LAYOUT_TOKENS_LARGE, CHECKBOX_GROUP_LAYOUT_TOKENS_MEDIUM,
    CHECKBOX_GROUP_MOTION_TOKENS_LARGE, CHECKBOX_GROUP_MOTION_TOKENS_MEDIUM,
    CHECKBOX_LAYOUT_TOKENS_LARGE, CHECKBOX_LAYOUT_TOKENS_MEDIUM, COLOR_SWATCH_LAYOUT_TOKENS_LARGE,
    COLOR_SWATCH_LAYOUT_TOKENS_MEDIUM, COLOR_WHEEL_HUE_TOKENS, COLOR_WHEEL_LAYOUT_TOKENS_LARGE,
    COLOR_WHEEL_LAYOUT_TOKENS_MEDIUM, COMMAND_LAYOUT_TOKENS_LARGE, COMMAND_LAYOUT_TOKENS_MEDIUM,
    CheckboxGroupLayoutTokens, CheckboxGroupMotionTokens, CheckboxLayoutTokens, ColorAliasTokens,
    ColorPaletteTokens, ColorScaleTokens, ColorSwatchLayoutTokens, ColorWheelHueTokens,
    ColorWheelLayoutTokens, CommandLayoutTokens, CommonColorScales, ComponentColorTokens,
    ComponentLayoutTokens, DROP_ZONE_LAYOUT_TOKENS_LARGE, DROP_ZONE_LAYOUT_TOKENS_MEDIUM,
    DROP_ZONE_MOTION_TOKENS_LARGE, DROP_ZONE_MOTION_TOKENS_MEDIUM, DropZoneLayoutTokens,
    DropZoneMotionTokens, FLIP_CARD_LAYOUT_TOKENS_LARGE, FLIP_CARD_LAYOUT_TOKENS_MEDIUM,
    FlipCardLayoutTokens, IconTokens, LABEL_MOTION_TOKENS_LARGE, LABEL_MOTION_TOKENS_MEDIUM,
    LabelMotionTokens, LayoutSemanticTokens, LayoutTokens, OverlayLayoutTokens, RadiusTokens,
    SLIDER_LAYOUT_TOKENS_LARGE, SLIDER_LAYOUT_TOKENS_MEDIUM, SLIDER_MOTION_TOKENS_LARGE,
    SLIDER_MOTION_TOKENS_MEDIUM, SWATCH_MOTION_TOKENS_LARGE, SWATCH_MOTION_TOKENS_MEDIUM,
    SWITCH_LAYOUT_TOKENS_LARGE, SWITCH_LAYOUT_TOKENS_MEDIUM, SWITCH_MOTION_TOKENS_LARGE,
    SWITCH_MOTION_TOKENS_MEDIUM, SemanticColorTokens, SemanticRoleTokens, SemanticScaleTokens,
    ShadowTokens, SliderLayoutTokens, SliderMotionTokens, SpaceTokens, SwatchMotionTokens,
    SwitchLayoutTokens, SwitchMotionTokens, TEXT_FIELD_MOTION_TOKENS_LARGE,
    TEXT_FIELD_MOTION_TOKENS_MEDIUM, TEXTAREA_MOTION_TOKENS_LARGE, TEXTAREA_MOTION_TOKENS_MEDIUM,
    TIME_FIELD_MOTION_TOKENS_LARGE, TIME_FIELD_MOTION_TOKENS_MEDIUM, TextFieldMotionTokens,
    TextareaMotionTokens, ThemeTokens, TimeFieldMotionTokens, TokenScale, TypographyTokens,
    UNDERLAY_MOTION_TOKENS_LARGE, UNDERLAY_MOTION_TOKENS_MEDIUM, UnderlayMotionTokens,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeSystem {
    Baseline,
    Express,
    BaselineTwo,
}

impl ThemeSystem {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Express => "express",
            Self::BaselineTwo => "baseline-two",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeColor {
    Light,
    Dark,
    Oled,
}

impl ThemeColor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Oled => "oled",
        }
    }

    pub fn css_color_scheme(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark | Self::Oled => "dark",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeScale {
    Medium,
    Large,
}

impl ThemeScale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }

    pub fn token_scale(self) -> TokenScale {
        match self {
            Self::Medium => TokenScale::Medium,
            Self::Large => TokenScale::Large,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeContext {
    pub system: ThemeSystem,
    pub color: ThemeColor,
    pub scale: ThemeScale,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            // Default to baseline-v2 + Medium so apps/components have a stable baseline.
            system: ThemeSystem::BaselineTwo,
            color: ThemeColor::Light,
            scale: ThemeScale::Medium,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub ctx: ThemeContext,
    pub tokens: ThemeTokens,
}

impl Theme {
    pub fn new(ctx: ThemeContext) -> Self {
        Self {
            ctx,
            tokens: resolve_tokens(ctx),
        }
    }

    pub fn light() -> Self {
        Self::new(ThemeContext {
            color: ThemeColor::Light,
            ..Default::default()
        })
    }

    pub fn dark() -> Self {
        Self::new(ThemeContext {
            color: ThemeColor::Dark,
            ..Default::default()
        })
    }

    pub fn oled() -> Self {
        Self::new(ThemeContext {
            color: ThemeColor::Oled,
            ..Default::default()
        })
    }

    pub fn baseline(color: ThemeColor, scale: ThemeScale) -> Self {
        Self::new(ThemeContext {
            system: ThemeSystem::Baseline,
            color,
            scale,
        })
    }

    pub fn express(color: ThemeColor, scale: ThemeScale) -> Self {
        Self::new(ThemeContext {
            system: ThemeSystem::Express,
            color,
            scale,
        })
    }

    pub fn baseline_two(color: ThemeColor, scale: ThemeScale) -> Self {
        Self::new(ThemeContext {
            system: ThemeSystem::BaselineTwo,
            color,
            scale,
        })
    }

    pub fn to_css_variables(&self) -> String {
        crate::css::theme_to_css_variables(self)
    }
}

fn color_scale(values: [&'static str; 10]) -> ColorScaleTokens {
    ColorScaleTokens {
        shade_50: values[0],
        shade_100: values[1],
        shade_200: values[2],
        shade_300: values[3],
        shade_400: values[4],
        shade_500: values[5],
        shade_600: values[6],
        shade_700: values[7],
        shade_800: values[8],
        shade_900: values[9],
    }
}

fn invert_scale(scale: ColorScaleTokens) -> ColorScaleTokens {
    ColorScaleTokens {
        shade_50: scale.shade_900,
        shade_100: scale.shade_800,
        shade_200: scale.shade_700,
        shade_300: scale.shade_600,
        shade_400: scale.shade_500,
        shade_500: scale.shade_400,
        shade_600: scale.shade_300,
        shade_700: scale.shade_200,
        shade_800: scale.shade_100,
        shade_900: scale.shade_50,
    }
}

pub fn accordion_motion_tokens(ctx: ThemeContext) -> AccordionMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => ACCORDION_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => ACCORDION_MOTION_TOKENS_LARGE,
    }
}

pub fn default_accordion_motion_tokens() -> AccordionMotionTokens {
    accordion_motion_tokens(ThemeContext::default())
}

pub fn button_motion_tokens(ctx: ThemeContext) -> ButtonMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => BUTTON_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => BUTTON_MOTION_TOKENS_LARGE,
    }
}

pub fn default_button_motion_tokens() -> ButtonMotionTokens {
    button_motion_tokens(ThemeContext::default())
}

pub fn swatch_motion_tokens(ctx: ThemeContext) -> SwatchMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => SWATCH_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => SWATCH_MOTION_TOKENS_LARGE,
    }
}

pub fn default_swatch_motion_tokens() -> SwatchMotionTokens {
    swatch_motion_tokens(ThemeContext::default())
}

pub fn switch_motion_tokens(ctx: ThemeContext) -> SwitchMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => SWITCH_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => SWITCH_MOTION_TOKENS_LARGE,
    }
}

pub fn default_switch_motion_tokens() -> SwitchMotionTokens {
    switch_motion_tokens(ThemeContext::default())
}

pub fn slider_motion_tokens(ctx: ThemeContext) -> SliderMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => SLIDER_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => SLIDER_MOTION_TOKENS_LARGE,
    }
}

pub fn default_slider_motion_tokens() -> SliderMotionTokens {
    slider_motion_tokens(ThemeContext::default())
}

pub fn drop_zone_motion_tokens(ctx: ThemeContext) -> DropZoneMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => DROP_ZONE_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => DROP_ZONE_MOTION_TOKENS_LARGE,
    }
}

pub fn default_drop_zone_motion_tokens() -> DropZoneMotionTokens {
    drop_zone_motion_tokens(ThemeContext::default())
}

pub fn drop_zone_layout_tokens(ctx: ThemeContext) -> DropZoneLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => DROP_ZONE_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => DROP_ZONE_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_drop_zone_layout_tokens() -> DropZoneLayoutTokens {
    drop_zone_layout_tokens(ThemeContext::default())
}

pub fn flip_card_layout_tokens(ctx: ThemeContext) -> FlipCardLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => FLIP_CARD_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => FLIP_CARD_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_flip_card_layout_tokens() -> FlipCardLayoutTokens {
    flip_card_layout_tokens(ThemeContext::default())
}

pub fn underlay_motion_tokens(ctx: ThemeContext) -> UnderlayMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => UNDERLAY_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => UNDERLAY_MOTION_TOKENS_LARGE,
    }
}

pub fn default_underlay_motion_tokens() -> UnderlayMotionTokens {
    underlay_motion_tokens(ThemeContext::default())
}

pub fn time_field_motion_tokens(ctx: ThemeContext) -> TimeFieldMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => TIME_FIELD_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => TIME_FIELD_MOTION_TOKENS_LARGE,
    }
}

pub fn default_time_field_motion_tokens() -> TimeFieldMotionTokens {
    time_field_motion_tokens(ThemeContext::default())
}

pub fn textarea_motion_tokens(ctx: ThemeContext) -> TextareaMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => TEXTAREA_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => TEXTAREA_MOTION_TOKENS_LARGE,
    }
}

pub fn default_textarea_motion_tokens() -> TextareaMotionTokens {
    textarea_motion_tokens(ThemeContext::default())
}

pub fn text_field_motion_tokens(ctx: ThemeContext) -> TextFieldMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => TEXT_FIELD_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => TEXT_FIELD_MOTION_TOKENS_LARGE,
    }
}

pub fn default_text_field_motion_tokens() -> TextFieldMotionTokens {
    text_field_motion_tokens(ThemeContext::default())
}

pub fn label_motion_tokens(ctx: ThemeContext) -> LabelMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => LABEL_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => LABEL_MOTION_TOKENS_LARGE,
    }
}

pub fn default_label_motion_tokens() -> LabelMotionTokens {
    label_motion_tokens(ThemeContext::default())
}

pub fn checkbox_group_motion_tokens(ctx: ThemeContext) -> CheckboxGroupMotionTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => CHECKBOX_GROUP_MOTION_TOKENS_MEDIUM,
        TokenScale::Large => CHECKBOX_GROUP_MOTION_TOKENS_LARGE,
    }
}

pub fn default_checkbox_group_motion_tokens() -> CheckboxGroupMotionTokens {
    checkbox_group_motion_tokens(ThemeContext::default())
}

pub fn button_layout_tokens(ctx: ThemeContext) -> ButtonLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => BUTTON_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => BUTTON_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_button_layout_tokens() -> ButtonLayoutTokens {
    button_layout_tokens(ThemeContext::default())
}

pub fn checkbox_layout_tokens(ctx: ThemeContext) -> CheckboxLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => CHECKBOX_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => CHECKBOX_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_checkbox_layout_tokens() -> CheckboxLayoutTokens {
    checkbox_layout_tokens(ThemeContext::default())
}

pub fn checkbox_group_layout_tokens(ctx: ThemeContext) -> CheckboxGroupLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => CHECKBOX_GROUP_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => CHECKBOX_GROUP_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_checkbox_group_layout_tokens() -> CheckboxGroupLayoutTokens {
    checkbox_group_layout_tokens(ThemeContext::default())
}

pub fn command_layout_tokens(ctx: ThemeContext) -> CommandLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => COMMAND_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => COMMAND_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_command_layout_tokens() -> CommandLayoutTokens {
    command_layout_tokens(ThemeContext::default())
}

pub fn slider_layout_tokens(ctx: ThemeContext) -> SliderLayoutTokens {
    resolve_tokens(ctx).slider_layout
}

pub fn default_slider_layout_tokens() -> SliderLayoutTokens {
    slider_layout_tokens(ThemeContext::default())
}

pub fn switch_layout_tokens(ctx: ThemeContext) -> SwitchLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => SWITCH_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => SWITCH_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_switch_layout_tokens() -> SwitchLayoutTokens {
    switch_layout_tokens(ThemeContext::default())
}

pub fn color_swatch_layout_tokens(ctx: ThemeContext) -> ColorSwatchLayoutTokens {
    match ctx.scale.token_scale() {
        TokenScale::Medium => COLOR_SWATCH_LAYOUT_TOKENS_MEDIUM,
        TokenScale::Large => COLOR_SWATCH_LAYOUT_TOKENS_LARGE,
    }
}

pub fn default_color_swatch_layout_tokens() -> ColorSwatchLayoutTokens {
    color_swatch_layout_tokens(ThemeContext::default())
}

pub fn color_wheel_layout_tokens(ctx: ThemeContext) -> ColorWheelLayoutTokens {
    resolve_tokens(ctx).color_wheel_layout
}

pub fn default_color_wheel_layout_tokens() -> ColorWheelLayoutTokens {
    color_wheel_layout_tokens(ThemeContext::default())
}

pub fn color_wheel_hue_tokens(ctx: ThemeContext) -> ColorWheelHueTokens {
    resolve_tokens(ctx).color_wheel_hue
}

pub fn default_color_wheel_hue_tokens() -> ColorWheelHueTokens {
    color_wheel_hue_tokens(ThemeContext::default())
}

pub fn overlay_layout_tokens(ctx: ThemeContext) -> OverlayLayoutTokens {
    resolve_tokens(ctx).overlay_layout
}

pub fn default_overlay_layout_tokens() -> OverlayLayoutTokens {
    overlay_layout_tokens(ThemeContext::default())
}

fn resolve_tokens(ctx: ThemeContext) -> ThemeTokens {
    const FEATURED_SNOW: &str = "oklch(99.11% 0 0)";
    const FEATURED_ECLIPSE: &str = "oklch(21.03% 0.0059 285.89)";
    // Align primary/focus token with upstream #006FEE baseline.
    const FEATURED_ACCENT: &str = "oklch(56.71% 0.2095 257.94)";
    const FEATURED_LIGHT_BACKGROUND: &str = "oklch(97.02% 0 0)";
    // Slightly darker than upstream muted token to keep WCAG AA (>= 4.5) against light background.
    const FEATURED_LIGHT_MUTED: &str = "oklch(53.5% 0.0138 285.94)";
    const FEATURED_LIGHT_DEFAULT: &str = "oklch(94% 0.001 286.375)";
    const FEATURED_LIGHT_SUCCESS: &str = "oklch(73.29% 0.1935 150.81)";
    const FEATURED_LIGHT_WARNING: &str = "oklch(78.19% 0.1585 72.33)";
    const FEATURED_LIGHT_DANGER: &str = "oklch(65.32% 0.2328 25.74)";
    const FEATURED_LIGHT_BORDER: &str = "oklch(90% 0.004 286.32)";
    const FEATURED_LIGHT_SEPARATOR: &str = "oklch(92% 0.004 286.32)";
    const FEATURED_DARK_BACKGROUND: &str = "oklch(12% 0.005 285.823)";
    const FEATURED_DARK_MUTED: &str = "oklch(70.5% 0.015 286.067)";
    const FEATURED_DARK_DEFAULT: &str = "oklch(27.4% 0.006 286.033)";
    const FEATURED_DARK_WARNING: &str = "oklch(82.03% 0.1388 76.34)";
    // Slightly darker than upstream dark danger token to keep WCAG AA (>= 4.5) with light foreground.
    const FEATURED_DARK_DANGER: &str = "oklch(57% 0.1967 24.63)";
    const FEATURED_DARK_SEGMENT: &str = "oklch(39.64% 0.01 285.93)";
    const FEATURED_DARK_BORDER: &str = "oklch(28% 0.006 286.033)";
    const FEATURED_DARK_SEPARATOR: &str = "oklch(25% 0.006 286.033)";

    let default_scale_light = color_scale([
        "oklch(98.51% 0 0)",
        "oklch(96.74% 0.0013 286.38)",
        "oklch(91.97% 0.0040 286.32)",
        "oklch(87.11% 0.0055 286.29)",
        "oklch(71.18% 0.0129 286.07)",
        "oklch(55.17% 0.0138 285.94)",
        "oklch(44.19% 0.0146 285.79)",
        "oklch(37.03% 0.0119 285.81)",
        "oklch(27.39% 0.0055 286.03)",
        "oklch(21.03% 0.0059 285.89)",
    ]);
    let primary_scale_light = color_scale([
        "oklch(95.37% 0.0211 252.50)",
        "oklch(90.70% 0.0434 251.52)",
        "oklch(81.59% 0.0885 251.95)",
        "oklch(72.66% 0.1349 253.30)",
        "oklch(64.68% 0.1781 254.76)",
        "oklch(56.71% 0.2095 257.94)",
        "oklch(49.20% 0.1800 257.73)",
        "oklch(40.12% 0.1436 257.21)",
        "oklch(30.74% 0.1034 255.59)",
        "oklch(20.33% 0.0604 251.77)",
    ]);
    let secondary_scale_light = color_scale([
        "oklch(94.79% 0.0229 308.19)",
        "oklch(89.27% 0.0465 307.88)",
        "oklch(78.49% 0.0953 307.10)",
        "oklch(67.84% 0.1451 305.94)",
        "oklch(57.67% 0.1916 304.03)",
        "oklch(48.78% 0.2254 300.51)",
        "oklch(41.66% 0.1893 300.81)",
        "oklch(34.24% 0.1511 301.27)",
        "oklch(26.42% 0.1100 302.06)",
        "oklch(18.07% 0.0636 303.75)",
    ]);
    let success_scale_light = color_scale([
        "oklch(96.91% 0.0230 161.75)",
        "oklch(93.66% 0.0452 160.37)",
        "oklch(87.62% 0.0903 159.15)",
        "oklch(82.35% 0.1328 156.79)",
        "oklch(77.44% 0.1681 154.24)",
        "oklch(73.29% 0.1935 150.81)",
        "oklch(62.21% 0.1627 151.05)",
        "oklch(50.61% 0.1302 151.36)",
        "oklch(37.99% 0.0941 152.28)",
        "oklch(24.57% 0.0551 154.14)",
    ]);
    let warning_scale_light = color_scale([
        "oklch(98.73% 0.0262 102.21)",
        "oklch(95.21% 0.0383 80.03)",
        "oklch(90.58% 0.0758 79.61)",
        "oklch(86.19% 0.1096 78.33)",
        "oklch(82.03% 0.1388 76.34)",
        "oklch(78.19% 0.1585 72.33)",
        "oklch(66.29% 0.1331 72.64)",
        "oklch(53.83% 0.1064 73.12)",
        "oklch(40.59% 0.0781 74.21)",
        "oklch(26.16% 0.0461 76.33)",
    ]);
    let danger_scale_light = color_scale([
        "oklch(94.87% 0.0271 354.51)",
        "oklch(90.04% 0.0540 356.25)",
        "oklch(80.41% 0.1127 357.73)",
        "oklch(72.22% 0.1710 0.42)",
        "oklch(65.63% 0.2185 4.63)",
        "oklch(61.92% 0.2419 11.33)",
        "oklch(52.39% 0.2037 10.67)",
        "oklch(42.65% 0.1640 9.91)",
        "oklch(32.05% 0.1208 8.83)",
        "oklch(20.85% 0.0735 5.57)",
    ]);
    let pink_scale_light = color_scale([
        "oklch(96.41% 0.0258 335.44)",
        "oklch(93.08% 0.0504 336.24)",
        "oklch(86.36% 0.1042 337.17)",
        "oklch(80.40% 0.1560 338.43)",
        "oklch(75.10% 0.2059 339.76)",
        "oklch(71.01% 0.2456 341.35)",
        "oklch(60.18% 0.2062 341.23)",
        "oklch(48.93% 0.1634 341.09)",
        "oklch(36.91% 0.1189 340.74)",
        "oklch(23.99% 0.0671 340.08)",
    ]);
    let cyan_scale_light = color_scale([
        "oklch(98.30% 0.0133 214.36)",
        "oklch(97.16% 0.0216 211.05)",
        "oklch(95.74% 0.0351 209.81)",
        "oklch(93.60% 0.0514 210.01)",
        "oklch(90.62% 0.0749 211.86)",
        "oklch(87.23% 0.1016 212.16)",
        "oklch(72.02% 0.1294 218.76)",
        "oklch(68.28% 0.1229 219.72)",
        "oklch(58.81% 0.1064 222.33)",
        "oklch(32.65% 0.0563 219.14)",
    ]);
    let common_colors = CommonColorScales {
        white: "oklch(100% 0 0)",
        black: "oklch(0% 0 0)",
        blue: primary_scale_light,
        purple: secondary_scale_light,
        green: success_scale_light,
        red: danger_scale_light,
        pink: pink_scale_light,
        yellow: warning_scale_light,
        cyan: cyan_scale_light,
        zinc: default_scale_light,
    };

    let semantic_scales = match ctx.color {
        ThemeColor::Light => SemanticScaleTokens {
            default: default_scale_light,
            primary: primary_scale_light,
            secondary: secondary_scale_light,
            success: success_scale_light,
            warning: warning_scale_light,
            danger: danger_scale_light,
        },
        ThemeColor::Dark | ThemeColor::Oled => SemanticScaleTokens {
            default: invert_scale(default_scale_light),
            primary: invert_scale(primary_scale_light),
            secondary: invert_scale(secondary_scale_light),
            success: invert_scale(success_scale_light),
            warning: invert_scale(warning_scale_light),
            danger: invert_scale(danger_scale_light),
        },
    };

    let layout_semantic = match ctx.color {
        ThemeColor::Light => LayoutSemanticTokens {
            background: FEATURED_LIGHT_BACKGROUND,
            foreground: FEATURED_ECLIPSE,
            divider: FEATURED_LIGHT_SEPARATOR,
            focus: FEATURED_ACCENT,
            content_1: "oklch(100% 0 0)",
            content_2: FEATURED_LIGHT_DEFAULT,
            content_3: FEATURED_LIGHT_SEPARATOR,
            content_4: FEATURED_LIGHT_BORDER,
        },
        ThemeColor::Dark => LayoutSemanticTokens {
            background: FEATURED_DARK_BACKGROUND,
            foreground: FEATURED_SNOW,
            divider: FEATURED_DARK_SEPARATOR,
            focus: FEATURED_ACCENT,
            content_1: FEATURED_ECLIPSE,
            content_2: FEATURED_DARK_DEFAULT,
            content_3: FEATURED_DARK_SEGMENT,
            content_4: FEATURED_DARK_BORDER,
        },
        ThemeColor::Oled => LayoutSemanticTokens {
            background: "oklch(0% 0 0)",
            foreground: FEATURED_SNOW,
            divider: "oklch(100% 0 0 / 0.18)",
            focus: FEATURED_ACCENT,
            content_1: "oklch(0% 0 0)",
            content_2: FEATURED_ECLIPSE,
            content_3: FEATURED_DARK_DEFAULT,
            content_4: FEATURED_DARK_SEGMENT,
        },
    };

    let semantic_roles = match ctx.color {
        ThemeColor::Light => SemanticRoleTokens {
            default: FEATURED_LIGHT_DEFAULT,
            default_fg: FEATURED_ECLIPSE,
            primary: FEATURED_ACCENT,
            primary_fg: FEATURED_SNOW,
            secondary: semantic_scales.secondary.shade_500,
            secondary_fg: FEATURED_SNOW,
            success: FEATURED_LIGHT_SUCCESS,
            success_fg: FEATURED_ECLIPSE,
            warning: FEATURED_LIGHT_WARNING,
            warning_fg: FEATURED_ECLIPSE,
            danger: FEATURED_LIGHT_DANGER,
            danger_fg: FEATURED_ECLIPSE,
        },
        ThemeColor::Dark | ThemeColor::Oled => SemanticRoleTokens {
            default: FEATURED_DARK_DEFAULT,
            default_fg: FEATURED_SNOW,
            primary: FEATURED_ACCENT,
            primary_fg: FEATURED_SNOW,
            secondary: semantic_scales.secondary.shade_500,
            secondary_fg: FEATURED_SNOW,
            success: FEATURED_LIGHT_SUCCESS,
            success_fg: FEATURED_ECLIPSE,
            warning: FEATURED_DARK_WARNING,
            warning_fg: FEATURED_ECLIPSE,
            danger: FEATURED_DARK_DANGER,
            danger_fg: FEATURED_SNOW,
        },
    };

    let palette = ColorPaletteTokens {
        gray_50: semantic_scales.default.shade_50,
        gray_200: semantic_scales.default.shade_200,
        gray_700: semantic_scales.default.shade_700,
        gray_900: semantic_scales.default.shade_900,
        accent_500: semantic_scales.primary.shade_500,
        accent_600: semantic_scales.primary.shade_600,
        accent_700: semantic_scales.primary.shade_700,
    };

    let semantic_colors = SemanticColorTokens {
        fg: layout_semantic.foreground,
        fg_muted: if matches!(ctx.color, ThemeColor::Light) {
            FEATURED_LIGHT_MUTED
        } else {
            FEATURED_DARK_MUTED
        },
        bg: layout_semantic.background,
        bg_muted: layout_semantic.content_1,
        accent: semantic_roles.primary,
        accent_fg: semantic_roles.primary_fg,
        accent_soft: semantic_scales.primary.shade_100,
        danger: semantic_roles.danger,
        danger_fg: semantic_roles.danger_fg,
        border: if matches!(ctx.color, ThemeColor::Light) {
            FEATURED_LIGHT_BORDER
        } else {
            FEATURED_DARK_BORDER
        },
        focus_ring: layout_semantic.focus,
    };

    let radius_base_px: u16 = 8;

    let layout = LayoutTokens {
        radius: RadiusTokens {
            // Keep a geometric radius ladder:
            // sm = 0.5x, md = 1.0x, lg = 1.5x (relative to base).
            sm_px: radius_base_px / 2,
            md_px: radius_base_px,
            lg_px: radius_base_px + (radius_base_px / 2),
        },
        space: SpaceTokens {
            space_3xs_px: 2,
            space_2xs_px: 4,
            xs_px: 4,
            sm_px: 8,
            md_px: 12,
            lg_px: 16,
        },
        shadow: match (ctx.color, ctx.scale.token_scale()) {
            (ThemeColor::Light, TokenScale::Medium) => ShadowTokens {
                sm: "0 1px 4px oklch(0% 0 0 / 0.12)",
                md: "0 1px 4px oklch(0% 0 0 / 0.16)",
            },
            (ThemeColor::Light, TokenScale::Large) => ShadowTokens {
                sm: "0 2px 6px oklch(0% 0 0 / 0.12)",
                md: "0 2px 6px oklch(0% 0 0 / 0.16)",
            },
            (ThemeColor::Dark | ThemeColor::Oled, TokenScale::Medium) => ShadowTokens {
                sm: "0 1px 4px oklch(0% 0 0 / 0.36)",
                md: "0 1px 4px oklch(0% 0 0 / 0.48)",
            },
            (ThemeColor::Dark | ThemeColor::Oled, TokenScale::Large) => ShadowTokens {
                sm: "0 2px 6px oklch(0% 0 0 / 0.36)",
                md: "0 2px 6px oklch(0% 0 0 / 0.48)",
            },
        },
    };

    let (
        typography,
        component_layout,
        overlay_layout,
        slider_layout,
        color_swatch_layout,
        color_wheel_layout,
    ) = match ctx.scale.token_scale() {
        TokenScale::Medium => (
            TypographyTokens {
                // Baseline: 12px for font-size-100 at medium scale.
                font_size_100_px: 12,
                // Baseline: 14px for font-size-150 at medium scale.
                font_size_150_px: 14,
                // Baseline: 16px for font-size-200 at medium scale.
                font_size_200_px: 16,
                // Baseline text metrics ladder follows Spectrum/HeroUI-style readable line boxes.
                line_height_100_px: 16,
                line_height_150_px: 20,
                line_height_200_px: 24,
                // Body uses the middle rung by default.
                body_font_size_px: 14,
                body_line_height_px: 20,
                // Heading ladder is explicit so h1-h6 never fall back to browser defaults.
                heading_h1_font_size_px: 28,
                heading_h1_line_height_px: 36,
                heading_h2_font_size_px: 24,
                heading_h2_line_height_px: 32,
                heading_h3_font_size_px: 20,
                heading_h3_line_height_px: 28,
                heading_h4_font_size_px: 18,
                heading_h4_line_height_px: 24,
                heading_h5_font_size_px: 16,
                heading_h5_line_height_px: 24,
                heading_h6_font_size_px: 14,
                heading_h6_line_height_px: 20,
            },
            ComponentLayoutTokens {
                // Baseline: 32px for component-height-100 at medium scale.
                component_height_100_px: 32,
                // Baseline: decorative separator opacity is tokenized to avoid component-local literals.
                separator_decorative_opacity_percent: 72,
            },
            OverlayLayoutTokens {
                z_index: 1000,
                panel_min_width_px: 240,
                viewport_inset_px: 16,
                enter_offset_y_px: 6,
                enter_scale: 0.98,
            },
            SLIDER_LAYOUT_TOKENS_MEDIUM,
            COLOR_SWATCH_LAYOUT_TOKENS_MEDIUM,
            COLOR_WHEEL_LAYOUT_TOKENS_MEDIUM,
        ),
        TokenScale::Large => (
            TypographyTokens {
                // Baseline: 14px for font-size-100 at large scale.
                font_size_100_px: 14,
                // Baseline: 16px for font-size-150 at large scale.
                font_size_150_px: 16,
                // Baseline: 19px for font-size-200 at large scale.
                font_size_200_px: 19,
                line_height_100_px: 20,
                line_height_150_px: 24,
                line_height_200_px: 28,
                body_font_size_px: 16,
                body_line_height_px: 24,
                heading_h1_font_size_px: 32,
                heading_h1_line_height_px: 40,
                heading_h2_font_size_px: 28,
                heading_h2_line_height_px: 36,
                heading_h3_font_size_px: 24,
                heading_h3_line_height_px: 32,
                heading_h4_font_size_px: 21,
                heading_h4_line_height_px: 28,
                heading_h5_font_size_px: 19,
                heading_h5_line_height_px: 28,
                heading_h6_font_size_px: 16,
                heading_h6_line_height_px: 24,
            },
            ComponentLayoutTokens {
                // Baseline: 40px for component-height-100 at large scale.
                component_height_100_px: 40,
                // Baseline: decorative separator opacity is tokenized to avoid component-local literals.
                separator_decorative_opacity_percent: 72,
            },
            OverlayLayoutTokens {
                z_index: 1000,
                panel_min_width_px: 280,
                viewport_inset_px: 20,
                enter_offset_y_px: 8,
                enter_scale: 0.98,
            },
            SLIDER_LAYOUT_TOKENS_LARGE,
            COLOR_SWATCH_LAYOUT_TOKENS_LARGE,
            COLOR_WHEEL_LAYOUT_TOKENS_LARGE,
        ),
    };
    let switch_layout = switch_layout_tokens(ctx);

    let color_aliases = ColorAliasTokens {
        text_default: semantic_colors.fg,
        text_muted: semantic_colors.fg_muted,
        surface_default: semantic_colors.bg,
        surface_muted: semantic_colors.bg_muted,
        border_default: semantic_colors.border,
        focus_ring: semantic_colors.focus_ring,
        accent: semantic_colors.accent,
        accent_fg: semantic_colors.accent_fg,
        danger: semantic_colors.danger,
        danger_fg: semantic_colors.danger_fg,
    };

    let component_colors = ComponentColorTokens {
        control_bg: layout_semantic.content_1,
        control_bg_hover: layout_semantic.content_2,
        control_border: semantic_colors.border,
        control_fg: semantic_colors.fg,
        surface_raised: layout_semantic.content_1,
        surface_overlay: if matches!(ctx.color, ThemeColor::Oled) {
            layout_semantic.content_3
        } else {
            layout_semantic.content_2
        },
    };

    let icons = match ctx.scale.token_scale() {
        TokenScale::Medium => IconTokens {
            size_100_px: 20,
            size_200_px: 22,
            stroke_100: 1.0,
        },
        TokenScale::Large => IconTokens {
            size_100_px: 24,
            size_200_px: 28,
            stroke_100: 1.0,
        },
    };

    ThemeTokens {
        common_colors,
        palette,
        semantic_scales,
        semantic_roles,
        semantic_colors,
        layout_semantic,
        color_aliases,
        component_colors,
        icons,
        layout,
        component_layout,
        overlay_layout,
        slider_layout,
        switch_layout,
        color_swatch_layout,
        color_wheel_layout,
        color_wheel_hue: COLOR_WHEEL_HUE_TOKENS,
        underlay_motion: underlay_motion_tokens(ctx),
        typography,
        button_layout: button_layout_tokens(ctx),
        checkbox_layout: checkbox_layout_tokens(ctx),
        command_layout: command_layout_tokens(ctx),
        checkbox_group_layout: checkbox_group_layout_tokens(ctx),
        flip_card_layout: flip_card_layout_tokens(ctx),
    }
}

#[cfg(test)]
#[path = "test/theme.rs"]
mod tests;
