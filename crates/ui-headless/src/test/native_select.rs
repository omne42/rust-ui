use super::*;
use ui_state_primitives::native_select::{
    NativeSelectOption, NativeSelectStateInput, normalize_options, resolve_options, resolve_state,
};

#[test]
fn use_native_select_maps_locale_and_semantic_attrs() {
    let options = resolve_options(
        "native-select",
        &normalize_options(vec![NativeSelectOption::new("system", "System")]),
    );
    let state = resolve_state(NativeSelectStateInput {
        disabled: false,
        invalid: true,
        required: true,
        has_placeholder: true,
        selected_index: Some(0),
        options: &options,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    let contract = use_native_select(NativeSelectOptions {
        state,
        aria_label: "Select mode".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.aria_label, "Select mode");
    assert_eq!(contract.attrs.aria_invalid, Some("true"));
    assert!(contract.attrs.disabled);
    assert!(contract.attrs.required);
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_slot, "native-select-control");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert!(contract.state.is_disabled);
    assert!(contract.state.is_invalid);
    assert!(contract.state.is_required);
    assert_eq!(contract.state.visual_state, "disabled");
    assert_eq!(contract.state.aria_source, "custom");
}

#[test]
fn resolve_native_select_change_index_uses_primitive_lookup_and_empty_guard() {
    let options = resolve_options(
        "native-select",
        &normalize_options(vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual").disabled(true),
        ]),
    );

    assert_eq!(resolve_native_select_change_index("", &options), None);
    assert_eq!(
        resolve_native_select_change_index("  system  ", &options),
        Some(0)
    );
    assert_eq!(resolve_native_select_change_index("manual", &options), None);
    assert_eq!(
        resolve_native_select_change_index("missing", &options),
        None
    );
}
