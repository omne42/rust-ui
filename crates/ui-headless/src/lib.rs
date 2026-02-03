//! `ui-headless` — interaction & accessibility primitives (React Aria analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod button;
pub mod checkbox;
pub mod focus_ring;
pub mod focus_trap;
pub mod focus_visible;
pub mod focus_within;
pub mod hover;
pub mod listbox;
pub mod menu;
pub mod menu_item;
pub mod modal;
pub mod modality;
pub mod overlay_stack;
pub mod popover_position;
pub mod press;
pub mod roving_tabindex;
pub mod switch;
pub mod text_field;

pub use button::{
    ButtonAria, ButtonAttrs, ButtonElement, ButtonHandlers, ButtonOptions, use_button,
};
pub use checkbox::{CheckboxAria, CheckboxAttrs, CheckboxHandlers, CheckboxOptions, use_checkbox};
pub use focus_ring::{FocusRingHandlers, FocusRingOptions, FocusRingState, use_focus_ring};
pub use focus_trap::{FocusTrapHandlers, FocusTrapOptions, use_focus_trap};
pub use focus_visible::{FocusVisibleState, provide_focus_visible, use_focus_visible};
pub use focus_within::{
    FocusWithinHandlers, FocusWithinOptions, FocusWithinState, use_focus_within,
};
pub use hover::{HoverHandlers, HoverOptions, HoverState, use_hover};
pub use listbox::{ListBoxAria, ListBoxAttrs, ListBoxHandlers, ListBoxOptions, use_listbox};
pub use menu::{MenuAria, MenuAttrs, MenuHandlers, MenuOnAction, MenuOptions, use_menu};
pub use menu_item::{
    MenuItemAria, MenuItemAttrs, MenuItemHandlers, MenuItemKind, MenuItemOptions, use_menu_item,
};
pub use modal::{ModalOptions, use_modal};
pub use modality::Modality;
pub use overlay_stack::{
    OverlayRegistration, provide_overlay_stack, use_overlay_stack, use_overlay_stack_registration,
};
pub use popover_position::{
    PopoverPlacement, PopoverPositionOptions, PopoverPositionState, use_popover_position,
};
pub use press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, PressState, use_press};
pub use roving_tabindex::{
    RovingOrientation, RovingTabIndexHandlers, RovingTabIndexOptions, RovingTabIndexState,
    use_roving_tabindex,
};
pub use switch::{SwitchAria, SwitchAttrs, SwitchHandlers, SwitchOptions, use_switch};
pub use text_field::{
    TextFieldAria, TextFieldInputAttrs, TextFieldLabelAttrs, TextFieldMessageAttrs,
    TextFieldOptions, use_text_field,
};
