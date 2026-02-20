use super::*;

#[test]
fn size_mappings_are_stable() {
    assert_eq!(SpinnerSize::Sm.class_name(), "ui-spinner--size-sm");
    assert_eq!(SpinnerSize::Md.class_name(), "ui-spinner--size-md");
    assert_eq!(SpinnerSize::Lg.class_name(), "ui-spinner--size-lg");

    assert_eq!(SpinnerSize::Sm.as_str(), "sm");
    assert_eq!(SpinnerSize::Md.as_str(), "md");
    assert_eq!(SpinnerSize::Lg.as_str(), "lg");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  custom-spinner  ".to_string())),
        Some("custom-spinner".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_detects_custom_source() {
    let default_aria_label = "Loading";
    assert_eq!(
        resolve_aria_label(None, default_aria_label),
        (default_aria_label.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  Loading  ".to_string()), default_aria_label),
        (default_aria_label.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some(" Fetching activity ".to_string()), default_aria_label),
        ("Fetching activity".to_string(), true)
    );
}

#[test]
fn resolve_state_tracks_source_contracts() {
    let state = resolve_state(SpinnerStateInput {
        size: SpinnerSize::Lg,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.size, SpinnerSize::Lg);
    assert_eq!(state.size_class, "ui-spinner--size-lg");
    assert_eq!(state.size_attr, "lg");
    assert_eq!(state.label_source_class, "ui-spinner--label-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.has_custom_aria_label);
    assert!(state.has_custom_class_name);
}

#[test]
fn marker_values_are_closed_sets() {
    let allowed_size = ["sm", "md", "lg"];
    let allowed_source = ["default", "custom"];

    for size in [SpinnerSize::Sm, SpinnerSize::Md, SpinnerSize::Lg] {
        for has_custom_aria_label in [false, true] {
            for has_custom_class_name in [false, true] {
                let state = resolve_state(SpinnerStateInput {
                    size,
                    has_custom_aria_label,
                    has_custom_class_name,
                });

                assert!(
                    allowed_size.contains(&state.size_attr),
                    "size marker must stay enumerable"
                );
                assert!(
                    allowed_source.contains(&state.label_source_attr),
                    "label source marker must stay enumerable"
                );
                assert!(
                    allowed_source.contains(&state.class_source_attr),
                    "class source marker must stay enumerable"
                );
            }
        }
    }
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-spinner-custom".to_string()),
        resolve_state(SpinnerStateInput {
            size: SpinnerSize::Sm,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-spinner",
        "ui-spinner--size-sm",
        "ui-spinner--label-custom",
        "ui-spinner--custom-class",
        "docs-spinner-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
