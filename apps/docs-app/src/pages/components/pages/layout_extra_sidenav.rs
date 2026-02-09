use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    SidebarCollapsible, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarSide, SidebarVariant,
    Sidenav,
};

pub(super) fn sidenav() -> AnyView {
    let (open, set_open) = signal(true);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));

    let controlled_items = vec![
        SidebarMenuItem {
            id: "dashboard".to_string(),
            label: "Dashboard".to_string(),
            href: Some("/dashboard".to_string()),
            badge: Some("8".to_string()),
            action_label: Some("Open Dashboard".to_string()),
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "reports".to_string(),
            label: "Reports".to_string(),
            href: Some("/reports".to_string()),
            badge: None,
            action_label: None,
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
    ];

    let static_items = vec![
        SidebarMenuItem {
            id: "overview".to_string(),
            label: "Overview".to_string(),
            href: Some("/overview".to_string()),
            badge: None,
            action_label: None,
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
        SidebarMenuItem {
            id: "settings".to_string(),
            label: "Settings".to_string(),
            href: Some("/settings".to_string()),
            badge: None,
            action_label: None,
            disabled: false,
            sub_items: vec![],
            default_sub_open: false,
        },
    ];

    let controlled_code = r#"let (open, set_open) = signal(true);
<Sidenav
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  side=SidebarSide::Right
  variant=SidebarVariant::Floating
>
  {content}
</Sidenav>"#;

    let states_code = r#"<Sidenav
  default_open=false
  collapsible=SidebarCollapsible::Icon
  enable_shortcut=false
  trigger_label=\"Toggle nav\".to_string()
>
  {content}
</Sidenav>"#;

    view! {
        <ComponentPage
            title="Sidenav"
            slug="sidenav"
            group="Layout"
            description="Spectrum-compatible Sidenav alias for upstream naming parity, preserving Sidebar controlled/uncontrolled accessibility contracts and HeroUI-level trigger/rail interaction behavior."
        >
            <Playground title="Controlled + Floating" code=controlled_code>
                <div class="docs-stack">
                    <Sidenav
                        open=Signal::derive(move || open.get())
                        on_open_change=on_open_change
                        side=SidebarSide::Right
                        variant=SidebarVariant::Floating
                        trigger_label="Toggle nav".to_string()
                    >
                        <SidebarHeader>
                            <div class="ui-muted">"Sidenav Header"</div>
                        </SidebarHeader>
                        <SidebarMenu items=controlled_items />
                    </Sidenav>
                    <span class="ui-muted">"open: " {move || open.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Icon Collapsible + No Shortcut" code=states_code>
                <Sidenav
                    default_open=false
                    collapsible=SidebarCollapsible::Icon
                    enable_shortcut=false
                    trigger_label="Toggle nav".to_string()
                    aria_label="Project navigation".to_string()
                    class_name="docs-sidenav-static".to_string()
                >
                    <SidebarHeader>
                        <div class="ui-muted">"Project"</div>
                    </SidebarHeader>
                    <SidebarMenu items=static_items />
                </Sidenav>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
