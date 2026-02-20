use super::*;
use ui_state_primitives::spacer::{SpacerAxis, SpacerSize};

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-spacer  ".to_string())),
        Some("docs-spacer".to_string())
    );
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(SpacerStateInput {
            axis: SpacerAxis::Vertical,
            size: SpacerSize::Md,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-spacer",
        "ui-spacer--axis-vertical",
        "ui-spacer--size-md",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
