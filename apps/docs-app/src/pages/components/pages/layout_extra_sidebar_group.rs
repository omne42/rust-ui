use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    SegmentedControl, SegmentedControlSize, Sidebar, SidebarCollapsible, SidebarGroup, SidebarMenu,
    SidebarMenuItem, SidebarMenuSubItem, SidebarSide, SidebarVariant, Switch,
};

pub(super) fn sidebar_group() -> AnyView {
    let group_items = vec![
        SidebarMenuItem {
            id: "support".to_string(),
            label: "Support".to_string(),
            href: Some("/support".to_string()),
            badge: Some("2".to_string()),
            action_label: Some("Support item action".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "feedback".to_string(),
            label: "Feedback".to_string(),
            href: Some("/feedback".to_string()),
            badge: Some("1".to_string()),
            action_label: Some("Feedback item action".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
    ];
    let showcase_items = group_items.clone();
    let workbench_items = group_items.clone();

    let collapsible_items = vec![SidebarMenuItem {
        id: "project".to_string(),
        label: "Project docs".to_string(),
        href: None,
        badge: None,
        action_label: Some("Project item action".to_string()),
        disabled: false,
        sub_items: vec![
            SidebarMenuSubItem {
                id: "install".to_string(),
                label: "Installation".to_string(),
                href: Some("/docs/install".to_string()),
                disabled: false,
            },
            SidebarMenuSubItem {
                id: "routing".to_string(),
                label: "Routing".to_string(),
                href: Some("/docs/routing".to_string()),
                disabled: false,
            },
        ],
        default_sub_open: true,
    }];
    let matrix_default_items = collapsible_items.clone();
    let matrix_collapsible_items = collapsible_items.clone();
    let matrix_disabled_items = collapsible_items.clone();

    let (workbench_open_raw, set_workbench_open_raw) = signal(true);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_last_open, set_workbench_last_open) = signal(true);
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_last_open.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let (workbench_action_count, set_workbench_action_count) = signal(0_u32);
    let on_workbench_action = Callback::new(move |_| {
        set_workbench_action_count.update(|count| *count += 1);
    });

    let label_options = vec!["Help".to_string(), "Resources".to_string()];
    let action_label_options = vec!["Add".to_string(), "Create".to_string()];
    let (workbench_label_index, set_workbench_label_index) = signal(Some(0_usize));
    let (workbench_action_label_index, set_workbench_action_label_index) = signal(Some(0_usize));
    let workbench_label = Signal::derive(move || {
        if workbench_label_index.get().unwrap_or(0) == 1 {
            "Resources".to_string()
        } else {
            "Help".to_string()
        }
    });
    let workbench_action_label = Signal::derive(move || {
        if workbench_action_label_index.get().unwrap_or(0) == 1 {
            "Create".to_string()
        } else {
            "Add".to_string()
        }
    });
    let (workbench_default_open, set_workbench_default_open) = signal(true);
    let (workbench_collapsible, set_workbench_collapsible) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_label, set_workbench_show_label) = signal(true);
    let (workbench_show_action, set_workbench_show_action) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let showcase_code = Signal::derive(move || {
        r#"let items = vec![SidebarMenuItem {
  id: "support".to_string(),
  label: "Support".to_string(),
  href: Some("/support".to_string()),
  badge: Some("2".to_string()),
  action_label: Some("Support item action".to_string()),
  disabled: false,
  sub_items: vec![],
  default_sub_open: false,
}];

<SidebarGroup
  label="Help".to_string()
  action_label="Add".to_string()
  on_action=Callback::new(move |_| {})
>
  <SidebarMenu items=items />
</SidebarGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "let (open_raw, set_open_raw) = signal({});\nlet open = Signal::derive(move || open_raw.get());\nlet on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));\nlet on_action = Callback::new(move |_| {{}});\n\n<SidebarGroup\n  open=open\n  default_open={}\n  on_open_change=on_open_change\n  on_action=on_action\n  collapsible={}\n  disabled={}\n  show_label={}\n  show_action={}\n  label={:?}.to_string()\n  action_label={:?}.to_string()\n  aria_label=\"Sidebar group controls\".to_string()\n  class_name={}\n/>",
            workbench_open_raw.get(),
            workbench_default_open.get(),
            workbench_collapsible.get(),
            workbench_disabled.get(),
            workbench_show_label.get(),
            workbench_show_action.get(),
            workbench_label.get(),
            workbench_action_label.get(),
            if workbench_custom_class.get() {
                "\"docs-sidebar-group-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarGroupWorkbenchActualConfig {{\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: \"count={}, last={}\",\n  on_action: \"count={}\",\n  collapsible: {},\n  disabled: {},\n  show_label: {},\n  show_action: {},\n  label: Some({:?}),\n  action_label: Some({:?}),\n  aria_label: Some(\"Sidebar group controls\"),\n  class_name: {:?},\n}}",
            workbench_open_raw.get(),
            workbench_default_open.get(),
            workbench_open_change_count.get(),
            workbench_last_open.get(),
            workbench_action_count.get(),
            workbench_collapsible.get(),
            workbench_disabled.get(),
            workbench_show_label.get(),
            workbench_show_action.get(),
            workbench_label.get(),
            workbench_action_label.get(),
            if workbench_custom_class.get() {
                Some("docs-sidebar-group-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarGroup label="Default".to_string() action_label="Add".to_string() />
<SidebarGroup collapsible=true show_action=false label="Collapsible".to_string() />
<SidebarGroup disabled=true show_label=false show_action=true aria_label="Disabled group".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarGroup"
            slug="sidebar-group"
            group="Layout"
            description="baseline-compatible sidebar group primitive with label/action header regions, controlled/uncontrolled collapsible state, baseline-style data contracts, and motion-ready collapse behavior."
        >
            <Playground title="Hello World (Default SidebarGroup)" code_signal=showcase_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar group playground".to_string()
                >
                    <SidebarGroup
                        label="Help".to_string()
                        action_label="Add".to_string()
                        on_action=Callback::new(move |_| {})
                        collapsible=false
                        aria_label="Help group".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-basic".to_string()
                            items=showcase_items
                            show_actions=false
                            aria_label="Help menu".to_string()
                        />
                    </SidebarGroup>
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-sidebar-group-workbench-label".to_string()
                            options=label_options.clone()
                            selected_index=workbench_label_index
                            set_selected_index=set_workbench_label_index
                            size=SegmentedControlSize::Sm
                            aria_label="SidebarGroup label".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sidebar-group-workbench-action-label".to_string()
                            options=action_label_options.clone()
                            selected_index=workbench_action_label_index
                            set_selected_index=set_workbench_action_label_index
                            size=SegmentedControlSize::Sm
                            aria_label="SidebarGroup action label".to_string()
                        />
                        <Switch checked=workbench_default_open set_checked=set_workbench_default_open>
                            "default_open"
                        </Switch>
                        <Switch checked=workbench_collapsible set_checked=set_workbench_collapsible>
                            "collapsible"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_show_label set_checked=set_workbench_show_label>
                            "show_label"
                        </Switch>
                        <Switch checked=workbench_show_action set_checked=set_workbench_show_action>
                            "show_action"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_open_raw.update(|open| *open = !*open)
                        >
                            "Toggle open"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Controlled group sidebar".to_string()
                    >
                        <SidebarGroup
                            open=workbench_open
                            default_open=workbench_default_open.get()
                            on_open_change=on_workbench_open_change
                            on_action=on_workbench_action
                            collapsible=workbench_collapsible.get()
                            disabled=workbench_disabled.get()
                            show_label=workbench_show_label.get()
                            show_action=workbench_show_action.get()
                            label=workbench_label.get()
                            action_label=workbench_action_label.get()
                            aria_label="Sidebar group controls".to_string()
                            class_name=if workbench_custom_class.get() {
                                "docs-sidebar-group-workbench".to_string()
                            } else {
                                String::new()
                            }
                        >
                            <SidebarMenu
                                id_base="docs-sidebar-group-collapsible".to_string()
                                items=workbench_items.clone()
                                allow_submenu_collapse=true
                                show_actions=true
                                show_badges=false
                                aria_label="Architecture menu".to_string()
                            />
                        </SidebarGroup>

                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">
                                "group open: " {move || workbench_open_raw.get()}
                                " · on_open_change count: " {move || workbench_open_change_count.get()}
                                " · on_action count: " {move || workbench_action_count.get()}
                            </span>
                        </div>
                    </Sidebar>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Collapsible / Disabled)" code_signal=matrix_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar group matrix".to_string()
                >
                    <SidebarGroup
                        label="Default".to_string()
                        action_label="Add".to_string()
                        aria_label="Default group".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-matrix-default".to_string()
                            items=matrix_default_items
                            aria_label="Default matrix menu".to_string()
                        />
                    </SidebarGroup>
                    <SidebarGroup
                        collapsible=true
                        show_action=false
                        label="Collapsible".to_string()
                        aria_label="Collapsible group".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-matrix-collapsible".to_string()
                            items=matrix_collapsible_items
                            aria_label="Collapsible matrix menu".to_string()
                        />
                    </SidebarGroup>
                    <SidebarGroup
                        disabled=true
                        show_label=false
                        show_action=true
                        action_label="Disabled action".to_string()
                        aria_label="Disabled group".to_string()
                        class_name="docs-sidebar-group-disabled".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-matrix-disabled".to_string()
                            items=matrix_disabled_items
                            aria_label="Disabled matrix menu".to_string()
                        />
                    </SidebarGroup>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
