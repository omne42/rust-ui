use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Sidebar, SidebarCollapsible, SidebarContent, SidebarRail, SidebarSide, SidebarVariant};

pub(super) fn sidebar_rail() -> AnyView {
    let (open_raw, set_open_raw) = signal(true);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let (workbench_default_open, set_workbench_default_open) = signal(true);
    let (workbench_right_side, set_workbench_right_side) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_open_raw.set(next);
        set_workbench_change_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<SidebarRail\n  open=Signal::derive(move || open_raw.get())\n  default_open={}\n  on_open_change=Callback::new(move |next| set_open_raw.set(next))\n  side=SidebarSide::{:?}\n  disabled={}\n  aria_label={}\n  label={}\n  class_name={}\n/>",
            workbench_default_open.get(),
            if workbench_right_side.get() {
                SidebarSide::Right
            } else {
                SidebarSide::Left
            },
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                "\"Toggle rail\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_label.get() {
                "\"toggle inspector\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-sidebar-rail-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarRailActualConfig {{\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: \"count={}\",\n  side: SidebarSide::{:?},\n  disabled: {},\n  aria_label: {},\n  label: {},\n  class_name: {},\n}}",
            open_raw.get(),
            workbench_default_open.get(),
            workbench_change_count.get(),
            if workbench_right_side.get() {
                SidebarSide::Right
            } else {
                SidebarSide::Left
            },
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                "Some(\"Toggle rail\")"
            } else {
                "None"
            },
            if workbench_custom_label.get() {
                "Some(\"toggle inspector\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-sidebar-rail-custom\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<SidebarRail default_open=true side=SidebarSide::Left />
<SidebarRail open=Signal::derive(move || open_raw.get()) on_open_change=Callback::new(move |next| set_open_raw.set(next)) side=SidebarSide::Right />
<SidebarRail disabled=true side=SidebarSide::Right class_name="docs-sidebar-rail-custom".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SidebarRail"
            slug="sidebar-rail"
            group="Layout"
            description="baseline-compatible sidebar rail primitive with controlled/uncontrolled open state, side-aware contracts, and baseline-style data markers."
        >
            <Playground
                title="Hello World (Default Rail)"
                code_signal=Signal::derive(move || r#"<SidebarRail default_open=true />"#.to_string())
            >
                <div class="docs-stack docs-stack--tight">
                    <SidebarRail default_open=true on_open_change=on_open_change />
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-rail-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_default_open.get()
                                on:change=move |ev| set_workbench_default_open.set(event_target_checked(&ev))
                            />
                            " default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_right_side.get()
                                on:change=move |ev| set_workbench_right_side.set(event_target_checked(&ev))
                            />
                            " side right"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_label.get()
                                on:change=move |ev| set_workbench_custom_label.set(event_target_checked(&ev))
                            />
                            " label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <SidebarRail
                        open=open
                        default_open=workbench_default_open.get()
                        on_open_change=on_workbench_open_change
                        side=if workbench_right_side.get() {
                            SidebarSide::Right
                        } else {
                            SidebarSide::Left
                        }
                        disabled=workbench_disabled.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Toggle rail".to_string()
                        } else {
                            String::new()
                        }
                        label=if workbench_custom_label.get() {
                            "toggle inspector".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-rail-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <Sidebar
                        open=open
                        on_open_change=on_workbench_open_change
                        side=if workbench_right_side.get() {
                            SidebarSide::Right
                        } else {
                            SidebarSide::Left
                        }
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
                    <span class="ui-muted">
                        "open: " {move || if open_raw.get() { "true" } else { "false" }}
                        " · on_open_change: " {move || workbench_change_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Left / Right / Disabled)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <SidebarRail default_open=true side=SidebarSide::Left />
                    <SidebarRail open=open on_open_change=on_open_change side=SidebarSide::Right />
                    <SidebarRail
                        disabled=true
                        side=SidebarSide::Right
                        class_name="docs-sidebar-rail-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
