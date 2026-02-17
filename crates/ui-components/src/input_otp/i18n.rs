use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct InputOtpStrings {
    pub aria_label: Arc<str>,
}

impl Default for InputOtpStrings {
    fn default() -> Self {
        Self {
            aria_label: "One-time code".into(),
        }
    }
}
