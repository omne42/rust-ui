use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{
    DisclosureGroup, DisclosureGroupSelectionMode, Dropdown, DropdownMotion, ListItem, ListSection,
    ListSectionHeadingTone, MenuItem, MenuItemKind, MenuSection, MenuSectionHeadingTone,
    PopoverMotion, StepList, StepListItem, StepListOrientation, StepListSize, Table,
    TableCellAlign, TableColumn, TableDensity, TableLayout, TableRow, TableVariant, Tree,
    TreeDensity, TreeNode, TreeTone, open_set,
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

    let code = Signal::derive(move || {
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
  striped=true
/>"#.to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let columns = vec![
  TableColumn::new("service", "Service"),
  TableColumn::new("region", "Region"),
  TableColumn::new("uptime", "Uptime").with_align(TableCellAlign::End),
];

<Table
  columns=columns
  rows=Vec::<TableRow>::new()
  variant=TableVariant::Outline
  density=TableDensity::Compact
  layout=TableLayout::Fixed
  sticky_header=true
  empty_label="No active incidents".to_string()
  class_name="docs-table-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Table"
            slug="table"
            group="Collections"
            description="Data table primitive with centralized row/column normalization and baseline-style state markers for density/layout/variant contracts."
        >
            <Playground title="Default + Striped" code_signal=code>
                <Table
                    columns=columns_primary
                    rows=rows_primary
                    caption="Service health".to_string()
                    striped=true
                />
            </Playground>

            <Playground title="Compact + Fixed + Empty" code_signal=states_code>
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
    let on_selected_change = Callback::new(move |next: Option<usize>| set_selected_index.set(next));

    let code = Signal::derive(move || {
        r#"let (selected_index, set_selected_index) = signal(Some(1_usize));
let on_selected_change = Callback::new(move |next: Option<usize>| set_selected_index.set(next));

<StepList
  steps=signal(vec![
    StepListItem::new("account", "Account").described("Create account and verify email"),
    StepListItem::new("shipping", "Shipping").described("Choose shipping address"),
    StepListItem::new("payment", "Payment").described("Add payment method"),
    StepListItem::new("review", "Review").described("Confirm and place order"),
  ]).0
  selected_index=selected_index.into()
  on_selected_change=on_selected_change
  completed_indices=vec![0]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<StepList
  steps=signal(vec![
    StepListItem::new("plan", "Plan").described("Pick your subscription tier"),
    StepListItem::new("profile", "Profile").described("Fill organization details"),
    StepListItem::new("billing", "Billing").described("Billing is locked").disabled(true),
    StepListItem::new("launch", "Launch").described("Start using the workspace"),
  ]).0
  orientation=StepListOrientation::Vertical
  size=StepListSize::L
  emphasized=true
  completed_indices=vec![0, 1]
  default_selected_index=3
  class_name="docs-step-list-custom".to_string()
  aria_label="Workspace setup steps".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="StepList"
            slug="step-list"
            group="Collections"
            description="baseline-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts."
        >
            <Playground title="Controlled Selection" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <StepList
                        steps=signal(steps).0
                        selected_index=selected_index.into()
                        on_selected_change=on_selected_change
                        completed_indices=vec![0]
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || selected_index.get().map_or("none".to_string(), |index| index.to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Vertical + Emphasized + Disabled" code_signal=states_code>
                <StepList
                    steps=signal(steps_with_disabled).0
                    orientation=StepListOrientation::Vertical
                    size=StepListSize::L
                    emphasized=true
                    completed_indices=vec![0, 1]
                    default_selected_index=3
                    class_name="docs-step-list-custom".to_string()
                    aria_label="Workspace setup steps".to_string()
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

    let code = Signal::derive(move || {
        r#"let nodes = vec![
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
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let nodes = vec![
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
  id_base="inventory-tree".to_string()
  nodes=nodes
  tone=TreeTone::Strong
  density=TreeDensity::Compact
  default_expanded_ids=BTreeSet::from(["root-services".to_string()])
  default_selected_id="svc-api".to_string()
  class_name="docs-tree-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Tree"
            slug="tree"
            group="Collections"
            description="Hierarchical tree with controllable expand/selection state and baseline-style density/tone/state marker contracts."
        >
            <Playground title="Default + Expanded Root" code_signal=code>
                <Tree
                    id_base="docs-tree-default".to_string()
                    nodes=nodes_primary
                    default_expanded_ids=BTreeSet::from(["root-app".to_string()])
                    default_selected_id="app-web".to_string()
                />
            </Playground>

            <Playground title="Strong + Compact" code_signal=states_code>
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

    let (expanded_multi, set_expanded_multi) = signal(open_set([0]));
    let on_multi_change = Callback::new(move |next: BTreeSet<usize>| set_expanded_multi.set(next));

    let (expanded_single, set_expanded_single) = signal(open_set([1]));
    let on_single_change =
        Callback::new(move |next: BTreeSet<usize>| set_expanded_single.set(next));

    let code = Signal::derive(move || {
        r#"let (expanded, set_expanded) = signal(open_set([0]));
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
</DisclosureGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (expanded, set_expanded) = signal(open_set([1]));
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

            <Playground title="Single + Disabled Item + Custom Class" code_signal=states_code>
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

pub(super) fn list_item() -> AnyView {
    let (selected_default, set_selected_default) = signal(true);
    let toggle_default = Callback::new(move |_| {
        set_selected_default.update(|value| *value = !*value);
    });

    let (selected_states, set_selected_states) = signal(true);
    let toggle_states = Callback::new(move |_| {
        set_selected_states.update(|value| *value = !*value);
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(true);

<ListItem
  index=0
  selected=selected.get()
  show_selection_indicator=true
  on_press=Callback::new(move |_| set_selected.update(|value| *value = !*value))
>
  "San Francisco"
</ListItem>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(true);

<ListItem
  id="docs-listbox-item-focused".to_string()
  index=1
  selected=selected.get()
  focused=true
  has_divider=true
  show_selection_indicator=true
  class_name="docs-listbox-item-custom".to_string()
  on_press=Callback::new(move |_| set_selected.update(|value| *value = !*value))
>
  "Tokyo"
</ListItem>

<ListItem index=2 disabled=true>
  "Disabled option"
</ListItem>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ListItem"
            slug="list-item"
            group="Collections"
            description="baseline-style list option primitive with centralized selection/focus/divider/source normalization and stable `slot` + `data-*` state contracts."
        >
            <Playground title="Selectable Option" code_signal=code>
                <div class="docs-stack">
                    <ListItem
                        index=0
                        selected=selected_default.get()
                        show_selection_indicator=true
                        on_press=toggle_default
                    >
                        "San Francisco"
                    </ListItem>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected_default.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Focused + Divider + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <ListItem
                        id="docs-listbox-item-focused".to_string()
                        index=1
                        selected=selected_states.get()
                        focused=true
                        has_divider=true
                        show_selection_indicator=true
                        class_name="docs-listbox-item-custom".to_string()
                        on_press=toggle_states
                    >
                        "Tokyo"
                    </ListItem>

                    <ListItem index=2 disabled=true>
                        "Disabled option"
                    </ListItem>

                    <span class="ui-muted">
                        "focused item selected: "
                        {move || selected_states.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list_section() -> AnyView {
    let (selected_primary, set_selected_primary) = signal(true);
    let toggle_primary = Callback::new(move |_| {
        set_selected_primary.update(|value| *value = !*value);
    });

    let (selected_secondary, set_selected_secondary) = signal(true);
    let toggle_secondary = Callback::new(move |_| {
        set_selected_secondary.update(|value| *value = !*value);
    });

    let code = Signal::derive(move || {
        r#"<ListSection
  title="Preferred regions".to_string()
  item_count=3
  aria_label="Preferred regions section".to_string()
>
  <ListItem index=0 selected=true show_selection_indicator=true>"US East"</ListItem>
  <ListItem index=1>"EU West"</ListItem>
  <ListItem index=2>"AP South"</ListItem>
</ListSection>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(true);

<ListSection
  title="Advanced targets".to_string()
  heading_tone=ListSectionHeadingTone::Quiet
  item_count=2
  sticky_heading=true
  show_divider=true
  class_name="docs-listbox-section-custom".to_string()
>
  <ListItem
    index=0
    selected=selected.get()
    focused=true
    show_selection_indicator=true
    on_press=Callback::new(move |_| set_selected.update(|value| *value = !*value))
  >
    "Primary target"
  </ListItem>
  <ListItem index=1 disabled=true>"Disabled fallback"</ListItem>
</ListSection>

<ListSection title="Empty section".to_string() item_count=0 disabled=true>
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
            <Playground title="Default Section" code_signal=code>
                <ListSection
                    title="Preferred regions".to_string()
                    item_count=3
                    aria_label="Preferred regions section".to_string()
                >
                    <ListItem index=0 selected=true show_selection_indicator=true>
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

            <Playground title="Quiet + Sticky + Divider + Empty" code_signal=states_code>
                <div class="docs-stack">
                    <ListSection
                        title="Advanced targets".to_string()
                        heading_tone=ListSectionHeadingTone::Quiet
                        item_count=2
                        sticky_heading=true
                        show_divider=true
                        class_name="docs-listbox-section-custom".to_string()
                    >
                        <ListItem
                            index=0
                            selected=selected_primary.get()
                            focused=true
                            show_selection_indicator=true
                            on_press=toggle_primary
                        >
                            "Primary target"
                        </ListItem>
                        <ListItem
                            index=1
                            selected=selected_secondary.get()
                            has_divider=true
                            show_selection_indicator=true
                            on_press=toggle_secondary
                        >
                            "Secondary target"
                        </ListItem>
                    </ListSection>

                    <ListSection
                        title="Empty section".to_string()
                        item_count=0
                        disabled=true
                    >
                        <span class="ui-muted">"No options available"</span>
                    </ListSection>

                    <span class="ui-muted">
                        "primary selected: "
                        {move || selected_primary.get().to_string()}
                        " · secondary selected: "
                        {move || selected_secondary.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_item() -> AnyView {
    let (checkbox_checked, set_checkbox_checked) = signal(true);
    let checkbox_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || checkbox_checked.get()),
    };

    let (radio_selected, set_radio_selected) = signal(true);
    let radio_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || radio_selected.get()),
    };

    let toggle_checkbox = Callback::new(move |_| {
        set_checkbox_checked.update(|value| *value = !*value);
    });

    let toggle_radio = Callback::new(move |_| {
        set_radio_selected.update(|value| *value = !*value);
    });

    let code = Signal::derive(move || {
        r#"<MenuItem
  index=0
  kind=MenuItemKind::Action
  aria_label="Open profile".to_string()
>
  "Open profile"
</MenuItem>

let (checked, set_checked) = signal(true);
let checkbox_kind = MenuItemKind::Checkbox {
  is_checked: Signal::derive(move || checked.get()),
};

<MenuItem
  index=1
  kind=checkbox_kind
  on_press=Callback::new(move |_| set_checked.update(|value| *value = !*value))
>
  "Pin to favorites"
</MenuItem>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (is_primary, set_is_primary) = signal(true);
let radio_kind = MenuItemKind::Radio {
  is_checked: Signal::derive(move || is_primary.get()),
};

<MenuItem
  id="docs-menu-item-radio".to_string()
  index=2
  kind=radio_kind
  focused=true
  has_submenu=true
  on_press=Callback::new(move |_| set_is_primary.update(|value| *value = !*value))
  class_name="docs-menu-item-custom".to_string()
>
  "Set as primary workspace"
</MenuItem>

<MenuItem index=3 disabled=true>
  "Disabled destructive action"
</MenuItem>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="MenuItem"
            slug="menu-item"
            group="Collections"
            description="baseline-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Action + Checkbox" code_signal=code>
                <div class="docs-stack">
                    <MenuItem
                        index=0
                        kind=MenuItemKind::Action
                        aria_label="Open profile".to_string()
                    >
                        "Open profile"
                    </MenuItem>

                    <MenuItem
                        index=1
                        kind=checkbox_kind
                        on_press=toggle_checkbox
                    >
                        "Pin to favorites"
                    </MenuItem>

                    <span class="ui-muted">
                        "checkbox checked: "
                        {move || checkbox_checked.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Radio + Submenu + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <MenuItem
                        id="docs-menu-item-radio".to_string()
                        index=2
                        kind=radio_kind
                        focused=true
                        has_submenu=true
                        on_press=toggle_radio
                        class_name="docs-menu-item-custom".to_string()
                    >
                        "Set as primary workspace"
                    </MenuItem>

                    <MenuItem index=3 disabled=true>
                        "Disabled destructive action"
                    </MenuItem>

                    <span class="ui-muted">
                        "radio selected: "
                        {move || radio_selected.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_section() -> AnyView {
    let (pinned_checked, set_pinned_checked) = signal(true);
    let pinned_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || pinned_checked.get()),
    };

    let (primary_checked, set_primary_checked) = signal(true);
    let primary_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || primary_checked.get()),
    };

    let toggle_pinned = Callback::new(move |_| set_pinned_checked.update(|value| *value = !*value));
    let toggle_primary =
        Callback::new(move |_| set_primary_checked.update(|value| *value = !*value));

    let code = Signal::derive(move || {
        r#"<MenuSection
  title="Workspace actions".to_string()
  item_count=3
  aria_label="Workspace actions section".to_string()
>
  <MenuItem index=0 kind=MenuItemKind::Action>"Open workspace"</MenuItem>
  <MenuItem index=1 kind=MenuItemKind::Action>"Rename workspace"</MenuItem>
  <MenuItem index=2 kind=MenuItemKind::Action>"Archive workspace"</MenuItem>
</MenuSection>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(true);
let radio_kind = MenuItemKind::Radio {
  is_checked: Signal::derive(move || checked.get()),
};

<MenuSection
  title="Advanced routing".to_string()
  heading_tone=MenuSectionHeadingTone::Quiet
  item_count=2
  sticky_heading=true
  show_divider=true
  class_name="docs-menu-section-custom".to_string()
>
  <MenuItem kind=radio_kind has_submenu=true on_press=Callback::new(move |_| set_checked.update(|value| *value = !*value))>
    "Set as primary route"
  </MenuItem>
  <MenuItem disabled=true>"Disabled fallback action"</MenuItem>
</MenuSection>

<MenuSection title="Empty state".to_string() item_count=0 disabled=true>
  <span class="ui-muted">"No actions available"</span>
</MenuSection>"#.to_string()
    });

    view! {
        <ComponentPage
            title="MenuSection"
            slug="menu-section"
            group="Collections"
            description="baseline-style menu section primitive with centralized heading/item/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Default Section" code_signal=code>
                <MenuSection
                    title="Workspace actions".to_string()
                    item_count=3
                    aria_label="Workspace actions section".to_string()
                >
                    <MenuItem index=0 kind=MenuItemKind::Action>
                        "Open workspace"
                    </MenuItem>
                    <MenuItem index=1 kind=MenuItemKind::Action>
                        "Rename workspace"
                    </MenuItem>
                    <MenuItem index=2 kind=MenuItemKind::Action>
                        "Archive workspace"
                    </MenuItem>
                </MenuSection>
            </Playground>

            <Playground title="Quiet + Sticky + Divider + Empty" code_signal=states_code>
                <div class="docs-stack">
                    <MenuSection
                        title="Advanced routing".to_string()
                        heading_tone=MenuSectionHeadingTone::Quiet
                        item_count=2
                        sticky_heading=true
                        show_divider=true
                        class_name="docs-menu-section-custom".to_string()
                    >
                        <MenuItem
                            kind=primary_kind
                            has_submenu=true
                            on_press=toggle_primary
                        >
                            "Set as primary route"
                        </MenuItem>
                        <MenuItem
                            kind=pinned_kind
                            on_press=toggle_pinned
                        >
                            "Pin fallback route"
                        </MenuItem>
                    </MenuSection>

                    <MenuSection
                        title="Empty state".to_string()
                        item_count=0
                        disabled=true
                    >
                        <span class="ui-muted">"No actions available"</span>
                    </MenuSection>

                    <span class="ui-muted">
                        "primary selected: "
                        {move || primary_checked.get().to_string()}
                        " · pinned: "
                        {move || pinned_checked.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dropdown() -> AnyView {
    let items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Sign out".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];

    let (last_action, set_last_action) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last_action.set(Some(index)));

    let (open_raw, set_open_raw) = signal(false);
    let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |index: usize| {
  let _ = index;
});

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
  let _ = index;
});

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

    view! {
        <ComponentPage
            title="Dropdown"
            slug="dropdown"
            group="Collections"
            description="baseline-style dropdown trigger primitive with centralized state/source contracts, controllable open state, and spring-tuned popover motion."
        >
            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <Dropdown id_base="docs-dropdown-default".to_string() items=items on_action=on_action>
                        "Open actions"
                    </Dropdown>
                    <span class="ui-muted">
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
                <div class="docs-stack">
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
                    <span class="ui-muted">"open: " {move || open_raw.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
