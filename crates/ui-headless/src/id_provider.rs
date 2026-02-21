use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct UiIdProvider {
    next_id: RwSignal<u64>,
}

impl UiIdProvider {
    pub fn new(seed: u64) -> Self {
        Self {
            next_id: RwSignal::new(seed),
        }
    }

    pub fn next(self) -> u64 {
        let current = self.next_id.get_untracked();
        self.next_id.update(|value| {
            *value = value.saturating_add(1);
        });
        current
    }

    pub fn next_prefixed_id(self, prefix: &str) -> String {
        format!("{prefix}-{}", self.next())
    }
}

pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {
    let provider = UiIdProvider::new(seed);
    provide_context(provider);
    provider
}

pub fn use_ui_id_provider() -> Option<UiIdProvider> {
    use_context::<UiIdProvider>()
}
