pub fn build_s1_layer_css_with(push_css: fn(&mut String)) -> String {
    super::logic::build_layer_css(push_css)
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
    }
}
