//! `ui-headless` — interaction & accessibility primitives (React Aria analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod button;
pub mod focus_ring;
pub mod focus_trap;
pub mod focus_visible;
pub mod focus_within;
pub mod hover;
pub mod modality;
pub mod overlay_stack;
pub mod press;

pub use button::{
    use_button, ButtonAria, ButtonAttrs, ButtonElement, ButtonHandlers, ButtonOptions,
};
pub use focus_ring::{use_focus_ring, FocusRingHandlers, FocusRingOptions, FocusRingState};
pub use focus_trap::{use_focus_trap, FocusTrapHandlers, FocusTrapOptions};
pub use focus_visible::{provide_focus_visible, use_focus_visible, FocusVisibleState};
pub use focus_within::{
    use_focus_within, FocusWithinHandlers, FocusWithinOptions, FocusWithinState,
};
pub use hover::{use_hover, HoverHandlers, HoverOptions, HoverState};
pub use modality::Modality;
pub use overlay_stack::{
    provide_overlay_stack, use_overlay_stack, use_overlay_stack_registration, OverlayRegistration,
};
pub use press::{use_press, OnPress, PressHandlers, PressOptions, PressState};
