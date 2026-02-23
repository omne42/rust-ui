use super::*;

#[test]
fn variant_density_layout_and_align_contracts_are_stable() {
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

    assert_eq!(
        TableDensity::Comfortable.class_name(),
        "ui-table--density-comfortable"
    );
    assert_eq!(
        TableDensity::Compact.class_name(),
        "ui-table--density-compact"
    );
    assert_eq!(TableDensity::Comfortable.as_attr(), "comfortable");
    assert_eq!(TableDensity::Compact.as_attr(), "compact");

    assert_eq!(TableLayout::Auto.class_name(), "ui-table--layout-auto");
    assert_eq!(TableLayout::Fixed.class_name(), "ui-table--layout-fixed");
    assert_eq!(TableLayout::Auto.as_attr(), "auto");
    assert_eq!(TableLayout::Fixed.as_attr(), "fixed");

    assert_eq!(
        TableCellAlign::Start.class_name(),
        "ui-table__cell--align-start"
    );
    assert_eq!(
        TableCellAlign::Center.class_name(),
        "ui-table__cell--align-center"
    );
    assert_eq!(
        TableCellAlign::End.class_name(),
        "ui-table__cell--align-end"
    );
    assert_eq!(TableCellAlign::Start.as_attr(), "start");
    assert_eq!(TableCellAlign::Center.as_attr(), "center");
    assert_eq!(TableCellAlign::End.as_attr(), "end");
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
        vec!["API".to_string(), DEFAULT_EMPTY_TEXT.to_string()]
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
fn normalize_aria_label_and_empty_text_use_fallbacks() {
    let (default_label, has_custom_label) = normalize_aria_label(Some("   ".to_string()));
    assert_eq!(default_label, DEFAULT_ARIA_LABEL);
    assert!(!has_custom_label);

    let (custom_label, has_custom_label) = normalize_aria_label(Some(" Incidents ".to_string()));
    assert_eq!(custom_label, "Incidents");
    assert!(has_custom_label);

    assert_eq!(normalize_empty_text(None), DEFAULT_EMPTY_TEXT);
    assert_eq!(
        normalize_empty_text(Some("  No records  ".to_string())),
        "No records"
    );
}
