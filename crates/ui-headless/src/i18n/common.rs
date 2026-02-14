use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CommonStrings {
    pub loading_aria_label: Arc<str>,
    pub close_aria_label: Arc<str>,
    pub clear_aria_label: Arc<str>,
    pub icon_button_aria_label: Arc<str>,
}

impl Default for CommonStrings {
    fn default() -> Self {
        Self {
            loading_aria_label: "Loading".into(),
            close_aria_label: "Close".into(),
            clear_aria_label: "Clear".into(),
            icon_button_aria_label: "Icon button".into(),
        }
    }
}
