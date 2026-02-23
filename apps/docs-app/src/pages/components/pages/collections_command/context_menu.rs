use super::*;

pub(crate) fn context_menu() -> AnyView {
    let workbench_items = vec![
        "Open".to_string(),
        "Rename".to_string(),
        "Delete".to_string(),
    ];
    let workbench_item_kinds = vec![
        MenuItemKind::Action,
        MenuItemKind::Action,
        MenuItemKind::Action,
    ];
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (last_action, set_last_action) = signal("None".to_string());
    let on_workbench_action =
        Callback::new(move |index: usize| set_last_action.set(index.to_string()));
    let (open_change_count, set_open_change_count) = signal(0_u32);
    let on_workbench_open_change_with_count = Callback::new(move |next: bool| {
        set_open_change_count.update(|count| *count += 1);
        on_workbench_open_change.run(next);
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_force_close_on_action, set_workbench_force_close_on_action) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_disable_middle, set_workbench_disable_middle) = signal(false);
    let (workbench_placement_key, set_workbench_placement_key) = signal("bottom-start".to_string());

    let hello_code = Signal::derive(move || {
        r#"<ContextMenu
  id_base="docs-context-menu".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Delete".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Right click or press Shift+F10"
</ContextMenu>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let placement = if workbench_placement_key.get() == "top-start" {
            "ui_headless::PopoverPlacement::TopStart"
        } else {
            "ui_headless::PopoverPlacement::BottomStart"
        };
        format!(
            "<ContextMenu\n  id_base=\"docs-context-menu-workbench\".to_string()\n  items=vec![\"Open\".to_string(), \"Rename\".to_string(), \"Delete\".to_string()]\n  on_action=on_action\n  is_disabled={}\n  disabled={}\n  disabled_indices={}\n  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]\n  is_close_on_action={}\n  close_on_action={}\n  placement={placement}\n  is_open=Signal::derive(move || open_raw.get())\n  open=Signal::derive(move || open_raw.get())\n  default_open={}\n  on_open_change=on_open_change\n  motion={}\n  lang={}\n  dir={}\n  aria_label=\"Workspace actions\".to_string()\n  class_name={}\n>\n  \"Right click to inspect\"\n</ContextMenu>",
            workbench_disabled.get(),
            workbench_disabled.get(),
            if workbench_disable_middle.get() {
                "vec![1]"
            } else {
                "vec![]"
            },
            workbench_force_close_on_action.get(),
            workbench_force_close_on_action.get(),
            workbench_open_raw.get(),
            if workbench_custom_motion.get() {
                "ui::ContextMenuMotion { popover: ui::PopoverMotion { initial_scale: 0.92, offset_y_px: 8.0, ..ui::PopoverMotion::default() } }"
            } else {
                "ui::ContextMenuMotion::default()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "ui_headless::A11yDirection::Rtl"
            } else {
                "ui_headless::A11yDirection::Ltr"
            },
            if workbench_custom_class.get() {
                "\"docs-context-menu-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_placement_key.get() == "top-start" {
            "TopStart"
        } else {
            "BottomStart"
        };
        format!(
            "ContextMenuWorkbenchConfig {{\n  id_base: \"docs-context-menu-workbench\",\n  items: [\"Open\", \"Rename\", \"Delete\"],\n  on_action: Some(\"Callback<usize>\"),\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {},\n  item_kinds: [Action, Action, Action],\n  is_close_on_action: Some({}),\n  close_on_action: {},\n  placement: {placement},\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: Some(\"Callback<bool>\"),\n  motion: {},\n  lang: {},\n  dir: {},\n  aria_label: Some(\"Workspace actions\"),\n  class_name: {},\n}}",
            workbench_disabled.get(),
            workbench_disabled.get(),
            if workbench_disable_middle.get() {
                "[1]"
            } else {
                "[]"
            },
            workbench_force_close_on_action.get(),
            workbench_force_close_on_action.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            if workbench_custom_motion.get() {
                "ContextMenuMotion::custom"
            } else {
                "ContextMenuMotion::default"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-context-menu-workbench\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ContextMenu id_base="ctx-default".to_string() items=vec!["Open".to_string(), "Rename".to_string(), "Delete".to_string()] on_action=Callback::new(move |_| {})>
  "Default"
</ContextMenu>
<ContextMenu id_base="ctx-keep-open".to_string() items=vec!["Copy".to_string(), "Paste".to_string(), "Inspect".to_string()] on_action=Callback::new(move |_| {}) close_on_action=false disabled_indices=vec![1]>
  "Keep open + disabled item"
</ContextMenu>
<ContextMenu id_base="ctx-disabled".to_string() items=vec!["Open".to_string()] on_action=Callback::new(move |_| {}) disabled=true>
  "Disabled trigger"
</ContextMenu>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ContextMenu"
            slug="context-menu"
            group="Collections"
            description="Context trigger menu with controlled open state and action callbacks."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ContextMenu
                    id_base="docs-context-menu-hello".to_string()
                    items=vec![
                        "Open".to_string(),
                        "Rename".to_string(),
                        "Delete".to_string(),
                    ]
                    on_action=Callback::new(|_: usize| {})
                >
                    "Right click or press Shift+F10"
                </ContextMenu>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full ContextMenu API and shows open/action callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="context-menu-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Placement"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_placement_key.set(event_target_value(&ev))>
                                <option value="bottom-start" selected=move || workbench_placement_key.get() == "bottom-start">"BottomStart"</option>
                                <option value="top-start" selected=move || workbench_placement_key.get() == "top-start">"TopStart"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_disable_middle set_checked=set_workbench_disable_middle>"Disabled middle item"</Switch>
                        <Switch checked=workbench_force_close_on_action set_checked=set_workbench_force_close_on_action>"Close on action"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="context-menu-workbench-preview">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(true)>"Open"</button>
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(false)>"Close"</button>
                    </div>
                    <ContextMenu
                        id_base="docs-context-menu-workbench".to_string()
                        items=workbench_items
                        on_action=on_workbench_action
                        is_disabled=workbench_disabled.get()
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_middle.get() {
                            vec![1]
                        } else {
                            vec![]
                        }
                        item_kinds=workbench_item_kinds
                        is_close_on_action=workbench_force_close_on_action.get()
                        close_on_action=workbench_force_close_on_action.get()
                        placement=if workbench_placement_key.get() == "top-start" {
                            ui_headless::PopoverPlacement::TopStart
                        } else {
                            ui_headless::PopoverPlacement::BottomStart
                        }
                        is_open=workbench_open
                        open=workbench_open
                        default_open=workbench_open_raw.get()
                        on_open_change=on_workbench_open_change_with_count
                        motion=if workbench_custom_motion.get() {
                            ui::ContextMenuMotion {
                                popover: ui::PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 8.0,
                                    ..ui::PopoverMotion::default()
                                },
                            }
                        } else {
                            ui::ContextMenuMotion::default()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        aria_label="Workspace actions".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-context-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        "Right click or press Shift+F10"
                    </ContextMenu>
                    <span class="ui-muted">
                        "open=" {move || workbench_open_raw.get()}
                        " · open_change_count=" {move || open_change_count.get()}
                        " · last_action=" {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <ContextMenu
                        id_base="docs-context-menu-matrix-default".to_string()
                        items=vec![
                            "Open".to_string(),
                            "Rename".to_string(),
                            "Delete".to_string(),
                        ]
                        on_action=Callback::new(|_: usize| {})
                    >
                        "Default"
                    </ContextMenu>
                    <ContextMenu
                        id_base="docs-context-menu-matrix-keep-open".to_string()
                        items=vec![
                            "Copy".to_string(),
                            "Paste".to_string(),
                            "Inspect".to_string(),
                        ]
                        on_action=Callback::new(|_: usize| {})
                        close_on_action=false
                        disabled_indices=vec![1]
                    >
                        "Keep open + disabled"
                    </ContextMenu>
                    <ContextMenu
                        id_base="docs-context-menu-matrix-disabled".to_string()
                        items=vec!["Open".to_string()]
                        on_action=Callback::new(|_: usize| {})
                        disabled=true
                    >
                        "Disabled trigger"
                    </ContextMenu>
                </div>
            </Playground>

        </ComponentPage>
    }
    .into_any()
}
