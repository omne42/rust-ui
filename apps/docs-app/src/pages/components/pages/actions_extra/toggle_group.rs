use super::*;

pub(crate) fn toggle_group() -> AnyView {
    let items = vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic"),
        ToggleGroupItem::new("underline", "Underline"),
        ToggleGroupItem::new("strike", "Strike").disabled(true),
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (selected_ids_raw, set_selected_ids_raw) =
        signal(BTreeSet::from(["bold".to_string(), "italic".to_string()]));
    let selected_ids: Signal<BTreeSet<String>> = Signal::derive(move || selected_ids_raw.get());
    let (on_selected_ids_change_runs, set_on_selected_ids_change_runs) = signal(0_u32);
    let on_selected_ids_change = Callback::new(move |next: BTreeSet<String>| {
        set_selected_ids_raw.set(next);
        set_on_selected_ids_change_runs.update(|count| *count += 1);
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let (on_action_runs, set_on_action_runs) = signal(0_u32);
    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
        set_on_action_runs.update(|count| *count += 1);
    });

    let (workbench_mode_index, set_workbench_mode_index) = signal(Some(0_usize));
    let mode_options = vec!["Multiple".to_string(), "Single".to_string()];
    let selection_mode = Signal::derive(move || match workbench_mode_index.get().unwrap_or(0) {
        1 => ToggleGroupSelectionMode::Single,
        _ => ToggleGroupSelectionMode::Multiple,
    });

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let orientation =
        Signal::derive(
            move || match workbench_orientation_index.get().unwrap_or(0) {
                1 => ToggleGroupOrientation::Vertical,
                _ => ToggleGroupOrientation::Horizontal,
            },
        );

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let variant_options = vec![
        "Default".to_string(),
        "Outline".to_string(),
        "Ghost".to_string(),
    ];
    let variant = Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Outline,
        2 => ToggleButtonVariant::Ghost,
        _ => ToggleButtonVariant::Default,
    });

    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let size_options = vec!["Xs".to_string(), "Sm".to_string(), "Md".to_string()];
    let size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::Sm,
        _ => ToggleButtonSize::M,
    });

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_attached, set_workbench_is_attached) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ToggleGroup
  id_base="docs-toggle-group-hello".to_string()
  items=vec![
    ToggleGroupItem::new("bold", "Bold"),
    ToggleGroupItem::new("italic", "Italic"),
    ToggleGroupItem::new("underline", "Underline"),
  ]
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-toggle-group-custom"
        } else {
            ""
        };
        [
            "<ToggleGroup".to_string(),
            "  id_base=\"docs-toggle-group-workbench\".to_string()".to_string(),
            "  items=vec![ToggleGroupItem::new(\"bold\", \"Bold\"), ToggleGroupItem::new(\"italic\", \"Italic\"), ToggleGroupItem::new(\"underline\", \"Underline\"), ToggleGroupItem::new(\"strike\", \"Strike\").disabled(true)]".to_string(),
            format!("  selection_mode={:?}", selection_mode.get()),
            "  selected_ids=selected_ids".to_string(),
            "  default_selected_ids=BTreeSet::from([\"bold\".to_string(), \"italic\".to_string()])".to_string(),
            "  on_selected_ids_change=on_selected_ids_change".to_string(),
            "  on_action=on_action".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  is_attached={}", bool_word(workbench_is_attached.get())),
            format!("  orientation={:?}", orientation.get()),
            format!("  variant={:?}", variant.get()),
            format!("  size={:?}", size.get()),
            "  aria_label=\"Text style toggles\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-toggle-group-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        format!(
            "ToggleGroupActualConfig {{\n  id_base: \"docs-toggle-group-workbench\",\n  items: \"sample_items(len=4)\",\n  selection_mode: {:?},\n  selected_ids: {:?},\n  default_selected_ids: Some(BTreeSet::from([\"bold\".to_string(), \"italic\".to_string()])),\n  on_selected_ids_change: \"runs={}\",\n  on_action: \"runs={},last={:?}\",\n  is_disabled: {},\n  is_attached: {},\n  orientation: {:?},\n  variant: {:?},\n  size: {:?},\n  aria_label: Some(\"Text style toggles\"),\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n  class_name: {class_name:?},\n}}",
            selection_mode.get(),
            selected_ids_raw.get(),
            on_selected_ids_change_runs.get(),
            on_action_runs.get(),
            last_action.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_attached.get()),
            orientation.get(),
            variant.get(),
            size.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ToggleGroup id_base="tg-default".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] selected_ids=Signal::derive(|| BTreeSet::from(["bold".to_string()])) selection_mode=ToggleGroupSelectionMode::Multiple />
<ToggleGroup id_base="tg-single".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] selection_mode=ToggleGroupSelectionMode::Single orientation=ToggleGroupOrientation::Vertical variant=ToggleButtonVariant::Outline size=ToggleButtonSize::Sm />
<ToggleGroup id_base="tg-disabled".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] is_disabled=true is_attached=false variant=ToggleButtonVariant::Ghost />"#.to_string()
    });

    view! {
        <ComponentPage
            title="ToggleGroup"
            slug="toggle-group"
            group="Actions"
            description="ToggleGroup playground with strict Showcase/Workbench/Matrix layout and full API feedback."
        >
            <Playground title="Hello World (Default Group)" code_signal=hello_code>
                <ToggleGroup
                    id_base="docs-toggle-group-hello".to_string()
                    items=showcase_items
                    aria_label="Formatting options".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toggle-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-toggle-group-mode".to_string()
                            options=mode_options.clone()
                            selected_index=workbench_mode_index
                            set_selected_index=set_workbench_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup selection mode".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup orientation".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup size".to_string()
                        />
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_attached set_checked=set_workbench_is_attached>
                            "is_attached"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="toggle-group-workbench-preview">
                    <ToggleGroup
                        id_base="docs-toggle-group-workbench".to_string()
                        items=workbench_items
                        selection_mode=selection_mode.get()
                        selected_ids=selected_ids
                        default_selected_ids=BTreeSet::from([
                            "bold".to_string(),
                            "italic".to_string(),
                        ])
                        on_selected_ids_change=on_selected_ids_change
                        on_action=on_action
                        is_disabled=workbench_is_disabled.get()
                        is_attached=workbench_is_attached.get()
                        orientation=orientation.get()
                        variant=variant.get()
                        size=size.get()
                        aria_label="Text style toggles".to_string()
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-toggle-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="toggle-group-workbench-feedback">
                        "selected_ids: " {move || format!("{:?}", selected_ids_raw.get())}
                        " · on_selected_ids_change: " {move || on_selected_ids_change_runs.get()}
                        " · on_action: " {move || on_action_runs.get()}
                        " · last_action: " {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Single / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="toggle-group-state-matrix">
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-default".to_string()
                        items=matrix_items.clone()
                        selected_ids=Signal::derive(move || BTreeSet::from(["bold".to_string()]))
                        selection_mode=ToggleGroupSelectionMode::Multiple
                    />
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-single".to_string()
                        items=matrix_items.clone()
                        selection_mode=ToggleGroupSelectionMode::Single
                        orientation=ToggleGroupOrientation::Vertical
                        variant=ToggleButtonVariant::Outline
                        size=ToggleButtonSize::Sm
                    />
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-disabled".to_string()
                        items=matrix_items
                        is_disabled=true
                        is_attached=false
                        variant=ToggleButtonVariant::Ghost
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
