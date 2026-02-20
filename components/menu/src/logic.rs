#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_checked_items: bool,
    pub has_disabled_items: bool,
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> MenuAccessibleName {
    let aria_label = normalize_optional_text(aria_label);
    let aria_labelledby = normalize_optional_text(aria_labelledby);

    if aria_label.is_some() {
        return MenuAccessibleName {
            aria_label,
            aria_labelledby: None,
        };
    }

    if aria_labelledby.is_some() {
        return MenuAccessibleName {
            aria_label: None,
            aria_labelledby,
        };
    }

    MenuAccessibleName {
        aria_label: Some("Menu".to_string()),
        aria_labelledby: None,
    }
}

pub fn resolve_state(
    item_count: usize,
    has_checked_items: bool,
    has_disabled_items: bool,
) -> MenuState {
    let has_items = item_count > 0;

    MenuState {
        is_empty: !has_items,
        has_items,
        has_checked_items,
        has_disabled_items,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
