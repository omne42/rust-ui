use super::*;
use ui_state_primitives::collapsible::CollapsibleStateInput;

#[test]
fn compose_class_name_includes_state_mode_and_custom_markers() {
    let state = resolve_state(CollapsibleStateInput {
        is_open: false,
        is_disabled: true,
        is_controlled: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    let class_name = compose_class_name(Some("docs-collapsible".to_string()), state);

    for token in [
        "ui-collapsible",
        "ui-collapsible--state-disabled",
        "ui-collapsible--mode-uncontrolled",
        "ui-collapsible--custom-motion",
        "ui-collapsible--custom-class",
        "docs-collapsible",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
