use std::sync::Arc;

use crate::action_bar::logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLEAR_LABEL, DEFAULT_SELECTION_EMPTY_LABEL,
    DEFAULT_SELECTION_MULTIPLE_SUFFIX, DEFAULT_SELECTION_SINGLE_LABEL,
};

#[derive(Clone, Debug)]
pub struct ActionBarStrings {
    pub aria_label: Arc<str>,
    pub clear_label: Arc<str>,
    pub selection_empty_label: Arc<str>,
    pub selection_single_label: Arc<str>,
    /// Template for counts > 1.
    ///
    /// - Recommended: include `{count}` placeholder (e.g. "{count} items selected").
    /// - If `{count}` is missing, the count will be prefixed (e.g. "3 items selected").
    pub selection_multiple_template: Arc<str>,
}

impl ActionBarStrings {
    pub fn selection_label(&self, count: usize) -> String {
        match count {
            0 => self.selection_empty_label.as_ref().to_string(),
            1 => self.selection_single_label.as_ref().to_string(),
            count => {
                let template = self.selection_multiple_template.as_ref();
                if template.contains("{count}") {
                    template.replace("{count}", &count.to_string())
                } else {
                    format!("{count} {template}")
                }
            }
        }
    }
}

impl Default for ActionBarStrings {
    fn default() -> Self {
        Self {
            aria_label: DEFAULT_ARIA_LABEL.into(),
            clear_label: DEFAULT_CLEAR_LABEL.into(),
            selection_empty_label: DEFAULT_SELECTION_EMPTY_LABEL.into(),
            selection_single_label: DEFAULT_SELECTION_SINGLE_LABEL.into(),
            selection_multiple_template: format!("{{count}} {DEFAULT_SELECTION_MULTIPLE_SUFFIX}")
                .into(),
        }
    }
}
