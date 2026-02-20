use super::*;

#[test]
fn semantic_overrides_emit_css_block() {
    let css = SemanticOverrides::new()
        .set(SemanticVariable::Primary, "oklch(66% 0.14 255)")
        .set(SemanticVariable::LayoutBackground, "oklch(98% 0 0)")
        .to_css_block(":root");

    assert!(css.contains(":root {"));
    assert!(css.contains("--ui-primary: oklch(66% 0.14 255);"));
    assert!(css.contains("--ui-layout-background: oklch(98% 0 0);"));
}

#[test]
fn semantic_overrides_last_write_wins() {
    let css = SemanticOverrides::new()
        .set(SemanticVariable::Primary, "oklch(60% 0.1 250)")
        .set(SemanticVariable::Primary, "oklch(64% 0.12 252)")
        .to_css_block(":root");

    assert!(!css.contains("--ui-primary: oklch(60% 0.1 250);"));
    assert!(css.contains("--ui-primary: oklch(64% 0.12 252);"));
}
