use super::*;

pub(crate) fn action_menu() -> AnyView {
    let default_items = vec![
        ActionMenuItemSpec::action("Profile"),
        ActionMenuItemSpec::action("Settings"),
        ActionMenuItemSpec::action("Log out"),
    ];
    let controlled_items = vec![
        ActionMenuItemSpec::action("Rename"),
        ActionMenuItemSpec::action("Duplicate").with_disabled(true),
        ActionMenuItemSpec::action("Archive"),
    ];
    let disabled_items = vec![
        ActionMenuItemSpec::action("Copy"),
        ActionMenuItemSpec::action("Move"),
    ];
    let empty_items: Vec<ActionMenuItemSpec> = Vec::new();
    let marker_items = vec![
        ActionMenuItemSpec::action("Open dashboard"),
        ActionMenuItemSpec::action("Duplicate project"),
        ActionMenuItemSpec::action("Archive workspace").with_disabled(true),
    ];

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (marker_open_raw, set_marker_open_raw) = signal(true);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change = Callback::new(move |next: bool| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<usize>);
    let on_marker_action =
        Callback::new(move |index: usize| set_last_marker_action.set(Some(index)));
    let on_hello_action = Callback::new(|_: usize| {});

    let hello_code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="action-menu-hello".to_string()
  item_specs=vec![ActionMenuItemSpec::action("Profile")]
  on_action=Callback::new(|_| {})
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="demo".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="action-controlled".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_close_on_action=false
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ActionMenu
  id_base="docs-action-menu-markers".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace").with_disabled(true),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_close_on_action=false
  open=open
  default_open=true
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  aria_label="Workspace actions".to_string()
  class_name="docs-action-menu-custom".to_string()
  motion=ui::ActionMenuMotion {
    popover: ui::PopoverMotion {
      initial_scale: 0.93,
      offset_y_px: 8.0,
      ..ui::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<ActionMenu
  id_base="action-disabled".to_string()
  item_specs=vec![
    ActionMenuItemSpec::action("Open dashboard"),
    ActionMenuItemSpec::action("Duplicate project"),
    ActionMenuItemSpec::action("Archive workspace"),
  ]
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
  is_disabled=true
/>
<ActionMenu
  id_base="action-empty".to_string()
  item_specs=Vec::<ActionMenuItemSpec>::new()
  on_action=Callback::new(move |index: usize| {
    logging::log!("action index: {}", index);
  })
/>"#
        .to_string()
    });

    let marker_motion = ui::ActionMenuMotion {
        popover: ui::PopoverMotion {
            initial_scale: 0.93,
            offset_y_px: 8.0,
            ..ui::PopoverMotion::default()
        },
    };
    let workbench_action_mode_options = vec!["close".to_string(), "keep-open".to_string()];
    let workbench_placement_options = vec![
        "bottom-start".to_string(),
        "bottom-end".to_string(),
        "top-start".to_string(),
    ];
    let workbench_size_options = vec![
        "xs".to_string(),
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
        "xl".to_string(),
    ];
    let (workbench_action_mode_index, set_workbench_action_mode_index) = signal(Some(0_usize));
    let (workbench_placement_index, set_workbench_placement_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_is_quiet, set_workbench_is_quiet) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let workbench_on_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let workbench_on_action =
        Callback::new(move |index: usize| set_workbench_last_action.set(Some(index)));

    let workbench_action_mode = Signal::derive(move || {
        if workbench_action_mode_index.get().unwrap_or(0) == 1 {
            ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
        } else {
            ui::action_menu::ActionMenuActionMode::CloseOnAction
        }
    });
    let workbench_placement =
        Signal::derive(move || match workbench_placement_index.get().unwrap_or(0) {
            1 => PopoverPlacement::BottomEnd,
            2 => PopoverPlacement::TopStart,
            _ => PopoverPlacement::BottomStart,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::Sm,
        3 => ActionButtonSize::Lg,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let workbench_item_specs = Signal::derive(move || {
        if workbench_disable_second.get() {
            vec![
                ActionMenuItemSpec::action("Open dashboard"),
                ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
                ActionMenuItemSpec::action("Archive workspace"),
            ]
        } else {
            vec![
                ActionMenuItemSpec::action("Open dashboard"),
                ActionMenuItemSpec::action("Duplicate project"),
                ActionMenuItemSpec::action("Archive workspace"),
            ]
        }
    });
    let workbench_items = Signal::derive(move || {
        vec![
            "Open dashboard".to_string(),
            "Duplicate project".to_string(),
            "Archive workspace".to_string(),
        ]
    });
    let workbench_disabled_indices = Signal::derive(move || {
        if workbench_disable_second.get() {
            vec![1_usize]
        } else {
            vec![]
        }
    });
    let workbench_item_kinds = Signal::derive(move || {
        vec![
            ui::MenuItemKind::Action,
            ui::MenuItemKind::Action,
            ui::MenuItemKind::Action,
        ]
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::ActionMenuMotion {
                popover: ui::PopoverMotion {
                    initial_scale: 0.94,
                    offset_y_px: 10.0,
                    ..ui::PopoverMotion::default()
                },
            }
        } else {
            ui::ActionMenuMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open_sig: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<ActionMenu".to_string(),
            "  id_base=\"docs-action-menu-workbench\".to_string()".to_string(),
            "  items=vec![\"Open dashboard\".into(), \"Duplicate project\".into(), \"Archive workspace\".into()]".to_string(),
            "  on_action=Callback::new(move |index: usize| { logging::log!(\"action={}\", index); })".to_string(),
            "  item_specs=vec![".to_string(),
            "    ActionMenuItemSpec::action(\"Open dashboard\"),".to_string(),
            "    ActionMenuItemSpec::action(\"Duplicate project\"),".to_string(),
            "    ActionMenuItemSpec::action(\"Archive workspace\"),".to_string(),
            "  ]".to_string(),
            "  disabled_state=ui::ActionMenuDisabledState::Enabled".to_string(),
            format!("  is_disabled={}", workbench_is_disabled.get()),
            format!("  disabled={}", workbench_is_disabled.get()),
            format!(
                "  disabled_indices={}",
                if workbench_disable_second.get() {
                    "vec![1]"
                } else {
                    "vec![]"
                }
            ),
            "  item_kinds=vec![ui::MenuItemKind::Action, ui::MenuItemKind::Action, ui::MenuItemKind::Action]".to_string(),
            format!(
                "  action_mode=ui::action_menu::ActionMenuActionMode::{}",
                if workbench_action_mode.get()
                    == ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
                {
                    "KeepOpenOnAction"
                } else {
                    "CloseOnAction"
                }
            ),
            format!(
                "  is_close_on_action={}",
                workbench_action_mode.get().is_close_on_action()
            ),
            format!(
                "  close_on_action={}",
                workbench_action_mode.get().is_close_on_action()
            ),
            format!("  placement=ui::PopoverPlacement::{:?}", workbench_placement.get()),
            "  is_open=open_sig".to_string(),
            "  open=open_sig".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            format!("  size=ActionButtonSize::{:?}", workbench_size.get()),
            format!("  is_quiet={}", workbench_is_quiet.get()),
            "  aria_label=\"Workspace actions\".to_string()".to_string(),
            "  lang=\"en\".to_string()".to_string(),
            "  dir=ui::A11yDirection::Ltr".to_string(),
            if workbench_custom_motion.get() {
                "  motion=ui::ActionMenuMotion { popover: ui::PopoverMotion { initial_scale: 0.94, offset_y_px: 10.0, ..ui::PopoverMotion::default() } }".to_string()
            } else {
                "  motion=ui::ActionMenuMotion::default()".to_string()
            },
            if workbench_custom_class.get() {
                "  class_name=\"docs-action-menu-workbench\".to_string()".to_string()
            } else {
                "  class_name=\"\".to_string()".to_string()
            },
            "/>".to_string(),
        ];
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "ActionMenuWorkbenchConfig {{\n  id_base: \"docs-action-menu-workbench\",\n  items: [\"Open dashboard\", \"Duplicate project\", \"Archive workspace\"],\n  on_action: Some(\"Callback<usize>\"),\n  item_specs: [\"action\", \"action\", \"action\"],\n  disabled_state: Some(\"Enabled\"),\n  is_disabled: Some({}),\n  disabled: Some({}),\n  disabled_indices: {},\n  item_kinds: [\"action\", \"action\", \"action\"],\n  action_mode: Some(\"{}\"),\n  is_close_on_action: Some({}),\n  close_on_action: Some({}),\n  placement: \"{:?}\",\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: Some(\"Callback<bool>\"),\n  size: \"{:?}\",\n  is_quiet: {},\n  aria_label: Some(\"Workspace actions\"),\n  lang: {},\n  dir: {},\n  motion: {},\n  class_name: {},\n  last_action: {:?},\n}}",
            workbench_is_disabled.get(),
            workbench_is_disabled.get(),
            if workbench_disable_second.get() {
                "vec![1]"
            } else {
                "vec![]"
            },
            if workbench_action_mode.get()
                == ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
            {
                "KeepOpenOnAction"
            } else {
                "CloseOnAction"
            },
            workbench_action_mode.get().is_close_on_action(),
            workbench_action_mode.get().is_close_on_action(),
            workbench_placement.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_size.get(),
            workbench_is_quiet.get(),
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(\"rtl\")"
            } else {
                "Some(\"ltr\")"
            },
            if workbench_custom_motion.get() {
                "ActionMenuMotion::custom"
            } else {
                "ActionMenuMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-action-menu-workbench\")"
            } else {
                "None"
            },
            workbench_last_action.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ActionMenu id_base="m1".to_string() item_specs=vec![ActionMenuItemSpec::action("A")] on_action=Callback::new(|_| {}) />
<ActionMenu id_base="m2".to_string() item_specs=vec![ActionMenuItemSpec::action("A"), ActionMenuItemSpec::action("B").with_disabled(true)] on_action=Callback::new(|_| {}) is_disabled=Some(false) disabled_indices=vec![1] />
<ActionMenu id_base="m3".to_string() item_specs=vec![ActionMenuItemSpec::action("A")] on_action=Callback::new(|_| {}) action_mode=ui::action_menu::ActionMenuActionMode::KeepOpenOnAction />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionMenu"
            slug="action-menu"
            group="Actions"
            description="ActionButton-triggered menu surface with baseline state/source data attrs and baseline-level popover spring motion (controlled/uncontrolled + close strategy)."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-hello".to_string()
                        item_specs=vec![ActionMenuItemSpec::action("Profile")]
                        on_action=on_hello_action
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-menu-workbench-controls">
                        <div class="docs-search__label">"Action mode"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-mode".to_string()
                            options=workbench_action_mode_options.clone()
                            selected_index=workbench_action_mode_index
                            set_selected_index=set_workbench_action_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu action mode".to_string()
                        />
                        <div class="docs-search__label">"Placement"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-placement".to_string()
                            options=workbench_placement_options.clone()
                            selected_index=workbench_placement_index
                            set_selected_index=set_workbench_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu placement".to_string()
                        />
                        <div class="docs-search__label">"Trigger size"</div>
                        <SegmentedControl
                            id_base="docs-action-menu-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionMenu trigger size".to_string()
                        />
                        <Switch checked=workbench_is_quiet set_checked=set_workbench_is_quiet>"Quiet trigger"</Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>"Disable menu"</Switch>
                        <Switch checked=workbench_disable_second set_checked=set_workbench_disable_second>"Disable second item"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="action-menu-workbench-preview">
                    <ActionMenu
                        id_base="docs-action-menu-workbench".to_string()
                        items=workbench_items.get()
                        on_action=workbench_on_action
                        item_specs=workbench_item_specs.get()
                        disabled_state=if workbench_is_disabled.get() {
                            ui::action_menu::ActionMenuDisabledState::Disabled
                        } else {
                            ui::action_menu::ActionMenuDisabledState::Enabled
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_is_disabled.get()
                        disabled_indices=workbench_disabled_indices.get()
                        item_kinds=workbench_item_kinds.get()
                        action_mode=workbench_action_mode.get()
                        is_close_on_action=workbench_action_mode.get().is_close_on_action()
                        close_on_action=workbench_action_mode.get().is_close_on_action()
                        placement=workbench_placement.get()
                        is_open=workbench_open
                        open=workbench_open
                        default_open=false
                        on_open_change=workbench_on_open_change
                        size=workbench_size.get()
                        is_quiet=workbench_is_quiet.get()
                        aria_label="Workspace actions".to_string()
                        lang=if workbench_rtl.get() { "ar".to_string() } else { "en".to_string() }
                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                        motion=workbench_motion.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-action-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get().to_string()}
                        " · last action: " {move || workbench_last_action.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu".to_string()
                        item_specs=default_items
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <ActionMenu
                        id_base="docs-action-menu-controlled".to_string()
                        item_specs=controlled_items
                        on_action=on_action
                        is_close_on_action=false
                        open=controlled_open
                        on_open_change=on_open_change
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_open_raw.set(true)>
                            "Open"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-item-kinds-source / data-open-source / data-open-change-source / data-motion-source in DevTools."
                    </div>
                    <ActionMenu
                        id_base="docs-action-menu-markers".to_string()
                        item_specs=marker_items
                        on_action=on_marker_action
                        is_close_on_action=false
                        open=marker_open
                        default_open=true
                        on_open_change=on_marker_open_change
                        aria_label="Workspace actions".to_string()
                        class_name="docs-action-menu-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || marker_open_raw.get()}
                        " · last action: "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-disabled".to_string()
                        item_specs=disabled_items
                        on_action=on_action
                        is_disabled=true
                    />

                    <ActionMenu
                        id_base="docs-action-menu-empty".to_string()
                        item_specs=empty_items
                        on_action=on_action
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Disabled Item / Keep Open)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <ActionMenu
                        id_base="docs-action-menu-matrix-default".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Duplicate project"),
                        ]
                        on_action=on_action
                    />
                    <ActionMenu
                        id_base="docs-action-menu-matrix-disabled-item".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Duplicate project").with_disabled(true),
                        ]
                        disabled_indices=vec![1]
                        on_action=on_action
                    />
                    <ActionMenu
                        id_base="docs-action-menu-matrix-keep-open".to_string()
                        item_specs=vec![
                            ActionMenuItemSpec::action("Open dashboard"),
                            ActionMenuItemSpec::action("Archive workspace"),
                        ]
                        action_mode=ui::action_menu::ActionMenuActionMode::KeepOpenOnAction
                        on_action=on_action
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
