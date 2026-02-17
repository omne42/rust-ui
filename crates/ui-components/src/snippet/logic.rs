use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetStateInput {
    pub is_multiline: bool,
    pub has_text: bool,
    pub has_label: bool,
    pub copyable: bool,
    pub has_custom_copied_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetViewState {
    pub is_multiline: bool,
    pub is_empty: bool,
    pub has_label: bool,
    pub copyable: bool,
    pub copy_is_actionable: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub copy_state_class: &'static str,
    pub copy_state_attr: &'static str,
    pub copied_label_source_class: &'static str,
    pub copied_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: SnippetStateInput) -> SnippetViewState {
    let (state_class, state_attr) = if input.is_multiline {
        ("ui-snippet--state-multiline", "multiline")
    } else {
        ("ui-snippet--state-single-line", "single-line")
    };

    let copy_is_actionable = input.copyable && input.has_text;
    let (copy_state_class, copy_state_attr) = if input.copyable {
        if copy_is_actionable {
            ("ui-snippet--copyable", "copyable")
        } else {
            ("ui-snippet--copy-disabled", "disabled")
        }
    } else {
        ("ui-snippet--copy-static", "static")
    };

    let (copied_label_source_class, copied_label_source_attr) = if input.has_custom_copied_label {
        ("ui-snippet--custom-copied-label", "custom")
    } else {
        ("ui-snippet--default-copied-label", "default")
    };

    SnippetViewState {
        is_multiline: input.is_multiline,
        is_empty: !input.has_text,
        has_label: input.has_label,
        copyable: input.copyable,
        copy_is_actionable,
        state_class,
        state_attr,
        copy_state_class,
        copy_state_attr,
        copied_label_source_class,
        copied_label_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SnippetViewState) -> String {
    let mut classes = vec![
        "ui-snippet".to_string(),
        state.state_class.to_string(),
        state.copy_state_class.to_string(),
        state.copied_label_source_class.to_string(),
    ];

    if state.has_label {
        classes.push("ui-snippet--with-label".to_string());
    }
    if state.is_empty {
        classes.push("ui-snippet--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-snippet--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[derive(Clone)]
pub struct SnippetLogic {
    pub copied: ReadSignal<bool>,
    pub is_copying: ReadSignal<bool>,
    pub has_copy_error: ReadSignal<bool>,
    pub copy: Callback<()>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn use_snippet_logic(_text: String) -> SnippetLogic {
    let (copied, _set_copied) = signal(false);
    let (is_copying, _set_is_copying) = signal(false);
    let (has_copy_error, _set_has_copy_error) = signal(false);
    let copy = Callback::new(|_| {});
    SnippetLogic {
        copied,
        is_copying,
        has_copy_error,
        copy,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn use_snippet_logic(text: String) -> SnippetLogic {
    let (copied, set_copied) = signal(false);
    let (is_copying, set_is_copying) = signal(false);
    let (has_copy_error, set_has_copy_error) = signal(false);

    let copied_timeout = StoredValue::new_local(None::<TimeoutHandle>);

    on_cleanup(move || {
        if let Some(handle) = copied_timeout.get_value() {
            handle.clear();
        }
    });

    let copy = Callback::new(move |_| {
        if copied.get_untracked() || is_copying.get_untracked() {
            return;
        }

        if text.trim().is_empty() {
            return;
        }

        set_has_copy_error.set(false);
        set_is_copying.set(true);

        let text = text.clone();
        let set_copied = set_copied.clone();
        let set_is_copying = set_is_copying.clone();
        let set_has_copy_error = set_has_copy_error.clone();
        let copied_timeout = copied_timeout;
        leptos::task::spawn_local(async move {
            if !write_to_clipboard(text).await {
                set_is_copying.set(false);
                set_has_copy_error.set(true);
                return;
            }

            set_copied.set(true);
            set_is_copying.set(false);
            set_has_copy_error.set(false);

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

    SnippetLogic {
        copied,
        is_copying,
        has_copy_error,
        copy,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Done  ".to_string())),
            Some("Done".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_copy_and_label_sources() {
        let state = resolve_state(SnippetStateInput {
            is_multiline: true,
            has_text: true,
            has_label: true,
            copyable: true,
            has_custom_copied_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_class, "ui-snippet--state-multiline");
        assert_eq!(state.copy_state_class, "ui-snippet--copyable");
        assert_eq!(
            state.copied_label_source_class,
            "ui-snippet--custom-copied-label"
        );
        assert!(!state.is_empty);
        assert!(state.has_label);
        assert!(state.copyable);
        assert!(state.copy_is_actionable);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn resolve_state_marks_empty_copyable_snippet_as_disabled_copy() {
        let state = resolve_state(SnippetStateInput {
            is_multiline: false,
            has_text: false,
            has_label: false,
            copyable: true,
            has_custom_copied_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(state.copy_state_attr, "disabled");
        assert!(state.is_empty);
        assert!(!state.copy_is_actionable);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-snippet".to_string()),
            resolve_state(SnippetStateInput {
                is_multiline: false,
                has_text: false,
                has_label: true,
                copyable: true,
                has_custom_copied_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-snippet",
            "ui-snippet--state-single-line",
            "ui-snippet--copy-disabled",
            "ui-snippet--default-copied-label",
            "ui-snippet--with-label",
            "ui-snippet--empty",
            "ui-snippet--custom-class",
            "docs-snippet",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
