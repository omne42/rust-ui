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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        (value, true)
    } else {
        (DEFAULT_ARIA_LABEL.to_string(), false)
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
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_guard_limits() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-sonner ".to_string())),
            Some("docs-sonner".to_string())
        );

        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some(" Status host ".to_string())),
            ("Status host".to_string(), true)
        );

        assert_eq!(normalize_max_toasts(0), 1);
        assert_eq!(normalize_max_toasts(2), 2);
    }

    #[test]
    fn queue_and_state_markers_follow_contract() {
        assert_eq!(state_attr(true), "portal");
        assert_eq!(state_attr(false), "inline");

        assert_eq!(queue_attr(1), "single");
        assert_eq!(queue_attr(3), "bounded");
        assert_eq!(queue_attr(6), "extended");
    }

    #[test]
    fn resolve_state_tracks_state_sources_and_store_origin() {
        let state = resolve_state(SonnerPartStateInput {
            slot: SonnerSlot::Root,
            position: SonnerPosition::TopCenter,
            portal: false,
            max_toasts: 0,
            has_custom_position: true,
            has_custom_portal: true,
            has_custom_max_toasts: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            store_source: SonnerStoreSource::Provided,
        });

        assert_eq!(state.slot_attr, "sonner");
        assert_eq!(state.base_class, "ui-sonner");
        assert_eq!(state.position_attr, "top-center");
        assert_eq!(state.state_attr, "inline");
        assert_eq!(state.portal_attr, "false");
        assert_eq!(state.max_toasts, 1);
        assert_eq!(state.queue_attr, "single");
        assert_eq!(state.position_source_attr, "custom");
        assert_eq!(state.portal_source_attr, "custom");
        assert_eq!(state.max_toasts_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.store_source_attr, "provided");
    }
}
