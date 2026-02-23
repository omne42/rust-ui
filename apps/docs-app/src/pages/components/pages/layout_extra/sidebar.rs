use super::*;

pub(crate) fn sidebar() -> AnyView {
    let showcase_code = Signal::derive(move || {
        r#"<Sidebar
  side=SidebarSide::Left
  variant=SidebarVariant::Sidebar
  collapsible=SidebarCollapsible::Offcanvas
  aria_label="Project navigation".to_string()
>
  <div class="ui-sidebar__header"><strong>"Workspace"</strong></div>
  <div class="ui-sidebar__content"><span>"Dashboard"</span><span>"Analytics"</span><span>"Settings"</span></div>
</Sidebar>"#
            .to_string()
    });

    let side_options = vec!["Left".to_string(), "Right".to_string()];
    let variant_options = vec![
        "Sidebar".to_string(),
        "Floating".to_string(),
        "Inset".to_string(),
    ];
    let collapsible_options = vec![
        "Offcanvas".to_string(),
        "Icon".to_string(),
        "None".to_string(),
    ];

    let (workbench_open_raw, set_workbench_open_raw) = signal(true);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_last_open, set_workbench_last_open) = signal(true);
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_last_open.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });

    let (workbench_side_index, set_workbench_side_index) = signal(Some(0_usize));
    let workbench_side = Signal::derive(move || match workbench_side_index.get().unwrap_or(0) {
        1 => SidebarSide::Right,
        _ => SidebarSide::Left,
    });
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => SidebarVariant::Floating,
            2 => SidebarVariant::Inset,
            _ => SidebarVariant::Sidebar,
        });
    let (workbench_collapsible_index, set_workbench_collapsible_index) = signal(Some(0_usize));
    let workbench_collapsible =
        Signal::derive(
            move || match workbench_collapsible_index.get().unwrap_or(0) {
                1 => SidebarCollapsible::Icon,
                2 => SidebarCollapsible::None,
                _ => SidebarCollapsible::Offcanvas,
            },
        );
    let (workbench_default_open, set_workbench_default_open) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_show_trigger, set_workbench_show_trigger) = signal(true);
    let (workbench_enable_shortcut, set_workbench_enable_shortcut) = signal(true);
    let (workbench_custom_shortcut, set_workbench_custom_shortcut) = signal(false);
    let (workbench_custom_trigger_label, set_workbench_custom_trigger_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let side = match workbench_side.get() {
            SidebarSide::Right => "SidebarSide::Right",
            SidebarSide::Left => "SidebarSide::Left",
        };
        let variant = match workbench_variant.get() {
            SidebarVariant::Floating => "SidebarVariant::Floating",
            SidebarVariant::Inset => "SidebarVariant::Inset",
            SidebarVariant::Sidebar => "SidebarVariant::Sidebar",
        };
        let collapsible = match workbench_collapsible.get() {
            SidebarCollapsible::Icon => "SidebarCollapsible::Icon",
            SidebarCollapsible::None => "SidebarCollapsible::None",
            SidebarCollapsible::Offcanvas => "SidebarCollapsible::Offcanvas",
        };

        format!(
            "let (open_raw, set_open_raw) = signal({});\nlet open = Signal::derive(move || open_raw.get());\nlet on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));\n\n<Sidebar\n  open=open\n  default_open={}\n  on_open_change=on_open_change\n  side={side}\n  variant={variant}\n  collapsible={collapsible}\n  disabled={}\n  show_trigger={}\n  enable_shortcut={}\n  shortcut_key={}.to_string()\n  trigger_label={}.to_string()\n  aria_label=\"Project navigation sidebar\".to_string()\n  class_name={}\n/>",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_default_open.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_trigger.get()),
            bool_word(workbench_enable_shortcut.get()),
            rust_string_literal(if workbench_custom_shortcut.get() {
                "j"
            } else {
                "b"
            }),
            rust_string_literal(if workbench_custom_trigger_label.get() {
                "Toggle nav panel"
            } else {
                "Toggle sidebar"
            }),
            if workbench_custom_class.get() {
                "\"docs-sidebar-workbench\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SidebarWorkbenchActualConfig {{\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: \"count={}, last={}\",\n  side: {:?},\n  variant: {:?},\n  collapsible: {:?},\n  disabled: {},\n  show_trigger: {},\n  enable_shortcut: {},\n  shortcut_key: Some({:?}),\n  trigger_label: Some({:?}),\n  aria_label: Some(\"Project navigation sidebar\"),\n  class_name: {:?},\n}}",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_default_open.get()),
            workbench_open_change_count.get(),
            bool_word(workbench_last_open.get()),
            workbench_side.get(),
            workbench_variant.get(),
            workbench_collapsible.get(),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_show_trigger.get()),
            bool_word(workbench_enable_shortcut.get()),
            if workbench_custom_shortcut.get() {
                "j"
            } else {
                "b"
            },
            if workbench_custom_trigger_label.get() {
                "Toggle nav panel"
            } else {
                "Toggle sidebar"
            },
            if workbench_custom_class.get() {
                Some("docs-sidebar-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Sidebar side=SidebarSide::Left variant=SidebarVariant::Sidebar collapsible=SidebarCollapsible::Offcanvas aria_label="Left default".to_string() />
<Sidebar side=SidebarSide::Right variant=SidebarVariant::Floating collapsible=SidebarCollapsible::Icon show_trigger=false aria_label="Right floating".to_string() />
<Sidebar side=SidebarSide::Left variant=SidebarVariant::Inset collapsible=SidebarCollapsible::None disabled=true enable_shortcut=false trigger_label="Disabled".to_string() aria_label="Disabled sidebar".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Sidebar"
            slug="sidebar"
            group="Layout"
            description="baseline-compatible sidebar primitive with controlled/uncontrolled open state, side+variant+collapsible contracts, keyboard shortcut toggle, and baseline-style data markers."
        >
            <Playground title="Hello World (Default Sidebar)" code_signal=showcase_code>
                <Sidebar
                    side=SidebarSide::Left
                    variant=SidebarVariant::Sidebar
                    collapsible=SidebarCollapsible::Offcanvas
                    aria_label="Project navigation".to_string()
                >
                    <div class="ui-sidebar__header">
                        <strong>"Workspace"</strong>
                    </div>
                    <div class="ui-sidebar__content">
                        <span>"Dashboard"</span>
                        <span>"Analytics"</span>
                        <span>"Settings"</span>
                    </div>
                </Sidebar>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sidebar-workbench-controls">
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-side".to_string()
                            options=side_options.clone()
                            selected_index=workbench_side_index
                            set_selected_index=set_workbench_side_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar side".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sidebar-workbench-collapsible".to_string()
                            options=collapsible_options.clone()
                            selected_index=workbench_collapsible_index
                            set_selected_index=set_workbench_collapsible_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sidebar collapsible".to_string()
                        />
                        <Switch checked=workbench_default_open set_checked=set_workbench_default_open>
                            "default_open"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_show_trigger set_checked=set_workbench_show_trigger>
                            "show_trigger"
                        </Switch>
                        <Switch checked=workbench_enable_shortcut set_checked=set_workbench_enable_shortcut>
                            "enable_shortcut"
                        </Switch>
                        <Switch checked=workbench_custom_shortcut set_checked=set_workbench_custom_shortcut>
                            "custom shortcut_key"
                        </Switch>
                        <Switch checked=workbench_custom_trigger_label set_checked=set_workbench_custom_trigger_label>
                            "custom trigger_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_open_raw.update(|open| *open = !*open)
                        >
                            "Toggle open"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Sidebar
                        open=workbench_open
                        default_open=workbench_default_open.get()
                        on_open_change=on_workbench_open_change
                        side=workbench_side.get()
                        variant=workbench_variant.get()
                        collapsible=workbench_collapsible.get()
                        disabled=workbench_disabled.get()
                        show_trigger=workbench_show_trigger.get()
                        enable_shortcut=workbench_enable_shortcut.get()
                        shortcut_key=if workbench_custom_shortcut.get() {
                            "j".to_string()
                        } else {
                            "b".to_string()
                        }
                        trigger_label=if workbench_custom_trigger_label.get() {
                            "Toggle nav panel".to_string()
                        } else {
                            "Toggle sidebar".to_string()
                        }
                        aria_label="Project navigation sidebar".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-sidebar-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <div class="ui-sidebar__header">
                            <strong>"Workbench"</strong>
                        </div>
                        <div class="ui-sidebar__content">
                            <span>"Inbox"</span>
                            <span>"Projects"</span>
                            <span>"Reports"</span>
                        </div>
                    </Sidebar>
                    <span class="ui-muted">
                        "open: " {move || bool_word(workbench_open_raw.get())}
                        " · on_open_change count: " {move || workbench_open_change_count.get()}
                        " · last: " {move || bool_word(workbench_last_open.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Left / Right / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Sidebar
                        collapsible=SidebarCollapsible::Offcanvas
                        aria_label="Left default".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Default"</span></div>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Right
                        variant=SidebarVariant::Floating
                        collapsible=SidebarCollapsible::Icon
                        show_trigger=false
                        aria_label="Right floating".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Right/Floating"</span></div>
                    </Sidebar>
                    <Sidebar
                        side=SidebarSide::Left
                        variant=SidebarVariant::Inset
                        collapsible=SidebarCollapsible::None
                        disabled=true
                        enable_shortcut=false
                        trigger_label="Disabled".to_string()
                        aria_label="Disabled sidebar".to_string()
                    >
                        <div class="ui-sidebar__content"><span>"Disabled"</span></div>
                    </Sidebar>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
