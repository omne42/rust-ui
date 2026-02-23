use std::sync::Arc;

use crate::text_input::date_field::logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_DAY_ARIA_LABEL,
    DEFAULT_LABEL, DEFAULT_MONTH_ARIA_LABEL, DEFAULT_PLACEHOLDER, DEFAULT_YEAR_ARIA_LABEL,
};

#[derive(Clone, Debug)]
pub struct DateFieldStrings {
    pub label: Arc<str>,
    pub placeholder: Arc<str>,
    pub aria_label: Arc<str>,
    pub year_aria_label: Arc<str>,
    pub month_aria_label: Arc<str>,
    pub day_aria_label: Arc<str>,
    pub clear_label: Arc<str>,
    pub clear_aria_label: Arc<str>,
}

impl Default for DateFieldStrings {
    fn default() -> Self {
        Self {
            label: DEFAULT_LABEL.into(),
            placeholder: DEFAULT_PLACEHOLDER.into(),
            aria_label: DEFAULT_ARIA_LABEL.into(),
            year_aria_label: DEFAULT_YEAR_ARIA_LABEL.into(),
            month_aria_label: DEFAULT_MONTH_ARIA_LABEL.into(),
            day_aria_label: DEFAULT_DAY_ARIA_LABEL.into(),
            clear_label: DEFAULT_CLEAR_LABEL.into(),
            clear_aria_label: DEFAULT_CLEAR_ARIA_LABEL.into(),
        }
    }
}
