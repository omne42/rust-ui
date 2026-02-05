use leptos::prelude::*;

pub const CLEAR_BUTTON_ARIA_LABEL: &str = "Clear search";

#[derive(Clone)]
pub struct SearchFieldState {
    pub show_clear_button: Memo<bool>,
}

pub fn should_show_clear_button(value: &str, disabled: bool, read_only: bool) -> bool {
    !disabled && !read_only && !value.is_empty()
}

pub fn use_search_field(
    value: ReadSignal<String>,
    disabled: bool,
    read_only: bool,
) -> SearchFieldState {
    let show_clear_button = Memo::new(move |_| {
        let value = value.get();
        should_show_clear_button(&value, disabled, read_only)
    });
    SearchFieldState { show_clear_button }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_button_requires_value_and_editable_state() {
        assert!(!should_show_clear_button("", false, false));
        assert!(should_show_clear_button("query", false, false));
        assert!(!should_show_clear_button("query", true, false));
        assert!(!should_show_clear_button("query", false, true));
    }
}
