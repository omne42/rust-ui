use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarMenuAction, SidebarSide, SidebarVariant,
};

pub(super) fn sidebar_menu_action() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Sidebar
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Icon
  show_trigger=false
>
  <SidebarContent>
    <div class="docs-stack docs-stack--tight">
      <div class="ui-sidebar-menu__item-main">
        <span>"Project Alpha"</span>
        <SidebarMenuAction
          aria_label="Open item actions".to_string()
          on_press=Callback::new(move |_| set_last_action.set("open".to_string()))
        />
      </div>
      <span class="ui-muted">"last action: " {move || last_action.get()}</span>
    </div>
  </SidebarContent>
</Sidebar>"#
            .to_string()
    });

    let always_visible_code = Signal::derive(move || {
        r#"<SidebarMenuAction
  hover_only=false
  disabled=true
  label="!".to_string()
  aria_label="Disabled always-visible action".to_string()
  class_name="docs-sidebar-menu-action-custom".to_string()
/>"#
        .to_string()
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let on_press = Callback::new(move |_| set_last_action.set("open".to_string()));

    view! {
        <ComponentPage
            title="SidebarMenuAction"
            slug="sidebar-menu-action"
            group="Layout"
            description="baseline-compatible sidebar menu action primitive with centralized visibility/disabled/source-state normalization and stable data-marker contracts."
        >
            <Playground title="Default Hover-Only Action" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar menu action playground".to_string()
                >
                    <SidebarContent aria_label="Sidebar action rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Project Alpha"</span>
                                <SidebarMenuAction
                                    aria_label="Open item actions".to_string()
                                    on_press=on_press
                                />
                            </div>
                            <span class="ui-muted">"last action: " {move || last_action.get()}</span>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground title="Always Visible + Disabled + Custom" code_signal=always_visible_code>
                <Sidebar
                    side=SidebarSide::Right
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Always visible action sidebar".to_string()
                >
                    <SidebarContent aria_label="Disabled action rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span class="ui-muted">"Locked Project"</span>
                                <SidebarMenuAction
                                    hover_only=false
                                    disabled=true
                                    label="!".to_string()
                                    aria_label="Disabled always-visible action".to_string()
                                    class_name="docs-sidebar-menu-action-custom".to_string()
                                />
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
