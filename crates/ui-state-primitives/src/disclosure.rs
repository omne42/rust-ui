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
mod tests {
    use super::*;

    #[test]
    fn resolve_state_tracks_open_and_closed_flags() {
        let state = resolve_state(DisclosureStateInput {
            is_open: true,
            is_disabled: false,
        });
        assert!(state.is_open);
        assert!(!state.is_closed);
        assert!(!state.is_disabled);
    }

    #[test]
    fn resolve_state_tracks_disabled_state() {
        let state = resolve_state(DisclosureStateInput {
            is_open: false,
            is_disabled: true,
        });
        assert!(!state.is_open);
        assert!(state.is_closed);
        assert!(state.is_disabled);
    }
}
