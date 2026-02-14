pub const BASE_CSS: &str = r#"
:root {
  /* Generated tokens should be appended after this block. */
}
"#;

use crate::theme::Theme;

pub fn theme_to_css_variables(theme: &Theme) -> String {
    let scheme = theme.ctx.color.css_color_scheme();
    let colors = &theme.tokens.semantic_colors;
    let layout = &theme.tokens.layout;
    let typography = &theme.tokens.typography;
    let component_layout = &theme.tokens.component_layout;

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
  --ui-danger: {danger};
  --ui-danger-fg: {danger_fg};
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

  --ui-font-size-200: {font_size_200}px;
  --ui-component-height-100: {component_height_100}px;
}}
"#,
        scheme = scheme,
        fg = colors.fg,
        fg_muted = colors.fg_muted,
        bg = colors.bg,
        bg_muted = colors.bg_muted,
        accent = colors.accent,
        accent_fg = colors.accent_fg,
        accent_soft = colors.accent_soft,
        danger = colors.danger,
        danger_fg = colors.danger_fg,
        border = colors.border,
        focus_ring = colors.focus_ring,
        radius_sm = layout.radius.sm_px,
        radius_md = layout.radius.md_px,
        radius_lg = layout.radius.lg_px,
        space_xs = layout.space.xs_px,
        space_sm = layout.space.sm_px,
        space_md = layout.space.md_px,
        space_lg = layout.space.lg_px,
        shadow_sm = layout.shadow.sm,
        shadow_md = layout.shadow.md,
        font_size_200 = typography.font_size_200_px,
        component_height_100 = component_layout.component_height_100_px,
    )
}

pub const SAFE_AREA_CSS: &str = r#"
.safe-area {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
"#;
