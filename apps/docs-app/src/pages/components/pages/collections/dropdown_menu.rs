use super::*;

pub(crate) fn dropdown_menu() -> AnyView {
    let default_items = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ];
    let showcase_items = default_items.clone();
    let default_playground_items = default_items.clone();
    let controlled_items = vec![
        "Rename".to_string(),
        "Move".to_string(),
        "Share".to_string(),
    ];
    let disabled_items = vec!["Duplicate".to_string(), "Archive".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let (interactive_last, set_interactive_last) = signal(None::<usize>);
    let on_interactive_action =
        Callback::new(move |index: usize| set_interactive_last.set(Some(index)));
    let (interactive_item_mode, set_interactive_item_mode) = signal(Some(0_usize));
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_close_on_action, set_interactive_close_on_action) = signal(true);
    let (interactive_controlled, set_interactive_controlled) = signal(false);
    let (interactive_with_disabled_items, set_interactive_with_disabled_items) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_open_raw, set_interactive_open_raw) = signal(false);
    let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());
    let on_interactive_open_change =
        Callback::new(move |next: bool| set_interactive_open_raw.set(next));
    let item_mode_options = vec![
        "3 items".to_string(),
        "2 items".to_string(),
        "empty".to_string(),
    ];

    let interactive_code = Signal::derive(move || {
        let item_mode = interactive_item_mode.get().unwrap_or(0);
        let disabled = interactive_disabled.get();
        let close_on_action = interactive_close_on_action.get();
        let controlled = interactive_controlled.get();
        let with_disabled_items = interactive_with_disabled_items.get();
        let custom_class = interactive_custom_class.get();
        let custom_motion = interactive_custom_motion.get();

        let items_code = match item_mode {
            1 => "vec![\"Rename\".into(), \"Share\".into()]".to_string(),
            2 => "Vec::<String>::new()".to_string(),
            _ => "vec![\"Duplicate\".into(), \"Move\".into(), \"Archive\".into()]".to_string(),
        };

        let mut lines = vec![
            format!("let items = {items_code};"),
            "".to_string(),
            "<DropdownMenu".to_string(),
            "  id_base=\"docs-dropdown-interactive\".into()".to_string(),
            "  items=items".to_string(),
            "  on_action=Callback::new(move |index: usize| { /* ... */ })".to_string(),
            "  is_disabled=Some(false)".to_string(),
            "  is_close_on_action=Some(true)".to_string(),
            "  placement=PopoverPlacement::BottomStart".to_string(),
            "  default_open=Some(false)".to_string(),
            "  trigger_variant=ButtonVariant::Secondary".to_string(),
            "  trigger_size=ButtonSize::Sm".to_string(),
        ];

        if disabled {
            lines.push("  is_disabled=Some(true)".to_string());
        }
        if !close_on_action {
            lines.push("  is_close_on_action=Some(false)".to_string());
        }
        if controlled {
            lines.push("  is_open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if with_disabled_items {
            lines.push("  disabled_indices=vec![1]".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-dropdown-custom\".into()".to_string());
        }
        if custom_motion {
            lines.push("  motion=DropdownMenuMotion {".to_string());
            lines.push("    popover: PopoverMotion {".to_string());
            lines.push("      initial_scale: 0.96,".to_string());
            lines.push("      offset_y_px: 14.0,".to_string());
            lines.push("      ..PopoverMotion::default()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }
        lines.push(">".to_string());
        lines.push("  \"Actions\"".to_string());
        lines.push("</DropdownMenu>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/dropdown_menu/styles.rs */\n{}",
            ui::dropdown_menu::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        let items: Vec<&str> = match interactive_item_mode.get().unwrap_or(0) {
            1 => vec!["Rename", "Share"],
            2 => Vec::new(),
            _ => vec!["Duplicate", "Move", "Archive"],
        };
        let item_kinds: Vec<&str> = if items.is_empty() {
            Vec::new()
        } else {
            vec!["Action"; items.len()]
        };
        let motion = if interactive_custom_motion.get() {
            "DropdownMenuMotion { popover: PopoverMotion { initial_scale: 0.96, offset_y_px: 14.0, ..PopoverMotion::default() } }"
        } else {
            "DropdownMenuMotion::default()"
        };
        let class_name: Option<&str> = if interactive_custom_class.get() {
            Some("docs-dropdown-custom")
        } else {
            None
        };
        let on_open_change_feedback = if interactive_controlled.get() {
            format!(
                "set_interactive_open_raw(open={})",
                interactive_open_raw.get()
            )
        } else {
            "uncontrolled".to_string()
        };
        format!(
            "DropdownMenuActualConfig {{\n  id_base: \"docs-dropdown-interactive\",\n  items: {:?},\n  is_disabled: {:?},\n  item_kinds: {:?},\n  is_close_on_action: {:?},\n  placement: {:?},\n  is_open: {:?},\n  default_open: {:?},\n  on_action: \"last={:?}\",\n  on_open_change: {:?},\n  trigger_variant: {:?},\n  trigger_size: {:?},\n  motion: {motion},\n  class_name: {:?},\n  disabled_indices: {:?},\n}}",
            items,
            Some(interactive_disabled.get()),
            item_kinds,
            Some(interactive_close_on_action.get()),
            ui_headless::PopoverPlacement::BottomStart,
            if interactive_controlled.get() {
                Some(interactive_open_raw.get())
            } else {
                None
            },
            Some(false),
            interactive_last.get(),
            on_open_change_feedback,
            ui::ButtonVariant::Secondary,
            ui::ButtonSize::Sm,
            class_name,
            if interactive_with_disabled_items.get() {
                vec![1]
            } else {
                Vec::new()
            },
        )
    });

    let code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open"
</DropdownMenu>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<DropdownMenu
  id_base="dd-controlled".to_string()
  items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()]
  on_action=Callback::new(move |_: usize| {})
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  close_on_action=false
  disabled_indices=vec![1]
>
  "Persistent"
</DropdownMenu>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</DropdownMenu>
<DropdownMenu
  id_base="dd-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</DropdownMenu>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<DropdownMenu id_base="dd-matrix-default".to_string() items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()] on_action=Callback::new(move |_: usize| {}) placement=PopoverPlacement::BottomStart trigger_variant=ButtonVariant::Secondary trigger_size=ButtonSize::Sm>
  "Default"
</DropdownMenu>
<DropdownMenu id_base="dd-matrix-controlled".to_string() items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()] on_action=Callback::new(move |_: usize| {}) is_open=Signal::derive(move || open.get()) default_open=false on_open_change=Callback::new(move |next| set_open.set(next)) is_close_on_action=Some(false) placement=PopoverPlacement::TopEnd trigger_variant=ButtonVariant::Secondary trigger_size=ButtonSize::Sm>
  "Controlled"
</DropdownMenu>
<DropdownMenu id_base="dd-matrix-disabled".to_string() items=vec!["Duplicate".to_string(), "Archive".to_string()] on_action=Callback::new(move |_: usize| {}) is_disabled=Some(true) item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action] motion=DropdownMenuMotion::default() class_name="docs-dropdown-custom".to_string()>
  "Disabled"
</DropdownMenu>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DropdownMenu"
            slug="dropdown-menu"
            group="Collections"
            description="Button trigger that opens a Menu in a Popover with baseline-style root attrs, controlled/uncontrolled state, and persistent-open action handling."
        >
            <Playground
                title="Hello World (Default DropdownMenu)"
                code_signal=code
            >
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-showcase".to_string()
                        items=showcase_items.clone()
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: tune close strategy, control mode, and state markers."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui/src/menu/dropdown_menu/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Items"</div>
                        <SegmentedControl
                            id_base="docs-dropdown-item-mode".to_string()
                            options=item_mode_options.clone()
                            selected_index=interactive_item_mode
                            set_selected_index=set_interactive_item_mode
                            size=SegmentedControlSize::Sm
                            aria_label="Dropdown item mode".to_string()
                        />

                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled trigger"
                        </Switch>
                        <Switch
                            checked=interactive_close_on_action
                            set_checked=set_interactive_close_on_action
                        >
                            "Close on action"
                        </Switch>
                        <Switch checked=interactive_controlled set_checked=set_interactive_controlled>
                            "Controlled open"
                        </Switch>
                        <Switch
                            checked=interactive_with_disabled_items
                            set_checked=set_interactive_with_disabled_items
                        >
                            "Disabled index = [1]"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let item_mode = interactive_item_mode.get().unwrap_or(0);
                    let items = match item_mode {
                        1 => vec!["Rename".to_string(), "Share".to_string()],
                        2 => Vec::<String>::new(),
                        _ => vec![
                            "Duplicate".to_string(),
                            "Move".to_string(),
                            "Archive".to_string(),
                        ],
                    };
                    let disabled_indices = if interactive_with_disabled_items.get() {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let item_kinds = if items.is_empty() {
                        Vec::new()
                    } else {
                        vec![MenuItemKind::Action; items.len()]
                    };
                    let motion = if interactive_custom_motion.get() {
                        DropdownMenuMotion {
                            popover: ui::PopoverMotion {
                                initial_scale: 0.96,
                                offset_y_px: 14.0,
                                ..ui::PopoverMotion::default()
                            },
                        }
                    } else {
                        DropdownMenuMotion::default()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-dropdown-custom".to_string()
                    } else {
                        String::new()
                    };

                    if interactive_controlled.get() {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <DropdownMenu
                                    id_base="docs-dropdown-interactive".to_string()
                                    items=items
                                    on_action=on_interactive_action
                                    disabled=interactive_disabled.get()
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    close_on_action=interactive_close_on_action.get()
                                    open=interactive_open
                                    on_open_change=on_interactive_open_change
                                    motion=motion
                                    class_name=class_name.clone()
                                >
                                    "Interactive"
                                </DropdownMenu>
                                <span class="ui-muted">
                                    "last: "
                                    {move || interactive_last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                                    " · open: "
                                    {move || interactive_open_raw.get()}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <DropdownMenu
                                    id_base="docs-dropdown-interactive".to_string()
                                    items=items
                                    on_action=on_interactive_action
                                    disabled=interactive_disabled.get()
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    close_on_action=interactive_close_on_action.get()
                                    motion=motion
                                    class_name=class_name
                                >
                                    "Interactive"
                                </DropdownMenu>
                                <span class="ui-muted">
                                    "last: "
                                    {move || interactive_last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown".to_string()
                        items=default_playground_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Persistent Open" code_signal=controlled_code>
                <div class="docs-stack">
                    <DropdownMenu
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=controlled_open
                        on_open_change=on_open_change
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                    <span class="ui-muted">"close_on_action: false (select keeps popover open)"</span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    >
                        "Disabled"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </DropdownMenu>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Controlled / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-matrix-default".to_string()
                        items=vec![
                            "Duplicate".to_string(),
                            "Move".to_string(),
                            "Archive".to_string(),
                        ]
                        on_action=on_action
                        placement=ui_headless::PopoverPlacement::BottomStart
                        trigger_variant=ui::ButtonVariant::Secondary
                        trigger_size=ui::ButtonSize::Sm
                    >
                        "Default"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-matrix-controlled".to_string()
                        items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()]
                        on_action=on_action
                        is_open=controlled_open
                        default_open=false
                        on_open_change=on_open_change
                        is_close_on_action=false
                        placement=ui_headless::PopoverPlacement::TopEnd
                        trigger_variant=ui::ButtonVariant::Secondary
                        trigger_size=ui::ButtonSize::Sm
                    >
                        "Controlled"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-matrix-disabled".to_string()
                        items=vec!["Duplicate".to_string(), "Archive".to_string()]
                        on_action=on_action
                        is_disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                        motion=DropdownMenuMotion::default()
                        class_name="docs-dropdown-custom".to_string()
                    >
                        "Disabled"
                    </DropdownMenu>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
