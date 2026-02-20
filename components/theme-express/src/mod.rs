pub use ui_theme::Theme;
use ui_theme::{ThemeColor, ThemeScale};

pub fn express_theme() -> Theme {
    Theme::express(ThemeColor::Light, ThemeScale::Medium)
}
