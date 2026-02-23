#[cfg(feature = "component-sidebar_group")]
pub mod group;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{SidebarCollapsible, SidebarSide, SidebarVariant};
pub use motion::SidebarMotion;
pub use view::Sidebar;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
