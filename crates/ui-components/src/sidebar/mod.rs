#[cfg(feature = "component-sidebar_group")]
pub mod group;
mod logic;
pub mod styles;
mod view;

pub use logic::{SidebarCollapsible, SidebarSide, SidebarVariant};
pub use view::Sidebar;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar";
pub const DEFAULT_SHORTCUT_KEY: &str = "b";
