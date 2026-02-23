use super::*;

#[test]
fn tray_a11y_contract_maps_overlay_attrs_and_locale() {
    let contract = use_tray_a11y(TrayA11yOptions {
        title_id: " tray-title ".to_string(),
        description_id: Some(" tray-description ".to_string()),
        has_description: true,
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(
        contract.attrs.aria_labelledby.as_deref(),
        Some("tray-title")
    );
    assert_eq!(
        contract.attrs.aria_describedby.as_deref(),
        Some("tray-description")
    );
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(
        contract.state.description_state,
        TrayDescriptionA11yState::WithDescription
    );
    assert!(contract.state.has_description);
}

#[test]
fn tray_a11y_contract_omits_description_when_hidden() {
    let contract = use_tray_a11y(TrayA11yOptions {
        title_id: "tray-title".to_string(),
        description_id: Some("tray-description".to_string()),
        has_description: false,
        lang: None,
        dir: None,
    });

    assert_eq!(
        contract.attrs.aria_labelledby.as_deref(),
        Some("tray-title")
    );
    assert_eq!(contract.attrs.aria_describedby, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(
        contract.state.description_state,
        TrayDescriptionA11yState::TitleOnly
    );
    assert!(!contract.state.has_description);
}
