use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct EmptyStateStrings {
    pub default_title: Arc<str>,
    pub default_description: Arc<str>,
    pub default_aria_label: Arc<str>,
}

impl Default for EmptyStateStrings {
    fn default() -> Self {
        Self {
            default_title: "Nothing to show".into(),
            default_description: "Try adjusting filters or refreshing data.".into(),
            default_aria_label: "Empty state".into(),
        }
    }
}
