pub fn build_layer_css(push_css: fn(&mut String)) -> String {
    let mut css = String::new();
    push_css(&mut css);
    css
}

pub fn has_ui_layer(css: &str) -> bool {
    css.contains("@layer ui")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_collects_css_from_callback() {
        fn push_css(out: &mut String) {
            out.push_str("@layer ui { .ui-test { color: var(--ui-fg); } }");
        }

        let css = build_layer_css(push_css);
        assert!(css.contains(".ui-test"));
        assert!(has_ui_layer(&css));
    }
}
