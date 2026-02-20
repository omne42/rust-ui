use super::*;

#[test]
fn demo_theme_cycle_is_stable() {
    assert_eq!(DemoTheme::Light.next(), DemoTheme::Dark);
    assert_eq!(DemoTheme::Dark.next(), DemoTheme::Oled);
    assert_eq!(DemoTheme::Oled.next(), DemoTheme::Light);
    assert_eq!(DemoTheme::Light.label(), "Light");
    assert_eq!(DemoTheme::Dark.label(), "Dark");
    assert_eq!(DemoTheme::Oled.label(), "OLED");
}
