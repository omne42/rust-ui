mod logic;
pub mod styles;
mod view;

pub use logic::{SidebarMenuItem, SidebarMenuState, SidebarMenuSubItem};
pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion as SidebarMenuMotion;
pub use view::SidebarMenu;

pub const DEFAULT_ARIA_LABEL: &str = "Sidebar menu";
pub const DEFAULT_ID_BASE: &str = "ui-sidebar-menu";
