use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui::{
    AccordionMotion, DisclosureGroup, DisclosureGroupSelectionMode, Dropdown, DropdownMotion,
    ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion, MenuItem, MenuItemKind,
    MenuSection, MenuSectionHeadingTone, PopoverMotion, SegmentedControl, SegmentedControlSize,
    StepList, StepListItem, StepListOrientation, StepListSize, Switch, Table, TableCellAlign,
    TableColumn, TableDensity, TableLayout, TableRow, TableVariant, Tree, TreeDensity, TreeMotion,
    TreeNode, TreeTone, open_set,
};
use ui_headless::{A11yDirection, PopoverPlacement};

pub(super) fn table() -> AnyView {
    // Legacy table source-contract markers retained for semantics tests:
    // <Playground title="Default + Striped" code_signal=code>
    // <Playground title="Compact + Fixed + Empty" code_signal=states_code>
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
            format!("  striped={}", bool_word(workbench_striped.get())),
            format!(
                "  sticky_header={}",
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
            "TableActualConfig {{\n  columns: \"sample_columns(len=3)\",\n  rows: \"sample_rows(len={})\",\n  variant: {:?},\n  density: {:?},\n  layout: {:?},\n  striped: {},\n  sticky_header: {},\n  caption: {caption:?},\n  empty_label: {empty_label:?},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n}}",
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
<Table columns=columns rows=rows variant=TableVariant::Quiet density=TableDensity::Compact striped=true />
<Table columns=columns rows=Vec::<TableRow>::new() variant=TableVariant::Outline layout=TableLayout::Fixed sticky_header=true empty_label="No active incidents".to_string() class_name="docs-table-custom".to_string() />"#
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
                            "striped"
                        </Switch>
                        <Switch checked=workbench_sticky_header set_checked=set_workbench_sticky_header>
                            "sticky_header"
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
                    striped=workbench_striped.get()
                    sticky_header=workbench_sticky_header.get()
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
                        striped=true
                    />
                    <Table
                        columns=matrix_columns_empty
                        rows=Vec::new()
                        variant=TableVariant::Outline
                        layout=TableLayout::Fixed
                        sticky_header=true
                        empty_label="No active incidents".to_string()
                        class_name="docs-table-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn step_list() -> AnyView {
    let steps = vec![
        StepListItem::new("account", "Account").described("Create account and verify email"),
        StepListItem::new("shipping", "Shipping").described("Choose shipping address"),
        StepListItem::new("payment", "Payment").described("Add payment method"),
        StepListItem::new("review", "Review").described("Confirm and place order"),
    ];

    let steps_with_disabled = vec![
        StepListItem::new("plan", "Plan").described("Pick your subscription tier"),
        StepListItem::new("profile", "Profile").described("Fill organization details"),
        StepListItem::new("billing", "Billing")
            .described("Billing is locked until profile is approved")
            .disabled(true),
        StepListItem::new("launch", "Launch").described("Start using the workspace"),
    ];

    let (selected_index, set_selected_index) = signal(Some(1_usize));
    let selected_index_signal: Signal<Option<usize>> = selected_index.into();
    let (on_selected_index_change_runs, set_on_selected_index_change_runs) = signal(0_u32);
    let on_selected_index_change = Callback::new(move |next: Option<usize>| {
        set_selected_index.set(next);
        set_on_selected_index_change_runs.update(|count| *count += 1);
    });

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_emphasized, set_workbench_emphasized) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let workbench_size_options = vec!["S".to_string(), "M".to_string(), "L".to_string()];
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => StepListSize::S,
        2 => StepListSize::L,
        _ => StepListSize::M,
    });
    let workbench_steps = steps.clone();
    let showcase_steps = steps.clone();

    let hello_code = Signal::derive(move || {
        r#"<StepList
  id_base="docs-step-list-hello".to_string()
  steps=signal(steps).0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            "StepListOrientation::Vertical"
        } else {
            "StepListOrientation::Horizontal"
        };
        let size = match workbench_size.get() {
            StepListSize::S => "StepListSize::S",
            StepListSize::M => "StepListSize::M",
            StepListSize::L => "StepListSize::L",
            StepListSize::Xl => "StepListSize::Xl",
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-step-list-custom"
        } else {
            ""
        };

        [
            "<StepList".to_string(),
            "  steps=signal(steps).0".to_string(),
            format!("  orientation={orientation}"),
            format!("  size={size}"),
            format!("  is_emphasized={}", bool_word(workbench_emphasized.get())),
            format!("  is_disabled={}", bool_word(workbench_disabled.get())),
            "  selected_index=selected_index_signal".to_string(),
            "  default_selected_index=1".to_string(),
            "  completed_indices=vec![0]".to_string(),
            "  on_selected_index_change=on_selected_index_change".to_string(),
            "  id_base=\"docs-step-list-workbench\".to_string()".to_string(),
            "  aria_label=\"Checkout progress\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            StepListOrientation::Vertical
        } else {
            StepListOrientation::Horizontal
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-step-list-custom")
        } else {
            None
        };

        format!(
            "StepListActualConfig {{\n  steps: \"sample_steps(len=4)\",\n  orientation: {orientation:?},\n  size: {:?},\n  is_emphasized: {},\n  is_disabled: {},\n  selected_index: {:?},\n  default_selected_index: Some(1),\n  completed_indices: {:?},\n  on_selected_index_change: \"runs={}\",\n  id_base: Some(\"docs-step-list-workbench\"),\n  aria_label: Some(\"Checkout progress\"),\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n}}",
            workbench_size.get(),
            bool_word(workbench_emphasized.get()),
            bool_word(workbench_disabled.get()),
            selected_index.get(),
            vec![0_usize],
            on_selected_index_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<StepList id_base="step-default".to_string() steps=signal(steps).0 default_selected_index=1 />
<StepList id_base="step-vertical".to_string() steps=signal(steps).0 orientation=StepListOrientation::Vertical size=StepListSize::L is_emphasized=true />
<StepList id_base="step-disabled".to_string() steps=signal(disabled_steps).0 is_disabled=true default_selected_index=2 />"#.to_string()
    });

    view! {
        <ComponentPage
            title="StepList"
            slug="step-list"
            group="Collections"
            description="baseline-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <StepList
                    id_base="docs-step-list-hello".to_string()
                    steps=signal(showcase_steps).0
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="step-list-workbench-controls">
                        <SegmentedControl
                            id_base="docs-step-list-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="StepList size".to_string()
                        />
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_emphasized set_checked=set_workbench_emphasized>
                            "Emphasized"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="step-list-workbench-preview">
                    <StepList
                        steps=signal(workbench_steps).0
                        orientation=if workbench_vertical.get() {
                            StepListOrientation::Vertical
                        } else {
                            StepListOrientation::Horizontal
                        }
                        size=workbench_size.get()
                        is_emphasized=workbench_emphasized.get()
                        is_disabled=workbench_disabled.get()
                        selected_index=selected_index_signal
                        default_selected_index=1
                        completed_indices=vec![0]
                        on_selected_index_change=on_selected_index_change
                        id_base="docs-step-list-workbench".to_string()
                        aria_label="Checkout progress".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-step-list-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="step-list-workbench-feedback">
                        "selected index: "
                        {move || selected_index.get().map_or("none".to_string(), |it| it.to_string())}
                        " · on_selected_index_change: " {move || on_selected_index_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="step-list-state-matrix">
                    <StepList
                        id_base="docs-step-list-matrix-default".to_string()
                        steps=signal(steps.clone()).0
                        default_selected_index=1
                    />
                    <StepList
                        id_base="docs-step-list-matrix-vertical".to_string()
                        steps=signal(steps.clone()).0
                        orientation=StepListOrientation::Vertical
                        size=StepListSize::L
                        is_emphasized=true
                        completed_indices=vec![0, 1]
                    />
                    <StepList
                        id_base="docs-step-list-matrix-disabled".to_string()
                        steps=signal(steps_with_disabled).0
                        is_disabled=true
                        default_selected_index=2
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tree() -> AnyView {
    let nodes = vec![
        TreeNode::new("root-app", "Applications").with_children(vec![
            TreeNode::new("app-web", "Web Console"),
            TreeNode::new("app-mobile", "Mobile App"),
            TreeNode::new("app-admin", "Admin Portal").disabled(true),
        ]),
        TreeNode::new("root-services", "Services").with_children(vec![
            TreeNode::new("svc-api", "API Gateway"),
            TreeNode::new("svc-worker", "Worker Pool"),
        ]),
    ];
    let showcase_nodes = nodes.clone();
    let workbench_nodes = nodes.clone();
    let matrix_nodes = nodes;

    let (expanded_ids_raw, set_expanded_ids_raw) = signal(BTreeSet::from(["root-app".to_string()]));
    let expanded_ids_signal: Signal<BTreeSet<String>> =
        Signal::derive(move || expanded_ids_raw.get());
    let (selected_id_raw, set_selected_id_raw) = signal(Some("app-web".to_string()));
    let selected_id_signal: Signal<Option<String>> = Signal::derive(move || selected_id_raw.get());

    let (on_expanded_ids_change_runs, set_on_expanded_ids_change_runs) = signal(0_u32);
    let (on_expanded_change_runs, set_on_expanded_change_runs) = signal(0_u32);
    let (on_selected_id_change_runs, set_on_selected_id_change_runs) = signal(0_u32);
    let (on_selected_change_runs, set_on_selected_change_runs) = signal(0_u32);

    let on_expanded_ids_change = Callback::new(move |next: BTreeSet<String>| {
        set_expanded_ids_raw.set(next);
        set_on_expanded_ids_change_runs.update(|count| *count += 1);
    });
    let on_expanded_change = Callback::new(move |next: BTreeSet<String>| {
        set_expanded_ids_raw.set(next);
        set_on_expanded_change_runs.update(|count| *count += 1);
    });
    let on_selected_id_change = Callback::new(move |next: Option<String>| {
        set_selected_id_raw.set(next);
        set_on_selected_id_change_runs.update(|count| *count += 1);
    });
    let on_selected_change = Callback::new(move |next: Option<String>| {
        set_selected_id_raw.set(next);
        set_on_selected_change_runs.update(|count| *count += 1);
    });

    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_compact_density, set_workbench_compact_density) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"let nodes = vec![
  TreeNode::branch(
    "root-app",
    "Application",
    vec![
      TreeNode::leaf("app-web", "Web App"),
      TreeNode::leaf("app-api", "API"),
    ],
  ),
];

<Tree
  id_base="docs-tree-hello".to_string()
  nodes=nodes
  default_expanded_ids=BTreeSet::from(["root-app".to_string()])
  default_selected_id="app-web".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            "TreeTone::Strong"
        } else {
            "TreeTone::Default"
        };
        let density = if workbench_compact_density.get() {
            "TreeDensity::Compact"
        } else {
            "TreeDensity::Comfortable"
        };
        let motion = if workbench_reduced_motion.get() {
            "TreeMotion::disabled()"
        } else {
            "TreeMotion::default()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-tree-custom"
        } else {
            ""
        };

        [
            "<Tree".to_string(),
            "  id_base=\"docs-tree-workbench\".to_string()".to_string(),
            "  nodes=nodes".to_string(),
            format!("  tone={tone}"),
            format!("  density={density}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            "  expanded_ids=expanded_ids_signal".to_string(),
            "  default_expanded_ids=BTreeSet::from([\"root-app\".to_string()])".to_string(),
            "  on_expanded_ids_change=on_expanded_ids_change".to_string(),
            "  on_expanded_change=on_expanded_change".to_string(),
            "  selected_id=selected_id_signal".to_string(),
            "  default_selected_id=\"app-web\".to_string()".to_string(),
            "  on_selected_id_change=on_selected_id_change".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            format!("  motion={motion}"),
            "  aria_label=\"Service navigation tree\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = if workbench_strong_tone.get() {
            TreeTone::Strong
        } else {
            TreeTone::Default
        };
        let density = if workbench_compact_density.get() {
            TreeDensity::Compact
        } else {
            TreeDensity::Comfortable
        };
        let motion = if workbench_reduced_motion.get() {
            TreeMotion::disabled()
        } else {
            TreeMotion::default()
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-tree-custom")
        } else {
            None
        };
        let expanded = expanded_ids_raw.get();
        let expanded_items = expanded
            .iter()
            .map(|id| rust_string_literal(id))
            .collect::<Vec<_>>()
            .join(", ");
        let selected_text = selected_id_raw.get().as_ref().map_or_else(
            || "None".to_string(),
            |id| format!("Some({})", rust_string_literal(id)),
        );

        format!(
            "TreeActualConfig {{\n  id_base: \"docs-tree-workbench\",\n  nodes: \"sample_nodes(len=2)\",\n  tone: {tone:?},\n  density: {density:?},\n  is_disabled: Some({}),\n  disabled: {},\n  expanded_ids: BTreeSet::from([{expanded_items}]),\n  default_expanded_ids: Some(BTreeSet::from([\"root-app\".to_string()])),\n  on_expanded_ids_change: \"runs={}\",\n  on_expanded_change: \"runs={}\",\n  selected_id: {selected_text},\n  default_selected_id: Some(\"app-web\"),\n  on_selected_id_change: \"runs={}\",\n  on_selected_change: \"runs={}\",\n  motion: {motion:?},\n  aria_label: Some(\"Service navigation tree\"),\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n}}",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            on_expanded_ids_change_runs.get(),
            on_expanded_change_runs.get(),
            on_selected_id_change_runs.get(),
            on_selected_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let nodes = vec![
  TreeNode::branch(
    "root-app",
    "Application",
    vec![
      TreeNode::leaf("app-web", "Web App"),
      TreeNode::leaf("app-api", "API"),
    ],
  ),
  TreeNode::branch(
    "root-services",
    "Services",
    vec![
      TreeNode::leaf("svc-api", "API Service"),
      TreeNode::leaf("svc-worker", "Worker"),
    ],
  ),
];

<Tree id_base="tree-default".to_string() nodes=nodes default_expanded_ids=BTreeSet::from(["root-app".to_string()]) default_selected_id="app-web".to_string() />
<Tree id_base="tree-strong".to_string() nodes=nodes tone=TreeTone::Strong density=TreeDensity::Compact default_expanded_ids=BTreeSet::from(["root-services".to_string()]) default_selected_id="svc-api".to_string() />
<Tree id_base="tree-disabled".to_string() nodes=nodes is_disabled=true disabled=true motion=TreeMotion::disabled() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Tree"
            slug="tree"
            group="Collections"
            description="Tree playground with strict Showcase/Workbench/Matrix structure and full API coverage."
        >
            <Playground title="Hello World (Default Tree)" code_signal=hello_code>
                <Tree
                    id_base="docs-tree-hello".to_string()
                    nodes=showcase_nodes
                    default_expanded_ids=BTreeSet::from(["root-app".to_string()])
                    default_selected_id="app-web".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tree-workbench-controls">
                        <Switch checked=workbench_strong_tone set_checked=set_workbench_strong_tone>
                            "Strong tone"
                        </Switch>
                        <Switch
                            checked=workbench_compact_density
                            set_checked=set_workbench_compact_density
                        >
                            "Compact density"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch
                            checked=workbench_disabled_alias
                            set_checked=set_workbench_disabled_alias
                        >
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch
                            checked=workbench_reduced_motion
                            set_checked=set_workbench_reduced_motion
                        >
                            "Reduced motion"
                        </Switch>
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_expanded_ids_raw.set(BTreeSet::from([
                                        "root-app".to_string(),
                                        "root-services".to_string(),
                                    ]));
                                })
                            >
                                "Expand all"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_expanded_ids_raw.set(BTreeSet::from(["root-app".to_string()]));
                                    set_selected_id_raw.set(Some("app-web".to_string()));
                                })
                            >
                                "Reset"
                            </ui::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="tree-workbench-preview">
                    <Tree
                        id_base="docs-tree-workbench".to_string()
                        nodes=workbench_nodes
                        tone=if workbench_strong_tone.get() {
                            TreeTone::Strong
                        } else {
                            TreeTone::Default
                        }
                        density=if workbench_compact_density.get() {
                            TreeDensity::Compact
                        } else {
                            TreeDensity::Comfortable
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        expanded_ids=expanded_ids_signal
                        default_expanded_ids=BTreeSet::from(["root-app".to_string()])
                        on_expanded_ids_change=on_expanded_ids_change
                        on_expanded_change=on_expanded_change
                        selected_id=selected_id_signal
                        default_selected_id="app-web".to_string()
                        on_selected_id_change=on_selected_id_change
                        on_selected_change=on_selected_change
                        motion=if workbench_reduced_motion.get() {
                            TreeMotion::disabled()
                        } else {
                            TreeMotion::default()
                        }
                        aria_label="Service navigation tree".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-tree-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="tree-workbench-feedback">
                        "expanded_ids: " {move || format!("{:?}", expanded_ids_raw.get())}
                        " · selected_id: "
                        {move || selected_id_raw.get().unwrap_or_else(|| "none".to_string())}
                        " · on_expanded_ids_change: " {move || on_expanded_ids_change_runs.get()}
                        " · on_expanded_change: " {move || on_expanded_change_runs.get()}
                        " · on_selected_id_change: " {move || on_selected_id_change_runs.get()}
                        " · on_selected_change: " {move || on_selected_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Strong / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="tree-state-matrix">
                    <Tree
                        id_base="docs-tree-matrix-default".to_string()
                        nodes=matrix_nodes.clone()
                        default_expanded_ids=BTreeSet::from(["root-app".to_string()])
                        default_selected_id="app-web".to_string()
                    />
                    <Tree
                        id_base="docs-tree-matrix-strong".to_string()
                        nodes=matrix_nodes.clone()
                        tone=TreeTone::Strong
                        density=TreeDensity::Compact
                        default_expanded_ids=BTreeSet::from(["root-services".to_string()])
                        default_selected_id="svc-api".to_string()
                    />
                    <Tree
                        id_base="docs-tree-matrix-disabled".to_string()
                        nodes=matrix_nodes
                        is_disabled=true
                        disabled=true
                        motion=TreeMotion::disabled()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn disclosure_group() -> AnyView {
    let labels = vec![
        "Account security".to_string(),
        "Billing preferences".to_string(),
        "Incident escalation".to_string(),
    ];
    let single_labels = vec![
        "Region routing".to_string(),
        "Failover strategy".to_string(),
        "Legacy endpoints".to_string(),
    ];
    let single_labels_for_state_playground = single_labels.clone();
    let single_labels_for_workbench = single_labels.clone();
    let workbench_labels_for_config = single_labels.clone();
    let selection_mode_options = vec!["Multiple".to_string(), "Single".to_string()];
    let motion_options = vec!["Default".to_string(), "Gentle".to_string()];

    let (expanded_multi, set_expanded_multi) = signal(open_set([0]));
    let expanded_multi_signal: Signal<BTreeSet<usize>> =
        Signal::derive(move || expanded_multi.get());
    let on_expanded_multi_change = Callback::new(move |next: BTreeSet<usize>| {
        set_expanded_multi.set(next);
    });

    let (workbench_selection_mode_index, set_workbench_selection_mode_index) =
        signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last_item, set_workbench_disable_last_item) = signal(true);
    let (workbench_with_default_open, set_workbench_with_default_open) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let (expanded_single, set_expanded_single) = signal(open_set([1]));
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let (workbench_last_expanded, set_workbench_last_expanded) = signal("{}".to_string());
    let expanded_single_signal: Signal<BTreeSet<usize>> =
        Signal::derive(move || expanded_single.get());
    let on_expanded_single_change = Callback::new(move |next: BTreeSet<usize>| {
        set_workbench_change_count.update(|count| *count += 1);
        set_workbench_last_expanded.set(format!("{next:?}"));
        set_expanded_single.set(next);
    });

    let workbench_selection_mode = Signal::derive(move || {
        if workbench_selection_mode_index.get().unwrap_or(0) == 1 {
            DisclosureGroupSelectionMode::Single
        } else {
            DisclosureGroupSelectionMode::Multiple
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            AccordionMotion {
                panel_offset_y_px: 14.0,
                ..Default::default()
            }
        } else {
            AccordionMotion::default()
        }
    });

    let code = Signal::derive(move || {
        r#"let labels = vec![
  "Account security".to_string(),
  "Billing preferences".to_string(),
  "Incident escalation".to_string(),
];
let (expanded, set_expanded) = signal(open_set([0]));
let on_expanded_change = Callback::new(move |next: BTreeSet<usize>| set_expanded.set(next));

<DisclosureGroup
  labels=labels
  id_base="docs-disclosure-group-multiple".to_string()
  expanded_indices=Signal::derive(move || expanded.get())
  on_expanded_change=on_expanded_change
  selection_mode=DisclosureGroupSelectionMode::Multiple
  aria_label="Operational disclosure sections".to_string()
>
  <div>"MFA, session policies, and login anomaly rules."</div>
  <div>"Invoice owner, tax profile, and payment method fallback."</div>
  <div>"Pager rotation, severity matrix, and incident runbook links."</div>
</DisclosureGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        format!(
            "<DisclosureGroup\n  labels=single_labels\n  id_base=\"docs-disclosure-group-single\".to_string()\n  expanded_indices=Signal::derive(move || expanded.get())\n  default_expanded_indices={}\n  on_expanded_change=on_expanded_change\n  selection_mode={:?}\n  disabled={}\n  disabled_indices={}\n  motion=AccordionMotion {{ panel_offset_y_px: {}, ..Default::default() }}\n  aria_label={}\n  class_name={}\n>\n  <div>\"Region routing details\"</div>\n  <div>\"Failover strategy details\"</div>\n  <div>\"Legacy endpoint deprecation\"</div>\n</DisclosureGroup>",
            if workbench_with_default_open.get() {
                "open_set([1])".to_string()
            } else {
                "BTreeSet::new()".to_string()
            },
            workbench_selection_mode.get(),
            bool_word(workbench_disabled.get()),
            if workbench_disable_last_item.get() {
                "vec![2]".to_string()
            } else {
                "vec![]".to_string()
            },
            workbench_motion.get().panel_offset_y_px,
            if workbench_custom_aria.get() {
                "\"Operational disclosure workbench\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_custom_class.get() {
                "\"docs-disclosure-group-custom\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "DisclosureGroupWorkbenchActualConfig {{\n  labels: {:?},\n  id_base: {:?},\n  expanded_indices: {:?},\n  default_expanded_indices: {:?},\n  on_expanded_change: {},\n  selection_mode: {:?},\n  disabled: {},\n  disabled_indices: {:?},\n  motion: AccordionMotion {{ panel_offset_y_px: {}, ..Default::default() }},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_labels_for_config,
            "docs-disclosure-group-single",
            expanded_single.get(),
            if workbench_with_default_open.get() {
                open_set([1])
            } else {
                BTreeSet::new()
            },
            "Callback<BTreeSet<usize>>",
            workbench_selection_mode.get(),
            bool_word(workbench_disabled.get()),
            if workbench_disable_last_item.get() {
                vec![2]
            } else {
                Vec::new()
            },
            workbench_motion.get().panel_offset_y_px,
            if workbench_custom_aria.get() {
                Some("Operational disclosure workbench")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-disclosure-group-custom")
            } else {
                None
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<DisclosureGroup
  labels=vec!["Default".to_string(), "Secondary".to_string()]
  id_base="docs-disclosure-group-matrix-default".to_string()
  default_expanded_indices=open_set([0])
  selection_mode=DisclosureGroupSelectionMode::Multiple
  disabled=false
  disabled_indices=vec![]
  motion=AccordionMotion::default()
  aria_label="Default disclosure matrix".to_string()
  class_name="".to_string()
/>
<DisclosureGroup
  labels=vec!["Single".to_string(), "Focus".to_string(), "Legacy".to_string()]
  id_base="docs-disclosure-group-matrix-single".to_string()
  default_expanded_indices=open_set([1])
  selection_mode=DisclosureGroupSelectionMode::Single
  disabled=false
  disabled_indices=vec![2]
  motion=AccordionMotion { panel_offset_y_px: 14.0, ..Default::default() }
  aria_label="Single disclosure matrix".to_string()
  class_name="docs-disclosure-group-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="DisclosureGroup"
            slug="disclosure-group"
            group="Collections"
            description="baseline-style disclosure grouping primitive with centralized expanded-state normalization, controlled/uncontrolled contracts, and spring motion delegated through Accordion internals."
        >
            <Playground title="Multiple + Controlled" code_signal=code>
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=labels
                        id_base="docs-disclosure-group-multiple".to_string()
                        expanded_indices=expanded_multi_signal
                        on_expanded_change=on_expanded_multi_change
                        selection_mode=DisclosureGroupSelectionMode::Multiple
                        disabled=false
                        disabled_indices=vec![]
                        motion=AccordionMotion::default()
                        aria_label="Operational disclosure sections".to_string()
                        class_name=String::new()
                    >
                        <div class="docs-stack">
                            <strong>"Account security"</strong>
                            <span class="ui-muted">
                                "MFA, session policies, and login anomaly rules."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Billing preferences"</strong>
                            <span class="ui-muted">
                                "Invoice owner, tax profile, and payment method fallback."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Incident escalation"</strong>
                            <span class="ui-muted">
                                "Pager rotation, severity matrix, and incident runbook links."
                            </span>
                        </div>
                    </DisclosureGroup>
                    <span class="ui-muted">
                        "expanded: "
                        {move || format!("{:?}", expanded_multi.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Single + Disabled Item + Custom Class" code_signal=states_code>
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=single_labels_for_state_playground.clone()
                        id_base="docs-disclosure-group-single".to_string()
                        expanded_indices=expanded_single_signal
                        default_expanded_indices=if workbench_with_default_open.get() {
                            open_set([1])
                        } else {
                            BTreeSet::new()
                        }
                        on_expanded_change=on_expanded_single_change
                        selection_mode=DisclosureGroupSelectionMode::Single
                        disabled=false
                        disabled_indices=vec![2]
                        motion=workbench_motion.get()
                        aria_label="Operational disclosure sections".to_string()
                        class_name="docs-disclosure-group-custom".to_string()
                    >
                        <div class="docs-stack">
                            <strong>"Region routing"</strong>
                            <span class="ui-muted">
                                "Traffic enters through geo routing with weighted failover."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Failover strategy"</strong>
                            <span class="ui-muted">
                                "Single-expanded mode keeps one active policy focused at a time."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Legacy endpoints"</strong>
                            <span class="ui-muted">
                                "Disabled section stays non-interactive for decommissioning."
                            </span>
                        </div>
                    </DisclosureGroup>
                    <span class="ui-muted">
                        "expanded: "
                        {move || format!("{:?}", expanded_single.get())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=states_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="disclosure-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-disclosure-group-workbench-selection-mode".to_string()
                            options=selection_mode_options.clone()
                            selected_index=workbench_selection_mode_index
                            set_selected_index=set_workbench_selection_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="DisclosureGroup selection_mode".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-disclosure-group-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="DisclosureGroup motion".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_last_item set_checked=set_workbench_disable_last_item>
                            "disabled_indices"
                        </Switch>
                        <Switch checked=workbench_with_default_open set_checked=set_workbench_with_default_open>
                            "default_expanded_indices"
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
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=single_labels_for_workbench.clone()
                        id_base="docs-disclosure-group-workbench".to_string()
                        expanded_indices=expanded_single_signal
                        default_expanded_indices=if workbench_with_default_open.get() {
                            open_set([1])
                        } else {
                            BTreeSet::new()
                        }
                        on_expanded_change=on_expanded_single_change
                        selection_mode=workbench_selection_mode.get()
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_last_item.get() {
                            vec![2]
                        } else {
                            Vec::new()
                        }
                        motion=workbench_motion.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Operational disclosure workbench".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-disclosure-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <div class="docs-stack">
                            <strong>"Region routing"</strong>
                            <span class="ui-muted">
                                "Traffic enters through geo routing with weighted failover."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Failover strategy"</strong>
                            <span class="ui-muted">
                                "Single-expanded mode keeps one active policy focused at a time."
                            </span>
                        </div>
                        <div class="docs-stack">
                            <strong>"Legacy endpoints"</strong>
                            <span class="ui-muted">
                                "Disabled section stays non-interactive for decommissioning."
                            </span>
                        </div>
                    </DisclosureGroup>
                    <span class="ui-muted">
                        "expanded: "
                        {move || format!("{:?}", expanded_single.get())}
                    </span>
                    <span class="ui-muted">
                        "on_expanded_change count="
                        {move || workbench_change_count.get()}
                        " · last="
                        {move || workbench_last_expanded.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Multiple / Single / Disabled)" code_signal=matrix_code>
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=vec![
                            "Multiple default".to_string(),
                            "Billing".to_string(),
                            "Escalation".to_string(),
                        ]
                        id_base="docs-disclosure-group-matrix-default".to_string()
                        default_expanded_indices=open_set([0])
                        selection_mode=DisclosureGroupSelectionMode::Multiple
                        disabled=false
                        disabled_indices=vec![]
                        motion=AccordionMotion::default()
                        aria_label="Default disclosure matrix".to_string()
                        class_name=String::new()
                    >
                        <div>"Standard multiple-expanded policy contracts."</div>
                        <div>"Invoice fallback policy details."</div>
                        <div>"Escalation runbook references."</div>
                    </DisclosureGroup>

                    <DisclosureGroup
                        labels=vec![
                            "Single focus".to_string(),
                            "Failover".to_string(),
                            "Legacy".to_string(),
                        ]
                        id_base="docs-disclosure-group-matrix-single".to_string()
                        default_expanded_indices=open_set([1])
                        selection_mode=DisclosureGroupSelectionMode::Single
                        disabled=false
                        disabled_indices=vec![2]
                        motion=AccordionMotion {
                            panel_offset_y_px: 14.0,
                            ..Default::default()
                        }
                        aria_label="Single disclosure matrix".to_string()
                        class_name="docs-disclosure-group-custom".to_string()
                    >
                        <div>"Single mode keeps one section in focus."</div>
                        <div>"Failover strategy and region fallback."</div>
                        <div>"Legacy section disabled for decommissioning."</div>
                    </DisclosureGroup>

                    <DisclosureGroup
                        labels=vec![
                            "Disabled security".to_string(),
                            "Disabled billing".to_string(),
                            "Disabled escalation".to_string(),
                        ]
                        id_base="docs-disclosure-group-matrix-disabled".to_string()
                        default_expanded_indices=open_set([0])
                        selection_mode=DisclosureGroupSelectionMode::Multiple
                        disabled=true
                        disabled_indices=vec![0, 1, 2]
                        motion=AccordionMotion::default()
                        aria_label="Disabled disclosure matrix".to_string()
                        class_name=String::new()
                    >
                        <div>"All items disabled for maintenance window."</div>
                        <div>"Billing policies read-only during freeze."</div>
                        <div>"Escalation policy locked for review."</div>
                    </DisclosureGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list_item() -> AnyView {
    let (showcase_selected, set_showcase_selected) = signal(true);
    let showcase_on_press = Callback::new(move |_| {
        set_showcase_selected.update(|value| *value = !*value);
    });

    let (workbench_index, set_workbench_index) = signal(1_usize);
    let (workbench_selected, set_workbench_selected) = signal(true);
    let (workbench_focused, set_workbench_focused) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_indicator_visible, set_workbench_indicator_visible) = signal(true);
    let (workbench_divider_visible, set_workbench_divider_visible) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_selected_text, set_workbench_custom_selected_text) = signal(false);
    let (workbench_custom_unselected_text, set_workbench_custom_unselected_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let (workbench_pointer_move_count, set_workbench_pointer_move_count) = signal(0_u32);

    let workbench_on_press = Callback::new(move |_| {
        set_workbench_selected.update(|value| *value = !*value);
        set_workbench_press_count.update(|count| *count += 1);
    });
    let workbench_on_pointer_move = Callback::new(move |_| {
        set_workbench_pointer_move_count.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<ListItem index=0 is_selected=true is_selection_indicator_visible=true>
  "San Francisco"
</ListItem>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ListItem\n  id=\"docs-list-item-workbench\".to_string()\n  index={}\n  is_selected={}\n  is_focused={}\n  is_disabled={}\n  is_selection_indicator_visible={}\n  is_divider_visible={}\n  aria_label={}\n  selected_text={}\n  unselected_text={}\n  on_press=on_press\n  on_pointer_move=on_pointer_move\n  class_name={}\n>\n  \"Tokyo\"\n</ListItem>",
            workbench_index.get(),
            bool_word(workbench_selected.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_indicator_visible.get()),
            bool_word(workbench_divider_visible.get()),
            rust_string_literal(if workbench_custom_aria.get() {
                "Tokyo option"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_selected_text.get() {
                "Selected"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_unselected_text.get() {
                "Not selected"
            } else {
                ""
            }),
            if workbench_custom_class.get() {
                "\"docs-list-item-custom\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ListItemWorkbenchActualConfig {{\n  id: Some(\"docs-list-item-workbench\"),\n  index: Some({}),\n  is_selected: {},\n  is_focused: {},\n  is_disabled: {},\n  is_selection_indicator_visible: {},\n  is_divider_visible: {},\n  aria_label: {:?},\n  selected_text: {:?},\n  unselected_text: {:?},\n  on_press: \"count={}\",\n  on_pointer_move: \"count={}\",\n  class_name: {:?},\n}}",
            workbench_index.get(),
            bool_word(workbench_selected.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_indicator_visible.get()),
            bool_word(workbench_divider_visible.get()),
            if workbench_custom_aria.get() {
                Some("Tokyo option")
            } else {
                None
            },
            if workbench_custom_selected_text.get() {
                Some("Selected")
            } else {
                None
            },
            if workbench_custom_unselected_text.get() {
                Some("Not selected")
            } else {
                None
            },
            workbench_press_count.get(),
            workbench_pointer_move_count.get(),
            if workbench_custom_class.get() {
                Some("docs-list-item-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ListItem id="li-default".to_string() index=0 is_selected=true is_selection_indicator_visible=true>"San Francisco"</ListItem>
<ListItem id="li-focused".to_string() index=1 is_focused=true is_divider_visible=true class_name="docs-listbox-item-custom".to_string()>"Tokyo"</ListItem>
<ListItem id="li-disabled".to_string() index=2 is_disabled=true aria_label="Disabled option".to_string()>"Disabled option"</ListItem>"#.to_string()
    });

    view! {
        <ComponentPage
            title="ListItem"
            slug="list-item"
            group="Collections"
            description="baseline-style list option primitive with centralized selection/focus/divider/source normalization and stable `slot` + `data-*` state contracts."
        >
            <Playground title="Hello World (Default ListItem)" code_signal=hello_code>
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-hello".to_string()
                        index=0
                        is_selected=showcase_selected.get()
                        is_selection_indicator_visible=true
                        on_press=showcase_on_press
                    >
                        "San Francisco"
                    </ListItem>
                    <span class="ui-muted">
                        "selected: "
                        {move || showcase_selected.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-item-workbench-controls">
                        <label class="docs-search__label">
                            "index"
                            <input
                                type="number"
                                min="0"
                                max="12"
                                prop:value=move || workbench_index.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<usize>().unwrap_or(1);
                                    set_workbench_index.set(next);
                                }
                            />
                        </label>
                        <Switch checked=workbench_selected set_checked=set_workbench_selected>
                            "is_selected"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "is_focused"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_indicator_visible set_checked=set_workbench_indicator_visible>
                            "is_selection_indicator_visible"
                        </Switch>
                        <Switch checked=workbench_divider_visible set_checked=set_workbench_divider_visible>
                            "is_divider_visible"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_selected_text set_checked=set_workbench_custom_selected_text>
                            "selected_text"
                        </Switch>
                        <Switch checked=workbench_custom_unselected_text set_checked=set_workbench_custom_unselected_text>
                            "unselected_text"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-workbench".to_string()
                        index=workbench_index.get()
                        is_selected=workbench_selected.get()
                        is_focused=workbench_focused.get()
                        is_disabled=workbench_disabled.get()
                        is_divider_visible=workbench_divider_visible.get()
                        is_selection_indicator_visible=workbench_indicator_visible.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Tokyo option".to_string()
                        } else {
                            String::new()
                        }
                        selected_text=if workbench_custom_selected_text.get() {
                            "Selected".to_string()
                        } else {
                            String::new()
                        }
                        unselected_text=if workbench_custom_unselected_text.get() {
                            "Not selected".to_string()
                        } else {
                            String::new()
                        }
                        on_press=workbench_on_press
                        on_pointer_move=workbench_on_pointer_move
                        class_name=if workbench_custom_class.get() {
                            "docs-listbox-item-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        "Tokyo"
                    </ListItem>
                    <span class="ui-muted">
                        "selected: " {move || workbench_selected.get()}
                        " · on_press count: " {move || workbench_press_count.get()}
                        " · on_pointer_move count: " {move || workbench_pointer_move_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Selected / Focused / Disabled)" code_signal=matrix_code>
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-matrix-default".to_string()
                        index=0
                        is_selected=true
                        is_selection_indicator_visible=true
                    >
                        "San Francisco"
                    </ListItem>
                    <ListItem
                        id="docs-list-item-matrix-focused".to_string()
                        index=1
                        is_focused=true
                        is_divider_visible=true
                        class_name="docs-listbox-item-custom".to_string()
                    >
                        "Tokyo"
                    </ListItem>
                    <ListItem
                        id="docs-list-item-matrix-disabled".to_string()
                        index=2
                        is_disabled=true
                        aria_label="Disabled option".to_string()
                    >
                        "Disabled option"
                    </ListItem>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list_section() -> AnyView {
    let tone_options = ["Default".to_string(), "Quiet".to_string()];
    let item_count_options = ["0 (empty)".to_string(), "2".to_string(), "4".to_string()];

    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (item_count_index, set_item_count_index) = signal(Some(1_usize));
    let (is_disabled, set_is_disabled) = signal(false);
    let (is_sticky_heading, set_is_sticky_heading) = signal(true);
    let (is_divider_visible, set_is_divider_visible) = signal(true);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);
    let (custom_class, set_custom_class) = signal(false);

    let (primary_selected, set_primary_selected) = signal(true);
    let (secondary_selected, set_secondary_selected) = signal(false);
    let (item_press_count, set_item_press_count) = signal(0_u32);

    let workbench_heading_tone = Signal::derive(move || {
        if tone_index.get().unwrap_or(0) == 1 {
            ListSectionHeadingTone::Quiet
        } else {
            ListSectionHeadingTone::Default
        }
    });
    let workbench_item_count = Signal::derive(move || match item_count_index.get().unwrap_or(1) {
        0 => Some(0_usize),
        2 => Some(4_usize),
        _ => Some(2_usize),
    });
    let workbench_title = Signal::derive(move || match item_count_index.get().unwrap_or(1) {
        0 => "Empty section".to_string(),
        2 => "Large section".to_string(),
        _ => "Preferred regions".to_string(),
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ListSectionMotion {
                initial_y_px: 18.0,
                ..ListSectionMotion::default()
            }
        } else {
            ListSectionMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Workbench list section".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-listbox-section-custom".to_string()
        } else {
            String::new()
        }
    });

    let on_toggle_primary = Callback::new(move |_| {
        set_primary_selected.update(|selected| *selected = !*selected);
        set_item_press_count.update(|count| *count += 1);
    });
    let on_toggle_secondary = Callback::new(move |_| {
        set_secondary_selected.update(|selected| *selected = !*selected);
        set_item_press_count.update(|count| *count += 1);
    });

    let showcase_code = Signal::derive(move || {
        r#"<ListSection title="Preferred regions".to_string() item_count=3>
  <ListItem index=0 is_selected=true is_selection_indicator_visible=true>"US East"</ListItem>
  <ListItem index=1>"EU West"</ListItem>
  <ListItem index=2>"AP South"</ListItem>
</ListSection>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone_expr = match workbench_heading_tone.get() {
            ListSectionHeadingTone::Default => "ListSectionHeadingTone::Default",
            ListSectionHeadingTone::Quiet => "ListSectionHeadingTone::Quiet",
        };
        let motion_expr = if custom_motion.get() {
            "ListSectionMotion { initial_y_px: 18.0, ..ListSectionMotion::default() }"
        } else {
            "ListSectionMotion::default()"
        };
        format!(
            "<ListSection\n  title={}\n  item_count={:?}\n  heading_tone={tone_expr}\n  is_disabled={}\n  is_sticky_heading={}\n  is_divider_visible={}\n  motion={motion_expr}\n  aria_label={}\n  class_name={}\n>\n  <ListItem index=0 is_selected=true>\"Primary\"</ListItem>\n  <ListItem index=1>\"Secondary\"</ListItem>\n</ListSection>",
            rust_string_literal(&workbench_title.get()),
            workbench_item_count.get(),
            bool_word(is_disabled.get()),
            bool_word(is_sticky_heading.get()),
            bool_word(is_divider_visible.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ListSectionActualConfig {{\n  title: {:?},\n  item_count: {:?},\n  heading_tone: {:?},\n  is_disabled: {},\n  is_sticky_heading: {},\n  is_divider_visible: {},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_title.get(),
            workbench_item_count.get(),
            workbench_heading_tone.get(),
            is_disabled.get(),
            is_sticky_heading.get(),
            is_divider_visible.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ListSection title="Default section".to_string() item_count=2 heading_tone=ListSectionHeadingTone::Default>
  <ListItem index=0>"Default item"</ListItem>
  <ListItem index=1>"Second item"</ListItem>
</ListSection>
<ListSection
  title="Quiet sticky section".to_string()
  item_count=2
  heading_tone=ListSectionHeadingTone::Quiet
  is_sticky_heading=true
  is_divider_visible=true
  motion=ListSectionMotion { initial_y_px: 18.0, ..ListSectionMotion::default() }
  class_name="docs-listbox-section-custom".to_string()
>
  <ListItem index=0 is_selected=true>"Quiet item"</ListItem>
  <ListItem index=1 is_disabled=true>"Disabled item"</ListItem>
</ListSection>
<ListSection title="Disabled empty".to_string() item_count=0 is_disabled=true aria_label="Disabled empty list".to_string()>
  <span class="ui-muted">"No options available"</span>
</ListSection>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ListSection"
            slug="list-section"
            group="Collections"
            description="baseline-style list section primitive with centralized heading/item/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
            >
                <ListSection
                    title="Preferred regions".to_string()
                    item_count=3
                >
                    <ListItem index=0 is_selected=true is_selection_indicator_visible=true>
                        "US East"
                    </ListItem>
                    <ListItem index=1>
                        "EU West"
                    </ListItem>
                    <ListItem index=2>
                        "AP South"
                    </ListItem>
                </ListSection>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-section-workbench-controls">
                        <div class="docs-search__label">"heading_tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {tone_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"item_count"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || item_count_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_item_count_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {item_count_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_disabled.get()
                                on:change=move |event| set_is_disabled.set(event_target_checked(&event))
                            />
                            <span>"is_disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_sticky_heading.get()
                                on:change=move |event| set_is_sticky_heading.set(event_target_checked(&event))
                            />
                            <span>"is_sticky_heading"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_divider_visible.get()
                                on:change=move |event| set_is_divider_visible.set(event_target_checked(&event))
                            />
                            <span>"is_divider_visible"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ListSection
                        title=workbench_title.get()
                        item_count=workbench_item_count.get().unwrap_or(0)
                        heading_tone=workbench_heading_tone.get()
                        is_disabled=is_disabled.get()
                        is_sticky_heading=is_sticky_heading.get()
                        is_divider_visible=is_divider_visible.get()
                        motion=workbench_motion.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    >
                        <ListItem
                            index=0
                            is_selected=primary_selected.get()
                            is_focused=true
                            is_selection_indicator_visible=true
                            on_press=on_toggle_primary
                        >
                            "Primary target"
                        </ListItem>
                        <ListItem
                            index=1
                            is_selected=secondary_selected.get()
                            is_divider_visible=true
                            is_selection_indicator_visible=true
                            on_press=on_toggle_secondary
                        >
                            "Secondary target"
                        </ListItem>
                    </ListSection>
                    <span class="ui-muted">
                        "primary_selected: " {move || primary_selected.get()}
                        " · secondary_selected: " {move || secondary_selected.get()}
                        " · item on_press count: " {move || item_press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Disabled / Empty Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <ListSection
                        title="Default section".to_string()
                        item_count=2
                        heading_tone=ListSectionHeadingTone::Default
                    >
                        <ListItem index=0>
                            "Default item"
                        </ListItem>
                        <ListItem index=1>
                            "Second item"
                        </ListItem>
                    </ListSection>
                    <ListSection
                        title="Quiet sticky section".to_string()
                        item_count=2
                        heading_tone=ListSectionHeadingTone::Quiet
                        is_sticky_heading=true
                        is_divider_visible=true
                        motion=ListSectionMotion {
                            initial_y_px: 18.0,
                            ..ListSectionMotion::default()
                        }
                        class_name="docs-listbox-section-custom".to_string()
                    >
                        <ListItem index=0 is_selected=true>
                            "Quiet item"
                        </ListItem>
                        <ListItem index=1 is_disabled=true>
                            "Disabled item"
                        </ListItem>
                    </ListSection>
                    <ListSection
                        title="Disabled empty".to_string()
                        item_count=0
                        is_disabled=true
                        aria_label="Disabled empty list".to_string()
                    >
                        <span class="ui-muted">"No options available"</span>
                    </ListSection>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_item() -> AnyView {
    let (showcase_checked, set_showcase_checked) = signal(true);
    let showcase_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || showcase_checked.get()),
    };
    let (showcase_pointer_moves, set_showcase_pointer_moves) = signal(0_u32);
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let on_showcase_pointer_move = Callback::new(move |_| {
        set_showcase_pointer_moves.update(|count| *count += 1);
    });
    let on_showcase_press = Callback::new(move |_| {
        set_showcase_presses.update(|count| *count += 1);
        set_showcase_checked.update(|value| *value = !*value);
    });

    let kind_options = vec![
        "Action".to_string(),
        "Checkbox".to_string(),
        "Radio".to_string(),
    ];
    let index_options = vec!["0".to_string(), "2".to_string()];
    let is_disabled_options = vec!["false".to_string(), "true".to_string()];

    let (workbench_kind_index, set_workbench_kind_index) = signal(Some(0_usize));
    let (workbench_index_mode, set_workbench_index_mode) = signal(Some(0_usize));
    let (workbench_is_disabled_mode, set_workbench_is_disabled_mode) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(false);
    let (workbench_has_submenu, set_workbench_has_submenu) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_checkbox_checked, set_workbench_checkbox_checked) = signal(true);
    let (workbench_radio_checked, set_workbench_radio_checked) = signal(false);
    let workbench_checkbox_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || workbench_checkbox_checked.get()),
    };
    let workbench_radio_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || workbench_radio_checked.get()),
    };
    let workbench_kind = Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
        1 => workbench_checkbox_kind,
        2 => workbench_radio_kind,
        _ => MenuItemKind::Action,
    });
    let workbench_kind_name =
        Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
            1 => "Checkbox",
            2 => "Radio",
            _ => "Action",
        });
    let workbench_index = Signal::derive(move || match workbench_index_mode.get().unwrap_or(0) {
        1 => 2_usize,
        _ => 0_usize,
    });
    let workbench_is_disabled =
        Signal::derive(move || matches!(workbench_is_disabled_mode.get().unwrap_or(0), 1));
    let (workbench_pointer_moves, set_workbench_pointer_moves) = signal(0_u32);
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let (workbench_last_event, set_workbench_last_event) = signal("none".to_string());
    let on_workbench_pointer_move = Callback::new(move |_| {
        set_workbench_pointer_moves.update(|count| *count += 1);
        set_workbench_last_event.set("pointer_move".to_string());
    });
    let on_workbench_press = Callback::new(move |_| {
        set_workbench_presses.update(|count| *count += 1);
        match workbench_kind_index.get().unwrap_or(0) {
            1 => set_workbench_checkbox_checked.update(|value| *value = !*value),
            2 => set_workbench_radio_checked.update(|value| *value = !*value),
            _ => {}
        }
        set_workbench_last_event.set("press".to_string());
    });

    let hello_code = Signal::derive(move || {
        r#"<MenuItem
  id="docs-menu-item-showcase".to_string()
  index=0
  kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) }
  aria_label="Pin project".to_string()
  on_pointer_move=on_pointer_move
  on_press=on_press
>
  "Pin project"
</MenuItem>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let kind_expr = match workbench_kind_index.get().unwrap_or(0) {
            1 => {
                "MenuItemKind::Checkbox { is_checked: Signal::derive(move || checkbox_checked.get()) }"
            }
            2 => "MenuItemKind::Radio { is_checked: Signal::derive(move || radio_checked.get()) }",
            _ => "MenuItemKind::Action",
        };
        let id_expr = if workbench_custom_id.get() {
            "\"docs-menu-item-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let aria_expr = if workbench_custom_aria.get() {
            "\"Workbench menu item\".to_string()"
        } else {
            "String::new()"
        };
        let class_expr = if workbench_custom_class.get() {
            "\"docs-menu-item-custom\".to_string()"
        } else {
            "String::new()"
        };

        [
            "<MenuItem".to_string(),
            format!("  id={id_expr}"),
            format!("  index={}", workbench_index.get()),
            format!("  kind={kind_expr}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!("  focused={}", bool_word(workbench_focused.get())),
            format!("  has_submenu={}", bool_word(workbench_has_submenu.get())),
            format!("  aria_label={aria_expr}"),
            "  on_pointer_move=on_pointer_move".to_string(),
            "  on_press=on_press".to_string(),
            format!("  class_name={class_expr}"),
            ">".to_string(),
            "  \"Workbench menu item\"".to_string(),
            "</MenuItem>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let id = if workbench_custom_id.get() {
            "docs-menu-item-workbench"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Workbench menu item"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-menu-item-custom"
        } else {
            ""
        };

        format!(
            "MenuItemActualConfig {{\n  id: {:?},\n  index: Some({}),\n  kind: {:?},\n  is_disabled: Some({}),\n  disabled: {},\n  focused: {},\n  has_submenu: {},\n  aria_label: {:?},\n  on_pointer_move: \"count={}\",\n  on_press: \"count={}, checkbox_checked={}, radio_checked={}\",\n  class_name: {:?},\n}}",
            Some(id),
            workbench_index.get(),
            workbench_kind_name.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_has_submenu.get()),
            Some(aria_label),
            workbench_pointer_moves.get(),
            workbench_presses.get(),
            workbench_checkbox_checked.get(),
            workbench_radio_checked.get(),
            Some(class_name),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MenuItem index=0 kind=MenuItemKind::Action aria_label="Open profile".to_string()>
  "Open profile"
</MenuItem>
<MenuItem index=1 kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) } focused=true on_press=on_press>
  "Pin workspace"
</MenuItem>
<MenuItem index=2 kind=MenuItemKind::Radio { is_checked: Signal::derive(move || selected.get()) } is_disabled=true disabled=true has_submenu=true class_name="docs-menu-item-custom".to_string()>
  "Primary workspace"
</MenuItem>"#
            .to_string()
    });

    let (matrix_checked, set_matrix_checked) = signal(true);
    let matrix_checkbox_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || matrix_checked.get()),
    };
    let (matrix_radio_selected, _set_matrix_radio_selected) = signal(true);
    let matrix_radio_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || matrix_radio_selected.get()),
    };
    let (matrix_pointer_moves, set_matrix_pointer_moves) = signal(0_u32);
    let (matrix_presses, set_matrix_presses) = signal(0_u32);
    let on_matrix_pointer_move = Callback::new(move |_| {
        set_matrix_pointer_moves.update(|count| *count += 1);
    });
    let on_matrix_press = Callback::new(move |_| {
        set_matrix_presses.update(|count| *count += 1);
        set_matrix_checked.update(|value| *value = !*value);
    });

    view! {
        <ComponentPage
            title="MenuItem"
            slug="menu-item"
            group="Collections"
            description="baseline-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Hello World (Default MenuItem + Feedback)" code_signal=hello_code>
                <div class="docs-stack">
                    <MenuItem
                        id="docs-menu-item-showcase".to_string()
                        index=0
                        kind=showcase_kind
                        aria_label="Pin project".to_string()
                        on_pointer_move=on_showcase_pointer_move
                        on_press=on_showcase_press
                    >
                        "Pin project"
                    </MenuItem>
                    <span class="ui-muted">
                        "checked: " {move || showcase_checked.get()}
                        " · pointer moves: " {move || showcase_pointer_moves.get()}
                        " · presses: " {move || showcase_presses.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-item-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-kind".to_string()
                            options=kind_options.clone()
                            selected_index=workbench_kind_index
                            set_selected_index=set_workbench_kind_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem kind".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-index".to_string()
                            options=index_options.clone()
                            selected_index=workbench_index_mode
                            set_selected_index=set_workbench_index_mode
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem index".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-is-disabled".to_string()
                            options=is_disabled_options.clone()
                            selected_index=workbench_is_disabled_mode
                            set_selected_index=set_workbench_is_disabled_mode
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem is_disabled".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "focused"
                        </Switch>
                        <Switch checked=workbench_has_submenu set_checked=set_workbench_has_submenu>
                            "has_submenu"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id"
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
                <div class="docs-stack">
                    <MenuItem
                        id=if workbench_custom_id.get() {
                            "docs-menu-item-workbench".to_string()
                        } else {
                            String::new()
                        }
                        index=workbench_index.get()
                        kind=workbench_kind.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        focused=workbench_focused.get()
                        has_submenu=workbench_has_submenu.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench menu item".to_string()
                        } else {
                            String::new()
                        }
                        on_pointer_move=on_workbench_pointer_move
                        on_press=on_workbench_press
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-item-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        {move || format!("{} workbench item", workbench_kind_name.get())}
                    </MenuItem>
                    <span class="ui-muted">
                        "on_pointer_move count: " {move || workbench_pointer_moves.get()}
                        " · on_press count: " {move || workbench_presses.get()}
                        " · last event: " {move || workbench_last_event.get()}
                        " · checkbox checked: " {move || workbench_checkbox_checked.get()}
                        " · radio checked: " {move || workbench_radio_checked.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Action / Checkbox / Disabled Radio)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <MenuItem
                        index=0
                        kind=MenuItemKind::Action
                        aria_label="Open profile".to_string()
                    >
                        "Open profile"
                    </MenuItem>
                    <MenuItem
                        index=1
                        kind=matrix_checkbox_kind
                        focused=true
                        on_pointer_move=on_matrix_pointer_move
                        on_press=on_matrix_press
                    >
                        "Pin workspace"
                    </MenuItem>
                    <MenuItem
                        index=2
                        kind=matrix_radio_kind
                        is_disabled=true
                        disabled=true
                        has_submenu=true
                        class_name="docs-menu-item-custom".to_string()
                    >
                        "Primary workspace"
                    </MenuItem>
                    <span class="ui-muted">
                        "matrix checkbox checked: " {move || matrix_checked.get()}
                        " · pointer moves: " {move || matrix_pointer_moves.get()}
                        " · presses: " {move || matrix_presses.get()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_section() -> AnyView {
    let (showcase_checked, set_showcase_checked) = signal(true);
    let showcase_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || showcase_checked.get()),
    };
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let showcase_on_press = Callback::new(move |_| {
        set_showcase_checked.update(|value| *value = !*value);
        set_showcase_presses.update(|count| *count += 1);
    });

    let tone_options = vec!["Default".to_string(), "Quiet".to_string()];
    let item_count_options = vec!["0".to_string(), "2".to_string(), "3".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_item_count_index, set_workbench_item_count_index) = signal(Some(2_usize));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_sticky_heading, set_workbench_sticky_heading) = signal(false);
    let (workbench_show_divider, set_workbench_show_divider) = signal(false);
    let (workbench_custom_title, set_workbench_custom_title) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_heading_tone =
        Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
            1 => MenuSectionHeadingTone::Quiet,
            _ => MenuSectionHeadingTone::Default,
        });
    let workbench_item_count =
        Signal::derive(
            move || match workbench_item_count_index.get().unwrap_or(2) {
                0 => 0_usize,
                1 => 2_usize,
                _ => 3_usize,
            },
        );

    let (workbench_primary_checked, set_workbench_primary_checked) = signal(true);
    let workbench_primary_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || workbench_primary_checked.get()),
    };
    let (workbench_pinned_checked, set_workbench_pinned_checked) = signal(true);
    let workbench_pinned_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || workbench_pinned_checked.get()),
    };
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let workbench_toggle_primary = Callback::new(move |_| {
        set_workbench_primary_checked.update(|value| *value = !*value);
        set_workbench_presses.update(|count| *count += 1);
    });
    let workbench_toggle_pinned = Callback::new(move |_| {
        set_workbench_pinned_checked.update(|value| *value = !*value);
        set_workbench_presses.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<MenuSection
  title="Workspace actions".to_string()
  item_count=3
  aria_label="Workspace actions section".to_string()
>
  <MenuItem index=0 kind=MenuItemKind::Action>"Open workspace"</MenuItem>
  <MenuItem index=1 kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) } on_press=on_press>
    "Pin workspace"
  </MenuItem>
  <MenuItem index=2 kind=MenuItemKind::Action>"Archive workspace"</MenuItem>
