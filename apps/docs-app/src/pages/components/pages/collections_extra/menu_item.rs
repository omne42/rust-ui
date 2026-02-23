use super::*;

pub(crate) fn menu_item() -> AnyView {
    let (showcase_checked, set_showcase_checked) = signal(true);
    let showcase_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || showcase_checked.get()),
    };
    let (showcase_pointer_moves, set_showcase_pointer_moves) = signal(0_u32);
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let on_showcase_pointer_move = Callback::new(move |_| {
        set_showcase_pointer_moves.update(|count| *count += 1);
    });
    let on_showcase_press = Callback::new(move |_| {
        set_showcase_presses.update(|count| *count += 1);
        set_showcase_checked.update(|value| *value = !*value);
    });

    let kind_options = vec![
        "Action".to_string(),
        "Checkbox".to_string(),
        "Radio".to_string(),
    ];
    let index_options = vec!["0".to_string(), "2".to_string()];
    let is_disabled_options = vec!["false".to_string(), "true".to_string()];

    let (workbench_kind_index, set_workbench_kind_index) = signal(Some(0_usize));
    let (workbench_index_mode, set_workbench_index_mode) = signal(Some(0_usize));
    let (workbench_is_disabled_mode, set_workbench_is_disabled_mode) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(false);
    let (workbench_has_submenu, set_workbench_has_submenu) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_checkbox_checked, set_workbench_checkbox_checked) = signal(true);
    let (workbench_radio_checked, set_workbench_radio_checked) = signal(false);
    let workbench_checkbox_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || workbench_checkbox_checked.get()),
    };
    let workbench_radio_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || workbench_radio_checked.get()),
    };
    let workbench_kind = Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
        1 => workbench_checkbox_kind,
        2 => workbench_radio_kind,
        _ => MenuItemKind::Action,
    });
    let workbench_kind_name =
        Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
            1 => "Checkbox",
            2 => "Radio",
            _ => "Action",
        });
    let workbench_index = Signal::derive(move || match workbench_index_mode.get().unwrap_or(0) {
        1 => 2_usize,
        _ => 0_usize,
    });
    let workbench_is_disabled =
        Signal::derive(move || matches!(workbench_is_disabled_mode.get().unwrap_or(0), 1));
    let (workbench_pointer_moves, set_workbench_pointer_moves) = signal(0_u32);
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let (workbench_last_event, set_workbench_last_event) = signal("none".to_string());
    let on_workbench_pointer_move = Callback::new(move |_| {
        set_workbench_pointer_moves.update(|count| *count += 1);
        set_workbench_last_event.set("pointer_move".to_string());
    });
    let on_workbench_press = Callback::new(move |_| {
        set_workbench_presses.update(|count| *count += 1);
        match workbench_kind_index.get().unwrap_or(0) {
            1 => set_workbench_checkbox_checked.update(|value| *value = !*value),
            2 => set_workbench_radio_checked.update(|value| *value = !*value),
            _ => {}
        }
        set_workbench_last_event.set("press".to_string());
    });

    let hello_code = Signal::derive(move || {
        r#"<MenuItem
  id="docs-menu-item-showcase".to_string()
  index=0
  kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) }
  aria_label="Pin project".to_string()
  on_pointer_move=on_pointer_move
  on_press=on_press
>
  "Pin project"
</MenuItem>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let kind_expr = match workbench_kind_index.get().unwrap_or(0) {
            1 => {
                "MenuItemKind::Checkbox { is_checked: Signal::derive(move || checkbox_checked.get()) }"
            }
            2 => "MenuItemKind::Radio { is_checked: Signal::derive(move || radio_checked.get()) }",
            _ => "MenuItemKind::Action",
        };
        let id_expr = if workbench_custom_id.get() {
            "\"docs-menu-item-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let aria_expr = if workbench_custom_aria.get() {
            "\"Workbench menu item\".to_string()"
        } else {
            "String::new()"
        };
        let class_expr = if workbench_custom_class.get() {
            "\"docs-menu-item-custom\".to_string()"
        } else {
            "String::new()"
        };

        [
            "<MenuItem".to_string(),
            format!("  id={id_expr}"),
            format!("  index={}", workbench_index.get()),
            format!("  kind={kind_expr}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!("  focused={}", bool_word(workbench_focused.get())),
            format!("  has_submenu={}", bool_word(workbench_has_submenu.get())),
            format!("  aria_label={aria_expr}"),
            "  on_pointer_move=on_pointer_move".to_string(),
            "  on_press=on_press".to_string(),
            format!("  class_name={class_expr}"),
            ">".to_string(),
            "  \"Workbench menu item\"".to_string(),
            "</MenuItem>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let id = if workbench_custom_id.get() {
            "docs-menu-item-workbench"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Workbench menu item"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-menu-item-custom"
        } else {
            ""
        };

        format!(
            "MenuItemActualConfig {{\n  id: {:?},\n  index: Some({}),\n  kind: {:?},\n  is_disabled: Some({}),\n  disabled: {},\n  focused: {},\n  has_submenu: {},\n  aria_label: {:?},\n  on_pointer_move: \"count={}\",\n  on_press: \"count={}, checkbox_checked={}, radio_checked={}\",\n  class_name: {:?},\n}}",
            Some(id),
            workbench_index.get(),
            workbench_kind_name.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_has_submenu.get()),
            Some(aria_label),
            workbench_pointer_moves.get(),
            workbench_presses.get(),
            workbench_checkbox_checked.get(),
            workbench_radio_checked.get(),
            Some(class_name),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MenuItem index=0 kind=MenuItemKind::Action aria_label="Open profile".to_string()>
  "Open profile"
</MenuItem>
<MenuItem index=1 kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) } focused=true on_press=on_press>
  "Pin workspace"
