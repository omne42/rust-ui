use super::*;

#[test]
fn state_and_description_attrs_follow_contract() {
    assert_eq!(
        ModalDescriptionState::WithDescription.as_state_attr(),
        "with-description"
    );
    assert_eq!(
        ModalDescriptionState::TitleOnly.as_state_attr(),
        "title-only"
    );
    assert_eq!(
        ModalDescriptionState::WithDescription.as_description_attr(),
        "present"
    );
    assert_eq!(
        ModalDescriptionState::TitleOnly.as_description_attr(),
        "absent"
    );
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-modal ".to_string())),
        Some("docs-modal".to_string())
    );
}

#[test]
fn normalize_required_text_falls_back_for_blank_values() {
    assert_eq!(
        normalize_required_text(" Confirm ".to_string(), DEFAULT_TITLE),
        "Confirm"
    );
    assert_eq!(
        normalize_required_text(" ".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );
}

#[test]
fn normalize_id_base_uses_default_for_blank_values() {
    assert_eq!(normalize_id_base(" docs-modal ".to_string()), "docs-modal");
    assert_eq!(normalize_id_base("  ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(ModalPartStateInput {
        slot: ModalSlot::Root,
        description_state: ModalDescriptionState::WithDescription,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "modal");
    assert_eq!(state.base_class, "ui-modal");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-modal".to_string()),
        resolve_state(ModalPartStateInput {
            slot: ModalSlot::Root,
            description_state: ModalDescriptionState::WithDescription,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-modal",
        "ui-modal--with-description",
        "ui-modal--custom-id",
        "ui-modal--custom-title",
        "ui-modal--custom-description",
        "ui-modal--custom-motion",
        "ui-modal--custom-exit",
        "ui-modal--custom-class",
        "docs-modal",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_open_state_supports_controlled_and_uncontrolled_modes() {
    let controlled = normalize_open_state(ModalOpenStateInput {
        is_open: Some(Signal::derive(|| true)),
        default_open: Some(false),
        on_open_change: None,
    });

    assert!(matches!(controlled.mode, ModalOpenMode::Controlled));
    assert!(controlled.open.is_some());
    assert!(!controlled.default_open);
    assert!(controlled.has_default_open);
    assert_eq!(controlled.open_prop_source, ModalOpenPropSource::IsOpen);

    let uncontrolled = normalize_open_state(ModalOpenStateInput {
        is_open: None,
        default_open: Some(true),
        on_open_change: Some(Callback::new(|_: bool| {})),
    });

    assert!(matches!(uncontrolled.mode, ModalOpenMode::Uncontrolled));
    assert!(uncontrolled.open.is_none());
    assert!(uncontrolled.default_open);
    assert!(uncontrolled.has_default_open);
    assert!(uncontrolled.has_open_change_handler);
    assert_eq!(uncontrolled.open_prop_source, ModalOpenPropSource::None);
}

#[test]
fn normalize_open_state_uses_implicit_default_when_missing() {
    let normalized = normalize_open_state(ModalOpenStateInput {
        is_open: None,
        default_open: None,
        on_open_change: None,
    });

    assert!(matches!(normalized.mode, ModalOpenMode::Uncontrolled));
    assert!(!normalized.default_open);
    assert!(!normalized.has_default_open);
    assert!(!normalized.has_open_change_handler);
    assert_eq!(normalized.open_prop_source, ModalOpenPropSource::None);
}

#[test]
fn normalize_on_exit_complete_defaults_to_noop_callback() {
    let default_callback = normalize_on_exit_complete(None);
    default_callback.run(());

    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();
    let custom = Callback::new(move |_| called_clone.store(true, Ordering::SeqCst));
    let custom_callback = normalize_on_exit_complete(Some(custom));
    custom_callback.run(());
    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn resolve_open_contract_derives_mode_and_source_markers() {
    let controlled_state = normalize_open_state(ModalOpenStateInput {
        is_open: Some(Signal::derive(|| true)),
        default_open: Some(false),
        on_open_change: Some(Callback::new(|_: bool| {})),
    });
    let controlled = resolve_open_contract(&controlled_state);
    assert_eq!(controlled.mode, ModalOpenMode::Controlled);
    assert_eq!(controlled.open_source, ModalOpenSource::Controlled);
    assert_eq!(controlled.open_change_source, ModalOpenChangeSource::Custom);
    assert_eq!(controlled.open_prop_source, ModalOpenPropSource::IsOpen);

    let uncontrolled_state = normalize_open_state(ModalOpenStateInput {
        is_open: None,
        default_open: None,
        on_open_change: None,
    });
    let uncontrolled = resolve_open_contract(&uncontrolled_state);
    assert_eq!(uncontrolled.mode, ModalOpenMode::Uncontrolled);
    assert_eq!(uncontrolled.open_source, ModalOpenSource::ImplicitDefault);
    assert_eq!(uncontrolled.open_change_source, ModalOpenChangeSource::None);
    assert_eq!(uncontrolled.open_prop_source, ModalOpenPropSource::None);
}

#[test]
fn resolve_content_state_normalizes_text_and_sources() {
    let state = resolve_content_state(ModalContentStateInput {
        id_base: "  ".to_string(),
        title: "  ".to_string(),
        description: Some(" desc ".to_string()),
        class_name: Some(" modal-custom ".to_string()),
    });

    assert_eq!(state.id_base, DEFAULT_ID_BASE);
    assert_eq!(state.title, DEFAULT_TITLE);
    assert_eq!(state.description.as_deref(), Some("desc"));
    assert_eq!(
        state.description_state,
        ModalDescriptionState::WithDescription
    );
    assert_eq!(state.class_name.as_deref(), Some("modal-custom"));
    assert!(!state.has_custom_id_base);
    assert!(!state.has_custom_title);
    assert!(state.has_custom_description);
    assert!(state.has_custom_class_name);
}
