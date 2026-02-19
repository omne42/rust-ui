mod logic;
pub mod styles;
mod view;

pub use crate::active_highlight::ActiveHighlightMotion as SidebarMenuMotion;
pub use logic::{SidebarMenuItem, SidebarMenuState, SidebarMenuSubItem};
pub use view::SidebarMenu;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar menu";
pub const DEFAULT_ID_BASE: &str = "ui-sidebar-menu";
