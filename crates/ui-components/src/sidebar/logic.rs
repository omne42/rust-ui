use crate::sidebar::{DEFAULT_ARIA_LABEL, DEFAULT_SHORTCUT_KEY};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarStateInput {
    pub side: SidebarSide,
    pub variant: SidebarVariant,
    pub collapsible: SidebarCollapsible,
    pub open: bool,
    pub disabled: bool,
    pub is_controlled: bool,
    pub show_trigger: bool,
    pub has_shortcut_key: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarState {
    pub side: SidebarSide,
    pub side_attr: &'static str,
    pub variant: SidebarVariant,
    pub variant_attr: &'static str,
    pub collapsible: SidebarCollapsible,
    pub collapsible_attr: &'static str,
    pub open: bool,
    pub closed: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub show_trigger: bool,
    pub show_rail: bool,
    pub has_shortcut_key: bool,
    pub has_custom_class_name: bool,
    pub state_attr: &'static str,
    pub control_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.into())
}

pub fn normalize_default_open(value: Option<bool>) -> bool {
    value.unwrap_or(true)
}

pub fn normalize_shortcut_key(value: Option<String>, enable_shortcut: bool) -> Option<String> {
    if !enable_shortcut {
        return None;
    }

    let normalized = normalize_optional_text(value)
        .unwrap_or_else(|| DEFAULT_SHORTCUT_KEY.into())
        .to_ascii_lowercase();

    let mut chars = normalized.chars();
    let first = chars.next()?;

    if chars.next().is_some() {
        return Some(first.to_string());
    }

    Some(first.to_string())
}

pub fn shortcut_hint(shortcut_key: Option<String>) -> Option<String> {
    shortcut_key.map(|shortcut_key| format!("Ctrl+{shortcut_key}"))
}

pub fn should_toggle_for_shortcut(
    key: &str,
    ctrl_key: bool,
    meta_key: bool,
    shortcut_key: Option<&str>,
    disabled: bool,
) -> bool {
    if disabled {
        return false;
    }

    let Some(shortcut_key) = shortcut_key else {
        return false;
    };

    if !(ctrl_key || meta_key) {
        return false;
    }

    key.eq_ignore_ascii_case(shortcut_key)
}

pub fn resolve_state(input: SidebarStateInput) -> SidebarState {
    let closed = !input.open;
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    SidebarState {
        side: input.side,
        side_attr: match input.side {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        },
        variant: input.variant,
        variant_attr: match input.variant {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        },
        collapsible: input.collapsible,
        collapsible_attr: match input.collapsible {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        },
        open: input.open,
        closed,
        disabled: input.disabled,
        enabled,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        show_trigger: input.show_trigger,
        show_rail: input.collapsible != SidebarCollapsible::None,
        has_shortcut_key: input.has_shortcut_key,
        has_custom_class_name: input.has_custom_class_name,
        state_attr: if input.disabled {
            "disabled"
        } else if input.open {
            "open"
        } else {
            "closed"
        },
        control_attr: if input.show_trigger {
            "trigger"
        } else {
            "manual"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SidebarState) -> String {
    let mut classes = vec!["ui-sidebar".to_string()];

    match state.side {
        SidebarSide::Left => classes.push("ui-sidebar--left".to_string()),
        SidebarSide::Right => classes.push("ui-sidebar--right".to_string()),
    }

    match state.variant {
        SidebarVariant::Sidebar => classes.push("ui-sidebar--variant-sidebar".to_string()),
        SidebarVariant::Floating => classes.push("ui-sidebar--variant-floating".to_string()),
        SidebarVariant::Inset => classes.push("ui-sidebar--variant-inset".to_string()),
    }

    match state.collapsible {
        SidebarCollapsible::Offcanvas => classes.push("ui-sidebar--offcanvas".to_string()),
        SidebarCollapsible::Icon => classes.push("ui-sidebar--icon".to_string()),
        SidebarCollapsible::None => classes.push("ui-sidebar--static".to_string()),
    }

    if state.open {
        classes.push("ui-sidebar--open".to_string());
    } else {
        classes.push("ui-sidebar--closed".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar--disabled".to_string());
    }

    if state.show_trigger {
        classes.push("ui-sidebar--with-trigger".to_string());
    }

    if state.has_shortcut_key {
        classes.push("ui-sidebar--with-shortcut".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar--controlled".to_string());
    } else {
        classes.push("ui-sidebar--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_shortcut_key_defaults_and_trims() {
        assert_eq!(
            normalize_shortcut_key(None, true),
            Some(DEFAULT_SHORTCUT_KEY.into())
        );
        assert_eq!(
            normalize_shortcut_key(Some("  K  ".to_string()), true),
            Some("k".to_string())
        );
        assert_eq!(
            normalize_shortcut_key(Some("".to_string()), true),
            Some("b".to_string())
        );
    }

    #[test]
    fn should_toggle_for_shortcut_requires_modifier_and_match() {
        assert!(should_toggle_for_shortcut(
            "b",
            true,
            false,
            Some("b"),
            false
        ));
        assert!(!should_toggle_for_shortcut(
            "b",
            false,
            false,
            Some("b"),
            false,
        ));
        assert!(!should_toggle_for_shortcut(
            "x",
            true,
            false,
            Some("b"),
            false,
        ));
        assert!(!should_toggle_for_shortcut(
            "b",
            true,
            false,
            Some("b"),
            true
        ));
    }

    #[test]
    fn resolve_state_tracks_sidebar_flags_and_attrs() {
        let state = resolve_state(SidebarStateInput {
            side: SidebarSide::Right,
            variant: SidebarVariant::Inset,
            collapsible: SidebarCollapsible::Icon,
            open: false,
            disabled: false,
            is_controlled: true,
            show_trigger: false,
            has_shortcut_key: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.side_attr, "right");
        assert_eq!(state.variant_attr, "inset");
        assert_eq!(state.collapsible_attr, "icon");
        assert_eq!(state.state_attr, "closed");
        assert!(state.closed);
        assert!(state.enabled);
        assert!(state.is_controlled);
        assert!(!state.is_uncontrolled);
        assert_eq!(state.control_attr, "manual");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_classes() {
        let class_name = compose_class_name(
            Some("demo".to_string()),
            resolve_state(SidebarStateInput {
                side: SidebarSide::Left,
                variant: SidebarVariant::Sidebar,
                collapsible: SidebarCollapsible::Offcanvas,
                open: true,
                disabled: true,
                is_controlled: false,
                show_trigger: true,
                has_shortcut_key: true,
                has_custom_class_name: true,
            }),
        );

        for needle in [
            "ui-sidebar",
            "ui-sidebar--left",
            "ui-sidebar--variant-sidebar",
            "ui-sidebar--offcanvas",
            "ui-sidebar--open",
            "ui-sidebar--disabled",
            "ui-sidebar--with-trigger",
            "ui-sidebar--with-shortcut",
            "ui-sidebar--uncontrolled",
            "ui-sidebar--custom-class",
            "demo",
        ] {
            assert!(
                class_name.contains(needle),
                "composed class should contain `{needle}`",
            );
        }
    }
}
