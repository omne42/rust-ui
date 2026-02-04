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

#[cfg(test)]
mod tests {
    use super::*;

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
}
