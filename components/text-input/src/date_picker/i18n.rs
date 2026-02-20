use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DatePickerStrings {
    pub calendar_aria_label: Arc<str>,
}

impl Default for DatePickerStrings {
    fn default() -> Self {
        Self {
            calendar_aria_label: "Date picker calendar".into(),
        }
    }
}
