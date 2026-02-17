use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Sidebar, SidebarCollapsible, SidebarContent, SidebarSide, SidebarVariant};

pub(super) fn sidebar_content() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"<SidebarContent aria_label="Workspace content".to_string()>
  <span>"Dashboard"</span>
  <span>"Projects"</span>
  <span>"Billing"</span>
</SidebarContent>"#
            .to_string()
    });

    let compact_code = Signal::derive(move || {
        r#"<SidebarContent
  padded=false
  scrollable=false
  aria_label="Compact static content".to_string()
  class_name="docs-sidebar-content-custom".to_string()
>
  <span class="ui-muted">"Activity"</span>
  <span class="ui-muted">"Usage"</span>
</SidebarContent>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarContent"
            slug="sidebar-content"
            group="Layout"
            description="baseline-compatible sidebar content region primitive with centralized padding/scroll/state contracts and baseline-style data markers."
        >
            <Playground title="Default Scrollable Content" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar content playground".to_string()
                >
                    <SidebarContent aria_label="Workspace content".to_string()>
                        <span>"Dashboard"</span>
                        <span>"Projects"</span>
                        <span>"Billing"</span>
                        <span>"Members"</span>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground title="Compact + Static + Custom" code_signal=compact_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Compact sidebar content".to_string()
                >
                    <SidebarContent
                        padded=false
                        scrollable=false
                        aria_label="Compact static content".to_string()
                        class_name="docs-sidebar-content-custom".to_string()
                    >
                        <span class="ui-muted">"Activity"</span>
                        <span class="ui-muted">"Usage"</span>
                        <span class="ui-muted">"Limits"</span>
                    </SidebarContent>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
