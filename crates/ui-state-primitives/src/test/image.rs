use super::*;

#[test]
fn status_attr_is_closed_set() {
    let expected = ["idle", "loading", "loaded", "error"];
    for status in [
        ImageStatus::Idle,
        ImageStatus::Loading,
        ImageStatus::Loaded,
        ImageStatus::Error,
    ] {
        assert!(expected.contains(&status.as_attr()));
    }
}

#[test]
fn radius_and_shadow_contracts_are_stable() {
    assert_eq!(ImageRadius::Sm.class_name(), "ui-image--radius-sm");
    assert_eq!(ImageRadius::Full.class_name(), "ui-image--radius-full");
    assert_eq!(ImageRadius::Lg.as_attr(), "lg");

    assert_eq!(ImageShadow::None.class_name(), "ui-image--shadow-none");
    assert_eq!(ImageShadow::Md.class_name(), "ui-image--shadow-md");
    assert_eq!(ImageShadow::Sm.as_attr(), "sm");
}

#[test]
fn shows_skeleton_while_loading() {
    let state = resolve_view_state(
        Some("https://example.com/a.png"),
        None,
        ImageStatus::Loading,
        false,
        false,
    );
    assert!(state.show_image);
    assert!(state.show_skeleton);
    assert!(!state.show_fallback);
    assert_eq!(state.status_attr, "loading");
}

#[test]
fn shows_fallback_when_src_missing_or_error() {
    let state = resolve_view_state(None, Some("fallback.png"), ImageStatus::Idle, false, false);
    assert!(state.show_fallback);
    assert_eq!(state.status_attr, "idle");

    let state = resolve_view_state(
        Some("bad.png"),
        Some("fallback.png"),
        ImageStatus::Error,
        false,
        false,
    );
    assert!(state.show_fallback);
    assert!(!state.show_image);
    assert_eq!(state.status_attr, "error");
}

#[test]
fn loaded_state_marks_loaded_only_for_non_empty_src() {
    let loaded = resolve_view_state(
        Some("photo.png"),
        Some("fallback.png"),
        ImageStatus::Loaded,
        false,
        true,
    );
    assert!(loaded.show_image);
    assert!(loaded.show_blurred);
    assert!(loaded.is_loaded);
    assert_eq!(loaded.status, ImageStatus::Loaded);

    let empty = resolve_view_state(
        Some("  "),
        Some("fallback.png"),
        ImageStatus::Loaded,
        false,
        true,
    );
    assert!(!empty.show_image);
    assert!(!empty.is_loaded);
}
