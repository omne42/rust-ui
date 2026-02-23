use super::*;

pub(crate) fn carousel() -> AnyView {
    let carousel_imports = CAROUSEL_DOC_IMPORTS.to_string();

    let base_items = vec![
        CarouselItem::new("welcome", "Welcome")
            .description("Build baseline-compatible surfaces with production-grade motion."),
        CarouselItem::new("tokens", "Theme Tokens")
            .description("Tune OKLCH and OLED tokens without breaking component contracts."),
        CarouselItem::new("shipping", "Shipping")
            .description("Run format + check + pre-commit and ship with confidence."),
    ];
    let base_items_for_default = base_items.clone();
    let base_items_for_stream = base_items.clone();

    let vertical_items = vec![
        CarouselItem::new("a", "Alpha").description("Vertical orientation demo."),
        CarouselItem::new("b", "Beta")
            .description("Second slide.")
            .disabled(true),
        CarouselItem::new("c", "Gamma").description("Loop disabled demo."),
    ];

    let marker_items = vec![
        CarouselItem::new("overview", "Overview")
            .description("Inspect source markers directly in DevTools."),
        CarouselItem::new("analytics", "Analytics")
            .description("Controlled index + motion markers for regressions."),
        CarouselItem::new("settings", "Settings")
            .description("Custom orientation and navigation mode markers.")
            .disabled(true),
    ];

    let (last_selected, set_last_selected) = signal(None::<usize>);
    let on_selected_change = Callback::new(move |next: Option<usize>| set_last_selected.set(next));

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some(0_usize));
    let controlled_selected: Signal<Option<usize>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<usize>| {
        set_controlled_selected_raw.set(next);
    });

    let (marker_selected_raw, set_marker_selected_raw) = signal(Some(1_usize));
    let marker_selected: Signal<Option<usize>> = Signal::derive(move || marker_selected_raw.get());
    let on_marker_selected_change = Callback::new(move |next: Option<usize>| {
        set_marker_selected_raw.set(next);
    });

    let state_matrix_options = vec![
        "Default".to_string(),
        "Empty".to_string(),
        "Disabled Middle".to_string(),
        "Vertical + No Loop".to_string(),
    ];
    let state_matrix_options_for_gallery = state_matrix_options.clone();
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_selected = Signal::derive(move || state_matrix_index.get().unwrap_or(0));
    let state_matrix_items = Signal::derive(move || match state_matrix_selected.get() {
        1 => Vec::<CarouselItem>::new(),
        2 => vec![
            CarouselItem::new("matrix-1", "Alpha").description("Default entry."),
            CarouselItem::new("matrix-2", "Beta")
                .description("Disabled item in this branch.")
                .disabled(true),
            CarouselItem::new("matrix-3", "Gamma").description("Remaining selectable entry."),
        ],
        3 => vec![
            CarouselItem::new("matrix-v1", "North").description("Vertical axis branch."),
            CarouselItem::new("matrix-v2", "South").description("Loop disabled branch."),
            CarouselItem::new("matrix-v3", "West").description("State matrix coverage."),
        ],
        _ => vec![
            CarouselItem::new("matrix-d1", "Overview").description("Default matrix branch."),
            CarouselItem::new("matrix-d2", "Metrics").description("Second matrix branch."),
            CarouselItem::new("matrix-d3", "Release").description("Third matrix branch."),
        ],
    });
    let state_matrix_orientation = Signal::derive(move || {
        if state_matrix_selected.get() == 3 {
            CarouselOrientation::Vertical
        } else {
            CarouselOrientation::Horizontal
        }
    });
    let state_matrix_is_loop = Signal::derive(move || state_matrix_selected.get() != 3);
    let state_matrix_code = Signal::derive(move || {
        r#"let (scenario, set_scenario) = signal(Some(0_usize));

<SegmentedControl
  id_base="docs-carousel-state-matrix-scenario".to_string()
  options=vec!["Default".to_string(), "Empty".to_string(), "Disabled Middle".to_string(), "Vertical + No Loop".to_string()]
  selected_index=scenario
  set_selected_index=set_scenario
/>

<Carousel
  id_base="docs-carousel-state-matrix".to_string()
  items=state_matrix_items.get()
  orientation=state_matrix_orientation.get()
  is_loop_navigation=state_matrix_is_loop.get()
/>"#
            .to_string()
    });

    let controlled_uncontrolled_items = vec![
        CarouselItem::new("cu-1", "Intro").description("Shared items for compare lane."),
        CarouselItem::new("cu-2", "Middle").description("Shared items for compare lane."),
        CarouselItem::new("cu-3", "Finish").description("Shared items for compare lane."),
    ];
    let (uncontrolled_last_selected, set_uncontrolled_last_selected) = signal(None::<usize>);
    let on_uncontrolled_selected_change =
        Callback::new(move |next: Option<usize>| set_uncontrolled_last_selected.set(next));
    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"let items = vec![
  CarouselItem::new("cu-1", "Intro"),
  CarouselItem::new("cu-2", "Middle"),
  CarouselItem::new("cu-3", "Finish"),
];
let (selected, set_selected) = signal(Some(0_usize));

