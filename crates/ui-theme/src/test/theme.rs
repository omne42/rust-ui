use super::*;

#[test]
fn to_css_variables_contains_expected_keys() {
    let theme = Theme::light();
    let css = theme.to_css_variables();
    assert!(css.contains("--ui-system:"));
    assert!(css.contains("--ui-color:"));
    assert!(css.contains("--ui-scale:"));
    assert!(css.contains("--ui-palette-gray-50:"));
    assert!(css.contains("--ui-common-white:"));
    assert!(css.contains("--ui-common-blue-500:"));
    assert!(css.contains("--ui-primary-500:"));
    assert!(css.contains("--ui-default-900:"));
    assert!(css.contains("--ui-layout-background:"));
    assert!(css.contains("--ui-semantic-fg:"));
    assert!(css.contains("--ui-primary:"));
    assert!(css.contains("--ui-primary-foreground:"));
    assert!(css.contains("--ui-alias-text-default:"));
    assert!(css.contains("--ui-component-control-bg:"));
    assert!(css.contains("--ui-icon-size-100:"));
    assert!(css.contains("color-scheme:"));
    assert!(css.contains("--ui-fg:"));
    assert!(css.contains("--ui-fg-muted:"));
    assert!(css.contains("--ui-bg-muted:"));
    assert!(css.contains("--ui-accent-fg:"));
    assert!(css.contains("--ui-danger:"));
    assert!(css.contains("--ui-radius-md:"));
    assert!(css.contains("--ui-space-3xs:"));
    assert!(css.contains("--ui-space-2xs:"));
    assert!(css.contains("--ui-shadow-md:"));
    assert!(css.contains("--ui-font-size-200:"));
    assert!(css.contains("--ui-font-size-150:"));
    assert!(css.contains("--ui-font-size-100:"));
    assert!(css.contains("--ui-line-height-150:"));
    assert!(css.contains("--ui-line-height-200:"));
    assert!(css.contains("--ui-body-font-size:"));
    assert!(css.contains("--ui-body-line-height:"));
    assert!(css.contains("--ui-heading-h1-font-size:"));
    assert!(css.contains("--ui-heading-h6-font-size:"));
    assert!(css.contains("--ui-font-size-sm:"));
    assert!(css.contains("--ui-font-size-lg:"));
    assert!(css.contains("--ui-component-height-100:"));
    assert!(css.contains("--ui-separator-decorative-opacity:"));
    assert!(css.contains("--ui-overlay-panel-min-width:"));
    assert!(css.contains("--ui-overlay-z-index:"));
    assert!(css.contains("--ui-slider-max-width:"));
    assert!(css.contains("--ui-slider-thumb-border-width:"));
    assert!(css.contains("--ui-slider-focus-ring-width:"));
    assert!(css.contains("--ui-underlay-transition-duration:"));
    assert!(css.contains("--ui-text-field-motion-duration:"));
    assert!(css.contains("--ui-text-field-motion-easing:"));
    assert!(css.contains("--ui-button-min-width:"));
    assert!(css.contains("--ui-button-spinner-size:"));
    assert!(css.contains("--ui-button-size-m-height:"));
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

#[test]
fn radius_scale_matches_button_contract() {
    let css = Theme::light().to_css_variables();
    assert!(css.contains("--ui-radius-sm: 4px;"));
    assert!(css.contains("--ui-radius-md: 8px;"));
    assert!(css.contains("--ui-radius-lg: 12px;"));
}
