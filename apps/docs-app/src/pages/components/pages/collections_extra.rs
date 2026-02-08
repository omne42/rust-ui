use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{
    DisclosureGroup, DisclosureGroupSelectionMode, Table, TableCellAlign, TableColumn,
    TableDensity, TableLayout, TableRow, TableVariant, Tree, TreeDensity, TreeNode, TreeTone,
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

    let (expanded_multi, set_expanded_multi) = signal(BTreeSet::from([0_usize]));
    let on_multi_change = Callback::new(move |next: BTreeSet<usize>| set_expanded_multi.set(next));

    let (expanded_single, set_expanded_single) = signal(BTreeSet::from([1_usize]));
    let on_single_change =
        Callback::new(move |next: BTreeSet<usize>| set_expanded_single.set(next));

    let code = r#"let (expanded, set_expanded) = signal(BTreeSet::from([0_usize]));
let on_expanded_change = Callback::new(move |next: BTreeSet<usize>| set_expanded.set(next));

<DisclosureGroup
  labels=vec![
    "Account security".to_string(),
    "Billing preferences".to_string(),
    "Incident escalation".to_string(),
  ]
  id_base="ops-disclosure-group".to_string()
  expanded_indices=expanded.into()
  on_expanded_change=on_expanded_change
  selection_mode=DisclosureGroupSelectionMode::Multiple
>
  <div>"Security policy details"</div>
  <div>"Billing ownership details"</div>
  <div>"Escalation chain details"</div>
</DisclosureGroup>"#;

    let states_code = r#"let (expanded, set_expanded) = signal(BTreeSet::from([1_usize]));
let on_expanded_change = Callback::new(move |next: BTreeSet<usize>| set_expanded.set(next));

<DisclosureGroup
  labels=vec![
    "Region routing".to_string(),
    "Failover strategy".to_string(),
    "Legacy endpoints".to_string(),
  ]
  id_base="ops-disclosure-group-single".to_string()
  expanded_indices=expanded.into()
  on_expanded_change=on_expanded_change
  selection_mode=DisclosureGroupSelectionMode::Single
  disabled_indices=vec![2]
  class_name="docs-disclosure-group-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="DisclosureGroup"
            slug="disclosure-group"
            group="Collections"
            description="Spectrum/HeroUI-style disclosure grouping primitive with centralized expanded-state normalization, controlled/uncontrolled contracts, and spring motion delegated through Accordion internals."
        >
            <Playground title="Multiple + Controlled" code=code>
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=labels
                        id_base="docs-disclosure-group-multiple".to_string()
                        expanded_indices=expanded_multi.into()
                        on_expanded_change=on_multi_change
                        selection_mode=DisclosureGroupSelectionMode::Multiple
                        aria_label="Operational disclosure sections".to_string()
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

            <Playground title="Single + Disabled Item + Custom Class" code=states_code>
                <div class="docs-stack">
                    <DisclosureGroup
                        labels=single_labels
                        id_base="docs-disclosure-group-single".to_string()
                        expanded_indices=expanded_single.into()
                        on_expanded_change=on_single_change
                        selection_mode=DisclosureGroupSelectionMode::Single
                        disabled_indices=vec![2]
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
        </ComponentPage>
    }
    .into_any()
}
