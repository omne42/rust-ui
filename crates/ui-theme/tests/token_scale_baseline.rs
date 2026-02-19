use ui_theme::{
    Theme, ThemeColor, ThemeContext, ThemeScale, ThemeSystem, accordion_motion_tokens,
    button_layout_tokens, button_motion_tokens, default_accordion_motion_tokens,
    default_button_layout_tokens, default_button_motion_tokens, default_overlay_layout_tokens,
    default_slider_layout_tokens, default_slider_motion_tokens, default_swatch_motion_tokens,
    default_switch_motion_tokens, default_text_field_motion_tokens, default_textarea_motion_tokens,
    overlay_layout_tokens, slider_layout_tokens, slider_motion_tokens, swatch_motion_tokens,
    switch_motion_tokens, text_field_motion_tokens, textarea_motion_tokens,
};

#[test]
fn token_scale_baselines_are_regression_testable() {
    let medium = Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium);
    assert_eq!(medium.tokens.typography.font_size_100_px, 12);
    assert_eq!(medium.tokens.typography.font_size_150_px, 14);
    assert_eq!(medium.tokens.typography.font_size_200_px, 16);
    assert_eq!(medium.tokens.typography.line_height_100_px, 16);
    assert_eq!(medium.tokens.typography.line_height_150_px, 20);
    assert_eq!(medium.tokens.typography.line_height_200_px, 24);
    assert_eq!(medium.tokens.typography.body_font_size_px, 14);
    assert_eq!(medium.tokens.typography.body_line_height_px, 20);
    assert_eq!(medium.tokens.typography.heading_h1_font_size_px, 28);
    assert_eq!(medium.tokens.typography.heading_h6_font_size_px, 14);
    assert_eq!(medium.tokens.component_layout.component_height_100_px, 32);
    assert_eq!(
        medium
            .tokens
            .component_layout
            .separator_decorative_opacity_percent,
        72
    );
    assert_eq!(medium.tokens.overlay_layout.panel_min_width_px, 240);
    assert_eq!(medium.tokens.overlay_layout.viewport_inset_px, 16);
    assert_eq!(medium.tokens.overlay_layout.enter_offset_y_px, 6);
    assert_eq!(medium.tokens.overlay_layout.enter_scale, 0.98);
    assert_eq!(medium.tokens.slider_layout.max_width_px, 352);
    assert_eq!(medium.tokens.slider_layout.thumb_border_width_px, 2);
    assert_eq!(medium.tokens.slider_layout.focus_ring_width_px, 2);
    assert_eq!(medium.tokens.underlay_motion.transition_duration_ms, 220);
    assert_eq!(medium.tokens.underlay_motion.visibility_duration_ms, 220);
    assert_eq!(medium.tokens.underlay_motion.backdrop_blur_px, 1);
    assert_eq!(medium.tokens.underlay_motion.scrim_alpha_percent, 56);
    assert_eq!(medium.tokens.icons.size_100_px, 20);
    assert_eq!(medium.tokens.icons.size_200_px, 22);

    let large = Theme::baseline_two(ThemeColor::Light, ThemeScale::Large);
    assert_eq!(large.tokens.typography.font_size_100_px, 14);
    assert_eq!(large.tokens.typography.font_size_150_px, 16);
    assert_eq!(large.tokens.typography.font_size_200_px, 19);
    assert_eq!(large.tokens.typography.line_height_100_px, 20);
    assert_eq!(large.tokens.typography.line_height_150_px, 24);
    assert_eq!(large.tokens.typography.line_height_200_px, 28);
    assert_eq!(large.tokens.typography.body_font_size_px, 16);
    assert_eq!(large.tokens.typography.body_line_height_px, 24);
    assert_eq!(large.tokens.typography.heading_h1_font_size_px, 32);
    assert_eq!(large.tokens.typography.heading_h6_font_size_px, 16);
    assert_eq!(large.tokens.component_layout.component_height_100_px, 40);
    assert_eq!(
        large
            .tokens
            .component_layout
            .separator_decorative_opacity_percent,
        72
    );
    assert_eq!(large.tokens.overlay_layout.panel_min_width_px, 280);
    assert_eq!(large.tokens.overlay_layout.viewport_inset_px, 20);
    assert_eq!(large.tokens.overlay_layout.enter_offset_y_px, 8);
    assert_eq!(large.tokens.overlay_layout.enter_scale, 0.98);
    assert_eq!(large.tokens.slider_layout.max_width_px, 400);
    assert_eq!(large.tokens.slider_layout.thumb_border_width_px, 2);
    assert_eq!(large.tokens.slider_layout.focus_ring_width_px, 2);
    assert_eq!(large.tokens.underlay_motion.transition_duration_ms, 240);
    assert_eq!(large.tokens.underlay_motion.visibility_duration_ms, 240);
    assert_eq!(large.tokens.underlay_motion.backdrop_blur_px, 1);
    assert_eq!(large.tokens.underlay_motion.scrim_alpha_percent, 56);
    assert_eq!(large.tokens.icons.size_100_px, 24);
    assert_eq!(large.tokens.icons.size_200_px, 28);
}

