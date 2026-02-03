use leptos::prelude::*;

pub const CLEAR_BUTTON_ARIA_LABEL: &str = "Clear search";

#[derive(Clone)]
pub struct SearchFieldState {
    pub show_clear_button: Memo<bool>,
}

pub fn use_search_field(value: ReadSignal<String>, disabled: bool) -> SearchFieldState {
    let show_clear_button = Memo::new(move |_| !disabled && !value.get().is_empty());
    SearchFieldState { show_clear_button }
}
