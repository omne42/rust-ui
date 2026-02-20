use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PaginationStrings {
    pub aria_label: Arc<str>,
    pub previous_page_aria_label: Arc<str>,
    pub next_page_aria_label: Arc<str>,
}

impl Default for PaginationStrings {
    fn default() -> Self {
        Self {
            aria_label: "Pagination".into(),
            previous_page_aria_label: "Previous page".into(),
            next_page_aria_label: "Next page".into(),
        }
    }
}
