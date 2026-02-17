//! `ui-headless` — interaction & accessibility primitives (A11y Baseline analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod a11y;
pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod controllable_state;
pub mod focus_ring;
pub mod focus_trap;
pub mod focus_visible;
pub mod focus_within;
pub mod hover;
pub mod hover_card;
pub mod i18n;
pub mod input_otp;
pub mod listbox;
pub mod menu;
pub mod menu_item;
pub mod modal;
pub mod modality;
pub mod number_field;
pub mod overlay_stack;
pub mod perf;
pub mod popover_position;
pub mod presence;
pub mod press;
pub mod radio_group;
pub mod roving_tabindex;
pub mod switch;
pub mod text_field;
pub mod tooltip;
pub mod tooltip_position;
pub mod trace;

pub use a11y::{
    A11yDirection, A11yLocaleAttrs, DisclosureTriggerA11yAttrs, PopupTriggerA11yAttrs,
    aria_controls_when_open, aria_expanded, disclosure_trigger_attrs, locale_attrs,
    popup_trigger_attrs,
};
pub use button::{
    ButtonAria, ButtonAttrs, ButtonElement, ButtonHandlers, ButtonOptions, use_button,
};
pub use checkbox::{CheckboxAria, CheckboxAttrs, CheckboxHandlers, CheckboxOptions, use_checkbox};
pub use combo_box::{
    ComboBoxAria, ComboBoxHandlers, ComboBoxInputAttrs, ComboBoxListBoxAttrs, ComboBoxOptions,
    use_combo_box,
};
pub use controllable_state::{
    ControllableOpenState, ControllableState, use_controllable_open_state_traced,
    use_controllable_state,
};
pub use focus_ring::{FocusRingHandlers, FocusRingOptions, FocusRingState, use_focus_ring};
pub use focus_trap::{FocusTrapHandlers, FocusTrapOptions, use_focus_trap};
pub use focus_visible::{FocusVisibleState, provide_focus_visible, use_focus_visible};
pub use focus_within::{
    FocusWithinHandlers, FocusWithinOptions, FocusWithinState, use_focus_within,
};
pub use hover::{HoverHandlers, HoverOptions, HoverState, use_hover};
pub use hover_card::{
    HoverCardTriggerAria, HoverCardTriggerHandlers, HoverCardTriggerOptions, HoverCardTriggerState,
    use_hover_card_trigger,
};
pub use i18n::{CommonStrings, UiI18n, provide_ui_i18n, use_ui_i18n};
pub use input_otp::{InputOtpAria, InputOtpHandlers, InputOtpOptions, use_input_otp};
pub use listbox::{ListBoxAria, ListBoxAttrs, ListBoxHandlers, ListBoxOptions, use_listbox};
pub use menu::{MenuAria, MenuAttrs, MenuHandlers, MenuOnAction, MenuOptions, use_menu};
pub use menu_item::{
    MenuItemAria, MenuItemAttrs, MenuItemHandlers, MenuItemKind, MenuItemOptions, use_menu_item,
};
pub use modal::{ModalOptions, use_modal};
pub use modality::Modality;
pub use number_field::{
    NumberFieldAria, NumberFieldHandlers, NumberFieldInputAttrs, NumberFieldOptions,
    use_number_field,
};
pub use overlay_stack::{
    OverlayRegistration, provide_overlay_stack, use_overlay_stack, use_overlay_stack_registration,
};
pub use perf::{UiPerfBudget, UiPerfProbe};
pub use popover_position::{
    PopoverPlacement, PopoverPositionOptions, PopoverPositionState, use_popover_position,
};
pub use presence::{Presence, use_presence};
pub use press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, PressState, use_press};
pub use radio_group::{RadioGroupAria, RadioGroupHandlers, RadioGroupOptions, use_radio_group};
pub use roving_tabindex::{
    RovingOrientation, RovingTabIndexHandlers, RovingTabIndexOptions, RovingTabIndexState,
    use_roving_tabindex,
};
pub use switch::{SwitchAria, SwitchAttrs, SwitchHandlers, SwitchOptions, use_switch};
pub use text_field::{
    TextFieldAria, TextFieldInputAttrs, TextFieldLabelAttrs, TextFieldMessageAttrs,
    TextFieldOptions, use_text_field,
};
pub use tooltip::{
    TooltipTriggerAria, TooltipTriggerHandlers, TooltipTriggerMode, TooltipTriggerOptions,
    TooltipTriggerState, use_tooltip_trigger,
};
pub use tooltip_position::{
    TooltipPlacement, TooltipPositionOptions, TooltipPositionState, use_tooltip_position,
};
pub use trace::{UiTrace, UiTraceEvent, UiTraceEventKind, provide_ui_trace, use_ui_trace};
