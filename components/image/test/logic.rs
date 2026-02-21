use super::*;

#[test]
fn normalize_props_centralizes_default_fallbacks() {
    let normalized = normalize_props(ImageNormalizeInput {
        src: Some("   ".to_string()),
        fallback_src: None,
        class_name: Some("   ".to_string()),
        lang: Some("   ".to_string()),
        radius: ImageRadius::Lg,
        shadow: ImageShadow::Sm,
    });

    assert_eq!(normalized.src, None);
    assert_eq!(normalized.src_attr, "");
    assert_eq!(normalized.fallback_src, None);
    assert_eq!(normalized.fallback_src_attr, "");
    assert_eq!(
        normalized.class_name,
        "ui-image ui-image--radius-lg ui-image--shadow-sm"
    );
    assert_eq!(normalized.lang, None);
}

#[test]
fn normalize_props_applies_trim_and_custom_class_priority() {
    let normalized = normalize_props(ImageNormalizeInput {
        src: Some(" https://example.com/img.png ".to_string()),
        fallback_src: Some(" /fallback.png ".to_string()),
        class_name: Some(" custom-image ".to_string()),
        lang: Some(" en-US ".to_string()),
        radius: ImageRadius::Md,
        shadow: ImageShadow::Md,
    });

    assert_eq!(
        normalized.src.as_deref(),
        Some("https://example.com/img.png")
    );
    assert_eq!(normalized.src_attr, "https://example.com/img.png");
    assert_eq!(normalized.fallback_src.as_deref(), Some("/fallback.png"));
    assert_eq!(normalized.fallback_src_attr, "/fallback.png");
    assert_eq!(
        normalized.class_name,
        "ui-image ui-image--radius-md ui-image--shadow-md custom-image"
    );
    assert_eq!(normalized.lang.as_deref(), Some("en-US"));
}

#[test]
fn derive_view_state_uses_typed_input_contract() {
    let state = derive_view_state(ImageViewStateInput {
        src: Some("https://example.com/image.png".to_string()),
        fallback_src: Some("/fallback.png".to_string()),
        status: ImageStatus::Loading,
        is_skeleton_disabled: false,
        is_blurred: true,
    });

    assert!(state.show_image);
    assert!(!state.show_fallback);
    assert!(state.show_skeleton);
    assert!(!state.show_blurred);
    assert_eq!(state.status_attr, "loading");
}

#[test]
fn apply_status_event_and_motion_source_are_centralized_in_logic() {
    assert_eq!(
        apply_status_event(ImageStatus::Loading, ImageStatusEvent::LoadSucceeded),
        ImageStatus::Loaded
    );

    let default_motion_source = resolve_motion_source(crate::motion::ImageMotion::default());
    assert_eq!(default_motion_source, ImageMotionSource::Default);
    assert_eq!(default_motion_source.as_attr(), "default");
    assert!(!default_motion_source.is_custom());

    let custom_motion_source = resolve_motion_source(crate::motion::ImageMotion {
        zoom_scale: 1.4,
        ..crate::motion::ImageMotion::default()
    });
    assert_eq!(custom_motion_source, ImageMotionSource::Custom);
    assert_eq!(custom_motion_source.as_attr(), "custom");
    assert!(custom_motion_source.is_custom());

    assert_eq!(ImageStatusSource::Initial.as_attr(), "initial");
    assert_eq!(ImageStatusSource::Event.as_attr(), "event");
}
