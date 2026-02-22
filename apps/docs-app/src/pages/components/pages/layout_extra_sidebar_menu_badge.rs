use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarMenuBadge, SidebarSide, SidebarVariant,
    Switch,
};

pub(super) fn sidebar_menu_badge() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<SidebarMenuBadge aria_label="Open reviews".to_string()>"7"</SidebarMenuBadge>"#
            .to_string()
    });

    let (muted, set_muted) = signal(false);
    let (disabled, set_disabled) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<SidebarMenuBadge".to_string()];
        if muted.get() {
            lines.push("  muted=true".to_string());
        }
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Muted archived items\".to_string()".to_string());
        } else {
            lines.push("  aria_label=\"Open reviews\".to_string()".to_string());
        }
        if custom_class_name.get() {
            lines.push("  class_name=\"docs-sidebar-menu-badge-custom\".to_string()".to_string());
        }
        lines.push(">".to_string());
        lines.push("  \"7\"".to_string());
        lines.push("</SidebarMenuBadge>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "SidebarMenuBadgeWorkbenchConfig {{\n  muted: {},\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            muted.get(),
            disabled.get(),
            if custom_aria_label.get() {
                "Muted archived items"
            } else {
                "Open reviews"
            },
            if custom_class_name.get() {
                "docs-sidebar-menu-badge-custom"
            } else {
                ""
            },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<SidebarMenuBadge aria_label="Open reviews".to_string()>"7"</SidebarMenuBadge>
<SidebarMenuBadge muted=true aria_label="Muted archived items".to_string()>"archived"</SidebarMenuBadge>
<SidebarMenuBadge muted=true disabled=true aria_label="Muted archived items".to_string() class_name="docs-sidebar-menu-badge-custom".to_string()>"archived"</SidebarMenuBadge>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenuBadge"
            slug="sidebar-menu-badge"
            group="Layout"
            description="baseline-compatible sidebar menu badge primitive with centralized tone/disabled/source-state normalization and stable data-marker contracts."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_code>
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

            <Playground
                title="Workbench (Muted + Disabled + Source)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=muted set_checked=set_muted>"Muted"</Switch>
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
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
                                    muted=muted.get()
                                    disabled=disabled.get()
                                    aria_label=if custom_aria_label.get() {
                                        "Muted archived items".to_string()
                                    } else {
                                        "Open reviews".to_string()
                                    }
                                    class_name=if custom_class_name.get() {
                                        "docs-sidebar-menu-badge-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                >
                                    {if muted.get() { "archived" } else { "7" }}
                                </SidebarMenuBadge>
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground title="State Matrix (Default / Muted / Disabled)" code_signal=matrix_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar menu badge matrix".to_string()
                >
                    <SidebarContent aria_label="Sidebar menu badge matrix rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Open reviews"</span>
                                <SidebarMenuBadge aria_label="Open reviews".to_string()>"7"</SidebarMenuBadge>
                            </div>
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Archived"</span>
                                <SidebarMenuBadge muted=true aria_label="Muted archived items".to_string()>
                                    "archived"
                                </SidebarMenuBadge>
                            </div>
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Disabled archived"</span>
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
