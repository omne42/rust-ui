use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::sync::Arc;
use ui_components::{
    Carousel, CarouselItem, CarouselOrientation, Command, CommandDialog, CommandGroup, CommandItem,
    ContextMenu, MenuItemKind, Menubar, MenubarMenu, NavigationMenu, NavigationMenuItem,
    SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn command() -> AnyView {
    let groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Suggestions",
            vec![
                CommandItem::new("calendar", "Calendar")
                    .keywords(vec!["date".to_string(), "event".to_string()])
                    .shortcut("⌘K"),
                CommandItem::new("search-emoji", "Search Emoji")
                    .keywords(vec!["emoji".to_string(), "icon".to_string()])
                    .shortcut("⌘E"),
                CommandItem::new("calculator", "Calculator")
                    .keywords(vec!["math".to_string(), "compute".to_string()]),
            ],
        ),
        CommandGroup::new(
            "Settings",
            vec![
                CommandItem::new("profile", "Profile").shortcut("⌘P"),
                CommandItem::new("billing", "Billing").shortcut("⌘B"),
                CommandItem::new("team", "Team").disabled(true),
            ],
        ),
    ]);

    let custom_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Quick Actions",
            vec![
                CommandItem::new("new-file", "New File")
                    .keywords(vec!["create".to_string(), "document".to_string()])
                    .shortcut("⌘N"),
                CommandItem::new("new-window", "New Window")
                    .keywords(vec!["window".to_string(), "workspace".to_string()])
                    .shortcut("⌘⇧N"),
            ],
        ),
        CommandGroup::new(
            "Account",
            vec![
                CommandItem::new("preferences", "Preferences").shortcut("⌘,"),
                CommandItem::new("manage-billing", "Manage Billing").shortcut("⌘⇧B"),
                CommandItem::new("admin-only", "Admin Only").disabled(true),
            ],
        ),
    ]);

    let marker_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Workspace",
            vec![
                CommandItem::new("open-recent", "Open Recent")
                    .keywords(vec!["recent".to_string(), "workspace".to_string()])
                    .shortcut("⌘R"),
                CommandItem::new("new-workspace", "New Workspace")
                    .keywords(vec!["create".to_string(), "workspace".to_string()])
                    .shortcut("⌘⇧W"),
            ],
        ),
        CommandGroup::new(
            "Automation",
            vec![
                CommandItem::new("run-tests", "Run Tests")
                    .keywords(vec!["test".to_string(), "verify".to_string()]),
                CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
            ],
        ),
    ]);

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_custom_action, set_last_custom_action) = signal("none".to_string());
    let on_custom_action = Callback::new(move |id: String| set_last_custom_action.set(id));

    let (last_marker_action, set_last_marker_action) = signal("none".to_string());
    let on_marker_action = Callback::new(move |id: String| set_last_marker_action.set(id));

    let code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Command
  id_base="docs-command".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Command
  id_base="docs-command-custom".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  placeholder="Search pages, actions, and settings...".to_string()
  empty_label="No command matches your search.".to_string()
  class_name="docs-command-custom".to_string()
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let mut custom_motion = ui_components::CommandMotion::default();
custom_motion.spring.stiffness = 240.0;
custom_motion.spring.damping = 20.0;
let (last_action, set_last_action) = signal("none".to_string());

