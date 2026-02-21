use super::*;

#[test]
fn logic_resolve_state_delegates_to_ui_state_primitives() {
    let input = DateInputGroupStateInput {
        variant: DateInputGroupVariant::Secondary,
        width: DateInputGroupWidth::Full,
        status: DateInputGroupStatus::Invalid,
        segmented: true,
        has_prefix: true,
        has_suffix: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    };

    assert_eq!(
        resolve_state(input),
        ui_state_primitives::date_input_group::resolve_state(input)
    );
}

#[test]
fn resolve_width_and_status_use_enum_constraints() {
    assert_eq!(
        resolve_width(false),
        ui_state_primitives::date_input_group::resolve_width(false)
    );
    assert_eq!(
        resolve_width(true),
        ui_state_primitives::date_input_group::resolve_width(true)
    );
    assert_eq!(
        resolve_status(false, false),
        ui_state_primitives::date_input_group::resolve_status(false, false)
    );
    assert_eq!(
        resolve_status(false, true),
        ui_state_primitives::date_input_group::resolve_status(false, true)
    );
    assert_eq!(
        resolve_status(true, false),
        ui_state_primitives::date_input_group::resolve_status(true, false)
    );
    assert_eq!(
        resolve_status(true, true),
        ui_state_primitives::date_input_group::resolve_status(true, true)
    );
}

#[test]
fn derive_state_centralizes_state_normalization_input_mapping() {
    let input = DateInputGroupStateDeriveInput {
        variant: DateInputGroupVariant::Secondary,
        width: DateInputGroupWidth::Full,
        status: DateInputGroupStatus::Invalid,
        is_segmented: true,
        has_prefix: true,
        has_suffix: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    };

    assert_eq!(
        derive_state(input),
        resolve_state(DateInputGroupStateInput {
            variant: DateInputGroupVariant::Secondary,
            width: DateInputGroupWidth::Full,
            status: DateInputGroupStatus::Invalid,
            segmented: true,
            has_prefix: true,
            has_suffix: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        })
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(DateInputGroupStateInput {
        variant: DateInputGroupVariant::Primary,
        width: DateInputGroupWidth::Fit,
        status: DateInputGroupStatus::Disabled,
        segmented: true,
        has_prefix: true,
        has_suffix: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-date-input-group".to_string()), state);

    for token in [
        "ui-date-input-group",
        "ui-date-input-group--variant-primary",
        "ui-date-input-group--fit-width",
        "ui-date-input-group--disabled",
        "ui-date-input-group--segmented",
        "ui-date-input-group--has-prefix",
        "ui-date-input-group--has-suffix",
        "ui-date-input-group--custom-class",
        "docs-date-input-group",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_motion_source_attrs_marks_default_motion() {
    let (motion_source_attr, custom_motion_attr) =
        resolve_motion_source_attrs(crate::DateInputGroupMotion::default());

    assert_eq!(motion_source_attr, "default");
    assert_eq!(custom_motion_attr, None);
}

#[test]
fn resolve_motion_source_attrs_marks_custom_motion() {
    let motion = crate::DateInputGroupMotion {
        enter_scale: 1.02,
        ..crate::DateInputGroupMotion::default()
    };
    let (motion_source_attr, custom_motion_attr) = resolve_motion_source_attrs(motion);

    assert_eq!(motion_source_attr, "custom");
    assert_eq!(custom_motion_attr, Some("true"));
}
