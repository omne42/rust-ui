use leptos::prelude::*;

pub fn aria_expanded(open: Signal<bool>) -> Signal<&'static str> {
    Signal::derive(move || if open.get() { "true" } else { "false" })
}

pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>> {
    Signal::derive(move || open.get().then(|| controls_id.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_expanded_is_false_when_closed() {
        let (open, set_open) = signal(false);
        let expanded = aria_expanded(open.into());

        assert_eq!(expanded.get_untracked(), "false");

        set_open.set(true);
        assert_eq!(expanded.get_untracked(), "true");
    }

    #[test]
    fn aria_controls_when_open_is_none_when_closed() {
        let (open, set_open) = signal(false);
        let controls = aria_controls_when_open(open.into(), "demo-controls".to_string());

        assert_eq!(controls.get_untracked(), None);

        set_open.set(true);
        assert_eq!(controls.get_untracked(), Some("demo-controls".to_string()));
    }
}
