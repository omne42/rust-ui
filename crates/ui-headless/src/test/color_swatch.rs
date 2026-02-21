use super::*;

#[test]
fn color_swatch_a11y_contract_maps_image_role_and_locale() {
    let contract = use_color_swatch_a11y(ColorSwatchA11yOptions {
        is_decorative: false,
        aria_label: " Brand blue ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, Some("img"));
    assert_eq!(contract.attrs.aria_hidden, None);
    assert_eq!(contract.attrs.aria_label.as_deref(), Some(" Brand blue "));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert!(contract.state.exposes_image_role);
    assert!(contract.state.has_label);
}

#[test]
fn color_swatch_a11y_contract_hides_decorative_swatches_from_tree() {
    let contract = use_color_swatch_a11y(ColorSwatchA11yOptions {
        is_decorative: true,
        aria_label: "Ignored".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.role, None);
    assert_eq!(contract.attrs.aria_label, None);
    assert_eq!(contract.attrs.aria_hidden, Some("true"));
    assert!(contract.state.is_decorative);
    assert!(!contract.state.exposes_image_role);
    assert!(!contract.state.has_label);
}