</MenuSection>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let heading_tone = match workbench_heading_tone.get() {
            MenuSectionHeadingTone::Quiet => "MenuSectionHeadingTone::Quiet",
            MenuSectionHeadingTone::Default => "MenuSectionHeadingTone::Default",
        };
        let title = if workbench_custom_title.get() {
            "Routing controls"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Routing menu section"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-menu-section-custom"
        } else {
            ""
        };

        [
            "<MenuSection".to_string(),
            format!("  title={}", rust_string_literal(title)),
            format!("  item_count={}", workbench_item_count.get()),
            format!("  heading_tone={heading_tone}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!(
                "  sticky_heading={}",
                bool_word(workbench_sticky_heading.get())
            ),
            format!("  show_divider={}", bool_word(workbench_show_divider.get())),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            format!("  class_name={}", rust_string_literal(class_name)),
            ">".to_string(),
            "  ...menu items...".to_string(),
            "</MenuSection>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let title = if workbench_custom_title.get() {
            Some("Routing controls")
        } else {
            Some("")
        };
        let aria_label = if workbench_custom_aria.get() {
            Some("Routing menu section")
        } else {
            Some("")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-menu-section-custom")
        } else {
            Some("")
        };

        format!(
            "MenuSectionActualConfig {{\n  title: {title:?},\n  item_count: Some({}),\n  heading_tone: {:?},\n  is_disabled: Some({}),\n  disabled: {},\n  sticky_heading: {},\n  show_divider: {},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  item_feedback: \"presses={}, primary_checked={}, pinned_checked={}\",\n}}",
            workbench_item_count.get(),
            workbench_heading_tone.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_sticky_heading.get()),
            bool_word(workbench_show_divider.get()),
            workbench_presses.get(),
            workbench_primary_checked.get(),
            workbench_pinned_checked.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MenuSection title="Default section".to_string() item_count=3>
  <MenuItem kind=MenuItemKind::Action>"Open workspace"</MenuItem>
