use std::sync::Arc;

use crate::logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CONTROLS_ARIA_LABEL, DEFAULT_INDICATOR_ARIA_LABEL_TEMPLATE,
    DEFAULT_INDICATORS_ARIA_LABEL, DEFAULT_NEXT_LABEL, DEFAULT_PREVIOUS_LABEL,
};

#[derive(Clone, Debug)]
pub struct CarouselStrings {
    pub aria_label: Arc<str>,
    pub controls_aria_label: Arc<str>,
    pub indicators_aria_label: Arc<str>,
    pub previous_label: Arc<str>,
    pub next_label: Arc<str>,
    pub indicator_aria_label_template: Arc<str>,
}

impl Default for CarouselStrings {
    fn default() -> Self {
        Self {
            aria_label: DEFAULT_ARIA_LABEL.into(),
            controls_aria_label: DEFAULT_CONTROLS_ARIA_LABEL.into(),
            indicators_aria_label: DEFAULT_INDICATORS_ARIA_LABEL.into(),
            previous_label: DEFAULT_PREVIOUS_LABEL.into(),
            next_label: DEFAULT_NEXT_LABEL.into(),
            indicator_aria_label_template: DEFAULT_INDICATOR_ARIA_LABEL_TEMPLATE.into(),
        }
    }
}
