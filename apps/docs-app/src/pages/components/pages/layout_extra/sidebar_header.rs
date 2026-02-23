use super::*;

pub(crate) fn sidebar_header() -> AnyView {
    let basic_code = Signal::derive(move || {
        r#"<SidebarHeader aria_label="Workspace header".to_string()>
  <strong>"Workspace"</strong>
  <span class="ui-muted">"5 active projects"</span>
</SidebarHeader>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<SidebarHeader
  disabled=true
  aria_label="Disabled inspector header".to_string()
  class_name="docs-sidebar-header-custom".to_string()
>
  <strong>"Inspector"</strong>
  <span class="ui-muted">"Read-only mode"</span>
</SidebarHeader>"#
            .to_string()
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_code = Signal::derive(move || {
        format!(
            "<SidebarHeader\n  disabled={}\n  aria_label={}\n  class_name={}\n>\n  <strong>\"Inspector\"</strong>\n  <span class=\"ui-muted\">\"Read-only mode\"</span>\n</SidebarHeader>",
            bool_word(workbench_disabled.get()),
            rust_string_literal(if workbench_custom_aria.get() {
                "Workbench inspector header"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_class.get() {
                "docs-sidebar-header-custom"
            } else {
                ""
            }),
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarHeaderActualConfig {{\n  disabled: {},\n  aria_label: {},\n  class_name: {},\n}}",
            bool_word(workbench_disabled.get()),
            if workbench_custom_aria.get() {
                "Some(\"Workbench inspector header\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-sidebar-header-custom\")"
            } else {
                "None"
            },
        )
    });

    view! {
        <ComponentPage
            title="SidebarHeader"
            slug="sidebar-header"
            group="Layout"
            description="baseline-compatible sidebar header region primitive with centralized disabled/source-state contracts and baseline-style data markers."
        >
            <Playground title="Hello World (Default Header Region)" code_signal=basic_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    show_trigger=false
                    aria_label="Sidebar header playground".to_string()
                >
                    <SidebarHeader aria_label="Workspace header".to_string()>
                        <strong>"Workspace"</strong>
                        <span class="ui-muted">"5 active projects"</span>
                    </SidebarHeader>
                    <div class="docs-stack docs-stack--tight">
                        <span>"Dashboard"</span>
                        <span>"Projects"</span>
                        <span>"Billing"</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (Disabled + Aria + Class)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
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
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Workbench header sidebar".to_string()
                >
                    <SidebarHeader
                        disabled=workbench_disabled.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench inspector header".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-header-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <strong>"Inspector"</strong>
                        <span class="ui-muted">"Read-only mode"</span>
                    </SidebarHeader>
                </Sidebar>
            </Playground>

            <Playground title="State Matrix (Disabled + Custom Class)" code_signal=disabled_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Inset
                    collapsible=SidebarCollapsible::Icon
                    show_trigger=false
                    aria_label="Disabled header sidebar".to_string()
                >
                    <SidebarHeader
                        disabled=true
                        aria_label="Disabled inspector header".to_string()
                        class_name="docs-sidebar-header-custom".to_string()
                    >
                        <strong>"Inspector"</strong>
                        <span class="ui-muted">"Read-only mode"</span>
                    </SidebarHeader>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Tokens"</span>
                        <span class="ui-muted">"Layers"</span>
                    </div>
                </Sidebar>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
