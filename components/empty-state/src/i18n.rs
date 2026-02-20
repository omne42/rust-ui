use std::sync::Arc;

use crate::logic::{DEFAULT_ARIA_LABEL, DEFAULT_DESCRIPTION, DEFAULT_TITLE};

#[derive(Clone, Debug)]
pub struct EmptyStateStrings {
    pub default_title: Arc<str>,
    pub default_description: Arc<str>,
    pub default_aria_label: Arc<str>,
}

impl Default for EmptyStateStrings {
    fn default() -> Self {
        Self {
            default_title: DEFAULT_TITLE.into(),
            default_description: DEFAULT_DESCRIPTION.into(),
            default_aria_label: DEFAULT_ARIA_LABEL.into(),
        }
    }
}
