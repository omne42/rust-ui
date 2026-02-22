use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Sidebar, SidebarCollapsible, SidebarFooter, SidebarSide, SidebarVariant, Switch};

pub(super) fn sidebar_footer() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<SidebarFooter bordered=true aria_label="Workspace footer".to_string()>
  <span class="ui-muted">"Free plan"</span>
  <span class="ui-muted">"2 seats remaining"</span>
</SidebarFooter>"#
            .to_string()
    });

    let (disabled, set_disabled) = signal(false);
    let (bordered, set_bordered) = signal(true);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<SidebarFooter".to_string()];
        lines.push(format!("  disabled={}", disabled.get()));
        lines.push(format!("  bordered={}", bordered.get()));
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Disabled usage footer\".to_string()".to_string());
        } else {
            lines.push("  aria_label=\"Workspace footer\".to_string()".to_string());
        }
        if custom_class_name.get() {
            lines.push("  class_name=\"docs-sidebar-footer-custom\".to_string()".to_string());
        }
        lines.push(">".to_string());
        lines.push("  <span class=\"ui-muted\">\"Quota\"</span>".to_string());
        lines.push("</SidebarFooter>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "SidebarFooterWorkbenchConfig {{\n  disabled: {},\n  bordered: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            disabled.get(),
            bordered.get(),
            if custom_aria_label.get() {
                "Disabled usage footer"
            } else {
                "Workspace footer"
            },
            if custom_class_name.get() {
                "docs-sidebar-footer-custom"
            } else {
                ""
            },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<SidebarFooter bordered=true aria_label="Workspace footer".to_string()>
  <span class="ui-muted">"Free plan"</span>
</SidebarFooter>
<SidebarFooter disabled=true bordered=true aria_label="Disabled usage footer".to_string() class_name="docs-sidebar-footer-custom".to_string()>
  <span class="ui-muted">"Read-only quota"</span>
</SidebarFooter>
<SidebarFooter disabled=false bordered=false aria_label="Minimal footer".to_string()>
  <span class="ui-muted">"No border"</span>
</SidebarFooter>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarFooter"
            slug="sidebar-footer"
            group="Layout"
            description="baseline-compatible sidebar footer region primitive with centralized border/disabled/source-state contracts and baseline-style data markers."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_code>
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

            <Playground
                title="Workbench (Disabled + Border + Source)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=bordered set_checked=set_bordered>"Bordered"</Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class_name set_checked=set_custom_class_name>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
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
                        disabled=disabled.get()
                        bordered=bordered.get()
                        aria_label=if custom_aria_label.get() {
                            "Disabled usage footer".to_string()
                        } else {
                            "Workspace footer".to_string()
                        }
                        class_name=if custom_class_name.get() {
                            "docs-sidebar-footer-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <span class="ui-muted">"Read-only quota"</span>
                        <span class="ui-muted">"Upgrade required"</span>
                    </SidebarFooter>
                </Sidebar>
            </Playground>

            <Playground title="State Matrix (Bordered / Disabled / Minimal)" code_signal=matrix_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar footer matrix".to_string()
                >
                    <SidebarFooter bordered=true aria_label="Workspace footer".to_string()>
                        <span class="ui-muted">"Free plan"</span>
                        <span class="ui-muted">"2 seats remaining"</span>
                    </SidebarFooter>
                    <SidebarFooter
                        disabled=true
                        bordered=true
                        aria_label="Disabled usage footer".to_string()
                        class_name="docs-sidebar-footer-custom".to_string()
                    >
                        <span class="ui-muted">"Read-only quota"</span>
                    </SidebarFooter>
                    <SidebarFooter bordered=false aria_label="Minimal footer".to_string()>
                        <span class="ui-muted">"No border"</span>
                    </SidebarFooter>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
