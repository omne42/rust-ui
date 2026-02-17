use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Sidebar, SidebarCollapsible, SidebarGroup, SidebarMenu, SidebarMenuItem, SidebarMenuSubItem,
    SidebarSide, SidebarVariant,
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

    let (action_count, set_action_count) = signal(0_usize);
    let on_group_action = Callback::new(move |_| set_action_count.update(|count| *count += 1));

    let (group_open_raw, set_group_open_raw) = signal(true);
    let group_open: Signal<bool> = Signal::derive(move || group_open_raw.get());
    let on_group_open_change = Callback::new(move |next: bool| set_group_open_raw.set(next));

    let base_code = Signal::derive(move || {
        r#"<SidebarGroup
  label="Help".to_string()
  action_label="Add".to_string()
  on_action=Callback::new(move |_| {})
>
  <SidebarMenu
    items=vec![
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
    ]
  />
</SidebarGroup>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<SidebarGroup
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  collapsible=true
  show_action=false
  label="Architecture".to_string()
>
  <SidebarMenu
    items=vec![SidebarMenuItem {
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
    }]
    allow_submenu_collapse=true
    show_actions=true
    show_badges=false
  />
</SidebarGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarGroup"
            slug="sidebar-group"
            group="Layout"
            description="baseline-compatible sidebar group primitive with label/action header regions, controlled/uncontrolled collapsible state, baseline-style data contracts, and motion-ready collapse behavior."
        >
            <Playground title="Label + Group Action" code_signal=base_code>
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
                        on_action=on_group_action
                        collapsible=false
                        aria_label="Help group".to_string()
                    >
                        <SidebarMenu
                            id_base="docs-sidebar-group-basic".to_string()
                            items=group_items
                            show_actions=false
                            aria_label="Help menu".to_string()
                        />
                    </SidebarGroup>
                    <div class="ui-sidebar__footer">
                        <span class="ui-muted">"group action count: " {move || action_count.get().to_string()}</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground title="Controlled + Collapsible Group" code_signal=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <button
                        class="ui-button"
                        type="button"
                        on:click=move |_| set_group_open_raw.update(|open| *open = !*open)
                    >
                        "Toggle group"
                    </button>

                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Controlled group sidebar".to_string()
                    >
                        <SidebarGroup
                            open=group_open
                            on_open_change=on_group_open_change
                            collapsible=true
                            show_action=false
                            label="Architecture".to_string()
                            aria_label="Architecture group".to_string()
                            class_name="docs-sidebar-group-custom".to_string()
                        >
                            <SidebarMenu
                                id_base="docs-sidebar-group-collapsible".to_string()
                                items=collapsible_items
                                allow_submenu_collapse=true
                                show_actions=true
                                show_badges=false
                                aria_label="Architecture menu".to_string()
                            />
                        </SidebarGroup>

                        <div class="ui-sidebar__footer">
                            <span class="ui-muted">
                                "group open: "
                                {move || if group_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
