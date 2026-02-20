pub use ui_state_primitives::disclosure::{DisclosureState, DisclosureStateInput};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisclosureIds {
    pub trigger_id: String,
    pub panel_id: String,
}

impl DisclosureIds {
    pub fn new(id_base: &str) -> Self {
        Self {
            trigger_id: format!("{id_base}-trigger"),
            panel_id: format!("{id_base}-panel"),
        }
    }
}

pub fn resolve_state(is_open: bool, is_disabled: bool) -> DisclosureState {
    ui_state_primitives::disclosure::resolve_state(DisclosureStateInput {
        is_open,
        is_disabled,
    })
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
