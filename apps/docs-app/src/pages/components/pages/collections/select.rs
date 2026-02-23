use super::*;

pub(crate) fn select() -> AnyView {
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (selected_index_raw, set_selected_index_raw) = signal(Some(1_usize));
    let selected_index: ReadSignal<Option<usize>> = selected_index_raw;
    let set_selected_index: WriteSignal<Option<usize>> = set_selected_index_raw;

    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (on_open_change_runs, set_on_open_change_runs) = signal(0_u32);
    let on_open_change = Callback::new(move |next: bool| {
        set_open_raw.set(next);
        set_on_open_change_runs.update(|count| *count += 1);
    });

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_place_top, set_workbench_place_top) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<Select
  id_base="docs-select-hello".to_string()
  items=vec!["Apple".to_string(), "Banana".to_string()]
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            "docs-select-custom"
        } else {
            ""
        };
        let dir = if workbench_rtl.get() {
            "ui_headless::A11yDirection::Rtl"
        } else {
            "ui_headless::A11yDirection::Ltr"
        };
        [
            "<Select".to_string(),
            "  id_base=\"docs-select-workbench\".to_string()".to_string(),
            "  items=vec![\"Apple\".to_string(), \"Banana\".to_string(), \"Cherry\".to_string(), \"Durian\".to_string()]".to_string(),
            "  selected_index=selected_index".to_string(),
            "  set_selected_index=set_selected_index".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            "  placeholder=\"Select fruit\".to_string()".to_string(),
            format!(
                "  disabled_indices={}",
                if workbench_disable_last.get() {
                    "vec![3]"
                } else {
                    "Vec::<usize>::new()"
                }
            ),
            format!(
                "  placement={}",
                if workbench_place_top.get() {
                    "PopoverPlacement::TopStart"
                } else {
                    "PopoverPlacement::BottomStart"
                }
            ),
            "  open=open".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=on_open_change".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "  motion=ui::select::SelectMotion::default()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-select-custom")
        } else {
            None
        };
        format!(
            "SelectActualConfig {{\n  id_base: \"docs-select-workbench\",\n  items: [\"Apple\", \"Banana\", \"Cherry\", \"Durian\"],\n  selected_index: {:?},\n  set_selected_index: \"WriteSignal<Option<usize>>\",\n  is_disabled: Some({}),\n  disabled: Some({}),\n  placeholder: Some(\"Select fruit\"),\n  disabled_indices: {},\n  placement: {},\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"runs={}\",\n  lang: Some(\"en-US\"),\n  dir: {},\n  motion: SelectMotion::default(),\n  class_name: {class_name:?},\n}}",
            selected_index_raw.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            if workbench_disable_last.get() {
                "vec![3]"
            } else {
                "vec![]"
            },
            if workbench_place_top.get() {
                "PopoverPlacement::TopStart"
            } else {
                "PopoverPlacement::BottomStart"
            },
            bool_word(open_raw.get()),
            on_open_change_runs.get(),
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Select id_base="select-default".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected />
<Select id_base="select-top".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected placement=PopoverPlacement::TopStart open=Signal::derive(|| false) default_open=false />
<Select id_base="select-disabled".to_string() items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()] selected_index=selected set_selected_index=set_selected is_disabled=true disabled=true />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select playground with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default Select)" code_signal=hello_code>
                <Select
                    id_base="docs-select-hello".to_string()
                    items=showcase_items
                    selected_index=selected_index
                    set_selected_index=set_selected_index
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="select-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_disable_last set_checked=set_workbench_disable_last>
                            "Disable last option"
                        </Switch>
                        <Switch checked=workbench_place_top set_checked=set_workbench_place_top>
                            "Top placement"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_open_raw.update(|value| *value = !*value))
                        >
                            "Toggle open signal"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="select-workbench-preview">
                    <Select
                        id_base="docs-select-workbench".to_string()
                        items=workbench_items
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        placeholder="Select fruit".to_string()
                        disabled_indices=if workbench_disable_last.get() {
                            vec![3]
                        } else {
                            Vec::new()
                        }
                        placement=if workbench_place_top.get() {
                            PopoverPlacement::TopStart
                        } else {
                            PopoverPlacement::BottomStart
                        }
                        open=open
                        default_open=false
                        on_open_change=on_open_change
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        motion=ui::select::SelectMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-select-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="select-workbench-feedback">
                        "open: " {move || open_raw.get()}
                        " · on_open_change: " {move || on_open_change_runs.get()}
                        " · selected_index: "
                        {move || selected_index_raw.get().map_or_else(|| "None".to_string(), |it| it.to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Top / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="select-state-matrix">
                    <Select
                        id_base="docs-select-matrix-default".to_string()
                        items=matrix_items.clone()
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                    />
                    <Select
                        id_base="docs-select-matrix-top".to_string()
                        items=matrix_items.clone()
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        placement=PopoverPlacement::TopStart
                        open=open
                        default_open=false
                    />
                    <Select
                        id_base="docs-select-matrix-disabled".to_string()
                        items=matrix_items
                        selected_index=selected_index
                        set_selected_index=set_selected_index
                        is_disabled=true
                        disabled=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
