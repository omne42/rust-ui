use crate::popover::should_dismiss_popover_on_escape;

#[test]
fn dismiss_on_escape_requires_topmost_non_composing_non_prevented_escape() {
    assert!(should_dismiss_popover_on_escape(
        "Escape", true, false, false
    ));
    assert!(!should_dismiss_popover_on_escape(
        "Enter", true, false, false
    ));
    assert!(!should_dismiss_popover_on_escape(
        "Escape", false, false, false
    ));
    assert!(!should_dismiss_popover_on_escape(
        "Escape", true, true, false
    ));
    assert!(!should_dismiss_popover_on_escape(
        "Escape", true, false, true
    ));
}
