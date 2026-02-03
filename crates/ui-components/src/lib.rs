//! `ui-components` — Leptos components that compose ui-core + ui-headless + ui-theme.

mod active_highlight;
mod css;
mod presence;

pub mod button;
pub mod checkbox;
pub mod listbox;
pub mod menu;
pub mod menu_trigger;
pub mod modal;
pub mod overlay;
pub mod popover;
pub mod root;
pub mod select;
pub mod switch;

pub use active_highlight::ActiveHighlightMotion;
pub use button::Button;
pub use button::{ButtonMotion, ButtonSize, ButtonVariant};
pub use checkbox::Checkbox;
pub use checkbox::motion::CheckboxMotion;
pub use checkbox::{CheckboxSize, CheckboxVariant};
pub use listbox::ListBox;
pub use menu::Menu;
pub use menu_trigger::MenuTrigger;
pub use modal::Modal;
pub use overlay::Overlay;
pub use overlay::OverlayMotion;
pub use popover::Popover;
pub use popover::PopoverMotion;
pub use root::UiRoot;
pub use select::Select;
pub use switch::Switch;
pub use switch::SwitchMotion;
pub use ui_headless::{MenuItemKind, OnPress, provide_focus_visible, provide_overlay_stack};
pub use ui_theme::Theme;
