use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Sidebar, SidebarCollapsible, SidebarFooter, SidebarSide, SidebarVariant};

pub(super) fn sidebar_footer() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"<SidebarFooter bordered=true aria_label="Workspace footer".to_string()>
  <span class="ui-muted">"Free plan"</span>
  <span class="ui-muted">"2 seats remaining"</span>
</SidebarFooter>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<SidebarFooter
  disabled=true
  bordered=true
  aria_label="Disabled usage footer".to_string()
  class_name="docs-sidebar-footer-custom".to_string()
>
  <span class="ui-muted">"Read-only quota"</span>
  <span class="ui-muted">"Upgrade required"</span>
</SidebarFooter>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarFooter"
            slug="sidebar-footer"
            group="Layout"
            description="Shadcn-compatible sidebar footer region primitive with centralized border/disabled/source-state contracts and Spectrum-style data markers."
        >
            <Playground title="Default Footer Region" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar footer playground".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <span>"Dashboard"</span>
                        <span>"Projects"</span>
                        <span>"Billing"</span>
                    </div>
                    <SidebarFooter bordered=true aria_label="Workspace footer".to_string()>
                        <span class="ui-muted">"Free plan"</span>
                        <span class="ui-muted">"2 seats remaining"</span>
                    </SidebarFooter>
                </Sidebar>
            </Playground>

            <Playground title="Disabled + Custom Class" code_signal=disabled_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Disabled footer sidebar".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Tokens"</span>
                        <span class="ui-muted">"Layers"</span>
                    </div>
                    <SidebarFooter
                        disabled=true
                        bordered=true
                        aria_label="Disabled usage footer".to_string()
                        class_name="docs-sidebar-footer-custom".to_string()
                    >
                        <span class="ui-muted">"Read-only quota"</span>
                        <span class="ui-muted">"Upgrade required"</span>
                    </SidebarFooter>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
