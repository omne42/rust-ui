//! Design tokens for `ui-theme`.
//!
//! This file is the single source of truth for token taxonomy and baselines.
//! Theme mapping happens in `theme.rs`; CSS variable emission happens in `css.rs`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenScale {
    Medium,
    Large,
}

#[derive(Clone, Copy)]
pub struct ColorPaletteTokens {
    pub gray_50: &'static str,
    pub gray_200: &'static str,
    pub gray_700: &'static str,
    pub gray_900: &'static str,
    pub accent_500: &'static str,
    pub accent_600: &'static str,
    pub accent_700: &'static str,
}

#[derive(Clone, Copy)]
pub struct ColorScaleTokens {
    pub shade_50: &'static str,
    pub shade_100: &'static str,
    pub shade_200: &'static str,
    pub shade_300: &'static str,
    pub shade_400: &'static str,
    pub shade_500: &'static str,
    pub shade_600: &'static str,
    pub shade_700: &'static str,
    pub shade_800: &'static str,
    pub shade_900: &'static str,
}

#[derive(Clone, Copy)]
pub struct SemanticScaleTokens {
    pub default: ColorScaleTokens,
    pub primary: ColorScaleTokens,
    pub secondary: ColorScaleTokens,
    pub success: ColorScaleTokens,
    pub warning: ColorScaleTokens,
    pub danger: ColorScaleTokens,
}

#[derive(Clone, Copy)]
pub struct CommonColorScales {
    pub white: &'static str,
    pub black: &'static str,
    pub blue: ColorScaleTokens,
    pub purple: ColorScaleTokens,
    pub green: ColorScaleTokens,
    pub red: ColorScaleTokens,
    pub pink: ColorScaleTokens,
    pub yellow: ColorScaleTokens,
    pub cyan: ColorScaleTokens,
    pub zinc: ColorScaleTokens,
}

#[derive(Clone, Copy)]
pub struct SemanticColorTokens {
    pub fg: &'static str,
    pub fg_muted: &'static str,
    pub bg: &'static str,
    pub bg_muted: &'static str,
    pub accent: &'static str,
    pub accent_fg: &'static str,
    pub accent_soft: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
    pub border: &'static str,
    pub focus_ring: &'static str,
}

#[derive(Clone, Copy)]
pub struct SemanticRoleTokens {
    pub default: &'static str,
    pub default_fg: &'static str,
    pub primary: &'static str,
    pub primary_fg: &'static str,
    pub secondary: &'static str,
    pub secondary_fg: &'static str,
    pub success: &'static str,
    pub success_fg: &'static str,
    pub warning: &'static str,
    pub warning_fg: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
}

#[derive(Clone, Copy)]
pub struct ColorAliasTokens {
    pub text_default: &'static str,
    pub text_muted: &'static str,
    pub surface_default: &'static str,
    pub surface_muted: &'static str,
    pub border_default: &'static str,
    pub focus_ring: &'static str,
    pub accent: &'static str,
    pub accent_fg: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
}

#[derive(Clone, Copy)]
pub struct LayoutSemanticTokens {
    pub background: &'static str,
    pub foreground: &'static str,
    pub divider: &'static str,
    pub focus: &'static str,
    pub content_1: &'static str,
    pub content_2: &'static str,
    pub content_3: &'static str,
    pub content_4: &'static str,
}

#[derive(Clone, Copy)]
pub struct ComponentColorTokens {
    pub control_bg: &'static str,
    pub control_bg_hover: &'static str,
    pub control_border: &'static str,
    pub control_fg: &'static str,
    pub surface_raised: &'static str,
    pub surface_overlay: &'static str,
}

#[derive(Clone, Copy)]
pub struct IconTokens {
    pub size_100_px: u16,
    pub size_200_px: u16,
    pub stroke_100: f32,
}

#[derive(Clone, Copy)]
pub struct LayoutTokens {
    pub radius: RadiusTokens,
    pub space: SpaceTokens,
    pub shadow: ShadowTokens,
}

