#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoHeightState {
    pub overflow_hidden: bool,
}

pub fn resolve_state() -> AutoHeightState {
    AutoHeightState {
        overflow_hidden: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_overflow_hidden() {
        assert!(resolve_state().overflow_hidden);
    }
}