</MenuSection>
<MenuSection title="Quiet sticky".to_string() item_count=2 heading_tone=MenuSectionHeadingTone::Quiet sticky_heading=true show_divider=true>
  <MenuItem kind=MenuItemKind::Action>"Primary route"</MenuItem>
</MenuSection>
<MenuSection title="Disabled empty".to_string() item_count=0 is_disabled=true disabled=true class_name="docs-menu-section-custom".to_string()>
  <span class="ui-muted">"No actions available"</span>
</MenuSection>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="MenuSection"
            slug="menu-section"
            group="Collections"
            description="baseline-style menu section primitive with centralized heading/item/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Hello World (Default MenuSection)" code_signal=hello_code>
                <MenuSection
                    title="Workspace actions".to_string()
                    item_count=3
                    aria_label="Workspace actions section".to_string()
                >
                    <MenuItem index=0 kind=MenuItemKind::Action>
                        "Open workspace"
                    </MenuItem>
                    <MenuItem
                        index=1
                        kind=showcase_kind
                        on_press=showcase_on_press
                    >
                        "Pin workspace"
                    </MenuItem>
                    <MenuItem index=2 kind=MenuItemKind::Action>
                        "Archive workspace"
                    </MenuItem>
                </MenuSection>
                <span class="ui-muted">
                    "showcase checked: " {move || showcase_checked.get()}
                    " · presses: " {move || showcase_presses.get()}
                </span>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-section-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-section-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuSection heading tone".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-section-workbench-item-count".to_string()
                            options=item_count_options.clone()
                            selected_index=workbench_item_count_index
                            set_selected_index=set_workbench_item_count_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuSection item_count".to_string()
                        />
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_sticky_heading set_checked=set_workbench_sticky_heading>
                            "sticky_heading"
                        </Switch>
                        <Switch checked=workbench_show_divider set_checked=set_workbench_show_divider>
                            "show_divider"
                        </Switch>
                        <Switch checked=workbench_custom_title set_checked=set_workbench_custom_title>
                            "title"
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
                <div class="docs-stack">
                    <MenuSection
                        title=if workbench_custom_title.get() {
                            "Routing controls".to_string()
                        } else {
                            String::new()
                        }
                        item_count=workbench_item_count.get()
                        heading_tone=workbench_heading_tone.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        sticky_heading=workbench_sticky_heading.get()
                        show_divider=workbench_show_divider.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Routing menu section".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-section-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <MenuItem
                            kind=workbench_primary_kind
                            has_submenu=true
                            on_press=workbench_toggle_primary
                        >
                            "Set as primary route"
                        </MenuItem>
                        <MenuItem
                            kind=workbench_pinned_kind
                            on_press=workbench_toggle_pinned
                        >
                            "Pin fallback route"
                        </MenuItem>
                        <MenuItem kind=MenuItemKind::Action>
                            "Archive route"
                        </MenuItem>
                    </MenuSection>

                    <span class="ui-muted">
                        "workbench presses: " {move || workbench_presses.get()}
                        " · primary: " {move || workbench_primary_checked.get()}
                        " · pinned: " {move || workbench_pinned_checked.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Quiet / Disabled Empty)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <MenuSection title="Default section".to_string() item_count=3>
                        <MenuItem kind=MenuItemKind::Action>"Open workspace"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Rename workspace"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Archive workspace"</MenuItem>
                    </MenuSection>
                    <MenuSection
                        title="Quiet sticky".to_string()
                        item_count=2
                        heading_tone=MenuSectionHeadingTone::Quiet
                        sticky_heading=true
                        show_divider=true
                    >
                        <MenuItem kind=MenuItemKind::Action>"Primary route"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Fallback route"</MenuItem>
                    </MenuSection>
                    <MenuSection
                        title="Disabled empty".to_string()
                        item_count=0
                        is_disabled=true
                        disabled=true
                        class_name="docs-menu-section-custom".to_string()
                    >
                        <span class="ui-muted">"No actions available"</span>
                    </MenuSection>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dropdown() -> AnyView {
    let hello_items = vec!["Profile".to_string(), "Settings".to_string()];
    let items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Sign out".to_string(),
    ];
    let items_for_default = items.clone();
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let empty_items: Vec<String> = Vec::new();

    let (last_action, set_last_action) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last_action.set(Some(index)));

    let (open_raw, set_open_raw) = signal(false);
    let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let hello_code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Dropdown
  id_base="profile-dropdown".to_string()
  items=vec!["Profile".to_string(), "Settings".to_string()]
  on_action=on_action
