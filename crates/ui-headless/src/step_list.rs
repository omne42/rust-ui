use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::step_list::{
    StepListItem, StepListOrientation, first_enabled_index, last_enabled_index, next_enabled_index,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepListRootA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn step_list_root_a11y_attrs(
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> StepListRootA11yAttrs {
    let locale = locale_attrs(lang, dir);
    StepListRootA11yAttrs {
        role: "list",
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemA11yInput {
    pub index: usize,
    pub selected_index: Option<usize>,
    pub first_enabled_index: Option<usize>,
    pub is_current: bool,
    pub is_disabled: bool,
    pub is_selectable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemA11yAttrs {
    pub aria_current: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub tabindex: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemSemanticState {
    pub is_current: bool,
    pub is_disabled: bool,
    pub is_selectable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemContract {
    pub attrs: StepListItemA11yAttrs,
    pub state: StepListItemSemanticState,
}

pub fn step_list_item_contract(input: StepListItemA11yInput) -> StepListItemContract {
    let tabindex = if input.is_disabled {
        -1
    } else if input.is_current
        || (input.selected_index.is_none() && input.first_enabled_index == Some(input.index))
    {
        0
    } else {
        -1
    };

    StepListItemContract {
        attrs: StepListItemA11yAttrs {
            aria_current: input.is_current.then_some("step"),
            aria_disabled: input.is_disabled.then_some("true"),
            tabindex,
        },
        state: StepListItemSemanticState {
            is_current: input.is_current,
            is_disabled: input.is_disabled,
            is_selectable: input.is_selectable,
        },
    }
}

pub fn resolve_step_list_next_index(
    items: &[StepListItem],
    orientation: StepListOrientation,
    current_index: usize,
    key: &str,
) -> Option<usize> {
    match key {
        "ArrowRight" if orientation == StepListOrientation::Horizontal => {
            next_enabled_index(items, current_index, 1)
        }
        "ArrowLeft" if orientation == StepListOrientation::Horizontal => {
            next_enabled_index(items, current_index, -1)
        }
        "ArrowDown" if orientation == StepListOrientation::Vertical => {
            next_enabled_index(items, current_index, 1)
        }
        "ArrowUp" if orientation == StepListOrientation::Vertical => {
            next_enabled_index(items, current_index, -1)
        }
        "Home" => first_enabled_index(items),
        "End" => last_enabled_index(items),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_list_root_attrs_expose_locale_and_role() {
        let attrs = step_list_root_a11y_attrs(
            "Setup steps".to_string(),
            Some(" zh-CN ".to_string()),
            Some(A11yDirection::Rtl),
        );
        assert_eq!(attrs.role, "list");
        assert_eq!(attrs.aria_label, "Setup steps");
        assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn step_list_item_contract_sets_tabindex_and_aria() {
        let contract = step_list_item_contract(StepListItemA11yInput {
            index: 1,
            selected_index: Some(1),
            first_enabled_index: Some(0),
            is_current: true,
            is_disabled: false,
            is_selectable: false,
        });
        assert_eq!(contract.attrs.aria_current, Some("step"));
        assert_eq!(contract.attrs.aria_disabled, None);
        assert_eq!(contract.attrs.tabindex, 0);
        assert!(!contract.state.is_selectable);

        let disabled = step_list_item_contract(StepListItemA11yInput {
            index: 2,
            selected_index: Some(1),
            first_enabled_index: Some(0),
            is_current: false,
            is_disabled: true,
            is_selectable: false,
        });
        assert_eq!(disabled.attrs.aria_current, None);
        assert_eq!(disabled.attrs.aria_disabled, Some("true"));
        assert_eq!(disabled.attrs.tabindex, -1);
    }

    #[test]
    fn keyboard_navigation_resolves_orientation_aware_targets() {
        let items = vec![
            StepListItem::new("a", "A"),
            StepListItem::new("b", "B").disabled(true),
            StepListItem::new("c", "C"),
        ];

        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "ArrowRight"),
            Some(2)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 2, "ArrowLeft"),
            Some(0)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Vertical, 0, "ArrowDown"),
            Some(2)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Vertical, 2, "ArrowUp"),
            Some(0)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 2, "Home"),
            Some(0)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "End"),
            Some(2)
        );
        assert_eq!(
            resolve_step_list_next_index(&items, StepListOrientation::Horizontal, 0, "Enter"),
            None
        );
    }
}
