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
    let color_swatch_layout = &theme.tokens.color_swatch_layout;
    let color_wheel_layout = &theme.tokens.color_wheel_layout;
    let color_wheel_hue = &theme.tokens.color_wheel_hue;
    let underlay_motion = &theme.tokens.underlay_motion;
    let drop_zone_layout = drop_zone_layout_tokens(theme.ctx);
    let flip_card_layout = flip_card_layout_tokens(theme.ctx);
    let button_layout = &theme.tokens.button_layout;
    let checkbox_layout = &theme.tokens.checkbox_layout;
    let switch_layout = &theme.tokens.switch_layout;
    let command_layout = &theme.tokens.command_layout;
    let checkbox_group_layout = &theme.tokens.checkbox_group_layout;
    let disabled_opacity = f64::from(checkbox_group_layout.disabled_opacity_percent) / 100.0;
    let text_field_motion = text_field_motion_tokens(theme.ctx);
    let label_motion = label_motion_tokens(theme.ctx);
    let checkbox_group_motion = checkbox_group_motion_tokens(theme.ctx);
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
    css_writeln!(
        css,
        "  --ui-fallback-icon-size-100: {}px;",
        icons.size_100_px
    );
    css_writeln!(css, "  --ui-icon-size-200: {}px;", icons.size_200_px);
    css_writeln!(
        css,
        "  --ui-fallback-icon-size-200: {}px;",
        icons.size_200_px
    );
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
    css_writeln!(css, "  --ui-fallback-bg: {};", colors.bg);
    css_writeln!(css, "  --ui-fallback-bg-muted: {};", colors.bg_muted);
    css_writeln!(css, "  --ui-fallback-border: {};", colors.border);
    css_writeln!(css, "  --ui-border-width: 1px;");
    css_writeln!(css, "  --ui-fallback-border-width: 1px;");
    css_writeln!(css, "  --ui-disabled-opacity: {};", disabled_opacity);
    css_writeln!(
        css,
        "  --ui-fallback-disabled-opacity: {};",
        disabled_opacity
    );
    css_writeln!(css, "  --ui-min-inline-size-none: 0px;");
    css_writeln!(css, "  --ui-fallback-min-inline-size-none: 0px;");
    css_writeln!(css, "  --ui-fallback-accent: {};", colors.accent);
    css_writeln!(css, "  --ui-fallback-accent-fg: {};", colors.accent_fg);
    css_writeln!(css, "  --ui-fallback-accent-soft: {};", colors.accent_soft);
    css_writeln!(css, "  --ui-fallback-focus-ring: {};", colors.focus_ring);
    css_writeln!(css, "  --ui-fallback-danger: {};", semantic_roles.danger);
    css_writeln!(css, "  --ui-fallback-danger-fg: {};", colors.danger_fg);
    css_writeln!(css);

    css_writeln!(css, "  --ui-radius-sm: {}px;", layout.radius.sm_px);
    css_writeln!(css, "  --ui-radius-md: {}px;", layout.radius.md_px);
    css_writeln!(css, "  --ui-radius-lg: {}px;", layout.radius.lg_px);
    css_writeln!(css, "  --ui-radius-full: 999px;");
    css_writeln!(css, "  --ui-fallback-radius-sm: {}px;", layout.radius.sm_px);
    css_writeln!(css, "  --ui-fallback-radius-md: {}px;", layout.radius.md_px);
    css_writeln!(css, "  --ui-fallback-radius-lg: {}px;", layout.radius.lg_px);
    css_writeln!(css, "  --ui-fallback-radius-full: 999px;");
    css_writeln!(css);

    let space_xl_px = u32::from(layout.space.lg_px) * 2;
    css_writeln!(css, "  --ui-space-3xs: {}px;", layout.space.space_3xs_px);
    css_writeln!(css, "  --ui-space-2xs: {}px;", layout.space.space_2xs_px);
    css_writeln!(css, "  --ui-space-xs: {}px;", layout.space.xs_px);
    css_writeln!(css, "  --ui-space-sm: {}px;", layout.space.sm_px);
    css_writeln!(css, "  --ui-space-md: {}px;", layout.space.md_px);
    css_writeln!(css, "  --ui-space-lg: {}px;", layout.space.lg_px);
    css_writeln!(css, "  --ui-space-xl: {}px;", space_xl_px);
    css_writeln!(
        css,
        "  --ui-fallback-space-3xs: {}px;",
        layout.space.space_3xs_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-space-2xs: {}px;",
        layout.space.space_2xs_px
    );
    css_writeln!(css, "  --ui-fallback-space-xs: {}px;", layout.space.xs_px);
    css_writeln!(css, "  --ui-fallback-space-sm: {}px;", layout.space.sm_px);
    css_writeln!(css, "  --ui-fallback-space-md: {}px;", layout.space.md_px);
    css_writeln!(css, "  --ui-fallback-space-lg: {}px;", layout.space.lg_px);
    css_writeln!(css, "  --ui-fallback-space-xl: {}px;", space_xl_px);
    css_writeln!(
        css,
        "  --ui-fieldset-horizontal-legend-min-inline-size: 128px;"
    );
    css_writeln!(
        css,
        "  --ui-fallback-fieldset-horizontal-legend-min-inline-size: 128px;"
    );
    css_writeln!(
        css,
        "  --ui-fieldset-horizontal-legend-max-inline-size: 224px;"
    );
    css_writeln!(
        css,
        "  --ui-fallback-fieldset-horizontal-legend-max-inline-size: 224px;"
    );
    css_writeln!(css);

    css_writeln!(css, "  --ui-shadow-sm: {};", layout.shadow.sm);
    css_writeln!(css, "  --ui-shadow-md: {};", layout.shadow.md);
    css_writeln!(css, "  --ui-fallback-shadow-sm: {};", layout.shadow.sm);
    css_writeln!(css, "  --ui-fallback-shadow-md: {};", layout.shadow.md);
    css_writeln!(css, "  --ui-image-zoom-initial: 1;");
    css_writeln!(css, "  --ui-fallback-image-zoom-initial: 1;");
    css_writeln!(css, "  --ui-image-blur: 14px;");
    css_writeln!(css, "  --ui-fallback-image-blur: 14px;");
    css_writeln!(css, "  --ui-image-blur-scale: 1.12;");
    css_writeln!(css, "  --ui-fallback-image-blur-scale: 1.12;");
    css_writeln!(css, "  --ui-image-blur-opacity: 0.45;");
    css_writeln!(css, "  --ui-fallback-image-blur-opacity: 0.45;");
    css_writeln!(css, "  --ui-image-skeleton-fg-mix: 10%;");
    css_writeln!(css, "  --ui-fallback-image-skeleton-fg-mix: 10%;");
    css_writeln!(css, "  --ui-image-skeleton-bg-size: 220% 100%;");
    css_writeln!(css, "  --ui-fallback-image-skeleton-bg-size: 220% 100%;");
    css_writeln!(css, "  --ui-image-skeleton-duration: 1.3s;");
    css_writeln!(css, "  --ui-fallback-image-skeleton-duration: 1.3s;");
    css_writeln!(css, "  --ui-image-shimmer-start: 120%;");
    css_writeln!(css, "  --ui-fallback-image-shimmer-start: 120%;");
    css_writeln!(css, "  --ui-image-shimmer-end: -120%;");
    css_writeln!(css, "  --ui-fallback-image-shimmer-end: -120%;");
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
        "  --ui-fallback-font-size-100: {}px;",
        typography.font_size_100_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-line-height-100: {}px;",
        typography.line_height_100_px
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
        "  --ui-fallback-heading-h5-font-size: {}px;",
        typography.heading_h5_font_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-heading-h5-line-height: {}px;",
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
    css_writeln!(css, "  --ui-fallback-alert-opacity: 1;");
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
    css_writeln!(
        css,
        "  --ui-fallback-component-height-100: {}px;",
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
        "  --ui-fallback-overlay-z-index: {};",
        overlay_layout.z_index
    );
    css_writeln!(
        css,
        "  --ui-overlay-panel-min-width: {}px;",
        overlay_layout.panel_min_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-overlay-panel-min-width: {}px;",
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
        "  --ui-fallback-overlay-viewport-inset: {}px;",
        overlay_layout.viewport_inset_px
    );
    css_writeln!(
        css,
        "  --ui-overlay-enter-offset-y: {}px;",
        overlay_layout.enter_offset_y_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-overlay-enter-offset-y: {}px;",
        overlay_layout.enter_offset_y_px
    );
    css_writeln!(
        css,
        "  --ui-overlay-enter-scale: {};",
        overlay_layout.enter_scale
    );
    css_writeln!(
        css,
        "  --ui-fallback-overlay-enter-scale: {};",
        overlay_layout.enter_scale
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-min-height: {}px;",
        drop_zone_layout.min_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-min-height: {}px;",
        drop_zone_layout.min_height_px
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-border-width: {}px;",
        drop_zone_layout.border_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-border-width: {}px;",
        drop_zone_layout.border_width_px
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-disabled-opacity: {};",
        f64::from(drop_zone_layout.disabled_opacity_percent) / 100.0
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-disabled-opacity: {};",
        f64::from(drop_zone_layout.disabled_opacity_percent) / 100.0
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-focus-outline-width: {}px;",
        drop_zone_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-focus-outline-width: {}px;",
        drop_zone_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-focus-outline-offset: {}px;",
        drop_zone_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-focus-outline-offset: {}px;",
        drop_zone_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-drop-zone-sr-only-size: {}px;",
        drop_zone_layout.sr_only_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-drop-zone-sr-only-size: {}px;",
        drop_zone_layout.sr_only_size_px
    );
    css_writeln!(
        css,
        "  --ui-flip-card-max-inline-size: {}px;",
        flip_card_layout.max_inline_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-max-inline-size: {}px;",
        flip_card_layout.max_inline_size_px
    );
    css_writeln!(
        css,
        "  --ui-flip-card-max-inline-viewport: {}vw;",
        flip_card_layout.max_inline_viewport_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-max-inline-viewport: {}vw;",
        flip_card_layout.max_inline_viewport_percent
    );
    css_writeln!(
        css,
        "  --ui-flip-card-aspect-ratio-width: {};",
        flip_card_layout.aspect_ratio_width
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-aspect-ratio-width: {};",
        flip_card_layout.aspect_ratio_width
    );
    css_writeln!(
        css,
        "  --ui-flip-card-aspect-ratio-height: {};",
        flip_card_layout.aspect_ratio_height
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-aspect-ratio-height: {};",
        flip_card_layout.aspect_ratio_height
    );
    css_writeln!(
        css,
        "  --ui-flip-card-perspective: {}px;",
        flip_card_layout.perspective_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-perspective: {}px;",
        flip_card_layout.perspective_px
    );
    css_writeln!(
        css,
        "  --ui-flip-card-disabled-opacity: {};",
        f64::from(flip_card_layout.disabled_opacity_percent) / 100.0
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-disabled-opacity: {};",
        f64::from(flip_card_layout.disabled_opacity_percent) / 100.0
    );
    css_writeln!(
        css,
        "  --ui-flip-card-focus-outline-width: {}px;",
        flip_card_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-focus-outline-width: {}px;",
        flip_card_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-flip-card-title-font-weight: {};",
        flip_card_layout.title_font_weight
    );
    css_writeln!(
        css,
        "  --ui-fallback-flip-card-title-font-weight: {};",
        flip_card_layout.title_font_weight
    );
    css_writeln!(
        css,
        "  --ui-slider-max-width: {}px;",
        slider_layout.max_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-slider-max-width: {}px;",
        slider_layout.max_width_px
    );
    css_writeln!(
        css,
        "  --ui-slider-thumb-border-width: {}px;",
        slider_layout.thumb_border_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-slider-thumb-border-width: {}px;",
        slider_layout.thumb_border_width_px
    );
    css_writeln!(
        css,
        "  --ui-slider-focus-ring-width: {}px;",
        slider_layout.focus_ring_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-slider-focus-ring-width: {}px;",
        slider_layout.focus_ring_width_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-size-xs: {}px;",
        color_swatch_layout.size_xs_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-size-sm: {}px;",
        color_swatch_layout.size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-size-md: {}px;",
        color_swatch_layout.size_md_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-size-lg: {}px;",
        color_swatch_layout.size_lg_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-size-xs: {}px;",
        color_swatch_layout.size_xs_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-size-sm: {}px;",
        color_swatch_layout.size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-size-md: {}px;",
        color_swatch_layout.size_md_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-size-lg: {}px;",
        color_swatch_layout.size_lg_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-radius-default: {}px;",
        color_swatch_layout.radius_default_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-radius-none: {}px;",
        color_swatch_layout.radius_none_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-radius-full: {}px;",
        color_swatch_layout.radius_full_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-radius-default: {}px;",
        color_swatch_layout.radius_default_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-radius-none: {}px;",
        color_swatch_layout.radius_none_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-radius-full: {}px;",
        color_swatch_layout.radius_full_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-wide-multiplier: {};",
        color_swatch_layout.shape_wide_multiplier
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-wide-multiplier: {};",
        color_swatch_layout.shape_wide_multiplier
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-checker-size: {}px;",
        color_swatch_layout.checker_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-checker-size: {}px;",
        color_swatch_layout.checker_size_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-slash-width: {}px;",
        color_swatch_layout.slash_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-slash-width: {}px;",
        color_swatch_layout.slash_width_px
    );
    css_writeln!(
        css,
        "  --ui-color-swatch-border-width: {}px;",
        color_swatch_layout.border_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-swatch-border-width: {}px;",
        color_swatch_layout.border_width_px
    );
    css_writeln!(css, "  --ui-color-swatch-y: 0px;");
    css_writeln!(css, "  --ui-fallback-color-swatch-y: 0px;");
    css_writeln!(css, "  --ui-color-thumb-x-start: 16%;");
    css_writeln!(css, "  --ui-color-thumb-x-center: 50%;");
    css_writeln!(css, "  --ui-color-thumb-x-end: 84%;");
    css_writeln!(css, "  --ui-color-thumb-y-start: 16%;");
    css_writeln!(css, "  --ui-color-thumb-y-center: 50%;");
    css_writeln!(css, "  --ui-color-thumb-y-end: 84%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-x-start: 16%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-x-center: 50%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-x-end: 84%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-y-start: 16%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-y-center: 50%;");
    css_writeln!(css, "  --ui-fallback-color-thumb-y-end: 84%;");
    css_writeln!(css, "  --ui-color-thumb-handle-size: 18px;");
    css_writeln!(css, "  --ui-fallback-color-thumb-handle-size: 18px;");
    css_writeln!(css, "  --ui-color-thumb-loupe-size: 30px;");
    css_writeln!(css, "  --ui-fallback-color-thumb-loupe-size: 30px;");
    css_writeln!(
        css,
        "  --ui-color-thumb-radius-full: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-thumb-radius-full: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(css, "  --ui-color-thumb-handle-border-width: 2px;");
    css_writeln!(css, "  --ui-fallback-color-thumb-handle-border-width: 2px;");
    css_writeln!(css, "  --ui-color-thumb-loupe-border-width: 1px;");
    css_writeln!(css, "  --ui-fallback-color-thumb-loupe-border-width: 1px;");
    css_writeln!(css, "  --ui-color-thumb-loupe-padding: 2px;");
    css_writeln!(css, "  --ui-fallback-color-thumb-loupe-padding: 2px;");
    css_writeln!(css, "  --ui-color-thumb-loupe-hidden-offset: 0.2rem;");
    css_writeln!(
        css,
        "  --ui-fallback-color-thumb-loupe-hidden-offset: 0.2rem;"
    );
    css_writeln!(css, "  --ui-color-thumb-loupe-hidden-scale: 0.88;");
    css_writeln!(css, "  --ui-fallback-color-thumb-loupe-hidden-scale: 0.88;");
    css_writeln!(css, "  --ui-color-thumb-disabled-opacity: 0.58;");
    css_writeln!(css, "  --ui-fallback-color-thumb-disabled-opacity: 0.58;");
    css_writeln!(
        css,
        "  --ui-color-wheel-size: {}px;",
        color_wheel_layout.size_px
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-track-thickness: {}px;",
        color_wheel_layout.track_thickness_px
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-thumb-size: {}px;",
        color_wheel_layout.thumb_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-size: {}px;",
        color_wheel_layout.size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-track-thickness: {}px;",
        color_wheel_layout.track_thickness_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-thumb-size: {}px;",
        color_wheel_layout.thumb_size_px
    );
    css_writeln!(css, "  --ui-color-wheel-hue-red: {};", color_wheel_hue.red);
    css_writeln!(
        css,
        "  --ui-color-wheel-hue-yellow: {};",
        color_wheel_hue.yellow
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-hue-green: {};",
        color_wheel_hue.green
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-hue-cyan: {};",
        color_wheel_hue.cyan
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-hue-blue: {};",
        color_wheel_hue.blue
    );
    css_writeln!(
        css,
        "  --ui-color-wheel-hue-magenta: {};",
        color_wheel_hue.magenta
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-red: {};",
        color_wheel_hue.red
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-yellow: {};",
        color_wheel_hue.yellow
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-green: {};",
        color_wheel_hue.green
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-cyan: {};",
        color_wheel_hue.cyan
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-blue: {};",
        color_wheel_hue.blue
    );
    css_writeln!(
        css,
        "  --ui-fallback-color-wheel-hue-magenta: {};",
        color_wheel_hue.magenta
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
        "  --ui-fallback-underlay-backdrop-blur: {}px;",
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
        "  --ui-fallback-text-field-motion-duration: {}ms;",
        text_field_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-text-field-motion-easing: {};",
        text_field_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-fallback-text-field-motion-easing: {};",
        text_field_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-label-motion-color-duration: {}ms;",
        label_motion.color_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-label-motion-color-duration: {}ms;",
        label_motion.color_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-label-motion-weight-duration: {}ms;",
        label_motion.weight_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-label-motion-weight-duration: {}ms;",
        label_motion.weight_duration_ms
    );
    css_writeln!(css, "  --ui-label-motion-easing: {};", label_motion.easing);
    css_writeln!(
        css,
        "  --ui-fallback-label-motion-easing: {};",
        label_motion.easing
    );
    css_writeln!(css, "  --ui-meter-track-height: {}px;", layout.space.sm_px);
    css_writeln!(
        css,
        "  --ui-fallback-meter-track-height: {}px;",
        layout.space.sm_px
    );
    css_writeln!(
        css,
        "  --ui-meter-track-height-sm: {}px;",
        layout.space.xs_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-track-height-sm: {}px;",
        layout.space.xs_px
    );
    css_writeln!(
        css,
        "  --ui-meter-track-height-lg: {}px;",
        layout.space.md_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-track-height-lg: {}px;",
        layout.space.md_px
    );
    css_writeln!(css, "  --ui-meter-track-radius: {}px;", layout.radius.lg_px);
    css_writeln!(
        css,
        "  --ui-fallback-meter-track-radius: {}px;",
        layout.radius.lg_px
    );
    css_writeln!(
        css,
        "  --ui-meter-track-border-width: var(--ui-border-width, var(--ui-fallback-border-width));"
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-track-border-width: var(--ui-fallback-border-width);"
    );
    css_writeln!(css, "  --ui-meter-indicator-color: {};", colors.accent);
    css_writeln!(
        css,
        "  --ui-fallback-meter-indicator-color: {};",
        colors.accent
    );
    css_writeln!(
        css,
        "  --ui-meter-indicator-color-danger: {};",
        semantic_roles.danger
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-indicator-color-danger: {};",
        semantic_roles.danger
    );
    css_writeln!(css, "  --ui-meter-progress: 0;");
    css_writeln!(css, "  --ui-fallback-meter-progress: 0;");
    css_writeln!(css, "  --ui-meter-determinate-width: 100%;");
    css_writeln!(css, "  --ui-fallback-meter-determinate-width: 100%;");
    css_writeln!(css, "  --ui-meter-indeterminate-width: 40%;");
    css_writeln!(css, "  --ui-fallback-meter-indeterminate-width: 40%;");
    css_writeln!(css, "  --ui-meter-indeterminate-start: -60%;");
    css_writeln!(css, "  --ui-fallback-meter-indeterminate-start: -60%;");
    css_writeln!(css, "  --ui-meter-indeterminate-mid: 80%;");
    css_writeln!(css, "  --ui-fallback-meter-indeterminate-mid: 80%;");
    css_writeln!(css, "  --ui-meter-indeterminate-end: 220%;");
    css_writeln!(css, "  --ui-fallback-meter-indeterminate-end: 220%;");
    css_writeln!(
        css,
        "  --ui-meter-indeterminate-duration: {}ms;",
        text_field_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-indeterminate-duration: {}ms;",
        text_field_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-meter-indeterminate-easing: {};",
        text_field_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-indeterminate-easing: {};",
        text_field_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-meter-shadow-transition-duration: {}ms;",
        label_motion.color_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-shadow-transition-duration: {}ms;",
        label_motion.color_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-meter-shadow-transition-easing: {};",
        label_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-fallback-meter-shadow-transition-easing: {};",
        label_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-duration: {}ms;",
        checkbox_group_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-easing: {};",
        checkbox_group_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-stiffness: {};",
        checkbox_group_motion.spring.stiffness
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-damping: {};",
        checkbox_group_motion.spring.damping
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-mass: {};",
        checkbox_group_motion.spring.mass
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-motion-precision: {};",
        checkbox_group_motion.spring.precision
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
        "  --ui-fallback-button-spinner-size: {}px;",
        button_layout.spinner_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-spinner-border: {}px;",
        button_layout.spinner_border_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-spinner-border: {}px;",
        button_layout.spinner_border_px
    );
    css_writeln!(
        css,
        "  --ui-button-spinner-duration: {}ms;",
        button_layout.spinner_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-spinner-duration: {}ms;",
        button_layout.spinner_duration_ms
    );
    css_writeln!(
        css,
        "  --ui-button-focus-outline-width: {}px;",
        button_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-focus-outline-width: {}px;",
        button_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-button-focus-outline-offset: {}px;",
        button_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-focus-outline-offset: {}px;",
        button_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-button-radius-full: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-radius-full: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(css, "  --ui-avatar-size-sm: 24px;");
    css_writeln!(css, "  --ui-avatar-size-md: 32px;");
    css_writeln!(css, "  --ui-avatar-size-lg: 40px;");
    css_writeln!(css, "  --ui-fallback-avatar-size-sm: 24px;");
    css_writeln!(css, "  --ui-fallback-avatar-size-md: 32px;");
    css_writeln!(css, "  --ui-fallback-avatar-size-lg: 40px;");
    css_writeln!(
        css,
        "  --ui-avatar-radius: {}px;",
        button_layout.radius_full_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-avatar-radius: {}px;",
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
        "  --ui-fallback-button-size-m-height: {}px;",
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
        "  --ui-fallback-button-size-l-font-size: {}px;",
        button_layout.l.font_size_px
    );
    css_writeln!(
        css,
        "  --ui-button-size-l-line-height: {}px;",
        button_layout.l.line_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-button-size-l-line-height: {}px;",
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

    let switch_disabled_opacity = f64::from(switch_layout.disabled_opacity_percent) / 100.0;
    css_writeln!(css, "  --ui-switch-gap: {}px;", switch_layout.gap_px);
    css_writeln!(
        css,
        "  --ui-fallback-switch-gap: {}px;",
        switch_layout.gap_px
    );
    css_writeln!(
        css,
        "  --ui-switch-track-width: {}px;",
        switch_layout.track_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-track-width: {}px;",
        switch_layout.track_width_px
    );
    css_writeln!(
        css,
        "  --ui-switch-track-height: {}px;",
        switch_layout.track_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-track-height: {}px;",
        switch_layout.track_height_px
    );
    css_writeln!(
        css,
        "  --ui-switch-track-padding: {}px;",
        switch_layout.track_padding_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-track-padding: {}px;",
        switch_layout.track_padding_px
    );
    css_writeln!(
        css,
        "  --ui-switch-thumb-size: {}px;",
        switch_layout.thumb_size_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-thumb-size: {}px;",
        switch_layout.thumb_size_px
    );
    css_writeln!(
        css,
        "  --ui-switch-thumb-checked-x: {}px;",
        switch_layout.checked_thumb_x_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-thumb-checked-x: {}px;",
        switch_layout.checked_thumb_x_px
    );
    css_writeln!(
        css,
        "  --ui-switch-focus-outline-width: {}px;",
        switch_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-focus-outline-width: {}px;",
        switch_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-switch-focus-outline-offset: {}px;",
        switch_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-focus-outline-offset: {}px;",
        switch_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-switch-disabled-opacity: {};",
        switch_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-disabled-opacity: {};",
        switch_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-switch-hover-brightness: {};",
        switch_layout.hover_brightness
    );
    css_writeln!(
        css,
        "  --ui-fallback-switch-hover-brightness: {};",
        switch_layout.hover_brightness
    );
    css_writeln!(css);

    let checkbox_disabled_opacity = f64::from(checkbox_layout.disabled_opacity_percent) / 100.0;
    css_writeln!(css, "  --ui-checkbox-gap: {}px;", checkbox_layout.gap_px);
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-gap: {}px;",
        checkbox_layout.gap_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-disabled-opacity: {};",
        checkbox_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-disabled-opacity: {};",
        checkbox_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-checkbox-focus-outline-width: {}px;",
        checkbox_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-focus-outline-width: {}px;",
        checkbox_layout.focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-focus-outline-offset: {}px;",
        checkbox_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-focus-outline-offset: {}px;",
        checkbox_layout.focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-size-default: {}px;",
        checkbox_layout.box_size_default_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-size-default: {}px;",
        checkbox_layout.box_size_default_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-size-sm: {}px;",
        checkbox_layout.box_size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-size-sm: {}px;",
        checkbox_layout.box_size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-size-lg: {}px;",
        checkbox_layout.box_size_lg_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-size-lg: {}px;",
        checkbox_layout.box_size_lg_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-radius-default: {}px;",
        checkbox_layout.box_radius_default_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-radius-default: {}px;",
        checkbox_layout.box_radius_default_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-radius-sm: {}px;",
        checkbox_layout.box_radius_sm_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-radius-sm: {}px;",
        checkbox_layout.box_radius_sm_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-radius-lg: {}px;",
        checkbox_layout.box_radius_lg_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-radius-lg: {}px;",
        checkbox_layout.box_radius_lg_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-indicator-size-default: {}px;",
        checkbox_layout.indicator_size_default_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-indicator-size-default: {}px;",
        checkbox_layout.indicator_size_default_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-indicator-size-sm: {}px;",
        checkbox_layout.indicator_size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-indicator-size-sm: {}px;",
        checkbox_layout.indicator_size_sm_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-indicator-size-lg: {}px;",
        checkbox_layout.indicator_size_lg_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-indicator-size-lg: {}px;",
        checkbox_layout.indicator_size_lg_px
    );
    let checkbox_group_disabled_opacity =
        f64::from(checkbox_group_layout.disabled_opacity_percent) / 100.0;
    css_writeln!(
        css,
        "  --ui-checkbox-group-gap: {}px;",
        checkbox_group_layout.gap_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-required-marker-gap: {}px;",
        checkbox_group_layout.required_marker_gap_px
    );
    css_writeln!(
        css,
        "  --ui-checkbox-group-disabled-opacity: {};",
        checkbox_group_disabled_opacity
    );
    css_writeln!(css);

    let command_disabled_opacity = f64::from(command_layout.disabled_opacity_percent) / 100.0;
    let command_option_disabled_opacity =
        f64::from(command_layout.option_disabled_opacity_percent) / 100.0;
    let command_group_heading_letter_spacing =
        f64::from(command_layout.group_heading_letter_spacing_centiem) / 100.0;
    css_writeln!(
        css,
        "  --ui-command-panel-max-width: {}px;",
        command_layout.panel_max_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-panel-max-width: {}px;",
        command_layout.panel_max_width_px
    );
    css_writeln!(
        css,
        "  --ui-command-list-max-height: {}px;",
        command_layout.list_max_height_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-list-max-height: {}px;",
        command_layout.list_max_height_px
    );
    css_writeln!(
        css,
        "  --ui-command-disabled-opacity: {};",
        command_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-disabled-opacity: {};",
        command_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-command-input-wrap-padding: {}px;",
        command_layout.input_wrap_padding_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-wrap-padding: {}px;",
        command_layout.input_wrap_padding_px
    );
    css_writeln!(
        css,
        "  --ui-command-input-wrap-border-mix: {}%;",
        command_layout.input_wrap_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-wrap-border-mix: {}%;",
        command_layout.input_wrap_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-input-wrap-bg-mix: {}%;",
        command_layout.input_wrap_bg_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-wrap-bg-mix: {}%;",
        command_layout.input_wrap_bg_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-input-padding-y: {}px;",
        command_layout.input_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-padding-y: {}px;",
        command_layout.input_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-command-input-padding-x: {}px;",
        command_layout.input_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-padding-x: {}px;",
        command_layout.input_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-command-input-focus-outline-width: {}px;",
        command_layout.input_focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-focus-outline-width: {}px;",
        command_layout.input_focus_outline_width_px
    );
    css_writeln!(
        css,
        "  --ui-command-input-focus-outline-offset: {}px;",
        command_layout.input_focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-input-focus-outline-offset: {}px;",
        command_layout.input_focus_outline_offset_px
    );
    css_writeln!(
        css,
        "  --ui-command-options-padding: {}px;",
        command_layout.options_padding_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-options-padding: {}px;",
        command_layout.options_padding_px
    );
    css_writeln!(
        css,
        "  --ui-command-group-gap: {}px;",
        command_layout.group_gap_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-gap: {}px;",
        command_layout.group_gap_px
    );
    css_writeln!(
        css,
        "  --ui-command-group-spacing: {}px;",
        command_layout.group_spacing_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-spacing: {}px;",
        command_layout.group_spacing_px
    );
    css_writeln!(
        css,
        "  --ui-command-group-border-mix: {}%;",
        command_layout.group_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-border-mix: {}%;",
        command_layout.group_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-group-heading-padding-x: {}px;",
        command_layout.group_heading_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-heading-padding-x: {}px;",
        command_layout.group_heading_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-command-group-heading-letter-spacing: {}em;",
        command_group_heading_letter_spacing
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-heading-letter-spacing: {}em;",
        command_group_heading_letter_spacing
    );
    css_writeln!(
        css,
        "  --ui-command-group-items-gap: {}px;",
        command_layout.group_items_gap_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-group-items-gap: {}px;",
        command_layout.group_items_gap_px
    );
    css_writeln!(
        css,
        "  --ui-command-option-gap: {}px;",
        command_layout.option_gap_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-option-gap: {}px;",
        command_layout.option_gap_px
    );
    css_writeln!(
        css,
        "  --ui-command-option-padding-y: {}px;",
        command_layout.option_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-option-padding-y: {}px;",
        command_layout.option_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-command-option-padding-x: {}px;",
        command_layout.option_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-option-padding-x: {}px;",
        command_layout.option_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-command-option-focus-mix: {}%;",
        command_layout.option_focus_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-option-focus-mix: {}%;",
        command_layout.option_focus_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-option-disabled-opacity: {};",
        command_option_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-option-disabled-opacity: {};",
        command_option_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-command-shortcut-border-mix: {}%;",
        command_layout.shortcut_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-shortcut-border-mix: {}%;",
        command_layout.shortcut_border_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-shortcut-bg-mix: {}%;",
        command_layout.shortcut_bg_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-shortcut-bg-mix: {}%;",
        command_layout.shortcut_bg_mix_percent
    );
    css_writeln!(
        css,
        "  --ui-command-shortcut-padding-x: {}px;",
        command_layout.shortcut_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-shortcut-padding-x: {}px;",
        command_layout.shortcut_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-command-shortcut-padding-y: {}px;",
        command_layout.shortcut_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-shortcut-padding-y: {}px;",
        command_layout.shortcut_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-command-empty-padding-y: {}px;",
        command_layout.empty_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-empty-padding-y: {}px;",
        command_layout.empty_padding_block_px
    );
    css_writeln!(
        css,
        "  --ui-command-empty-padding-x: {}px;",
        command_layout.empty_padding_inline_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-command-empty-padding-x: {}px;",
        command_layout.empty_padding_inline_px
    );
    css_writeln!(css);

    css_writeln!(
        css,
        "  --ui-fallback-common-white: {};",
        common_colors.white
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-black: {};",
        common_colors.black
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-red-500: {};",
        common_colors.red.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-red-600: {};",
        common_colors.red.shade_600
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-yellow-500: {};",
        common_colors.yellow.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-green-500: {};",
        common_colors.green.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-green-600: {};",
        common_colors.green.shade_600
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-cyan-500: {};",
        common_colors.cyan.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-blue-500: {};",
        common_colors.blue.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-blue-600: {};",
        common_colors.blue.shade_600
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-purple-500: {};",
        common_colors.purple.shade_500
    );
    css_writeln!(
        css,
        "  --ui-fallback-common-zinc-500: {};",
        common_colors.zinc.shade_500
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
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-gap: {}px;",
        checkbox_group_layout.gap_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-required-marker-gap: {}px;",
        checkbox_group_layout.required_marker_gap_px
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-disabled-opacity: {};",
        checkbox_group_disabled_opacity
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-duration: {}ms;",
        checkbox_group_motion.duration_ms
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-easing: {};",
        checkbox_group_motion.easing
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-stiffness: {};",
        checkbox_group_motion.spring.stiffness
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-damping: {};",
        checkbox_group_motion.spring.damping
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-mass: {};",
        checkbox_group_motion.spring.mass
    );
    css_writeln!(
        css,
        "  --ui-fallback-checkbox-group-motion-precision: {};",
        checkbox_group_motion.spring.precision
    );
    css_writeln!(css, "  --ui-fallback-action-bar-max-width: 736px;");
    css_writeln!(
        css,
        "  --ui-fallback-action-bar-clear-underline-offset: 0.12em;"
    );
    css_writeln!(css, "  --ui-fallback-action-bar-translate-y-initial: 0px;");
    css_writeln!(css, "  --ui-fallback-action-bar-opacity-initial: 1;");
    css_writeln!(
        css,
        "  --ui-fallback-action-bar-emphasis-border-width: 2px;"
    );
    css_writeln!(
        css,
        "  --ui-fallback-action-bar-clear-label-custom-shadow: 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width)) color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent), var(--ui-shadow-md, var(--ui-fallback-shadow-md));"
    );
    css_writeln!(css, "}}");
    css
}
