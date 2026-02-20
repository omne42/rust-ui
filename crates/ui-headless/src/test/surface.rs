use super::*;
use ui_state_primitives::surface::{
    SurfaceElevation, SurfaceStateInput, SurfaceTone, resolve_state,
};

#[test]
fn use_surface_maps_region_locale_and_state_markers() {
    let state = resolve_state(SurfaceStateInput {
        tone: SurfaceTone::Strong,
        elevation: SurfaceElevation::Floating,
        bordered: true,
        padded: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_surface(SurfaceOptions {
        state,
        aria_label: " Deployment summary ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "region");
    assert_eq!(contract.attrs.aria_label, " Deployment summary ");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_tone, "strong");
    assert_eq!(contract.attrs.data_elevation, "floating");
    assert_eq!(contract.attrs.data_state, "bordered");
    assert_eq!(contract.attrs.data_bordered, Some("true"));
    assert_eq!(contract.attrs.data_padded, None);
    assert_eq!(contract.attrs.data_plain, None);
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(contract.state.state, "bordered");
}

#[test]
fn use_surface_exposes_plain_defaults_without_optional_markers() {
    let state = resolve_state(SurfaceStateInput {
        tone: SurfaceTone::Default,
        elevation: SurfaceElevation::Raised,
        bordered: false,
        padded: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_surface(SurfaceOptions {
        state,
        aria_label: "Surface".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.data_tone, "default");
    assert_eq!(contract.attrs.data_elevation, "raised");
    assert_eq!(contract.attrs.data_state, "plain");
    assert_eq!(contract.attrs.data_bordered, None);
    assert_eq!(contract.attrs.data_padded, None);
    assert_eq!(contract.attrs.data_plain, Some("true"));
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
}
