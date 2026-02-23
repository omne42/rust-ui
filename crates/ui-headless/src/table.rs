use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::table::{DEFAULT_ARIA_LABEL, TableState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TableA11yHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_variant: &'static str,
    pub data_density: &'static str,
    pub data_layout: &'static str,
    pub data_state: &'static str,
    pub data_striped: Option<&'static str>,
    pub data_sticky_header: Option<&'static str>,
    pub data_has_caption: Option<&'static str>,
    pub data_row_count: String,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableA11yState {
    pub variant: &'static str,
    pub density: &'static str,
    pub layout: &'static str,
    pub state: &'static str,
    pub is_striped: bool,
    pub has_sticky_header: bool,
    pub has_caption: bool,
    pub row_count: usize,
    pub is_empty: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub aria_source: &'static str,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableA11yContract {
    pub attrs: TableA11yAttrs,
    pub handlers: TableA11yHandlers,
    pub state: TableA11yState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableA11yOptions {
    pub state: TableState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

fn normalize_aria_label(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return DEFAULT_ARIA_LABEL.into();
    }
    trimmed.into()
}

pub fn use_table_a11y(options: TableA11yOptions) -> TableA11yContract {
    let locale = locale_attrs(options.lang, options.dir);
    let state = options.state;

    TableA11yContract {
        attrs: TableA11yAttrs {
            role: "region",
            aria_label: normalize_aria_label(options.aria_label),
            lang: locale.lang,
            dir: locale.dir,
            data_variant: state.variant_attr,
            data_density: state.density_attr,
            data_layout: state.layout_attr,
            data_state: state.data_state_attr,
            data_striped: state.is_striped.then_some("true"),
            data_sticky_header: state.has_sticky_header.then_some("true"),
            data_has_caption: state.has_caption.then_some("true"),
            data_row_count: state.row_count.to_string(),
            data_aria_source: state.aria_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
        },
        handlers: TableA11yHandlers,
        state: TableA11yState {
            variant: state.variant_attr,
            density: state.density_attr,
            layout: state.layout_attr,
            state: state.data_state_attr,
            is_striped: state.is_striped,
            has_sticky_header: state.has_sticky_header,
            has_caption: state.has_caption,
            row_count: state.row_count,
            is_empty: state.is_empty,
            has_custom_aria_label: state.aria_source_attr == "custom",
            has_custom_class_name: state.has_custom_class_name,
            aria_source: state.aria_source_attr,
            class_source: state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/table.rs"]
mod tests;
