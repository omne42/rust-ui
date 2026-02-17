use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarSide, SidebarTrigger, SidebarVariant,
};

pub(super) fn sidebar_trigger() -> AnyView {
    let (open_raw, set_open_raw) = signal(true);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let default_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<SidebarTrigger on_open_change=Callback::new(move |next| set_open_raw.set(next)) />
<Sidebar
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Offcanvas
  show_trigger=false
>
  <SidebarContent>
    <span>"Dashboard"</span>
    <span>"Projects"</span>
    <span>"Billing"</span>
  </SidebarContent>
</Sidebar>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<SidebarTrigger
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  aria_label="Toggle inspector sidebar".to_string()
  label="Inspector".to_string()
  class_name="docs-sidebar-trigger-custom".to_string()
/>
<span class="ui-muted">
  "open: "
  {move || if open_raw.get() { "true" } else { "false" }}
</span>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarTrigger"
            slug="sidebar-trigger"
            group="Layout"
            description="baseline-compatible sidebar trigger primitive with controlled/uncontrolled open state, baseline-style data contracts, and motion-ready state transitions."
        >
            <Playground title="Default Trigger" code_signal=default_code>
                <div class="docs-stack docs-stack--tight">
                    <SidebarTrigger on_open_change=on_open_change />
                    <Sidebar
                        open=open
                        on_open_change=on_open_change
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        show_trigger=false
                        aria_label="Sidebar trigger playground".to_string()
                    >
                        <SidebarContent aria_label="Workspace content".to_string()>
                            <span>"Dashboard"</span>
                            <span>"Projects"</span>
                            <span>"Billing"</span>
                        </SidebarContent>
                    </Sidebar>
                </div>
            </Playground>

            <Playground title="Controlled + Custom Label" code_signal=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <SidebarTrigger
                        open=open
                        on_open_change=on_open_change
                        aria_label="Toggle inspector sidebar".to_string()
                        label="Inspector".to_string()
                        class_name="docs-sidebar-trigger-custom".to_string()
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || if open_raw.get() { "true" } else { "false" }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
