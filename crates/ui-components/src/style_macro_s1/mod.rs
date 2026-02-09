pub use crate::css::push_components_css;

pub fn build_s1_layer_css() -> String {
    let mut css = String::new();
    push_components_css(&mut css);
    css
}
