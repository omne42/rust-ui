use super::*;

pub(crate) fn menubar() -> AnyView {
    let default_menus = vec![
        MenubarMenu::new(
            "file",
            "File",
            vec![
                "New Tab".to_string(),
                "New Window".to_string(),
                "Save".to_string(),
            ],
        )
        .disabled_indices(vec![2])
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "edit",
            "Edit",
            vec!["Undo".to_string(), "Redo".to_string(), "Find".to_string()],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "view",
            "View",
            vec![
                "Zoom In".to_string(),
                "Zoom Out".to_string(),
                "Actual Size".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
    ];

    let controlled_menus = vec![
        MenubarMenu::new(
            "window",
            "Window",
            vec![
                "Minimize".to_string(),
                "Zoom".to_string(),
                "Bring All to Front".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "help",
            "Help",
            vec!["Search".to_string(), "Documentation".to_string()],
        )
        .item_kinds(vec![MenuItemKind::Action, MenuItemKind::Action]),
        MenubarMenu::new("tools", "Tools", vec!["Inspector".to_string()]).disabled(true),
    ];

    let marker_menus = vec![
        MenubarMenu::new(
            "workspace",
            "Workspace",
            vec![
                "Open File".to_string(),
                "Open Folder".to_string(),
                "Save All".to_string(),
            ],
        )
        .disabled_indices(vec![2])
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "run",
            "Run",
            vec![
                "Run".to_string(),
                "Debug".to_string(),
                "Profile".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new("help", "Help", vec!["Docs".to_string()]),
    ];

    let (last_action, set_last_action) = signal(None::<(usize, usize)>);
    let on_action = Callback::new(move |action: (usize, usize)| set_last_action.set(Some(action)));

    let (last_controlled_action, set_last_controlled_action) = signal(None::<(usize, usize)>);
    let on_controlled_action =
        Callback::new(move |action: (usize, usize)| set_last_controlled_action.set(Some(action)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(None::<usize>);
    let controlled_open: Signal<Option<usize>> = Signal::derive(move || controlled_open_raw.get());
    let on_open_index_change = Callback::new(move |next: Option<usize>| {
        set_controlled_open_raw.set(next);
    });

    let (marker_open_raw, set_marker_open_raw) = signal(Some(0usize));
    let marker_open: Signal<Option<usize>> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change =
        Callback::new(move |next: Option<usize>| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<(usize, usize)>);
    let on_marker_action =
        Callback::new(move |action: (usize, usize)| set_last_marker_action.set(Some(action)));

    let menu_set_options = vec![
        "app".to_string(),
        "workspace".to_string(),
        "compact".to_string(),
    ];
    let (menu_set_index, set_menu_set_index) = signal(Some(0_usize));
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_flip_placement, set_workbench_flip_placement) = signal(false);
    let (workbench_default_open, set_workbench_default_open) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_menus: Signal<Vec<MenubarMenu>> =
        Signal::derive(move || match menu_set_index.get().unwrap_or(0) {
            1 => vec![
                MenubarMenu::new(
                    "workspace",
                    "Workspace",
                    vec![
                        "Open File".to_string(),
                        "Open Folder".to_string(),
                        "Save All".to_string(),
                    ],
                )
                .disabled_indices(vec![2])
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "run",
                    "Run",
                    vec![
                        "Run".to_string(),
                        "Debug".to_string(),
                        "Profile".to_string(),
                    ],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
            ],
            2 => vec![MenubarMenu::new(
                "quick",
                "Quick",
                vec!["Command Palette".to_string(), "Recent".to_string()],
            )],
            _ => vec![
                MenubarMenu::new(
                    "file",
                    "File",
                    vec![
                        "New Tab".to_string(),
                        "New Window".to_string(),
                        "Save".to_string(),
                    ],
                )
                .disabled_indices(vec![2])
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "edit",
                    "Edit",
                    vec!["Undo".to_string(), "Redo".to_string(), "Find".to_string()],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "view",
                    "View",
                    vec![
                        "Zoom In".to_string(),
                        "Zoom Out".to_string(),
                        "Actual Size".to_string(),
                    ],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
            ],
        });
    let (workbench_last_action, set_workbench_last_action) = signal(None::<(usize, usize)>);
    let on_workbench_action =
        Callback::new(move |action: (usize, usize)| set_workbench_last_action.set(Some(action)));
    let (workbench_open_raw, set_workbench_open_raw) = signal(None::<usize>);
    let on_workbench_open_change =
        Callback::new(move |next: Option<usize>| set_workbench_open_raw.set(next));

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<Menubar".to_string()];

        lines.push("  id_base=\"docs-menubar-workbench\".into()".to_string());
        lines.push("  menus=menus".to_string());
        lines.push("  on_action=on_action".to_string());
        lines.push(format!(
            "  close_on_action={}",
            workbench_close_on_action.get()
        ));
        lines.push(format!(
            "  is_close_on_action={}",
            workbench_close_on_action.get()
        ));

        if workbench_flip_placement.get() {
            lines.push("  placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()".to_string());
        }
        if workbench_default_open.get() {
            lines.push("  default_open_index=0".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-menubar-custom\".into()".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push("  motion=ui::MenubarMotion {".to_string());
            lines.push("    popover: ui::PopoverMotion {".to_string());
            lines.push("      initial_scale: 0.94,".to_string());
            lines.push("      offset_y_px: 10.0,".to_string());
            lines.push("      ..ui::PopoverMotion::default()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }
        lines.push(
            "  on_open_index_change=Callback::new(move |next| set_open.set(next))".to_string(),
        );
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/menubar/styles.rs */\n{}",
            ui::menubar::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let menus = workbench_menus.get();
        let mut class_tokens = vec!["ui-menubar".to_string()];
        if workbench_custom_class.get() {
            class_tokens.push("docs-menubar-custom".to_string());
        }
        format!(
            "MenubarActualConfig {{\n  id_base: {:?},\n  menus: {:?},\n  on_open_index_change: {:?},\n  on_action: \"last={:?}\",\n  menu_set: \"{}\",\n  menu_count: {},\n  close_on_action: {},\n  is_close_on_action: {:?},\n  placement: \"{}\",\n  default_open_index: {},\n  custom_motion: {},\n  custom_class_name: {},\n  class_name: {:?},\n  class: \"{}\",\n}}",
            "docs-menubar-workbench",
            menus
                .iter()
                .map(|menu| menu.id.as_str())
                .collect::<Vec<_>>(),
            "handler",
            workbench_last_action.get(),
            match menu_set_index.get().unwrap_or(0) {
                1 => "workspace",
                2 => "compact",
                _ => "app",
            },
            menus.len(),
            workbench_close_on_action.get(),
            Some(workbench_close_on_action.get()),
            if workbench_flip_placement.get() {
                "bottom-start-flipped"
            } else {
                "bottom-start"
            },
            if workbench_default_open.get() {
                "Some(0)"
            } else {
                "None"
            },
            workbench_custom_motion.get(),
            workbench_custom_class.get(),
            if workbench_custom_class.get() {
                "docs-menubar-custom"
            } else {
                ""
            },
            class_tokens.join(" ")
        )
    });

    let code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal(None::<(usize, usize)>);

<Menubar
  id_base="docs-menubar".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |action: (usize, usize)| set_last_action.set(Some(action)))
/>
<span class="ui-muted">
  "last action (menu:item): "
  {move || last_action.get().map(|(m, i)| format!("{m}:{i}")).unwrap_or_else(|| "None".to_string())}
</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open_menu, set_open_menu) = signal(None::<usize>);

<Menubar
  id_base="docs-menubar-controlled".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |_: (usize, usize)| {})
  close_on_action=false
  open_index=Signal::derive(move || open_menu.get())
  on_open_index_change=Callback::new(move |next| set_open_menu.set(next))
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(Some(0usize));

<Menubar
  id_base="docs-menubar-markers".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |_: (usize, usize)| {})
  close_on_action=false
  placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
  open_index=Signal::derive(move || open_raw.get())
  default_open_index=1
  on_open_index_change=Callback::new(move |next| set_open_raw.set(next))
  class_name="docs-menubar-custom".to_string()
  motion=ui::MenubarMotion {
    popover: ui::PopoverMotion {
      initial_scale: 0.94,
      offset_y_px: 10.0,
      ..ui::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let marker_motion = ui::MenubarMotion {
        popover: ui::PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 10.0,
            ..ui::PopoverMotion::default()
        },
    };
    let default_menus_for_hello = default_menus.clone();
    let default_menus_for_default = default_menus.clone();
    let controlled_menus_for_matrix = controlled_menus.clone();
    let controlled_menus_for_controlled = controlled_menus.clone();

    view! {
        <ComponentPage
            title="Menubar"
            slug="menubar"
            group="Collections"
            description="baseline-compatible persistent menubar with horizontal trigger navigation, baseline-style state/source attrs, and baseline-level spring popover motion reuse."
        >
            <Playground title="Hello World (Default API)" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-hello".to_string()
                        menus=default_menus_for_hello
                        on_action=on_action
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Menubar."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/menu/menubar/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Menu set"</div>
                        <SegmentedControl
                            id_base="docs-menubar-workbench-set".to_string()
                            options=menu_set_options.clone()
                            selected_index=menu_set_index
                            set_selected_index=set_menu_set_index
                            size=SegmentedControlSize::Sm
                            aria_label="Menubar menu set".to_string()
                        />
                        <Switch
                            checked=workbench_close_on_action
                            set_checked=set_workbench_close_on_action
                        >
                            "Close on action"
                        </Switch>
                        <Switch
                            checked=workbench_flip_placement
                            set_checked=set_workbench_flip_placement
                        >
                            "Flip placement"
                        </Switch>
                        <Switch
                            checked=workbench_default_open
                            set_checked=set_workbench_default_open
                        >
                            "Default open menu"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class
                            set_checked=set_workbench_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=workbench_custom_motion
                            set_checked=set_workbench_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            {move || {
                                let placement = if workbench_flip_placement.get() {
                                    ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
                                } else {
                                    ui::menubar::DEFAULT_PLACEMENT
                                };
                                let class_name = if workbench_custom_class.get() {
                                    "docs-menubar-custom".to_string()
                                } else {
                                    String::new()
                                };
                                let motion = if workbench_custom_motion.get() {
                                    ui::MenubarMotion {
                                        popover: ui::PopoverMotion {
                                            initial_scale: 0.94,
                                            offset_y_px: 10.0,
                                            ..ui::PopoverMotion::default()
                                        },
                                    }
                                } else {
                                    ui::MenubarMotion::default()
                                };

                                if workbench_default_open.get() {
                                    view! {
                                        <Menubar
                                            id_base="docs-menubar-workbench".to_string()
                                            menus=workbench_menus.get()
                                            on_action=on_workbench_action
                                            close_on_action=workbench_close_on_action.get()
                                            is_close_on_action=workbench_close_on_action.get()
                                            placement=placement
                                            default_open_index=0
                                            on_open_index_change=on_workbench_open_change
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <Menubar
                                            id_base="docs-menubar-workbench".to_string()
                                            menus=workbench_menus.get()
                                            on_action=on_workbench_action
                                            close_on_action=workbench_close_on_action.get()
                                            is_close_on_action=workbench_close_on_action.get()
                                            placement=placement
                                            on_open_index_change=on_workbench_open_change
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                    .into_any()
                                }
                            }}
                            <span class="ui-muted">
                                "open menu index: "
                                {move || {
                                    workbench_open_raw
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())
                                }}
                            </span>
                            <span class="ui-muted">
                                "last action (menu:item): "
                                {move || {
                                    workbench_last_action
                                        .get()
                                        .map(|(menu_index, item_index)| {
                                            format!("{}:{}", menu_index, item_index)
                                        })
                                        .unwrap_or_else(|| "None".to_string())
                                }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Open / Close / Controlled Comparison)" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-matrix".to_string()
                        menus=controlled_menus_for_matrix
                        on_action=on_controlled_action
                        close_on_action=false
                        is_close_on_action=false
                        open_index=controlled_open
                        on_open_index_change=on_open_index_change
                        class_name="docs-menubar-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Desktop Menubar + Action Dispatch" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-default".to_string()
                        menus=default_menus_for_default
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open + Persistent + Disabled Menu" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-controlled".to_string()
                        menus=controlled_menus_for_controlled
                        on_action=on_controlled_action
                        close_on_action=false
                        open_index=controlled_open
                        on_open_index_change=on_open_index_change
                        class_name="docs-menubar-custom".to_string()
                    />
                    <span class="ui-muted">
                        "open menu index: "
                        {move || {
                            controlled_open_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_controlled_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_open_raw.set(Some(0))>
                            "Open Menu 0"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(Some(1))>
                            "Open Menu 1"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(None)>
                            "Close"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-class-source / data-close-on-action-source / data-open-index-source / data-motion-source in DevTools."
                    </div>
                    <Menubar
                        id_base="docs-menubar-markers".to_string()
                        menus=marker_menus
                        on_action=on_marker_action
                        close_on_action=false
                        placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
                        open_index=marker_open
                        default_open_index=1
                        on_open_index_change=on_marker_open_change
                        class_name="docs-menubar-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "open menu index: "
                        {move || {
                            marker_open_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
