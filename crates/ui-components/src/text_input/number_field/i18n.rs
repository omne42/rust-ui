use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct NumberFieldStrings {
    pub decrement_aria_label: Arc<str>,
    pub increment_aria_label: Arc<str>,
}

impl Default for NumberFieldStrings {
    fn default() -> Self {
        Self {
            decrement_aria_label: "Decrement".into(),
            increment_aria_label: "Increment".into(),
        }
    }
}
