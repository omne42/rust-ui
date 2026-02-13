use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarRail, SidebarSide, SidebarVariant,
};

pub(super) fn sidebar_rail() -> AnyView {
    let (open_raw, set_open_raw) = signal(true);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let (right_open_raw, set_right_open_raw) = signal(true);
    let right_open: Signal<bool> = Signal::derive(move || right_open_raw.get());
    let on_right_open_change = Callback::new(move |next: bool| set_right_open_raw.set(next));

    let default_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<SidebarRail on_open_change=Callback::new(move |next| set_open_raw.set(next)) />
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

<SidebarRail
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  side=SidebarSide::Right
  aria_label="Toggle right rail".to_string()
  label="toggle inspector".to_string()
  class_name="docs-sidebar-rail-custom".to_string()
/>
<Sidebar
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  side=SidebarSide::Right
  variant=SidebarVariant::Inset
  collapsible=SidebarCollapsible::Icon
  show_trigger=false
>
  <SidebarContent>
    <span class="ui-muted">"Tokens"</span>
    <span class="ui-muted">"Layers"</span>
    <span class="ui-muted">"Motion"</span>
  </SidebarContent>
</Sidebar>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarRail"
            slug="sidebar-rail"
            group="Layout"
            description="Shadcn-compatible sidebar rail primitive with controlled/uncontrolled open state, side-aware contracts, and Spectrum-style data markers."
        >
            <Playground title="Default Rail" code_signal=default_code>
                <div class="docs-stack docs-stack--tight">
                    <SidebarRail on_open_change=on_open_change />
                    <Sidebar
                        open=open
                        on_open_change=on_open_change
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        show_trigger=false
                        aria_label="Sidebar rail playground".to_string()
                    >
                        <SidebarContent aria_label="Workspace content".to_string()>
                            <span>"Dashboard"</span>
                            <span>"Projects"</span>
                            <span>"Billing"</span>
                        </SidebarContent>
                    </Sidebar>
                </div>
            </Playground>

            <Playground title="Controlled Right Rail" code_signal=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <SidebarRail
                        open=right_open
                        on_open_change=on_right_open_change
                        side=SidebarSide::Right
                        aria_label="Toggle right rail".to_string()
                        label="toggle inspector".to_string()
                        class_name="docs-sidebar-rail-custom".to_string()
                    />
                    <Sidebar
                        open=right_open
                        on_open_change=on_right_open_change
                        side=SidebarSide::Right
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Right inspector sidebar".to_string()
                    >
                        <SidebarContent aria_label="Inspector content".to_string()>
                            <span class="ui-muted">"Tokens"</span>
                            <span class="ui-muted">"Layers"</span>
                            <span class="ui-muted">"Motion"</span>
                        </SidebarContent>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
