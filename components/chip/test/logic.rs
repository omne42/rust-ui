use super::*;

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ChipStateInput {
            variant: ChipVariant::Accent,
            size: ChipSize::Sm,
            disabled: false,
            has_dismiss_action: false,
            has_custom_dismiss_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-chip",
        "ui-chip--variant-accent",
        "ui-chip--size-sm",
        "ui-chip--static",
        "ui-chip--dismiss-label-default",
        "ui-chip--enabled",
        "ui-chip--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
