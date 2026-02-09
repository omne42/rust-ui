pub use crate::Theme;

pub fn snapshot_theme_css(theme: Theme) -> String {
    theme.to_css_variables()
}
