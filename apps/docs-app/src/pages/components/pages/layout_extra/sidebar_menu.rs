use super::*;

pub(crate) fn sidebar_menu() -> AnyView {
    let items = vec![
        SidebarMenuItem {
            id: "workspace".to_string(),
            label: "Workspace".to_string(),
            href: None,
            badge: Some("6".to_string()),
            action_label: Some("Workspace actions".to_string()),
            disabled: false,
            sub_items: vec![
                SidebarMenuSubItem {
                    id: "overview".to_string(),
                    label: "Overview".to_string(),
                    href: Some("/workspace/overview".to_string()),
                    disabled: false,
                },
                SidebarMenuSubItem {
                    id: "tokens".to_string(),
                    label: "Design tokens".to_string(),
                    href: Some("/workspace/tokens".to_string()),
                    disabled: false,
                },
            ],
            default_sub_open: true,
        },
        SidebarMenuItem {
            id: "releases".to_string(),
            label: "Releases".to_string(),
            href: None,
            badge: Some("2".to_string()),
            action_label: Some("Release actions".to_string()),
            disabled: false,
            sub_items: vec![SidebarMenuSubItem {
                id: "changelog".to_string(),
                label: "Changelog".to_string(),
                href: Some("/releases/changelog".to_string()),
                disabled: false,
            }],
            default_sub_open: false,
        },
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items_first = items.clone();
    let matrix_items_second = items.clone();
    let matrix_items_third = items;

    let (active_id_raw, set_active_id_raw) = signal(Some("tokens".to_string()));
    let active_id: Signal<Option<String>> = Signal::derive(move || active_id_raw.get());
    let (on_active_id_change_runs, set_on_active_id_change_runs) = signal(0_u32);
    let on_active_id_change = Callback::new(move |next: Option<String>| {
        set_active_id_raw.set(next);
        set_on_active_id_change_runs.update(|count| *count += 1);
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let (on_action_runs, set_on_action_runs) = signal(0_u32);
    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
        set_on_action_runs.update(|count| *count += 1);
    });

    let (last_item_action, set_last_item_action) = signal("none".to_string());
    let (on_item_action_runs, set_on_item_action_runs) = signal(0_u32);
    let on_item_action = Callback::new(move |id: String| {
        set_last_item_action.set(id);
        set_on_item_action_runs.update(|count| *count += 1);
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_badges, set_workbench_show_badges) = signal(true);
    let (workbench_show_actions, set_workbench_show_actions) = signal(true);
    let (workbench_allow_submenu_collapse, set_workbench_allow_submenu_collapse) = signal(true);
    let (workbench_enable_shortcut, set_workbench_enable_shortcut) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<SidebarMenu
  items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")]
  id_base="docs-sidebar-menu-hello".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            "docs-sidebar-menu-custom"
        } else {
            ""
        };
        let motion = "SidebarMenuMotion::default()";
        [
            "<SidebarMenu".to_string(),
            "  items=vec![SidebarMenuItem::new(\"workspace\", \"Workspace\"), SidebarMenuItem::new(\"releases\", \"Releases\")]".to_string(),
            "  id_base=\"docs-sidebar-menu-workbench\".to_string()".to_string(),
            "  active_id=active_id".to_string(),
            "  default_active_id=\"tokens\".to_string()".to_string(),
            "  on_active_id_change=on_active_id_change".to_string(),
            "  on_action=on_action".to_string(),
            "  on_item_action=on_item_action".to_string(),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!("  show_badges={}", bool_word(workbench_show_badges.get())),
            format!("  show_actions={}", bool_word(workbench_show_actions.get())),
            format!(
                "  allow_submenu_collapse={}",
                bool_word(workbench_allow_submenu_collapse.get())
            ),
            format!(
                "  enable_keyboard_shortcut={}",
                bool_word(workbench_enable_shortcut.get())
            ),
            "  keyboard_shortcut_key=\"k\".to_string()".to_string(),
            format!("  motion={motion}"),
            "  aria_label=\"Workspace menu\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-sidebar-menu-custom")
        } else {
            None
        };
        let motion = SidebarMenuMotion::default();

        format!(
            "SidebarMenuActualConfig {{\n  items: \"sample_items(len=2)\",\n  id_base: Some(\"docs-sidebar-menu-workbench\"),\n  active_id: {:?},\n  default_active_id: Some(\"tokens\"),\n  on_active_id_change: \"runs={}\",\n  on_action: \"runs={},last={:?}\",\n  on_item_action: \"runs={},last={:?}\",\n  disabled: {},\n  show_badges: {},\n  show_actions: {},\n  allow_submenu_collapse: {},\n  enable_keyboard_shortcut: {},\n  keyboard_shortcut_key: Some(\"k\"),\n  motion: {motion:?},\n  aria_label: Some(\"Workspace menu\"),\n  class_name: {class_name:?},\n}}",
            active_id_raw.get(),
            on_active_id_change_runs.get(),
            on_action_runs.get(),
            last_action.get(),
            on_item_action_runs.get(),
            last_item_action.get(),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_badges.get()),
            bool_word(workbench_show_actions.get()),
            bool_word(workbench_allow_submenu_collapse.get()),
            bool_word(workbench_enable_shortcut.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-default".to_string() default_active_id="tokens".to_string() />
<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-badges-off".to_string() show_badges=false show_actions=true allow_submenu_collapse=true />
<SidebarMenu items=vec![SidebarMenuItem::new("workspace", "Workspace"), SidebarMenuItem::new("releases", "Releases")] id_base="menu-disabled".to_string() disabled=true enable_keyboard_shortcut=false motion=SidebarMenuMotion::default() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenu"
            slug="sidebar-menu"
            group="Layout"
            description="SidebarMenu playground with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default Sidebar Menu)" code_signal=hello_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar menu hello".to_string()
                >
                    <SidebarMenu
                        items=showcase_items
                        id_base="docs-sidebar-menu-hello".to_string()
                    />
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-menu-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_show_badges set_checked=set_workbench_show_badges>
                            "show_badges"
                        </Switch>
                        <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                            "show_actions"
                        </Switch>
                        <Switch
                            checked=workbench_allow_submenu_collapse
                            set_checked=set_workbench_allow_submenu_collapse
                        >
                            "allow_submenu_collapse"
                        </Switch>
                        <Switch checked=workbench_enable_shortcut set_checked=set_workbench_enable_shortcut>
                            "enable_keyboard_shortcut"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="sidebar-menu-workbench-preview">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar menu workbench".to_string()
                    >
                        <SidebarMenu
                            items=workbench_items
                            id_base="docs-sidebar-menu-workbench".to_string()
                            active_id=active_id
                            default_active_id="tokens".to_string()
                            on_active_id_change=on_active_id_change
                            on_action=on_action
                            on_item_action=on_item_action
                            disabled=workbench_disabled.get()
                            show_badges=workbench_show_badges.get()
                            show_actions=workbench_show_actions.get()
                            allow_submenu_collapse=workbench_allow_submenu_collapse.get()
                            enable_keyboard_shortcut=workbench_enable_shortcut.get()
                            keyboard_shortcut_key="k".to_string()
                            motion=SidebarMenuMotion::default()
                            aria_label="Workspace menu".to_string()
                            class_name=if workbench_custom_class.get() {
                                "docs-sidebar-menu-custom".to_string()
                            } else {
                                String::new()
                            }
                        />
                    </Sidebar>
                    <span class="ui-muted" data-slot="sidebar-menu-workbench-feedback">
                        "active_id: "
                        {move || active_id_raw.get().unwrap_or_else(|| "none".to_string())}
                        " · on_active_id_change: " {move || on_active_id_change_runs.get()}
                        " · on_action: " {move || on_action_runs.get()}
                        " · on_item_action: " {move || on_item_action_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Badges Off / Disabled)"
                code_signal=matrix_code
            >
                <div class="docs-row" data-slot="sidebar-menu-state-matrix">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar default".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_first
                            id_base="docs-sidebar-menu-matrix-default".to_string()
                            default_active_id="tokens".to_string()
                        />
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar no badges".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_second
                            id_base="docs-sidebar-menu-matrix-badges-off".to_string()
                            show_badges=false
                            show_actions=true
                            allow_submenu_collapse=true
                        />
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar disabled".to_string()
                    >
                        <SidebarMenu
                            items=matrix_items_third
                            id_base="docs-sidebar-menu-matrix-disabled".to_string()
                            disabled=true
                            enable_keyboard_shortcut=false
                            motion=SidebarMenuMotion::default()
                        />
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
