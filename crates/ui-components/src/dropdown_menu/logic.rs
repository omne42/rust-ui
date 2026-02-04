#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropdownMenuIds {
    pub menu_id: String,
}

pub fn resolve_ids(id_base: &str) -> DropdownMenuIds {
    DropdownMenuIds {
        menu_id: format!("{id_base}-menu"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_id_derives_from_base() {
        let ids = resolve_ids("demo");
        assert_eq!(ids.menu_id, "demo-menu");
    }
}
