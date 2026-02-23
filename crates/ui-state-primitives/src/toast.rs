pub const DEFAULT_MAX_TOASTS: usize = 3;
pub const DEFAULT_TITLE: &str = "Notification";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastRecord<T> {
    pub id: String,
    pub payload: T,
    pub is_open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastStateOptions {
    pub max_toasts: usize,
}

impl Default for ToastStateOptions {
    fn default() -> Self {
        Self {
            max_toasts: DEFAULT_MAX_TOASTS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastMutationKind {
    Pushed,
    OverflowClosed,
    Dismissed,
    Cleared,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastMutation {
    pub id: String,
    pub kind: ToastMutationKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastState<T> {
    max_toasts: usize,
    toasts: Vec<ToastRecord<T>>,
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_title(value: String, default_title: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        default_title.trim().into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastSlot {
    Root,
    Content,
    Title,
    Description,
    Close,
}

impl ToastSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastSlot::Root => "toast",
            ToastSlot::Content => "toast-content",
            ToastSlot::Title => "toast-title",
            ToastSlot::Description => "toast-description",
            ToastSlot::Close => "toast-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ToastSlot::Root => "ui-toast",
            ToastSlot::Content => "ui-toast__content",
            ToastSlot::Title => "ui-toast__title",
            ToastSlot::Description => "ui-toast__description",
            ToastSlot::Close => "ui-toast__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastPartStateInput {
    pub slot: ToastSlot,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastPartState {
    pub slot: ToastSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub variant: ToastVariant,
    pub variant_attr: &'static str,
    pub description_attr: &'static str,
    pub close_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub is_open: bool,
    pub has_description: bool,
    pub has_custom_id: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_on_close: bool,
    pub has_custom_on_exit_complete: bool,
    pub id_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastViewportSlot {
    Root,
}

impl ToastViewportSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastViewportSlot::Root => "toast-viewport",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ToastViewportSlot::Root => "ui-toast-viewport",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastStoreSource {
    Provided,
    Context,
    Local,
}

impl ToastStoreSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToastStoreSource::Provided => "provided",
            ToastStoreSource::Context => "context",
            ToastStoreSource::Local => "local",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportStateInput {
    pub slot: ToastViewportSlot,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: ToastStoreSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastViewportState {
    pub slot: ToastViewportSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub queue_attr: &'static str,
    pub portal_attr: &'static str,
    pub max_toasts: usize,
    pub portal: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub portal_source_attr: &'static str,
    pub max_toasts_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub store_source: ToastStoreSource,
    pub store_source_attr: &'static str,
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

pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState {
    let max_toasts = normalize_max_toasts(input.max_toasts);

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

impl<T> ToastState<T> {
    pub fn new(options: ToastStateOptions) -> Self {
        Self {
            max_toasts: normalize_max_toasts(options.max_toasts),
            toasts: Vec::new(),
        }
    }

    pub fn from_records(options: ToastStateOptions, records: Vec<ToastRecord<T>>) -> Self {
        Self {
            max_toasts: normalize_max_toasts(options.max_toasts),
            toasts: records,
        }
    }

    pub fn max_toasts(&self) -> usize {
        self.max_toasts
    }

    pub fn toasts(&self) -> &[ToastRecord<T>] {
        &self.toasts
    }

    pub fn into_records(self) -> Vec<ToastRecord<T>> {
        self.toasts
    }

    pub fn push(&mut self, id: String, payload: T) -> Vec<ToastMutation> {
        let mut mutations = vec![ToastMutation {
            id: id.clone(),
            kind: ToastMutationKind::Pushed,
        }];

        self.toasts.push(ToastRecord {
            id,
            payload,
            is_open: true,
        });

        let overflow = self.toasts.len().saturating_sub(self.max_toasts);
        for _ in 0..overflow {
            if self.toasts.is_empty() {
                break;
            }

            if self.toasts[0].is_open {
                self.toasts[0].is_open = false;
                mutations.push(ToastMutation {
                    id: self.toasts[0].id.clone(),
                    kind: ToastMutationKind::OverflowClosed,
                });
            }

            let oldest = self.toasts.remove(0);
            self.toasts.push(oldest);
        }

        mutations
    }

    pub fn dismiss(&mut self, id: &str) -> Option<ToastMutation> {
        let id = id.trim();
        if id.is_empty() {
            return None;
        }

        let toast = self.toasts.iter_mut().find(|toast| toast.id == id)?;
        if !toast.is_open {
            return None;
        }

        toast.is_open = false;
        Some(ToastMutation {
            id: toast.id.clone(),
            kind: ToastMutationKind::Dismissed,
        })
    }

    pub fn clear(&mut self) -> Vec<ToastMutation> {
        let mut mutations = Vec::new();

        for toast in &mut self.toasts {
            if !toast.is_open {
                continue;
            }

            toast.is_open = false;
            mutations.push(ToastMutation {
                id: toast.id.clone(),
                kind: ToastMutationKind::Cleared,
            });
        }

        mutations
    }

    pub fn remove(&mut self, id: &str) -> Option<ToastMutation> {
        let id = id.trim();
        if id.is_empty() {
            return None;
        }

        let index = self.toasts.iter().position(|toast| toast.id == id)?;
        let removed = self.toasts.remove(index);

        Some(ToastMutation {
            id: removed.id,
            kind: ToastMutationKind::Removed,
        })
    }
}

#[cfg(test)]
#[path = "test/toast.rs"]
mod tests;
