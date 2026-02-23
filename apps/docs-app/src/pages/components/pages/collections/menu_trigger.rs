use super::*;

pub(crate) fn menu_trigger() -> AnyView {
    let default_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let disabled_items = vec!["Copy".to_string(), "Move".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open menu"
</MenuTrigger>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<MenuTrigger
  id_base="trigger-controlled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  close_on_action=false
  disabled_indices=vec![1]
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
>
  "Controlled"
</MenuTrigger>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</MenuTrigger>
<MenuTrigger
  id_base="trigger-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</MenuTrigger>"#
            .to_string()
    });
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last, set_workbench_last) = signal(None::<usize>);
    let on_workbench_action =
        Callback::new(move |index: usize| set_workbench_last.set(Some(index)));
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_top_end, set_workbench_top_end) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::menu_trigger::MenuTriggerMotion::default();
        if workbench_custom_motion.get() {
            motion.popover.offset_y_px = 12.0;
        }
        motion
    });
    let workbench_code = Signal::derive(move || {
        let close_on_action = workbench_close_on_action.get();
        let disabled = workbench_disabled.get();
        let disable_second = workbench_disable_second.get();
        let top_end = workbench_top_end.get();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec![
            "let (open, set_open) = signal(false);".to_string(),
            "<MenuTrigger".to_string(),
            "  id_base=\"docs-menu-trigger-workbench\".into()".to_string(),
            "  items=vec![\"Profile\".into(), \"Settings\".into(), \"Archive\".into()]".to_string(),
            "  on_action=Callback::new(move |_: usize| {})".to_string(),
            format!("  is_disabled={}", workbench_disabled.get()),
            "  open=Signal::derive(move || open.get())".to_string(),
            "  is_open=Signal::derive(move || open.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open.set(next))".to_string(),
            format!("  is_close_on_action={}", close_on_action),
            "  motion=MenuTriggerMotion::default()".to_string(),
        ];
        if !close_on_action {
            snippet.push("  close_on_action=false".to_string());
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if disable_second {
            snippet.push("  disabled_indices=vec![1]".to_string());
        }
        if top_end {
            snippet.push("  placement=PopoverPlacement::TopEnd".to_string());
        }
        if custom_label {
            snippet.push("  aria_label=\"Workbench menu trigger\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-menu-trigger-workbench\".into()".to_string());
        }
        snippet.extend([
            ">".to_string(),
            "  \"Workbench\"".to_string(),
            "</MenuTrigger>".to_string(),
        ]);
        snippet.join("\n")
    });
    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/trigger/styles.rs */\n{}",
            ui::menu_trigger::styles::CSS
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_top_end.get() {
            PopoverPlacement::TopEnd
        } else {
            PopoverPlacement::BottomStart
        };
        let disabled_indices = if workbench_disable_second.get() {
            vec![1]
        } else {
            Vec::new()
        };
        let motion = workbench_motion.get();

        format!(
            "MenuTriggerActualConfig {{\n  id_base: \"docs-menu-trigger-workbench\",\n  items: [\"Profile\", \"Settings\", \"Archive\"],\n  on_action: \"set_workbench_last\",\n  is_disabled: {},\n  disabled: {},\n  disabled_indices: {:?},\n  item_kinds: [Action, Action, Action],\n  is_close_on_action: {},\n  close_on_action: {},\n  placement: PopoverPlacement::{:?},\n  is_open: {},\n  open: {},\n  default_open: false,\n  on_open_change: \"set_workbench_open_raw\",\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  custom_aria_label: {},\n  custom_class_name: {},\n  last_action: {},\n}}",
            workbench_disabled.get(),
            workbench_disabled.get(),
            disabled_indices,
            workbench_close_on_action.get(),
            workbench_close_on_action.get(),
            placement,
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            motion,
            if workbench_custom_label.get() {
                Some("Workbench menu trigger")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-menu-trigger-workbench")
            } else {
                None
            },
            workbench_custom_label.get(),
            workbench_custom_class.get(),
            workbench_last
                .get()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    });

    view! {
        <ComponentPage
            title="MenuTrigger"
            slug="menu-trigger"
            group="Collections"
            description="Button-triggered menu surface with baseline state attrs and controlled/uncontrolled close-strategy semantics."
        >
            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger".to_string()
                        items=default_items.clone()
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open menu"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui/src/menu/trigger/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区用于 current 与 baseline 对比；Config/Code/CSS Test 区用于行为和样式契约验证。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-trigger-config-controls">
                        <button
                            type="button"
                            on:click=move |_| set_workbench_close_on_action.update(|value| *value = !*value)
                        >
                            "Toggle close_on_action"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_disabled.update(|value| *value = !*value)
                        >
                            "Toggle disabled"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_disable_second.update(|value| *value = !*value)
                        >
                            "Toggle disabled item #1"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_top_end.update(|value| *value = !*value)
                        >
                            "Toggle placement (bottom-start/top-end)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_label.update(|value| *value = !*value)
                        >
                            "Toggle custom aria label"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_class.update(|value| *value = !*value)
                        >
                            "Toggle custom class"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_custom_motion.update(|value| *value = !*value)
                        >
                            "Toggle custom motion"
                        </button>
                        <p class="ui-muted" data-slot="menu-trigger-config-summary">
                            {move || {
                                format!(
                                    "config: open={} close_on_action={} disabled={} placement={} custom_label={} custom_class={}",
                                    workbench_open_raw.get(),
                                    workbench_close_on_action.get(),
                                    workbench_disabled.get(),
                                    if workbench_top_end.get() {
                                        "top-end"
                                    } else {
                                        "bottom-start"
                                    },
                                    workbench_custom_label.get(),
                                    workbench_custom_class.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let placement = if workbench_top_end.get() {
                        PopoverPlacement::TopEnd
                    } else {
                        PopoverPlacement::BottomStart
                    };
                    let disabled_indices = if workbench_disable_second.get() {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let aria_label = if workbench_custom_label.get() {
                        "Workbench menu trigger".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-menu-trigger-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="menu-trigger-workbench-display">
                            <span class="ui-muted">
                                "display: current config vs baseline"
                            </span>
                            <div class="docs-row">
                                <div class="docs-stack">
                                    <span class="ui-muted">"Current"</span>
                                    <MenuTrigger
                                        id_base="docs-menu-trigger-workbench".to_string()
                                        items=vec![
                                            "Profile".to_string(),
                                            "Settings".to_string(),
                                            "Archive".to_string(),
                                        ]
                                        on_action=on_workbench_action
                                        is_disabled=workbench_disabled.get()
                                        close_on_action=workbench_close_on_action.get()
                                        is_close_on_action=workbench_close_on_action.get()
                                        disabled=workbench_disabled.get()
                                        disabled_indices=disabled_indices
                                        is_open=workbench_open
                                        open=workbench_open
                                        default_open=false
                                        on_open_change=on_workbench_open_change
                                        placement=placement
                                        motion=workbench_motion.get()
                                        aria_label=aria_label
                                        class_name=class_name
                                        item_kinds=vec![
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                        ]
                                    >
                                        "Workbench"
                                    </MenuTrigger>
                                    <span class="ui-muted">
                                        "open: "
                                        {workbench_open_raw.get()}
                                        " · last: "
                                        {workbench_last
                                            .get()
                                            .map(|value| value.to_string())
                                            .unwrap_or_else(|| "None".to_string())}
                                    </span>
                                </div>

                                <div class="docs-stack">
                                    <span class="ui-muted">"Baseline"</span>
                                    <MenuTrigger
                                        id_base="docs-menu-trigger-workbench-baseline".to_string()
                                        items=vec![
                                            "Profile".to_string(),
                                            "Settings".to_string(),
                                            "Archive".to_string(),
                                        ]
                                        on_action=on_action
                                        item_kinds=vec![
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                            MenuItemKind::Action,
                                        ]
                                    >
                                        "Baseline"
                                    </MenuTrigger>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Default / Controlled / Disabled)" code_signal=controlled_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <span class="ui-muted">"Default"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-default".to_string()
                            items=vec![
                                "Profile".to_string(),
                                "Settings".to_string(),
                                "Log out".to_string(),
                            ]
                            on_action=on_action
                            item_kinds=vec![
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                            ]
                        >
                            "Default"
                        </MenuTrigger>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"Controlled + keep open"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-controlled".to_string()
                            items=vec![
                                "Rename".to_string(),
                                "Duplicate".to_string(),
                                "Archive".to_string(),
                            ]
                            on_action=on_action
                            is_open=controlled_open
                            on_open_change=on_open_change
                            is_close_on_action=false
                            close_on_action=false
                            motion=ui::menu_trigger::MenuTriggerMotion::default()
                            item_kinds=vec![
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                                MenuItemKind::Action,
                            ]
                        >
                            "Controlled"
                        </MenuTrigger>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"Disabled trigger"</span>
                        <MenuTrigger
                            id_base="docs-menu-trigger-matrix-disabled".to_string()
                            items=vec!["Copy".to_string(), "Move".to_string()]
                            on_action=on_action
                            is_disabled=true
                            disabled=true
                            default_open=false
                            item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                        >
                            "Disabled"
                        </MenuTrigger>
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <MenuTrigger
                        id_base="docs-menu-trigger-controlled".to_string()
                        items=controlled_items.clone()
                        on_action=on_action
                        close_on_action=false
                        disabled_indices=vec![1]
                        open=controlled_open
                        on_open_change=on_open_change
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger-disabled".to_string()
                        items=disabled_items.clone()
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    >
                        "Disabled"
                    </MenuTrigger>

                    <MenuTrigger
                        id_base="docs-menu-trigger-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </MenuTrigger>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
