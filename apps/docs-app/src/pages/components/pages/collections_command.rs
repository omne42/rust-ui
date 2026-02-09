use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::sync::Arc;
use ui_components::{Command, CommandGroup, CommandItem, ContextMenu, MenuItemKind};

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

    let (last_default_action, set_last_default_action) = signal(None::<usize>);
    let on_default_action =
        Callback::new(move |index: usize| set_last_default_action.set(Some(index)));

    let (last_keep_open_action, set_last_keep_open_action) = signal(None::<usize>);
    let on_keep_open_action =
        Callback::new(move |index: usize| set_last_keep_open_action.set(Some(index)));

    let code = r#"<ContextMenu
  id_base=\"docs-context-menu\".to_string()
  items=items
  on_action=on_action
>
  "Right click or press Shift+F10"
</ContextMenu>"#;

    let states_code = r#"<ContextMenu
  id_base=\"docs-context-menu-persistent\".to_string()
  items=items
  on_action=on_action
  close_on_action=false
  disabled_indices=vec![1]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
>
  "Persistent + disabled item"
</ContextMenu>"#;

    view! {
        <ComponentPage
            title="ContextMenu"
            slug="context-menu"
            group="Collections"
            description="Shadcn-compatible context trigger menu with right-click + keyboard open semantics, Spectrum state attrs, and HeroUI-level popover spring motion reuse."
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
        </ComponentPage>
    }
    .into_any()
}
