use super::*;

#[test]
fn axis_and_size_mappings_are_stable() {
    assert_eq!(
        SpacerAxis::Vertical.class_name(),
        "ui-spacer--axis-vertical"
    );
    assert_eq!(
        SpacerAxis::Horizontal.class_name(),
        "ui-spacer--axis-horizontal"
    );
    assert_eq!(SpacerAxis::Vertical.as_attr(), "vertical");
    assert_eq!(SpacerAxis::Horizontal.as_attr(), "horizontal");

    assert_eq!(SpacerSize::Xs.class_name(), "ui-spacer--size-xs");
    assert_eq!(SpacerSize::Sm.class_name(), "ui-spacer--size-sm");
    assert_eq!(SpacerSize::Md.class_name(), "ui-spacer--size-md");
    assert_eq!(SpacerSize::Lg.class_name(), "ui-spacer--size-lg");
    assert_eq!(SpacerSize::Xl.class_name(), "ui-spacer--size-xl");

    assert_eq!(SpacerSize::Xs.as_attr(), "xs");
    assert_eq!(SpacerSize::Sm.as_attr(), "sm");
    assert_eq!(SpacerSize::Md.as_attr(), "md");
    assert_eq!(SpacerSize::Lg.as_attr(), "lg");
    assert_eq!(SpacerSize::Xl.as_attr(), "xl");
}

#[test]
fn resolve_state_tracks_axis_and_size_flags() {
    let state = resolve_state(SpacerStateInput {
        axis: SpacerAxis::Horizontal,
        size: SpacerSize::Lg,
        has_custom_class_name: true,
    });

    assert_eq!(state.axis, SpacerAxis::Horizontal);
    assert_eq!(state.axis_class, "ui-spacer--axis-horizontal");
    assert_eq!(state.axis_attr, "horizontal");
    assert!(state.is_horizontal);
    assert!(!state.is_vertical);

    assert_eq!(state.size, SpacerSize::Lg);
    assert_eq!(state.size_class, "ui-spacer--size-lg");
    assert_eq!(state.size_attr, "lg");

    assert!(state.has_custom_class_name);
}
