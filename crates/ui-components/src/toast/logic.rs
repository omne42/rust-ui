use crate::toast::{
    ToastPartState, ToastPartStateInput, ToastViewportState, ToastViewportStateInput,
};
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

pub const DEFAULT_TITLE: &str = "Notification";
pub const DEFAULT_VIEWPORT_PORTAL: bool = true;
pub const DEFAULT_VIEWPORT_MAX_TOASTS: usize = 3;

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

    pub fn as_attr(self) -> &'static str {
        match self {
            ToastVariant::Default => "default",
            ToastVariant::Accent => "accent",
            ToastVariant::Danger => "danger",
        }
    }

    pub fn aria_live(self) -> &'static str {
        match self {
            ToastVariant::Danger => "assertive",
            _ => "polite",
        }
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn toast_state_attr(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closing" }
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn close_mode_attr(has_on_close: bool) -> &'static str {
    if has_on_close { "handler" } else { "noop" }
}

pub fn viewport_state_attr(portal: bool) -> &'static str {
    if portal { "portal" } else { "inline" }
}

pub fn viewport_queue_attr(max_toasts: usize) -> &'static str {
    if max_toasts <= 1 {
        "single"
    } else if max_toasts <= 3 {
        "bounded"
    } else {
        "extended"
    }
}

pub fn normalize_viewport_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

