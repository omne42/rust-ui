use super::*;

pub(crate) fn dropdown() -> AnyView {
    let hello_items = vec!["Profile".to_string(), "Settings".to_string()];
    let items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Sign out".to_string(),
    ];
    let items_for_default = items.clone();
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let empty_items: Vec<String> = Vec::new();

    let (last_action, set_last_action) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last_action.set(Some(index)));

    let (open_raw, set_open_raw) = signal(false);
    let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let hello_code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Dropdown
  id_base="profile-dropdown".to_string()
  items=vec!["Profile".to_string(), "Settings".to_string()]
  on_action=on_action
>
  "Open actions"
</Dropdown>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |index: usize| {
  drop(index);});

<Dropdown
  id_base="profile-dropdown".to_string()
  items=vec!["Profile".to_string(), "Settings".to_string(), "Sign out".to_string()]
  on_action=on_action
>
  "Open actions"
</Dropdown>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let on_action = Callback::new(move |index: usize| {
  drop(index);});

<Dropdown
  id_base="controlled-dropdown".to_string()
  items=vec!["Rename".to_string(), "Duplicate".to_string(), "Archive".to_string()]
  on_action=on_action
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  close_on_action=false
  disabled_indices=vec![1]
  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]
  motion=DropdownMotion {
    popover: PopoverMotion { initial_scale: 0.94, offset_y_px: 12.0, ..PopoverMotion::default() },
  }
  class_name="docs-dropdown-custom".to_string()
>
  "Controlled dropdown"
