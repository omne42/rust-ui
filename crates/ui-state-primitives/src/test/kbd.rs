use super::*;

#[test]
fn size_class_names_and_attrs_are_stable() {
    assert_eq!(KbdSize::Sm.class_name(), "ui-kbd--size-sm");
    assert_eq!(KbdSize::Md.class_name(), "ui-kbd--size-md");

    assert_eq!(KbdSize::Sm.as_attr(), "sm");
    assert_eq!(KbdSize::Md.as_attr(), "md");
}

#[test]
fn resolve_state_tracks_size_keys_and_class_source() {
    let state = resolve_state(KbdStateInput {
        size: KbdSize::Sm,
        has_keys: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.size, KbdSize::Sm);
    assert_eq!(state.size_class, "ui-kbd--size-sm");
    assert_eq!(state.size_attr, "sm");
    assert_eq!(state.state_class, "ui-kbd--state-with-keys");
    assert_eq!(state.state_attr, "with-keys");
    assert!(state.has_keys);
    assert!(state.has_custom_class_name);
}
