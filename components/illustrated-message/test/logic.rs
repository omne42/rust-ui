use super::*;

#[test]
fn resolves_flags() {
    let state = resolve_view_state(true, Some("Hello"), Some("World"), true);
    assert!(state.show_illustration);
    assert!(state.show_title);
    assert!(state.show_description);
    assert!(state.show_actions);

    let state = resolve_view_state(false, Some(" "), None, false);
    assert!(!state.show_illustration);
    assert!(!state.show_title);
    assert!(!state.show_description);
    assert!(!state.show_actions);
}
