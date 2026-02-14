use ui_theme::{Theme, ThemeColor, ThemeScale};

#[test]
fn token_scale_baselines_are_regression_testable() {
    let medium = Theme::spectrum_two(ThemeColor::Light, ThemeScale::Medium);
    assert_eq!(medium.tokens.typography.font_size_200_px, 16);
    assert_eq!(medium.tokens.component_layout.component_height_100_px, 32);

    let large = Theme::spectrum_two(ThemeColor::Light, ThemeScale::Large);
    assert_eq!(large.tokens.typography.font_size_200_px, 19);
    assert_eq!(large.tokens.component_layout.component_height_100_px, 40);
}