pub fn resolve_state(input: ToastPartStateInput) -> ToastPartState {
    ToastPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: toast_state_attr(input.is_open),
        variant: input.variant,
        variant_attr: input.variant.as_attr(),
        description_attr: description_attr(input.has_description),
        close_mode_attr: close_mode_attr(input.has_custom_on_close),
        open_attr: input.is_open.then_some("true"),
        is_open: input.is_open,
        has_description: input.has_description,
        has_custom_id: input.has_custom_id,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_on_close: input.has_custom_on_close,
        has_custom_on_exit_complete: input.has_custom_on_exit_complete,
        id_source_attr: source_attr(input.has_custom_id),
        description_source_attr: source_attr(input.has_custom_description),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        close_source_attr: source_attr(input.has_custom_on_close),
        exit_source_attr: source_attr(input.has_custom_on_exit_complete),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ToastPartState) -> String {
    let mut classes = vec![
        state.base_class.to_string(),
        state.variant.class_name().to_string(),
    ];

    if state.is_open {
        classes.push("ui-toast--open".to_string());
    } else {
        classes.push("ui-toast--closing".to_string());
    }

    if state.has_description {
        classes.push("ui-toast--with-description".to_string());
    } else {
        classes.push("ui-toast--title-only".to_string());
    }

    if state.has_custom_id {
        classes.push("ui-toast--custom-id".to_string());
    }

    if state.has_custom_description {
        classes.push("ui-toast--custom-description".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-toast--custom-motion".to_string());
    }

    if state.has_custom_on_close {
        classes.push("ui-toast--custom-close".to_string());
    }

    if state.has_custom_on_exit_complete {
        classes.push("ui-toast--custom-exit".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-toast--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState {
    let max_toasts = normalize_viewport_max_toasts(input.max_toasts);

    ToastViewportState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: viewport_state_attr(input.portal),
        queue_attr: viewport_queue_attr(max_toasts),
        portal_attr: if input.portal { "true" } else { "false" },
        max_toasts,
        portal: input.portal,
        has_custom_portal: input.has_custom_portal,
        has_custom_max_toasts: input.has_custom_max_toasts,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        portal_source_attr: source_attr(input.has_custom_portal),
        max_toasts_source_attr: source_attr(input.has_custom_max_toasts),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        store_source: input.store_source,
        store_source_attr: input.store_source.as_attr(),
    }
}

pub fn compose_viewport_class_name(
    base_class_name: Option<String>,
    state: ToastViewportState,
) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.portal {
        classes.push("ui-toast-viewport--portal".to_string());
    } else {
        classes.push("ui-toast-viewport--inline".to_string());
    }

    if state.has_custom_portal {
        classes.push("ui-toast-viewport--custom-portal".to_string());
    }

    if state.has_custom_max_toasts {
        classes.push("ui-toast-viewport--custom-max-toasts".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-toast-viewport--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-toast-viewport--custom-class".to_string());
    }

    if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_title(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_TITLE.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
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
    use crate::toast::ToastSlot;

    fn with_store(max_toasts: usize, f: impl FnOnce(ToastStore)) {
        Owner::new().with(|| {
            let store = provide_toast_store(ToastStoreOptions { max_toasts });
            f(store);
        });
    }

    #[test]
    fn variant_aria_live_matches_severity() {
        assert_eq!(ToastVariant::Default.aria_live(), "polite");
        assert_eq!(ToastVariant::Accent.aria_live(), "polite");
        assert_eq!(ToastVariant::Danger.aria_live(), "assertive");
    }

    #[test]
    fn store_push_adds_toast_and_returns_id() {
        with_store(3, |store| {
            let id = store.push.run(ToastOptions::simple("Hello"));
            assert!(!id.trim().is_empty());

            let toasts = store.toasts().get_untracked();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, id);
            assert!(toasts[0].open.get_untracked());
        });
    }

    #[test]
    fn store_overflow_marks_oldest_closing_and_rotates_to_end() {
        with_store(2, |store| {
            let id1 = store.push.run(ToastOptions::simple("One"));
            let id2 = store.push.run(ToastOptions::simple("Two"));
            let id3 = store.push.run(ToastOptions::simple("Three"));

            let toasts = store.toasts().get_untracked();
            assert_eq!(toasts.len(), 3);

            assert_eq!(toasts[0].id, id2);
            assert_eq!(toasts[1].id, id3);
            assert_eq!(toasts[2].id, id1);

            assert!(toasts[0].open.get_untracked());
            assert!(toasts[1].open.get_untracked());
            assert!(!toasts[2].open.get_untracked());
        });
    }

    #[test]
    fn store_dismiss_marks_toast_closed() {
        with_store(3, |store| {
            let id1 = store.push.run(ToastOptions::simple("One"));
            let id2 = store.push.run(ToastOptions::simple("Two"));

            store.dismiss.run(id1.clone());

            let toasts = store.toasts().get_untracked();
            let t1 = toasts.iter().find(|t| t.id == id1).unwrap();
            let t2 = toasts.iter().find(|t| t.id == id2).unwrap();
            assert!(!t1.open.get_untracked());
            assert!(t2.open.get_untracked());
        });
    }

    #[test]
    fn store_clear_marks_all_toasts_closed() {
        with_store(3, |store| {
            store.push.run(ToastOptions::simple("One"));
            store.push.run(ToastOptions::simple("Two"));

            store.clear.run(());

            let toasts = store.toasts().get_untracked();
            assert!(!toasts.is_empty());
            for toast in toasts {
                assert!(!toast.open.get_untracked());
            }
        });
    }

    #[test]
    fn store_remove_drops_toast_by_id() {
        with_store(3, |store| {
            let id1 = store.push.run(ToastOptions::simple("One"));
            let id2 = store.push.run(ToastOptions::simple("Two"));

            store.remove(&id1);

            let toasts = store.toasts().get_untracked();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, id2);
        });
    }

    #[test]
    fn toast_options_helper_sets_defaults() {
        let opts = ToastOptions::simple("Hello");
        assert_eq!(opts.title, "Hello");
        assert_eq!(opts.variant, ToastVariant::Default);
        assert!(opts.duration_ms.is_some());
    }

    #[test]
    fn normalize_helpers_trim_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-toast  ".to_string())),
            Some("docs-toast".to_string())
        );

        assert_eq!(normalize_title("  Saved  ".to_string()), "Saved");
        assert_eq!(normalize_title("\n\t".to_string()), "Notification");
        assert_eq!(
            normalize_description(Some("  done  ".to_string())),
            Some("done".to_string())
        );
    }

    #[test]
    fn compose_toast_class_name_tracks_state_markers() {
        let class_name = compose_class_name(
            Some("docs-toast-custom".to_string()),
            resolve_state(ToastPartStateInput {
                slot: ToastSlot::Root,
                variant: ToastVariant::Accent,
                is_open: false,
                has_description: true,
                has_custom_id: true,
                has_custom_description: true,
                has_custom_class_name: true,
                has_custom_motion: true,
                has_custom_on_close: true,
                has_custom_on_exit_complete: true,
            }),
        );

        for token in [
            "ui-toast",
            "ui-toast--variant-accent",
            "ui-toast--closing",
            "ui-toast--with-description",
            "ui-toast--custom-class",
            "docs-toast-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
