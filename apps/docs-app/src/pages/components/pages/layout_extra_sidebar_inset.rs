use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Sidebar, SidebarCollapsible, SidebarInset, SidebarSide, SidebarVariant, Switch};

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

    let (workbench_side_right, set_workbench_side_right) = signal(false);
    let (workbench_padded, set_workbench_padded) = signal(true);
    let (workbench_recessed, set_workbench_recessed) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_side = Signal::derive(move || {
        if workbench_side_right.get() {
            SidebarSide::Right
        } else {
            SidebarSide::Left
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<SidebarInset\n  side=SidebarSide::{:?}\n  padded={}\n  recessed={}\n  disabled={}\n  aria_label={:?}.to_string()\n  class_name={:?}.to_string()\n>\n  <span>\"Workbench content\"</span>\n</SidebarInset>",
            workbench_side.get(),
            workbench_padded.get(),
            workbench_recessed.get(),
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                "Workbench inset panel"
            } else {
                ""
            },
            if workbench_custom_class.get() {
                "docs-sidebar-inset-custom"
            } else {
                ""
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarInsetActualConfig {{\n  side: {:?},\n  padded: {},\n  recessed: {},\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_side.get(),
            workbench_padded.get(),
            workbench_recessed.get(),
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                Some("Workbench inset panel")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-sidebar-inset-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarInset
  side=SidebarSide::Left
  padded=true
  recessed=true
  disabled=false
  aria_label="Default inset".to_string()
>
  <span>"Overview"</span>
</SidebarInset>
<SidebarInset
  side=SidebarSide::Right
  padded=false
  recessed=false
  disabled=true
  aria_label="Disabled inset".to_string()
  class_name="docs-sidebar-inset-custom".to_string()
>
  <span>"Read-only"</span>
</SidebarInset>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarInset"
            slug="sidebar-inset"
            group="Layout"
            description="baseline-compatible sidebar inset primitive with side/padding/surface contracts and baseline-style root data markers."
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

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-inset-workbench-controls">
                        <Switch checked=workbench_side_right set_checked=set_workbench_side_right>
                            "side=Right"
                        </Switch>
                        <Switch checked=workbench_padded set_checked=set_workbench_padded>
                            "padded"
                        </Switch>
                        <Switch checked=workbench_recessed set_checked=set_workbench_recessed>
                            "recessed"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
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
                    side=workbench_side.get()
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar inset workbench".to_string()
                >
                    <SidebarInset
                        side=workbench_side.get()
                        padded=workbench_padded.get()
                        recessed=workbench_recessed.get()
                        disabled=workbench_disabled.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench inset panel".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-inset-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <span>"Overview"</span>
                        <span>"Recent activity"</span>
                    </SidebarInset>
                </Sidebar>
            </Playground>

            <Playground
                title="State Matrix (Side / Surface / Disabled Comparison)"
                code_signal=matrix_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Sidebar inset matrix left".to_string()
                    >
                        <SidebarInset
                            side=SidebarSide::Left
                            padded=true
                            recessed=true
                            disabled=false
                            aria_label="Default inset".to_string()
                        >
                            <span>"Overview"</span>
                        </SidebarInset>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Right
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::Offcanvas
                        show_trigger=false
                        aria_label="Sidebar inset matrix right".to_string()
                    >
                        <SidebarInset
                            side=SidebarSide::Right
                            padded=false
                            recessed=false
                            disabled=true
                            aria_label="Disabled inset".to_string()
                            class_name="docs-sidebar-inset-custom".to_string()
                        >
                            <span>"Read-only"</span>
                        </SidebarInset>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
