#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridListSemantics {
    pub root_role: &'static str,
    pub item_role: &'static str,
    pub section_role: &'static str,
    pub supports_roving_tabindex: bool,
}

pub fn resolve_grid_list_semantics() -> GridListSemantics {
    GridListSemantics {
        // GridList currently forwards to ListBox primitives.
        root_role: "listbox",
        item_role: "option",
        section_role: "group",
        supports_roving_tabindex: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantics_forward_to_listbox_contract() {
        let semantics = resolve_grid_list_semantics();

        assert_eq!(semantics.root_role, "listbox");
        assert_eq!(semantics.item_role, "option");
        assert_eq!(semantics.section_role, "group");
        assert!(semantics.supports_roving_tabindex);
    }
}
