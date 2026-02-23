use super::*;

pub(crate) fn action_group() -> AnyView {
    let items = vec![
        ActionGroupItem::new("align-left", "Align Left"),
        ActionGroupItem::new("align-center", "Align Center"),
        ActionGroupItem::new("align-right", "Align Right"),
        ActionGroupItem::new("align-justify", "Justify").disabled(true),
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (workbench_selected_ids, set_workbench_selected_ids) =
        signal(BTreeSet::from(["align-left".to_string()]));
    let workbench_selected_ids_signal: Signal<BTreeSet<String>> =
        Signal::derive(move || workbench_selected_ids.get());
    let (workbench_last_action, set_workbench_last_action) = signal("none".to_string());
    let (workbench_selection_change_count, set_workbench_selection_change_count) = signal(0_u32);

    let on_workbench_selected_change = Callback::new(move |next: BTreeSet<String>| {
        set_workbench_selected_ids.set(next);
        set_workbench_selection_change_count.update(|count| *count += 1);
    });

    let on_workbench_action = Callback::new(move |id: String| {
        set_workbench_last_action.set(id);
    });

    let tone_options = vec!["Default".to_string(), "Strong".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => ActionGroupTone::Strong,
        _ => ActionGroupTone::Default,
    });
    let (workbench_multiple, set_workbench_multiple) = signal(false);
    let workbench_selection_mode = Signal::derive(move || {
        if workbench_multiple.get() {
            ActionGroupSelectionMode::Multiple
        } else {
            ActionGroupSelectionMode::Single
        }
    });
    let workbench_default_selected_ids = Signal::derive(move || {
        if workbench_multiple.get() {
            BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
        } else {
            BTreeSet::from(["align-left".to_string()])
        }
    });
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ActionGroup
  id_base="text-align".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let selection_mode = if workbench_multiple.get() {
            "ActionGroupSelectionMode::Multiple"
        } else {
            "ActionGroupSelectionMode::Single"
        };
        let tone = match workbench_tone.get() {
            ActionGroupTone::Strong => "ActionGroupTone::Strong",
            _ => "ActionGroupTone::Default",
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-action-group-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let selected_literal = format!("{:?}", workbench_selected_ids.get());
        let default_selected_literal = format!("{:?}", workbench_default_selected_ids.get());

        [
            "<ActionGroup".to_string(),
            "  id_base=\"docs-action-group-workbench\".to_string()".to_string(),
            "  items=vec![".to_string(),
            "    ActionGroupItem::new(\"align-left\", \"Align Left\"),".to_string(),
            "    ActionGroupItem::new(\"align-center\", \"Align Center\"),".to_string(),
            "    ActionGroupItem::new(\"align-right\", \"Align Right\"),".to_string(),
            "    ActionGroupItem::new(\"align-justify\", \"Justify\").disabled(true),".to_string(),
            "  ]".to_string(),
            format!("  tone={tone}"),
            format!("  selection_mode={selection_mode}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  selected_ids=Signal::derive(move || {selected_literal})"),
            format!("  default_selected_ids={default_selected_literal}"),
            "  on_selected_ids_change=Callback::new(move |next| { drop(next); })".to_string(),
            "  on_action=Callback::new(move |id| { drop(id); })".to_string(),
            "  aria_label=\"Text alignment actions\".to_string()".to_string(),
            format!(
                "  lang={}.to_string()",
                rust_string_literal(if workbench_rtl.get() { "ar" } else { "en-US" })
            ),
            format!("  dir={dir}"),
            format!("  class_name={class_name}"),
            ">".to_string(),
            "</ActionGroup>".to_string(),
        ]
        .join("\n")
    });

    let workbench_items_for_config = workbench_items.clone();
    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone.get() {
            ActionGroupTone::Strong => "strong",
            _ => "default",
        };
        let selection_mode = match workbench_selection_mode.get() {
            ActionGroupSelectionMode::Multiple => "multiple",
            ActionGroupSelectionMode::Single => "single",
            ActionGroupSelectionMode::None => "none",
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };
        let class_name = if workbench_custom_class.get() {
            Some("docs-action-group-workbench")
        } else {
            None
        };

        format!(
            "ActionGroupWorkbenchActualConfig {{\n  id_base: \"docs-action-group-workbench\",\n  items: {:?},\n  tone: \"{tone}\",\n  selection_mode: \"{selection_mode}\",\n  is_disabled: {},\n  selected_ids: {:?},\n  default_selected_ids: Some({:?}),\n  on_selected_ids_change: \"count={}\",\n  on_action: \"last={}\",\n  aria_label: Some(\"Text alignment actions\"),\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n  class_name: {class_name:?},\n}}",
            workbench_items_for_config.clone(),
            bool_word(workbench_is_disabled.get()),
            workbench_selected_ids.get(),
            workbench_default_selected_ids.get(),
            workbench_selection_change_count.get(),
            workbench_last_action.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionGroup
  id_base="text-align-default".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
/>
<ActionGroup
  id_base="text-align-multiple".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
  selection_mode=ActionGroupSelectionMode::Multiple
  tone=ActionGroupTone::Strong
  default_selected_ids=BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
/>
<ActionGroup
  id_base="text-align-disabled".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
    ActionGroupItem::new("align-justify", "Justify").disabled(true),
  ]
  is_disabled=true
  aria_label="Disabled text alignment".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ActionGroup"
            slug="action-group"
            group="Actions"
            description="Selectable action cluster with centralized selection normalization and baseline-style state/source data contracts."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-default".to_string()
                        items=showcase_items
                    />
                    <span class="ui-muted">
                        "basic selection cluster"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-action-group-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionGroup tone".to_string()
                        />
                        <Switch checked=workbench_multiple set_checked=set_workbench_multiple>
                            "Multiple selection_mode"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_ids.set(BTreeSet::from(["align-left".to_string()]))
                            >
                                "Select left"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_ids.set(BTreeSet::new())
                            >
                                "Clear selection"
                            </button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-workbench".to_string()
                        items=workbench_items.clone()
                        tone=workbench_tone.get()
                        selection_mode=workbench_selection_mode.get()
                        is_disabled=workbench_is_disabled.get()
                        selected_ids=workbench_selected_ids_signal
                        default_selected_ids=workbench_default_selected_ids.get()
                        on_selected_ids_change=on_workbench_selected_change
                        on_action=on_workbench_action
                        aria_label="Text alignment actions".to_string()
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-action-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected_ids.get().iter().cloned().collect::<Vec<_>>().join(", ")}
                        " · selection changes: "
                        {move || workbench_selection_change_count.get()}
                        " · last action: "
                        {move || workbench_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Multiple / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionGroup
                        id_base="docs-action-group-matrix-default".to_string()
                        items=matrix_items.clone()
                    />
                    <ActionGroup
                        id_base="docs-action-group-matrix-multiple".to_string()
                        items=matrix_items.clone()
                        selection_mode=ActionGroupSelectionMode::Multiple
                        default_selected_ids=BTreeSet::from([
                            "align-left".to_string(),
                            "align-center".to_string(),
                        ])
                        tone=ActionGroupTone::Strong
                    />
                    <ActionGroup
                        id_base="docs-action-group-matrix-disabled".to_string()
                        items=matrix_items
                        is_disabled=true
                        aria_label="Disabled text alignment".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
