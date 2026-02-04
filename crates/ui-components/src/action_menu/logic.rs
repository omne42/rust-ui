#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuIds {
    pub menu_id: String,
}

pub fn resolve_ids(id_base: &str) -> ActionMenuIds {
    ActionMenuIds {
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
}
