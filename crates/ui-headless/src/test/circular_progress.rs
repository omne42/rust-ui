use super::*;

#[test]
fn use_circular_progress_maps_locale_and_custom_source_attrs() {
    let contract = use_circular_progress(CircularProgressOptions {
        state: CircularProgressState {
            size_px: Some(24.0),
            thickness_px: Some(3.0),
            has_custom_size: true,
            has_custom_thickness: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            size_source_attr: "custom",
            thickness_source_attr: "custom",
            label_source_attr: "custom",
            class_source_attr: "custom",
        },
        aria_label: "Syncing mailbox".to_string(),
        lang: Some("  zh-CN  ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "progressbar");
    assert_eq!(contract.attrs.aria_label, "Syncing mailbox");
    assert_eq!(contract.attrs.aria_valuemin, "0");
    assert_eq!(contract.attrs.aria_valuemax, "100");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_state, "indeterminate");
    assert_eq!(contract.attrs.data_motion, "spin");
    assert_eq!(contract.attrs.data_size, Some("custom"));
    assert_eq!(contract.attrs.data_thickness, Some("custom"));
    assert_eq!(contract.attrs.data_size_source, "custom");
    assert_eq!(contract.attrs.data_thickness_source, "custom");
    assert_eq!(contract.attrs.data_label_source, "custom");
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(contract.attrs.data_custom_size, Some("true"));
    assert_eq!(contract.attrs.data_custom_thickness, Some("true"));
    assert_eq!(contract.attrs.data_custom_aria_label, Some("true"));
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.state.state, "indeterminate");
    assert_eq!(contract.state.motion, "spin");
    assert!(contract.state.has_custom_size);
    assert!(contract.state.has_custom_thickness);
    assert!(contract.state.has_custom_aria_label);
    assert!(contract.state.has_custom_class_name);
}

#[test]
fn use_circular_progress_keeps_default_source_attrs_without_locale() {
    let contract = use_circular_progress(CircularProgressOptions {
        state: CircularProgressState {
            size_px: None,
            thickness_px: None,
            has_custom_size: false,
            has_custom_thickness: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            size_source_attr: "default",
            thickness_source_attr: "default",
            label_source_attr: "default",
            class_source_attr: "default",
        },
        aria_label: "Loading".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.attrs.data_size, None);
    assert_eq!(contract.attrs.data_thickness, None);
    assert_eq!(contract.attrs.data_size_source, "default");
    assert_eq!(contract.attrs.data_thickness_source, "default");
    assert_eq!(contract.attrs.data_label_source, "default");
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.data_custom_size, None);
    assert_eq!(contract.attrs.data_custom_thickness, None);
    assert_eq!(contract.attrs.data_custom_aria_label, None);
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.state.size_source, "default");
    assert_eq!(contract.state.thickness_source, "default");
    assert_eq!(contract.state.label_source, "default");
    assert_eq!(contract.state.class_source, "default");
    assert!(!contract.state.has_custom_size);
    assert!(!contract.state.has_custom_thickness);
    assert!(!contract.state.has_custom_aria_label);
    assert!(!contract.state.has_custom_class_name);
}