#[test]
fn css_variables_emit_theme_axes() {
    let css = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Large).to_css_variables();
    assert!(css.contains("--ui-system: baseline-two;"));
    assert!(css.contains("--ui-color: dark;"));
    assert!(css.contains("--ui-scale: large;"));
    assert!(css.contains("--ui-line-height-100: 20px;"));
    assert!(css.contains("--ui-line-height-150: 24px;"));
    assert!(css.contains("--ui-line-height-200: 28px;"));
    assert!(css.contains("--ui-body-font-size: 16px;"));
    assert!(css.contains("--ui-heading-h1-font-size: 32px;"));
    assert!(css.contains("--ui-heading-h6-font-size: 16px;"));
    assert!(css.contains("--ui-font-size-sm: 14px;"));
    assert!(css.contains("--ui-font-size-lg: 19px;"));
    assert!(css.contains("--ui-space-3xs: 2px;"));
    assert!(css.contains("--ui-space-2xs: 4px;"));
    assert!(css.contains("--ui-slider-max-width:"));
    assert!(css.contains("--ui-slider-thumb-border-width:"));
    assert!(css.contains("--ui-slider-focus-ring-width:"));
    assert!(css.contains("--ui-underlay-transition-duration:"));
    assert!(css.contains("--ui-underlay-transition-easing:"));
    assert!(css.contains("--ui-separator-decorative-opacity:"));
}

#[test]
fn palette_values_follow_baseline_light_dark_pairs() {
    let light = Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium);
    assert_eq!(light.tokens.palette.gray_50, "oklch(98.51% 0 0)");
    assert_eq!(
        light.tokens.palette.accent_600,
        "oklch(49.20% 0.1800 257.73)"
    );

    let dark = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium);
    assert_eq!(dark.tokens.palette.gray_50, "oklch(21.03% 0.0059 285.89)");
    assert_eq!(
        dark.tokens.palette.accent_600,
        "oklch(72.66% 0.1349 253.30)"
    );
}

#[test]
fn semantic_scales_follow_light_dark_inversion() {
    let light = Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium);
    let dark = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium);

    assert_eq!(
        light.tokens.semantic_scales.primary.shade_50,
        dark.tokens.semantic_scales.primary.shade_900
    );
    assert_eq!(
        light.tokens.semantic_scales.default.shade_900,
        dark.tokens.semantic_scales.default.shade_50
    );
}

#[test]
fn common_colors_are_theme_invariant() {
    let light = Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium);
    let dark = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium);

    assert_eq!(
        light.tokens.common_colors.blue.shade_500,
        dark.tokens.common_colors.blue.shade_500
    );
    assert_eq!(light.tokens.common_colors.white, "oklch(100% 0 0)");
    assert_eq!(light.tokens.common_colors.black, "oklch(0% 0 0)");
}