</Dropdown>"#
            .to_string()
    });

    let motion = DropdownMotion {
        popover: PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 12.0,
            ..PopoverMotion::default()
        },
    };
    let workbench_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Sign out".to_string(),
    ];
    let placement_options = vec![
        "bottom-start".to_string(),
        "bottom-end".to_string(),
        "top-start".to_string(),
        "top-end".to_string(),
    ];
    let (workbench_placement_index, set_workbench_placement_index) = signal(Some(0_usize));
    let workbench_placement =
        Signal::derive(move || match workbench_placement_index.get().unwrap_or(0) {
            1 => PopoverPlacement::BottomEnd,
            2 => PopoverPlacement::TopStart,
            3 => PopoverPlacement::TopEnd,
            _ => PopoverPlacement::BottomStart,
        });
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_has_disabled_items, set_workbench_has_disabled_items) = signal(false);
    let (workbench_has_item_kinds, set_workbench_has_item_kinds) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open_signal: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let workbench_on_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let workbench_on_action =
        Callback::new(move |index: usize| set_workbench_last_action.set(Some(index)));

    let workbench_code = Signal::derive(move || {
        let placement = workbench_placement.get();
        let controlled = workbench_controlled.get();
        let is_disabled = workbench_is_disabled.get();
        let close_on_action = workbench_close_on_action.get();
        let has_disabled_items = workbench_has_disabled_items.get();
        let has_item_kinds = workbench_has_item_kinds.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();
        let lang = if workbench_lang_zh.get() {
            "\"zh-CN\""
        } else {
            "\"en-US\""
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        let mut lines = vec![
            "let on_action = Callback::new(move |index: usize| {".to_string(),
            "  drop(index);".to_string(),
            "});".to_string(),
            String::new(),
        ];

        if controlled {
            lines.push("let (open_raw, set_open_raw) = signal(false);".to_string());
            lines.push(
                "let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());"
                    .to_string(),
            );
            lines.push(
                "let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));"
                    .to_string(),
            );
            lines.push(String::new());
        }

        lines.push("<Dropdown".to_string());
        lines.push("  id_base=\"dropdown-workbench\".into()".to_string());
        lines.push(
            "  items=vec![\"Profile\".into(), \"Settings\".into(), \"Sign out\".into()]"
                .to_string(),
        );
        lines.push("  on_action=on_action".to_string());
        lines.push("  is_close_on_action=Some(true)".to_string());
        lines.push(format!("  default_open={}", bool_word(!controlled)));
        lines.push(format!("  lang={lang}.into()"));
        lines.push(format!("  dir={dir}"));
        lines.push("  aria_label=\"Workbench dropdown\".into()".to_string());
        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if !close_on_action {
            lines.push("  close_on_action=false".to_string());
        }
        if controlled {
            lines.push("  is_open=open_signal".to_string());
            lines.push("  on_open_change=on_open_change".to_string());
        }
        if placement != PopoverPlacement::BottomStart {
            lines.push(format!("  placement=PopoverPlacement::{placement:?}"));
        }
        if has_disabled_items {
            lines.push("  disabled_indices=vec![1]".to_string());
        }
        if has_item_kinds {
            lines.push(
                "  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]".to_string(),
            );
        }
        if custom_motion {
            lines.push("  motion=DropdownMotion {".to_string());
            lines.push(
                "    popover: PopoverMotion { initial_scale: 0.92, offset_y_px: 14.0, ..PopoverMotion::default() },".to_string(),
            );
            lines.push("  }".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-dropdown-workbench\".into()".to_string());
        }

        lines.push(">".to_string());
        lines.push("  \"Workbench dropdown\"".to_string());
        lines.push("</Dropdown>".to_string());

        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/dropdown/styles.rs */\n{}",
            ui::dropdown::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = workbench_placement.get();
        let controlled = workbench_controlled.get();
        let is_disabled = workbench_is_disabled.get();
        let close_on_action = workbench_close_on_action.get();
        let has_disabled_items = workbench_has_disabled_items.get();
        let has_item_kinds = workbench_has_item_kinds.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();
        let open = workbench_open_raw.get();
        let last_action = workbench_last_action.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let disabled_indices = if has_disabled_items {
            "vec![1]"
        } else {
            "vec![]"
        };
        let item_kinds = if has_item_kinds {
            "vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]"
        } else {
            "vec![]"
        };

        let mut classes = vec!["ui-dropdown".to_string()];
        if is_disabled {
            classes.push("ui-dropdown--disabled".to_string());
        }
        if close_on_action {
            classes.push("ui-dropdown--has-items".to_string());
        } else {
            classes.push("ui-dropdown--persistent".to_string());
        }
        if controlled {
            classes.push("ui-dropdown--controlled".to_string());
        }
        if custom_class {
            classes.push("ui-dropdown--custom-class".to_string());
            classes.push("docs-dropdown-workbench".to_string());
        }

        format!(
            "DropdownActualConfig {{\n  id_base: \"docs-dropdown-workbench\",\n  items: [\"Profile\", \"Settings\", \"Sign out\"],\n  on_action: Some(\"workbench_on_action\"),\n  is_disabled: Some({is_disabled}),\n  disabled: {is_disabled},\n  disabled_indices: {disabled_indices},\n  item_kinds: {item_kinds},\n  is_close_on_action: Some({close_on_action}),\n  close_on_action: {close_on_action},\n  placement: Some({placement:?}),\n  is_open: {is_open},\n  open: {open},\n  default_open: Some({default_open}),\n  on_open_change: {on_open_change},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  motion: {motion},\n  aria_label: Some(\"Workbench dropdown\"),\n  class_name: {class_name},\n  controlled: {controlled},\n  has_disabled_items: {has_disabled_items},\n  has_item_kinds: {has_item_kinds},\n  custom_motion: {custom_motion},\n  class: {class_tokens:?},\n  last_action: {last_action},\n}}",
            is_open = if controlled {
                "Some(workbench_open_signal)"
            } else {
                "None"
            },
            default_open = !controlled,
            on_open_change = if controlled {
                "Some(\"workbench_on_open_change\")"
            } else {
                "None"
            },
            motion = if custom_motion {
                "DropdownMotion::custom"
            } else {
                "DropdownMotion::default"
            },
            class_name = if custom_class {
                "Some(\"docs-dropdown-workbench\")"
            } else {
                "None"
            },
            class_tokens = classes.join(" "),
            last_action = last_action
                .map(|index| index.to_string())
                .unwrap_or_else(|| "None".to_string()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let items = vec![
  "Profile".to_string(),
  "Settings".to_string(),
  "Sign out".to_string(),
];
let on_action = Callback::new(move |_: usize| {});
let (open, set_open) = signal(false);

<div class="docs-row">
  <Dropdown id_base="matrix-default".to_string() items=items.clone() on_action=on_action>
    "Default"
  </Dropdown>
  <Dropdown
    id_base="matrix-controlled".to_string()
    items=items.clone()
    on_action=on_action
    is_open=Signal::derive(move || open.get())
    on_open_change=Callback::new(move |next| set_open.set(next))
    close_on_action=false
  >
    "Controlled"
  </Dropdown>
  <Dropdown id_base="matrix-disabled".to_string() items=items.clone() on_action=on_action is_disabled=true>
    "Disabled"
  </Dropdown>
  <Dropdown id_base="matrix-empty".to_string() items=Vec::<String>::new() on_action=on_action>
    "Empty"
  </Dropdown>
</div>"#.to_string()
    });
    let (matrix_open_raw, set_matrix_open_raw) = signal(false);
    let matrix_open_signal: Signal<bool> = Signal::derive(move || matrix_open_raw.get());
    let matrix_on_open_change = Callback::new(move |next: bool| set_matrix_open_raw.set(next));
    let (matrix_last_action, set_matrix_last_action) = signal(None::<usize>);
    let matrix_on_action =
        Callback::new(move |index: usize| set_matrix_last_action.set(Some(index)));

    view! {
        <ComponentPage
            title="Dropdown"
            slug="dropdown"
            group="Collections"
            description="baseline-style dropdown trigger primitive with centralized state/source contracts, controllable open state, and spring-tuned popover motion."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row" data-slot="dropdown-hello-world">
                    <Dropdown
                        id_base="docs-dropdown-hello".to_string()
                        items=hello_items
                        on_action=on_action
                    >
                        "Open actions"
                    </Dropdown>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 四区合一：用于快速比对 controlled、placement、motion 与状态来源标记。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/menu/dropdown/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Placement"</div>
                        <SegmentedControl
                            id_base="docs-dropdown-workbench-placement".to_string()
                            options=placement_options.clone()
                            selected_index=workbench_placement_index
                            set_selected_index=set_workbench_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Dropdown placement".to_string()
                        />
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_close_on_action set_checked=set_workbench_close_on_action>
                            "Close on action"
                        </Switch>
                        <Switch
                            checked=workbench_has_disabled_items
                            set_checked=set_workbench_has_disabled_items
                        >
                            "Disabled item"
                        </Switch>
                        <Switch checked=workbench_has_item_kinds set_checked=set_workbench_has_item_kinds>
                            "Item kinds"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let placement = workbench_placement.get();
                    let controlled = workbench_controlled.get();
                    let is_disabled = workbench_is_disabled.get();
                    let close_on_action = workbench_close_on_action.get();
                    let has_disabled_items = workbench_has_disabled_items.get();
                    let has_item_kinds = workbench_has_item_kinds.get();
                    let custom_motion = workbench_custom_motion.get();
                    let custom_class = workbench_custom_class.get();
                    let disabled_indices = if has_disabled_items {
                        vec![1]
                    } else {
                        Vec::new()
                    };
                    let item_kinds = if has_item_kinds {
                        vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    } else {
                        Vec::new()
                    };
                    let motion = if custom_motion {
                        DropdownMotion {
                            popover: PopoverMotion {
                                initial_scale: 0.92,
                                offset_y_px: 14.0,
                                ..PopoverMotion::default()
                            },
                        }
                    } else {
                        DropdownMotion::default()
                    };
                    let class_name = if custom_class {
                        "docs-dropdown-workbench".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if workbench_lang_zh.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    };
                    let dir = if workbench_rtl_dir.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    if controlled {
                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="dropdown-workbench-preview">
                                <Dropdown
                                    id_base="docs-dropdown-workbench".to_string()
                                    items=workbench_items.clone()
                                    on_action=workbench_on_action
                                    is_disabled=is_disabled
                                    close_on_action=close_on_action
                                    is_close_on_action=close_on_action
                                    placement=placement
                                    is_open=workbench_open_signal
                                    on_open_change=workbench_on_open_change
                                    default_open=false
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    motion=motion
                                    aria_label="Workbench dropdown".to_string()
                                    lang=lang.clone()
                                    dir=dir
                                    class_name=class_name
                                >
                                    "Workbench dropdown"
                                </Dropdown>
                                <span class="ui-muted">
                                    "open: "
                                    {workbench_open_raw.get()}
                                    " · last action: "
                                    {workbench_last_action
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="dropdown-workbench-preview">
                                <Dropdown
                                    id_base="docs-dropdown-workbench".to_string()
                                    items=workbench_items.clone()
                                    on_action=workbench_on_action
                                    is_disabled=is_disabled
                                    close_on_action=close_on_action
                                    is_close_on_action=close_on_action
                                    placement=placement
                                    default_open=true
                                    disabled_indices=disabled_indices
                                    item_kinds=item_kinds
                                    motion=motion
                                    aria_label="Workbench dropdown".to_string()
                                    lang=lang
                                    dir=dir
                                    class_name=class_name
                                >
                                    "Workbench dropdown"
                                </Dropdown>
                                <span class="ui-muted">
                                    "open: internal (uncontrolled)"
                                    " · last action: "
                                    {workbench_last_action
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="State Matrix Compare" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="dropdown-state-matrix">
                    <div class="docs-row">
                        <div class="docs-card">
                            <h2>"Default"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-default".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                            >
                                "Default"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Controlled + Persistent"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-controlled".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                                is_open=matrix_open_signal
                                on_open_change=matrix_on_open_change
                                close_on_action=false
                            >
                                "Controlled"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Disabled"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-disabled".to_string()
                                items=items.clone()
                                on_action=matrix_on_action
                                is_disabled=true
                            >
                                "Disabled"
                            </Dropdown>
                        </div>

                        <div class="docs-card">
                            <h2>"Empty"</h2>
                            <Dropdown
                                id_base="docs-dropdown-compare-empty".to_string()
                                items=empty_items.clone()
                                on_action=matrix_on_action
                            >
                                "Empty"
                            </Dropdown>
                        </div>
                    </div>

                    <span class="ui-muted">
                        "controlled open: "
                        {move || matrix_open_raw.get()}
                        " · last action: "
                        {move || {
                            matrix_last_action
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Default" code_signal=code>
                <div class="docs-row" data-slot="dropdown-default-playground">
                    <Dropdown
                        id_base="docs-dropdown-default".to_string()
                        items=items_for_default.clone()
                        on_action=on_action
                    >
                        "Open actions"
                    </Dropdown>
                    <span class="ui-muted" data-slot="dropdown-last-action">
                        "last action: "
                        {move || {
                            last_action
                                .get()
                                .map(|idx| idx.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Persistent + Motion" code_signal=states_code>
                <div class="docs-stack" data-slot="dropdown-controlled-playground">
                    <Dropdown
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=open_signal
                        on_open_change=on_open_change
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                        motion=motion
                        class_name="docs-dropdown-custom".to_string()
                    >
                        "Controlled dropdown"
                    </Dropdown>
                    <span class="ui-muted" data-slot="dropdown-controlled-open">
                        "open: "
                        {move || open_raw.get()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
