use super::*;

#[test]
fn listbox_option_attrs_encode_disabled_selected_and_focused_state() {
    let disabled = listbox_option_a11y_attrs(ListBoxOptionA11yInput {
        is_disabled: true,
        is_selected: true,
        is_focused: true,
    });
    assert_eq!(disabled.role, "option");
    assert_eq!(disabled.data_state, "disabled");
    assert_eq!(disabled.aria_disabled, Some("true"));
    assert_eq!(disabled.aria_selected, Some("true"));
    assert_eq!(disabled.data_disabled, Some("true"));

    let selected = listbox_option_a11y_attrs(ListBoxOptionA11yInput {
        is_disabled: false,
        is_selected: true,
        is_focused: false,
    });
    assert_eq!(selected.data_state, "selected");
    assert_eq!(selected.aria_selected, Some("true"));
    assert_eq!(selected.data_selected, Some("true"));
    assert_eq!(selected.data_focused, None);
    assert_eq!(selected.aria_disabled, None);

    let focused = listbox_option_a11y_attrs(ListBoxOptionA11yInput {
        is_disabled: false,
        is_selected: false,
        is_focused: true,
    });
    assert_eq!(focused.data_state, "focused");
    assert_eq!(focused.data_focused, Some("true"));
    assert_eq!(focused.data_selected, None);

    let idle = listbox_option_a11y_attrs(ListBoxOptionA11yInput {
        is_disabled: false,
        is_selected: false,
        is_focused: false,
    });
    assert_eq!(idle.data_state, "idle");
    assert_eq!(idle.aria_selected, None);
    assert_eq!(idle.aria_disabled, None);
}
