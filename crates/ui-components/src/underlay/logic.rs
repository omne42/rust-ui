use crate::underlay::{UnderlayPartState, UnderlayPartStateInput};

pub const DEFAULT_OPEN: bool = false;
pub const DEFAULT_TRANSPARENT: bool = false;
pub const DEFAULT_DISABLED: bool = false;

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn state_attr(is_open: bool, is_disabled: bool) -> &'static str {
    if is_disabled {
        "disabled"
    } else if is_open {
        "open"
    } else {
        "closed"
    }
}

pub fn tone_attr(is_transparent: bool) -> &'static str {
    if is_transparent {
        "transparent"
    } else {
        "scrim"
    }
}

pub fn close_mode_attr(is_interactive: bool) -> &'static str {
    if is_interactive {
        "interactive"
    } else {
        "static"
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState {
    let is_open = input.open && !input.disabled;
    let is_interactive = is_open && input.has_on_close;

    UnderlayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(is_open, input.disabled),
        tone_attr: tone_attr(input.transparent),
        close_mode_attr: close_mode_attr(is_interactive),
        open_attr: is_open.then_some("true"),
        transparent_attr: input.transparent.then_some("true"),
        disabled_attr: input.disabled.then_some("true"),
        interactive_attr: is_interactive.then_some("true"),
        is_open,
        is_transparent: input.transparent,
        is_disabled: input.disabled,
        is_interactive,
        has_custom_transparent: input.has_custom_transparent,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_close_handler: input.has_custom_close_handler,
        has_custom_class_name: input.has_custom_class_name,
        transparent_source_attr: source_attr(input.has_custom_transparent),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        close_source_attr: source_attr(input.has_custom_close_handler),
        class_source_attr: source_attr(input.has_custom_class_name),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayPartState) -> String {
    let mut classes = vec![state.base_class.to_string()];

    if state.is_open {
        classes.push("ui-underlay--open".to_string());
    }

    if state.is_transparent {
        classes.push("ui-underlay--transparent".to_string());
    }

    if state.is_disabled {
        classes.push("ui-underlay--disabled".to_string());
    }

    if state.is_interactive {
        classes.push("ui-underlay--interactive".to_string());
    }

    if state.has_custom_transparent {
        classes.push("ui-underlay--custom-transparent".to_string());
    }

    if state.has_custom_disabled {
        classes.push("ui-underlay--custom-disabled".to_string());
    }

    if state.has_custom_close_handler {
        classes.push("ui-underlay--custom-close".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-underlay--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::underlay::UnderlaySlot;

    #[test]
    fn normalize_helpers_trim_text_and_filter_blanks() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-underlay ".to_string())),
            Some("docs-underlay".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_state_tone_and_source_markers() {
        let state = resolve_state(UnderlayPartStateInput {
            slot: UnderlaySlot::Root,
            open: true,
            transparent: true,
            disabled: false,
            has_on_close: true,
            has_custom_transparent: true,
            has_custom_disabled: false,
            has_custom_close_handler: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.slot_attr, "underlay");
        assert_eq!(state.base_class, "ui-underlay");
        assert_eq!(state.state_attr, "open");
        assert_eq!(state.tone_attr, "transparent");
        assert_eq!(state.close_mode_attr, "interactive");
        assert_eq!(state.open_attr, Some("true"));
        assert_eq!(state.transparent_attr, Some("true"));
        assert_eq!(state.disabled_attr, None);
        assert_eq!(state.interactive_attr, Some("true"));
        assert_eq!(state.transparent_source_attr, "custom");
        assert_eq!(state.disabled_source_attr, "default");
        assert_eq!(state.close_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(UnderlayPartStateInput {
            slot: UnderlaySlot::Root,
            open: true,
            transparent: false,
            disabled: true,
            has_on_close: true,
            has_custom_transparent: false,
            has_custom_disabled: true,
            has_custom_close_handler: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-underlay".to_string()), state);
        assert!(class_name.contains("ui-underlay"));
        assert!(class_name.contains("ui-underlay--disabled"));
        assert!(class_name.contains("ui-underlay--custom-disabled"));
        assert!(class_name.contains("ui-underlay--custom-close"));
        assert!(class_name.contains("ui-underlay--custom-class"));
        assert!(class_name.contains("docs-underlay"));
    }

    #[test]
    fn state_and_close_mode_helpers_cover_all_paths() {
        assert_eq!(state_attr(false, false), "closed");
        assert_eq!(state_attr(true, false), "open");
        assert_eq!(state_attr(true, true), "disabled");

        assert_eq!(tone_attr(false), "scrim");
        assert_eq!(tone_attr(true), "transparent");

        assert_eq!(close_mode_attr(false), "static");
        assert_eq!(close_mode_attr(true), "interactive");
    }
}
