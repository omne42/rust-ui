#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisclosureIds {
    pub trigger_id: String,
    pub panel_id: String,
}

impl DisclosureIds {
    pub fn new(id_base: &str) -> Self {
        Self {
            trigger_id: format!("{id_base}-trigger"),
            panel_id: format!("{id_base}-panel"),
        }
    }
}
