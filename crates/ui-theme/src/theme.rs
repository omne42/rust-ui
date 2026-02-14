use crate::tokens::{
    ColorAliasTokens, ColorPaletteTokens, ComponentColorTokens, ComponentLayoutTokens, IconTokens,
    LayoutTokens, RadiusTokens, SemanticColorTokens, ShadowTokens, SpaceTokens, ThemeTokens,
    TokenScale, TypographyTokens,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeSystem {
    Spectrum,
    Express,
    SpectrumTwo,
}

impl ThemeSystem {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Spectrum => "spectrum",
            Self::Express => "express",
            Self::SpectrumTwo => "spectrum-two",
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
            // Default to Spectrum Two + Medium so apps/components have a stable baseline.
            system: ThemeSystem::SpectrumTwo,
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

    pub fn spectrum(color: ThemeColor, scale: ThemeScale) -> Self {
        Self::new(ThemeContext {
            system: ThemeSystem::Spectrum,
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

    pub fn spectrum_two(color: ThemeColor, scale: ThemeScale) -> Self {
        Self::new(ThemeContext {
            system: ThemeSystem::SpectrumTwo,
            color,
            scale,
        })
    }

    pub fn to_css_variables(&self) -> String {
        crate::css::theme_to_css_variables(self)
    }
}

fn resolve_tokens(ctx: ThemeContext) -> ThemeTokens {
    let semantic_colors = match ctx.color {
        ThemeColor::Light => SemanticColorTokens {
            fg: "oklch(21.01% 0.0318 264.66)",
            fg_muted: "oklch(55.10% 0.0234 264.36)",
            bg: "oklch(100% 0 0)",
            bg_muted: "oklch(96.70% 0.0029 264.54)",
            accent: "oklch(54.61% 0.2152 262.88)",
            accent_fg: "oklch(100% 0 0)",
            accent_soft: "oklch(97.05% 0.0142 254.60)",
            danger: "oklch(59% 0.2419 11.33)",
            danger_fg: "oklch(100% 0 0)",
            border: "oklch(87.17% 0.0093 258.34)",
            focus_ring: "oklch(54.61% 0.2152 262.88)",
        },
        ThemeColor::Dark => SemanticColorTokens {
            fg: "oklch(98.46% 0.0017 247.84)",
            fg_muted: "oklch(71.37% 0.0192 261.32)",
            bg: "oklch(18.31% 0.0309 263.38)",
            bg_muted: "oklch(21.97% 0.0398 264.13)",
            accent: "oklch(71.37% 0.1434 254.62)",
            accent_fg: "oklch(18.31% 0.0309 263.38)",
            accent_soft: "oklch(71.37% 0.1434 254.62 / 0.18)",
            danger: "oklch(59% 0.2419 11.33)",
            danger_fg: "oklch(100% 0 0)",
            border: "oklch(37.17% 0.0392 257.29)",
            focus_ring: "oklch(80.91% 0.0956 251.81)",
        },
        ThemeColor::Oled => SemanticColorTokens {
            fg: "oklch(98.46% 0.0017 247.84)",
            fg_muted: "oklch(71.37% 0.0192 261.32)",
            bg: "oklch(0% 0 0)",
            bg_muted: "oklch(18.31% 0.0309 263.38)",
            accent: "oklch(71.37% 0.1434 254.62)",
            accent_fg: "oklch(0% 0 0)",
            accent_soft: "oklch(71.37% 0.1434 254.62 / 0.18)",
            danger: "oklch(59% 0.2419 11.33)",
            danger_fg: "oklch(100% 0 0)",
            border: "oklch(32.58% 0.0404 257.29)",
            focus_ring: "oklch(80.91% 0.0956 251.81)",
        },
    };

    // v0 mapping: system affects token choices, but we currently keep a single set of semantic values.
    // The axis is still first-class in the ThemeContext so future deltas are centralized here.
    let _system = ctx.system;

    let layout = LayoutTokens {
        radius: RadiusTokens {
            sm_px: 4,
            md_px: 6,
            lg_px: 10,
        },
        space: SpaceTokens {
            xs_px: 4,
            sm_px: 8,
            md_px: 12,
            lg_px: 16,
        },
        shadow: match ctx.color {
            ThemeColor::Light => ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.08)",
                md: "0 4px 12px rgba(0,0,0,0.12)",
            },
            ThemeColor::Dark => ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.20)",
                md: "0 10px 30px rgba(0,0,0,0.45)",
            },
            ThemeColor::Oled => ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.35)",
                md: "0 10px 30px rgba(0,0,0,0.60)",
            },
        },
    };

    let (typography, component_layout) = match ctx.scale.token_scale() {
        TokenScale::Medium => (
            TypographyTokens {
                // Spectrum baseline: 16px for font-size-200 at medium scale.
                font_size_200_px: 16,
            },
            ComponentLayoutTokens {
                // Spectrum baseline: 32px for component-height-100 at medium scale.
                component_height_100_px: 32,
            },
        ),
        TokenScale::Large => (
            TypographyTokens {
                // Spectrum baseline: 19px for font-size-200 at large scale.
                font_size_200_px: 19,
            },
            ComponentLayoutTokens {
                // Spectrum baseline: 40px for component-height-100 at large scale.
                component_height_100_px: 40,
            },
        ),
    };

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

    ThemeTokens {
        palette: ColorPaletteTokens {},
        semantic_colors,
        color_aliases,
        component_colors: ComponentColorTokens {},
        icons: IconTokens {},
        layout,
        component_layout,
        typography,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_css_variables_contains_expected_keys() {
        let theme = Theme::light();
        let css = theme.to_css_variables();
        assert!(css.contains("color-scheme:"));
        assert!(css.contains("--ui-fg:"));
        assert!(css.contains("--ui-fg-muted:"));
        assert!(css.contains("--ui-bg-muted:"));
        assert!(css.contains("--ui-accent-fg:"));
        assert!(css.contains("--ui-danger:"));
        assert!(css.contains("--ui-radius-md:"));
        assert!(css.contains("--ui-shadow-md:"));
        assert!(css.contains("--ui-font-size-200:"));
        assert!(css.contains("--ui-component-height-100:"));
    }

    #[test]
    fn light_theme_sets_light_color_scheme() {
        let css = Theme::light().to_css_variables();
        assert!(css.contains("color-scheme: light;"));
    }

    #[test]
    fn dark_and_oled_themes_set_dark_color_scheme() {
        let dark_css = Theme::dark().to_css_variables();
        assert!(dark_css.contains("color-scheme: dark;"));
        let oled_css = Theme::oled().to_css_variables();
        assert!(oled_css.contains("color-scheme: dark;"));
    }
}
