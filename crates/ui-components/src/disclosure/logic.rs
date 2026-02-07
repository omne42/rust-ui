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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureState {
    pub is_open: bool,
    pub is_closed: bool,
    pub is_disabled: bool,
}

pub fn resolve_state(is_open: bool, is_disabled: bool) -> DisclosureState {
    DisclosureState {
        is_open,
        is_closed: !is_open,
        is_disabled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_derived_from_id_base() {
        let ids = DisclosureIds::new("example");
        assert_eq!(ids.trigger_id, "example-trigger");
        assert_eq!(ids.panel_id, "example-panel");
    }

    #[test]
    fn resolve_state_tracks_open_and_closed_flags() {
        let state = resolve_state(true, false);
        assert!(state.is_open);
        assert!(!state.is_closed);
        assert!(!state.is_disabled);
    }

    #[test]
    fn resolve_state_tracks_disabled_state() {
        let state = resolve_state(false, true);
        assert!(!state.is_open);
        assert!(state.is_closed);
        assert!(state.is_disabled);
    }
}
