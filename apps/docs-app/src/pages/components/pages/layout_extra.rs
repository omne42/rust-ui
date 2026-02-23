use super::playground_workbench::{bool_word, rust_string_literal};
#[path = "layout_extra_sidebar_content.rs"]
mod layout_extra_sidebar_content;
#[path = "layout_extra_sidebar_footer.rs"]
mod layout_extra_sidebar_footer;
#[path = "layout_extra_sidebar_group.rs"]
mod layout_extra_sidebar_group;
#[path = "layout_extra_sidebar_inset.rs"]
mod layout_extra_sidebar_inset;
#[path = "layout_extra_sidebar_menu_action.rs"]
mod layout_extra_sidebar_menu_action;
#[path = "layout_extra_sidebar_menu_badge.rs"]
mod layout_extra_sidebar_menu_badge;
#[path = "layout_extra_sidebar_rail.rs"]
mod layout_extra_sidebar_rail;
#[path = "layout_extra_sidebar_trigger.rs"]
mod layout_extra_sidebar_trigger;
#[path = "layout_extra_surface.rs"]
mod layout_extra_surface;

use crate::pages::components::{ComponentDoc, ComponentPage};

pub(super) const SCROLL_AREA_DOC: ComponentDoc = ComponentDoc {
    name: "ScrollArea",
    slug: "scroll-area",
    group: "Layout",
    page: scroll_area,
};
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    A11yDirection, SegmentedControl, SegmentedControlSize, Sidebar, SidebarCollapsible,
    SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarMenuMotion, SidebarMenuSubItem,
    SidebarSide, SidebarVariant, Snippet, Switch,
};
use ui_layout::{
    AspectRatio, AspectRatioPreset, AspectRatioRadius, Grid, GridAlign, GridColumns, GridGap,
    GridJustify, GridRows, Resizable, ResizableMotion, ResizableOrientation, ScrollArea,
    ScrollAreaOrientation, View, ViewBackground, ViewBorder, ViewPadding, ViewRadius,
};

#[path = "layout_extra/aspect_ratio.rs"]
mod aspect_ratio;
#[path = "layout_extra/grid.rs"]
mod grid;
#[path = "layout_extra/resizable.rs"]
mod resizable;
#[path = "layout_extra/scroll_area.rs"]
mod scroll_area;
#[path = "layout_extra/sidebar.rs"]
mod sidebar;
#[path = "layout_extra/sidebar_content.rs"]
mod sidebar_content;
#[path = "layout_extra/sidebar_footer.rs"]
mod sidebar_footer;
#[path = "layout_extra/sidebar_group.rs"]
mod sidebar_group;
#[path = "layout_extra/sidebar_header.rs"]
mod sidebar_header;
#[path = "layout_extra/sidebar_inset.rs"]
mod sidebar_inset;
#[path = "layout_extra/sidebar_menu.rs"]
mod sidebar_menu;
#[path = "layout_extra/sidebar_menu_action.rs"]
mod sidebar_menu_action;
#[path = "layout_extra/sidebar_menu_badge.rs"]
mod sidebar_menu_badge;
#[path = "layout_extra/sidebar_rail.rs"]
mod sidebar_rail;
#[path = "layout_extra/sidebar_trigger.rs"]
mod sidebar_trigger;
#[path = "layout_extra/surface.rs"]
mod surface;

pub(super) use aspect_ratio::aspect_ratio;
pub(super) use grid::grid;
pub(super) use resizable::resizable;
pub(super) use scroll_area::scroll_area;
pub(super) use sidebar::sidebar;
pub(super) use sidebar_content::sidebar_content;
pub(super) use sidebar_footer::sidebar_footer;
pub(super) use sidebar_group::sidebar_group;
pub(super) use sidebar_header::sidebar_header;
pub(super) use sidebar_inset::sidebar_inset;
pub(super) use sidebar_menu::sidebar_menu;
pub(super) use sidebar_menu_action::sidebar_menu_action;
pub(super) use sidebar_menu_badge::sidebar_menu_badge;
pub(super) use sidebar_rail::sidebar_rail;
pub(super) use sidebar_trigger::sidebar_trigger;
pub(super) use surface::surface;
