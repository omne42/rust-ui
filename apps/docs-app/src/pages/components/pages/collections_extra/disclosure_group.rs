use super::*;

pub(crate) fn disclosure_group() -> AnyView {
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
        </ComponentPage>
    }
    .into_any()
}
