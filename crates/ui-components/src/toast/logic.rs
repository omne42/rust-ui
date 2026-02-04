use leptos::prelude::*;

fn next_id() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|cell| {
        let id = cell.get();
        cell.set(id + 1);
        id
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl ToastVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ToastVariant::Default => "ui-toast--variant-default",
            ToastVariant::Accent => "ui-toast--variant-accent",
            ToastVariant::Danger => "ui-toast--variant-danger",
        }
    }

    pub fn aria_live(self) -> &'static str {
        match self {
            ToastVariant::Danger => "assertive",
            _ => "polite",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToastOptions {
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration_ms: Option<u64>,
}

impl ToastOptions {
    pub fn simple(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration_ms: Some(3500),
        }
    }
}

#[derive(Clone)]
pub struct ToastInstance {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
}

#[derive(Clone, Copy, Debug)]
pub struct ToastStoreOptions {
    pub max_toasts: usize,
}

impl Default for ToastStoreOptions {
    fn default() -> Self {
        Self { max_toasts: 3 }
    }
}

#[derive(Clone)]
pub struct ToastStore {
    toasts: ReadSignal<Vec<ToastInstance>>,
    set_toasts: WriteSignal<Vec<ToastInstance>>,
    #[cfg(target_arch = "wasm32")]
    timeouts: StoredValue<std::collections::HashMap<String, TimeoutHandle>, LocalStorage>,
    pub push: Callback<ToastOptions, String>,
    pub dismiss: Callback<String>,
    pub clear: Callback<()>,
}

pub fn provide_toast_store(options: ToastStoreOptions) -> ToastStore {
    let (toasts, set_toasts) = signal(Vec::<ToastInstance>::new());

    #[cfg(target_arch = "wasm32")]
    let timeouts: StoredValue<std::collections::HashMap<String, TimeoutHandle>, LocalStorage> =
        StoredValue::new_local(std::collections::HashMap::new());

    let push = Callback::new({
        let max_toasts = options.max_toasts.max(1);
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |opts: ToastOptions| -> String {
            let id = format!("ui-toast-{}", next_id());
            let (open, set_open) = signal(true);

            let instance = ToastInstance {
                id: id.clone(),
                title: opts.title,
                description: opts.description,
                variant: opts.variant,
                open,
                set_open,
            };

            set_toasts.update(|list| {
                list.push(instance);
                if list.len() > max_toasts {
                    let overflow = list.len() - max_toasts;
                    for _ in 0..overflow {
                        if !list.is_empty() {
                            // Mark the oldest toast as closing; it will be removed by its exit callback.
                            #[cfg(target_arch = "wasm32")]
                            {
                                let mut map = timeouts.get_value();
                                if let Some(handle) = map.remove(&list[0].id) {
                                    handle.clear();
                                }
                                timeouts.set_value(map);
                            }
                            list[0].set_open.set(false);
                            // Rotate it to the end so it can animate out without affecting newer entries.
                            let oldest = list.remove(0);
                            list.push(oldest);
                        }
                    }
                }
            });

            #[cfg(target_arch = "wasm32")]
            if let Some(duration_ms) = opts.duration_ms.filter(|v| *v > 0) {
                let set_open_for_timeout = set_open.clone();
                let id_for_timeout = id.clone();
                let Ok(handle) = set_timeout_with_handle(
                    move || set_open_for_timeout.set(false),
                    std::time::Duration::from_millis(duration_ms),
                ) else {
                    return id;
                };

                let mut map = timeouts.get_value();
                if let Some(prev) = map.remove(&id_for_timeout) {
                    prev.clear();
                }
                map.insert(id_for_timeout, handle);
                timeouts.set_value(map);
            }

            id
        }
    });

    let dismiss = Callback::new({
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |id: String| {
            if id.trim().is_empty() {
                return;
            }
            let list = toasts.get_untracked();
            for toast in list {
                if toast.id == id {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let mut map = timeouts.get_value();
                        if let Some(handle) = map.remove(&toast.id) {
                            handle.clear();
                        }
                        timeouts.set_value(map);
                    }
                    toast.set_open.set(false);
                    break;
                }
            }
        }
    });

    let clear = Callback::new({
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |_| {
            let list = toasts.get_untracked();
            for toast in list {
                toast.set_open.set(false);
            }

            #[cfg(target_arch = "wasm32")]
            {
                let map = timeouts.get_value();
                for (_, handle) in map {
                    handle.clear();
                }
                timeouts.set_value(std::collections::HashMap::new());
            }
        }
    });

    let store = ToastStore {
        toasts,
        set_toasts,
        #[cfg(target_arch = "wasm32")]
        timeouts,
        push,
        dismiss,
        clear,
    };
    provide_context(store.clone());
    store
}

pub fn use_toast_store() -> Option<ToastStore> {
    use_context::<ToastStore>()
}

impl ToastStore {
    pub fn toasts(&self) -> ReadSignal<Vec<ToastInstance>> {
        self.toasts
    }

    pub fn push_simple(&self, title: impl Into<String>) -> String {
        self.push.run(ToastOptions::simple(title))
    }

    pub fn push_danger(&self, title: impl Into<String>, description: impl Into<String>) -> String {
        self.push.run(ToastOptions {
            title: title.into(),
            description: Some(description.into()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        })
    }

    pub fn remove(&self, id: &str) {
        let id = id.trim();
        if id.is_empty() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            let mut map = self.timeouts.get_value();
            if let Some(handle) = map.remove(id) {
                handle.clear();
            }
            self.timeouts.set_value(map);
        }

        self.set_toasts.update(|list| list.retain(|t| t.id != id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_map_to_expected_live_regions() {
        assert_eq!(ToastVariant::Default.aria_live(), "polite");
        assert_eq!(ToastVariant::Danger.aria_live(), "assertive");
    }

    #[test]
    fn toast_options_helper_sets_defaults() {
        let opts = ToastOptions::simple("Hello");
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.variant, ToastVariant::Default);
        assert!(opts.duration_ms.is_some());
    }
}
