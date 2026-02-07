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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
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
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Toggle theme  ".to_string())),
            Some("Toggle theme".to_string())
        );
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn normalize_modes_deduplicates_and_falls_back_to_defaults() {
        assert_eq!(
            normalize_modes(vec![ThemeMode::Dark, ThemeMode::Dark, ThemeMode::Light]),
            vec![ThemeMode::Dark, ThemeMode::Light]
        );
        assert_eq!(
            normalize_modes(Vec::new()),
            vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
        );
    }

    #[test]
    fn next_cycles_through_modes() {
        let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled];
        assert_eq!(resolve_next(ThemeMode::Light, &modes), ThemeMode::Dark);
        assert_eq!(resolve_next(ThemeMode::Dark, &modes), ThemeMode::Oled);
        assert_eq!(resolve_next(ThemeMode::Oled, &modes), ThemeMode::Light);
    }

    #[test]
    fn next_falls_back_to_first_when_current_not_found() {
        let modes = [ThemeMode::Dark, ThemeMode::Light];
        assert_eq!(resolve_next(ThemeMode::Oled, &modes), ThemeMode::Dark);
    }

    #[test]
    fn empty_modes_defaults_to_light() {
        assert_eq!(resolve_next(ThemeMode::Dark, &[]), ThemeMode::Light);
    }

    #[test]
    fn resolve_state_tracks_mode_enablement_and_metadata() {
        let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled];
        let enabled = resolve_state(ThemeMode::Dark, &modes, false, true, true, true);
        assert!(enabled.is_enabled);
        assert!(!enabled.is_disabled);
        assert_eq!(enabled.mode_count, 3);
        assert_eq!(enabled.current_mode_attr, "dark");
        assert_eq!(enabled.next_mode_attr, "oled");
        assert!(enabled.has_custom_modes);
        assert!(enabled.has_custom_aria_label);
        assert!(enabled.has_custom_class_name);

        let disabled = resolve_state(ThemeMode::Light, &modes, true, false, false, false);
        assert!(disabled.is_disabled);
        assert!(!disabled.is_enabled);
        assert_eq!(disabled.next_mode, ThemeMode::Dark);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(
                ThemeMode::Light,
                &[ThemeMode::Light, ThemeMode::Dark],
                false,
                true,
                true,
                true,
            ),
        );

        for token in [
            "ui-theme-toggle-button",
            "ui-theme-toggle-button--enabled",
            "ui-theme-toggle-button--custom-modes",
            "ui-theme-toggle-button--custom-aria-label",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