<Command
  id_base="docs-command-markers".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  on_action=Callback::new(move |id: String| set_last_action.set(id))
  placeholder="Search workspace actions...".to_string()
  empty_label="No workspace action found.".to_string()
  aria_label="Workspace command center".to_string()
  class_name="docs-command-custom".to_string()
  motion=custom_motion
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let mut marker_motion = ui_components::CommandMotion::default();
    marker_motion.spring.stiffness = 240.0;
    marker_motion.spring.damping = 20.0;

    let workbench_options = vec![
        "Default".to_string(),
        "Custom labels".to_string(),
        "Disabled + custom motion".to_string(),
    ];
    let (workbench_index, set_workbench_index) = signal(Some(0_usize));
    let workbench_disabled = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let workbench_custom_text = Signal::derive(move || workbench_index.get().unwrap_or(0) >= 1);
    let workbench_custom_motion = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);

    let (last_workbench_action, set_last_workbench_action) = signal("none".to_string());
    let on_workbench_action = Callback::new(move |id: String| set_last_workbench_action.set(id));
    let groups_for_workbench = groups.clone();

    let workbench_motion = Signal::derive(move || {
        let mut motion = ui_components::CommandMotion::default();
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 280.0;
            motion.spring.damping = 21.0;
        }
        motion
    });

    let workbench_code = Signal::derive(move || {
        let use_custom = workbench_custom_text.get();
        let use_motion = workbench_custom_motion.get();
        let is_disabled = workbench_disabled.get();

        let mut lines = vec![
            "<Command".to_string(),
            "  id_base=\"docs-command-workbench\".to_string()".to_string(),
            "  groups=groups.clone()".to_string(),
            "  on_action=Callback::new(move |id: String| set_last_action.set(id))".to_string(),
        ];

        if use_custom {
            lines.push("  placeholder=\"Search docs actions...\".to_string()".to_string());
            lines.push("  empty_label=\"No docs action found.\".to_string()".to_string());
            lines.push("  aria_label=\"Docs command center\".to_string()".to_string());
            lines.push("  class_name=\"docs-command-custom\".to_string()".to_string());
        }

        if is_disabled {
            lines.push("  disabled=true".to_string());
        }

        if use_motion {
            lines.push("  motion=ui_components::CommandMotion {".to_string());
            lines.push("    spring: ui_motion::spring::SpringConfig {".to_string());
            lines.push("      stiffness: 280.0,".to_string());
            lines.push("      damping: 21.0,".to_string());
            lines.push("      ..ui_motion::presets::spring_slide()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/command/styles.rs */\n{}",
            ui_components::command::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let scenario = workbench_index.get().unwrap_or(0);
        format!(
            "CommandWorkbenchConfig {{\n  scenario: {scenario},\n  disabled: {},\n  custom_text: {},\n  custom_motion: {},\n  class_name: {},\n}}",
            workbench_disabled.get(),
            workbench_custom_text.get(),
            workbench_custom_motion.get(),
            if workbench_custom_text.get() {
                "\"docs-command-custom\""
            } else {
                "\"\""
            }
        )
    });

    view! {
        <ComponentPage
            title="Command"
            slug="command"
            group="Collections"
            description="baseline-compatible command palette with grouped filtering, listbox keyboard semantics, baseline data contracts, and baseline-level spring active-highlight motion."
        >
            <Playground title="Grouped Search + Keyboard Action" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Command
                        id_base="docs-command-default".to_string()
                        groups=groups.clone()
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Custom Placeholder + Empty Label + Disabled Items" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Command
                        id_base="docs-command-custom".to_string()
                        groups=custom_groups
                        on_action=on_custom_action
                        placeholder="Search pages, actions, and settings...".to_string()
                        empty_label="No command matches your search.".to_string()
                        class_name="docs-command-custom".to_string()
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_custom_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="ui-muted">
                        "Inspect data-id-source / data-placeholder-source / data-empty-label-source / data-aria-label-source / data-action-source / data-motion-source in DevTools."
                    </div>
                    <Command
                        id_base="docs-command-markers".to_string()
                        groups=marker_groups
                        on_action=on_marker_action
                        placeholder="Search workspace actions...".to_string()
                        empty_label="No workspace action found.".to_string()
                        aria_label="Workspace command center".to_string()
                        class_name="docs-command-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_marker_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test workbench for command state/source contract tuning."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/command/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="command-workbench-controls">
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-command-workbench-scenario".to_string()
                            options=workbench_options.clone()
                            selected_index=workbench_index
                            set_selected_index=set_workbench_index
                            size=SegmentedControlSize::Sm
                            aria_label="Command scenario".to_string()
                        />

                        <div class="ui-muted">
                            "disabled: "
                            {move || workbench_disabled.get().to_string()}
                        </div>
                        <div class="ui-muted">
                            "custom labels: "
                            {move || workbench_custom_text.get().to_string()}
                        </div>
                        <div class="ui-muted">
                            "custom motion: "
                            {move || workbench_custom_motion.get().to_string()}
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Command
                        id_base="docs-command-workbench".to_string()
                        groups=groups_for_workbench.clone()
                        on_action=on_workbench_action
                        disabled=workbench_disabled.get()
                        motion=workbench_motion.get()
                        placeholder=if workbench_custom_text.get() {
                            "Search docs actions...".to_string()
                        } else {
                            String::new()
                        }
                        empty_label=if workbench_custom_text.get() {
                            "No docs action found.".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_text.get() {
                            "Docs command center".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_text.get() {
                            "docs-command-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_workbench_action.get()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn context_menu() -> AnyView {
    let default_items = vec![
        "Back".to_string(),
        "Forward".to_string(),
        "Reload".to_string(),
    ];
    let keep_open_items = vec![
        "Copy".to_string(),
        "Paste".to_string(),
        "Inspect".to_string(),
    ];
    let marker_items = vec![
        "Duplicate".to_string(),
        "Rename".to_string(),
        "Delete".to_string(),
    ];

    let (last_default_action, set_last_default_action) = signal(None::<usize>);
    let on_default_action =
        Callback::new(move |index: usize| set_last_default_action.set(Some(index)));

    let (last_keep_open_action, set_last_keep_open_action) = signal(None::<usize>);
    let on_keep_open_action =
        Callback::new(move |index: usize| set_last_keep_open_action.set(Some(index)));

    let (marker_open_raw, set_marker_open_raw) = signal(false);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change = Callback::new(move |next: bool| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<usize>);
    let on_marker_action =
        Callback::new(move |index: usize| set_last_marker_action.set(Some(index)));

    let code = Signal::derive(move || {
        r#"<ContextMenu
  id_base="docs-context-menu".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Right click or press Shift+F10"
</ContextMenu>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ContextMenu
  id_base="docs-context-menu-persistent".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  close_on_action=false
  disabled_indices=vec![1]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
>
  "Persistent + disabled item"
</ContextMenu>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<ContextMenu
  id_base="docs-context-menu-markers".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  open=Signal::derive(move || open_raw.get())
  default_open=true
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  close_on_action=false
  disabled_indices=vec![2]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
  aria_label="Workspace context actions".to_string()
  class_name="docs-context-menu-custom".to_string()
  motion=ui_components::ContextMenuMotion {
    popover: ui_components::PopoverMotion {
      initial_scale: 0.94,
      offset_y_px: 10.0,
      ..ui_components::PopoverMotion::default()
    },
  }
>
  "Inspect state + source markers"
</ContextMenu>"#
            .to_string()
    });

    let marker_motion = ui_components::ContextMenuMotion {
        popover: ui_components::PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 10.0,
            ..ui_components::PopoverMotion::default()
        },
    };

    view! {
        <ComponentPage
            title="ContextMenu"
            slug="context-menu"
            group="Collections"
            description="baseline-compatible context trigger menu with right-click + keyboard open semantics, baseline state/source attrs, and baseline-level popover spring motion reuse."
        >
            <Playground title="Right Click + Keyboard Open" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <ContextMenu
                        id_base="docs-context-menu-default".to_string()
                        items=default_items
                        on_action=on_default_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Right click or press Shift+F10"
                    </ContextMenu>
                    <span class="ui-muted">
                        "last action: "
                        {move || {
                            last_default_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Persistent + Disabled + ItemKinds" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ContextMenu
                        id_base="docs-context-menu-persistent".to_string()
                        items=keep_open_items
                        on_action=on_keep_open_action
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                        aria_label="File actions".to_string()
                        class_name="docs-context-menu-custom".to_string()
                    >
                        "Persistent + disabled item"
                    </ContextMenu>
                    <span class="ui-muted">
                        "last action: "
                        {move || {
                            last_keep_open_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted">"close_on_action: false (selection keeps menu open)"</span>
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
                        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-close-on-action-source / data-open-source / data-motion-source in DevTools."
                    </div>
                    <ContextMenu
                        id_base="docs-context-menu-markers".to_string()
                        items=marker_items
                        on_action=on_marker_action
                        open=marker_open
                        default_open=true
                        on_open_change=on_marker_open_change
                        close_on_action=false
                        disabled_indices=vec![2]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                        aria_label="Workspace context actions".to_string()
                        class_name="docs-context-menu-custom".to_string()
                        motion=marker_motion
                    >
                        "Right click or press Shift+F10 to inspect markers"
                    </ContextMenu>
                    <span class="ui-muted">
                        "open: "
                        {move || if marker_open_raw.get() { "true" } else { "false" }}
                    </span>
                    <span class="ui-muted">
                        "last action: "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|value| value.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menubar() -> AnyView {
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

        lines.push("  id_base=\"docs-menubar-workbench\".to_string()".to_string());
        lines.push("  menus=menus".to_string());
        lines.push("  on_action=on_action".to_string());
        lines.push(format!(
            "  close_on_action={}",
            workbench_close_on_action.get()
        ));

        if workbench_flip_placement.get() {
            lines.push(
                "  placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()".to_string(),
            );
        }
        if workbench_default_open.get() {
            lines.push("  default_open_index=0".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-menubar-custom\".to_string()".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push("  motion=ui_components::MenubarMotion {".to_string());
            lines.push("    popover: ui_components::PopoverMotion {".to_string());
            lines.push("      initial_scale: 0.94,".to_string());
            lines.push("      offset_y_px: 10.0,".to_string());
            lines.push("      ..ui_components::PopoverMotion::default()".to_string());
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
            "/* crates/ui-components/src/menubar/styles.rs */\n{}",
            ui_components::menubar::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let menus = workbench_menus.get();
        let mut class_tokens = vec!["ui-menubar".to_string()];
        if workbench_custom_class.get() {
            class_tokens.push("docs-menubar-custom".to_string());
        }
        format!(
            "MenubarActualConfig {{\n  menu_set: \"{}\",\n  menu_count: {},\n  close_on_action: {},\n  placement: \"{}\",\n  default_open_index: {},\n  custom_motion: {},\n  custom_class_name: {},\n  class: \"{}\",\n}}",
            match menu_set_index.get().unwrap_or(0) {
                1 => "workspace",
                2 => "compact",
                _ => "app",
            },
            menus.len(),
            workbench_close_on_action.get(),
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
  placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()
  open_index=Signal::derive(move || open_raw.get())
  default_open_index=1
  on_open_index_change=Callback::new(move |next| set_open_raw.set(next))
  class_name="docs-menubar-custom".to_string()
  motion=ui_components::MenubarMotion {
    popover: ui_components::PopoverMotion {
      initial_scale: 0.94,
      offset_y_px: 10.0,
      ..ui_components::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let marker_motion = ui_components::MenubarMotion {
        popover: ui_components::PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 10.0,
            ..ui_components::PopoverMotion::default()
        },
    };

    view! {
        <ComponentPage
            title="Menubar"
            slug="menubar"
            group="Collections"
            description="baseline-compatible persistent menubar with horizontal trigger navigation, baseline-style state/source attrs, and baseline-level spring popover motion reuse."
        >
            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Menubar."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/menubar/styles.rs".to_string()
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
                                    ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()
                                } else {
                                    ui_components::menubar::DEFAULT_PLACEMENT
                                };
                                let class_name = if workbench_custom_class.get() {
                                    "docs-menubar-custom".to_string()
                                } else {
                                    String::new()
                                };
                                let motion = if workbench_custom_motion.get() {
                                    ui_components::MenubarMotion {
                                        popover: ui_components::PopoverMotion {
                                            initial_scale: 0.94,
                                            offset_y_px: 10.0,
                                            ..ui_components::PopoverMotion::default()
                                        },
                                    }
                                } else {
                                    ui_components::MenubarMotion::default()
                                };

                                if workbench_default_open.get() {
                                    view! {
                                        <Menubar
                                            id_base="docs-menubar-workbench".to_string()
                                            menus=workbench_menus.get()
                                            on_action=on_workbench_action
                                            close_on_action=workbench_close_on_action.get()
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

            <Playground title="Desktop Menubar + Action Dispatch" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-default".to_string()
                        menus=default_menus
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
                        menus=controlled_menus
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
                        placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()
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

pub(super) fn navigation_menu() -> AnyView {
    let base_items = vec![
        NavigationMenuItem::new("overview", "Overview", "/docs/overview"),
        NavigationMenuItem::new("components", "Components", "/docs/components"),
        NavigationMenuItem::new("patterns", "Patterns", "/docs/patterns"),
        NavigationMenuItem::new("tokens", "Tokens", "/docs/tokens").disabled(true),
    ];

    let controlled_items = vec![
        NavigationMenuItem::new("home", "Home", "/"),
        NavigationMenuItem::new("docs", "Docs", "/docs"),
        NavigationMenuItem::new("blog", "Blog", "/blog"),
    ];

    let marker_items = vec![
        NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
        NavigationMenuItem::new("projects", "Projects", "/projects"),
        NavigationMenuItem::new("settings", "Settings", "/settings"),
    ];

    let (last_selected, set_last_selected) = signal("none".to_string());
    let on_selected_id_change = Callback::new(move |next: Option<String>| {
        set_last_selected.set(next.unwrap_or_else(|| "none".to_string()))
    });

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some("docs".to_string()));
    let controlled_selected: Signal<Option<String>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<String>| {
        set_controlled_selected_raw.set(next);
    });

    let (marker_selected_raw, set_marker_selected_raw) = signal(Some("projects".to_string()));
    let marker_selected: Signal<Option<String>> = Signal::derive(move || marker_selected_raw.get());
    let on_marker_selected_change = Callback::new(move |next: Option<String>| {
        set_marker_selected_raw.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (last_selected, set_last_selected) = signal("none".to_string());

<NavigationMenu
  id_base="docs-navigation-menu".to_string()
  items=vec![
    NavigationMenuItem::new("overview", "Overview", "/overview"),
    NavigationMenuItem::new("components", "Components", "/components"),
    NavigationMenuItem::new("patterns", "Patterns", "/patterns"),
  ]
  default_selected_id="components".to_string()
  on_selected_id_change=Callback::new(move |next: Option<String>| {
    set_last_selected.set(next.unwrap_or_else(|| "none".to_string()));
  })
/>
<span class="ui-muted">"last selected: " {move || last_selected.get()}</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some("docs".to_string()));

<NavigationMenu
  id_base="docs-navigation-menu-controlled".to_string()
  items=vec![
    NavigationMenuItem::new("docs", "Docs", "/docs"),
    NavigationMenuItem::new("api", "API", "/api"),
    NavigationMenuItem::new("guides", "Guides", "/guides"),
  ]
  selected_id=Signal::derive(move || selected.get())
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some("projects".to_string()));
let mut custom_motion = ui_components::NavigationMenuMotion::default();
custom_motion.spring.stiffness = 260.0;
custom_motion.spring.damping = 24.0;

<NavigationMenu
  id_base="docs-navigation-menu-markers".to_string()
  items=vec![
    NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
    NavigationMenuItem::new("projects", "Projects", "/projects"),
    NavigationMenuItem::new("settings", "Settings", "/settings"),
  ]
  selected_id=Signal::derive(move || selected.get())
  default_selected_id="workspace".to_string()
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
  aria_label="Workspace navigation".to_string()
  class_name="docs-navigation-menu-custom".to_string()
  motion=custom_motion
/>"#
        .to_string()
    });

    let (workbench_selected_raw, set_workbench_selected_raw) = signal(Some("projects".to_string()));
    let workbench_selected: Signal<Option<String>> =
        Signal::derive(move || workbench_selected_raw.get());
    let on_workbench_selected_change = Callback::new(move |next: Option<String>| {
        set_workbench_selected_raw.set(next);
    });
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_activate_on_focus, set_workbench_activate_on_focus) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_stiffness, set_workbench_stiffness) = signal(260_u16);
    let (workbench_damping, set_workbench_damping) = signal(24_u16);

    let workbench_code = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let activate_on_focus = workbench_activate_on_focus.get();
        let disable_second = workbench_disable_second.get();
        let custom_class = workbench_custom_class.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_motion = workbench_custom_motion.get();
        let mode_line = if controlled {
            "  selected_id=Signal::derive(move || selected.get())\n  on_selected_id_change=Callback::new(move |next| set_selected.set(next))\n".to_string()
        } else {
            "  default_selected_id=\"workspace\".to_string()\n".to_string()
        };
        let class_line = if custom_class {
            "  class_name=\"docs-navigation-menu-custom\".to_string()\n".to_string()
        } else {
            String::new()
        };
        let aria_line = if custom_aria {
            "  aria_label=\"Workbench navigation\".to_string()\n".to_string()
        } else {
            String::new()
        };
        let motion_line = if custom_motion {
            format!(
                "  motion={{\n    let mut motion = ui_components::NavigationMenuMotion::default();\n    motion.spring.stiffness = {}.0;\n    motion.spring.damping = {}.0;\n    motion\n  }}\n",
                workbench_stiffness.get(),
                workbench_damping.get(),
            )
        } else {
            String::new()
        };
        format!(
            "<NavigationMenu\n  id_base=\"docs-navigation-menu-workbench\".to_string()\n  items=items /* second item disabled: {disable_second} */\n{mode_line}  activate_on_focus={activate_on_focus}\n{class_line}{aria_line}{motion_line}/>"
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/navigation_menu/styles.rs */\n{}",
            ui_components::navigation_menu::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected_raw
            .get()
            .unwrap_or_else(|| "none".to_string());
        let controlled = workbench_controlled.get();
        let activate_on_focus = workbench_activate_on_focus.get();
        let disable_second = workbench_disable_second.get();
        let custom_class = workbench_custom_class.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_motion = workbench_custom_motion.get();
        format!(
            "NavigationMenuActualConfig {{\n  mode: \"{}\",\n  selected_id: \"{}\",\n  activate_on_focus: {},\n  disable_second_item: {},\n  custom_class: {},\n  custom_aria_label: {},\n  custom_motion: {},\n  spring_stiffness: {}.0,\n  spring_damping: {}.0,\n}}",
            if controlled {
                "controlled"
            } else {
                "uncontrolled"
            },
            selected,
            activate_on_focus,
            disable_second,
            custom_class,
            custom_aria,
            custom_motion,
            workbench_stiffness.get(),
            workbench_damping.get(),
        )
    });

    let mut marker_motion = ui_components::NavigationMenuMotion::default();
    marker_motion.spring.stiffness = 260.0;
    marker_motion.spring.damping = 24.0;

    view! {
        <ComponentPage
            title="NavigationMenu"
            slug="navigation-menu"
            group="Collections"
            description="baseline-compatible horizontal navigation menu with roving keyboard focus, controllable selection state, baseline data contracts, and baseline-level active-highlight spring motion reuse."
        >
            <Playground title="Default + Roving Focus + Selection" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-default".to_string()
                        items=base_items
                        default_selected_id="components".to_string()
                        on_selected_id_change=on_selected_id_change
                    />
                    <span class="ui-muted">
                        "last selected: "
                        {move || last_selected.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Manual Activation" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-controlled".to_string()
                        items=controlled_items
                        selected_id=controlled_selected
                        on_selected_id_change=on_controlled_selected_change
                        activate_on_focus=false
                        class_name="docs-navigation-menu-custom".to_string()
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button
                            type="button"
                            on:click=move |_| set_marker_selected_raw.set(Some("workspace".to_string()))
                        >
                            "Select Workspace"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_marker_selected_raw.set(Some("projects".to_string()))
                        >
                            "Select Projects"
                        </button>
                        <button type="button" on:click=move |_| set_marker_selected_raw.set(None)>
                            "Clear"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-activate-on-focus-source / data-selected-id-source / data-selected-id-change-source / data-motion-source in DevTools."
                    </div>
                    <NavigationMenu
                        id_base="docs-navigation-menu-markers".to_string()
                        items=marker_items
                        selected_id=marker_selected
                        default_selected_id="workspace".to_string()
                        on_selected_id_change=on_marker_selected_change
                        activate_on_focus=false
                        aria_label="Workspace navigation".to_string()
                        class_name="docs-navigation-menu-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || marker_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with baseline/configured comparison, live settings, copy-ready code, and scoped CSS test."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/navigation_menu/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="navigation-menu-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_controlled.get()
                                on:change=move |ev| set_workbench_controlled.set(event_target_checked(&ev))
                            />
                            " Controlled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_activate_on_focus.get()
                                on:change=move |ev| set_workbench_activate_on_focus.set(event_target_checked(&ev))
                            />
                            " Activate on focus"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_second.get()
                                on:change=move |ev| set_workbench_disable_second.set(event_target_checked(&ev))
                            />
                            " Disable second item"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " Custom aria label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            "Stiffness "
                            <input
                                type="range"
                                min="160"
                                max="360"
                                prop:value=move || workbench_stiffness.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_stiffness.set(next.clamp(160, 360));
                                    }
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "Damping "
                            <input
                                type="range"
                                min="12"
                                max="40"
                                prop:value=move || workbench_damping.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_damping.set(next.clamp(12, 40));
                                    }
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="navigation-menu-workbench">
                    <span class="ui-muted">
                        "display: baseline vs configured"
                    </span>
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("workspace".to_string()))>
                            "Select Workspace"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("projects".to_string()))>
                            "Select Projects"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("settings".to_string()))>
                            "Select Settings"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(None)>
                            "Clear"
                        </button>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Baseline"</div>
                        <NavigationMenu
                            id_base="docs-navigation-menu-workbench-baseline".to_string()
                            items=vec![
                                NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                                NavigationMenuItem::new("projects", "Projects", "/projects"),
                                NavigationMenuItem::new("settings", "Settings", "/settings"),
                            ]
                            default_selected_id="workspace".to_string()
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Configured"</div>
                        {move || {
                            let items = vec![
                                NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                                NavigationMenuItem::new("projects", "Projects", "/projects")
                                    .disabled(workbench_disable_second.get()),
                                NavigationMenuItem::new("settings", "Settings", "/settings"),
                            ];
                            let custom_class = workbench_custom_class.get();
                            let custom_aria = workbench_custom_aria.get();
                            let mut motion = ui_components::NavigationMenuMotion::default();
                            if workbench_custom_motion.get() {
                                motion.spring.stiffness = f64::from(workbench_stiffness.get());
                                motion.spring.damping = f64::from(workbench_damping.get());
                            }

                            if workbench_controlled.get() {
                                match (custom_aria, custom_class) {
                                    (true, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (true, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                }
                            } else {
                                let default_selected = workbench_selected_raw
                                    .get()
                                    .unwrap_or_else(|| "workspace".to_string());
                                match (custom_aria, custom_class) {
                                    (true, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (true, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                }
                            }
                        }}
                    </div>
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn carousel() -> AnyView {
    let base_items = vec![
        CarouselItem::new("welcome", "Welcome")
            .description("Build baseline-compatible surfaces with production-grade motion."),
        CarouselItem::new("tokens", "Theme Tokens")
            .description("Tune OKLCH and OLED tokens without breaking component contracts."),
        CarouselItem::new("shipping", "Shipping")
            .description("Run format + check + pre-commit and ship with confidence."),
    ];

    let vertical_items = vec![
        CarouselItem::new("a", "Alpha").description("Vertical orientation demo."),
        CarouselItem::new("b", "Beta")
            .description("Second slide.")
            .disabled(true),
        CarouselItem::new("c", "Gamma").description("Loop disabled demo."),
    ];

    let marker_items = vec![
        CarouselItem::new("overview", "Overview")
            .description("Inspect source markers directly in DevTools."),
        CarouselItem::new("analytics", "Analytics")
            .description("Controlled index + motion markers for regressions."),
        CarouselItem::new("settings", "Settings")
            .description("Custom orientation and navigation mode markers.")
            .disabled(true),
    ];

    let (last_selected, set_last_selected) = signal(None::<usize>);
    let on_selected_change = Callback::new(move |next: Option<usize>| set_last_selected.set(next));

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some(0_usize));
    let controlled_selected: Signal<Option<usize>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<usize>| {
        set_controlled_selected_raw.set(next);
    });

    let (marker_selected_raw, set_marker_selected_raw) = signal(Some(1_usize));
    let marker_selected: Signal<Option<usize>> = Signal::derive(move || marker_selected_raw.get());
    let on_marker_selected_change = Callback::new(move |next: Option<usize>| {
        set_marker_selected_raw.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (last_selected, set_last_selected) = signal(None::<usize>);

<Carousel
  id_base="docs-carousel".to_string()
  items=vec![
    CarouselItem::new("release-1", "Release 1").description("Faster build pipeline"),
    CarouselItem::new("release-2", "Release 2").description("New audit dashboard"),
    CarouselItem::new("release-3", "Release 3").description("Improved accessibility"),
  ]
  default_selected_index=1
  on_selected_index_change=Callback::new(move |next: Option<usize>| {
    set_last_selected.set(next);
  })
/>
<span class="ui-muted">"last selected: " {move || last_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}</span>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));

<Carousel
  id_base="docs-carousel-vertical".to_string()
  items=vec![
    CarouselItem::new("slide-a", "Slide A").description("First item"),
    CarouselItem::new("slide-b", "Slide B").description("Second item"),
    CarouselItem::new("slide-c", "Slide C").description("Third item"),
  ]
  selected_index=Signal::derive(move || selected.get())
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  orientation=CarouselOrientation::Vertical
  loop_navigation=false
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let mut custom_motion = ui_components::CarouselMotion::default();
custom_motion.spring.stiffness = 250.0;
custom_motion.spring.damping = 22.0;

<Carousel
  id_base="docs-carousel-markers".to_string()
  items=vec![
    CarouselItem::new("spotlight-1", "Spotlight 1").description("Migration complete"),
    CarouselItem::new("spotlight-2", "Spotlight 2").description("Latency reduced"),
    CarouselItem::new("spotlight-3", "Spotlight 3").description("Error rate stable"),
  ]
  selected_index=Signal::derive(move || selected.get())
  default_selected_index=0
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  orientation=CarouselOrientation::Vertical
  loop_navigation=false
  aria_label="Workspace spotlight".to_string()
  class_name="docs-carousel-custom".to_string()
  motion=custom_motion
/>"#
        .to_string()
    });

    let mut marker_motion = ui_components::CarouselMotion::default();
    marker_motion.spring.stiffness = 250.0;
    marker_motion.spring.damping = 22.0;

    view! {
        <ComponentPage
            title="Carousel"
            slug="carousel"
            group="Collections"
            description="baseline-compatible carousel with controllable slide index, orientation-aware keyboard navigation, baseline data contracts, and baseline-level spring indicator-highlight motion reuse."
        >
            <Playground title="Default + Indicator Motion" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Carousel
                        id_base="docs-carousel-default".to_string()
                        items=base_items
                        default_selected_index=1
                        on_selected_index_change=on_selected_change
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            last_selected
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Vertical + No Loop" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Carousel
                        id_base="docs-carousel-vertical".to_string()
                        items=vertical_items
                        selected_index=controlled_selected
                        on_selected_index_change=on_controlled_selected_change
                        orientation=CarouselOrientation::Vertical
                        loop_navigation=false
                        aria_label="Feature carousel".to_string()
                        class_name="docs-carousel-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled selected: "
                        {move || {
                            controlled_selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_selected_raw.set(Some(0))>
                            "Select Overview"
                        </button>
                        <button type="button" on:click=move |_| set_marker_selected_raw.set(Some(1))>
                            "Select Analytics"
                        </button>
                        <button type="button" on:click=move |_| set_marker_selected_raw.set(None)>
                            "Clear"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-orientation-source / data-loop-navigation-source / data-selected-index-source / data-selected-index-change-source / data-motion-source in DevTools."
                    </div>
                    <Carousel
                        id_base="docs-carousel-markers".to_string()
                        items=marker_items
                        selected_index=marker_selected
                        default_selected_index=0
                        on_selected_index_change=on_marker_selected_change
                        orientation=CarouselOrientation::Vertical
                        loop_navigation=false
                        aria_label="Workspace spotlight".to_string()
                        class_name="docs-carousel-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            marker_selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn command_dialog() -> AnyView {
    let groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Suggestions",
            vec![
                CommandItem::new("calendar", "Calendar")
                    .keywords(vec!["date".to_string(), "event".to_string()])
                    .shortcut("⌘K"),
                CommandItem::new("search-emoji", "Search Emoji")
                    .keywords(vec!["emoji".to_string(), "icon".to_string()])
                    .shortcut("⌘E"),
                CommandItem::new("calculator", "Calculator")
                    .keywords(vec!["math".to_string(), "compute".to_string()]),
            ],
        ),
        CommandGroup::new(
            "Settings",
            vec![
                CommandItem::new("profile", "Profile").shortcut("⌘P"),
                CommandItem::new("billing", "Billing").shortcut("⌘B"),
                CommandItem::new("team", "Team").disabled(true),
            ],
        ),
    ]);

    let marker_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Workspace",
            vec![
                CommandItem::new("new-file", "New File")
                    .keywords(vec!["create".to_string(), "document".to_string()])
                    .shortcut("⌘N"),
                CommandItem::new("new-window", "New Window")
                    .keywords(vec!["window".to_string(), "workspace".to_string()])
                    .shortcut("⌘⇧N"),
            ],
        ),
        CommandGroup::new(
            "Account",
            vec![
                CommandItem::new("preferences", "Preferences").shortcut("⌘,"),
                CommandItem::new("manage-billing", "Manage Billing").shortcut("⌘⇧B"),
                CommandItem::new("admin-only", "Admin Only").disabled(true),
            ],
        ),
    ]);

    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_marker_action, set_last_marker_action) = signal("none".to_string());
    let on_marker_action = Callback::new(move |id: String| set_last_marker_action.set(id));

    let code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let (last_action, set_last_action) = signal("none".to_string());

<CommandDialog
  groups=vec![
    CommandGroup::new("Navigation", vec![
      CommandItem::new("go-dashboard", "Go to Dashboard"),
      CommandItem::new("open-projects", "Open Projects"),
    ]),
    CommandGroup::new("Actions", vec![
      CommandItem::new("run-tests", "Run Tests"),
      CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
    ]),
  ]
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"<CommandDialog
  groups=vec![
    CommandGroup::new("Navigation", vec![
      CommandItem::new("go-dashboard", "Go to Dashboard"),
      CommandItem::new("open-projects", "Open Projects"),
    ]),
    CommandGroup::new("Actions", vec![
      CommandItem::new("run-tests", "Run Tests"),
      CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
    ]),
  ]
  id_base="docs-command-dialog-marker".to_string()
  title="Workspace Commands".to_string()
  description="Inspect source-state markers".to_string()
  default_open=true
  close_on_action=false
  placeholder="Search pages, actions, and settings...".to_string()
  empty_label="No command matches your search.".to_string()
  aria_label="Workspace command dialog".to_string()
  class_name="docs-command-dialog-custom".to_string()
  overlay_motion=ui_components::OverlayMotion {
    initial_scale: 0.95,
    initial_y_px: 10.0,
    ..ui_components::OverlayMotion::default()
  }
/>"#
        .to_string()
    });

    let marker_overlay_motion = ui_components::OverlayMotion {
        initial_scale: 0.95,
        initial_y_px: 10.0,
        ..ui_components::OverlayMotion::default()
    };

    view! {
        <ComponentPage
            title="CommandDialog"
            slug="command-dialog"
            group="Collections"
            description="baseline-compatible command dialog that composes Modal + Command, supports controlled/uncontrolled open state, emits baseline data contracts, and reuses baseline-level overlay/active-highlight spring motion."
        >
            <Playground title="Controlled Open + Action Close" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_open_raw.set(true)>
                            "Open CommandDialog"
                        </button>
                        <button type="button" on:click=move |_| set_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <CommandDialog
                        id_base="docs-command-dialog-controlled".to_string()
                        title="Quick Actions".to_string()
                        description="Press ⌘K-style filtering and Enter to run actions.".to_string()
                        groups=groups.clone()
                        open=open
                        on_open_change=on_open_change
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || if open_raw.get() { "true" } else { "false" }}
                    </span>
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="ui-muted">
                        "Inspect data-id-source / data-title-source / data-description-source / data-placeholder-source / data-action-source / data-overlay-motion-source in DevTools."
                    </div>
                    <CommandDialog
                        id_base="docs-command-dialog-marker".to_string()
                        title="Workspace Commands".to_string()
                        description="close_on_action=false keeps the dialog open after choosing an action.".to_string()
                        groups=marker_groups
                        default_open=true
                        close_on_action=false
                        on_action=on_marker_action
                        placeholder="Search pages, actions, and settings...".to_string()
                        empty_label="No command matches your search.".to_string()
                        aria_label="Workspace command dialog".to_string()
                        class_name="docs-command-dialog-custom".to_string()
                        overlay_motion=marker_overlay_motion
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_marker_action.get()}
                    </span>
                    <span class="ui-muted">"close_on_action: false (dialog stays open)"</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
