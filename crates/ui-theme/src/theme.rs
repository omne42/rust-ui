use crate::tokens::{ColorTokens, RadiusTokens, ShadowTokens, SpaceTokens};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub scheme: ColorScheme,
    pub colors: ColorTokens,
    pub radius: RadiusTokens,
    pub space: SpaceTokens,
    pub shadow: ShadowTokens,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            scheme: ColorScheme::Light,
            colors: ColorTokens {
                fg: "oklch(21.01% 0.0318 264.66)",
                fg_muted: "oklch(55.10% 0.0234 264.36)",
                bg: "oklch(100% 0 0)",
                bg_muted: "oklch(96.70% 0.0029 264.54)",
                accent: "oklch(54.61% 0.2152 262.88)",
                accent_fg: "oklch(100% 0 0)",
                accent_soft: "oklch(97.05% 0.0142 254.60)",
                border: "oklch(87.17% 0.0093 258.34)",
                focus_ring: "oklch(54.61% 0.2152 262.88)",
            },
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
            shadow: ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.08)",
                md: "0 4px 12px rgba(0,0,0,0.12)",
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            scheme: ColorScheme::Dark,
            colors: ColorTokens {
                fg: "oklch(98.46% 0.0017 247.84)",
                fg_muted: "oklch(71.37% 0.0192 261.32)",
                bg: "oklch(18.31% 0.0309 263.38)",
                bg_muted: "oklch(21.97% 0.0398 264.13)",
                accent: "oklch(71.37% 0.1434 254.62)",
                accent_fg: "oklch(18.31% 0.0309 263.38)",
                accent_soft: "oklch(71.37% 0.1434 254.62 / 0.18)",
                border: "oklch(37.17% 0.0392 257.29)",
                focus_ring: "oklch(80.91% 0.0956 251.81)",
            },
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
            shadow: ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.20)",
                md: "0 10px 30px rgba(0,0,0,0.45)",
            },
        }
    }

    pub fn oled() -> Self {
        Self {
            scheme: ColorScheme::Dark,
            colors: ColorTokens {
                fg: "oklch(98.46% 0.0017 247.84)",
                fg_muted: "oklch(71.37% 0.0192 261.32)",
                bg: "oklch(0% 0 0)",
                bg_muted: "oklch(18.31% 0.0309 263.38)",
                accent: "oklch(71.37% 0.1434 254.62)",
                accent_fg: "oklch(0% 0 0)",
                accent_soft: "oklch(71.37% 0.1434 254.62 / 0.18)",
                border: "oklch(32.58% 0.0404 257.29)",
                focus_ring: "oklch(80.91% 0.0956 251.81)",
            },
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
            shadow: ShadowTokens {
                sm: "0 1px 2px rgba(0,0,0,0.35)",
                md: "0 10px 30px rgba(0,0,0,0.60)",
            },
        }
    }

    pub fn to_css_variables(&self) -> String {
        let scheme = match self.scheme {
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
        };
        format!(
            r#":root {{
  color-scheme: {scheme};
  --ui-fg: {fg};
  --ui-fg-muted: {fg_muted};
  --ui-bg: {bg};
  --ui-bg-muted: {bg_muted};
  --ui-accent: {accent};
  --ui-accent-fg: {accent_fg};
  --ui-accent-soft: {accent_soft};
  --ui-border: {border};
  --ui-focus-ring: {focus_ring};

  --ui-radius-sm: {radius_sm}px;
  --ui-radius-md: {radius_md}px;
  --ui-radius-lg: {radius_lg}px;

  --ui-space-xs: {space_xs}px;
  --ui-space-sm: {space_sm}px;
  --ui-space-md: {space_md}px;
  --ui-space-lg: {space_lg}px;

  --ui-shadow-sm: {shadow_sm};
  --ui-shadow-md: {shadow_md};
}}
"#,
            scheme = scheme,
            fg = self.colors.fg,
            fg_muted = self.colors.fg_muted,
            bg = self.colors.bg,
            bg_muted = self.colors.bg_muted,
            accent = self.colors.accent,
            accent_fg = self.colors.accent_fg,
            accent_soft = self.colors.accent_soft,
            border = self.colors.border,
            focus_ring = self.colors.focus_ring,
            radius_sm = self.radius.sm_px,
            radius_md = self.radius.md_px,
            radius_lg = self.radius.lg_px,
            space_xs = self.space.xs_px,
            space_sm = self.space.sm_px,
            space_md = self.space.md_px,
            space_lg = self.space.lg_px,
            shadow_sm = self.shadow.sm,
            shadow_md = self.shadow.md,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_variables_contains_expected_keys() {
        let theme = Theme::light();
        let css = theme.to_css_variables();
        assert!(css.contains("color-scheme:"));
        assert!(css.contains("--ui-fg:"));
        assert!(css.contains("--ui-fg-muted:"));
        assert!(css.contains("--ui-bg-muted:"));
        assert!(css.contains("--ui-accent-fg:"));
        assert!(css.contains("--ui-radius-md:"));
        assert!(css.contains("--ui-shadow-md:"));
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
