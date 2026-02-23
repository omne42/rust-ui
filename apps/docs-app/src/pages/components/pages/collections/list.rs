use super::*;

pub(crate) fn list() -> AnyView {
    let hello_items: Arc<[String]> = vec!["Overview".to_string(), "Billing".to_string()].into();
    let showcase_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
        "Audit Logs".to_string(),
    ]
    .into();
    let showcase_items_for_showcase = showcase_items.clone();
    let showcase_items_for_matrix = showcase_items.clone();
    let showcase_items_for_matrix_after = showcase_items.clone();
    let showcase_items_for_stream_snapshot = showcase_items.clone();
    let showcase_items_for_stream_streaming = showcase_items.clone();
    let disabled_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
    ]
    .into();
    let empty_items: Arc<[String]> = Vec::<String>::new().into();

    let (showcase_selected_default, set_showcase_selected_default) = signal(Some(0_usize));
    let (showcase_selected_unsynced, set_showcase_selected_unsynced) = signal(Some(1_usize));
    let (showcase_selected_disabled, set_showcase_selected_disabled) = signal(Some(0_usize));
    let (showcase_selected_empty, set_showcase_selected_empty) = signal(None::<usize>);

    let workbench_items: Arc<[String]> = vec![
        "Overview".to_string(),
        "Billing".to_string(),
        "Integrations".to_string(),
        "Audit Logs".to_string(),
        "Security".to_string(),
    ]
    .into();
    let (workbench_selected, set_workbench_selected) = signal(Some(1_usize));
    let (workbench_sync_active, set_workbench_sync_active) = signal(true);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_root_disabled, set_workbench_root_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_action_count, set_workbench_action_count) = signal(0_u32);
    let (workbench_last_action, set_workbench_last_action) = signal(None::<usize>);
    let on_workbench_action = Callback::new(move |index: usize| {
        set_workbench_last_action.set(Some(index));
        set_workbench_action_count.update(|count| *count += 1);
    });
    let list_code_imports =
        "use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::{AiOutputStatus, AiRenderMode, AiSpace, List};".to_string();
    let list_snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let list_streaming_mode = Signal::derive(|| AiRenderMode::Streaming);
    let list_draft_output = Signal::derive(|| AiOutputStatus::Draft);
    let list_verified_output = Signal::derive(|| AiOutputStatus::Verified);
    let (state_matrix_controlled_selected, set_state_matrix_controlled_selected) =
        signal(Some(1_usize));
    let (snapshot_selected, set_snapshot_selected) = signal(Some(0_usize));
    let (streaming_selected, set_streaming_selected) = signal(Some(2_usize));

    let hello_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec!["Overview".to_string(), "Billing".to_string()].into();
<List id_base="list-hello".to_string() items=items aria_label="Settings navigation".to_string() />"#
            .to_string()
    });

    let showcase_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

let (selected_a, set_selected_a) = signal(Some(0_usize));
let (selected_b, set_selected_b) = signal(Some(1_usize));
let (selected_c, set_selected_c) = signal(Some(0_usize));
let (selected_empty, set_selected_empty) = signal(None::<usize>);

<List
  id_base="list-default".to_string()
  items=items.clone()
  selected_index=selected_a.into()
  on_selected_index_change=Callback::new(move |next| set_selected_a.set(next))
  aria_label="Default list".to_string()
  disabled_indices=vec![2]
/>
<List
  id_base="list-unsynced".to_string()
  items=items
  selected_index=selected_b.into()
  on_selected_index_change=Callback::new(move |next| set_selected_b.set(next))
  aria_label="Unsynced list".to_string()
  is_active_index_synced_to_selected=false
/>
<List
  id_base="list-disabled".to_string()
  items=vec!["Overview".to_string(), "Billing".to_string(), "Integrations".to_string()].into()
  selected_index=selected_c.into()
  on_selected_index_change=Callback::new(move |next| set_selected_c.set(next))
  aria_label="Disabled list".to_string()
  is_disabled=true
/>
<List
  id_base="list-empty".to_string()
  items=Vec::<String>::new().into()
  selected_index=selected_empty.into()
  on_selected_index_change=Callback::new(move |next| set_selected_empty.set(next))
  aria_label="Empty list".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

let (controlled_selected, set_controlled_selected) = signal(Some(1_usize));

<List
  id_base="list-matrix-uncontrolled".to_string()
  items=items.clone()
  aria_label="Matrix uncontrolled list".to_string()
/>
<List
  id_base="list-matrix-controlled".to_string()
  items=items.clone()
  selected_index=controlled_selected.into()
  on_selected_index_change=Callback::new(move |next| set_controlled_selected.set(next))
  aria_label="Matrix controlled list".to_string()
