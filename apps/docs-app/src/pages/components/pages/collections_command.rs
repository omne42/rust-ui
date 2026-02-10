use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::sync::Arc;
use ui_components::{
    Carousel, CarouselItem, CarouselOrientation, Command, CommandDialog, CommandGroup, CommandItem,
    ContextMenu, MenuItemKind, Menubar, MenubarMenu, NavigationMenu, NavigationMenuItem,
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

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_custom_action, set_last_custom_action) = signal("none".to_string());
    let on_custom_action = Callback::new(move |id: String| set_last_custom_action.set(id));

    let code = r#"let groups: Arc<[CommandGroup]> = Arc::from(vec![
  CommandGroup::new("Suggestions", vec![
    CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
    CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
  ]),
  CommandGroup::new("Settings", vec![
    CommandItem::new("profile", "Profile"),
    CommandItem::new("billing", "Billing"),
  ]),
]);

<Command
  id_base="docs-command".to_string()
  groups=groups
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>"#;

    let states_code = r#"<Command
  id_base="docs-command-custom".to_string()
  groups=groups
  placeholder="Search pages, actions, and settings...".to_string()
  empty_label="No command matches your search.".to_string()
  class_name="docs-command-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Command"
            slug="command"
            group="Collections"
            description="Shadcn-compatible command palette with grouped filtering, listbox keyboard semantics, and HeroUI-level spring active-highlight motion."
        >
            <Playground title="Grouped Search + Keyboard Action" code=code>
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

            <Playground title="Custom Placeholder + Empty Label + Disabled Items" code=states_code>
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

    let code = r#"<ContextMenu
  id_base="docs-context-menu".to_string()
  items=items
  on_action=on_action
>
  "Right click or press Shift+F10"
</ContextMenu>"#;

    let states_code = r#"<ContextMenu
  id_base="docs-context-menu-persistent".to_string()
  items=items
  on_action=on_action
  close_on_action=false
  disabled_indices=vec![1]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
>
  "Persistent + disabled item"
</ContextMenu>"#;

    let marker_code = r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ContextMenu
  id_base="docs-context-menu-markers".to_string()
  items=items
  on_action=on_action
  open=open
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
</ContextMenu>"#;

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
            description="Shadcn-compatible context trigger menu with right-click + keyboard open semantics, Spectrum state/source attrs, and HeroUI-level popover spring motion reuse."
        >
            <Playground title="Right Click + Keyboard Open" code=code>
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

            <Playground title="Persistent + Disabled + ItemKinds" code=states_code>
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

            <Playground title="State + Source Markers" code=marker_code>
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

    let code = r#"let menus = vec![
  MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
  MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
];

<Menubar
  id_base="docs-menubar".to_string()
  menus=menus
  on_action=Callback::new(move |(menu_index, item_index)| {
    set_last_action.set(Some((menu_index, item_index)));
  })
/>"#;

    let states_code = r#"let (open_menu, set_open_menu) = signal(None::<usize>);
let open_menu_signal: Signal<Option<usize>> = Signal::derive(move || open_menu.get());

<Menubar
  id_base="docs-menubar-controlled".to_string()
  menus=menus
  on_action=on_action
  close_on_action=false
  open_index=open_menu_signal
  on_open_index_change=Callback::new(move |next| set_open_menu.set(next))
/>"#;

    let marker_code = r#"let (open_raw, set_open_raw) = signal(Some(0usize));
let open_signal: Signal<Option<usize>> = Signal::derive(move || open_raw.get());

<Menubar
  id_base="docs-menubar-markers".to_string()
  menus=menus
  on_action=on_action
  close_on_action=false
  placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()
  open_index=open_signal
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
/>"#;

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
            description="Shadcn-compatible persistent menubar with horizontal trigger navigation, Spectrum-style state/source attrs, and HeroUI-level spring popover motion reuse."
        >
            <Playground title="Desktop Menubar + Action Dispatch" code=code>
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

            <Playground title="Controlled Open + Persistent + Disabled Menu" code=states_code>
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

            <Playground title="State + Source Markers" code=marker_code>
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

    let code = r#"<NavigationMenu
  id_base=\"docs-navigation-menu\".to_string()
  items=items
  default_selected_id=\"components\".to_string()
  on_selected_id_change=Callback::new(move |next: Option<String>| {
    set_last_selected.set(next.unwrap_or_else(|| \"none\".to_string()));
  })
/>"#;

    let states_code = r#"let (selected, set_selected) = signal(Some(\"docs\".to_string()));
let selected_signal: Signal<Option<String>> = Signal::derive(move || selected.get());

<NavigationMenu
  id_base=\"docs-navigation-menu-controlled\".to_string()
  items=items
  selected_id=selected_signal
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
/>"#;

    view! {
        <ComponentPage
            title="NavigationMenu"
            slug="navigation-menu"
            group="Collections"
            description="Shadcn-compatible horizontal navigation menu with roving keyboard focus, controllable selection state, Spectrum data contracts, and HeroUI-level active-highlight spring motion reuse."
        >
            <Playground title="Default + Roving Focus + Selection" code=code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-default".to_string()
                        items=base_items
                        default_selected_id="components".to_string()
                        on_selected_id_change=on_selected_id_change
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || last_selected.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Selection + activate_on_focus=false" code=states_code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-controlled".to_string()
                        items=controlled_items
                        selected_id=controlled_selected
                        on_selected_id_change=on_controlled_selected_change
                        activate_on_focus=false
                        aria_label="Header navigation".to_string()
                        class_name="docs-navigation-menu-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled selected: "
                        {move || controlled_selected_raw.get().unwrap_or_else(|| "none".to_string())}
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
            .description("Build Spectrum-compatible surfaces with production-grade motion."),
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

    let (last_selected, set_last_selected) = signal(None::<usize>);
    let on_selected_change = Callback::new(move |next: Option<usize>| set_last_selected.set(next));

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some(0_usize));
    let controlled_selected: Signal<Option<usize>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<usize>| {
        set_controlled_selected_raw.set(next);
    });

    let code = r#"<Carousel
  id_base=\"docs-carousel\".to_string()
  items=items
  default_selected_index=1
  on_selected_index_change=Callback::new(move |next: Option<usize>| {
    set_last_selected.set(next);
  })
/>"#;

    let states_code = r#"let (selected, set_selected) = signal(Some(0_usize));
let selected_signal: Signal<Option<usize>> = Signal::derive(move || selected.get());

<Carousel
  id_base=\"docs-carousel-vertical\".to_string()
  items=items
  selected_index=selected_signal
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  orientation=CarouselOrientation::Vertical
  loop_navigation=false
/>"#;

    view! {
        <ComponentPage
            title="Carousel"
            slug="carousel"
            group="Collections"
            description="Shadcn-compatible carousel with controllable slide index, orientation-aware keyboard navigation, Spectrum data contracts, and HeroUI-level spring indicator-highlight motion reuse."
        >
            <Playground title="Default + Indicator Motion" code=code>
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

            <Playground title="Controlled + Vertical + No Loop" code=states_code>
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

    let code = r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<CommandDialog
  groups=groups
  open=open
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>"#;

    let marker_code = r#"<CommandDialog
  groups=groups
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
/>"#;

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
            description="Shadcn-compatible command dialog that composes Modal + Command, supports controlled/uncontrolled open state, emits Spectrum data contracts, and reuses HeroUI-level overlay/active-highlight spring motion."
        >
            <Playground title="Controlled Open + Action Close" code=code>
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

            <Playground title="State + Source Markers" code=marker_code>
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
