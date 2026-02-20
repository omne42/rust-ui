use super::*;
use ui_state_primitives::status_light::{
    StatusLightRole, StatusLightStateInput, StatusLightVariant, resolve_state,
};

#[test]
fn use_status_light_maps_live_region_and_locale_attrs() {
    let state = resolve_state(StatusLightStateInput {
        variant: StatusLightVariant::Danger,
        role: Some(StatusLightRole::Status),
        has_custom_class_name: true,
    });

    let contract = use_status_light(StatusLightOptions {
        state,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, Some("status"));
    assert_eq!(contract.attrs.aria_live, Some("polite"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_variant, "danger");
    assert_eq!(contract.attrs.data_state, "live");
    assert_eq!(contract.attrs.data_live, Some("true"));
    assert_eq!(contract.attrs.data_static, None);
    assert_eq!(contract.attrs.data_role, Some("status"));
    assert_eq!(contract.attrs.data_role_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(contract.state.state, "live");
    assert!(contract.state.is_live);
}

#[test]
fn use_status_light_keeps_static_state_without_live_region_attrs() {
    let state = resolve_state(StatusLightStateInput {
        variant: StatusLightVariant::Default,
        role: None,
        has_custom_class_name: false,
    });

    let contract = use_status_light(StatusLightOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.role, None);
    assert_eq!(contract.attrs.aria_live, None);
    assert_eq!(contract.attrs.data_variant, "default");
    assert_eq!(contract.attrs.data_state, "static");
    assert_eq!(contract.attrs.data_live, None);
    assert_eq!(contract.attrs.data_static, Some("true"));
    assert_eq!(contract.attrs.data_role, None);
    assert_eq!(contract.attrs.data_role_source, "none");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.state.state, "static");
    assert!(!contract.state.is_live);
}
