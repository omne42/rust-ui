use std::sync::Arc;

use crate::well::logic::DEFAULT_ARIA_LABEL;

#[derive(Clone, Debug)]
pub struct WellStrings {
    pub aria_label: Arc<str>,
}

impl Default for WellStrings {
    fn default() -> Self {
        Self {
            aria_label: DEFAULT_ARIA_LABEL.into(),
        }
    }
}
