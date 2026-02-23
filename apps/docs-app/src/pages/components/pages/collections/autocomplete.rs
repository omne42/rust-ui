use super::*;

pub(crate) fn autocomplete() -> AnyView {
    let hello_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
    ];
    let items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let items_for_validation = items.clone();
    let items_for_stream_snapshot = items.clone();
    let items_for_stream_streaming = items.clone();
    let controlled_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let (snapshot_selected, set_snapshot_selected) = signal(Some(1_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let disabled_items = vec![
        "Berlin".to_string(),
        "Boston".to_string(),
        "Brisbane".to_string(),
    ];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let empty_items_for_state_matrix = empty_items.clone();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let hello_code = Signal::derive(move || {
        r#"<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string()]
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);

<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
  description="Search and pick one city".to_string()
  error="City is required".to_string()
  is_invalid=Signal::derive(move || invalid.get())
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(2_usize));
let (open, set_open) = signal(false);

<Autocomplete
  id_base="city-controlled".to_string()
  label="Controlled city".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  disabled_indices=vec![3]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<Autocomplete
  id_base="city-disabled".to_string()
  label="Disabled city".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string(), "Perth".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  is_disabled=true
/>
<Autocomplete
  id_base="city-empty".to_string()
  label="Empty city list".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#
        .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r#"// Streaming is optional for Autocomplete; fallback is snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=AiRenderMode::Snapshot output_status=AiOutputStatus::Verified>
    <Autocomplete id_base="docs-autocomplete-snapshot".to_string() ... />
  </AiSpace>
  <AiSpace mode=AiRenderMode::Streaming output_status=AiOutputStatus::Draft>
    <Autocomplete id_base="docs-autocomplete-streaming".to_string() ... />
  </AiSpace>
</div>"#
            .to_string()
    });

    let autocomplete_code_imports = "use leptos::prelude::*;\nuse ui::Autocomplete;".to_string();

    let persisted_autocomplete_workbench_selected = load_autocomplete_workbench_selected();
    let (workbench_selected, set_workbench_selected) =
        signal(persisted_autocomplete_workbench_selected.or(Some(2_usize)));
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_controlled_open, set_workbench_controlled_open) = signal(false);
    let (workbench_use_controlled_open, set_workbench_use_controlled_open) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_autocomplete_workbench_selected.is_some());
    let workbench_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let workbench_items_for_state_matrix = workbench_items.clone();

    Effect::new(move |_| {
        let selected = workbench_selected.get();

        if workbench_persist_state.get() {
            if let Some(selected_index) = selected {
                save_autocomplete_workbench_selected(selected_index);
            } else {
                clear_autocomplete_workbench_selected();
            }
        } else {
            clear_autocomplete_workbench_selected();
        }
    });

    let workbench_code = Signal::derive(move || {
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(2_usize));".to_string(),
            "let (open, set_open) = signal(false);".to_string(),
            "<Autocomplete".to_string(),
            "  id_base=\"docs-autocomplete-workbench\".into()".to_string(),
            "  label=\"City\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"San Francisco\".into(),".to_string(),
            "    \"Seattle\".into(),".to_string(),
            "    \"Shanghai\".into(),".to_string(),
            "    \"Shenzhen\".into(),".to_string(),
            "    \"Singapore\".into(),".to_string(),
            "  ]".to_string(),
            "  selected_index=selected".to_string(),
            "  default_selected_index=2".to_string(),
            "  on_selected_index_change=Callback::new(move |next| set_selected.set(next))"
                .to_string(),
            "  set_selected_index=set_selected".to_string(),
            "  is_required=Signal::derive(move || false)".to_string(),
            "  required=Signal::derive(move || false)".to_string(),
            "  aria_describedby=Signal::derive(move || Some(\"docs-autocomplete-hint\".to_string()))"
                .to_string(),
            "  description=\"Search and pick one city\".into()".to_string(),
            "  error=\"City is required\".into()".to_string(),
            "  placeholder=\"Type…\".into()".to_string(),
            "  empty_message=\"No matches\".into()".to_string(),
            "  default_open=false".to_string(),
            "  lang=\"en\".into()".to_string(),
            "  dir=A11yDirection::Ltr".to_string(),
            "  motion=AutocompleteMotion::default()".to_string(),
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
            lines.push("  open=Signal::derive(move || open.get())".to_string());
            lines
                .push("  on_open_change=Callback::new(move |next| set_open.set(next))".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-autocomplete-workbench--custom\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/autocomplete/src/styles.rs */\n{}",
            ui::autocomplete::styles::CSS,
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let disable_last = workbench_disable_last.get();
        let _open = workbench_controlled_open.get();
        let use_controlled_open = workbench_use_controlled_open.get();
        let custom_class = workbench_custom_class.get();

        let mut class = vec!["ui-autocomplete".to_string()];
        if custom_class {
            class.push("docs-autocomplete-workbench--custom".to_string());
        }
        if invalid {
            class.push("ui-autocomplete--invalid".to_string());
        }
        if use_controlled_open {
            class.push("ui-autocomplete--controlled".to_string());
        }

        format!(
            "AutocompleteWorkbenchConfig {{\n  id_base: \"docs-autocomplete-workbench\",\n  label: \"City\",\n  items: [\"San Francisco\", \"Seattle\", \"Shanghai\", \"Shenzhen\", \"Singapore\"],\n  selected_index: {selected:?},\n  default_selected_index: Some(2),\n  on_selected_index_change: Some(\"Callback<Option<usize>>\"),\n  set_selected_index: Some(\"WriteSignal<Option<usize>>\"),\n  is_disabled: Some({disabled}),\n  disabled: {disabled},\n  disabled_indices: {},\n  is_required: Some(false),\n  required: Some(false),\n  is_invalid: Some({invalid}),\n  invalid: Some({invalid}),\n  aria_describedby: Some(\"docs-autocomplete-hint\"),\n  description: Some(\"Search and pick one city\"),\n  error: Some(\"City is required\"),\n  placeholder: Some(\"Type…\"),\n  empty_message: Some(\"No matches\"),\n  is_open: {},\n  open: {},\n  default_open: Some(false),\n  on_open_change: {},\n  lang: Some(\"en\"),\n  dir: Some(\"ltr\"),\n  motion: AutocompleteMotion::default(),\n  class_name: {},\n  class: \"{}\",\n}}",
            if disable_last { "vec![4]" } else { "vec![]" },
            if use_controlled_open {
                "Some(true)"
            } else {
                "None"
            },
            if use_controlled_open {
                "Some(true)"
            } else {
                "None"
            },
            if use_controlled_open {
                "Some(\"Callback<bool>\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-autocomplete-workbench--custom\")"
            } else {
                "None"
            },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with baseline-style root attrs, controlled/uncontrolled open state, and baseline-level active highlight motion."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=autocomplete_code_imports.clone()
            >
                <AiSpace mode=snapshot_mode output_status=verified_output>
                    <div class="docs-stack" data-slot="autocomplete-hello-world">
                        <Autocomplete
                            id_base="docs-autocomplete-hello".to_string()
                            label="City".to_string()
                            items=hello_items
                        />
                    </div>
                </AiSpace>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="Autocomplete 单画布调参：支持 settings / code / css-test 联动，并可选持久化 selected index。"
                code_signal=workbench_code
                code_imports=autocomplete_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/autocomplete/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="autocomplete-workbench-controls">
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
                <div class="docs-stack" data-slot="autocomplete-workbench" style="width: min(100%, 420px);">
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
                        let controlled_open =
                            Signal::derive(move || workbench_controlled_open.get());
                        let on_workbench_open_change =
                            Callback::new(move |next: bool| set_workbench_controlled_open.set(next));
                        let class_name = if custom_class {
                            "docs-autocomplete-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { vec![] };

                        if use_controlled_open {
                            view! {
                                <div class="docs-card" data-slot="autocomplete-workbench-canvas">
                                    <Autocomplete
                                        id_base="docs-autocomplete-workbench".to_string()
                                        label="City".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_open=controlled_open
                                        on_open_change=on_workbench_open_change
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices.clone()
                                        description="Search and pick one city".to_string()
                                        error="City is required".to_string()
                                        class_name=class_name.clone()
                                    />
                                </div>
                            }
                            .into_any()
                        } else {
                            view! {
                                <div class="docs-card" data-slot="autocomplete-workbench-canvas">
                                    <Autocomplete
                                        id_base="docs-autocomplete-workbench".to_string()
                                        label="City".to_string()
                                        items=workbench_items.clone()
                                        selected_index=workbench_selected
                                        set_selected_index=set_workbench_selected
                                        is_invalid=Signal::derive(move || invalid)
                                        is_disabled=disabled
                                        disabled_indices=disabled_indices
                                        description="Search and pick one city".to_string()
                                        error="City is required".to_string()
                                        class_name=class_name
                                    />
                                </div>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>



            <Playground
                title="Selection + Validation"
                code_signal=code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-stack" data-slot="autocomplete-validation-playground">
                    <Autocomplete
                        id_base="docs-autocomplete".to_string()
                        label="City".to_string()
                        items=items_for_validation
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                        description="Search and pick one city".to_string()
                        error="City is required".to_string()
                        is_invalid=Signal::derive(move || invalid.get())
                        placeholder="Type…".to_string()
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
            </Playground>

            <Playground
                title="Controlled Open State"
                code_signal=controlled_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-stack" data-slot="autocomplete-controlled-playground">
                    <Autocomplete
                        id_base="docs-autocomplete-controlled".to_string()
                        label="Controlled city".to_string()
                        items=controlled_items
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        is_open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=vec![3]
                        description="Open state is externally controlled".to_string()
                    />
                    <span class="ui-muted" data-slot="autocomplete-controlled-open">
                        "open: "
                        {move || controlled_open_raw.get()}
                    </span>
                    <span class="ui-muted" data-slot="autocomplete-controlled-selected">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Disabled + Empty"
                code_signal=states_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-row" data-slot="autocomplete-states-playground">
                    <div class="docs-stack" data-slot="autocomplete-disabled-playground">
                        <Autocomplete
                            id_base="docs-autocomplete-disabled".to_string()
                            label="Disabled city".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            is_disabled=true
                        />
                        <span class="ui-muted" data-slot="autocomplete-disabled-selected">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" data-slot="autocomplete-empty-playground">
                        <Autocomplete
                            id_base="docs-autocomplete-empty".to_string()
                            label="Empty city list".to_string()
                            items=empty_items.clone()
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted" data-slot="autocomplete-empty-selected">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="autocomplete-state-matrix">
                <h3>"状态矩阵 State Matrix（受控 / 非受控）"</h3>
                <ul data-slot="autocomplete-state-rows">
                    <li>
                        <code>"open mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"disabled"</code>
                        " = root disabled | enabled with disabled options"
                    </li>
                    <li>
                        <code>"validation"</code>
                        " = valid | invalid"
                    </li>
                    <li>
                        <code>"item set"</code>
                        " = has items | empty"
                    </li>
                    <li>
                        <code>"selection"</code>
                        " = selected | none"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="autocomplete-parameter-matrix">
                <h3>"参数矩阵 Parameter Matrix（API / 默认值）"</h3>
                <ul data-slot="autocomplete-parameter-rows">
                    <li>
                        <code>"is_open + on_open_change + default_open"</code>
                        " = open 受控/非受控轴（default_open 默认 false）"
                    </li>
                    <li>
                        <code>"selected_index + on_selected_index_change + default_selected_index"</code>
                        " = selection 受控/非受控轴（default_selected_index 默认 none，越界值自动忽略）"
                    </li>
                    <li>
                        <code>"set_selected_index"</code>
                        " = 迁移期历史别名（桥接到 on_selected_index_change）"
                    </li>
                    <li>
                        <code>"is_disabled / is_required / is_invalid"</code>
                        " = 布尔轴，默认 false（历史别名：disabled / required / invalid）"
                    </li>
                    <li>
                        <code>"label / id_base / placeholder / empty_message"</code>
                        " = 默认值来自 ui-state-primitives（Options / autocomplete / Type… / No matches）"
                    </li>
                </ul>
            </section>



            <Playground
                title="State Matrix (Validation / Controlled / Empty)"
                code_signal=states_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-row" data-slot="autocomplete-state-matrix-playground">
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-invalid".to_string()
                        label="Invalid".to_string()
                        items=workbench_items_for_state_matrix.clone()
                        selected_index=workbench_selected
                        set_selected_index=set_workbench_selected
                        is_invalid=Signal::derive(move || workbench_invalid.get())
                        description="Validation state".to_string()
                        error="City is required".to_string()
                        placeholder="Type…".to_string()
                        empty_message="No matches".to_string()
                    />
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-controlled".to_string()
                        label="Controlled open".to_string()
                        items=workbench_items_for_state_matrix.clone()
                        selected_index=workbench_selected
                        set_selected_index=set_workbench_selected
                        is_open=Signal::derive(move || workbench_controlled_open.get())
                        on_open_change=Callback::new(move |next: bool| set_workbench_controlled_open.set(next))
                        default_open=false
                    />
                    <Autocomplete
                        id_base="docs-autocomplete-matrix-empty".to_string()
                        label="Empty".to_string()
                        items=empty_items_for_state_matrix.clone()
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                        empty_message="No matches".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="Autocomplete 不是正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=autocomplete_code_imports.clone()
            >
                <div class="docs-row" data-slot="autocomplete-streaming-snapshot">
                    <div
                        class="docs-stack"
                        style="min-width: 260px; width: min(100%, 320px);"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <Autocomplete
                                id_base="docs-autocomplete-snapshot".to_string()
                                label="Snapshot mode".to_string()
                                items=items_for_stream_snapshot
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
                            <Autocomplete
                                id_base="docs-autocomplete-streaming".to_string()
                                label="Streaming preview".to_string()
                                items=items_for_stream_streaming
                                selected_index=streaming_selected
                                set_selected_index=set_streaming_selected
                            />
                        </AiSpace>
                        <div class="ui-muted">"Streaming preview keeps fallback=snapshot contract explicit."</div>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="autocomplete-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::Autocomplete;\n\nlet (selected, set_selected) = signal(Some(1_usize));\n<Autocomplete id_base=\"docs-autocomplete\".to_string() label=\"City\".to_string() items=vec![\"Tokyo\".to_string(), \"Osaka\".to_string()] selected_index=selected set_selected_index=set_selected />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-autocomplete-source-copy".to_string()
                />
                <ul data-slot="autocomplete-source-paths">
                    <li><code>"components/autocomplete/src/mod.rs"</code></li>
                    <li><code>"components/autocomplete/src/logic.rs"</code></li>
                    <li><code>"components/autocomplete/src/view.rs"</code></li>
                    <li><code>"components/autocomplete/src/styles.rs"</code></li>
                    <li><code>"components/autocomplete/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="autocomplete-source-prerequisites">
                    <li><code>"component-autocomplete"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
