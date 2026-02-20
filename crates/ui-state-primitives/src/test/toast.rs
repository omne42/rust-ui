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
