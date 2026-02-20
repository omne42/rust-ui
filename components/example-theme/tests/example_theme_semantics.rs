use ui_example_theme::{Theme, example_theme};

#[test]
fn example_theme_returns_light_theme() {
    let theme = example_theme();
    assert_eq!(theme.ctx, Theme::light().ctx);
}
