use super::*;

#[test]
fn use_direction_maps_locale_and_direction_attrs() {
    let contract = use_direction(DirectionOptions {
        direction: DirectionMode::Rtl,
        lang: Some("  ar-EG ".to_string()),
    });

    assert_eq!(contract.attrs.lang.as_deref(), Some("ar-EG"));
    assert_eq!(contract.attrs.dir, "rtl");
    assert_eq!(contract.attrs.data_direction, "rtl");
    assert_eq!(contract.state.direction, DirectionMode::Rtl);
}

#[test]
fn use_direction_defaults_to_direction_without_lang() {
    let contract = use_direction(DirectionOptions {
        direction: DirectionMode::Ltr,
        lang: None,
    });

    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, "ltr");
    assert_eq!(contract.attrs.data_direction, "ltr");
    assert_eq!(contract.state.direction, DirectionMode::Ltr);
}
