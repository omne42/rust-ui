use leptos::prelude::*;

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: ReadSignal<bool>,
    pub copy: Callback<()>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_snippet_logic(_text: String) -> SnippetLogic {
    let (copied, _set_copied) = signal(false);
    let copy = Callback::new(|_| {});
    SnippetLogic { copied, copy }
}

#[cfg(target_arch = "wasm32")]
pub fn use_snippet_logic(text: String) -> SnippetLogic {
    let (copied, set_copied) = signal(false);

    let copied_timeout = StoredValue::new_local(None::<TimeoutHandle>);

    on_cleanup(move || {
        if let Some(handle) = copied_timeout.get_value() {
            handle.clear();
        }
    });

    let copy = Callback::new(move |_| {
        if copied.get_untracked() {
            return;
        }

        if text.trim().is_empty() {
            return;
        }

        let text = text.clone();
        let set_copied = set_copied.clone();
        let copied_timeout = copied_timeout;
        leptos::task::spawn_local(async move {
            if !write_to_clipboard(text).await {
                return;
            }

            set_copied.set(true);

            if let Some(handle) = copied_timeout.get_value() {
                handle.clear();
            }
            copied_timeout.set_value(None);

            let set_copied_for_timeout = set_copied.clone();
            let timeout_result = set_timeout_with_handle(
                move || {
                    set_copied_for_timeout.set(false);
                    copied_timeout.set_value(None);
                },
                std::time::Duration::from_millis(2000),
            );
            if let Ok(handle) = timeout_result {
                copied_timeout.set_value(Some(handle));
            }
        });
    });

    SnippetLogic { copied, copy }
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
