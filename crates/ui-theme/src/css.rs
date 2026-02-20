pub const BASE_CSS: &str = r#"
:root {
  /* Generated tokens should be appended after this block. */
}
"#;

use std::fmt::Write;

use crate::theme::{Theme, text_field_motion_tokens};
use crate::tokens::ColorScaleTokens;

macro_rules! css_writeln {
    ($target:expr $(, $arg:expr)*) => {{
        match writeln!($target $(, $arg)*) {
            Ok(()) | Err(_) => {}
        }
    }};
}

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
        css_writeln!(css, "{selector} {{");
        for (variable, value) in &self.entries {
            css_writeln!(css, "  {}: {};", variable.as_css_var(), value);
        }
        css_writeln!(css, "}}");
        css
    }
}

fn write_scale_variables(css: &mut String, name: &str, scale: ColorScaleTokens) {
    css_writeln!(css, "  --ui-{name}-50: {};", scale.shade_50);
    css_writeln!(css, "  --ui-{name}-100: {};", scale.shade_100);
    css_writeln!(css, "  --ui-{name}-200: {};", scale.shade_200);
    css_writeln!(css, "  --ui-{name}-300: {};", scale.shade_300);
    css_writeln!(css, "  --ui-{name}-400: {};", scale.shade_400);
    css_writeln!(css, "  --ui-{name}-500: {};", scale.shade_500);
    css_writeln!(css, "  --ui-{name}-600: {};", scale.shade_600);
    css_writeln!(css, "  --ui-{name}-700: {};", scale.shade_700);
    css_writeln!(css, "  --ui-{name}-800: {};", scale.shade_800);
    css_writeln!(css, "  --ui-{name}-900: {};", scale.shade_900);
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
    let overlay_layout = &theme.tokens.overlay_layout;
    let slider_layout = &theme.tokens.slider_layout;
    let underlay_motion = &theme.tokens.underlay_motion;
    let button_layout = &theme.tokens.button_layout;
    let text_field_motion = text_field_motion_tokens(theme.ctx);
    let mut css = String::new();

    css_writeln!(css, ":root {{");
    css_writeln!(css, "  --ui-system: {system};");
    css_writeln!(css, "  --ui-color: {color};");
    css_writeln!(css, "  --ui-scale: {scale};");
    css_writeln!(css, "  color-scheme: {scheme};");
    css_writeln!(css);

    css_writeln!(css, "  --ui-palette-gray-50: {};", palette.gray_50);
    css_writeln!(css, "  --ui-palette-gray-200: {};", palette.gray_200);
    css_writeln!(css, "  --ui-palette-gray-700: {};", palette.gray_700);
    css_writeln!(css, "  --ui-palette-gray-900: {};", palette.gray_900);
    css_writeln!(css, "  --ui-palette-accent-500: {};", palette.accent_500);
    css_writeln!(css, "  --ui-palette-accent-600: {};", palette.accent_600);
    css_writeln!(css, "  --ui-palette-accent-700: {};", palette.accent_700);
    css_writeln!(css);

    css_writeln!(css, "  --ui-common-white: {};", common_colors.white);
    css_writeln!(css, "  --ui-common-black: {};", common_colors.black);
    write_scale_variables(&mut css, "common-blue", common_colors.blue);
    write_scale_variables(&mut css, "common-purple", common_colors.purple);
    write_scale_variables(&mut css, "common-green", common_colors.green);
    write_scale_variables(&mut css, "common-red", common_colors.red);
    write_scale_variables(&mut css, "common-pink", common_colors.pink);
    write_scale_variables(&mut css, "common-yellow", common_colors.yellow);
    write_scale_variables(&mut css, "common-cyan", common_colors.cyan);
    write_scale_variables(&mut css, "common-zinc", common_colors.zinc);
    css_writeln!(css);

    write_scale_variables(&mut css, "default", semantic_scales.default);
    write_scale_variables(&mut css, "primary", semantic_scales.primary);
    write_scale_variables(&mut css, "secondary", semantic_scales.secondary);
    write_scale_variables(&mut css, "success", semantic_scales.success);
    write_scale_variables(&mut css, "warning", semantic_scales.warning);
    write_scale_variables(&mut css, "danger", semantic_scales.danger);
    css_writeln!(css);

    css_writeln!(css, "  --ui-default: {};", semantic_roles.default);
    css_writeln!(
        css,
        "  --ui-default-foreground: {};",
        semantic_roles.default_fg
    );
    css_writeln!(css, "  --ui-primary: {};", semantic_roles.primary);
    css_writeln!(
        css,
        "  --ui-primary-foreground: {};",
        semantic_roles.primary_fg
    );
    css_writeln!(css, "  --ui-secondary: {};", semantic_roles.secondary);
    css_writeln!(
        css,
        "  --ui-secondary-foreground: {};",
        semantic_roles.secondary_fg
    );
    css_writeln!(css, "  --ui-success: {};", semantic_roles.success);
    css_writeln!(
        css,
        "  --ui-success-foreground: {};",
        semantic_roles.success_fg
    );
    css_writeln!(css, "  --ui-warning: {};", semantic_roles.warning);
    css_writeln!(
        css,
        "  --ui-warning-foreground: {};",
        semantic_roles.warning_fg
    );
    css_writeln!(css, "  --ui-danger: {};", semantic_roles.danger);
    css_writeln!(
        css,
        "  --ui-danger-foreground: {};",
        semantic_roles.danger_fg
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-layout-background: {};",
        layout_semantic.background
    );
    css_writeln!(
        css,
        "  --ui-layout-foreground: {};",
        layout_semantic.foreground
    );
    css_writeln!(css, "  --ui-layout-divider: {};", layout_semantic.divider);
    css_writeln!(css, "  --ui-layout-focus: {};", layout_semantic.focus);
    css_writeln!(
        css,
        "  --ui-layout-content-1: {};",
        layout_semantic.content_1
    );
    css_writeln!(
        css,
        "  --ui-layout-content-2: {};",
        layout_semantic.content_2
    );
    css_writeln!(
        css,
        "  --ui-layout-content-3: {};",
        layout_semantic.content_3
    );
    css_writeln!(
        css,
        "  --ui-layout-content-4: {};",
        layout_semantic.content_4
    );
    css_writeln!(css);

    css_writeln!(css, "  --ui-semantic-fg: {};", colors.fg);
    css_writeln!(css, "  --ui-semantic-fg-muted: {};", colors.fg_muted);
    css_writeln!(css, "  --ui-semantic-bg: {};", colors.bg);
    css_writeln!(css, "  --ui-semantic-bg-muted: {};", colors.bg_muted);
    css_writeln!(css, "  --ui-semantic-accent: {};", colors.accent);
    css_writeln!(css, "  --ui-semantic-accent-fg: {};", colors.accent_fg);
    css_writeln!(css, "  --ui-semantic-accent-soft: {};", colors.accent_soft);
    css_writeln!(css, "  --ui-semantic-danger: {};", colors.danger);
    css_writeln!(css, "  --ui-semantic-danger-fg: {};", colors.danger_fg);
    css_writeln!(css, "  --ui-semantic-border: {};", colors.border);
    css_writeln!(css, "  --ui-semantic-focus-ring: {};", colors.focus_ring);
    css_writeln!(css);

    css_writeln!(css, "  --ui-alias-text-default: {};", aliases.text_default);
    css_writeln!(css, "  --ui-alias-text-muted: {};", aliases.text_muted);
    css_writeln!(
        css,
        "  --ui-alias-surface-default: {};",
        aliases.surface_default
    );
    css_writeln!(
        css,
        "  --ui-alias-surface-muted: {};",
        aliases.surface_muted
    );
    css_writeln!(
        css,
        "  --ui-alias-border-default: {};",
        aliases.border_default
    );
    css_writeln!(css, "  --ui-alias-focus-ring: {};", aliases.focus_ring);
    css_writeln!(css, "  --ui-alias-accent: {};", aliases.accent);
    css_writeln!(css, "  --ui-alias-accent-fg: {};", aliases.accent_fg);
    css_writeln!(css, "  --ui-alias-danger: {};", aliases.danger);
    css_writeln!(css, "  --ui-alias-danger-fg: {};", aliases.danger_fg);
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-component-control-bg: {};",
        component_colors.control_bg
    );
    css_writeln!(
        css,
        "  --ui-component-control-bg-hover: {};",
        component_colors.control_bg_hover
    );
    css_writeln!(
        css,
        "  --ui-component-control-border: {};",
        component_colors.control_border
    );
    css_writeln!(
        css,
        "  --ui-component-control-fg: {};",
        component_colors.control_fg
    );
    css_writeln!(
        css,
        "  --ui-component-surface-raised: {};",
        component_colors.surface_raised
    );
    css_writeln!(
        css,
        "  --ui-component-surface-overlay: {};",
        component_colors.surface_overlay
    );
    css_writeln!(css);

    css_writeln!(css, "  --ui-icon-size-100: {}px;", icons.size_100_px);
    css_writeln!(css, "  --ui-icon-size-200: {}px;", icons.size_200_px);
    css_writeln!(css, "  --ui-icon-stroke-100: {};", icons.stroke_100);
    css_writeln!(css);

    css_writeln!(css, "  --ui-fg: {};", colors.fg);
    css_writeln!(css, "  --ui-fg-muted: {};", colors.fg_muted);
    css_writeln!(css, "  --ui-bg: {};", colors.bg);
    css_writeln!(css, "  --ui-bg-muted: {};", colors.bg_muted);
    css_writeln!(css, "  --ui-accent: {};", colors.accent);
    css_writeln!(css, "  --ui-accent-fg: {};", colors.accent_fg);
    css_writeln!(css, "  --ui-accent-soft: {};", colors.accent_soft);
    css_writeln!(css, "  --ui-border: {};", colors.border);
    css_writeln!(css, "  --ui-focus-ring: {};", colors.focus_ring);
    css_writeln!(css, "  --ui-background: {};", layout_semantic.background);
    css_writeln!(css, "  --ui-foreground: {};", layout_semantic.foreground);
    css_writeln!(css, "  --ui-divider: {};", layout_semantic.divider);
    css_writeln!(css, "  --ui-focus: {};", layout_semantic.focus);
    css_writeln!(css, "  --ui-content1: {};", layout_semantic.content_1);
    css_writeln!(css, "  --ui-content2: {};", layout_semantic.content_2);
    css_writeln!(css, "  --ui-content3: {};", layout_semantic.content_3);
    css_writeln!(css, "  --ui-content4: {};", layout_semantic.content_4);
    css_writeln!(css, "  --ui-danger-fg: {};", colors.danger_fg);
    css_writeln!(css, "  --ui-fallback-fg: {};", colors.fg);
    css_writeln!(css, "  --ui-fallback-fg-muted: {};", colors.fg_muted);
    css_writeln!(css, "  --ui-fallback-bg-muted: {};", colors.bg_muted);
    css_writeln!(css, "  --ui-fallback-border: {};", colors.border);
    css_writeln!(css, "  --ui-fallback-accent: {};", colors.accent);
    css_writeln!(css, "  --ui-fallback-accent-fg: {};", colors.accent_fg);
    css_writeln!(css, "  --ui-fallback-danger: {};", semantic_roles.danger);
    css_writeln!(css, "  --ui-fallback-danger-fg: {};", colors.danger_fg);
    css_writeln!(css);

    css_writeln!(css, "  --ui-radius-sm: {}px;", layout.radius.sm_px);
    css_writeln!(css, "  --ui-radius-md: {}px;", layout.radius.md_px);
    css_writeln!(css, "  --ui-radius-lg: {}px;", layout.radius.lg_px);
    css_writeln!(css, "  --ui-fallback-radius-md: {}px;", layout.radius.md_px);
    css_writeln!(css, "  --ui-fallback-radius-lg: {}px;", layout.radius.lg_px);
    css_writeln!(css);

    css_writeln!(css, "  --ui-space-3xs: {}px;", layout.space.space_3xs_px);
    css_writeln!(css, "  --ui-space-2xs: {}px;", layout.space.space_2xs_px);
    css_writeln!(css, "  --ui-space-xs: {}px;", layout.space.xs_px);
    css_writeln!(css, "  --ui-space-sm: {}px;", layout.space.sm_px);
    css_writeln!(css, "  --ui-space-md: {}px;", layout.space.md_px);
    css_writeln!(css, "  --ui-space-lg: {}px;", layout.space.lg_px);
    css_writeln!(css, "  --ui-fallback-space-xs: {}px;", layout.space.xs_px);
    css_writeln!(css, "  --ui-fallback-space-sm: {}px;", layout.space.sm_px);
    css_writeln!(css, "  --ui-fallback-space-md: {}px;", layout.space.md_px);
    css_writeln!(css);

    css_writeln!(css, "  --ui-shadow-sm: {};", layout.shadow.sm);
    css_writeln!(css, "  --ui-shadow-md: {};", layout.shadow.md);
    css_writeln!(css, "  --ui-fallback-shadow-sm: {};", layout.shadow.sm);
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-font-size-100: {}px;",
        typography.font_size_100_px
    );
    css_writeln!(
        css,
        "  --ui-line-height-100: {}px;",
        typography.line_height_100_px
    );
    css_writeln!(
        css,
        "  --ui-font-size-150: {}px;",
        typography.font_size_150_px
    );
    css_writeln!(
        css,
        "  --ui-line-height-150: {}px;",
        typography.line_height_150_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-font-size-150: {}px;",
        typography.font_size_150_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-line-height-150: {}px;",
        typography.line_height_150_px
    );
    css_writeln!(
        css,
        "  --ui-font-size-200: {}px;",
        typography.font_size_200_px
    );
    css_writeln!(
        css,
        "  --ui-line-height-200: {}px;",
        typography.line_height_200_px
    );
    css_writeln!(
        css,
        "  --ui-body-font-size: {}px;",
        typography.body_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-body-line-height: {}px;",
        typography.body_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h1-font-size: {}px;",
        typography.heading_h1_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h1-line-height: {}px;",
        typography.heading_h1_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h2-font-size: {}px;",
        typography.heading_h2_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h2-line-height: {}px;",
        typography.heading_h2_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h3-font-size: {}px;",
        typography.heading_h3_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h3-line-height: {}px;",
        typography.heading_h3_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h4-font-size: {}px;",
        typography.heading_h4_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h4-line-height: {}px;",
        typography.heading_h4_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h5-font-size: {}px;",
        typography.heading_h5_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h5-line-height: {}px;",
        typography.heading_h5_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h6-font-size: {}px;",
        typography.heading_h6_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-heading-h6-line-height: {}px;",
        typography.heading_h6_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-heading-h6-font-size: {}px;",
        typography.heading_h6_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-heading-h6-line-height: {}px;",
        typography.heading_h6_line_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-heading-h6-line-height-inline: {}px;",
        typography.line_height_150_px
    );
    css_writeln!(css, "  --ui-fallback-alert-icon-size: 20px;");
    css_writeln!(css, "  --ui-fallback-alert-icon-size-inline: 18px;");
    css_writeln!(css, "  --ui-fallback-alert-icon-margin-top-inline: 1px;");
    css_writeln!(css, "  --ui-fallback-alert-body-gap: 2px;");
    css_writeln!(css, "  --ui-fallback-alert-body-font-size: 13px;");
    css_writeln!(css, "  --ui-fallback-alert-body-line-height: 1.45;");
    css_writeln!(css, "  --ui-fallback-alert-sr-only-size: 1px;");
    css_writeln!(css, "  --ui-fallback-alert-translate-y: 0px;");
    css_writeln!(css, "  --ui-fallback-alert-scale: 1;");
    // Semantic aliases used by some component styles.
    css_writeln!(
        css,
        "  --ui-font-size-sm: {}px;",
        typography.font_size_100_px
    );
    css_writeln!(
        css,
        "  --ui-font-size-md: {}px;",
        typography.font_size_150_px
    );
    css_writeln!(
        css,
        "  --ui-font-size-lg: {}px;",
        typography.font_size_200_px
    );
    css_writeln!(
        css,
        "  --ui-component-height-100: {}px;",
        component_layout.component_height_100_px
    );
    let separator_decorative_opacity =
        f64::from(component_layout.separator_decorative_opacity_percent) / 100.0;
    css_writeln!(
        css,
        "  --ui-separator-decorative-opacity: {};",
        separator_decorative_opacity
    );
    css_writeln!(css, "  --ui-overlay-z-index: {};", overlay_layout.z_index);
    css_writeln!(
        css,
        "  --ui-overlay-panel-min-width: {}px;",
        overlay_layout.panel_min_width_px
    );
    css_writeln!(
        css,
        "  --ui-tooltip-max-width: {}px;",
        overlay_layout.panel_min_width_px
    );
    css_writeln!(
        css,
        "  --ui-overlay-viewport-inset: {}px;",
        overlay_layout.viewport_inset_px
    );
    css_writeln!(
        css,
        "  --ui-overlay-enter-offset-y: {}px;",
        overlay_layout.enter_offset_y_px
    );
    css_writeln!(
        css,
        "  --ui-overlay-enter-scale: {};",
        overlay_layout.enter_scale
    );
    css_writeln!(
        css,
        "  --ui-slider-max-width: {}px;",
        slider_layout.max_width_px
    );
    css_writeln!(
        css,
        "  --ui-slider-thumb-border-width: {}px;",
        slider_layout.thumb_border_width_px
    );
    css_writeln!(
        css,
        "  --ui-slider-focus-ring-width: {}px;",
        slider_layout.focus_ring_width_px
    );
    css_writeln!(
        css,
        "  --ui-underlay-transition-duration: {}ms;",
        underlay_motion.transition_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-underlay-visibility-duration: {}ms;",
        underlay_motion.visibility_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-underlay-backdrop-blur: {}px;",
        underlay_motion.backdrop_blur_px
    );
    css_writeln!(
        css,
        "  --ui-underlay-scrim-alpha: {}%;",
        underlay_motion.scrim_alpha_percent
    );
    css_writeln!(
        css,
        "  --ui-underlay-transition-easing: {};",
        underlay_motion.transition_easing
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-text-field-motion-duration: {}ms;",
        text_field_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-text-field-motion-easing: {};",
        text_field_motion.easing
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-button-min-width: {}px;",
        button_layout.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-font-size: {}px;",
        button_layout.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-spinner-size: {}px;",
        button_layout.spinner_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-spinner-border: {}px;",
        button_layout.spinner_border_px
    );
    css_writeln!(
        css,
        "  --ui-button-spinner-duration: {}ms;",
        button_layout.spinner_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-button-focus-outline-width: {}px;",
        button_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-focus-outline-offset: {}px;",
        button_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-button-radius-full: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-button-size-xs-height: {}px;",
        button_layout.xs.height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-min-width: {}px;",
        button_layout.xs.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-padding-x: {}px;",
        button_layout.xs.padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-font-size: {}px;",
        button_layout.xs.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-line-height: {}px;",
        button_layout.xs.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-gap: {}px;",
        button_layout.xs.gap_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xs-icon: {}px;",
        button_layout.xs.icon_size_px
    );

    css_writeln!(
        css,
        "  --ui-button-size-s-height: {}px;",
        button_layout.s.height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-min-width: {}px;",
        button_layout.s.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-padding-x: {}px;",
        button_layout.s.padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-font-size: {}px;",
        button_layout.s.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-line-height: {}px;",
        button_layout.s.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-gap: {}px;",
        button_layout.s.gap_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-s-icon: {}px;",
        button_layout.s.icon_size_px
    );

    css_writeln!(
        css,
        "  --ui-button-size-m-height: {}px;",
        button_layout.m.height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-min-width: {}px;",
        button_layout.m.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-padding-x: {}px;",
        button_layout.m.padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-font-size: {}px;",
        button_layout.m.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-line-height: {}px;",
        button_layout.m.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-gap: {}px;",
        button_layout.m.gap_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-m-icon: {}px;",
        button_layout.m.icon_size_px
    );

    css_writeln!(
        css,
        "  --ui-button-size-l-height: {}px;",
        button_layout.l.height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-min-width: {}px;",
        button_layout.l.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-padding-x: {}px;",
        button_layout.l.padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-font-size: {}px;",
        button_layout.l.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-line-height: {}px;",
        button_layout.l.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-gap: {}px;",
        button_layout.l.gap_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-icon: {}px;",
        button_layout.l.icon_size_px
    );

    css_writeln!(
        css,
        "  --ui-button-size-xl-height: {}px;",
        button_layout.xl.height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-min-width: {}px;",
        button_layout.xl.min_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-padding-x: {}px;",
        button_layout.xl.padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-font-size: {}px;",
        button_layout.xl.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-line-height: {}px;",
        button_layout.xl.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-gap: {}px;",
        button_layout.xl.gap_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-xl-icon: {}px;",
        button_layout.xl.icon_size_px
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-fallback-common-white: {};",
        common_colors.white
    );
    css_writeln!(
        css,
        "  --ui-fallback-font-size-150: {}px;",
        typography.font_size_150_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-line-height-150: {}px;",
        typography.line_height_150_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-size-s-font-size: {}px;",
        button_layout.s.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-size-s-line-height: {}px;",
        button_layout.s.line_height_px
    );
    css_writeln!(css, "}}");
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
#[path = "test/css.rs"]
mod tests;
