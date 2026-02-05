#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuOpenFocusStrategy {
    #[default]
    First,
    Last,
}

impl MenuOpenFocusStrategy {
    pub fn default_index(self, item_count: usize) -> usize {
        match self {
            Self::First => 0,
            Self::Last => item_count.saturating_sub(1),
        }
    }
}

pub fn focus_strategy_for_open_key(key: &str) -> Option<MenuOpenFocusStrategy> {
    match key {
        "ArrowDown" => Some(MenuOpenFocusStrategy::First),
        "ArrowUp" => Some(MenuOpenFocusStrategy::Last),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

pub fn resolve_ids(id_base: &str) -> ActionMenuIds {
    ActionMenuIds {
        trigger_id: format!("{id_base}-trigger"),
        menu_id: format!("{id_base}-menu"),
    }
}

pub fn resolve_trigger_aria_label(value: Option<&str>) -> String {
    value
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .unwrap_or_else(|| "More actions".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_include_menu_suffix() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.trigger_id, "demo-trigger");
        assert_eq!(ids.menu_id, "demo-menu");
    }

    #[test]
    fn aria_label_defaults_and_trims() {
        assert_eq!(resolve_trigger_aria_label(None), "More actions".to_string());
        assert_eq!(
            resolve_trigger_aria_label(Some("  More  ")),
            "More".to_string()
        );
    }

    #[test]
    fn focus_strategy_for_open_key_maps_arrow_keys() {
        assert_eq!(
            focus_strategy_for_open_key("ArrowDown"),
            Some(MenuOpenFocusStrategy::First)
        );
        assert_eq!(
            focus_strategy_for_open_key("ArrowUp"),
            Some(MenuOpenFocusStrategy::Last)
        );
        assert_eq!(focus_strategy_for_open_key("Enter"), None);
    }

    #[test]
    fn focus_strategy_default_index() {
        assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
        assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
    }
}
