use super::*;
use ui_state_primitives::legend::{LegendStateInput, LegendTone, resolve_state};

#[test]
fn use_legend_maps_locale_and_state_attrs() {
    let state = resolve_state(LegendStateInput {
        tone: LegendTone::Strong,
        is_required: true,
        is_disabled: true,
        has_custom_text: true,
        has_custom_indicator: false,
        has_custom_class_name: true,
    });

    let contract = use_legend(LegendOptions {
        state,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_tone, "strong");
    assert_eq!(contract.attrs.data_state, "required");
    assert_eq!(contract.attrs.data_required, Some("true"));
    assert_eq!(contract.attrs.data_disabled, Some("true"));
    assert_eq!(contract.attrs.data_text_source, "custom");
    assert_eq!(contract.attrs.data_indicator_source, "default");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_legend_handles_optional_state() {
    let state = resolve_state(LegendStateInput {
        tone: LegendTone::Default,
        is_required: false,
        is_disabled: false,
        has_custom_text: false,
        has_custom_indicator: false,
        has_custom_class_name: false,
    });

    let contract = use_legend(LegendOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.attrs.data_tone, "default");
    assert_eq!(contract.attrs.data_state, "optional");
    assert_eq!(contract.attrs.data_required, None);
    assert_eq!(contract.attrs.data_disabled, None);
    assert_eq!(contract.attrs.data_text_source, "default");
    assert_eq!(contract.attrs.data_indicator_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
}