#[derive(Clone, Copy)]
pub struct RadiusTokens {
    pub sm_px: u16,
    pub md_px: u16,
    pub lg_px: u16,
}

#[derive(Clone, Copy)]
pub struct SpaceTokens {
    pub space_3xs_px: u16,
    pub space_2xs_px: u16,
    pub xs_px: u16,
    pub sm_px: u16,
    pub md_px: u16,
    pub lg_px: u16,
}

#[derive(Clone, Copy)]
pub struct ShadowTokens {
    pub sm: &'static str,
    pub md: &'static str,
}

#[derive(Clone, Copy)]
pub struct ComponentLayoutTokens {
    // Baseline examples (must be regression-testable):
    pub component_height_100_px: u16,
    pub separator_decorative_opacity_percent: u8,
}

#[derive(Clone, Copy)]
pub struct TypographyTokens {
    // Baseline examples (must be regression-testable):
    pub font_size_100_px: u16,
    pub font_size_150_px: u16,
    pub font_size_200_px: u16,
    pub line_height_100_px: u16,
    pub line_height_150_px: u16,
    pub line_height_200_px: u16,
    pub body_font_size_px: u16,
    pub body_line_height_px: u16,
    pub heading_h1_font_size_px: u16,
    pub heading_h1_line_height_px: u16,
    pub heading_h2_font_size_px: u16,
    pub heading_h2_line_height_px: u16,
    pub heading_h3_font_size_px: u16,
    pub heading_h3_line_height_px: u16,
    pub heading_h4_font_size_px: u16,
    pub heading_h4_line_height_px: u16,
    pub heading_h5_font_size_px: u16,
    pub heading_h5_line_height_px: u16,
    pub heading_h6_font_size_px: u16,
    pub heading_h6_line_height_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayLayoutTokens {
    pub z_index: u16,
    pub panel_min_width_px: u16,
    pub viewport_inset_px: u16,
    pub enter_offset_y_px: u16,
    pub enter_scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringMotionTokens {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub precision: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccordionMotionTokens {
    pub spring: SpringMotionTokens,
    pub indicator_closed_rotation_deg: f64,
    pub indicator_open_rotation_deg: f64,
    pub panel_offset_y_px: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotionTokens {
    pub spring: SpringMotionTokens,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwatchMotionTokens {
    pub spring: SpringMotionTokens,
    pub selected_scale: f64,
    pub selected_ring_opacity: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchMotionTokens {
    pub spring: SpringMotionTokens,
    pub pressed_width_default_px: f64,
    pub pressed_width_min_px: f64,
    pub pressed_width_max_px: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderMotionTokens {
    pub spring: SpringMotionTokens,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropZoneMotionTokens {
    pub spring: SpringMotionTokens,
    pub hover_scale: f64,
    pub drop_scale: f64,
    pub hover_highlight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropZoneLayoutTokens {
    pub min_height_px: u16,
    pub border_width_px: u16,
    pub disabled_opacity_percent: u8,
    pub focus_outline_width_px: u16,
    pub focus_outline_offset_px: u16,
    pub sr_only_size_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlipCardLayoutTokens {
    pub max_inline_size_px: u16,
    pub max_inline_viewport_percent: u8,
    pub aspect_ratio_width: u8,
    pub aspect_ratio_height: u8,
    pub perspective_px: u16,
    pub disabled_opacity_percent: u8,
    pub focus_outline_width_px: u16,
    pub title_font_weight: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliderLayoutTokens {
    pub max_width_px: u16,
    pub thumb_border_width_px: u16,
    pub focus_ring_width_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSwatchLayoutTokens {
    pub size_xs_px: u16,
    pub size_sm_px: u16,
    pub size_md_px: u16,
    pub size_lg_px: u16,
    pub radius_default_px: u16,
    pub radius_none_px: u16,
    pub radius_full_px: u16,
    pub shape_wide_multiplier: f64,
    pub checker_size_px: u16,
    pub slash_width_px: u16,
    pub border_width_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorWheelLayoutTokens {
    pub size_px: u16,
    pub track_thickness_px: u16,
    pub thumb_size_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorWheelHueTokens {
    pub red: &'static str,
    pub yellow: &'static str,
    pub green: &'static str,
    pub cyan: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayMotionTokens {
    pub transition_duration_ms: u16,
    pub visibility_duration_ms: u16,
    pub backdrop_blur_px: u16,
    pub scrim_alpha_percent: u8,
    pub transition_easing: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeFieldMotionTokens {
    pub spring: SpringMotionTokens,
    pub hidden_scale: f64,
    pub hover_scale: f64,
    pub tap_scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaMotionTokens {
    pub duration_ms: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldMotionTokens {
    pub duration_ms: u16,
    pub easing: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelMotionTokens {
    pub color_duration_ms: u16,
    pub weight_duration_ms: u16,
    pub easing: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxGroupMotionTokens {
    pub duration_ms: u16,
    pub easing: &'static str,
    pub spring: SpringMotionTokens,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupLayoutTokens {
    pub gap_px: u16,
    pub required_marker_gap_px: u16,
    pub disabled_opacity_percent: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonSizeLayoutTokens {
    pub height_px: u16,
    pub min_width_px: u16,
    pub padding_inline_px: u16,
    pub font_size_px: u16,
    pub line_height_px: u16,
    pub gap_px: u16,
    pub icon_size_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonLayoutTokens {
    pub min_width_px: u16,
    pub font_size_px: u16,
    pub spinner_size_px: u16,
    pub spinner_border_px: u16,
    pub spinner_duration_ms: u16,
    pub focus_outline_width_px: u16,
    pub focus_outline_offset_px: u16,
    pub radius_full_px: u16,
    pub xs: ButtonSizeLayoutTokens,
    pub s: ButtonSizeLayoutTokens,
    pub m: ButtonSizeLayoutTokens,
    pub l: ButtonSizeLayoutTokens,
    pub xl: ButtonSizeLayoutTokens,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxLayoutTokens {
    pub gap_px: u16,
    pub disabled_opacity_percent: u8,
    pub focus_outline_width_px: u16,
    pub focus_outline_offset_px: u16,
    pub box_size_default_px: u16,
    pub box_size_sm_px: u16,
    pub box_size_lg_px: u16,
    pub box_radius_default_px: u16,
    pub box_radius_sm_px: u16,
    pub box_radius_lg_px: u16,
    pub indicator_size_default_px: u16,
    pub indicator_size_sm_px: u16,
    pub indicator_size_lg_px: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandLayoutTokens {
    pub panel_max_width_px: u16,
    pub list_max_height_px: u16,
    pub disabled_opacity_percent: u8,
    pub input_wrap_padding_px: u16,
    pub input_wrap_border_mix_percent: u8,
    pub input_wrap_bg_mix_percent: u8,
    pub input_padding_block_px: u16,
    pub input_padding_inline_px: u16,
    pub input_focus_outline_width_px: u16,
    pub input_focus_outline_offset_px: u16,
    pub options_padding_px: u16,
    pub group_gap_px: u16,
    pub group_spacing_px: u16,
    pub group_border_mix_percent: u8,
    pub group_heading_padding_inline_px: u16,
    pub group_heading_letter_spacing_centiem: u16,
    pub group_items_gap_px: u16,
    pub option_gap_px: u16,
    pub option_padding_block_px: u16,
    pub option_padding_inline_px: u16,
    pub option_focus_mix_percent: u8,
    pub option_disabled_opacity_percent: u8,
    pub shortcut_border_mix_percent: u8,
    pub shortcut_bg_mix_percent: u8,
    pub shortcut_padding_inline_px: u16,
    pub shortcut_padding_block_px: u16,
    pub empty_padding_block_px: u16,
    pub empty_padding_inline_px: u16,
}

#[path = "tokens/component_layout_consts.rs"]
mod component_layout_consts;
pub use component_layout_consts::{
    CHECKBOX_LAYOUT_TOKENS_LARGE, CHECKBOX_LAYOUT_TOKENS_MEDIUM, COMMAND_LAYOUT_TOKENS_LARGE,
    COMMAND_LAYOUT_TOKENS_MEDIUM,
};

pub const ACCORDION_MOTION_TOKENS_MEDIUM: AccordionMotionTokens = AccordionMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 18.0,
        mass: 1.0,
        precision: 0.001,
    },
    indicator_closed_rotation_deg: 0.0,
    indicator_open_rotation_deg: 90.0,
    panel_offset_y_px: 4.0,
};

pub const ACCORDION_MOTION_TOKENS_LARGE: AccordionMotionTokens = AccordionMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 18.0,
        mass: 1.0,
        precision: 0.001,
    },
    indicator_closed_rotation_deg: 0.0,
    indicator_open_rotation_deg: 90.0,
    panel_offset_y_px: 6.0,
};

pub const BUTTON_MOTION_TOKENS_MEDIUM: ButtonMotionTokens = ButtonMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    hover_scale: 1.05,
    tap_scale: 0.95,
};

pub const BUTTON_MOTION_TOKENS_LARGE: ButtonMotionTokens = ButtonMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    hover_scale: 1.05,
    tap_scale: 0.95,
};

pub const SWATCH_MOTION_TOKENS_MEDIUM: SwatchMotionTokens = SwatchMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 280.0,
        damping: 20.0,
        mass: 1.0,
        precision: 0.001,
    },
    selected_scale: 1.06,
    selected_ring_opacity: 1.0,
};

pub const SWATCH_MOTION_TOKENS_LARGE: SwatchMotionTokens = SwatchMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 280.0,
        damping: 20.0,
        mass: 1.0,
        precision: 0.001,
    },
    selected_scale: 1.06,
    selected_ring_opacity: 1.0,
};

pub const SWITCH_MOTION_TOKENS_MEDIUM: SwitchMotionTokens = SwitchMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    pressed_width_default_px: 19.0,
    pressed_width_min_px: 16.0,
    pressed_width_max_px: 64.0,
};

pub const SWITCH_MOTION_TOKENS_LARGE: SwitchMotionTokens = SwitchMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    pressed_width_default_px: 19.0,
    pressed_width_min_px: 16.0,
    pressed_width_max_px: 64.0,
};

pub const SLIDER_MOTION_TOKENS_MEDIUM: SliderMotionTokens = SliderMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 340.0,
        damping: 28.0,
        mass: 0.9,
        precision: 0.001,
    },
};

pub const SLIDER_MOTION_TOKENS_LARGE: SliderMotionTokens = SliderMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 340.0,
        damping: 28.0,
        mass: 0.9,
        precision: 0.001,
    },
};

pub const DROP_ZONE_MOTION_TOKENS_MEDIUM: DropZoneMotionTokens = DropZoneMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 18.0,
        mass: 1.0,
        precision: 0.001,
    },
    hover_scale: 1.01,
    drop_scale: 1.02,
    hover_highlight: 0.35,
};

pub const DROP_ZONE_MOTION_TOKENS_LARGE: DropZoneMotionTokens = DropZoneMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 18.0,
        mass: 1.0,
        precision: 0.001,
    },
    hover_scale: 1.01,
    drop_scale: 1.02,
    hover_highlight: 0.35,
};

pub const DROP_ZONE_LAYOUT_TOKENS_MEDIUM: DropZoneLayoutTokens = DropZoneLayoutTokens {
    min_height_px: 120,
    border_width_px: 1,
    disabled_opacity_percent: 50,
    focus_outline_width_px: 3,
    focus_outline_offset_px: 2,
    sr_only_size_px: 1,
};

pub const DROP_ZONE_LAYOUT_TOKENS_LARGE: DropZoneLayoutTokens = DropZoneLayoutTokens {
    min_height_px: 136,
    border_width_px: 1,
    disabled_opacity_percent: 50,
    focus_outline_width_px: 3,
    focus_outline_offset_px: 2,
    sr_only_size_px: 1,
};

pub const FLIP_CARD_LAYOUT_TOKENS_MEDIUM: FlipCardLayoutTokens = FlipCardLayoutTokens {
    max_inline_size_px: 336,
    max_inline_viewport_percent: 92,
    aspect_ratio_width: 4,
    aspect_ratio_height: 3,
    perspective_px: 1200,
    disabled_opacity_percent: 60,
    focus_outline_width_px: 3,
    title_font_weight: 650,
};

pub const FLIP_CARD_LAYOUT_TOKENS_LARGE: FlipCardLayoutTokens = FlipCardLayoutTokens {
    max_inline_size_px: 384,
    max_inline_viewport_percent: 92,
    aspect_ratio_width: 4,
    aspect_ratio_height: 3,
    perspective_px: 1320,
    disabled_opacity_percent: 60,
    focus_outline_width_px: 3,
    title_font_weight: 650,
};

pub const SLIDER_LAYOUT_TOKENS_MEDIUM: SliderLayoutTokens = SliderLayoutTokens {
    max_width_px: 352,
    thumb_border_width_px: 2,
    focus_ring_width_px: 2,
};

pub const SLIDER_LAYOUT_TOKENS_LARGE: SliderLayoutTokens = SliderLayoutTokens {
    max_width_px: 400,
    thumb_border_width_px: 2,
    focus_ring_width_px: 2,
};

pub const COLOR_SWATCH_LAYOUT_TOKENS_MEDIUM: ColorSwatchLayoutTokens = ColorSwatchLayoutTokens {
    size_xs_px: 14,
    size_sm_px: 16,
    size_md_px: 20,
    size_lg_px: 24,
    radius_default_px: 4,
    radius_none_px: 0,
    radius_full_px: 9999,
    shape_wide_multiplier: 2.5,
    checker_size_px: 8,
    slash_width_px: 1,
    border_width_px: 1,
};

pub const COLOR_SWATCH_LAYOUT_TOKENS_LARGE: ColorSwatchLayoutTokens = ColorSwatchLayoutTokens {
    size_xs_px: 16,
    size_sm_px: 18,
    size_md_px: 22,
    size_lg_px: 28,
    radius_default_px: 4,
    radius_none_px: 0,
    radius_full_px: 9999,
    shape_wide_multiplier: 2.5,
    checker_size_px: 10,
    slash_width_px: 1,
    border_width_px: 1,
};

pub const COLOR_WHEEL_LAYOUT_TOKENS_MEDIUM: ColorWheelLayoutTokens = ColorWheelLayoutTokens {
    size_px: 176,
    track_thickness_px: 16,
    thumb_size_px: 16,
};

pub const COLOR_WHEEL_LAYOUT_TOKENS_LARGE: ColorWheelLayoutTokens = ColorWheelLayoutTokens {
    size_px: 192,
    track_thickness_px: 18,
    thumb_size_px: 18,
};

pub const COLOR_WHEEL_HUE_TOKENS: ColorWheelHueTokens = ColorWheelHueTokens {
    red: "#ff0000",
    yellow: "#ffff00",
    green: "#00ff00",
    cyan: "#00ffff",
    blue: "#0000ff",
    magenta: "#ff00ff",
};

pub const UNDERLAY_MOTION_TOKENS_MEDIUM: UnderlayMotionTokens = UnderlayMotionTokens {
    transition_duration_ms: 220,
    visibility_duration_ms: 220,
    backdrop_blur_px: 1,
    scrim_alpha_percent: 56,
    transition_easing: "cubic-bezier(0.22, 1, 0.36, 1)",
};

pub const UNDERLAY_MOTION_TOKENS_LARGE: UnderlayMotionTokens = UnderlayMotionTokens {
    transition_duration_ms: 240,
    visibility_duration_ms: 240,
    backdrop_blur_px: 1,
    scrim_alpha_percent: 56,
    transition_easing: "cubic-bezier(0.22, 1, 0.36, 1)",
};

pub const TIME_FIELD_MOTION_TOKENS_MEDIUM: TimeFieldMotionTokens = TimeFieldMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    hidden_scale: 0.85,
    hover_scale: 1.04,
    tap_scale: 0.96,
};

pub const TIME_FIELD_MOTION_TOKENS_LARGE: TimeFieldMotionTokens = TimeFieldMotionTokens {
    spring: SpringMotionTokens {
        stiffness: 260.0,
        damping: 16.0,
        mass: 1.0,
        precision: 0.001,
    },
    hidden_scale: 0.85,
    hover_scale: 1.04,
    tap_scale: 0.96,
};

pub const TEXTAREA_MOTION_TOKENS_MEDIUM: TextareaMotionTokens =
    TextareaMotionTokens { duration_ms: 180 };

pub const TEXTAREA_MOTION_TOKENS_LARGE: TextareaMotionTokens =
    TextareaMotionTokens { duration_ms: 200 };

pub const TEXT_FIELD_MOTION_TOKENS_MEDIUM: TextFieldMotionTokens = TextFieldMotionTokens {
    duration_ms: 180,
    easing: "cubic-bezier(0.2, 0, 0, 1)",
};

pub const TEXT_FIELD_MOTION_TOKENS_LARGE: TextFieldMotionTokens = TextFieldMotionTokens {
    duration_ms: 200,
    easing: "cubic-bezier(0.2, 0, 0, 1)",
};

pub const LABEL_MOTION_TOKENS_MEDIUM: LabelMotionTokens = LabelMotionTokens {
    color_duration_ms: 180,
    weight_duration_ms: 180,
    easing: "cubic-bezier(0.2, 0, 0, 1)",
};

pub const LABEL_MOTION_TOKENS_LARGE: LabelMotionTokens = LabelMotionTokens {
    color_duration_ms: 200,
    weight_duration_ms: 200,
    easing: "cubic-bezier(0.2, 0, 0, 1)",
};

pub const CHECKBOX_GROUP_MOTION_TOKENS_MEDIUM: CheckboxGroupMotionTokens =
    CheckboxGroupMotionTokens {
        duration_ms: 180,
        easing: "cubic-bezier(0.2, 0, 0, 1)",
        spring: SpringMotionTokens {
            stiffness: 260.0,
            damping: 16.0,
            mass: 1.0,
            precision: 0.001,
        },
    };

pub const CHECKBOX_GROUP_MOTION_TOKENS_LARGE: CheckboxGroupMotionTokens =
    CheckboxGroupMotionTokens {
        duration_ms: 200,
        easing: "cubic-bezier(0.2, 0, 0, 1)",
        spring: SpringMotionTokens {
            stiffness: 260.0,
            damping: 16.0,
            mass: 1.0,
            precision: 0.001,
        },
    };

pub const CHECKBOX_GROUP_LAYOUT_TOKENS_MEDIUM: CheckboxGroupLayoutTokens =
    CheckboxGroupLayoutTokens {
        gap_px: 4,
        required_marker_gap_px: 4,
        disabled_opacity_percent: 60,
    };

pub const CHECKBOX_GROUP_LAYOUT_TOKENS_LARGE: CheckboxGroupLayoutTokens =
    CheckboxGroupLayoutTokens {
        gap_px: 4,
        required_marker_gap_px: 4,
        disabled_opacity_percent: 60,
    };

pub const BUTTON_LAYOUT_TOKENS_MEDIUM: ButtonLayoutTokens = ButtonLayoutTokens {
    min_width_px: 80,
    font_size_px: 14,
    spinner_size_px: 16,
    spinner_border_px: 2,
    spinner_duration_ms: 800,
    focus_outline_width_px: 3,
    focus_outline_offset_px: 2,
    radius_full_px: 9999,
    xs: ButtonSizeLayoutTokens {
        height_px: 24,
        min_width_px: 56,
        padding_inline_px: 8,
        font_size_px: 12,
        line_height_px: 16,
        gap_px: 4,
        icon_size_px: 24,
    },
    s: ButtonSizeLayoutTokens {
        height_px: 28,
        min_width_px: 68,
        padding_inline_px: 10,
        font_size_px: 13,
        line_height_px: 18,
        gap_px: 4,
        icon_size_px: 28,
    },
    m: ButtonSizeLayoutTokens {
        height_px: 32,
        min_width_px: 80,
        padding_inline_px: 12,
        font_size_px: 14,
        line_height_px: 20,
        gap_px: 6,
        icon_size_px: 32,
    },
    l: ButtonSizeLayoutTokens {
        height_px: 36,
        min_width_px: 92,
        padding_inline_px: 16,
        font_size_px: 15,
        line_height_px: 22,
        gap_px: 8,
        icon_size_px: 36,
    },
    xl: ButtonSizeLayoutTokens {
        height_px: 40,
        min_width_px: 104,
        padding_inline_px: 20,
        font_size_px: 16,
        line_height_px: 24,
        gap_px: 8,
        icon_size_px: 40,
    },
};

pub const BUTTON_LAYOUT_TOKENS_LARGE: ButtonLayoutTokens = ButtonLayoutTokens {
    min_width_px: 80,
    font_size_px: 14,
    spinner_size_px: 16,
    spinner_border_px: 2,
    spinner_duration_ms: 800,
    focus_outline_width_px: 3,
    focus_outline_offset_px: 2,
    radius_full_px: 9999,
    xs: ButtonSizeLayoutTokens {
        height_px: 24,
        min_width_px: 56,
        padding_inline_px: 8,
        font_size_px: 12,
        line_height_px: 16,
        gap_px: 4,
        icon_size_px: 24,
    },
    s: ButtonSizeLayoutTokens {
        height_px: 28,
        min_width_px: 68,
        padding_inline_px: 10,
        font_size_px: 13,
        line_height_px: 18,
        gap_px: 4,
        icon_size_px: 28,
    },
    m: ButtonSizeLayoutTokens {
        height_px: 32,
        min_width_px: 80,
        padding_inline_px: 12,
        font_size_px: 14,
        line_height_px: 20,
        gap_px: 6,
        icon_size_px: 32,
    },
    l: ButtonSizeLayoutTokens {
        height_px: 36,
        min_width_px: 92,
        padding_inline_px: 16,
        font_size_px: 15,
        line_height_px: 22,
        gap_px: 8,
        icon_size_px: 36,
    },
    xl: ButtonSizeLayoutTokens {
        height_px: 40,
        min_width_px: 104,
        padding_inline_px: 20,
        font_size_px: 16,
        line_height_px: 24,
        gap_px: 8,
        icon_size_px: 40,
    },
};

#[derive(Clone, Copy)]
pub struct ThemeTokens {
    pub common_colors: CommonColorScales,
    pub palette: ColorPaletteTokens,
    pub semantic_scales: SemanticScaleTokens,
    pub semantic_roles: SemanticRoleTokens,
    pub semantic_colors: SemanticColorTokens,
    pub layout_semantic: LayoutSemanticTokens,
    pub color_aliases: ColorAliasTokens,
    pub component_colors: ComponentColorTokens,
    pub icons: IconTokens,
    pub layout: LayoutTokens,
    pub component_layout: ComponentLayoutTokens,
    pub overlay_layout: OverlayLayoutTokens,
    pub slider_layout: SliderLayoutTokens,
    pub color_swatch_layout: ColorSwatchLayoutTokens,
    pub color_wheel_layout: ColorWheelLayoutTokens,
    pub color_wheel_hue: ColorWheelHueTokens,
    pub underlay_motion: UnderlayMotionTokens,
    pub typography: TypographyTokens,
    pub button_layout: ButtonLayoutTokens,
    pub checkbox_layout: CheckboxLayoutTokens,
    pub command_layout: CommandLayoutTokens,
    pub checkbox_group_layout: CheckboxGroupLayoutTokens,
    pub flip_card_layout: FlipCardLayoutTokens,
}