#[test]
fn accordion_motion_tokens_follow_scale_baseline() {
    let medium = accordion_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = accordion_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.spring.stiffness, 260.0);
    assert_eq!(medium.spring.damping, 18.0);
    assert_eq!(medium.panel_offset_y_px, 4.0);
    assert_eq!(large.panel_offset_y_px, 6.0);
    assert_eq!(default_accordion_motion_tokens(), medium);
}

#[test]
fn button_motion_tokens_follow_scale_baseline() {
    let medium = button_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = button_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.spring.stiffness, 260.0);
    assert_eq!(medium.spring.damping, 16.0);
    assert_eq!(medium.hover_scale, 1.05);
    assert_eq!(medium.tap_scale, 0.95);
    assert_eq!(large.hover_scale, 1.05);
    assert_eq!(large.tap_scale, 0.95);
    assert_eq!(default_button_motion_tokens(), medium);
}

#[test]
fn swatch_motion_tokens_follow_scale_baseline() {
    let medium = swatch_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = swatch_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.spring.stiffness, 280.0);
    assert_eq!(medium.spring.damping, 20.0);
    assert_eq!(medium.spring.mass, 1.0);
    assert_eq!(medium.selected_scale, 1.06);
    assert_eq!(medium.selected_ring_opacity, 1.0);
    assert_eq!(large, medium);
    assert_eq!(default_swatch_motion_tokens(), medium);
}

#[test]
fn button_layout_tokens_follow_theme_token_baseline() {
    let medium = button_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = button_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.min_width_px, 80);
    assert_eq!(medium.font_size_px, 14);
    assert_eq!(medium.spinner_size_px, 16);
    assert_eq!(medium.spinner_border_px, 2);
    assert_eq!(medium.spinner_duration_ms, 800);
    assert_eq!(medium.focus_outline_width_px, 3);
    assert_eq!(medium.focus_outline_offset_px, 2);
    assert_eq!(medium.radius_full_px, 9999);
    assert_eq!(medium.m.height_px, 32);
    assert_eq!(medium.m.min_width_px, 80);
    assert_eq!(medium.m.padding_inline_px, 12);
    assert_eq!(medium.m.font_size_px, 14);
    assert_eq!(medium.m.line_height_px, 20);
    assert_eq!(medium.m.gap_px, 6);
    assert_eq!(medium.m.icon_size_px, 32);

    assert_eq!(large, medium);
    assert_eq!(default_button_layout_tokens(), medium);
}

#[test]
fn switch_motion_tokens_follow_scale_baseline() {
    let medium = switch_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = switch_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.spring.stiffness, 260.0);
    assert_eq!(medium.spring.damping, 16.0);
    assert_eq!(medium.pressed_width_default_px, 19.0);
    assert_eq!(medium.pressed_width_min_px, 16.0);
    assert_eq!(medium.pressed_width_max_px, 64.0);
    assert_eq!(large, medium);
    assert_eq!(default_switch_motion_tokens(), medium);
}

#[test]
fn slider_motion_tokens_follow_scale_baseline() {
    let medium = slider_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = slider_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.spring.stiffness, 340.0);
    assert_eq!(medium.spring.damping, 28.0);
    assert_eq!(medium.spring.mass, 0.9);
    assert_eq!(medium.spring.precision, 0.001);
    assert_eq!(large, medium);
    assert_eq!(default_slider_motion_tokens(), medium);
}

#[test]
fn slider_layout_tokens_follow_scale_baseline() {
    let medium = slider_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = slider_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.max_width_px, 352);
    assert_eq!(medium.thumb_border_width_px, 2);
    assert_eq!(medium.focus_ring_width_px, 2);
    assert_eq!(large.max_width_px, 400);
    assert_eq!(large.thumb_border_width_px, 2);
    assert_eq!(large.focus_ring_width_px, 2);
    assert_eq!(default_slider_layout_tokens(), medium);
}

#[test]
fn textarea_motion_tokens_follow_scale_baseline() {
    let medium = textarea_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = textarea_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.duration_ms, 180);
    assert_eq!(large.duration_ms, 200);
    assert_eq!(default_textarea_motion_tokens(), medium);
}

