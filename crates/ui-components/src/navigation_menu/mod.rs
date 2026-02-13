mod logic;
mod motion;
pub mod styles;
mod view;

pub use crate::active_highlight::ActiveHighlightMotion as NavigationMenuMotion;
pub use logic::{DEFAULT_ACTIVATE_ON_FOCUS, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};
pub use view::NavigationMenu;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationMenuItem {
    pub id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

impl NavigationMenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: href.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationMenuItemResolved {
    pub id: String,
    pub dom_id: String,
    pub label: String,
    pub href: String,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuSlot {
    Root,
    List,
    Item,
    Highlight,
}

impl NavigationMenuSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            NavigationMenuSlot::Root => "navigation-menu",
            NavigationMenuSlot::List => "navigation-menu-list",
            NavigationMenuSlot::Item => "navigation-menu-item",
            NavigationMenuSlot::Highlight => "navigation-menu-highlight",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            NavigationMenuSlot::Root => "ui-navigation-menu",
            NavigationMenuSlot::List => "ui-navigation-menu__list",
            NavigationMenuSlot::Item => "ui-navigation-menu__item",
            NavigationMenuSlot::Highlight => "ui-active-highlight",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationMenuPartStateInput {
    pub slot: NavigationMenuSlot,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_disabled_items: bool,
    pub activate_on_focus: bool,
    pub is_controlled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_activate_on_focus: bool,
    pub has_custom_selected_id: bool,
    pub has_custom_default_selected_id: bool,
    pub has_custom_on_selected_id_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationMenuPartState {
    pub slot: NavigationMenuSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub selected_attr: &'static str,
    pub focus_attr: &'static str,
    pub focus_activation_attr: &'static str,
    pub selection_mode_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub closed_attr: Option<&'static str>,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_focus: bool,
    pub has_disabled_items: bool,
    pub activate_on_focus: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_activate_on_focus: bool,
    pub has_custom_selected_id: bool,
    pub has_custom_default_selected_id: bool,
    pub has_custom_on_selected_id_change: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub activate_on_focus_source_attr: &'static str,
    pub selected_id_source_attr: &'static str,
    pub default_selected_id_source_attr: &'static str,
    pub selected_id_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
