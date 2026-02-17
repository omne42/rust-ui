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
}

#[derive(Clone, Copy)]
pub struct TypographyTokens {
    // Baseline examples (must be regression-testable):
    pub font_size_200_px: u16,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonSizeLayoutTokens {
    pub height_px: u16,
    pub padding_inline_px: u16,
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
        padding_inline_px: 8,
        gap_px: 4,
        icon_size_px: 24,
    },
    s: ButtonSizeLayoutTokens {
        height_px: 28,
        padding_inline_px: 10,
        gap_px: 4,
        icon_size_px: 28,
    },
    m: ButtonSizeLayoutTokens {
        height_px: 32,
        padding_inline_px: 12,
        gap_px: 6,
        icon_size_px: 32,
    },
    l: ButtonSizeLayoutTokens {
        height_px: 36,
        padding_inline_px: 16,
        gap_px: 8,
        icon_size_px: 36,
    },
    xl: ButtonSizeLayoutTokens {
        height_px: 40,
        padding_inline_px: 20,
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
        padding_inline_px: 8,
        gap_px: 4,
        icon_size_px: 24,
    },
    s: ButtonSizeLayoutTokens {
        height_px: 28,
        padding_inline_px: 10,
        gap_px: 4,
        icon_size_px: 28,
    },
    m: ButtonSizeLayoutTokens {
        height_px: 32,
        padding_inline_px: 12,
        gap_px: 6,
        icon_size_px: 32,
    },
    l: ButtonSizeLayoutTokens {
        height_px: 36,
        padding_inline_px: 16,
        gap_px: 8,
        icon_size_px: 36,
    },
    xl: ButtonSizeLayoutTokens {
        height_px: 40,
        padding_inline_px: 20,
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
    pub typography: TypographyTokens,
    pub button_layout: ButtonLayoutTokens,
}
