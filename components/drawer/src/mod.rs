mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::DrawerPlacement;
pub use motion::DrawerMotion;
pub use view::Drawer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerSlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl DrawerSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            DrawerSlot::Root => "drawer",
            DrawerSlot::Header => "drawer-header",
            DrawerSlot::Title => "drawer-title",
            DrawerSlot::Description => "drawer-description",
            DrawerSlot::Body => "drawer-body",
            DrawerSlot::Footer => "drawer-footer",
            DrawerSlot::Close => "drawer-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            DrawerSlot::Root => "ui-drawer",
            DrawerSlot::Header => "ui-drawer__header",
            DrawerSlot::Title => "ui-drawer__title",
            DrawerSlot::Description => "ui-drawer__description",
            DrawerSlot::Body => "ui-drawer__body",
            DrawerSlot::Footer => "ui-drawer__footer",
            DrawerSlot::Close => "ui-drawer__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartStateInput {
    pub slot: DrawerSlot,
    pub placement: DrawerPlacement,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartState {
    pub slot: DrawerSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub placement_attr: &'static str,
    pub placement_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub placement_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}
