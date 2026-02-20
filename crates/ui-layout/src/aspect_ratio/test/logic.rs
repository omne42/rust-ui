use super::*;

#[test]
fn compose_class_name_merges_custom_class_and_flags() {
    let state = resolve_state(AspectRatioStateInput {
        ratio: AspectRatioPreset::Video,
        radius: AspectRatioRadius::Sm,
        bordered: true,
        fill: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class = compose_class_name(Some("docs-aspect".to_string()), state);

    for class_name in [
        "ui-aspect-ratio",
        "ui-aspect-ratio--ratio-video",
        "ui-aspect-ratio--radius-sm",
        "ui-aspect-ratio--bordered",
        "ui-aspect-ratio--custom-class",
        "docs-aspect",
    ] {
        assert!(
            class.contains(class_name),
            "class list should include `{class_name}`; got: {class}"
        );
    }
}
