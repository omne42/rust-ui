use leptos::prelude::*;

pub(crate) fn aria_controls_when_open(
    open: Signal<bool>,
    controls_id: String,
) -> Signal<Option<String>> {
    Signal::derive(move || open.get().then(|| controls_id.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_controls_when_open_is_none_when_closed() {
        let (open, set_open) = signal(false);
        let controls = aria_controls_when_open(open.into(), "demo-controls".to_string());

        assert_eq!(controls.get_untracked(), None);

        set_open.set(true);
        assert_eq!(controls.get_untracked(), Some("demo-controls".to_string()));
    }
}
