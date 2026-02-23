use super::*;

pub(crate) fn tree() -> AnyView {
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
