mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::TrayMotion;
pub use view::Tray;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraySlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl TraySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            TraySlot::Root => "tray",
            TraySlot::Header => "tray-header",
            TraySlot::Title => "tray-title",
            TraySlot::Description => "tray-description",
            TraySlot::Body => "tray-body",
            TraySlot::Footer => "tray-footer",
            TraySlot::Close => "tray-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            TraySlot::Root => "ui-tray",
            TraySlot::Header => "ui-tray__header",
            TraySlot::Title => "ui-tray__title",
            TraySlot::Description => "ui-tray__description",
            TraySlot::Body => "ui-tray__body",
            TraySlot::Footer => "ui-tray__footer",
            TraySlot::Close => "ui-tray__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayPartStateInput {
    pub slot: TraySlot,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub is_fixed_height: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayPartState {
    pub slot: TraySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub size_attr: &'static str,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub is_fixed_height: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}
