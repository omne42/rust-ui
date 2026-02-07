use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{
    Table, TableCellAlign, TableColumn, TableDensity, TableLayout, TableRow, TableVariant, Tree,
    TreeDensity, TreeNode, TreeTone,
};

pub(super) fn table() -> AnyView {
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

    let empty_rows: Vec<TableRow> = Vec::new();

    let columns_primary = columns.clone();
    let columns_secondary = columns;
    let rows_primary = rows.clone();

    let code = r#"let columns = vec![
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
  striped=true
/>"#;

    let states_code = r#"<Table
  columns=columns
  rows=Vec::<TableRow>::new()
  variant=TableVariant::Outline
  density=TableDensity::Compact
  layout=TableLayout::Fixed
  sticky_header=true
  empty_label="No active incidents".to_string()
  class_name="docs-table-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Table"
            slug="table"
            group="Collections"
            description="Data table primitive with centralized row/column normalization and Spectrum-style state markers for density/layout/variant contracts."
        >
            <Playground title="Default + Striped" code=code>
                <Table
                    columns=columns_primary
                    rows=rows_primary
                    caption="Service health".to_string()
                    striped=true
                />
            </Playground>

            <Playground title="Compact + Fixed + Empty" code=states_code>
                <Table
                    columns=columns_secondary
                    rows=empty_rows
                    variant=TableVariant::Outline
                    density=TableDensity::Compact
                    layout=TableLayout::Fixed
                    sticky_header=true
                    empty_label="No active incidents".to_string()
                    class_name="docs-table-custom".to_string()
                />
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

    let nodes_primary = nodes.clone();
    let nodes_secondary = nodes;

    let code = r#"let nodes = vec![
  TreeNode::new("root-app", "Applications").with_children(vec![
    TreeNode::new("app-web", "Web Console"),
    TreeNode::new("app-mobile", "Mobile App"),
  ]),
  TreeNode::new("root-services", "Services").with_children(vec![
    TreeNode::new("svc-api", "API Gateway"),
    TreeNode::new("svc-worker", "Worker Pool"),
  ]),
];

<Tree
  id_base="services-tree".to_string()
  nodes=nodes
  default_expanded_ids=BTreeSet::from(["root-app".to_string()])
  default_selected_id="app-web".to_string()
/>"#;

    let states_code = r#"<Tree
  id_base="inventory-tree".to_string()
  nodes=nodes
  tone=TreeTone::Strong
  density=TreeDensity::Compact
  default_expanded_ids=BTreeSet::from(["root-services".to_string()])
  default_selected_id="svc-api".to_string()
  class_name="docs-tree-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Tree"
            slug="tree"
            group="Collections"
            description="Hierarchical tree with controllable expand/selection state and Spectrum-style density/tone/state marker contracts."
        >
            <Playground title="Default + Expanded Root" code=code>
                <Tree
                    id_base="docs-tree-default".to_string()
                    nodes=nodes_primary
                    default_expanded_ids=BTreeSet::from(["root-app".to_string()])
                    default_selected_id="app-web".to_string()
                />
            </Playground>

            <Playground title="Strong + Compact" code=states_code>
                <Tree
                    id_base="docs-tree-strong".to_string()
                    nodes=nodes_secondary
                    tone=TreeTone::Strong
                    density=TreeDensity::Compact
                    default_expanded_ids=BTreeSet::from(["root-services".to_string()])
                    default_selected_id="svc-api".to_string()
                    class_name="docs-tree-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
