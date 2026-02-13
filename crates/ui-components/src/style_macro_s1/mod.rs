mod logic;
mod styles;
mod view;

pub use crate::css::push_components_css;
pub use logic::{build_layer_css, has_ui_layer};
pub use styles::{STYLE_MACRO_S1_LAYER, has_s1_layer};
pub use view::build_s1_layer_css_with;

pub fn build_s1_layer_css() -> String {
    let mut css = String::new();
    push_components_css(&mut css);
    css
}
