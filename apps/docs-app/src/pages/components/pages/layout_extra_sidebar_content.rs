use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Sidebar, SidebarCollapsible, SidebarContent, SidebarSide, SidebarVariant, Switch};

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

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_padded, set_workbench_padded) = signal(true);
    let (workbench_scrollable, set_workbench_scrollable) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<SidebarContent\n  disabled={}\n  padded={}\n  scrollable={}\n  aria_label={:?}.to_string()\n  class_name={:?}.to_string()\n>\n  <span>\"Workbench row\"</span>\n</SidebarContent>",
            workbench_disabled.get(),
            workbench_padded.get(),
            workbench_scrollable.get(),
            if workbench_custom_aria.get() {
                "Workbench content"
            } else {
                ""
            },
            if workbench_custom_class.get() {
                "docs-sidebar-content-custom"
            } else {
                ""
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarContentActualConfig {{\n  disabled: {},\n  padded: {},\n  scrollable: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_disabled.get(),
            workbench_padded.get(),
            workbench_scrollable.get(),
            if workbench_custom_aria.get() {
                Some("Workbench content")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-sidebar-content-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarContent
  disabled=false
  padded=true
  scrollable=true
  aria_label="Default content".to_string()
>
  <span>"Dashboard"</span>
</SidebarContent>
<SidebarContent
  disabled=true
  padded=false
  scrollable=false
  aria_label="Disabled compact content".to_string()
  class_name="docs-sidebar-content-custom".to_string()
>
  <span>"Read-only"</span>
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

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-content-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_padded set_checked=set_workbench_padded>
                            "padded"
                        </Switch>
                        <Switch checked=workbench_scrollable set_checked=set_workbench_scrollable>
                            "scrollable"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar content workbench".to_string()
                >
                    <SidebarContent
                        disabled=workbench_disabled.get()
                        padded=workbench_padded.get()
                        scrollable=workbench_scrollable.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench content".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-content-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <span>"Dashboard"</span>
                        <span>"Projects"</span>
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

            <Playground
                title="State Matrix (Padding / Scroll / Disabled Comparison)"
                code_signal=matrix_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        show_trigger=false
                        aria_label="Sidebar content matrix default".to_string()
                    >
                        <SidebarContent
                            disabled=false
                            padded=true
                            scrollable=true
                            aria_label="Default content".to_string()
                        >
                            <span>"Dashboard"</span>
                            <span>"Projects"</span>
                        </SidebarContent>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar content matrix compact".to_string()
                    >
                        <SidebarContent
                            disabled=true
                            padded=false
                            scrollable=false
                            aria_label="Disabled compact content".to_string()
                            class_name="docs-sidebar-content-custom".to_string()
                        >
                            <span>"Read-only"</span>
                        </SidebarContent>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