</MenuItem>
<MenuItem index=2 kind=MenuItemKind::Radio { is_checked: Signal::derive(move || selected.get()) } is_disabled=true disabled=true has_submenu=true class_name="docs-menu-item-custom".to_string()>
  "Primary workspace"
</MenuItem>"#
            .to_string()
    });

    let (matrix_checked, set_matrix_checked) = signal(true);
    let matrix_checkbox_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || matrix_checked.get()),
    };
    let (matrix_radio_selected, _set_matrix_radio_selected) = signal(true);
    let matrix_radio_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || matrix_radio_selected.get()),
    };
    let (matrix_pointer_moves, set_matrix_pointer_moves) = signal(0_u32);
    let (matrix_presses, set_matrix_presses) = signal(0_u32);
    let on_matrix_pointer_move = Callback::new(move |_| {
        set_matrix_pointer_moves.update(|count| *count += 1);
    });
    let on_matrix_press = Callback::new(move |_| {
        set_matrix_presses.update(|count| *count += 1);
        set_matrix_checked.update(|value| *value = !*value);
    });

    view! {
        <ComponentPage
            title="MenuItem"
            slug="menu-item"
            group="Collections"
            description="baseline-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Hello World (Default MenuItem + Feedback)" code_signal=hello_code>
                <div class="docs-stack">
                    <MenuItem
                        id="docs-menu-item-showcase".to_string()
                        index=0
                        kind=showcase_kind
                        aria_label="Pin project".to_string()
                        on_pointer_move=on_showcase_pointer_move
                        on_press=on_showcase_press
                    >
                        "Pin project"
                    </MenuItem>
                    <span class="ui-muted">
                        "checked: " {move || showcase_checked.get()}
                        " · pointer moves: " {move || showcase_pointer_moves.get()}
                        " · presses: " {move || showcase_presses.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-item-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-kind".to_string()
                            options=kind_options.clone()
                            selected_index=workbench_kind_index
                            set_selected_index=set_workbench_kind_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem kind".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-index".to_string()
                            options=index_options.clone()
                            selected_index=workbench_index_mode
                            set_selected_index=set_workbench_index_mode
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem index".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-item-workbench-is-disabled".to_string()
                            options=is_disabled_options.clone()
                            selected_index=workbench_is_disabled_mode
                            set_selected_index=set_workbench_is_disabled_mode
                            size=SegmentedControlSize::Sm
                            aria_label="MenuItem is_disabled".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "focused"
                        </Switch>
                        <Switch checked=workbench_has_submenu set_checked=set_workbench_has_submenu>
                            "has_submenu"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <MenuItem
                        id=if workbench_custom_id.get() {
                            "docs-menu-item-workbench".to_string()
                        } else {
                            String::new()
                        }
                        index=workbench_index.get()
                        kind=workbench_kind.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        focused=workbench_focused.get()
                        has_submenu=workbench_has_submenu.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Workbench menu item".to_string()
                        } else {
                            String::new()
                        }
                        on_pointer_move=on_workbench_pointer_move
                        on_press=on_workbench_press
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-item-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        {move || format!("{} workbench item", workbench_kind_name.get())}
                    </MenuItem>
                    <span class="ui-muted">
                        "on_pointer_move count: " {move || workbench_pointer_moves.get()}
                        " · on_press count: " {move || workbench_presses.get()}
                        " · last event: " {move || workbench_last_event.get()}
                        " · checkbox checked: " {move || workbench_checkbox_checked.get()}
                        " · radio checked: " {move || workbench_radio_checked.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Action / Checkbox / Disabled Radio)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <MenuItem
                        index=0
                        kind=MenuItemKind::Action
                        aria_label="Open profile".to_string()
                    >
                        "Open profile"
                    </MenuItem>
                    <MenuItem
                        index=1
                        kind=matrix_checkbox_kind
                        focused=true
                        on_pointer_move=on_matrix_pointer_move
                        on_press=on_matrix_press
                    >
                        "Pin workspace"
                    </MenuItem>
                    <MenuItem
                        index=2
                        kind=matrix_radio_kind
                        is_disabled=true
                        disabled=true
                        has_submenu=true
                        class_name="docs-menu-item-custom".to_string()
                    >
                        "Primary workspace"
                    </MenuItem>
                    <span class="ui-muted">
                        "matrix checkbox checked: " {move || matrix_checked.get()}
                        " · pointer moves: " {move || matrix_pointer_moves.get()}
                        " · presses: " {move || matrix_presses.get()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
