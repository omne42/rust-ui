use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui::{
    AccordionMotion, DisclosureGroup, DisclosureGroupSelectionMode, Dropdown, DropdownMotion,
    ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion, MenuItem, MenuItemKind,
    MenuSection, MenuSectionHeadingTone, PopoverMotion, SegmentedControl, SegmentedControlSize,
    StepList, StepListItem, StepListOrientation, StepListSize, Switch, Table, TableCellAlign,
    TableColumn, TableDensity, TableLayout, TableRow, TableVariant, Tree, TreeDensity, TreeMotion,
    TreeNode, TreeTone, open_set,
};
use ui_headless::{A11yDirection, PopoverPlacement};

#[path = "collections_extra/disclosure_group.rs"]
mod disclosure_group;
#[path = "collections_extra/dropdown.rs"]
mod dropdown;
#[path = "collections_extra/list_item.rs"]
mod list_item;
#[path = "collections_extra/list_section.rs"]
mod list_section;
#[path = "collections_extra/menu_item.rs"]
mod menu_item;
#[path = "collections_extra/menu_section.rs"]
mod menu_section;
#[path = "collections_extra/step_list.rs"]
mod step_list;
#[path = "collections_extra/table.rs"]
mod table;
#[path = "collections_extra/tree.rs"]
mod tree;

pub(super) use disclosure_group::disclosure_group;
pub(super) use dropdown::dropdown;
pub(super) use list_item::list_item;
pub(super) use list_section::list_section;
pub(super) use menu_item::menu_item;
pub(super) use menu_section::menu_section;
pub(super) use step_list::step_list;
pub(super) use table::table;
pub(super) use tree::tree;
