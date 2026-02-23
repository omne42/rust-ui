use super::*;

pub(crate) fn chart() -> AnyView {
    let revenue_points = vec![
        ChartPoint::new("jan", "Jan", 12.0),
        ChartPoint::new("feb", "Feb", 18.5),
        ChartPoint::new("mar", "Mar", 17.2),
        ChartPoint::new("apr", "Apr", 24.7),
        ChartPoint::new("may", "May", 28.1),
    ];

    let line_points = vec![
        ChartPoint::new("q1", "Q1", 42.0),
        ChartPoint::new("q2", "Q2", 56.0),
        ChartPoint::new("q3", "Q3", 51.0),
        ChartPoint::new("q4", "Q4", 63.0),
    ];
    let flat_points = vec![
        ChartPoint::new("alpha", "Alpha", 20.0),
        ChartPoint::new("beta", "Beta", 20.0),
        ChartPoint::new("gamma", "Gamma", 20.0),
    ];
    let revenue_points_for_workbench = revenue_points.clone();
    let line_points_for_workbench = line_points.clone();
    let flat_points_for_workbench = flat_points.clone();
    let revenue_points_for_matrix = revenue_points.clone();
    let line_points_for_matrix = line_points.clone();
    let flat_points_for_matrix = flat_points.clone();
    let revenue_points_for_bar = revenue_points.clone();
    let line_points_for_controlled = line_points.clone();
    let revenue_points_for_contrast = revenue_points_for_matrix.clone();
    let line_points_for_contrast = line_points_for_controlled.clone();
    let line_points_for_line = line_points_for_controlled.clone();
    let line_points_for_stream = line_points_for_controlled.clone();

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (controlled_active_raw, set_controlled_active_raw) = signal(1_usize);
    let controlled_active: Signal<usize> = Signal::derive(move || controlled_active_raw.get());
    let on_controlled_active_change =
        Callback::new(move |next: usize| set_controlled_active_raw.set(next));

    let kind_options = vec!["bar".to_string(), "line".to_string()];
    let dataset_options = vec![
        "revenue".to_string(),
        "growth".to_string(),
        "flat".to_string(),
    ];
    let persisted_workbench_state = load_chart_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_kind_index, set_workbench_kind_index) =
        signal(Some(initial_workbench_state.kind_index));
    let (workbench_dataset_index, set_workbench_dataset_index) =
        signal(Some(initial_workbench_state.dataset_index));
    let (workbench_is_disabled, set_workbench_is_disabled) =
        signal(initial_workbench_state.is_disabled);
    let (workbench_is_show_grid, set_workbench_is_show_grid) =
        signal(initial_workbench_state.is_show_grid);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.custom_class);
    let (workbench_lang, set_workbench_lang) = signal(initial_workbench_state.lang);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_active_raw, set_workbench_active_raw) = signal(0_usize);
    let workbench_active_signal: Signal<usize> = Signal::derive(move || workbench_active_raw.get());
    let workbench_on_active_index_change =
        Callback::new(move |next: usize| set_workbench_active_raw.set(next));
    let (workbench_last_action, set_workbench_last_action) = signal("none".to_string());
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let workbench_on_action = Callback::new(move |id: String| set_workbench_last_action.set(id));

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_chart_workbench_state(ChartWorkbenchState {
                kind_index: workbench_kind_index.get().unwrap_or(0),
                dataset_index: workbench_dataset_index.get().unwrap_or(0),
                is_disabled: workbench_is_disabled.get(),
                is_show_grid: workbench_is_show_grid.get(),
                custom_class: workbench_custom_class.get(),
                lang: workbench_lang.get(),
            });
        } else {
            clear_chart_workbench_state();
        }
    });

    let workbench_kind = Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
        1 => ChartKind::Line,
        _ => ChartKind::Bar,
    });
    let workbench_dataset_name: Signal<&'static str> =
        Signal::derive(move || match workbench_dataset_index.get().unwrap_or(0) {
            1 => "growth",
            2 => "flat",
            _ => "revenue",
        });
    let workbench_points =
        Signal::derive(move || match workbench_dataset_index.get().unwrap_or(0) {
            1 => line_points_for_workbench.clone(),
            2 => flat_points_for_workbench.clone(),
            _ => revenue_points_for_workbench.clone(),
        });

    let hello_code = Signal::derive(move || {
        r#"<Chart points=vec![ChartPoint::new("jan", "Jan", 12.0), ChartPoint::new("feb", "Feb", 18.5), ChartPoint::new("mar", "Mar", 17.2)] />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let is_disabled = workbench_is_disabled.get();
        let is_show_grid = workbench_is_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let active_index = workbench_active_raw.get();

        let mut out = vec![
            "<Chart".to_string(),
            "  id_base=\"docs-chart-workbench\".into()".to_string(),
            format!("  // dataset: {dataset}"),
            "  points=/* see preview dataset */".to_string(),
            format!("  kind=ChartKind::{kind:?}"),
            format!("  active_index=Signal::derive(move || {active_index})"),
            "  default_active_index=Some(1)".to_string(),
            "  on_active_index_change=Callback::new(move |next| { /* set active */ })".to_string(),
            "  aria_label=\"Revenue trend chart\".into()".to_string(),
            format!("  dir={dir}"),
            "  motion=ChartMotion::default()".to_string(),
        ];

        if is_disabled {
            out.push("  is_disabled=true".to_string());
        }
        if !is_show_grid {
            out.push("  is_show_grid=false".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-chart-custom\".into()".to_string());
        }
        if lang {
            out.push("  lang=\"en-US\".into()".to_string());
        }
        out.push("  on_action=Callback::new(move |id: String| { /* ... */ })".to_string());
        out.push("/>".to_string());

        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let points = workbench_points.get();
        let is_disabled = workbench_is_disabled.get();
        let is_show_grid = workbench_is_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let active_index = workbench_active_raw.get();

        let mut class_tokens = vec![
            "ui-chart".to_string(),
            match kind {
                ChartKind::Bar => "ui-chart--bar".to_string(),
                ChartKind::Line => "ui-chart--line".to_string(),
            },
            if is_disabled {
                "ui-chart--disabled".to_string()
            } else {
                "ui-chart--uncontrolled".to_string()
            },
        ];
        if is_show_grid {
            class_tokens.push("ui-chart--grid".to_string());
        }
        if custom_class {
            class_tokens.push("ui-chart--custom-class".to_string());
            class_tokens.push("docs-chart-custom".to_string());
        }

        format!(
            "ChartActualConfig {{\n  points: {points:?},\n  id_base: Some(\"docs-chart-workbench\"),\n  kind: {kind:?},\n  active_index: Some({active_index}),\n  default_active_index: Some(1),\n  on_active_index_change: Some(\"workbench_on_active_index_change\"),\n  on_action: Some(\"workbench_on_action\"),\n  is_disabled: {is_disabled},\n  is_show_grid: {is_show_grid},\n  motion: ChartMotion::default(),\n  aria_label: Some(\"Revenue trend chart\"),\n  class_name: {class_name},\n  lang: {lang_value},\n  dir: Some({dir}),\n  dataset: \"{dataset}\",\n  class: \"{class_tokens}\",\n  marker_expectations: [\"data-kind\", \"data-state\", \"data-active-index\", \"data-motion-source\"],\n}}",
            class_name = if custom_class {
                "Some(\"docs-chart-custom\")"
            } else {
                "None"
            },
            lang_value = if lang { "Some(\"en-US\")" } else { "None" },
            class_tokens = class_tokens.join(" "),
        )
    });

    let chart_test_css_source = Signal::derive(move || {
        format!(
            "/* components/chart/src/styles.rs */\n{}",
            ui::chart::styles::CSS
        )
    });

    let bar_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Chart
  id_base="docs-chart-bar".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
    ChartPoint::new("apr", "Apr", 24.7),
    ChartPoint::new("may", "May", 28.1),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let line_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="docs-chart-line".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
  class_name="docs-chart-custom".to_string()
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<Chart id_base="docs-chart-matrix-bar".to_string() kind=ChartKind::Bar points=vec![...] />
<Chart id_base="docs-chart-matrix-line".to_string() kind=ChartKind::Line points=vec![...] />
<Chart id_base="docs-chart-matrix-disabled".to_string() kind=ChartKind::Bar is_disabled=true points=vec![...] />
<Chart id_base="docs-chart-matrix-empty".to_string() kind=ChartKind::Line points=vec![] />"#.to_string()
    });

    let chart_imports =
        "use leptos::prelude::*;\nuse ui::{Chart, ChartKind, ChartPoint};".to_string();

    let controlled_contrast_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="docs-chart-uncontrolled-contrast".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
  ]
  kind=ChartKind::Bar
