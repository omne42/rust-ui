use crate::tokens::{ColorTokens, RadiusTokens, ShadowTokens, SpaceTokens};

#[derive(Clone, Copy)]
pub struct Theme {
    pub colors: ColorTokens,
    pub radius: RadiusTokens,
    pub space: SpaceTokens,
    pub shadow: ShadowTokens,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            colors: ColorTokens {
                fg: "#111827",
                bg: "#ffffff",
                accent: "#2563eb",
                border: "#d1d5db",
                focus_ring: "#2563eb",
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

    pub fn to_css_variables(&self) -> String {
        format!(
            r#":root {{
  --ui-fg: {fg};
  --ui-bg: {bg};
  --ui-accent: {accent};
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
            fg = self.colors.fg,
            bg = self.colors.bg,
            accent = self.colors.accent,
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
        assert!(css.contains("--ui-fg:"));
        assert!(css.contains("--ui-radius-md:"));
        assert!(css.contains("--ui-shadow-md:"));
    }
}
