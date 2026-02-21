#[cfg(feature = "component-menu_item")]
pub mod item;
mod logic;
pub mod motion;
#[cfg(feature = "component-menu_section")]
pub mod section;
pub mod styles;
mod view;

#[cfg(feature = "component-menu_item")]
pub use item::MenuItem;
pub use motion::MenuMotion;
#[cfg(feature = "component-menu_section")]
pub use section::{MenuSection, MenuSectionHeadingTone};
pub use view::Menu;

use crate::MenuItemKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItemSpec {
    pub label: String,
    pub kind: MenuItemKind,
    pub is_disabled: bool,
}

impl MenuItemSpec {
    pub fn action(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            kind: MenuItemKind::Action,
            is_disabled: false,
        }
    }

    pub fn with_kind(mut self, kind: MenuItemKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
