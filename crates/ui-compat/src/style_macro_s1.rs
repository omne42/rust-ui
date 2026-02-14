pub const STYLE_MACRO_S1_LAYER: &str = "@layer ui";

pub fn build_layer_css(push_css: fn(&mut String)) -> String {
    let mut css = String::new();
    push_css(&mut css);
    css
}

pub fn has_ui_layer(css: &str) -> bool {
    css.contains(STYLE_MACRO_S1_LAYER)
}

pub fn has_s1_layer(css: &str) -> bool {
    has_ui_layer(css)
}

pub fn build_s1_layer_css_with(push_css: fn(&mut String)) -> String {
    build_layer_css(push_css)
}

pub fn build_s1_layer_css() -> String {
    let mut css = String::new();
    ui_components::push_components_css(&mut css);
    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_s1_layer_css_with_uses_callback_output() {
        fn push_css(out: &mut String) {
            out.push_str("@layer ui { .ui-alpha { color: var(--ui-fg); } }");
        }

        let css = build_s1_layer_css_with(push_css);
        assert!(css.contains(".ui-alpha"));
        assert!(has_s1_layer(&css));
    }
}
