use super::*;
use crate::toast::ToastSlot;

fn with_store(max_toasts: usize, f: impl FnOnce(ToastStore)) {
    Owner::new().with(|| {
        let store = provide_toast_store(ToastStoreOptions { max_toasts });
        f(store);
    });
}

#[test]
fn variant_aria_live_matches_severity() {
    assert_eq!(ToastVariant::Default.aria_live(), "polite");
    assert_eq!(ToastVariant::Accent.aria_live(), "polite");
    assert_eq!(ToastVariant::Danger.aria_live(), "assertive");
}

#[test]
fn agent_contracts_are_typed_and_stable() {
    let toast_contract = toast_agent_contract();
    assert_eq!(toast_contract.schema_attr, "ui.toast.v1");
    assert_eq!(
        toast_contract.intent_attr,
        ToastAgentIntent::NotificationItem.as_attr()
    );
    assert_eq!(
        toast_contract.action_model_attr,
        ToastAgentActionModel::DismissClose.as_attr()
    );
    assert_eq!(
        toast_contract.state_axis_attr,
        "state|variant|description|close-mode|open"
    );
    assert_eq!(
        toast_contract.source_axis_attr,
        "id|description|class|motion|close|exit|open"
    );

    let viewport_contract = toast_viewport_agent_contract();
    assert_eq!(viewport_contract.schema_attr, "ui.toast.viewport.v1");
    assert_eq!(
        viewport_contract.intent_attr,
        ToastAgentIntent::NotificationViewport.as_attr()
    );
    assert_eq!(
        viewport_contract.action_model_attr,
        ToastAgentActionModel::QueueDismissRemove.as_attr()
    );
    assert_eq!(
        viewport_contract.state_axis_attr,
        "state|queue|portal|max-toasts"
    );
    assert_eq!(
        viewport_contract.source_axis_attr,
        "portal|max-toasts|class|motion|store"
    );
}

#[test]
fn store_push_adds_toast_and_returns_id() {
    with_store(3, |store| {
        let id = store.push.run(ToastOptions::simple("Hello"));
        assert!(!id.trim().is_empty());

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, id);
        assert!(toasts[0].is_open);
    });
}

#[test]
fn store_overflow_marks_oldest_closing_and_rotates_to_end() {
    with_store(2, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));
        let id3 = store.push.run(ToastOptions::simple("Three"));

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 3);

        assert_eq!(toasts[0].id, id2);
        assert_eq!(toasts[1].id, id3);
        assert_eq!(toasts[2].id, id1);

        assert!(toasts[0].is_open);
        assert!(toasts[1].is_open);
        assert!(!toasts[2].is_open);
    });
}

#[test]
fn store_dismiss_marks_toast_closed() {
    with_store(3, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));

        store.dismiss.run(id1.clone());

        let toasts = store.toasts().get_untracked();
        let t1 = toasts.iter().find(|t| t.id == id1).unwrap();
        let t2 = toasts.iter().find(|t| t.id == id2).unwrap();
        assert!(!t1.is_open);
        assert!(t2.is_open);
    });
}

#[test]
fn store_clear_marks_all_toasts_closed() {
    with_store(3, |store| {
        store.push.run(ToastOptions::simple("One"));
        store.push.run(ToastOptions::simple("Two"));

        store.clear.run(());

        let toasts = store.toasts().get_untracked();
        assert!(!toasts.is_empty());
        for toast in toasts {
            assert!(!toast.is_open);
        }
    });
}

#[test]
fn store_remove_drops_toast_by_id() {
    with_store(3, |store| {
        let id1 = store.push.run(ToastOptions::simple("One"));
        let id2 = store.push.run(ToastOptions::simple("Two"));

        store.remove(&id1);

        let toasts = store.toasts().get_untracked();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].id, id2);
    });
}

#[test]
fn toast_options_helper_sets_defaults() {
    let opts = ToastOptions::simple("Hello");
    assert_eq!(opts.title, "Hello");
    assert_eq!(opts.variant, ToastVariant::Default);
    assert!(opts.duration_ms.is_some());
}

#[test]
fn normalize_helpers_trim_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-toast  ".to_string())),
        Some("docs-toast".to_string())
    );

    assert_eq!(normalize_title("  Saved  ".to_string()), "Saved");
    assert_eq!(normalize_title("\n\t".to_string()), "Notification");
    assert_eq!(
        normalize_description(Some("  done  ".to_string())),
        Some("done".to_string())
    );
}

#[test]
fn open_state_config_prefers_is_open_and_sets_default_open_fallback() {
    let (is_open_raw, _set_is_open_raw) = signal(false);
    let (open_raw, _set_open_raw) = signal(true);

    let config =
        resolve_open_state_config(Some(is_open_raw.into()), Some(open_raw.into()), None, None);

    assert!(config.is_controlled);
    assert_eq!(config.open_source_attr, "is_open");
    assert_eq!(config.default_open, Some(true));
    assert!(!config.has_custom_default_open);
    assert!(!config.has_custom_on_open_change);
}

#[test]
fn compose_toast_class_name_tracks_state_markers() {
    let class_name = compose_class_name(
        Some("docs-toast-custom".to_string()),
        resolve_state(ToastPartStateInput {
            slot: ToastSlot::Root,
            variant: ToastVariant::Accent,
            is_open: false,
            has_description: true,
            has_custom_id: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_on_close: true,
            has_custom_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-toast",
        "ui-toast--variant-accent",
        "ui-toast--closing",
        "ui-toast--with-description",
        "ui-toast--custom-class",
        "docs-toast-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}