#[test]
fn text_field_motion_tokens_follow_scale_baseline() {
    let medium = text_field_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = text_field_motion_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.duration_ms, 180);
    assert_eq!(medium.easing, "cubic-bezier(0.2, 0, 0, 1)");
    assert_eq!(large.duration_ms, 200);
    assert_eq!(large.easing, "cubic-bezier(0.2, 0, 0, 1)");
    assert_eq!(default_text_field_motion_tokens(), medium);
}

#[test]
fn overlay_layout_tokens_follow_scale_baseline() {
    let medium = overlay_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Medium,
    });
    let large = overlay_layout_tokens(ThemeContext {
        system: ThemeSystem::BaselineTwo,
        color: ThemeColor::Light,
        scale: ThemeScale::Large,
    });

    assert_eq!(medium.z_index, 1000);
    assert_eq!(medium.panel_min_width_px, 240);
    assert_eq!(medium.viewport_inset_px, 16);
    assert_eq!(medium.enter_offset_y_px, 6);
    assert_eq!(medium.enter_scale, 0.98);
    assert_eq!(large.z_index, 1000);
    assert_eq!(large.panel_min_width_px, 280);
    assert_eq!(large.viewport_inset_px, 20);
    assert_eq!(large.enter_offset_y_px, 8);
    assert_eq!(large.enter_scale, 0.98);
    assert_eq!(default_overlay_layout_tokens(), medium);
}

#[test]
fn oled_uses_true_black_background() {
    let oled = Theme::baseline_two(ThemeColor::Oled, ThemeScale::Medium);
    assert_eq!(oled.tokens.semantic_colors.bg, "oklch(0% 0 0)");
    assert_eq!(oled.tokens.layout_semantic.content_1, "oklch(0% 0 0)");
}

#[test]
fn semantic_roles_align_with_upstream_defaults() {
    let light = Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium);
    assert_eq!(light.tokens.layout_semantic.background, "oklch(97.02% 0 0)");
    assert_eq!(
        light.tokens.layout_semantic.foreground,
        "oklch(21.03% 0.0059 285.89)"
    );
    assert_eq!(
        light.tokens.semantic_roles.default,
        "oklch(94% 0.001 286.375)"
    );
    assert_eq!(
        light.tokens.semantic_roles.primary,
        "oklch(56.71% 0.2095 257.94)"
    );
    assert_eq!(
        light.tokens.semantic_roles.secondary,
        "oklch(48.78% 0.2254 300.51)"
    );
    assert_eq!(
        light.tokens.semantic_roles.success,
        "oklch(73.29% 0.1935 150.81)"
    );
    assert_eq!(
        light.tokens.semantic_roles.warning,
        "oklch(78.19% 0.1585 72.33)"
    );
    assert_eq!(
        light.tokens.semantic_roles.danger,
        "oklch(65.32% 0.2328 25.74)"
    );
    assert_eq!(
        light.tokens.semantic_colors.border,
        "oklch(90% 0.004 286.32)"
    );

    let dark = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium);
    assert_eq!(
        dark.tokens.layout_semantic.background,
        "oklch(12% 0.005 285.823)"
    );
    assert_eq!(dark.tokens.layout_semantic.foreground, "oklch(99.11% 0 0)");
    assert_eq!(
        dark.tokens.semantic_roles.default,
        "oklch(27.4% 0.006 286.033)"
    );
    assert_eq!(
        dark.tokens.semantic_roles.primary,
        "oklch(56.71% 0.2095 257.94)"
    );
    assert_eq!(
        dark.tokens.semantic_roles.secondary,
        "oklch(57.67% 0.1916 304.03)"
    );
    assert_eq!(
        dark.tokens.semantic_roles.warning,
        "oklch(82.03% 0.1388 76.34)"
    );
    assert_eq!(dark.tokens.semantic_roles.danger, "oklch(57% 0.1967 24.63)");
    assert_eq!(
        dark.tokens.semantic_colors.border,
        "oklch(28% 0.006 286.033)"
    );
}
