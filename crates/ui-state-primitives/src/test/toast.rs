use super::*;

fn payload(text: &str) -> String {
    let mut value = String::with_capacity(text.len());
    value.push_str(text);
    value
}

#[test]
fn max_toasts_is_normalized_to_one() {
    let state = ToastState::<String>::new(ToastStateOptions { max_toasts: 0 });
    assert_eq!(state.max_toasts(), 1);
}

#[test]
fn push_overflow_closes_oldest_and_rotates_to_end() {
    let mut state = ToastState::new(ToastStateOptions { max_toasts: 2 });

    state.push("one".to_string(), payload("One"));
    state.push("two".to_string(), payload("Two"));
    let mutations = state.push("three".to_string(), payload("Three"));

    assert_eq!(state.toasts().len(), 3);
    assert_eq!(state.toasts()[0].id, "two");
    assert_eq!(state.toasts()[1].id, "three");
    assert_eq!(state.toasts()[2].id, "one");

    assert!(state.toasts()[0].is_open);
    assert!(state.toasts()[1].is_open);
    assert!(!state.toasts()[2].is_open);

    assert!(
        mutations
            .iter()
            .any(|m| { m.id == "three" && m.kind == ToastMutationKind::Pushed })
    );
    assert!(
        mutations
            .iter()
            .any(|m| { m.id == "one" && m.kind == ToastMutationKind::OverflowClosed })
    );
}

#[test]
fn dismiss_closes_open_toast_once() {
    let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
    state.push("one".to_string(), payload("One"));

    let first = state.dismiss("one");
    let second = state.dismiss("one");

    assert_eq!(
        first,
        Some(ToastMutation {
            id: "one".to_string(),
            kind: ToastMutationKind::Dismissed,
        })
    );
    assert_eq!(second, None);
}

#[test]
fn clear_closes_only_open_toasts() {
    let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
    state.push("one".to_string(), payload("One"));
    state.push("two".to_string(), payload("Two"));
    state.dismiss("one");

    let mutations = state.clear();

    assert_eq!(mutations.len(), 1);
    assert_eq!(mutations[0].id, "two");
    assert_eq!(mutations[0].kind, ToastMutationKind::Cleared);
}

#[test]
fn remove_drops_toast_by_id() {
    let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
    state.push("one".to_string(), payload("One"));
    state.push("two".to_string(), payload("Two"));

    let removed = state.remove("one");

    assert_eq!(
        removed,
        Some(ToastMutation {
            id: "one".to_string(),
            kind: ToastMutationKind::Removed,
        })
    );
    assert_eq!(state.toasts().len(), 1);
    assert_eq!(state.toasts()[0].id, "two");
}

#[test]
fn text_normalization_helpers_trim_and_default() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-toast  ".to_string())),
        Some("docs-toast".to_string())
    );

    assert_eq!(
        normalize_title("  Saved  ".to_string(), DEFAULT_TITLE),
        "Saved"
    );
    assert_eq!(
        normalize_title(" \t ".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );
    assert_eq!(
        normalize_description(Some("  done  ".to_string())),
        Some("done".to_string())
    );
}

#[test]
fn toast_state_markers_are_derived_from_input() {
    let part = resolve_state(ToastPartStateInput {
        slot: ToastSlot::Root,
        variant: ToastVariant::Danger,
        is_open: false,
        has_description: true,
        has_custom_id: true,
        has_custom_description: true,
        has_custom_class_name: false,
        has_custom_motion: true,
        has_custom_on_close: true,
        has_custom_on_exit_complete: false,
    });
    assert_eq!(part.state_attr, "closing");
    assert_eq!(part.variant_attr, "danger");
    assert_eq!(part.description_attr, "present");
    assert_eq!(part.close_mode_attr, "handler");
    assert_eq!(part.id_source_attr, "custom");
    assert_eq!(part.class_source_attr, "default");
    assert_eq!(part.motion_source_attr, "custom");
    assert_eq!(part.exit_source_attr, "default");

    let viewport = resolve_viewport_state(ToastViewportStateInput {
        slot: ToastViewportSlot::Root,
        portal: false,
        max_toasts: 0,
        has_custom_portal: true,
        has_custom_max_toasts: true,
        has_custom_class_name: false,
        has_custom_motion: true,
        store_source: ToastStoreSource::Local,
    });
    assert_eq!(viewport.state_attr, "inline");
    assert_eq!(viewport.queue_attr, "single");
    assert_eq!(viewport.max_toasts, 1);
    assert_eq!(viewport.portal_source_attr, "custom");
    assert_eq!(viewport.max_toasts_source_attr, "custom");
    assert_eq!(viewport.store_source_attr, "local");
}
