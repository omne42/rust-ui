use crate::close_button::DEFAULT_ARIA_LABEL;
use crate::toast::{
    ToastMotion, ToastPartState, ToastPartStateInput, ToastSlot, ToastStoreSource, ToastVariant,
    ToastViewportSlot, ToastViewportState, ToastViewportStateInput,
};
use leptos::prelude::*;
use ui_headless::{LiveRegionPriority, OnPress};
use ui_state_primitives::toast as toast_state;

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
pub const DEFAULT_OPEN: bool = true;
pub const DEFAULT_VIEWPORT_PORTAL: bool = true;
pub const DEFAULT_VIEWPORT_MAX_TOASTS: usize = toast_state::DEFAULT_MAX_TOASTS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastAgentIntent {
    NotificationItem,
    NotificationViewport,
}

impl ToastAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::NotificationItem => "notification-item",
            Self::NotificationViewport => "notification-viewport",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastAgentActionModel {
    DismissClose,
    QueueDismissRemove,
}

impl ToastAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::DismissClose => "dismiss|close",
            Self::QueueDismissRemove => "queue|dismiss|remove",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastAgentContract {
    pub schema_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn toast_agent_contract() -> ToastAgentContract {
    ToastAgentContract {
        schema_attr: "ui.toast.v1",
        intent_attr: ToastAgentIntent::NotificationItem.as_attr(),
        action_model_attr: ToastAgentActionModel::DismissClose.as_attr(),
        state_axis_attr: "state|variant|description|close-mode|open",
        source_axis_attr: "id|description|class|motion|close|exit|open",
    }
}

pub fn toast_viewport_agent_contract() -> ToastAgentContract {
    ToastAgentContract {
        schema_attr: "ui.toast.viewport.v1",
        intent_attr: ToastAgentIntent::NotificationViewport.as_attr(),
        action_model_attr: ToastAgentActionModel::QueueDismissRemove.as_attr(),
        state_axis_attr: "state|queue|portal|max-toasts",
        source_axis_attr: "portal|max-toasts|class|motion|store",
    }
}

pub fn toast_state_attr(is_open: bool) -> &'static str {
    toast_state::toast_state_attr(is_open)
}

pub fn description_attr(has_description: bool) -> &'static str {
    toast_state::description_attr(has_description)
}

pub fn close_mode_attr(has_on_close: bool) -> &'static str {
    toast_state::close_mode_attr(has_on_close)
}

pub fn viewport_state_attr(portal: bool) -> &'static str {
    toast_state::viewport_state_attr(portal)
}

pub fn viewport_queue_attr(max_toasts: usize) -> &'static str {
    toast_state::viewport_queue_attr(max_toasts)
}

pub fn normalize_viewport_max_toasts(max_toasts: usize) -> usize {
    toast_state::normalize_max_toasts(max_toasts)
}

