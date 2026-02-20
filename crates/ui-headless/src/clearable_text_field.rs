use leptos::prelude::*;

#[derive(Clone)]
pub struct ClearableTextFieldOptions {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_clearable: bool,
    pub is_empty: Signal<bool>,
    pub on_clear: Option<Callback<()>>,
}

#[derive(Clone)]
pub struct ClearableTextFieldAttrs {
    pub aria_keyshortcuts: Memo<Option<&'static str>>,
}

#[derive(Clone)]
pub struct ClearableTextFieldHandlers {
    pub on_key_down: Callback<String, bool>,
}

#[derive(Clone)]
pub struct ClearableTextFieldState {
    pub can_clear_on_escape: Memo<bool>,
}

#[derive(Clone)]
pub struct ClearableTextField {
    pub attrs: ClearableTextFieldAttrs,
    pub handlers: ClearableTextFieldHandlers,
    pub state: ClearableTextFieldState,
}

pub fn use_clearable_text_field(options: ClearableTextFieldOptions) -> ClearableTextField {
    let can_clear_on_escape = Memo::new({
        let is_empty = options.is_empty;
        move |_| {
            options.is_clearable && !options.is_disabled && !options.is_read_only && !is_empty.get()
        }
    });

    let aria_keyshortcuts = Memo::new(move |_| can_clear_on_escape.get().then_some("Escape"));

    let on_clear = options.on_clear;
    let on_key_down = Callback::new(move |key: String| -> bool {
        if key != "Escape" || !can_clear_on_escape.get_untracked() {
            return false;
        }
        if let Some(on_clear) = on_clear.as_ref() {
            on_clear.run(());
        }
        true
    });

    ClearableTextField {
        attrs: ClearableTextFieldAttrs { aria_keyshortcuts },
        handlers: ClearableTextFieldHandlers { on_key_down },
        state: ClearableTextFieldState {
            can_clear_on_escape,
        },
    }
}

#[cfg(test)]
#[path = "test/clearable_text_field.rs"]
mod tests;
