const DEFAULT_ARIA_LABEL: &str = "Sidebar";
const DEFAULT_SHORTCUT_KEY: &str = "b";

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

pub fn normalize_trigger_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| "Toggle sidebar".to_string())
}

pub fn normalize_default_open(value: Option<bool>) -> bool {
    value.unwrap_or(true)
}

pub fn resolve_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn resolve_trigger_visibility(is_trigger_visible: Option<bool>, show_trigger: bool) -> bool {
    is_trigger_visible.unwrap_or(show_trigger)
}

pub fn resolve_shortcut_enabled(is_shortcut_enabled: Option<bool>, enable_shortcut: bool) -> bool {
    is_shortcut_enabled.unwrap_or(enable_shortcut)
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
#[path = "../test/logic.rs"]
mod tests;