pub fn resolve_state(input: ToastPartStateInput) -> ToastPartState {
    let state = toast_state::resolve_state(input);
    ToastPartState {
        state_attr: toast_state_attr(state.is_open),
        description_attr: description_attr(state.has_description),
        close_mode_attr: close_mode_attr(state.has_custom_on_close),
        ..state
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastStateDerivationInput {
    pub variant: ToastVariant,
    pub is_open: bool,
    pub has_description: bool,
    pub has_custom_id: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_on_close: bool,
    pub has_custom_on_exit_complete: bool,
}

pub fn resolve_toast_part_state(input: ToastStateDerivationInput) -> ToastPartState {
    resolve_state(ToastPartStateInput {
        slot: ToastSlot::Root,
        variant: input.variant,
        is_open: input.is_open,
        has_description: input.has_description,
        has_custom_id: input.has_custom_id,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_custom_on_close: input.has_custom_on_close,
        has_custom_on_exit_complete: input.has_custom_on_exit_complete,
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: ToastPartState) -> String {
    let mut classes = vec![state.base_class.into(), state.variant.class_name().into()];

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
    let state = toast_state::resolve_viewport_state(input);
    ToastViewportState {
        state_attr: viewport_state_attr(state.portal),
        queue_attr: viewport_queue_attr(state.max_toasts),
        ..state
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportStateDerivationInput {
    pub is_portal: bool,
    pub max_toasts: usize,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: ToastStoreSource,
}

pub fn resolve_toast_viewport_state(
    input: ToastViewportStateDerivationInput,
) -> ToastViewportState {
    resolve_viewport_state(ToastViewportStateInput {
        slot: ToastViewportSlot::Root,
        portal: input.is_portal,
        max_toasts: input.max_toasts,
        has_custom_portal: input.has_custom_portal,
        has_custom_max_toasts: input.has_custom_max_toasts,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        store_source: input.store_source,
    })
}

pub fn compose_viewport_class_name(
    base_class_name: Option<String>,
    state: ToastViewportState,
) -> String {
    let mut classes = vec![state.base_class.into()];

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
    toast_state::normalize_optional_text(value)
}

pub fn normalize_title(value: String) -> String {
    toast_state::normalize_title(value, DEFAULT_TITLE)
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    toast_state::normalize_description(value)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastNormalizeInput {
    pub title: String,
    pub id: Option<String>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    pub motion: ToastMotion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastNormalizedProps {
    pub title: String,
    pub id: Option<String>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    pub has_custom_id: bool,
    pub has_description: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_props(input: ToastNormalizeInput) -> ToastNormalizedProps {
    let id = normalize_optional_text(input.id);
    let description = normalize_description(input.description);
    let class_name = normalize_optional_text(input.class_name);
    let has_description = description.is_some();

    ToastNormalizedProps {
        title: normalize_title(input.title),
        has_custom_id: id.is_some(),
        has_custom_description: has_description,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion: input.motion != ToastMotion::default(),
        id,
        description,
        class_name,
        has_description,
    }
}

#[derive(Clone)]
pub struct ToastOpenStateConfig {
    pub controlled_open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub open_source_attr: &'static str,
}

pub fn resolve_open_state_config(
    is_open: Option<Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
) -> ToastOpenStateConfig {
    let (controlled_open, open_source_attr) = if let Some(is_open) = is_open {
        (Some(is_open), "is_open")
    } else {
        (None, "implicit")
    };

    let has_custom_default_open = default_open.is_some();
    let has_custom_on_open_change = on_open_change.is_some();

    ToastOpenStateConfig {
        is_controlled: controlled_open.is_some(),
        controlled_open,
        default_open: default_open.or(Some(DEFAULT_OPEN)),
        on_open_change,
        has_custom_default_open,
        has_custom_on_open_change,
        open_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastOpenStateMarkers {
    pub control_mode_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn resolve_open_state_markers(config: &ToastOpenStateConfig) -> ToastOpenStateMarkers {
    ToastOpenStateMarkers {
        control_mode_attr: if config.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        default_open_source_attr: if config.has_custom_default_open {
            "provided"
        } else {
            "implicit"
        },
        open_change_source_attr: if config.has_custom_on_open_change {
            "provided"
        } else {
            "none"
        },
    }
}

#[derive(Clone)]
pub struct ToastCallbacksConfig {
    pub on_close: OnPress,
    pub on_exit_complete: Callback<()>,
    pub has_custom_on_close: bool,
    pub has_custom_on_exit_complete: bool,
}

pub fn resolve_callbacks_config(
    on_close: Option<OnPress>,
    on_exit_complete: Option<Callback<()>>,
) -> ToastCallbacksConfig {
    let has_custom_on_close = on_close.is_some();
    let has_custom_on_exit_complete = on_exit_complete.is_some();

    ToastCallbacksConfig {
        on_close: on_close.unwrap_or_else(|| Callback::new(|_| {})),
        on_exit_complete: on_exit_complete.unwrap_or_else(|| Callback::new(|_| {})),
        has_custom_on_close,
        has_custom_on_exit_complete,
    }
}

pub fn resolve_close_aria_label(close_aria_label: Option<String>, default_label: &str) -> String {
    normalize_optional_text(close_aria_label)
        .or_else(|| normalize_optional_text(Some(default_label.to_string())))
        .unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string())
}

pub fn resolve_live_region_priority(variant: ToastVariant) -> LiveRegionPriority {
    match variant {
        ToastVariant::Danger => LiveRegionPriority::Assertive,
        ToastVariant::Default | ToastVariant::Accent => LiveRegionPriority::Polite,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportConfig {
    pub is_portal: bool,
    pub max_toasts: usize,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
}

pub fn resolve_viewport_config(is_portal: bool, max_toasts: usize) -> ToastViewportConfig {
    ToastViewportConfig {
        is_portal,
        max_toasts,
        has_custom_portal: is_portal != DEFAULT_VIEWPORT_PORTAL,
        has_custom_max_toasts: max_toasts != DEFAULT_VIEWPORT_MAX_TOASTS,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastViewportNormalizeInput {
    pub is_portal: bool,
    pub max_toasts: usize,
    pub class_name: Option<String>,
    pub motion: ToastMotion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastViewportNormalizedProps {
    pub viewport: ToastViewportConfig,
    pub class_name: Option<String>,
    pub normalized_max_toasts: usize,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

pub fn normalize_viewport_props(
    input: ToastViewportNormalizeInput,
) -> ToastViewportNormalizedProps {
    let viewport = resolve_viewport_config(input.is_portal, input.max_toasts);
    let class_name = normalize_optional_text(input.class_name);

    ToastViewportNormalizedProps {
        normalized_max_toasts: normalize_viewport_max_toasts(viewport.max_toasts),
        has_custom_class_name: class_name.is_some(),
        has_custom_motion: input.motion != ToastMotion::default(),
        viewport,
        class_name,
    }
}

pub fn resolve_viewport_store(
    store: Option<ToastStore>,
    max_toasts: usize,
) -> (ToastStore, ToastStoreSource) {
    if let Some(provided_store) = store {
        (provided_store, ToastStoreSource::Provided)
    } else if let Some(context_store) = use_toast_store() {
        (context_store, ToastStoreSource::Context)
    } else {
        (
            provide_toast_store(ToastStoreOptions { max_toasts }),
            ToastStoreSource::Local,
        )
    }
}

pub fn resolve_instance_open(toasts: &[ToastInstance], id: &str) -> bool {
    toasts
        .iter()
        .find(|toast| toast.id == id)
        .map(|toast| toast.is_open)
        .unwrap_or(false)
}

pub fn resolve_instance_description(description: Option<String>) -> String {
    description.unwrap_or_default()
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ToastPayload {
    title: String,
    description: Option<String>,
    variant: ToastVariant,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastInstance {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub is_open: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ToastStoreOptions {
    pub max_toasts: usize,
}

impl Default for ToastStoreOptions {
    fn default() -> Self {
        Self {
            max_toasts: DEFAULT_VIEWPORT_MAX_TOASTS,
        }
    }
}

fn to_primitive_records(list: &[ToastInstance]) -> Vec<toast_state::ToastRecord<ToastPayload>> {
    list.iter()
        .map(|toast| toast_state::ToastRecord {
            id: toast.id.clone(),
            payload: ToastPayload {
                title: toast.title.clone(),
                description: toast.description.clone(),
                variant: toast.variant,
            },
            is_open: toast.is_open,
        })
        .collect()
}

fn from_primitive_records(
    records: Vec<toast_state::ToastRecord<ToastPayload>>,
) -> Vec<ToastInstance> {
    records
        .into_iter()
        .map(|record| ToastInstance {
            id: record.id,
            title: record.payload.title,
            description: record.payload.description,
            variant: record.payload.variant,
            is_open: record.is_open,
        })
        .collect()
}

fn dismiss_timeout_ids(mutations: &[toast_state::ToastMutation]) -> Vec<String> {
    mutations
        .iter()
        .filter_map(|mutation| match mutation.kind {
            toast_state::ToastMutationKind::Pushed => None,
            _ => Some(mutation.id.clone()),
        })
        .collect()
}

#[derive(Clone)]
pub struct ToastStore {
    max_toasts: usize,
    toasts: ReadSignal<Vec<ToastInstance>>,
    set_toasts: WriteSignal<Vec<ToastInstance>>,
    #[cfg(target_arch = "wasm32")]
    timeouts: StoredValue<std::collections::HashMap<String, TimeoutHandle>, LocalStorage>,
    pub push: Callback<ToastOptions, String>,
    pub dismiss: Callback<String>,
    pub clear: Callback<()>,
}

pub fn provide_toast_store(options: ToastStoreOptions) -> ToastStore {
    let max_toasts = toast_state::normalize_max_toasts(options.max_toasts);
    let (toasts, set_toasts) = signal(Vec::<ToastInstance>::new());

    #[cfg(target_arch = "wasm32")]
    let timeouts: StoredValue<std::collections::HashMap<String, TimeoutHandle>, LocalStorage> =
        StoredValue::new_local(std::collections::HashMap::new());

    let push = Callback::new({
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |opts: ToastOptions| -> String {
            let id = format!("ui-toast-{}", next_id());
            let payload = ToastPayload {
                title: opts.title,
                description: opts.description,
                variant: opts.variant,
            };
            let mut mutations: Vec<toast_state::ToastMutation> = Vec::new();
            set_toasts.update(|list| {
                let mut state = toast_state::ToastState::from_records(
                    toast_state::ToastStateOptions { max_toasts },
                    to_primitive_records(list),
                );
                mutations = state.push(id.clone(), payload.clone());
                *list = from_primitive_records(state.into_records());
            });

            #[cfg(target_arch = "wasm32")]
            {
                let mut map = timeouts.get_value();
                for toast_id in dismiss_timeout_ids(&mutations) {
                    if let Some(handle) = map.remove(&toast_id) {
                        handle.clear();
                    }
                }
                timeouts.set_value(map);
            }

            #[cfg(target_arch = "wasm32")]
            if let Some(duration_ms) = opts.duration_ms.filter(|v| *v > 0) {
                let id_for_timeout = id.clone();
                let Ok(handle) = set_timeout_with_handle(
                    {
                        let set_toasts = set_toasts;
                        move || {
                            set_toasts.update(|list| {
                                let mut state = toast_state::ToastState::from_records(
                                    toast_state::ToastStateOptions { max_toasts },
                                    to_primitive_records(list),
                                );
                                drop(state.dismiss(&id_for_timeout));
                                *list = from_primitive_records(state.into_records());
                            });
                        }
                    },
                    std::time::Duration::from_millis(duration_ms),
                ) else {
                    return id;
                };

                let mut map = timeouts.get_value();
                if let Some(prev) = map.remove(&id) {
                    prev.clear();
                }
                map.insert(id.clone(), handle);
                timeouts.set_value(map);
            }

            id
        }
    });

    let dismiss = Callback::new({
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |id: String| {
            let mut dismissed: Option<String> = None;
            set_toasts.update(|list| {
                let mut state = toast_state::ToastState::from_records(
                    toast_state::ToastStateOptions { max_toasts },
                    to_primitive_records(list),
                );
                if let Some(mutation) = state.dismiss(&id) {
                    dismissed = Some(mutation.id);
                }
                *list = from_primitive_records(state.into_records());
            });

            #[cfg(target_arch = "wasm32")]
            if let Some(dismissed) = dismissed {
                let mut map = timeouts.get_value();
                if let Some(handle) = map.remove(&dismissed) {
                    handle.clear();
                }
                timeouts.set_value(map);
            }
        }
    });

    let clear = Callback::new({
        #[cfg(target_arch = "wasm32")]
        let timeouts = timeouts;
        move |_| {
            let mut cleared_ids = Vec::new();
            set_toasts.update(|list| {
                let mut state = toast_state::ToastState::from_records(
                    toast_state::ToastStateOptions { max_toasts },
                    to_primitive_records(list),
                );
                let mutations = state.clear();
                cleared_ids = dismiss_timeout_ids(&mutations);
                *list = from_primitive_records(state.into_records());
            });

            #[cfg(target_arch = "wasm32")]
            {
                let mut map = timeouts.get_value();
                for id in cleared_ids {
                    if let Some(handle) = map.remove(&id) {
                        handle.clear();
                    }
                }
                timeouts.set_value(map);
            }
        }
    });

    let store = ToastStore {
        max_toasts,
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
        let mut removed: Option<String> = None;
        self.set_toasts.update(|list| {
            let mut state = toast_state::ToastState::from_records(
                toast_state::ToastStateOptions {
                    max_toasts: self.max_toasts,
                },
                to_primitive_records(list),
            );
            if let Some(mutation) = state.remove(id) {
                removed = Some(mutation.id);
            }
            *list = from_primitive_records(state.into_records());
        });

        #[cfg(target_arch = "wasm32")]
        if let Some(removed) = removed {
            let mut map = self.timeouts.get_value();
            if let Some(handle) = map.remove(&removed) {
                handle.clear();
            }
            self.timeouts.set_value(map);
        }
    }
}

#[cfg(test)]
#[path = "../../test/toast/logic.rs"]
mod tests;
