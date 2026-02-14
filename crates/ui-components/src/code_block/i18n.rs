use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CodeBlockStrings {
    pub copy_to_clipboard_aria_label: Arc<str>,
    pub copied_status_text: Arc<str>,
}

impl Default for CodeBlockStrings {
    fn default() -> Self {
        Self {
            copy_to_clipboard_aria_label: "Copy to clipboard".into(),
            copied_status_text: "Copied".into(),
        }
    }
}
