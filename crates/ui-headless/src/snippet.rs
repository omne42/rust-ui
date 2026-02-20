use crate::a11y::{A11yDirection, locale_attrs};
use crate::controllable_state::use_controllable_state;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SnippetCopyOptions {
    pub text: String,
    pub is_copyable: bool,
    pub is_copied: Option<Signal<bool>>,
    pub default_copied: Option<bool>,
    pub on_copied_change: Option<Callback<bool>>,
    pub on_copy_error: Option<Callback<()>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SnippetCopyState {
    pub copied: Signal<bool>,
    pub is_loading: ReadSignal<bool>,
    pub has_error: ReadSignal<bool>,
    pub is_copyable: bool,
    pub is_actionable: bool,
}

#[derive(Clone)]
pub struct SnippetCopyHandlers {
    pub on_copy: Callback<()>,
    pub on_retry: Callback<()>,
}

#[derive(Clone)]
pub struct SnippetCopyAttrs {
    pub aria_busy: Signal<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SnippetCopyContract {
    pub state: SnippetCopyState,
    pub handlers: SnippetCopyHandlers,
    pub attrs: SnippetCopyAttrs,
}

#[cfg(target_arch = "wasm32")]
async fn write_to_clipboard(text: String) -> bool {
    use leptos::__reexports::wasm_bindgen_futures::JsFuture;
    use leptos::__reexports::wasm_bindgen_futures::js_sys::{self, Function, Promise, Reflect};
    use leptos::wasm_bindgen::{JsCast, JsValue};

    let global = js_sys::global();

    let navigator = match Reflect::get(&global, &JsValue::from_str("navigator")) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let clipboard = match Reflect::get(&navigator, &JsValue::from_str("clipboard")) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let write_text = match Reflect::get(&clipboard, &JsValue::from_str("writeText")) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let Ok(write_text) = write_text.dyn_into::<Function>() else {
        return false;
    };

    let Ok(promise) = write_text.call1(&clipboard, &JsValue::from_str(&text)) else {
        return false;
    };

    let Ok(promise) = promise.dyn_into::<Promise>() else {
        return false;
    };

    JsFuture::from(promise).await.is_ok()
}

pub fn use_snippet_copy(options: SnippetCopyOptions) -> SnippetCopyContract {
    let SnippetCopyOptions {
        text,
        is_copyable,
        is_copied,
        default_copied,
        on_copied_change,
        on_copy_error,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let has_text = !text.trim().is_empty();
    let is_actionable = is_copyable && has_text;

    let copied_state = use_controllable_state(
        is_copied,
        Some(default_copied.unwrap_or(false)),
        on_copied_change,
    );

    let copied = Signal::derive(move || copied_state.value.get());
    #[cfg(target_arch = "wasm32")]
    let request_copied = copied_state.request_change;
    #[cfg(not(target_arch = "wasm32"))]
    let _unused_on_copy_error = on_copy_error;
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);
    #[cfg(target_arch = "wasm32")]
    let on_copy_error = on_copy_error.unwrap_or_else(|| Callback::new(|_| {}));

    let copied_timeout = StoredValue::new_local(None::<TimeoutHandle>);

    on_cleanup(move || {
        if let Some(handle) = copied_timeout.get_value() {
            handle.clear();
        }
    });

    let on_copy = Callback::new(move |_| {
        if copied.get_untracked() || is_loading.get_untracked() {
            return;
        }

        if !is_actionable {
            return;
        }

        set_has_error.set(false);
        set_is_loading.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            let text = text.clone();
            let set_is_loading = set_is_loading.clone();
            let set_has_error = set_has_error.clone();
            let request_copied = request_copied.clone();
            let copied_timeout = copied_timeout;
            let on_copy_error = on_copy_error.clone();

            leptos::task::spawn_local(async move {
                if !write_to_clipboard(text).await {
                    set_is_loading.set(false);
                    set_has_error.set(true);
                    on_copy_error.run(());
                    return;
                }

                request_copied.run(true);
                set_is_loading.set(false);
                set_has_error.set(false);

                if let Some(handle) = copied_timeout.get_value() {
                    handle.clear();
                }
                copied_timeout.set_value(None);

                let request_copied_for_timeout = request_copied.clone();
                let timeout_result = set_timeout_with_handle(
                    move || {
                        request_copied_for_timeout.run(false);
                        copied_timeout.set_value(None);
                    },
                    std::time::Duration::from_millis(2000),
                );
                if let Ok(handle) = timeout_result {
                    copied_timeout.set_value(Some(handle));
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _unused_text = &text;
            set_is_loading.set(false);
        }
    });

    let on_retry = on_copy;

    let aria_busy = Signal::derive(move || is_loading.get().then_some("true"));

    SnippetCopyContract {
        state: SnippetCopyState {
            copied,
            is_loading,
            has_error,
            is_copyable,
            is_actionable,
        },
        handlers: SnippetCopyHandlers { on_copy, on_retry },
        attrs: SnippetCopyAttrs {
            aria_busy,
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
#[path = "test/snippet.rs"]
mod tests;
