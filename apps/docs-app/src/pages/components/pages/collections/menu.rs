use super::*;

pub(crate) fn menu() -> AnyView {
    let hello_item_specs = vec![
        MenuItemSpec::action("New file"),
        MenuItemSpec::action("Share with team"),
    ];
    let workbench_items: Arc<[String]> = vec![
        "New file".to_string(),
        "Share with team".to_string(),
        "Sort ascending".to_string(),
    ]
    .into();
    let workbench_item_specs = vec![
        MenuItemSpec::action("New file"),
        MenuItemSpec::action("Share with team"),
        MenuItemSpec::action("Sort ascending"),
    ];

    let (showcase_last_action, set_showcase_last_action) = signal(None::<usize>);
    let on_showcase_action =
        Callback::new(move |index: usize| set_showcase_last_action.set(Some(index)));

    let default_index_options = vec!["0".to_string(), "1".to_string(), "2".to_string()];
    let (workbench_default_index, set_workbench_default_index) = signal(Some(0_usize));
    let (workbench_use_labelledby, set_workbench_use_labelledby) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_share_checked, set_workbench_share_checked) = signal(true);
    let (workbench_sort_ascending, set_workbench_sort_ascending) = signal(true);
    let (workbench_action_count, set_workbench_action_count) = signal(0_u32);
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let on_workbench_action = Callback::new(move |index: usize| {
        set_workbench_action_count.update(|count| *count += 1);
        set_workbench_last_action.set(Some(index));
        match index {
            1 => set_workbench_share_checked.update(|value| *value = !*value),
            2 => set_workbench_sort_ascending.update(|value| *value = !*value),
            _ => {}
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<Menu
  id_base="menu-hello".to_string()
  item_specs=vec![MenuItemSpec::action("New file"), MenuItemSpec::action("Share with team")]
  on_action=Callback::new(move |_: usize| {})
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let default_index = workbench_default_index.get().unwrap_or(0).min(2);
        let disabled_indices = if workbench_disable_second.get() {
            "vec![1]".to_string()
        } else {
            "Vec::<usize>::new()".to_string()
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-menu-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let aria_label = if workbench_use_labelledby.get() {
            "String::new()"
        } else {
            "\"Workbench menu actions\".to_string()"
        };
        let aria_labelledby = if workbench_use_labelledby.get() {
            "\"docs-menu-workbench-heading\".to_string()"
        } else {
            "String::new()"
        };

        format!(
            "<Menu\n  id_base=\"docs-menu-workbench\".to_string()\n  items=vec![\"New file\".to_string(), \"Share with team\".to_string(), \"Sort ascending\".to_string()].into()\n  on_action=on_action\n  item_specs=vec![\n    MenuItemSpec::action(\"New file\"),\n    MenuItemSpec::action(\"Share with team\"),\n    MenuItemSpec::action(\"Sort ascending\"),\n  ]\n  id=\"docs-menu-workbench-root\".to_string()\n  aria_label={aria_label}\n  aria_labelledby={aria_labelledby}\n  is_disabled=Some({})\n  disabled={}\n  disabled_indices={disabled_indices}\n  item_kinds=vec![\n    MenuItemKind::Action,\n    MenuItemKind::Checkbox {{ is_checked: Signal::derive(move || share_checked.get()) }},\n    MenuItemKind::Radio {{ is_checked: Signal::derive(move || sort_ascending.get()) }},\n  ]\n  default_index={default_index}\n  motion=MenuMotion::default()\n  class_name={class_name}\n/>",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let default_index = workbench_default_index.get().unwrap_or(0).min(2);
        let disabled_indices = if workbench_disable_second.get() {
            vec![1_usize]
        } else {
            Vec::new()
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-menu-workbench")
        } else {
            None
        };
        let aria_label = if workbench_use_labelledby.get() {
            None
        } else {
            Some("Workbench menu actions")
        };
        let aria_labelledby = if workbench_use_labelledby.get() {
            Some("docs-menu-workbench-heading")
        } else {
            None
        };
        format!(
            "MenuActualConfig {{\n  id_base: \"docs-menu-workbench\",\n  items: [\"New file\", \"Share with team\", \"Sort ascending\"],\n  on_action: \"count={} last={:?}\",\n  item_specs: [\"action(New file)\", \"action(Share with team)\", \"action(Sort ascending)\"],\n  id: Some(\"docs-menu-workbench-root\"),\n  aria_label: {aria_label:?},\n  aria_labelledby: {aria_labelledby:?},\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {disabled_indices:?},\n  item_kinds: [\"Action\", \"Checkbox\", \"Radio\"],\n  default_index: {default_index},\n  motion: MenuMotion::default(),\n  class_name: {class_name:?},\n}}",
            workbench_action_count.get(),
            workbench_last_action.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Menu id_base="menu-default".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) item_specs=vec![MenuItemSpec::action("New file"), MenuItemSpec::action("Share with team"), MenuItemSpec::action("Sort ascending")] default_index=1 />
<Menu id_base="menu-labelledby".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) aria_labelledby="menu-matrix-heading".to_string() disabled_indices=vec![1] />
<Menu id_base="menu-disabled".to_string() items=vec!["New file".to_string(), "Share with team".to_string(), "Sort ascending".to_string()].into() on_action=Callback::new(move |_: usize| {}) is_disabled=Some(true) disabled=true class_name="docs-menu-workbench".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with action / checkbox / radio kinds, full API workbench config, and callback feedback."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">"最小默认路径：仅 `id_base + item_specs + on_action`"</span>
                    <Menu
                        id_base="docs-menu-hello".to_string()
                        item_specs=hello_item_specs
                        on_action=on_showcase_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || showcase_last_action.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-workbench-default-index".to_string()
                            options=default_index_options.clone()
                            selected_index=workbench_default_index
                            set_selected_index=set_workbench_default_index
                            size=SegmentedControlSize::Sm
                            aria_label="Menu default index".to_string()
                        />
                        <Switch checked=workbench_use_labelledby set_checked=set_workbench_use_labelledby>
                            "aria_labelledby"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_second set_checked=set_workbench_disable_second>
                            "disabled_indices=[1]"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <h3 id="docs-menu-workbench-heading">"Workbench Menu"</h3>
                    <Menu
                        id_base="docs-menu-workbench".to_string()
                        items=workbench_items.clone()
                        on_action=on_workbench_action
                        item_specs=workbench_item_specs.clone()
                        id="docs-menu-workbench-root".to_string()
                        aria_label=if workbench_use_labelledby.get() {
                            String::new()
                        } else {
                            "Workbench menu actions".to_string()
                        }
                        aria_labelledby=if workbench_use_labelledby.get() {
                            "docs-menu-workbench-heading".to_string()
                        } else {
                            String::new()
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_second.get() {
                            vec![1]
                        } else {
                            Vec::new()
                        }
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Checkbox {
                                is_checked: Signal::derive(move || workbench_share_checked.get()),
                            },
                            MenuItemKind::Radio {
                                is_checked: Signal::derive(move || workbench_sort_ascending.get()),
                            },
                        ]
                        default_index=workbench_default_index.get().unwrap_or(0).min(2)
                        motion=MenuMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "actions="
                        {move || workbench_action_count.get()}
                        " · last="
                        {move || workbench_last_action.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · share_checked="
                        {move || workbench_share_checked.get()}
                        " · sort_ascending="
                        {move || workbench_sort_ascending.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / LabelledBy / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <div class="docs-stack docs-stack--tight">
                        <Menu
                            id_base="docs-menu-matrix-default".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            item_specs=vec![
                                MenuItemSpec::action("New file"),
                                MenuItemSpec::action("Share with team"),
                                MenuItemSpec::action("Sort ascending"),
                            ]
                            default_index=1
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"default_index=1 + item_specs"</span>
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <h4 id="docs-menu-matrix-label">"Matrix Label"</h4>
                        <Menu
                            id_base="docs-menu-matrix-labelledby".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            aria_labelledby="docs-menu-matrix-label".to_string()
                            disabled_indices=vec![1]
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"aria_labelledby + disabled_indices"</span>
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <Menu
                            id_base="docs-menu-matrix-disabled".to_string()
                            items=Arc::from(vec![
                                "New file".to_string(),
                                "Share with team".to_string(),
                                "Sort ascending".to_string(),
                            ])
                            on_action=Callback::new(|_: usize| {})
                            aria_label="Disabled matrix menu".to_string()
                            is_disabled=true
                            disabled=true
                            class_name="docs-menu-workbench".to_string()
                            motion=MenuMotion::default()
                        />
                        <span class="ui-muted">"is_disabled + disabled + class_name"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
