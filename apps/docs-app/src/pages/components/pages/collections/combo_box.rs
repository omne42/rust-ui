use super::*;

pub(crate) fn combo_box() -> AnyView {
    let showcase_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let showcase_items_for_hello = showcase_items.clone();
    let showcase_items_for_showcase = showcase_items.clone();
    let showcase_items_for_matrix = showcase_items.clone();
    let showcase_items_for_stream_snapshot = showcase_items.clone();
    let showcase_items_for_stream_streaming = showcase_items.clone();
    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let empty_items: Vec<String> = Vec::new();
    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let combo_box_code_imports = "use leptos::prelude::*;\nuse ui::ComboBox;".to_string();

    let (hello_selected, set_hello_selected) = signal(Some(1_usize));
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));
    let (empty_selected, set_empty_selected) = signal(None::<usize>);
    let (snapshot_selected, set_snapshot_selected) = signal(Some(1_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let workbench_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let persisted_combo_box_workbench_selected = load_combo_box_workbench_selected();
    let (workbench_selected, set_workbench_selected) =
        signal(persisted_combo_box_workbench_selected.or(Some(1_usize)));
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_controlled_open, set_workbench_controlled_open) = signal(false);
    let (workbench_on_open_change_runs, set_workbench_on_open_change_runs) = signal(0_u32);
    let (workbench_use_controlled_open, set_workbench_use_controlled_open) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_combo_box_workbench_selected.is_some());
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_controlled_open.set(next);
        set_workbench_on_open_change_runs.update(|count| *count += 1);
    });

    Effect::new(move |_| {
        let selected = workbench_selected.get();

        if workbench_persist_state.get() {
            if let Some(selected_index) = selected {
                save_combo_box_workbench_selected(selected_index);
            } else {
                clear_combo_box_workbench_selected();
            }
        } else {
            clear_combo_box_workbench_selected();
        }
    });

    let hello_code = Signal::derive(move || {
        r#"let items = vec![
  "Rust".to_string(),
  "TypeScript".to_string(),
  "Go".to_string(),
];
let (selected, set_selected) = signal(Some(1_usize));

<ComboBox
  id_base="docs-combo-box-hello".to_string()
  label="Language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let showcase_code = Signal::derive(move || {
        r#"let items = vec![
  "Rust".to_string(),
  "TypeScript".to_string(),
  "Go".to_string(),
  "Python".to_string(),
  "Zig".to_string(),
];

let (selected_default, set_selected_default) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);
let (selected_controlled, set_selected_controlled) = signal(Some(2_usize));
let (open, set_open) = signal(false);
let (selected_disabled, set_selected_disabled) = signal(Some(0_usize));
let (selected_empty, set_selected_empty) = signal(None::<usize>);

<ComboBox
  id_base="combo-default".to_string()
  label="Default".to_string()
  items=items.clone()
  selected_index=selected_default
  set_selected_index=set_selected_default
  disabled_indices=vec![4]
  description="Pick one runtime language".to_string()
  error="Language is required".to_string()
  is_invalid=Signal::derive(move || invalid.get())
/>
<ComboBox
  id_base="combo-controlled".to_string()
  label="Controlled open".to_string()
  items=items.clone()
  selected_index=selected_controlled
  set_selected_index=set_selected_controlled
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
/>
<ComboBox
  id_base="combo-disabled".to_string()
  label="Disabled".to_string()
  items=vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()]
  selected_index=selected_disabled
  set_selected_index=set_selected_disabled
  is_disabled=true
