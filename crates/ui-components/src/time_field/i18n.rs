use std::sync::Arc;

use crate::time_field::logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_HOUR_ARIA_LABEL,
    DEFAULT_LABEL, DEFAULT_MINUTE_ARIA_LABEL, DEFAULT_PLACEHOLDER,
};

#[derive(Clone, Debug)]
pub struct TimeFieldStrings {
    pub label: Arc<str>,
    pub placeholder: Arc<str>,
    pub aria_label: Arc<str>,
    pub hour_aria_label: Arc<str>,
    pub minute_aria_label: Arc<str>,
    pub clear_label: Arc<str>,
    pub clear_aria_label: Arc<str>,
}

impl Default for TimeFieldStrings {
    fn default() -> Self {
        Self {
            label: DEFAULT_LABEL.into(),
            placeholder: DEFAULT_PLACEHOLDER.into(),
            aria_label: DEFAULT_ARIA_LABEL.into(),
            hour_aria_label: DEFAULT_HOUR_ARIA_LABEL.into(),
            minute_aria_label: DEFAULT_MINUTE_ARIA_LABEL.into(),
            clear_label: DEFAULT_CLEAR_LABEL.into(),
            clear_aria_label: DEFAULT_CLEAR_ARIA_LABEL.into(),
        }
    }
}
