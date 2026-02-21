mod logic;
mod motion;
pub mod styles;
mod view;

pub use ui_disclosure::DisclosureMotion as CollapsibleMotion;
pub use ui_state_primitives::collapsible::{
    CollapsibleState, CollapsibleStateInput, DEFAULT_ID_BASE, DEFAULT_TITLE,
};
pub use view::Collapsible;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