/>
<List
  id_base="list-matrix-disabled".to_string()
  items=items
  selected_index=controlled_selected.into()
  on_selected_index_change=Callback::new(move |next| set_controlled_selected.set(next))
  aria_label="Matrix disabled list".to_string()
  is_disabled=true
/>"#
        .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec![
  "Overview".to_string(),
  "Billing".to_string(),
  "Integrations".to_string(),
  "Audit Logs".to_string(),
].into();

// List is Streaming Optional; fallback remains snapshot.
<div data-ui-streaming="optional" data-ui-fallback="snapshot">
  <AiSpace mode=Signal::derive(|| AiRenderMode::Snapshot) output_status=Signal::derive(|| AiOutputStatus::Verified)>
    <List id_base="docs-list-snapshot".to_string() items=items.clone() selected_index=snapshot_selected.into() on_selected_index_change=on_snapshot_change aria_label="Snapshot list".to_string() />
  </AiSpace>
  <AiSpace mode=Signal::derive(|| AiRenderMode::Streaming) output_status=Signal::derive(|| AiOutputStatus::Draft)>
    <List id_base="docs-list-streaming".to_string() items=items selected_index=streaming_selected.into() on_selected_index_change=on_streaming_change aria_label="Streaming list".to_string() />
  </AiSpace>