/>

<Chart
  id_base="docs-chart-controlled-contrast".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Chart
  id_base="docs-chart-streaming-snapshot".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  aria_label="Snapshot contract marker".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Chart
  id_base="docs-chart-source-first".to_string()
  points=vec![
    ChartPoint::new("apr", "Apr", 24.7),
    ChartPoint::new("may", "May", 28.1),
    ChartPoint::new("jun", "Jun", 31.6),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| {
    logging::log!("clicked point: {id}");
  })
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Chart"
            slug="chart"
            group="Display"
            description="baseline-compatible chart primitive with bar/line modes, controlled active-index state, baseline-style data contracts, and baseline-level spring highlight motion for legends."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <Chart id_base="docs-chart-hello".to_string() points=vec![ChartPoint::new("jan", "Jan", 12.0), ChartPoint::new("feb", "Feb", 18.5), ChartPoint::new("mar", "Mar", 17.2)] />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=chart_imports.clone()
                test_css_source=chart_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/chart/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Kind"</div>
                            <SegmentedControl
                                id_base="docs-chart-kind".to_string()
                                options=kind_options.clone()
                                selected_index=workbench_kind_index
                                set_selected_index=set_workbench_kind_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart kind".to_string()
                            />

                            <div class="docs-search__label">"Dataset"</div>
                            <SegmentedControl
                                id_base="docs-chart-dataset".to_string()
                                options=dataset_options.clone()
                                selected_index=workbench_dataset_index
                                set_selected_index=set_workbench_dataset_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart dataset".to_string()
                            />

                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "Disabled"
                            </Switch>
                            <Switch checked=workbench_is_show_grid set_checked=set_workbench_is_show_grid>
                                "Show Grid"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                            <div data-slot="chart-workbench-toggle-lang">
                                <Switch checked=workbench_lang set_checked=set_workbench_lang>
                                    "Lang=en-US"
                                </Switch>
                            </div>
                            <div data-slot="chart-workbench-toggle-rtl">
                                <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                                    "Dir=rtl"
                                </Switch>
                            </div>
                            <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                                "Persist workbench state"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-workbench">
                    {move || {
                        let points = workbench_points.get();
                        let kind = workbench_kind.get();
                        let disabled = workbench_is_disabled.get();
                        let is_show_grid = workbench_is_show_grid.get();
                        let class_name = workbench_custom_class
                            .get()
                            .then_some("docs-chart-custom".to_string());
                        let lang = workbench_lang.get().then_some("en-US".to_string());
                        let dir = if workbench_rtl_dir.get() {
                            ui::A11yDirection::Rtl
                        } else {
                            ui::A11yDirection::Ltr
                        };
                        let persist = workbench_persist_state.get();

                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="chart-workbench-canvas">
                                <span class="ui-muted">
                                    "persist: "
                                    {if persist { "on" } else { "off" }}
                                </span>
                                <Chart
                                    id_base="docs-chart-workbench".to_string()
                                    points=points
                                    kind=kind
                                    active_index=workbench_active_signal
                                    default_active_index=1
                                    on_active_index_change=workbench_on_active_index_change
                                    is_disabled=disabled
                                    is_show_grid=is_show_grid
                                    motion=ui::ChartMotion::default()
                                    aria_label="Revenue trend chart".to_string()
                                    class_name=class_name.unwrap_or_default()
                                    lang=lang.unwrap_or_default()
                                    dir=dir
                                    on_action=workbench_on_action
                                />
                            </div>
                        }
                    }}
                    <span class="ui-muted">
                        "workbench last action: "
                        {move || workbench_last_action.get()}
                        " · active: "
                        {move || workbench_active_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Bar / Line / Disabled / Empty)"
                code_signal=matrix_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Bar / Revenue"</span>
                        <Chart
                            id_base="docs-chart-matrix-bar".to_string()
                            points=revenue_points_for_matrix.clone()
                            kind=ChartKind::Bar
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Line / Growth"</span>
                        <Chart
                            id_base="docs-chart-matrix-line".to_string()
                            points=line_points_for_matrix.clone()
                            kind=ChartKind::Line
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Disabled"</span>
                        <div data-slot="chart-e2e-state-disabled">
                            <Chart
                                id_base="docs-chart-matrix-disabled".to_string()
                                points=flat_points_for_matrix.clone()
                                kind=ChartKind::Bar
                                is_disabled=true
                            />
                        </div>
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Empty"</span>
                        <Chart
                            id_base="docs-chart-matrix-empty".to_string()
                            points=vec![]
                            kind=ChartKind::Line
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                description="Compare default uncontrolled behavior with external active-index control."
                code_signal=controlled_contrast_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Uncontrolled / internal state"</span>
                        <Chart
                            id_base="docs-chart-uncontrolled-contrast".to_string()
                            points=revenue_points_for_contrast
                            kind=ChartKind::Bar
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Controlled / external signal"</span>
                        <Chart
                            id_base="docs-chart-controlled-contrast".to_string()
                            points=line_points_for_contrast
                            kind=ChartKind::Line
                            active_index=controlled_active
                            on_active_index_change=on_controlled_active_change
                        />
                        <span class="ui-muted">
                            "active index: "
                            {move || controlled_active_raw.get()}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Bar + Hover/Keyboard + Action"
                code_signal=bar_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-bar".to_string()
                        points=revenue_points_for_bar.clone()
                        kind=ChartKind::Bar
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled Line + Active Index"
                code_signal=line_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div data-slot="chart-e2e-controlled-line">
                        <Chart
                            id_base="docs-chart-line".to_string()
                            points=line_points_for_line
                            kind=ChartKind::Line
                            active_index=controlled_active
                            on_active_index_change=on_controlled_active_change
                            aria_label="Quarterly growth line chart".to_string()
                            class_name="docs-chart-custom".to_string()
                        />
                    </div>
                    <span class="ui-muted">
                        "controlled active index: "
                        {move || controlled_active_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Chart is `Streaming Optional`; component rendering remains snapshot-based with `fallback=snapshot`."
                code_signal=stream_snapshot_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-streaming-policy">
                    <Chart
                        id_base="docs-chart-streaming-snapshot".to_string()
                        points=line_points_for_stream
                        kind=ChartKind::Line
                        aria_label="Snapshot contract marker".to_string()
                    />
                    <span class="ui-muted" data-slot="chart-streaming-policy-note">
                        "Streaming Optional; fallback=snapshot."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects imports for direct run. Source: components/chart/src/{mod,logic,view,styles,motion}.rs. Dependency baseline: ui = { default-features = false, features = [\"component-chart\", \"inject-css\"] } + mount under UiRoot."
                code_signal=source_first_code
                code_imports=chart_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-source-first">
                    <h3>"Source-first / Copy-Paste Ready"</h3>
                    <span class="ui-muted" data-slot="chart-copy-ready-hint">
                        "Playground copy action injects missing imports through "
                        <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                        "."
                    </span>
                    <span class="ui-muted">
                        "Dependency baseline (Cargo.toml): "
                        <code>
                            "ui = { default-features = false, features = [\"component-chart\", \"inject-css\"] }"
                        </code>
                    </span>
                    <ul class="ui-muted" data-slot="chart-source-paths">
                        <li><code>"components/chart/src/mod.rs"</code></li>
                        <li><code>"components/chart/src/logic.rs"</code></li>
                        <li><code>"components/chart/src/view.rs"</code></li>
                        <li><code>"components/chart/src/styles.rs"</code></li>
                        <li><code>"components/chart/src/motion.rs"</code></li>
                    </ul>
                    <div class="docs-stack docs-stack--tight" data-slot="chart-parameter-matrix">
                        <h4>"Parameter Matrix (API Names + Defaults)"</h4>
                        <ul class="ui-muted">
                            <li><code>"kind"</code>" -> default "<code>"ChartKind::Bar"</code></li>
                            <li><code>"default_active_index"</code>" -> default "<code>"0 (clamped)"</code></li>
                            <li><code>"is_disabled"</code>" -> default "<code>"false"</code></li>
                            <li><code>"is_show_grid"</code>" -> default "<code>"true"</code></li>
                            <li><code>"id_base"</code>" -> default "<code>"\"ui-chart\""</code></li>
                            <li><code>"aria_label"</code>" -> default "<code>"\"Chart\""</code></li>
                        </ul>
                    </div>
                    <div class="docs-stack docs-stack--tight" data-slot="chart-state-matrix-summary">
                        <h4>"State Matrix Coverage"</h4>
                        <ul class="ui-muted">
                            <li>"Controlled vs uncontrolled: explicit side-by-side playground."</li>
                            <li>"Disabled/empty/kind branches: covered by Comparison Matrix."</li>
                            <li>"Size/variant: N/A for Chart (not part of current public API axis)."</li>
                        </ul>
                    </div>
                    <span class="ui-muted">"Mount under UiRoot to ensure theme vars and css injection."</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