/>
<ComboBox
  id_base="combo-empty".to_string()
  label="Empty".to_string()
  items=Vec::<String>::new()
  selected_index=selected_empty
  set_selected_index=set_selected_empty
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(1_usize));".to_string(),
            "let (open, set_open) = signal(false);".to_string(),
            "<ComboBox".to_string(),
            "  id_base=\"docs-combo-box-workbench\".into()".to_string(),
            "  label=\"Language\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"Rust\".into(),".to_string(),
            "    \"TypeScript\".into(),".to_string(),
            "    \"Go\".into(),".to_string(),
            "    \"Python\".into(),".to_string(),
            "    \"Zig\".into(),".to_string(),
            "  ]".to_string(),
            "  selected_index=selected".to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  is_required=Signal::derive(move || true)".to_string(),
            "  aria_describedby=Signal::derive(|| Some(\"combo-box-help\".to_string()))"
                .to_string(),
            "  description=\"Pick one runtime language\".into()".to_string(),
            "  error=\"Language is required\".into()".to_string(),
            "  placeholder=\"Search language\".into()".to_string(),
            "  empty_message=\"No language found\".into()".to_string(),
            "  toggle_button_aria_label=\"Open language options\".into()".to_string(),
            "  default_open=false".to_string(),
            "  lang=\"en-US\".into()".to_string(),
            "  dir=ui_headless::A11yDirection::Ltr".to_string(),
            "  motion=ui::combo_box::ComboBoxMotion::default()".to_string(),
        ];

        if invalid {
            lines.push("  is_invalid=Signal::derive(move || true)".to_string());
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if disable_last {
            lines.push("  disabled_indices=vec![4]".to_string());
        }
        if use_controlled_open {
            lines.push("  is_open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-combo-box-workbench--custom\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/combo-box/src/styles.rs */\n{}",
            ui::combo_box::styles::CSS,
        )
    });

    let output_mode_code = Signal::derive(move || {
        r#"// Streaming is optional for ComboBox; fallback is snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=AiRenderMode::Snapshot output_status=AiOutputStatus::Verified>
    <ComboBox id_base="docs-combo-box-snapshot".to_string() ... />
  </AiSpace>
  <AiSpace mode=AiRenderMode::Streaming output_status=AiOutputStatus::Draft>
    <ComboBox id_base="docs-combo-box-streaming".to_string() ... />
  </AiSpace>
</div>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<ComboBox id_base="combo-matrix-default".to_string() label="Default".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_default set_selected_index=set_selected_default />
<ComboBox id_base="combo-matrix-controlled".to_string() label="Controlled".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_controlled set_selected_index=set_selected_controlled is_open=Signal::derive(move || open.get()) default_open=false />
<ComboBox id_base="combo-matrix-disabled".to_string() label="Disabled".to_string() items=vec!["Rust".to_string(), "TypeScript".to_string(), "Go".to_string(), "Python".to_string(), "Zig".to_string()] selected_index=selected_disabled set_selected_index=set_selected_disabled is_disabled=true disabled_indices=vec![4] />"#.to_string()
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let open = workbench_controlled_open.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut class = vec!["ui-combo-box".to_string()];
        if custom_class {
            class.push("docs-combo-box-workbench--custom".to_string());
        }
        if invalid {
            class.push("ui-combo-box--invalid".to_string());
        }
        if use_controlled_open {
            class.push("ui-combo-box--controlled".to_string());
        }

        format!(
            "ComboBoxWorkbenchConfig {{\n  id_base: \"docs-combo-box-workbench\",\n  label: \"Language\",\n  items: [\"Rust\", \"TypeScript\", \"Go\", \"Python\", \"Zig\"],\n  selected_index: {selected:?},\n  set_selected_index: \"WriteSignal<Option<usize>>\",\n  is_disabled: Some({disabled}),\n  disabled_indices: {},\n  is_required: Some(true),\n  is_invalid: Some({invalid}),\n  aria_describedby: Some(Some(\"combo-box-help\")),\n  description: Some(\"Pick one runtime language\"),\n  error: Some(\"Language is required\"),\n  placeholder: Some(\"Search language\"),\n  empty_message: Some(\"No language found\"),\n  toggle_button_aria_label: Some(\"Open language options\"),\n  is_open: {},\n  default_open: Some(false),\n  on_open_change: \"runs={}\",\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  motion: ComboBoxMotion::default(),\n  class_name: {},\n  controlled_open_enabled: {use_controlled_open},\n  controlled_open_state: {open},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            if use_controlled_open {
                format!("Some({open})")
            } else {
                "None".to_string()
            },
            workbench_on_open_change_runs.get(),
            if custom_class {
                "Some(\"docs-combo-box-workbench--custom\")"
            } else {
                "None"
            },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover, baseline-style root attrs, and baseline-level panel/highlight motion."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="最小路径：默认 API 即可运行，保留输入筛选 + 列表选择语义。"
                code_signal=hello_code
                code_imports=combo_box_code_imports.clone()
            >
                <AiSpace mode=snapshot_mode output_status=verified_output>
                    <div class="docs-stack" style="width: min(100%, 320px);">
                        <ComboBox
                            id_base="docs-combo-box-hello".to_string()
                            label="Language".to_string()
                            items=showcase_items_for_hello.clone()
                            selected_index=hello_selected
                            set_selected_index=set_hello_selected
                        />
                        <span class="ui-muted">
                            "hello selected: "
                            {move || hello_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </AiSpace>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动，并可选持久化 selected index。"
                code_signal=workbench_code
                code_imports=combo_box_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/combo-box/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="combo-box-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_invalid.get()
                                on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev))
                            />
                            " Invalid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled root"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_last.get()
                                on:change=move |ev| set_workbench_disable_last.set(event_target_checked(&ev))
                            />
                            " Disable last option"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_use_controlled_open.get()
                                on:change=move |ev| set_workbench_use_controlled_open.set(event_target_checked(&ev))
                            />
                            " Controlled open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class marker"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist selected index (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="combo-box-workbench" style="width: min(100%, 420px);">
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_controlled_open.update(|value| *value = !*value)
                            })
                        >
                            "Toggle open"
                        </ui::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || workbench_controlled_open.get()}
                            " · on_open_change: "
                            {move || workbench_on_open_change_runs.get()}
                            " · selected: "
                            {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            " · persist selected: "
                            {move || if workbench_persist_state.get() { "on" } else { "off" }}
                        </span>
                    </div>

                    {move || {
                        let invalid = workbench_invalid.get();
                        let disabled = workbench_disabled.get();
                        let disable_last = workbench_disable_last.get();
                        let use_controlled_open = workbench_use_controlled_open.get();
                        let custom_class = workbench_custom_class.get();
                        let controlled_open: Signal<bool> =
                            Signal::derive(move || workbench_controlled_open.get());
                        let class_name = if custom_class {
                            "docs-combo-box-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { Vec::new() };

                        if use_controlled_open {
                            view! {
                                <div class="docs-card" data-slot="combo-box-workbench-canvas">
                                    <ComboBox
                                        id_base="docs-combo-box-workbench".to_string()
                                        label="Language".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_required=Signal::derive(move || true)
                                        aria_describedby=Signal::derive(|| {
                                            Some("combo-box-help".to_string())
                                        })
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        placeholder="Search language".to_string()
                                        empty_message="No language found".to_string()
                                        toggle_button_aria_label="Open language options".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        is_open=controlled_open
                                        default_open=false
                                        on_open_change=on_workbench_open_change
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        motion=ui::combo_box::ComboBoxMotion::default()
                                        class_name=class_name.clone()
                                    />
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="docs-card" data-slot="combo-box-workbench-canvas">
                                    <ComboBox
                                        id_base="docs-combo-box-workbench".to_string()
                                        label="Language".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_required=Signal::derive(move || true)
                                        aria_describedby=Signal::derive(|| {
                                            Some("combo-box-help".to_string())
                                        })
                                        description="Pick one runtime language".to_string()
                                        error="Language is required".to_string()
                                        placeholder="Search language".to_string()
                                        empty_message="No language found".to_string()
                                        toggle_button_aria_label="Open language options".to_string()
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        default_open=false
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        motion=ui::combo_box::ComboBoxMotion::default()
                                        class_name=class_name
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="combo-box-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="combo-box-state-rows">
                    <li>
                        <code>"open mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"disabled"</code>
                        " = root disabled | enabled with disabled options"
                    </li>
                    <li>
                        <code>"item set"</code>
                        " = has items | empty"
                    </li>
                    <li>
                        <code>"validation"</code>
                        " = valid | invalid"
                    </li>
                    <li>
                        <code>"selection"</code>
                        " = selected | none"
                    </li>
                </ul>
            </section>

            <Playground
                title="State Matrix (Default / Controlled / Disabled)"
                code_signal=matrix_code
                code_imports=combo_box_code_imports.clone()
            >
                <div class="docs-row" data-slot="combo-box-state-matrix-playground">
                    <ComboBox
                        id_base="docs-combo-box-matrix-default".to_string()
                        label="Default".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=selected
                        set_selected_index=set_selected
                        placeholder="Search language".to_string()
                    />
                    <ComboBox
                        id_base="docs-combo-box-matrix-controlled".to_string()
                        label="Controlled".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        is_open=controlled_open
                        default_open=false
                        on_open_change=on_open_change
                        motion=ui::combo_box::ComboBoxMotion::default()
                    />
                    <ComboBox
                        id_base="docs-combo-box-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        items=showcase_items_for_matrix.clone()
                        selected_index=disabled_selected
                        set_selected_index=set_disabled_selected
                        is_disabled=true
                        disabled_indices=vec![4]
                        class_name="docs-combo-box-workbench--custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Showcase Variants"
                description="同一套 ComboBox 在校验、受控 open、禁用、空数据四种状态下的对比展示。"
                code_signal=showcase_code
                code_imports=combo_box_code_imports.clone()
            >
                <div class="docs-row" data-slot="combo-box-showcase">
                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"validation + disabled option"</span>
                        <ComboBox
                            id_base="docs-combo-box".to_string()
                            label="Language".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=selected
                            set_selected_index=set_selected
                            disabled_indices=vec![4]
                            description="Pick one runtime language".to_string()
                            error="Language is required".to_string()
                            is_invalid=Signal::derive(move || invalid.get())
                        />
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                            >
                                {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                            </ui::Button>
                            <span class="ui-muted">
                                "selected: "
                                {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            </span>
                        </div>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"controlled open"</span>
                        <ComboBox
                            id_base="docs-combo-box-controlled".to_string()
                            label="Controlled language".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=controlled_selected
                            set_selected_index=set_controlled_selected
                            is_open=controlled_open
                            on_open_change=on_open_change
                            disabled_indices=vec![4]
                            description="Open state is externally controlled".to_string()
                        />
                        <div class="docs-row">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_controlled_open_raw.update(|value| *value = !*value)
                                })
                            >
                                "Toggle open"
                            </ui::Button>
                            <span class="ui-muted">
                                "open: "
                                {move || controlled_open_raw.get()}
                                " · selected: "
                                {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                            </span>
                        </div>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"disabled root"</span>
                        <ComboBox
                            id_base="docs-combo-box-disabled".to_string()
                            label="Disabled language".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            is_disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 260px; width: min(100%, 320px);">
                        <span class="ui-muted">"empty items"</span>
                        <ComboBox
                            id_base="docs-combo-box-empty".to_string()
                            label="Empty language list".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="ComboBox 不是正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=combo_box_code_imports
            >
                <div class="docs-row" data-slot="combo-box-streaming-snapshot">
                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <ComboBox
                                id_base="docs-combo-box-snapshot".to_string()
                                label="Snapshot mode".to_string()
                                items=showcase_items_for_stream_snapshot
                                selected_index=snapshot_selected
                                set_selected_index=set_snapshot_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Snapshot baseline: verified + copy-ready."</div>
                    </div>

                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="streaming"
                    >
                        <AiSpace mode=streaming_mode output_status=draft_output>
                            <ComboBox
                                id_base="docs-combo-box-streaming".to_string()
                                label="Streaming preview".to_string()
                                items=showcase_items_for_stream_streaming
                                selected_index=streaming_selected
                                set_selected_index=set_streaming_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Streaming preview keeps fallback=snapshot contract explicit."</div>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="combo-box-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::ComboBox;\n\nlet (selected, set_selected) = signal(Some(1_usize));\n<ComboBox id_base=\"docs-combo-box\".to_string() label=\"Language\".to_string() items=vec![\"Rust\".to_string(), \"TypeScript\".to_string()] selected_index=selected set_selected_index=set_selected />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-combo-box-source-copy".to_string()
                />
                <ul data-slot="combo-box-source-paths">
                    <li><code>"components/combo-box/src/mod.rs"</code></li>
                    <li><code>"components/combo-box/src/logic.rs"</code></li>
                    <li><code>"components/combo-box/src/view.rs"</code></li>
                    <li><code>"components/combo-box/src/styles.rs"</code></li>
                    <li><code>"components/combo-box/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="combo-box-source-prerequisites">
                    <li><code>"component-combo_box"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