</div>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let custom_motion = workbench_custom_motion.get();

        let mut lines = vec![
            "let (selected, set_selected) = signal(Some(1_usize));".to_string(),
            "<List".to_string(),
            "  id_base=\"docs-list-workbench\".into()".to_string(),
            "  items=vec![".to_string(),
            "    \"Overview\".into(),".to_string(),
            "    \"Billing\".into(),".to_string(),
            "    \"Integrations\".into(),".to_string(),
            "    \"Audit Logs\".into(),".to_string(),
            "    \"Security\".into(),".to_string(),
            "  ].into()".to_string(),
            "  selected_index=selected.into()".to_string(),
            "  default_selected_index=Some(1)".to_string(),
            "  on_selected_index_change=Callback::new(move |next| set_selected.set(next))"
                .to_string(),
            "  id=\"docs-list-workbench-root\".into()".to_string(),
            "  aria_label=\"List workbench\".into()".to_string(),
            "  aria_labelledby=\"docs-list-workbench-heading\".into()".to_string(),
            "  on_action=Callback::new(move |_index| {})".to_string(),
            "  default_active_index=1".to_string(),
        ];

        if !sync_active {
            lines.push("  is_active_index_synced_to_selected=false".to_string());
        }
        if root_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if disable_last {
            lines.push("  disabled_indices=vec![4]".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-list-workbench--custom\".into()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".into()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        } else {
            lines.push("  lang=\"en-US\".into()".to_string());
            lines.push("  dir=A11yDirection::Ltr".to_string());
        }
        if custom_motion {
            lines.push(
                "  motion=ui::list::ListMotion { spring: ui::list::ListMotion::default().spring, highlight_scale: 1.03 }"
                    .to_string(),
            );
        } else {
            lines.push("  motion=ui::list::ListMotion::default()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/list/styles.rs */\n{}\n\n/* ListItem contract */\n{}\n\n/* ListSection contract */\n{}",
            ui::list::styles::CSS,
            ui::list::styles::ITEM_CSS,
            ui::list::styles::SECTION_CSS,
        )
    });

    let workbench_items_for_config = workbench_items.clone();
    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected.get();
        let sync_active = workbench_sync_active.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_last = workbench_disable_last.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let custom_motion = workbench_custom_motion.get();

        let mut class = vec!["ui-listbox".to_string()];
        if custom_class {
            class.push("docs-list-workbench--custom".to_string());
        }
        if root_disabled {
            class.push("data-disabled=true".to_string());
        }
        if disable_last {
            class.push("data-has-disabled-options=true".to_string());
        }

        format!(
            "ListWorkbenchConfig {{\n  id_base: Some(\"docs-list-workbench\"),\n  items: {:?},\n  selected_index: {selected:?},\n  default_selected_index: Some(1),\n  on_selected_index_change: \"bound(set_workbench_selected)\",\n  id: Some(\"docs-list-workbench-root\"),\n  aria_label: Some(\"List workbench\"),\n  aria_labelledby: Some(\"docs-list-workbench-heading\"),\n  lang: {:?},\n  dir: {:?},\n  is_disabled: {root_disabled},\n  disabled_indices: {},\n  on_action: \"count={} last={:?}\",\n  default_active_index: 1,\n  is_active_index_synced_to_selected: {sync_active},\n  motion: {:?},\n  class_name: {:?},\n  custom_class: {custom_class},\n  class: \"{}\",\n}}",
            workbench_items_for_config.clone(),
            if rtl { "ar" } else { "en-US" },
            if rtl { "rtl" } else { "ltr" },
            if disable_last { "vec![4]" } else { "vec![]" },
            workbench_action_count.get(),
            workbench_last_action.get(),
            if custom_motion { "custom" } else { "default" },
            if custom_class {
                Some("docs-list-workbench--custom")
            } else {
                None::<&str>
            },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="List"
            slug="list"
            group="Collections"
            description="List primitive with centralized root-state markers and optional active-index sync controls."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="默认路径：不接受控状态轴，仅传 `id_base + items + aria_label` 即可运行。"
                code_signal=hello_code
            >
                <div class="docs-stack" data-slot="list-hello" style="width: min(100%, 320px);">
                    <List
                        id_base="docs-list-hello".to_string()
                        items=hello_items
                        aria_label="Settings navigation".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="按钮式 workbench：单画布调参，支持 settings / code / css-test 面板联动。"
                code_signal=workbench_code
                code_imports=list_code_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/list/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_sync_active.get()
                                on:change=move |ev| set_workbench_sync_active.set(event_target_checked(&ev))
                            />
                            " Sync active index to selected"
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
                                prop:checked=move || workbench_root_disabled.get()
                                on:change=move |ev| set_workbench_root_disabled.set(event_target_checked(&ev))
                            />
                            " Disable root"
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
                                prop:checked=move || workbench_rtl.get()
                                on:change=move |ev| set_workbench_rtl.set(event_target_checked(&ev))
                            />
                            " RTL (lang + dir)"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="list-workbench" style="width: min(100%, 420px);">
                    <span id="docs-list-workbench-heading" class="ui-muted">"List workbench heading"</span>
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · disabled indices: "
                        {move || if workbench_disable_last.get() { "[4]" } else { "[]" }}
                        " · on_action: "
                        {move || format!("{} / {:?}", workbench_action_count.get(), workbench_last_action.get())}
                    </span>
                    {move || {
                        let disable_last = workbench_disable_last.get();
                        let root_disabled = workbench_root_disabled.get();
                        let sync_active = workbench_sync_active.get();
                        let custom_class = workbench_custom_class.get();
                        let rtl = workbench_rtl.get();
                        let custom_motion = workbench_custom_motion.get();

                        let class_name = if custom_class {
                            "docs-list-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let disabled_indices = if disable_last { vec![4] } else { Vec::new() };

                        view! {
                            <div class="docs-card" data-slot="list-workbench-canvas">
                                <List
                                    id_base="docs-list-workbench".to_string()
                                    items=workbench_items.clone()
                                    selected_index=workbench_selected.into()
                                    default_selected_index=1
                                    on_selected_index_change=Callback::new(move |next| set_workbench_selected.set(next))
                                    id="docs-list-workbench-root".to_string()
                                    aria_label="List workbench".to_string()
                                    aria_labelledby="docs-list-workbench-heading".to_string()
                                    lang=if rtl { "ar".to_string() } else { "en-US".to_string() }
                                    dir=if rtl { A11yDirection::Rtl } else { A11yDirection::Ltr }
                                    is_active_index_synced_to_selected=sync_active
                                    is_disabled=root_disabled
                                    disabled_indices=disabled_indices
                                    on_action=on_workbench_action
                                    default_active_index=1
                                    motion=if custom_motion {
                                        ui::list::ListMotion {
                                            spring: ui::list::ListMotion::default().spring,
                                        }
                                    } else {
                                        ui::list::ListMotion::default()
                                    }
                                    class_name=class_name
                                />
                            </div>
                        }
                        .into_any()
                    }}
                </div>
            </Playground>

            <Playground
                title="状态矩阵 State Matrix（受控 / 非受控）"
                description="同一组数据对照 uncontrolled / controlled / disabled 三种语义状态。"
                code_signal=state_matrix_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-state-matrix">
                    <div class="docs-stack">
                        <span class="ui-muted">"uncontrolled"</span>
                        <List
                            id_base="docs-list-matrix-uncontrolled".to_string()
                            items=showcase_items_for_matrix.clone()
                            aria_label="Matrix uncontrolled list".to_string()
                        />
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"controlled"</span>
                        <List
                            id_base="docs-list-matrix-controlled".to_string()
                            items=showcase_items_for_matrix.clone()
                            selected_index=state_matrix_controlled_selected.into()
                            on_selected_index_change=Callback::new(move |next| set_state_matrix_controlled_selected.set(next))
                            aria_label="Matrix controlled list".to_string()
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || state_matrix_controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <span class="ui-muted">"disabled"</span>
                        <List
                            id_base="docs-list-matrix-disabled".to_string()
                            items=showcase_items_for_matrix.clone()
                            selected_index=state_matrix_controlled_selected.into()
                            on_selected_index_change=Callback::new(move |next| set_state_matrix_controlled_selected.set(next))
                            aria_label="Matrix disabled list".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="展示：多场景"
                description="同一套 List 在默认、unsynced、disabled root、empty 四种状态下的行为对比。"
                code_signal=showcase_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-showcase">
                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"default + disabled option"</span>
                        <List
                            id_base="docs-list-default".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=showcase_selected_default.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_default.set(next))
                            aria_label="Default list".to_string()
                            disabled_indices=vec![2]
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_default.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"unsynced active index"</span>
                        <List
                            id_base="docs-list-unsynced".to_string()
                            items=showcase_items_for_showcase.clone()
                            selected_index=showcase_selected_unsynced.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_unsynced.set(next))
                            aria_label="Unsynced list".to_string()
                            is_active_index_synced_to_selected=false
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_unsynced.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"disabled root"</span>
                        <List
                            id_base="docs-list-disabled".to_string()
                            items=disabled_items
                            selected_index=showcase_selected_disabled.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_disabled.set(next))
                            aria_label="Disabled list".to_string()
                            is_disabled=true
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_disabled.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack" style="min-width: 220px;">
                        <span class="ui-muted">"empty list"</span>
                        <List
                            id_base="docs-list-empty".to_string()
                            items=empty_items
                            selected_index=showcase_selected_empty.into()
                            on_selected_index_change=Callback::new(move |next| set_showcase_selected_empty.set(next))
                            aria_label="Empty list".to_string()
                        />
                        <span class="ui-muted">
                            "selected: "
                            {move || showcase_selected_empty.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Controlled / Uncontrolled / Disabled Comparison)"
                code_signal=state_matrix_code
                code_imports=list_code_imports.clone()
            >
                <div class="docs-row" data-slot="list-state-matrix-after-workbench">
                    <div class="docs-stack">
                        <span class="ui-muted">"uncontrolled"</span>
                        <List
                            id_base="docs-list-matrix-after-uncontrolled".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            aria_label="Matrix uncontrolled list".to_string()
                        />
                    </div>
                    <div class="docs-stack">
                        <span class="ui-muted">"unsynced active index"</span>
                        <List
                            id_base="docs-list-matrix-after-unsynced".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            default_selected_index=1
                            aria_label="Matrix unsynced list".to_string()
                            is_active_index_synced_to_selected=false
                        />
                    </div>
                    <div class="docs-stack">
                        <span class="ui-muted">"disabled root"</span>
                        <List
                            id_base="docs-list-matrix-after-disabled".to_string()
                            items=showcase_items_for_matrix_after.clone()
                            aria_label="Matrix disabled list".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming/Snapshot Display"
                description="List 非正文阅读面：Streaming Optional，fallback=snapshot。"
                code_signal=output_mode_code
                code_imports=list_code_imports
            >
                <div class="docs-row" data-slot="list-streaming-snapshot">
                    <div
                        class="docs-stack"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="snapshot"
                    >
                        <AiSpace mode=list_snapshot_mode output_status=list_verified_output>
                            <List
                                id_base="docs-list-snapshot".to_string()
                                items=showcase_items_for_stream_snapshot
                                selected_index=snapshot_selected.into()
                                on_selected_index_change=Callback::new(move |next| set_snapshot_selected.set(next))
                                aria_label="Snapshot list".to_string()
                            />
                        </AiSpace>
                        <span class="ui-muted">"Snapshot baseline: verified + copy-ready."</span>
                    </div>

                    <div
                        class="docs-stack"
                        data-ui-streaming="optional"
                        data-ui-fallback="snapshot"
                        data-ui-output-state="streaming"
                    >
                        <AiSpace mode=list_streaming_mode output_status=list_draft_output>
                            <List
                                id_base="docs-list-streaming".to_string()
                                items=showcase_items_for_stream_streaming
                                selected_index=streaming_selected.into()
                                on_selected_index_change=Callback::new(move |next| set_streaming_selected.set(next))
                                aria_label="Streaming list".to_string()
                            />
                        </AiSpace>
                        <span class="ui-muted">
                            "Streaming preview keeps fallback=snapshot contract explicit."
                        </span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="list-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " with one-click copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::List;\n\nlet items: Arc<[String]> = vec![\"Overview\".to_string(), \"Billing\".to_string()].into();\n<List id_base=\"docs-list\".to_string() items=items aria_label=\"Settings navigation\".to_string() />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-list-source-copy".to_string()
                />
                <ul data-slot="list-source-paths">
                    <li><code>"components/list/src/mod.rs"</code></li>
                    <li><code>"components/list/src/logic.rs"</code></li>
                    <li><code>"components/list/src/view.rs"</code></li>
                    <li><code>"components/list/src/styles.rs"</code></li>
                    <li><code>"components/list/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="list-source-prerequisites">
                    <li><code>"component-list"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
