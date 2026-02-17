pub const BASE_CSS: &str = r#"
:root {
  /* Generated tokens should be appended after this block. */
}
"#;

use std::fmt::Write;

use crate::theme::Theme;
use crate::tokens::ColorScaleTokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticVariable {
    Default,
    DefaultForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Success,
    SuccessForeground,
    Warning,
    WarningForeground,
    Danger,
    DangerForeground,
    LayoutBackground,
    LayoutForeground,
    LayoutDivider,
    LayoutFocus,
    LayoutContent1,
    LayoutContent2,
    LayoutContent3,
    LayoutContent4,
    SemanticFg,
    SemanticFgMuted,
    SemanticBg,
    SemanticBgMuted,
    SemanticAccent,
    SemanticAccentForeground,
    SemanticAccentSoft,
    SemanticDanger,
    SemanticDangerForeground,
    SemanticBorder,
    SemanticFocusRing,
}

impl SemanticVariable {
    pub const fn as_css_var(self) -> &'static str {
        match self {
            Self::Default => "--ui-default",
            Self::DefaultForeground => "--ui-default-foreground",
            Self::Primary => "--ui-primary",
            Self::PrimaryForeground => "--ui-primary-foreground",
            Self::Secondary => "--ui-secondary",
            Self::SecondaryForeground => "--ui-secondary-foreground",
            Self::Success => "--ui-success",
            Self::SuccessForeground => "--ui-success-foreground",
            Self::Warning => "--ui-warning",
            Self::WarningForeground => "--ui-warning-foreground",
            Self::Danger => "--ui-danger",
            Self::DangerForeground => "--ui-danger-foreground",
            Self::LayoutBackground => "--ui-layout-background",
            Self::LayoutForeground => "--ui-layout-foreground",
            Self::LayoutDivider => "--ui-layout-divider",
            Self::LayoutFocus => "--ui-layout-focus",
            Self::LayoutContent1 => "--ui-layout-content-1",
            Self::LayoutContent2 => "--ui-layout-content-2",
            Self::LayoutContent3 => "--ui-layout-content-3",
            Self::LayoutContent4 => "--ui-layout-content-4",
            Self::SemanticFg => "--ui-semantic-fg",
            Self::SemanticFgMuted => "--ui-semantic-fg-muted",
            Self::SemanticBg => "--ui-semantic-bg",
            Self::SemanticBgMuted => "--ui-semantic-bg-muted",
            Self::SemanticAccent => "--ui-semantic-accent",
            Self::SemanticAccentForeground => "--ui-semantic-accent-fg",
            Self::SemanticAccentSoft => "--ui-semantic-accent-soft",
            Self::SemanticDanger => "--ui-semantic-danger",
            Self::SemanticDangerForeground => "--ui-semantic-danger-fg",
            Self::SemanticBorder => "--ui-semantic-border",
            Self::SemanticFocusRing => "--ui-semantic-focus-ring",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SemanticOverrides {
    entries: Vec<(SemanticVariable, String)>,
}

impl SemanticOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, variable: SemanticVariable, value: impl Into<String>) -> Self {
        let value = value.into();
        if let Some((_, existing)) = self.entries.iter_mut().find(|(key, _)| *key == variable) {
            *existing = value;
        } else {
            self.entries.push((variable, value));
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn to_css_block(&self, selector: &str) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut css = String::new();
        let _ = writeln!(css, "{selector} {{");
        for (variable, value) in &self.entries {
            let _ = writeln!(css, "  {}: {};", variable.as_css_var(), value);
        }
        let _ = writeln!(css, "}}");
        css
    }
}

fn write_scale_variables(css: &mut String, name: &str, scale: ColorScaleTokens) {
    let _ = writeln!(css, "  --ui-{name}-50: {};", scale.shade_50);
    let _ = writeln!(css, "  --ui-{name}-100: {};", scale.shade_100);
    let _ = writeln!(css, "  --ui-{name}-200: {};", scale.shade_200);
    let _ = writeln!(css, "  --ui-{name}-300: {};", scale.shade_300);
    let _ = writeln!(css, "  --ui-{name}-400: {};", scale.shade_400);
    let _ = writeln!(css, "  --ui-{name}-500: {};", scale.shade_500);
    let _ = writeln!(css, "  --ui-{name}-600: {};", scale.shade_600);
    let _ = writeln!(css, "  --ui-{name}-700: {};", scale.shade_700);
    let _ = writeln!(css, "  --ui-{name}-800: {};", scale.shade_800);
    let _ = writeln!(css, "  --ui-{name}-900: {};", scale.shade_900);
}

pub fn theme_to_css_variables(theme: &Theme) -> String {
    let system = theme.ctx.system.as_str();
    let color = theme.ctx.color.as_str();
    let scale = theme.ctx.scale.as_str();
    let scheme = theme.ctx.color.css_color_scheme();
    let palette = &theme.tokens.palette;
    let common_colors = &theme.tokens.common_colors;
    let colors = &theme.tokens.semantic_colors;
    let aliases = &theme.tokens.color_aliases;
    let component_colors = &theme.tokens.component_colors;
    let icons = &theme.tokens.icons;
    let layout = &theme.tokens.layout;
    let layout_semantic = &theme.tokens.layout_semantic;
    let semantic_roles = &theme.tokens.semantic_roles;
    let semantic_scales = &theme.tokens.semantic_scales;
    let typography = &theme.tokens.typography;
    let component_layout = &theme.tokens.component_layout;
    let button_layout = &theme.tokens.button_layout;
    let mut css = String::new();

    let _ = writeln!(css, ":root {{");
    let _ = writeln!(css, "  --ui-system: {system};");
    let _ = writeln!(css, "  --ui-color: {color};");
    let _ = writeln!(css, "  --ui-scale: {scale};");
    let _ = writeln!(css, "  color-scheme: {scheme};");
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-palette-gray-50: {};", palette.gray_50);
    let _ = writeln!(css, "  --ui-palette-gray-200: {};", palette.gray_200);
    let _ = writeln!(css, "  --ui-palette-gray-700: {};", palette.gray_700);
    let _ = writeln!(css, "  --ui-palette-gray-900: {};", palette.gray_900);
    let _ = writeln!(css, "  --ui-palette-accent-500: {};", palette.accent_500);
    let _ = writeln!(css, "  --ui-palette-accent-600: {};", palette.accent_600);
    let _ = writeln!(css, "  --ui-palette-accent-700: {};", palette.accent_700);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-common-white: {};", common_colors.white);
    let _ = writeln!(css, "  --ui-common-black: {};", common_colors.black);
    write_scale_variables(&mut css, "common-blue", common_colors.blue);
    write_scale_variables(&mut css, "common-purple", common_colors.purple);
    write_scale_variables(&mut css, "common-green", common_colors.green);
    write_scale_variables(&mut css, "common-red", common_colors.red);
    write_scale_variables(&mut css, "common-pink", common_colors.pink);
    write_scale_variables(&mut css, "common-yellow", common_colors.yellow);
    write_scale_variables(&mut css, "common-cyan", common_colors.cyan);
    write_scale_variables(&mut css, "common-zinc", common_colors.zinc);
    let _ = writeln!(css);

    write_scale_variables(&mut css, "default", semantic_scales.default);
    write_scale_variables(&mut css, "primary", semantic_scales.primary);
    write_scale_variables(&mut css, "secondary", semantic_scales.secondary);
    write_scale_variables(&mut css, "success", semantic_scales.success);
    write_scale_variables(&mut css, "warning", semantic_scales.warning);
    write_scale_variables(&mut css, "danger", semantic_scales.danger);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-default: {};", semantic_roles.default);
    let _ = writeln!(
        css,
        "  --ui-default-foreground: {};",
        semantic_roles.default_fg
    );
    let _ = writeln!(css, "  --ui-primary: {};", semantic_roles.primary);
    let _ = writeln!(
        css,
        "  --ui-primary-foreground: {};",
        semantic_roles.primary_fg
    );
    let _ = writeln!(css, "  --ui-secondary: {};", semantic_roles.secondary);
    let _ = writeln!(
        css,
        "  --ui-secondary-foreground: {};",
        semantic_roles.secondary_fg
    );
    let _ = writeln!(css, "  --ui-success: {};", semantic_roles.success);
    let _ = writeln!(
        css,
        "  --ui-success-foreground: {};",
        semantic_roles.success_fg
    );
    let _ = writeln!(css, "  --ui-warning: {};", semantic_roles.warning);
    let _ = writeln!(
        css,
        "  --ui-warning-foreground: {};",
        semantic_roles.warning_fg
    );
    let _ = writeln!(css, "  --ui-danger: {};", semantic_roles.danger);
    let _ = writeln!(
        css,
        "  --ui-danger-foreground: {};",
        semantic_roles.danger_fg
    );
    let _ = writeln!(css);

    let _ = writeln!(
        css,
        "  --ui-layout-background: {};",
        layout_semantic.background
    );
    let _ = writeln!(
        css,
        "  --ui-layout-foreground: {};",
        layout_semantic.foreground
    );
    let _ = writeln!(css, "  --ui-layout-divider: {};", layout_semantic.divider);
    let _ = writeln!(css, "  --ui-layout-focus: {};", layout_semantic.focus);
    let _ = writeln!(
        css,
        "  --ui-layout-content-1: {};",
        layout_semantic.content_1
    );
    let _ = writeln!(
        css,
        "  --ui-layout-content-2: {};",
        layout_semantic.content_2
    );
    let _ = writeln!(
        css,
        "  --ui-layout-content-3: {};",
        layout_semantic.content_3
    );
    let _ = writeln!(
        css,
        "  --ui-layout-content-4: {};",
        layout_semantic.content_4
    );
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-semantic-fg: {};", colors.fg);
    let _ = writeln!(css, "  --ui-semantic-fg-muted: {};", colors.fg_muted);
    let _ = writeln!(css, "  --ui-semantic-bg: {};", colors.bg);
    let _ = writeln!(css, "  --ui-semantic-bg-muted: {};", colors.bg_muted);
    let _ = writeln!(css, "  --ui-semantic-accent: {};", colors.accent);
    let _ = writeln!(css, "  --ui-semantic-accent-fg: {};", colors.accent_fg);
    let _ = writeln!(css, "  --ui-semantic-accent-soft: {};", colors.accent_soft);
    let _ = writeln!(css, "  --ui-semantic-danger: {};", colors.danger);
    let _ = writeln!(css, "  --ui-semantic-danger-fg: {};", colors.danger_fg);
    let _ = writeln!(css, "  --ui-semantic-border: {};", colors.border);
    let _ = writeln!(css, "  --ui-semantic-focus-ring: {};", colors.focus_ring);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-alias-text-default: {};", aliases.text_default);
    let _ = writeln!(css, "  --ui-alias-text-muted: {};", aliases.text_muted);
    let _ = writeln!(
        css,
        "  --ui-alias-surface-default: {};",
        aliases.surface_default
    );
    let _ = writeln!(
        css,
        "  --ui-alias-surface-muted: {};",
        aliases.surface_muted
    );
    let _ = writeln!(
        css,
        "  --ui-alias-border-default: {};",
        aliases.border_default
    );
    let _ = writeln!(css, "  --ui-alias-focus-ring: {};", aliases.focus_ring);
    let _ = writeln!(css, "  --ui-alias-accent: {};", aliases.accent);
    let _ = writeln!(css, "  --ui-alias-accent-fg: {};", aliases.accent_fg);
    let _ = writeln!(css, "  --ui-alias-danger: {};", aliases.danger);
    let _ = writeln!(css, "  --ui-alias-danger-fg: {};", aliases.danger_fg);
    let _ = writeln!(css);

    let _ = writeln!(
        css,
        "  --ui-component-control-bg: {};",
        component_colors.control_bg
    );
    let _ = writeln!(
        css,
        "  --ui-component-control-bg-hover: {};",
        component_colors.control_bg_hover
    );
    let _ = writeln!(
        css,
        "  --ui-component-control-border: {};",
        component_colors.control_border
    );
    let _ = writeln!(
        css,
        "  --ui-component-control-fg: {};",
        component_colors.control_fg
    );
    let _ = writeln!(
        css,
        "  --ui-component-surface-raised: {};",
        component_colors.surface_raised
    );
    let _ = writeln!(
        css,
        "  --ui-component-surface-overlay: {};",
        component_colors.surface_overlay
    );
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-icon-size-100: {}px;", icons.size_100_px);
    let _ = writeln!(css, "  --ui-icon-size-200: {}px;", icons.size_200_px);
    let _ = writeln!(css, "  --ui-icon-stroke-100: {};", icons.stroke_100);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-fg: {};", colors.fg);
    let _ = writeln!(css, "  --ui-fg-muted: {};", colors.fg_muted);
    let _ = writeln!(css, "  --ui-bg: {};", colors.bg);
    let _ = writeln!(css, "  --ui-bg-muted: {};", colors.bg_muted);
    let _ = writeln!(css, "  --ui-accent: {};", colors.accent);
    let _ = writeln!(css, "  --ui-accent-fg: {};", colors.accent_fg);
    let _ = writeln!(css, "  --ui-accent-soft: {};", colors.accent_soft);
    let _ = writeln!(css, "  --ui-border: {};", colors.border);
    let _ = writeln!(css, "  --ui-focus-ring: {};", colors.focus_ring);
    let _ = writeln!(css, "  --ui-background: {};", layout_semantic.background);
    let _ = writeln!(css, "  --ui-foreground: {};", layout_semantic.foreground);
    let _ = writeln!(css, "  --ui-divider: {};", layout_semantic.divider);
    let _ = writeln!(css, "  --ui-focus: {};", layout_semantic.focus);
    let _ = writeln!(css, "  --ui-content1: {};", layout_semantic.content_1);
    let _ = writeln!(css, "  --ui-content2: {};", layout_semantic.content_2);
    let _ = writeln!(css, "  --ui-content3: {};", layout_semantic.content_3);
    let _ = writeln!(css, "  --ui-content4: {};", layout_semantic.content_4);
    let _ = writeln!(css, "  --ui-danger-fg: {};", colors.danger_fg);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-radius-sm: {}px;", layout.radius.sm_px);
    let _ = writeln!(css, "  --ui-radius-md: {}px;", layout.radius.md_px);
    let _ = writeln!(css, "  --ui-radius-lg: {}px;", layout.radius.lg_px);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-space-xs: {}px;", layout.space.xs_px);
    let _ = writeln!(css, "  --ui-space-sm: {}px;", layout.space.sm_px);
    let _ = writeln!(css, "  --ui-space-md: {}px;", layout.space.md_px);
    let _ = writeln!(css, "  --ui-space-lg: {}px;", layout.space.lg_px);
    let _ = writeln!(css);

    let _ = writeln!(css, "  --ui-shadow-sm: {};", layout.shadow.sm);
    let _ = writeln!(css, "  --ui-shadow-md: {};", layout.shadow.md);
    let _ = writeln!(css);

    let _ = writeln!(
        css,
        "  --ui-font-size-200: {}px;",
        typography.font_size_200_px
    );
    let _ = writeln!(
        css,
        "  --ui-component-height-100: {}px;",
        component_layout.component_height_100_px
    );
    let _ = writeln!(css);

    let _ = writeln!(
        css,
        "  --ui-button-min-width: {}px;",
        button_layout.min_width_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-font-size: {}px;",
        button_layout.font_size_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-spinner-size: {}px;",
        button_layout.spinner_size_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-spinner-border: {}px;",
        button_layout.spinner_border_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-spinner-duration: {}ms;",
        button_layout.spinner_duration_ms
    );
    let _ = writeln!(
        css,
        "  --ui-button-focus-outline-width: {}px;",
        button_layout.focus_outline_width_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-focus-outline-offset: {}px;",
        button_layout.focus_outline_offset_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-radius-full: {}px;",
        button_layout.radius_full_px
    );
    let _ = writeln!(css);

    let _ = writeln!(
        css,
        "  --ui-button-size-xs-height: {}px;",
        button_layout.xs.height_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xs-padding-x: {}px;",
        button_layout.xs.padding_inline_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xs-gap: {}px;",
        button_layout.xs.gap_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xs-icon: {}px;",
        button_layout.xs.icon_size_px
    );

    let _ = writeln!(
        css,
        "  --ui-button-size-s-height: {}px;",
        button_layout.s.height_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-s-padding-x: {}px;",
        button_layout.s.padding_inline_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-s-gap: {}px;",
        button_layout.s.gap_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-s-icon: {}px;",
        button_layout.s.icon_size_px
    );

    let _ = writeln!(
        css,
        "  --ui-button-size-m-height: {}px;",
        button_layout.m.height_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-m-padding-x: {}px;",
        button_layout.m.padding_inline_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-m-gap: {}px;",
        button_layout.m.gap_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-m-icon: {}px;",
        button_layout.m.icon_size_px
    );

    let _ = writeln!(
        css,
        "  --ui-button-size-l-height: {}px;",
        button_layout.l.height_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-l-padding-x: {}px;",
        button_layout.l.padding_inline_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-l-gap: {}px;",
        button_layout.l.gap_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-l-icon: {}px;",
        button_layout.l.icon_size_px
    );

    let _ = writeln!(
        css,
        "  --ui-button-size-xl-height: {}px;",
        button_layout.xl.height_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xl-padding-x: {}px;",
        button_layout.xl.padding_inline_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xl-gap: {}px;",
        button_layout.xl.gap_px
    );
    let _ = writeln!(
        css,
        "  --ui-button-size-xl-icon: {}px;",
        button_layout.xl.icon_size_px
    );
    let _ = writeln!(css, "}}");
    css
}

pub const SAFE_AREA_CSS: &str = r#"
.safe-area {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_overrides_emit_css_block() {
        let css = SemanticOverrides::new()
            .set(SemanticVariable::Primary, "oklch(66% 0.14 255)")
            .set(SemanticVariable::LayoutBackground, "oklch(98% 0 0)")
            .to_css_block(":root");

        assert!(css.contains(":root {"));
        assert!(css.contains("--ui-primary: oklch(66% 0.14 255);"));
        assert!(css.contains("--ui-layout-background: oklch(98% 0 0);"));
    }

    #[test]
    fn semantic_overrides_last_write_wins() {
        let css = SemanticOverrides::new()
            .set(SemanticVariable::Primary, "oklch(60% 0.1 250)")
            .set(SemanticVariable::Primary, "oklch(64% 0.12 252)")
            .to_css_block(":root");

        assert!(!css.contains("--ui-primary: oklch(60% 0.1 250);"));
        assert!(css.contains("--ui-primary: oklch(64% 0.12 252);"));
    }
}