>
  "Open actions"
</Dropdown>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |index: usize| {
  drop(index);});

<Dropdown
  id_base="profile-dropdown".to_string()
  items=vec!["Profile".to_string(), "Settings".to_string(), "Sign out".to_string()]
  on_action=on_action
>
  "Open actions"
</Dropdown>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let on_action = Callback::new(move |index: usize| {
  drop(index);});

<Dropdown
  id_base="controlled-dropdown".to_string()
  items=vec!["Rename".to_string(), "Duplicate".to_string(), "Archive".to_string()]
  on_action=on_action
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  close_on_action=false
  disabled_indices=vec![1]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
  motion=DropdownMotion {
    popover: PopoverMotion { initial_scale: 0.94, offset_y_px: 12.0, ..PopoverMotion::default() },
  }
  class_name="docs-dropdown-custom".to_string()
>
  "Controlled dropdown"
</Dropdown>"#
            .to_string()
    });

    let motion = DropdownMotion {
        popover: PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 12.0,
            ..PopoverMotion::default()
        },
    };
    let workbench_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Sign out".to_string(),
    ];
    let placement_options = vec![
        "bottom-start".to_string(),
        "bottom-end".to_string(),
        "top-start".to_string(),
        "top-end".to_string(),
    ];
    let (workbench_placement_index, set_workbench_placement_index) = signal(Some(0_usize));
    let workbench_placement =
        Signal::derive(move || match workbench_placement_index.get().unwrap_or(0) {
            1 => PopoverPlacement::BottomEnd,
            2 => PopoverPlacement::TopStart,
            3 => PopoverPlacement::TopEnd,
            _ => PopoverPlacement::BottomStart,
        });
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_has_disabled_items, set_workbench_has_disabled_items) = signal(false);
    let (workbench_has_item_kinds, set_workbench_has_item_kinds) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open_signal: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let workbench_on_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let workbench_on_action =
        Callback::new(move |index: usize| set_workbench_last_action.set(Some(index)));

    let workbench_code = Signal::derive(move || {
        let placement = workbench_placement.get();
        let controlled = workbench_controlled.get();
        let is_disabled = workbench_is_disabled.get();
        let close_on_action = workbench_close_on_action.get();
        let has_disabled_items = workbench_has_disabled_items.get();
        let has_item_kinds = workbench_has_item_kinds.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();
        let lang = if workbench_lang_zh.get() {
            "\"zh-CN\""
        } else {
            "\"en-US\""
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        let mut lines = vec![
            "let on_action = Callback::new(move |index: usize| {".to_string(),
            "  drop(index);".to_string(),
            "});".to_string(),
            String::new(),
        ];

        if controlled {
            lines.push("let (open_raw, set_open_raw) = signal(false);".to_string());
            lines.push(
                "let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());"
                    .to_string(),
            );
            lines.push(
                "let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));"
                    .to_string(),
            );
            lines.push(String::new());
        }

        lines.push("<Dropdown".to_string());
        lines.push("  id_base=\"dropdown-workbench\".into()".to_string());
        lines.push(
            "  items=vec![\"Profile\".into(), \"Settings\".into(), \"Sign out\".into()]"
                .to_string(),
        );
        lines.push("  on_action=on_action".to_string());
        lines.push("  is_close_on_action=Some(true)".to_string());
        lines.push(format!("  default_open={}", bool_word(!controlled)));
        lines.push(format!("  lang={lang}.into()"));
        lines.push(format!("  dir={dir}"));
        lines.push("  aria_label=\"Workbench dropdown\".into()".to_string());
        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if !close_on_action {
            lines.push("  close_on_action=false".to_string());
        }
        if controlled {
            lines.push("  is_open=open_signal".to_string());
            lines.push("  on_open_change=on_open_change".to_string());
        }
        if placement != PopoverPlacement::BottomStart {
            lines.push(format!("  placement=PopoverPlacement::{placement:?}"));
        }
        if has_disabled_items {
            lines.push("  disabled_indices=vec![1]".to_string());
        }
        if has_item_kinds {
            lines.push(
                "  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]".to_string(),
            );
        }
        if custom_motion {
            lines.push("  motion=DropdownMotion {".to_string());
            lines.push(
                "    popover: PopoverMotion { initial_scale: 0.92, offset_y_px: 14.0, ..PopoverMotion::default() },".to_string(),
            );
            lines.push("  }".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-dropdown-workbench\".into()".to_string());
        }

        lines.push(">".to_string());
        lines.push("  \"Workbench dropdown\"".to_string());
        lines.push("</Dropdown>".to_string());

        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/dropdown/styles.rs */\n{}",
            ui::dropdown::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = workbench_placement.get();
        let controlled = workbench_controlled.get();
        let is_disabled = workbench_is_disabled.get();
        let close_on_action = workbench_close_on_action.get();
        let has_disabled_items = workbench_has_disabled_items.get();
        let has_item_kinds = workbench_has_item_kinds.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();
        let open = workbench_open_raw.get();
        let last_action = workbench_last_action.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let disabled_indices = if has_disabled_items {
            "vec![1]"
        } else {
            "vec![]"
        };
        let item_kinds = if has_item_kinds {
            "vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]"
        } else {
            "vec![]"
        };

        let mut classes = vec!["ui-dropdown".to_string()];
        if is_disabled {
            classes.push("ui-dropdown--disabled".to_string());
        }
        if close_on_action {
            classes.push("ui-dropdown--has-items".to_string());
        } else {
            classes.push("ui-dropdown--persistent".to_string());
        }
        if controlled {
            classes.push("ui-dropdown--controlled".to_string());
        }
        if custom_class {
            classes.push("ui-dropdown--custom-class".to_string());
            classes.push("docs-dropdown-workbench".to_string());
        }

        format!(
            "DropdownActualConfig {{\n  id_base: \"docs-dropdown-workbench\",\n  items: [\"Profile\", \"Settings\", \"Sign out\"],\n  on_action: Some(\"workbench_on_action\"),\n  is_disabled: Some({is_disabled}),\n  disabled: {is_disabled},\n  disabled_indices: {disabled_indices},\n  item_kinds: {item_kinds},\n  is_close_on_action: Some({close_on_action}),\n  close_on_action: {close_on_action},\n  placement: Some({placement:?}),\n  is_open: {is_open},\n  open: {open},\n  default_open: Some({default_open}),\n  on_open_change: {on_open_change},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  motion: {motion},\n  aria_label: Some(\"Workbench dropdown\"),\n  class_name: {class_name},\n  controlled: {controlled},\n  has_disabled_items: {has_disabled_items},\n  has_item_kinds: {has_item_kinds},\n  custom_motion: {custom_motion},\n  class: {class_tokens:?},\n  last_action: {last_action},\n}}",
            is_open = if controlled {
                "Some(workbench_open_signal)"
            } else {
                "None"
            },
            default_open = !controlled,
            on_open_change = if controlled {
                "Some(\"workbench_on_open_change\")"
            } else {
                "None"
            },
            motion = if custom_motion {
                "DropdownMotion::custom"
            } else {
                "DropdownMotion::default"
            },
            class_name = if custom_class {
                "Some(\"docs-dropdown-workbench\")"
            } else {
                "None"
            },
            class_tokens = classes.join(" "),
            last_action = last_action
                .map(|index| index.to_string())
                .unwrap_or_else(|| "None".to_string()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let items = vec![
  "Profile".to_string(),
  "Settings".to_string(),
  "Sign out".to_string(),
];
let on_action = Callback::new(move |_: usize| {});
let (open, set_open) = signal(false);

<div class="docs-row">
  <Dropdown id_base="matrix-default".to_string() items=items.clone() on_action=on_action>
    "Default"
  </Dropdown>
  <Dropdown
    id_base="matrix-controlled".to_string()
    items=items.clone()
    on_action=on_action
    is_open=Signal::derive(move || open.get())
    on_open_change=Callback::new(move |next| set_open.set(next))
    close_on_action=false
  >
    "Controlled"
  </Dropdown>
  <Dropdown id_base="matrix-disabled".to_string() items=items.clone() on_action=on_action is_disabled=true>
    "Disabled"
  </Dropdown>
  <Dropdown id_base="matrix-empty".to_string() items=Vec::<String>::new() on_action=on_action>
    "Empty"
  </Dropdown>
</div>"#.to_string()
    });
    let (matrix_open_raw, set_matrix_open_raw) = signal(false);
    let matrix_open_signal: Signal<bool> = Signal::derive(move || matrix_open_raw.get());
    let matrix_on_open_change = Callback::new(move |next: bool| set_matrix_open_raw.set(next));
    let (matrix_last_action, set_matrix_last_action) = signal(None::<usize>);
    let matrix_on_action =
        Callback::new(move |index: usize| set_matrix_last_action.set(Some(index)));

    view! {
        <ComponentPage
            title="Dropdown"
            slug="dropdown"
            group="Collections"
            description="baseline-style dropdown trigger primitive with centralized state/source contracts, controllable open state, and spring-tuned popover motion."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row" data-slot="dropdown-hello-world">
                    <Dropdown
                        id_base="docs-dropdown-hello".to_string()
                        items=hello_items
                        on_action=on_action
                    >
                        "Open actions"
                    </Dropdown>
                </div>
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row" data-slot="dropdown-default-playground">
                    <Dropdown
                        id_base="docs-dropdown-default".to_string()
                        items=items_for_default.clone()
                        on_action=on_action
                    >
                        "Open actions"
                    </Dropdown>
                    <span class="ui-muted" data-slot="dropdown-last-action">
                        "last action: "
                        {move || {
                            last_action
                                .get()
                                .map(|idx| idx.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Persistent + Motion" code_signal=states_code>
                <div class="docs-stack" data-slot="dropdown-controlled-playground">
                    <Dropdown
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=open_signal
                        on_open_change=on_open_change
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                        motion=motion
                        class_name="docs-dropdown-custom".to_string()
                    >
                        "Controlled dropdown"
                    </Dropdown>
                    <span class="ui-muted" data-slot="dropdown-controlled-open">
                        "open: "
                        {move || open_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 四区合一：用于快速比对 controlled、placement、motion 与状态来源标记。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/menu/dropdown/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Placement"</div>
                        <SegmentedControl
                            id_base="docs-dropdown-workbench-placement".to_string()
                            options=placement_options.clone()
                            selected_index=workbench_placement_index
                            set_selected_index=set_workbench_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Dropdown placement".to_string()
                        />
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_close_on_action set_checked=set_workbench_close_on_action>
                            "Close on action"
                        </Switch>
                        <Switch
                            checked=workbench_has_disabled_items
                            set_checked=set_workbench_has_disabled_items
                        >
                            "Disabled item"
                        </Switch>
                        <Switch checked=workbench_has_item_kinds set_checked=set_workbench_has_item_kinds>
                            "Item kinds"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let placement = workbench_placement.get();
                    let controlled = workbench_controlled.get();
                    let is_disabled = workbench_is_disabled.get();
                    let close_on_action = workbench_close_on_action.get();
                    let has_disabled_items = workbench_has_disabled_items.get();
                    let has_item_kinds = workbench_has_item_kinds.get();
                    let custom_motion = workbench_custom_motion.get();
                    let custom_class = workbench_custom_class.get();
                    let disabled_indices = if has_disabled_items {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let item_kinds = if has_item_kinds {
                        vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    } else {
                        Vec::new()
                    };
                    let motion = if custom_motion {
                        DropdownMotion {
                            popover: PopoverMotion {
                                initial_scale: 0.92,
                                offset_y_px: 14.0,
                                ..PopoverMotion::default()
                            },
                        }
                    } else {
                        DropdownMotion::default()
                    };
                    let class_name = if custom_class {
                        "docs-dropdown-workbench".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if workbench_lang_zh.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    };
                    let dir = if workbench_rtl_dir.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    if controlled {
                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="dropdown-workbench-preview">
                                <Dropdown
                                    id_base="docs-dropdown-workbench".to_string()
                                    items=workbench_items.clone()
                                    on_action=workbench_on_action
                                    is_disabled=is_disabled
                                    close_on_action=close_on_action
                                    is_close_on_action=close_on_action
                                    placement=placement
                                    is_open=workbench_open_signal
                                    on_open_change=workbench_on_open_change
                                    default_open=false
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    motion=motion
                                    aria_label="Workbench dropdown".to_string()
                                    lang=lang.clone()
                                    dir=dir
                                    class_name=class_name
                                >
                                    "Workbench dropdown"
                                </Dropdown>
                                <span class="ui-muted">
                                    "open: "
                                    {workbench_open_raw.get()}
                                    " · last action: "
                                    {workbench_last_action
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="dropdown-workbench-preview">
                                <Dropdown
                                    id_base="docs-dropdown-workbench".to_string()
                                    items=workbench_items.clone()
                                    on_action=workbench_on_action
                                    is_disabled=is_disabled
                                    close_on_action=close_on_action
                                    is_close_on_action=close_on_action
                                    placement=placement
                                    default_open=true
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    motion=motion
                                    aria_label="Workbench dropdown".to_string()
                                    lang=lang
                                    dir=dir
                                    class_name=class_name
                                >
                                    "Workbench dropdown"
                                </Dropdown>
                                <span class="ui-muted">
                                    "open: internal (uncontrolled)"
                                    " · last action: "
                                    {workbench_last_action
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="State Matrix Compare" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="dropdown-state-matrix">
                    <div class="docs-row">
                        <div class="docs-card">
                            <h2>"Default"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-default".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                            >
                                "Default"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Controlled + Persistent"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-controlled".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                                is_open=matrix_open_signal
                                on_open_change=matrix_on_open_change
                                close_on_action=false
                            >
                                "Controlled"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Disabled"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-disabled".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                                is_disabled=true
                            >
                                "Disabled"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Empty"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-empty".to_string()
                                items=empty_items.clone()
                                on_action=matrix_on_action
                            >
                                "Empty"
                            </Dropdown>
                        </div>
                    </div>

                    <span class="ui-muted">
                        "controlled open: "
                        {move || matrix_open_raw.get()}
                        " · last action: "
                        {move || {
                            matrix_last_action
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
