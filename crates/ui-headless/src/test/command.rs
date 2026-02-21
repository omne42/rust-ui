use super::*;
use crate::A11yDirection;

#[test]
fn command_input_attrs_maps_locale_context() {
    let attrs = command_input_attrs(Some("  zh-CN ".to_string()), Some(A11yDirection::Rtl));

    assert_eq!(attrs.role, "combobox");
    assert_eq!(attrs.aria_autocomplete, "list");
    assert_eq!(attrs.aria_expanded, "true");
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn command_option_attrs_encode_state_contract() {
    let disabled = command_option_a11y_attrs(CommandOptionA11yInput {
        is_disabled: true,
        is_selected: false,
        is_focused: true,
    });
    assert_eq!(disabled.role, "option");
    assert_eq!(disabled.data_state, "disabled");
    assert_eq!(disabled.aria_disabled, Some("true"));
    assert_eq!(disabled.data_disabled, Some("true"));
    assert_eq!(disabled.aria_selected, None);

    let selected = command_option_a11y_attrs(CommandOptionA11yInput {
        is_disabled: false,
        is_selected: true,
        is_focused: false,
    });
    assert_eq!(selected.data_state, "selected");
    assert_eq!(selected.aria_selected, Some("true"));
    assert_eq!(selected.data_selected, Some("true"));
    assert_eq!(selected.data_focused, None);
}

#[test]
fn command_input_key_down_result_is_stable() {
    assert_eq!(
        resolve_command_input_key_down("Escape", true),
        CommandInputKeyDownResult::ClearedQuery
    );
    assert_eq!(
        resolve_command_input_key_down("Escape", false),
        CommandInputKeyDownResult::Ignored
    );
    assert_eq!(
        resolve_command_input_key_down("ArrowDown", true),
        CommandInputKeyDownResult::DelegatedToListBox
    );
    assert_eq!(
        resolve_command_input_key_down("x", true),
        CommandInputKeyDownResult::Ignored
    );
}
