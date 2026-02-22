use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarMenuAction, SidebarSide, SidebarVariant,
    Switch,
};

pub(super) fn sidebar_menu_action() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Sidebar
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Icon
  show_trigger=false
>
  <SidebarContent>
    <div class="docs-stack docs-stack--tight">
      <div class="ui-sidebar-menu__item-main">
        <span>"Project Alpha"</span>
        <SidebarMenuAction
          aria_label="Open item actions".to_string()
          on_press=Callback::new(move |_| set_last_action.set("open".to_string()))
        />
      </div>
      <span class="ui-muted">"last action: " {move || last_action.get()}</span>
    </div>
  </SidebarContent>
</Sidebar>"#
            .to_string()
    });

    let always_visible_code = Signal::derive(move || {
        r#"<SidebarMenuAction
  hover_only=false
  disabled=true
  label="!".to_string()
  aria_label="Disabled always-visible action".to_string()
  class_name="docs-sidebar-menu-action-custom".to_string()
/>"#
        .to_string()
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let on_press = Callback::new(move |_| set_last_action.set("open".to_string()));

    let (workbench_hover_only, set_workbench_hover_only) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let on_workbench_press = Callback::new(move |_| {
        set_workbench_press_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<SidebarMenuAction".to_string(),
            format!("  hover_only={}", workbench_hover_only.get()),
            format!("  disabled={}", workbench_disabled.get()),
        ];
        if workbench_custom_label.get() {
            lines.push("  label=\"!\".to_string()".to_string());
        }
        if workbench_custom_aria.get() {
            lines.push("  aria_label=\"Workbench item actions\".to_string()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-sidebar-menu-action-custom\".to_string()".to_string());
        }
        lines.push("  on_press=on_press".to_string());
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarMenuActionActualConfig {{\n  hover_only: {},\n  disabled: {},\n  label: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  on_press: \"count={}\",\n}}",
            workbench_hover_only.get(),
            workbench_disabled.get(),
            if workbench_custom_label.get() {
                Some("!")
            } else {
                None
            },
            if workbench_custom_aria.get() {
                Some("Workbench item actions")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-sidebar-menu-action-custom")
            } else {
                None
            },
            workbench_press_count.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SidebarMenuAction
  hover_only=true
  disabled=false
  aria_label="Hover action".to_string()
  on_press=on_press
/>
<SidebarMenuAction
  hover_only=false
  disabled=false
  label="!".to_string()
  aria_label="Always visible action".to_string()
  class_name="docs-sidebar-menu-action-custom".to_string()
  on_press=on_press
/>
<SidebarMenuAction
  hover_only=false
  disabled=true
  label="x".to_string()
  aria_label="Disabled action".to_string()
  on_press=on_press
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SidebarMenuAction"
            slug="sidebar-menu-action"
            group="Layout"
            description="baseline-compatible sidebar menu action primitive with centralized visibility/disabled/source-state normalization and stable data-marker contracts."
        >
            <Playground title="Default Hover-Only Action" code_signal=default_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Sidebar menu action playground".to_string()
                >
                    <SidebarContent aria_label="Sidebar action rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Project Alpha"</span>
                                <SidebarMenuAction
                                    aria_label="Open item actions".to_string()
                                    on_press=on_press
                                />
                            </div>
                            <span class="ui-muted">"last action: " {move || last_action.get()}</span>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground title="Always Visible + Disabled + Custom" code_signal=always_visible_code>
                <Sidebar
                    side=SidebarSide::Right
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Always visible action sidebar".to_string()
                >
                    <SidebarContent aria_label="Disabled action rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span class="ui-muted">"Locked Project"</span>
                                <SidebarMenuAction
                                    hover_only=false
                                    disabled=true
                                    label="!".to_string()
                                    aria_label="Disabled always-visible action".to_string()
                                    class_name="docs-sidebar-menu-action-custom".to_string()
                                />
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-menu-action-workbench-controls">
                        <Switch checked=workbench_hover_only set_checked=set_workbench_hover_only>
                            "hover_only"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "label"
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
                    aria_label="Sidebar menu action workbench".to_string()
                >
                    <SidebarContent aria_label="Sidebar workbench rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Workbench item"</span>
                                <SidebarMenuAction
                                    hover_only=workbench_hover_only.get()
                                    disabled=workbench_disabled.get()
                                    label=if workbench_custom_label.get() {
                                        "!".to_string()
                                    } else {
                                        String::new()
                                    }
                                    aria_label=if workbench_custom_aria.get() {
                                        "Workbench item actions".to_string()
                                    } else {
                                        String::new()
                                    }
                                    class_name=if workbench_custom_class.get() {
                                        "docs-sidebar-menu-action-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    on_press=on_workbench_press
                                />
                            </div>
                            <span class="ui-muted">
                                "on_press count: "
                                {move || workbench_press_count.get()}
                            </span>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>

            <Playground
                title="State Matrix (Hover / Always Visible / Disabled Comparison)"
                code_signal=matrix_code
            >
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar menu action matrix".to_string()
                >
                    <SidebarContent aria_label="Sidebar action matrix rows".to_string()>
                        <div class="docs-stack docs-stack--tight">
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Hover action"</span>
                                <SidebarMenuAction
                                    hover_only=true
                                    disabled=false
                                    aria_label="Hover action".to_string()
                                    on_press=on_workbench_press
                                />
                            </div>
                            <div class="ui-sidebar-menu__item-main">
                                <span>"Always visible"</span>
                                <SidebarMenuAction
                                    hover_only=false
                                    disabled=false
                                    label="!".to_string()
                                    aria_label="Always visible action".to_string()
                                    class_name="docs-sidebar-menu-action-custom".to_string()
                                    on_press=on_workbench_press
                                />
                            </div>
                            <div class="ui-sidebar-menu__item-main">
                                <span class="ui-muted">"Disabled"</span>
                                <SidebarMenuAction
                                    hover_only=false
                                    disabled=true
                                    label="x".to_string()
                                    aria_label="Disabled action".to_string()
                                    on_press=on_workbench_press
                                />
                            </div>
                        </div>
                    </SidebarContent>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
