pub use ui_components::Theme;

pub fn snapshot_theme_css(theme: Theme) -> String {
    theme.to_css_variables()
}
