use super::*;
use ui_state_primitives::aspect_ratio::{
    AspectRatioPreset, AspectRatioRadius, AspectRatioStateInput, resolve_state,
};

#[test]
fn use_aspect_ratio_maps_region_locale_and_state_markers() {
    let state = resolve_state(AspectRatioStateInput {
        ratio: AspectRatioPreset::UltraWide,
        radius: AspectRatioRadius::Lg,
        bordered: true,
        fill: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_aspect_ratio(AspectRatioOptions {
        state,
        aria_label: " Trailer preview ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "region");
    assert_eq!(contract.attrs.aria_label, " Trailer preview ");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_ratio, "ultra-wide");
    assert_eq!(contract.attrs.data_radius, "lg");
    assert_eq!(contract.attrs.data_bordered, Some("true"));
    assert_eq!(contract.attrs.data_fill, Some("true"));
    assert_eq!(contract.attrs.data_state, "media");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_aspect_ratio_omits_optional_markers_for_plain_defaults() {
    let state = resolve_state(AspectRatioStateInput {
        ratio: AspectRatioPreset::Video,
        radius: AspectRatioRadius::None,
        bordered: false,
        fill: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_aspect_ratio(AspectRatioOptions {
        state,
        aria_label: "Aspect ratio frame".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.data_ratio, "video");
    assert_eq!(contract.attrs.data_radius, "none");
    assert_eq!(contract.attrs.data_bordered, None);
    assert_eq!(contract.attrs.data_fill, None);
    assert_eq!(contract.attrs.data_state, "plain");
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
}
