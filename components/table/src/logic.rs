use ui_state_primitives::table as primitives;

pub use primitives::{
    DEFAULT_ARIA_LABEL, DEFAULT_EMPTY_TEXT, TableCellAlign, TableColumn, TableDensity, TableLayout,
    TableRow, TableState, TableStateInput, TableVariant, normalize_aria_label, normalize_columns,
    normalize_empty_text, normalize_optional_text, normalize_rows, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: TableState) -> String {
    let mut classes = vec![
        "ui-table".to_string(),
        state.variant_class.into(),
        state.density_class.into(),
        state.layout_class.into(),
    ];

    if state.is_striped {
        classes.push("ui-table--striped".to_string());
    }
    if state.has_sticky_header {
        classes.push("ui-table--sticky-header".to_string());
    }
    if state.has_caption {
        classes.push("ui-table--with-caption".to_string());
    }
    if state.is_empty {
        classes.push("ui-table--empty".to_string());
    } else {
        classes.push("ui-table--has-rows".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-table--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
