use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::sync::Arc;
use ui::{
    Carousel, CarouselItem, CarouselOrientation, Command, CommandDialog, CommandGroup, CommandItem,
    ContextMenu, MenuItemKind, Menubar, MenubarMenu, NavigationMenu, NavigationMenuItem,
    SegmentedControl, SegmentedControlSize, Switch,
};
use ui_headless::A11yDirection;

const COMMAND_DIALOG_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{CommandDialog, CommandGroup, CommandItem};";
const COMMAND_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::{Command, CommandGroup, CommandItem};";
const CAROUSEL_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{Carousel, CarouselItem, CarouselOrientation};";

#[path = "collections_command/carousel.rs"]
mod carousel;
#[path = "collections_command/command.rs"]
mod command;
#[path = "collections_command/command_dialog.rs"]
mod command_dialog;
#[path = "collections_command/context_menu.rs"]
mod context_menu;
#[path = "collections_command/menubar.rs"]
mod menubar;
#[path = "collections_command/navigation_menu.rs"]
mod navigation_menu;

pub(super) use carousel::carousel;
pub(super) use command::command;
pub(super) use command_dialog::command_dialog;
pub(super) use context_menu::context_menu;
pub(super) use menubar::menubar;
pub(super) use navigation_menu::navigation_menu;
