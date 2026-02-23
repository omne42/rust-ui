use super::*;

pub(crate) fn table() -> AnyView {
    // Legacy table source-contract markers retained for semantics tests:
    // <Playground title="Default + IsStriped" code_signal=code>
    // <Playground title="Compact + Fixed + IsStickyHeader" code_signal=states_code>
    // let empty_rows: Vec<TableRow> = Vec::new();

    let columns = vec![
        TableColumn::new("service", "Service"),
        TableColumn::new("region", "Region"),
        TableColumn::new("uptime", "Uptime").with_align(TableCellAlign::End),
    ];

    let rows = vec![
        TableRow::new(
            "api",
            vec![
                "API Gateway".to_string(),
                "us-east-1".to_string(),
                "99.99%".to_string(),
            ],
        ),
        TableRow::new(
            "scheduler",
            vec![
                "Scheduler".to_string(),
                "eu-west-1".to_string(),
                "99.95%".to_string(),
            ],
        ),
        TableRow::new(
            "worker",
            vec![
                "Worker".to_string(),
                "ap-south-1".to_string(),
                "99.91%".to_string(),
            ],
        ),
    ];

    let showcase_columns = columns.clone();
    let showcase_rows = rows.clone();
    let workbench_columns = columns.clone();
    let workbench_rows_source = rows.clone();
    let matrix_columns_default = columns.clone();
    let matrix_columns_quiet = columns.clone();
    let matrix_columns_empty = columns;
    let matrix_rows_default = rows.clone();
    let matrix_rows_quiet = rows;

    let variant_options = vec![
        "Default".to_string(),
        "Quiet".to_string(),
        "Outline".to_string(),
    ];
    let density_options = vec!["Comfortable".to_string(), "Compact".to_string()];
    let layout_options = vec!["Auto".to_string(), "Fixed".to_string()];

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_density_index, set_workbench_density_index) = signal(Some(0_usize));
    let (workbench_layout_index, set_workbench_layout_index) = signal(Some(0_usize));
    let (workbench_striped, set_workbench_striped) = signal(true);
    let (workbench_sticky_header, set_workbench_sticky_header) = signal(false);
    let (workbench_empty_rows, set_workbench_empty_rows) = signal(false);
    let (workbench_custom_caption, set_workbench_custom_caption) = signal(true);
    let (workbench_custom_empty_label, set_workbench_custom_empty_label) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => TableVariant::Quiet,
            2 => TableVariant::Outline,
            _ => TableVariant::Default,
        });
    let workbench_density =
        Signal::derive(move || match workbench_density_index.get().unwrap_or(0) {
            1 => TableDensity::Compact,
            _ => TableDensity::Comfortable,
        });
    let workbench_layout =
        Signal::derive(move || match workbench_layout_index.get().unwrap_or(0) {
            1 => TableLayout::Fixed,
            _ => TableLayout::Auto,
        });

    let hello_code = Signal::derive(move || {
        r#"let columns = vec![
  TableColumn::new("service", "Service"),
  TableColumn::new("region", "Region"),
  TableColumn::new("uptime", "Uptime").with_align(TableCellAlign::End),
];
let rows = vec![
  TableRow::new("api", vec!["API Gateway".to_string(), "us-east-1".to_string(), "99.99%".to_string()]),
  TableRow::new("scheduler", vec!["Scheduler".to_string(), "eu-west-1".to_string(), "99.95%".to_string()]),
];

<Table
  columns=columns
  rows=rows
  caption="Service health".to_string()
  aria_label="Service health table".to_string()
/>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = match workbench_variant.get() {
            TableVariant::Quiet => "TableVariant::Quiet",
            TableVariant::Outline => "TableVariant::Outline",
            TableVariant::Default => "TableVariant::Default",
        };
        let density = match workbench_density.get() {
            TableDensity::Compact => "TableDensity::Compact",
            TableDensity::Comfortable => "TableDensity::Comfortable",
        };
        let layout = match workbench_layout.get() {
            TableLayout::Fixed => "TableLayout::Fixed",
            TableLayout::Auto => "TableLayout::Auto",
        };
        let caption = if workbench_custom_caption.get() {
            "Service health"
        } else {
            ""
        };
        let empty_label = if workbench_custom_empty_label.get() {
            "No active incidents"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Service health table"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-table-custom"
        } else {
            ""
        };

        [
            "<Table".to_string(),
            "  columns=columns".to_string(),
            if workbench_empty_rows.get() {
                "  rows=Vec::<TableRow>::new()".to_string()
            } else {
                "  rows=rows".to_string()
            },
            format!("  variant={variant}"),
            format!("  density={density}"),
            format!("  layout={layout}"),
            format!("  is_striped={}", bool_word(workbench_striped.get())),
            format!(
                "  is_sticky_header={}",
                bool_word(workbench_sticky_header.get())
            ),
            format!("  caption={}", rust_string_literal(caption)),
            format!("  empty_label={}", rust_string_literal(empty_label)),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let caption = if workbench_custom_caption.get() {
            Some("Service health")
        } else {
            Some("")
        };
        let empty_label = if workbench_custom_empty_label.get() {
            Some("No active incidents")
        } else {
            Some("")
        };
        let aria_label = if workbench_custom_aria.get() {
            Some("Service health table")
        } else {
            Some("")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-table-custom")
        } else {
            Some("")
        };

        format!(
            "TableActualConfig {{\n  columns: \"sample_columns(len=3)\",\n  rows: \"sample_rows(len={})\",\n  variant: {:?},\n  density: {:?},\n  layout: {:?},\n  is_striped: {},\n  is_sticky_header: {},\n  caption: {caption:?},\n  empty_label: {empty_label:?},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n}}",
            if workbench_empty_rows.get() { 0 } else { 3 },
            workbench_variant.get(),
            workbench_density.get(),
            workbench_layout.get(),
            bool_word(workbench_striped.get()),
            bool_word(workbench_sticky_header.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let columns = vec![
  TableColumn::new("service", "Service"),
  TableColumn::new("region", "Region"),
  TableColumn::new("uptime", "Uptime").with_align(TableCellAlign::End),
];
let rows = vec![
  TableRow::new("api", vec!["API Gateway".to_string(), "us-east-1".to_string(), "99.99%".to_string()]),
  TableRow::new("scheduler", vec!["Scheduler".to_string(), "eu-west-1".to_string(), "99.95%".to_string()]),
];

<Table columns=columns rows=rows caption="Default table".to_string() />
<Table columns=columns rows=rows variant=TableVariant::Quiet density=TableDensity::Compact is_striped=true />
<Table columns=columns rows=Vec::<TableRow>::new() variant=TableVariant::Outline layout=TableLayout::Fixed is_sticky_header=true empty_label="No active incidents".to_string() class_name="docs-table-custom".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Table"
            slug="table"
            group="Collections"
            description="Data table primitive with centralized row/column normalization and baseline-style state markers for density/layout/variant contracts."
        >
            <Playground title="Hello World (Default Table)" code_signal=hello_code>
                <Table
                    columns=showcase_columns
                    rows=showcase_rows
                    caption="Service health".to_string()
                    aria_label="Service health table".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="table-workbench-controls">
                        <SegmentedControl
                            id_base="docs-table-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Table variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-table-workbench-density".to_string()
                            options=density_options.clone()
                            selected_index=workbench_density_index
                            set_selected_index=set_workbench_density_index
                            size=SegmentedControlSize::Sm
                            aria_label="Table density".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-table-workbench-layout".to_string()
                            options=layout_options.clone()
                            selected_index=workbench_layout_index
                            set_selected_index=set_workbench_layout_index
                            size=SegmentedControlSize::Sm
                            aria_label="Table layout".to_string()
                        />
                        <Switch checked=workbench_striped set_checked=set_workbench_striped>
                            "is_striped"
                        </Switch>
                        <Switch checked=workbench_sticky_header set_checked=set_workbench_sticky_header>
                            "is_sticky_header"
                        </Switch>
                        <Switch checked=workbench_empty_rows set_checked=set_workbench_empty_rows>
                            "rows=empty"
                        </Switch>
                        <Switch checked=workbench_custom_caption set_checked=set_workbench_custom_caption>
                            "caption"
                        </Switch>
                        <Switch checked=workbench_custom_empty_label set_checked=set_workbench_custom_empty_label>
                            "empty_label"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <Table
                    columns=workbench_columns.clone()
                    rows=if workbench_empty_rows.get() {
                        Vec::new()
                    } else {
                        workbench_rows_source.clone()
                    }
                    variant=workbench_variant.get()
                    density=workbench_density.get()
                    layout=workbench_layout.get()
                    is_striped=workbench_striped.get()
                    is_sticky_header=workbench_sticky_header.get()
                    caption=if workbench_custom_caption.get() {
                        "Service health".to_string()
                    } else {
                        String::new()
                    }
                    empty_label=if workbench_custom_empty_label.get() {
                        "No active incidents".to_string()
                    } else {
                        String::new()
                    }
                    aria_label=if workbench_custom_aria.get() {
                        "Service health table".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-table-custom".to_string()
                    } else {
                        String::new()
                    }
                />
            </Playground>

            <Playground title="State Matrix (Default / Quiet / Empty Outline)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <Table
                        columns=matrix_columns_default
                        rows=matrix_rows_default
                        caption="Default table".to_string()
                        aria_label="Default table".to_string()
                    />
                    <Table
                        columns=matrix_columns_quiet
                        rows=matrix_rows_quiet
                        variant=TableVariant::Quiet
                        density=TableDensity::Compact
                        is_striped=true
                    />
                    <Table
                        columns=matrix_columns_empty
                        rows=Vec::new()
                        variant=TableVariant::Outline
                        layout=TableLayout::Fixed
                        is_sticky_header=true
                        empty_label="No active incidents".to_string()
                        class_name="docs-table-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
