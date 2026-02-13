pub const STYLE_MACRO_S1_LAYER: &str = "@layer ui";

pub fn has_s1_layer(css: &str) -> bool {
    super::logic::has_ui_layer(css)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_s1_layer_detects_ui_layer_marker() {
        assert!(has_s1_layer("@layer ui { .x { color: red; } }"));
        assert!(!has_s1_layer(".x { color: red; }"));
    }
}
