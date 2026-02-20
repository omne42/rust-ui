pub const DEFAULT_ARIA_LABEL: &str = "Notifications";
pub const DEFAULT_PORTAL: bool = true;
pub const DEFAULT_MAX_TOASTS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl SonnerPosition {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerPosition::TopLeft => "top-left",
            SonnerPosition::TopCenter => "top-center",
            SonnerPosition::TopRight => "top-right",
            SonnerPosition::BottomLeft => "bottom-left",
            SonnerPosition::BottomCenter => "bottom-center",
            SonnerPosition::BottomRight => "bottom-right",
        }
    }

    pub fn class_suffix(self) -> &'static str {
        self.as_attr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerSlot {
    Root,
    Viewport,
}

impl SonnerSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerSlot::Root => "sonner",
            SonnerSlot::Viewport => "sonner-viewport",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            SonnerSlot::Root => "ui-sonner",
            SonnerSlot::Viewport => "ui-sonner__viewport",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SonnerStoreSource {
    Provided,
    Context,
    Local,
}

impl SonnerStoreSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerStoreSource::Provided => "provided",
            SonnerStoreSource::Context => "context",
            SonnerStoreSource::Local => "local",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerPartStateInput {
    pub slot: SonnerSlot,
    pub position: SonnerPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub store_source: SonnerStoreSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerPartState {
    pub slot: SonnerSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub queue_attr: &'static str,
    pub position: SonnerPosition,
    pub position_attr: &'static str,
    pub portal: bool,
    pub portal_attr: &'static str,
    pub max_toasts: usize,
    pub has_custom_position: bool,
    pub has_custom_portal: bool,
    pub has_custom_max_toasts: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub position_source_attr: &'static str,
    pub portal_source_attr: &'static str,
    pub max_toasts_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub store_source: SonnerStoreSource,
    pub store_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        (value, true)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn state_attr(portal: bool) -> &'static str {
    if portal { "portal" } else { "inline" }
}

pub fn queue_attr(max_toasts: usize) -> &'static str {
    if max_toasts <= 1 {
        "single"
    } else if max_toasts <= 3 {
        "bounded"
    } else {
        "extended"
    }
}

pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState {
    let normalized_max_toasts = normalize_max_toasts(input.max_toasts);

    SonnerPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.portal),
        queue_attr: queue_attr(normalized_max_toasts),
        position: input.position,
        position_attr: input.position.as_attr(),
        portal: input.portal,
        portal_attr: if input.portal { "true" } else { "false" },
        max_toasts: normalized_max_toasts,
        has_custom_position: input.has_custom_position,
        has_custom_portal: input.has_custom_portal,
        has_custom_max_toasts: input.has_custom_max_toasts,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        position_source_attr: source_attr(input.has_custom_position),
        portal_source_attr: source_attr(input.has_custom_portal),
        max_toasts_source_attr: source_attr(input.has_custom_max_toasts),
        aria_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        store_source: input.store_source,
        store_source_attr: input.store_source.as_attr(),
    }
}

#[cfg(test)]
#[path = "test/sonner.rs"]
mod tests;