<div class="docs-stack docs-stack--tight">
  <Carousel
    id_base="docs-carousel-controlled".to_string()
    items=items.clone()
    selected_index=Signal::derive(move || selected.get())
    on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  />

  <Carousel
    id_base="docs-carousel-uncontrolled".to_string()
    items=items.clone()
    default_selected_index=1
    on_selected_index_change=Callback::new(move |_| {})
  />
</div>"#
            .to_string()
    });

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let stream_requested_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot".to_string()
        } else {
            "streaming".to_string()
        }
    });
    let stream_requested_output_status = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "verified".to_string()
        } else {
            "draft".to_string()
        }
    });
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Streaming is optional for Carousel; fallback stays snapshot.
<Carousel
  id_base="docs-carousel-stream".to_string()
  items=vec![
    CarouselItem::new("stream-1", "Snapshot"),
    CarouselItem::new("stream-2", "Fallback"),
  ]
/>"#
        .to_string()
    });

    let minimal_code = Signal::derive(move || {
        r#"<Carousel
  id_base="docs-carousel".to_string()
  items=vec![CarouselItem::new("welcome", "Welcome")]
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (last_selected, set_last_selected) = signal(None::<usize>);

<Carousel
  id_base="docs-carousel".to_string()
  items=vec![
    CarouselItem::new("release-1", "Release 1").description("Faster build pipeline"),
    CarouselItem::new("release-2", "Release 2").description("New audit dashboard"),
    CarouselItem::new("release-3", "Release 3").description("Improved accessibility"),
  ]
  default_selected_index=1
  on_selected_index_change=Callback::new(move |next: Option<usize>| {
    set_last_selected.set(next);
  })
/>
<span class="ui-muted">"last selected: " {move || last_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}</span>"#.to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));

<Carousel
  id_base="docs-carousel-vertical".to_string()
  items=vec![
    CarouselItem::new("slide-a", "Slide A").description("First item"),
    CarouselItem::new("slide-b", "Slide B").description("Second item"),
    CarouselItem::new("slide-c", "Slide C").description("Third item"),
  ]
  selected_index=Signal::derive(move || selected.get())
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  orientation=CarouselOrientation::Vertical
  is_loop_navigation=false
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let mut custom_motion = ui::CarouselMotion::default();
custom_motion.spring.stiffness = 250.0;
custom_motion.spring.damping = 22.0;

<Carousel
  id_base="docs-carousel-markers".to_string()
  items=vec![
    CarouselItem::new("spotlight-1", "Spotlight 1").description("Migration complete"),
    CarouselItem::new("spotlight-2", "Spotlight 2").description("Latency reduced"),
    CarouselItem::new("spotlight-3", "Spotlight 3").description("Error rate stable"),
  ]
  selected_index=Signal::derive(move || selected.get())
  default_selected_index=0
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  orientation=CarouselOrientation::Vertical
  is_loop_navigation=false
  aria_label="Workspace spotlight".to_string()
  class_name="docs-carousel-custom".to_string()
  motion=custom_motion
/>"#
        .to_string()
    });

    let mut marker_motion = ui::CarouselMotion::default();
    marker_motion.spring.stiffness = 250.0;
    marker_motion.spring.damping = 22.0;

    let workbench_options = vec![
        "Baseline".to_string(),
        "Vertical + Custom Label".to_string(),
        "Disabled + Custom Motion".to_string(),
    ];
    let (workbench_index, set_workbench_index) = signal(Some(0_usize));
    let workbench_vertical = Signal::derive(move || workbench_index.get().unwrap_or(0) >= 1);
    let workbench_disabled = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let workbench_custom_text = Signal::derive(move || workbench_index.get().unwrap_or(0) >= 1);
    let workbench_custom_motion = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let (workbench_preserve_context, set_workbench_preserve_context) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let (workbench_selected_raw, set_workbench_selected_raw) = signal(Some(0_usize));
    let workbench_selected: Signal<Option<usize>> =
        Signal::derive(move || workbench_selected_raw.get());
    let (workbench_last_selected, set_workbench_last_selected) = signal("none".to_string());
    let on_workbench_selected_change = Callback::new(move |next: Option<usize>| {
        set_workbench_selected_raw.set(next);
        set_workbench_last_selected.set(
            next.map(|index| index.to_string())
                .unwrap_or_else(|| "none".to_string()),
        );
    });

    let reset_workbench_selected = set_workbench_selected_raw;
    let reset_workbench_last_selected = set_workbench_last_selected;
    Effect::new(move |_| {
        workbench_index.with(|_| ());
        if !workbench_preserve_context.get() {
            reset_workbench_selected.set(Some(0));
            reset_workbench_last_selected.set("none".to_string());
        }
    });

    let workbench_items = Signal::derive(move || {
        vec![
            CarouselItem::new(
                "workbench-overview",
                if workbench_custom_text.get() {
                    "Overview"
                } else {
                    "Welcome"
                },
            )
            .description("Inspect state/source markers under scenario toggles."),
            CarouselItem::new(
                "workbench-metrics",
                if workbench_custom_text.get() {
                    "Metrics"
                } else {
                    "Theme Tokens"
                },
            )
            .description("Middle item toggles disabled state in scenario #3.")
            .disabled(workbench_disabled.get()),
            CarouselItem::new(
                "workbench-release",
                if workbench_custom_text.get() {
                    "Release"
                } else {
                    "Shipping"
                },
            )
            .description("Verify keyboard + pointer flow in isolated canvas."),
        ]
    });

    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::CarouselMotion::default();
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 280.0;
            motion.spring.damping = 24.0;
        }
        motion
    });

    let workbench_code = Signal::derive(move || {
        let scenario = workbench_index.get().unwrap_or(0);
        let preserve = workbench_preserve_context.get();
        let selected = workbench_selected_raw.get();
        let aria_label = if workbench_custom_text.get() {
            "\"Workbench carousel\".to_string()"
        } else {
            "String::new()"
        };
        let class_name = if workbench_custom_text.get() {
            "\"docs-carousel-custom\".to_string()"
        } else {
            "String::new()"
        };
        let orientation_line = if workbench_vertical.get() {
            "  orientation=CarouselOrientation::Vertical\n"
        } else {
            "  orientation=CarouselOrientation::Horizontal\n"
        };
        let motion_line = if workbench_custom_motion.get() {
            "  motion={ let mut motion = ui::CarouselMotion::default(); motion.spring.stiffness = 280.0; motion.spring.damping = 24.0; motion }\n"
        } else {
            "  motion=ui::CarouselMotion::default()\n"
        };
        let lang_line = if workbench_lang_zh.get() {
            "  lang=\"zh-CN\".to_string()\n"
        } else {
            "  lang=\"en-US\".to_string()\n"
        };
        let dir_line = if workbench_rtl.get() {
            "  dir=ui_headless::A11yDirection::Rtl\n"
        } else {
            "  dir=ui_headless::A11yDirection::Ltr\n"
        };
        format!(
            "let (selected, set_selected) = signal({selected:?});\n\
let preserve_context = {preserve}; // optional\n\
// scenario: {scenario}\n\
<Carousel\n\
  id_base=\"docs-carousel-workbench\".to_string()\n\
  items=workbench_items\n\
  default_selected_index=0\n\
  selected_index=Signal::derive(move || selected.get())\n\
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))\n\
  is_loop_navigation={}\n\
  aria_label={aria_label}\n\
  controls_aria_label=\"Carousel controls\".to_string()\n\
  indicators_aria_label=\"Carousel indicators\".to_string()\n\
  previous_label=\"Previous slide\".to_string()\n\
  next_label=\"Next slide\".to_string()\n\
  indicator_aria_label_template=\"Go to slide {{index}}\".to_string()\n\
{orientation_line}{lang_line}{dir_line}  class_name={class_name}\n\
{motion_line}/>",
            !workbench_disabled.get(),
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/carousel/src/styles.rs */\n{}",
            ui::carousel::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let scenario = workbench_index.get().unwrap_or(0);
        let is_vertical = workbench_vertical.get();
        let is_loop_navigation = !workbench_disabled.get();
        let class_name = if workbench_custom_text.get() {
            Some("docs-carousel-custom")
        } else {
            None
        };
        format!(
            "CarouselWorkbenchConfig {{\n  id_base: \"docs-carousel-workbench\",\n  items: [\"workbench-overview\", \"workbench-metrics\", \"workbench-release\"],\n  default_selected_index: Some(0),\n  on_selected_index_change: \"last_selected={:?}\",\n  orientation: {:?},\n  is_loop_navigation: {},\n  aria_label: {:?},\n  controls_aria_label: Some(\"Carousel controls\"),\n  indicators_aria_label: Some(\"Carousel indicators\"),\n  previous_label: Some(\"Previous slide\"),\n  next_label: Some(\"Next slide\"),\n  indicator_aria_label_template: Some(\"Go to slide {{index}}\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {class_name:?},\n  scenario: {scenario},\n  preserve_context: {},\n  selected_index: {:?},\n  vertical: {},\n  disabled_middle_item: {},\n  custom_text: {},\n  custom_motion: {},\n}}",
            workbench_last_selected.get(),
            if is_vertical {
                CarouselOrientation::Vertical
            } else {
                CarouselOrientation::Horizontal
            },
            is_loop_navigation,
            if workbench_custom_text.get() {
                Some("Workbench carousel")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            workbench_preserve_context.get(),
            workbench_selected_raw.get(),
            is_vertical,
            workbench_disabled.get(),
            workbench_custom_text.get(),
            workbench_custom_motion.get(),
        )
    });

    view! {
        <ComponentPage
            title="Carousel"
            slug="carousel"
            group="Collections"
            description="baseline-compatible carousel with controllable slide index, orientation-aware keyboard navigation, baseline data contracts, and baseline-level spring indicator-highlight motion reuse."
        >
            <Playground title="Hello World (Minimal)" code_signal=minimal_code>
                <Carousel
                    id_base="docs-carousel-minimal".to_string()
                    items=vec![CarouselItem::new("welcome", "Welcome")]
                />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Workbench canvas: scoped CSS live-edit + optional selected-index context persistence across scenario switches."
                code_signal=workbench_code
                code_imports=carousel_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/carousel/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="carousel-workbench-controls">
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-carousel-workbench-scenario".to_string()
                            options=workbench_options.clone()
                            selected_index=workbench_index
                            set_selected_index=set_workbench_index
                            size=SegmentedControlSize::Sm
                            aria_label="Carousel workbench scenario".to_string()
                        />
                        <Switch
                            checked=workbench_preserve_context
                            set_checked=set_workbench_preserve_context
                        >
                            " Preserve selected index context (optional)"
                        </Switch>
                        <div class="ui-muted">
                            "vertical: "
                            {move || workbench_vertical.get()}
                        </div>
                        <div class="ui-muted">
                            "disabled_middle_item: "
                            {move || workbench_disabled.get()}
                        </div>
                        <div class="ui-muted">
                            "custom_text: "
                            {move || workbench_custom_text.get()}
                        </div>
                        <div class="ui-muted">
                            "custom_motion: "
                            {move || workbench_custom_motion.get()}
                        </div>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="carousel-workbench">
                    <div class="docs-row" data-slot="carousel-workbench-actions">
                        <button
                            type="button"
                            data-slot="carousel-workbench-select-0"
                            on:click=move |_| set_workbench_selected_raw.set(Some(0))
                        >
                            "Select #0"
                        </button>
                        <button
                            type="button"
                            data-slot="carousel-workbench-select-1"
                            on:click=move |_| set_workbench_selected_raw.set(Some(1))
                        >
                            "Select #1"
                        </button>
                        <button
                            type="button"
                            data-slot="carousel-workbench-clear"
                            on:click=move |_| set_workbench_selected_raw.set(None)
                        >
                            "Clear"
                        </button>
                    </div>
                    <div data-slot="carousel-workbench-canvas">
                        <Carousel
                            id_base="docs-carousel-workbench".to_string()
                            items=workbench_items.get()
                            default_selected_index=0
                            selected_index=workbench_selected
                            on_selected_index_change=on_workbench_selected_change
                            orientation=if workbench_vertical.get() {
                                CarouselOrientation::Vertical
                            } else {
                                CarouselOrientation::Horizontal
                            }
                            is_loop_navigation=!workbench_disabled.get()
                            aria_label=if workbench_custom_text.get() {
                                "Workbench carousel".to_string()
                            } else {
                                String::new()
                            }
                            controls_aria_label="Carousel controls".to_string()
                            indicators_aria_label="Carousel indicators".to_string()
                            previous_label="Previous slide".to_string()
                            next_label="Next slide".to_string()
                            indicator_aria_label_template="Go to slide {index}".to_string()
                            lang=if workbench_lang_zh.get() {
                                "zh-CN".to_string()
                            } else {
                                "en-US".to_string()
                            }
                            dir=if workbench_rtl.get() {
                                ui_headless::A11yDirection::Rtl
                            } else {
                                ui_headless::A11yDirection::Ltr
                            }
                            class_name=if workbench_custom_text.get() {
                                "docs-carousel-custom".to_string()
                            } else {
                                String::new()
                            }
                            motion=workbench_motion.get()
                        />
                    </div>
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            workbench_selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted" data-slot="carousel-workbench-last-selected">
                        "last selected: "
                        {move || workbench_last_selected.get()}
                    </span>
                    <span class="ui-muted">
                        "persist_context: "
                        {move || workbench_preserve_context.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Empty / Disabled / Vertical)"
                description="Switch between default/empty/disabled/vertical branches and verify state markers."
                code_signal=state_matrix_code
                code_imports=carousel_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="carousel-state-matrix-final">
                    <SegmentedControl
                        id_base="docs-carousel-state-matrix-final-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Carousel state matrix scenario".to_string()
                    />

                    <Carousel
                        id_base="docs-carousel-state-matrix-final".to_string()
                        items=state_matrix_items.get()
                        default_selected_index=0
                        orientation=state_matrix_orientation.get()
                        is_loop_navigation=state_matrix_is_loop.get()
                        controls_aria_label="Carousel controls".to_string()
                        indicators_aria_label="Carousel indicators".to_string()
                        previous_label="Previous slide".to_string()
                        next_label="Next slide".to_string()
                        indicator_aria_label_template="Go to slide {index}".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        motion=ui::CarouselMotion::default()
                    />

                    <span class="ui-muted">
                        "state mode: "
                        {move || match state_matrix_selected.get() {
                            0 => "default",
                            1 => "empty",
                            2 => "disabled-middle",
                            _ => "vertical-no-loop",
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Default + Indicator Motion" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Carousel
                        id_base="docs-carousel-default".to_string()
                        items=base_items_for_default
                        default_selected_index=1
                        on_selected_index_change=on_selected_change
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            last_selected
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Scenario Gallery"
                description="Switch between default/empty/disabled/vertical branches and verify state markers."
                code_signal=state_matrix_code
                code_imports=carousel_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="carousel-state-matrix">
                    <SegmentedControl
                        id_base="docs-carousel-state-matrix-scenario".to_string()
                        options=state_matrix_options_for_gallery.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Carousel state matrix scenario".to_string()
                    />

                    <Carousel
                        id_base="docs-carousel-state-matrix".to_string()
                        items=state_matrix_items.get()
                        orientation=state_matrix_orientation.get()
                        is_loop_navigation=state_matrix_is_loop.get()
                    />

                    <span class="ui-muted">
                        "state mode: "
                        {move || match state_matrix_selected.get() {
                            0 => "default",
                            1 => "empty",
                            2 => "disabled-middle",
                            _ => "vertical-no-loop",
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side compare `selected_index + on_selected_index_change` versus `default_selected_index` paths."
                code_signal=controlled_uncontrolled_code
                code_imports=carousel_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="carousel-controlled-uncontrolled">
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Controlled"</strong>
                        <Carousel
                            id_base="docs-carousel-controlled".to_string()
                            items=controlled_uncontrolled_items.clone()
                            selected_index=controlled_selected
                            on_selected_index_change=on_controlled_selected_change
                        />
                        <span class="ui-muted">
                            "controlled selected: "
                            {move || {
                                controlled_selected_raw
                                    .get()
                                    .map(|index| index.to_string())
                                    .unwrap_or_else(|| "None".to_string())
                            }}
                        </span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <strong>"Uncontrolled"</strong>
                        <Carousel
                            id_base="docs-carousel-uncontrolled".to_string()
                            items=controlled_uncontrolled_items.clone()
                            default_selected_index=1
                            on_selected_index_change=on_uncontrolled_selected_change
                        />
                        <span class="ui-muted">
                            "default selected: 1"
                        </span>
                        <span class="ui-muted">
                            "last selected: "
                            {move || {
                                uncontrolled_last_selected
                                    .get()
                                    .map(|index| index.to_string())
                                    .unwrap_or_else(|| "None".to_string())
                            }}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled + Vertical + No Loop" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Carousel
                        id_base="docs-carousel-vertical".to_string()
                        items=vertical_items
                        selected_index=controlled_selected
                        on_selected_index_change=on_controlled_selected_change
                        orientation=CarouselOrientation::Vertical
                        is_loop_navigation=false
                        aria_label="Feature carousel".to_string()
                        class_name="docs-carousel-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled selected: "
                        {move || {
                            controlled_selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight" data-slot="carousel-e2e-markers">
                    <div class="docs-row" data-slot="carousel-e2e-marker-actions">
                        <button
                            type="button"
                            data-slot="carousel-e2e-select-overview"
                            on:click=move |_| set_marker_selected_raw.set(Some(0))
                        >
                            "Select Overview"
                        </button>
                        <button
                            type="button"
                            data-slot="carousel-e2e-select-analytics"
                            on:click=move |_| set_marker_selected_raw.set(Some(1))
                        >
                            "Select Analytics"
                        </button>
                        <button
                            type="button"
                            data-slot="carousel-e2e-clear"
                            on:click=move |_| set_marker_selected_raw.set(None)
                        >
                            "Clear"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-orientation-source / data-loop-navigation-source / data-selected-index-source / data-selected-index-change-source / data-motion-source in DevTools."
                    </div>
                    <Carousel
                        id_base="docs-carousel-markers".to_string()
                        items=marker_items
                        selected_index=marker_selected
                        default_selected_index=0
                        on_selected_index_change=on_marker_selected_change
                        orientation=CarouselOrientation::Vertical
                        is_loop_navigation=false
                        aria_label="Workspace spotlight".to_string()
                        class_name="docs-carousel-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            marker_selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Carousel is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=carousel_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="carousel-streaming-policy"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-carousel-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Carousel requested stream mode".to_string()
                    />
                    <Carousel
                        id_base="docs-carousel-stream".to_string()
                        items=base_items_for_stream
                        default_selected_index=0
                    />
                    <span class="ui-muted">
                        "requested mode: "
                        {move || stream_requested_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "requested output status: "
                        {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "Streaming Optional; fallback=snapshot."
                    </span>
                    <span class="ui-muted">
                        "effective component markers: data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" data-slot="carousel-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted" data-slot="carousel-copy-ready-hint">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground and the CodeBlock "
                    <code>"Copy"</code>
                    " action to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"CAROUSEL_DOC_IMPORTS"</code>
                    " + "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-carousel\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" data-slot="carousel-source-paths">
                    <li><code>"components/carousel/src/mod.rs"</code></li>
                    <li><code>"components/carousel/src/logic.rs"</code></li>
                    <li><code>"components/carousel/src/view.rs"</code></li>
                    <li><code>"components/carousel/src/styles.rs"</code></li>
                    <li><code>"components/carousel/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
