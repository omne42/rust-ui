#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
    Oled,
}

impl ThemeMode {
    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
            ThemeMode::Oled => "OLED",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
            ThemeMode::Oled => "oled",
        }
    }

    pub fn icon(self) -> ThemeToggleIcon {
        match self {
            ThemeMode::Light => ThemeToggleIcon::Sun,
            ThemeMode::Dark => ThemeToggleIcon::Moon,
            ThemeMode::Oled => ThemeToggleIcon::Oled,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeToggleIcon {
    Sun,
    Moon,
    Oled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeToggleViewState {
    pub icon: ThemeToggleIcon,
    pub next: ThemeMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThemeToggleState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub mode_count: usize,
    pub has_custom_modes: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub current_mode: ThemeMode,
    pub current_mode_attr: &'static str,
    pub next_mode: ThemeMode,
    pub next_mode_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_modes(modes: Vec<ThemeMode>) -> Vec<ThemeMode> {
    let mut normalized = Vec::new();

    for mode in modes {
        if !normalized.contains(&mode) {
            normalized.push(mode);
        }
    }

    if normalized.is_empty() {
        vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
    } else {
        normalized
    }
}

pub fn resolve_next(current: ThemeMode, modes: &[ThemeMode]) -> ThemeMode {
    let Some(first) = modes.first().copied() else {
        return ThemeMode::default();
    };
    let index = modes.iter().position(|&m| m == current);
    let Some(index) = index else {
        return first;
    };
    modes
        .get((index + 1) % modes.len())
        .copied()
        .unwrap_or(first)
}

pub fn resolve_view_state(current: ThemeMode, modes: &[ThemeMode]) -> ThemeToggleViewState {
    let next = resolve_next(current, modes);
    ThemeToggleViewState {
        icon: current.icon(),
        next,
    }
}

pub fn resolve_state(
    current: ThemeMode,
    modes: &[ThemeMode],
    disabled: bool,
    has_custom_modes: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ThemeToggleState {
    let next_mode = resolve_next(current, modes);

    ThemeToggleState {
        is_disabled: disabled,
        is_enabled: !disabled,
        mode_count: modes.len(),
        has_custom_modes,
        has_custom_aria_label,
        has_custom_class_name,
        current_mode: current,
        current_mode_attr: current.as_attr(),
        next_mode,
        next_mode_attr: next_mode.as_attr(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ThemeToggleState) -> String {
    let mut classes = vec!["ui-theme-toggle-button".to_string()];

    if state.is_enabled {
        classes.push("ui-theme-toggle-button--enabled".to_string());
    }
    if state.is_disabled {
        classes.push("ui-theme-toggle-button--disabled".to_string());
    }
    if state.has_custom_modes {
        classes.push("ui-theme-toggle-button--custom-modes".to_string());
    }
    if state.has_custom_aria_label {
        classes.push("ui-theme-toggle-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/theme_toggle/logic.rs"]
mod tests;
