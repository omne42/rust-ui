//! `ui-components` — Leptos components that compose ui-core + ui-headless + ui-theme.

mod active_highlight;
mod css;
mod presence;

pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod circular_progress;
pub mod combo_box;
pub mod divider;
pub mod icon_button;
pub mod listbox;
pub mod menu;
pub mod menu_trigger;
pub mod modal;
pub mod overlay;
pub mod popover;
pub mod radio;
pub mod root;
pub mod select;
pub mod switch;
pub mod tabs;
pub mod text_area;
pub mod text_field;
pub mod tooltip;

pub use active_highlight::ActiveHighlightMotion;
pub use avatar::{Avatar, AvatarSize};
pub use badge::Badge;
pub use badge::BadgeVariant;
pub use button::Button;
pub use button::{ButtonMotion, ButtonSize, ButtonVariant};
pub use checkbox::Checkbox;
pub use checkbox::motion::CheckboxMotion;
pub use checkbox::{CheckboxSize, CheckboxVariant};
pub use circular_progress::CircularProgress;
pub use combo_box::ComboBox;
pub use divider::{Divider, DividerOrientation};
pub use icon_button::IconButton;
pub use listbox::ListBox;
pub use menu::Menu;
pub use menu_trigger::MenuTrigger;
pub use modal::Modal;
pub use overlay::Overlay;
pub use overlay::OverlayMotion;
pub use popover::Popover;
pub use popover::PopoverMotion;
pub use radio::{Radio, RadioGroup, RadioGroupOrientation, RadioMotion};
pub use root::UiRoot;
pub use select::Select;
pub use switch::Switch;
pub use switch::SwitchMotion;
pub use tabs::Tabs;
pub use text_area::TextArea;
pub use text_field::TextField;
pub use tooltip::Tooltip;
pub use tooltip::TooltipMotion;
pub use ui_headless::{MenuItemKind, OnPress, provide_focus_visible, provide_overlay_stack};
pub use ui_theme::Theme;

#[doc(hidden)]
pub fn push_components_css(out: &mut String) {
    css::push_components_css(out);
}
