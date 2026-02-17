use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ButtonCopyStrings {
    pub copy_button_label: Arc<str>,
    pub copied_status_text: Arc<str>,
    pub copy_failed_status_text: Arc<str>,
}

impl Default for ButtonCopyStrings {
    fn default() -> Self {
        Self {
            copy_button_label: "Copy".into(),
            copied_status_text: "Copied".into(),
            copy_failed_status_text: "Copy failed".into(),
        }
    }
}
