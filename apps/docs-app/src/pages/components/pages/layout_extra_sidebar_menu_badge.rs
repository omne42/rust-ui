use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarMenuBadge, SidebarSide, SidebarVariant,
};

pub(super) fn sidebar_menu_badge() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"<SidebarMenuBadge aria_label="Open reviews".to_string()>"7"</SidebarMenuBadge>"#
            .to_string()
    });

    let muted_code = Signal::derive(move || {
        r#"<SidebarMenuBadge
  muted=true
  disabled=true
  aria_label="Muted archived items".to_string()
  class_name="docs-sidebar-menu-badge-custom".to_string()
>
  "archived"
</SidebarMenuBadge>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenuBadge"
            slug="sidebar-menu-badge"
            group="Layout"
            description="baseline-compatible sidebar menu badge primitive with centralized tone/disabled/source-state normalization and stable data-marker contracts."
        >
            <Playground title="Default Numeric Badge" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar menu badge playground".to_string()
                >
                    <SidebarContent aria_label="Sidebar badge rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Open reviews"</span>
                                <SidebarMenuBadge aria_label="Open reviews".to_string()>"7"</SidebarMenuBadge>
                            </div>
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Deploy requests"</span>
                                <SidebarMenuBadge aria_label="Deploy requests".to_string()>"2"</SidebarMenuBadge>
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground title="Muted + Disabled + Custom" code_signal=muted_code>
                <Sidebar
                    side=SidebarSide::Right
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Muted badge sidebar".to_string()
                >
                    <SidebarContent aria_label="Muted badge rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span class="ui-muted">"Archived items"</span>
                                <SidebarMenuBadge
                                    muted=true
                                    disabled=true
                                    aria_label="Muted archived items".to_string()
                                    class_name="docs-sidebar-menu-badge-custom".to_string()
                                >
                                    "archived"
                                </SidebarMenuBadge>
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
