use super::*;
use crate::table::{TableDensity, TableLayout, TableStateInput};

#[test]
fn variant_class_names_and_attrs_are_stable() {
    assert_eq!(
        TableVariant::Default.class_name(),
        "ui-table--variant-default"
    );
    assert_eq!(TableVariant::Quiet.class_name(), "ui-table--variant-quiet");
    assert_eq!(
        TableVariant::Outline.class_name(),
        "ui-table--variant-outline"
    );

    assert_eq!(TableVariant::Default.as_attr(), "default");
    assert_eq!(TableVariant::Quiet.as_attr(), "quiet");
    assert_eq!(TableVariant::Outline.as_attr(), "outline");
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" ready ".to_string())),
        Some("ready".to_string())
    );
}

#[test]
fn normalize_columns_and_rows_shape_data() {
    let columns = normalize_columns(vec![
        TableColumn::new("  ", " Service "),
        TableColumn::new("uptime", " ").with_align(TableCellAlign::End),
    ]);
    assert_eq!(columns[0].key, "col-1");
    assert_eq!(columns[0].label, "Service");
    assert_eq!(columns[1].label, "Column 2");

    let rows = normalize_rows(
        vec![TableRow::new(" ", vec![" API ".to_string()])],
        columns.len(),
    );

    assert_eq!(rows[0].id, "row-1");
    assert_eq!(
        rows[0].cells,
        vec!["API".to_string(), DEFAULT_EMPTY_TEXT.into()]
    );
}

#[test]
fn resolve_state_tracks_sources_and_data_state() {
    let state = resolve_state(TableStateInput {
        variant: TableVariant::Outline,
        density: TableDensity::Compact,
        layout: TableLayout::Fixed,
        striped: true,
        sticky_header: true,
        has_caption: false,
        row_count: 0,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.variant_attr, "outline");
    assert_eq!(state.density_attr, "compact");
    assert_eq!(state.layout_attr, "fixed");
    assert_eq!(state.data_state_attr, "empty");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert!(state.is_empty);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-table".to_string()),
        resolve_state(TableStateInput {
            variant: TableVariant::Quiet,
            density: TableDensity::Comfortable,
            layout: TableLayout::Auto,
            striped: true,
            sticky_header: true,
            has_caption: true,
            row_count: 3,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-table",
        "ui-table--variant-quiet",
        "ui-table--density-comfortable",
        "ui-table--layout-auto",
        "ui-table--striped",
        "ui-table--sticky-header",
        "ui-table--with-caption",
        "ui-table--has-rows",
        "ui-table--custom-class",
        "docs-table",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}
