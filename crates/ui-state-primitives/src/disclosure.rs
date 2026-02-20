#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureStateInput {
    pub is_open: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureState {
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
}

pub fn resolve_state(input: DisclosureStateInput) -> DisclosureState {
    DisclosureState {
        is_open: input.is_open,
        is_closed: !input.is_open,
        is_disabled: input.is_disabled,
    }
}

#[cfg(test)]
#[path = "test/disclosure.rs"]
mod tests;
