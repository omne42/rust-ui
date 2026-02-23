use super::*;

pub(crate) fn list_item() -> AnyView {
    let (showcase_selected, set_showcase_selected) = signal(true);
    let showcase_on_press = Callback::new(move |_| {
        set_showcase_selected.update(|value| *value = !*value);
    });

    let (workbench_index, set_workbench_index) = signal(1_usize);
    let (workbench_selected, set_workbench_selected) = signal(true);
    let (workbench_focused, set_workbench_focused) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_indicator_visible, set_workbench_indicator_visible) = signal(true);
    let (workbench_divider_visible, set_workbench_divider_visible) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_selected_text, set_workbench_custom_selected_text) = signal(false);
    let (workbench_custom_unselected_text, set_workbench_custom_unselected_text) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let (workbench_pointer_move_count, set_workbench_pointer_move_count) = signal(0_u32);

    let workbench_on_press = Callback::new(move |_| {
        set_workbench_selected.update(|value| *value = !*value);
        set_workbench_press_count.update(|count| *count += 1);
    });
    let workbench_on_pointer_move = Callback::new(move |_| {
        set_workbench_pointer_move_count.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<ListItem index=0 is_selected=true is_selection_indicator_visible=true>
  "San Francisco"
</ListItem>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ListItem\n  id=\"docs-list-item-workbench\".to_string()\n  index={}\n  is_selected={}\n  is_focused={}\n  is_disabled={}\n  is_selection_indicator_visible={}\n  is_divider_visible={}\n  aria_label={}\n  selected_text={}\n  unselected_text={}\n  on_press=on_press\n  on_pointer_move=on_pointer_move\n  class_name={}\n>\n  \"Tokyo\"\n</ListItem>",
            workbench_index.get(),
            bool_word(workbench_selected.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_indicator_visible.get()),
            bool_word(workbench_divider_visible.get()),
            rust_string_literal(if workbench_custom_aria.get() {
                "Tokyo option"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_selected_text.get() {
                "Selected"
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_unselected_text.get() {
                "Not selected"
            } else {
                ""
            }),
            if workbench_custom_class.get() {
                "\"docs-list-item-custom\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ListItemWorkbenchActualConfig {{\n  id: Some(\"docs-list-item-workbench\"),\n  index: Some({}),\n  is_selected: {},\n  is_focused: {},\n  is_disabled: {},\n  is_selection_indicator_visible: {},\n  is_divider_visible: {},\n  aria_label: {:?},\n  selected_text: {:?},\n  unselected_text: {:?},\n  on_press: \"count={}\",\n  on_pointer_move: \"count={}\",\n  class_name: {:?},\n}}",
            workbench_index.get(),
            bool_word(workbench_selected.get()),
            bool_word(workbench_focused.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_indicator_visible.get()),
            bool_word(workbench_divider_visible.get()),
            if workbench_custom_aria.get() {
                Some("Tokyo option")
            } else {
                None
            },
            if workbench_custom_selected_text.get() {
                Some("Selected")
            } else {
                None
            },
            if workbench_custom_unselected_text.get() {
                Some("Not selected")
            } else {
                None
            },
            workbench_press_count.get(),
            workbench_pointer_move_count.get(),
            if workbench_custom_class.get() {
                Some("docs-list-item-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ListItem id="li-default".to_string() index=0 is_selected=true is_selection_indicator_visible=true>"San Francisco"</ListItem>
<ListItem id="li-focused".to_string() index=1 is_focused=true is_divider_visible=true class_name="docs-listbox-item-custom".to_string()>"Tokyo"</ListItem>
<ListItem id="li-disabled".to_string() index=2 is_disabled=true aria_label="Disabled option".to_string()>"Disabled option"</ListItem>"#.to_string()
    });

    view! {
        <ComponentPage
            title="ListItem"
            slug="list-item"
            group="Collections"
            description="baseline-style list option primitive with centralized selection/focus/divider/source normalization and stable `slot` + `data-*` state contracts."
        >
            <Playground title="Hello World (Default ListItem)" code_signal=hello_code>
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-hello".to_string()
                        index=0
                        is_selected=showcase_selected.get()
                        is_selection_indicator_visible=true
                        on_press=showcase_on_press
                    >
                        "San Francisco"
                    </ListItem>
                    <span class="ui-muted">
                        "selected: "
                        {move || showcase_selected.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-item-workbench-controls">
                        <label class="docs-search__label">
                            "index"
                            <input
                                type="number"
                                min="0"
                                max="12"
                                prop:value=move || workbench_index.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<usize>().unwrap_or(1);
                                    set_workbench_index.set(next);
                                }
                            />
                        </label>
                        <Switch checked=workbench_selected set_checked=set_workbench_selected>
                            "is_selected"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "is_focused"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_indicator_visible set_checked=set_workbench_indicator_visible>
                            "is_selection_indicator_visible"
                        </Switch>
                        <Switch checked=workbench_divider_visible set_checked=set_workbench_divider_visible>
                            "is_divider_visible"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_selected_text set_checked=set_workbench_custom_selected_text>
                            "selected_text"
                        </Switch>
                        <Switch checked=workbench_custom_unselected_text set_checked=set_workbench_custom_unselected_text>
                            "unselected_text"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-workbench".to_string()
                        index=workbench_index.get()
                        is_selected=workbench_selected.get()
                        is_focused=workbench_focused.get()
                        is_disabled=workbench_disabled.get()
                        is_divider_visible=workbench_divider_visible.get()
                        is_selection_indicator_visible=workbench_indicator_visible.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Tokyo option".to_string()
                        } else {
                            String::new()
                        }
                        selected_text=if workbench_custom_selected_text.get() {
                            "Selected".to_string()
                        } else {
                            String::new()
                        }
                        unselected_text=if workbench_custom_unselected_text.get() {
                            "Not selected".to_string()
                        } else {
                            String::new()
                        }
                        on_press=workbench_on_press
                        on_pointer_move=workbench_on_pointer_move
                        class_name=if workbench_custom_class.get() {
                            "docs-listbox-item-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        "Tokyo"
                    </ListItem>
                    <span class="ui-muted">
                        "selected: " {move || workbench_selected.get()}
                        " · on_press count: " {move || workbench_press_count.get()}
                        " · on_pointer_move count: " {move || workbench_pointer_move_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Selected / Focused / Disabled)" code_signal=matrix_code>
                <div class="docs-stack">
                    <ListItem
                        id="docs-list-item-matrix-default".to_string()
                        index=0
                        is_selected=true
                        is_selection_indicator_visible=true
                    >
                        "San Francisco"
                    </ListItem>
                    <ListItem
                        id="docs-list-item-matrix-focused".to_string()
                        index=1
                        is_focused=true
                        is_divider_visible=true
                        class_name="docs-listbox-item-custom".to_string()
                    >
                        "Tokyo"
                    </ListItem>
                    <ListItem
                        id="docs-list-item-matrix-disabled".to_string()
                        index=2
                        is_disabled=true
                        aria_label="Disabled option".to_string()
                    >
                        "Disabled option"
                    </ListItem>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
