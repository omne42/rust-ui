use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Sidebar, SidebarCollapsible, SidebarInset, SidebarSide, SidebarVariant};

pub(super) fn sidebar_inset() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"<SidebarInset aria_label="Workspace inset region".to_string()>
  <span>"Overview"</span>
  <span>"Recent activity"</span>
  <span>"Pinned links"</span>
</SidebarInset>"#
            .to_string()
    });

    let compact_code = Signal::derive(move || {
        r#"<SidebarInset
  side=SidebarSide::Right
  padded=false
  recessed=false
  disabled=true
  aria_label="Inspector inset panel".to_string()
  class_name="docs-sidebar-inset-custom".to_string()
>
  <span class="ui-muted">"Read-only"</span>
  <span class="ui-muted">"3 warnings"</span>
</SidebarInset>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarInset"
            slug="sidebar-inset"
            group="Layout"
            description="Shadcn-compatible sidebar inset primitive with side/padding/surface contracts and Spectrum-style root data markers."
        >
            <Playground title="Default Inset Region" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar inset playground".to_string()
                >
                    <SidebarInset aria_label="Workspace inset region".to_string()>
                        <span>"Overview"</span>
                        <span>"Recent activity"</span>
                        <span>"Pinned links"</span>
                    </SidebarInset>
                </Sidebar>
            </Playground>

            <Playground title="Compact + Plain + Disabled" code_signal=compact_code>
                <Sidebar
                    side=SidebarSide::Right
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Inspector sidebar inset".to_string()
                >
                    <SidebarInset
                        side=SidebarSide::Right
                        padded=false
                        recessed=false
                        disabled=true
                        aria_label="Inspector inset panel".to_string()
                        class_name="docs-sidebar-inset-custom".to_string()
                    >
                        <span class="ui-muted">"Read-only"</span>
                        <span class="ui-muted">"3 warnings"</span>
                    </SidebarInset>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
