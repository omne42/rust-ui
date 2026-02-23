use super::*;

pub(crate) fn command() -> AnyView {
    let groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Suggestions",
            vec![
                CommandItem::new("calendar", "Calendar")
                    .keywords(vec!["date".to_string(), "event".to_string()])
                    .shortcut("⌘K"),
                CommandItem::new("search-emoji", "Search Emoji")
                    .keywords(vec!["emoji".to_string(), "icon".to_string()])
                    .shortcut("⌘E"),
                CommandItem::new("calculator", "Calculator")
                    .keywords(vec!["math".to_string(), "compute".to_string()]),
            ],
        ),
        CommandGroup::new(
            "Settings",
            vec![
                CommandItem::new("profile", "Profile").shortcut("⌘P"),
                CommandItem::new("billing", "Billing").shortcut("⌘B"),
                CommandItem::new("team", "Team").disabled(true),
            ],
        ),
    ]);

    let custom_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Quick Actions",
            vec![
                CommandItem::new("new-file", "New File")
                    .keywords(vec!["create".to_string(), "document".to_string()])
                    .shortcut("⌘N"),
                CommandItem::new("new-window", "New Window")
                    .keywords(vec!["window".to_string(), "workspace".to_string()])
                    .shortcut("⌘⇧N"),
            ],
        ),
        CommandGroup::new(
            "Account",
            vec![
                CommandItem::new("preferences", "Preferences").shortcut("⌘,"),
                CommandItem::new("manage-billing", "Manage Billing").shortcut("⌘⇧B"),
                CommandItem::new("admin-only", "Admin Only").disabled(true),
            ],
        ),
    ]);

    let marker_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Workspace",
            vec![
                CommandItem::new("open-recent", "Open Recent")
                    .keywords(vec!["recent".to_string(), "workspace".to_string()])
                    .shortcut("⌘R"),
                CommandItem::new("new-workspace", "New Workspace")
                    .keywords(vec!["create".to_string(), "workspace".to_string()])
                    .shortcut("⌘⇧W"),
            ],
        ),
        CommandGroup::new(
            "Automation",
            vec![
                CommandItem::new("run-tests", "Run Tests")
                    .keywords(vec!["test".to_string(), "verify".to_string()]),
                CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
            ],
        ),
    ]);

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_custom_action, set_last_custom_action) = signal("none".to_string());
    let on_custom_action = Callback::new(move |id: String| set_last_custom_action.set(id));

    let (last_marker_action, set_last_marker_action) = signal("none".to_string());
    let on_marker_action = Callback::new(move |id: String| set_last_marker_action.set(id));

    let hello_groups: Arc<[CommandGroup]> = Arc::from(vec![CommandGroup::new(
        "Quick Start",
        vec![CommandItem::new("open-dashboard", "Open Dashboard")],
    )]);

    let hello_code = Signal::derive(move || {
        r#"<Command
  id_base="docs-command-hello".to_string()
  groups=Arc::from(vec![CommandGroup::new("Quick Start", vec![CommandItem::new("open-dashboard", "Open Dashboard")])])
/>"#
            .to_string()
    });

    let command_api_groups = groups.clone();
    let (command_api_query_raw, set_command_api_query_raw) = signal("cal".to_string());
    let command_api_query: Signal<String> = Signal::derive(move || command_api_query_raw.get());
    let (command_api_disabled, set_command_api_disabled) = signal(false);
    let (command_api_rtl, set_command_api_rtl) = signal(false);
    let (command_api_custom_motion, set_command_api_custom_motion) = signal(false);
    let (command_api_custom_class, set_command_api_custom_class) = signal(false);
    let (command_api_query_change_runs, set_command_api_query_change_runs) = signal(0_u32);
    let on_command_api_query_change = Callback::new(move |next: String| {
        set_command_api_query_raw.set(next);
        set_command_api_query_change_runs.update(|count| *count += 1);
    });
    let (command_api_action_runs, set_command_api_action_runs) = signal(0_u32);
    let (command_api_last_action, set_command_api_last_action) = signal("none".to_string());
    let on_command_api_action = Callback::new(move |id: String| {
        set_command_api_last_action.set(id);
        set_command_api_action_runs.update(|count| *count += 1);
    });
    let command_api_motion = Signal::derive(move || {
        if command_api_custom_motion.get() {
            let mut motion = ui::CommandMotion::default();
            motion.spring.stiffness = 260.0;
            motion.spring.damping = 21.0;
            motion
        } else {
            ui::CommandMotion::default()
        }
    });
    let command_api_code = Signal::derive(move || {
        let lang = if command_api_rtl.get() { "ar" } else { "en" };
        let dir = if command_api_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if command_api_custom_class.get() {
            "docs-command-custom"
        } else {
            ""
        };
        let motion = if command_api_custom_motion.get() {
            "CommandMotion { spring: SpringConfig { stiffness: 260.0, damping: 21.0, ..spring_slide() }, ..CommandMotion::default() }"
        } else {
            "CommandMotion::default()"
        };
        format!(
            "<Command\n  id_base=\"docs-command-api-workbench\".to_string()\n  groups=groups.clone()\n  query=Signal::derive(move || query_raw.get())\n  default_query=\"cal\".to_string()\n  on_query_change=on_query_change\n  on_action=on_action\n  is_disabled={}\n  motion={motion}\n  placeholder=\"Search docs actions...\".to_string()\n  empty_label=\"No docs action found.\".to_string()\n  aria_label=\"Docs command center\".to_string()\n  lang=\"{lang}\".to_string()\n  dir={dir}\n  class_name={:?}\n/>",
            command_api_disabled.get(),
            class_name,
        )
    });
    let command_api_actual_config = Signal::derive(move || {
        let class_name = if command_api_custom_class.get() {
            Some("docs-command-custom")
        } else {
            Some("")
        };
        format!(
            "CommandApiWorkbenchConfig {{\n  id_base: \"docs-command-api-workbench\",\n  groups: \"sample_groups(len=2)\",\n  query: {:?},\n  default_query: Some(\"cal\"),\n  on_query_change: \"runs={}\",\n  on_action: \"runs={}, last={:?}\",\n  is_disabled: {},\n  motion: {},\n  placeholder: Some(\"Search docs actions...\"),\n  empty_label: Some(\"No docs action found.\"),\n  aria_label: Some(\"Docs command center\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {class_name:?},\n}}",
            command_api_query_raw.get(),
            command_api_query_change_runs.get(),
            command_api_action_runs.get(),
            command_api_last_action.get(),
            command_api_disabled.get(),
            if command_api_custom_motion.get() {
                "CommandMotion::custom"
            } else {
                "CommandMotion::default"
            },
            if command_api_rtl.get() { "ar" } else { "en" },
            if command_api_rtl.get() { "rtl" } else { "ltr" },
        )
    });

    let state_matrix_groups = groups.clone();
    let state_matrix_options = vec![
        "Idle (default)".to_string(),
        "Query with results".to_string(),
        "Query empty".to_string(),
        "Disabled".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_selected = Signal::derive(move || state_matrix_index.get().unwrap_or(0));
    let state_matrix_default_query = Signal::derive(move || match state_matrix_selected.get() {
        1 => "cal".to_string(),
        2 => "no-match".to_string(),
        _ => String::new(),
    });
    let state_matrix_disabled = Signal::derive(move || state_matrix_selected.get() == 3);

    let state_matrix_code = Signal::derive(move || {
        let mut lines = vec![
            "let groups = vec![CommandGroup::new(\"Suggestions\", vec![CommandItem::new(\"calendar\", \"Calendar\")])];".to_string(),
            String::new(),
            "<Command".to_string(),
            "  id_base=\"docs-command-state-matrix\".to_string()".to_string(),
            "  groups=groups.clone()".to_string(),
        ];

        let query = state_matrix_default_query.get();
        if !query.is_empty() {
            lines.push(format!("  default_query=\"{query}\".to_string()"));
        }
        if state_matrix_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });

    let compare_groups = groups.clone();
    let (controlled_query_raw, set_controlled_query_raw) = signal("cal".to_string());
    let controlled_query: Signal<String> = Signal::derive(move || controlled_query_raw.get());
    let on_controlled_query_change =
        Callback::new(move |next: String| set_controlled_query_raw.set(next));
    let (last_controlled_query_action, set_last_controlled_query_action) =
        signal("none".to_string());
    let on_controlled_query_action =
        Callback::new(move |id: String| set_last_controlled_query_action.set(id));
    let (last_uncontrolled_query_action, set_last_uncontrolled_query_action) =
        signal("none".to_string());
    let on_uncontrolled_query_action =
        Callback::new(move |id: String| set_last_uncontrolled_query_action.set(id));

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"let (query, set_query) = signal("cal".to_string());
let groups = vec![
  CommandGroup::new("Suggestions", vec![CommandItem::new("calendar", "Calendar")]),
  CommandGroup::new("Settings", vec![CommandItem::new("profile", "Profile")]),
];

<div class="docs-stack docs-stack--tight">
  <Command
    id_base="docs-command-controlled".to_string()
    groups=groups.clone()
    query=Signal::derive(move || query.get())
    on_query_change=Callback::new(move |next: String| set_query.set(next))
  />
  <Command
    id_base="docs-command-uncontrolled".to_string()
    groups=groups.clone()
    default_query="cal".to_string()
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
    let stream_groups = groups.clone();
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Streaming is optional; fallback stays snapshot.
let groups = vec![
  CommandGroup::new("Suggestions", vec![CommandItem::new("calendar", "Calendar")]),
];

<Command
  id_base="docs-command-stream".to_string()
  groups=groups.clone()
  default_query="cal".to_string()
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Command
  id_base="docs-command".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Command
  id_base="docs-command-custom".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  placeholder="Search pages, actions, and settings...".to_string()
  empty_label="No command matches your search.".to_string()
  class_name="docs-command-custom".to_string()
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let mut custom_motion = ui::CommandMotion::default();
custom_motion.spring.stiffness = 240.0;
custom_motion.spring.damping = 20.0;
let (last_action, set_last_action) = signal("none".to_string());

<Command
  id_base="docs-command-markers".to_string()
  groups=vec![
    CommandGroup::new("Suggestions", vec![
      CommandItem::new("calendar", "Calendar").shortcut("⌘K"),
      CommandItem::new("search-emoji", "Search Emoji").shortcut("⌘E"),
    ]),
    CommandGroup::new("Settings", vec![
      CommandItem::new("profile", "Profile"),
      CommandItem::new("billing", "Billing"),
    ]),
  ]
  on_action=Callback::new(move |id: String| set_last_action.set(id))
  placeholder="Search workspace actions...".to_string()
  empty_label="No workspace action found.".to_string()
  aria_label="Workspace command center".to_string()
  class_name="docs-command-custom".to_string()
  motion=custom_motion
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let mut marker_motion = ui::CommandMotion::default();
    marker_motion.spring.stiffness = 240.0;
    marker_motion.spring.damping = 20.0;

    let workbench_options = vec![
        "Default".to_string(),
        "Custom labels".to_string(),
        "Disabled + custom motion".to_string(),
    ];
    let (workbench_index, set_workbench_index) = signal(Some(0_usize));
    let workbench_disabled = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let workbench_custom_text = Signal::derive(move || workbench_index.get().unwrap_or(0) >= 1);
    let workbench_custom_motion = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);

    let (last_workbench_action, set_last_workbench_action) = signal("none".to_string());
    let on_workbench_action = Callback::new(move |id: String| set_last_workbench_action.set(id));
    let groups_for_workbench = groups.clone();

    let workbench_motion = Signal::derive(move || {
        let mut motion = ui::CommandMotion::default();
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 280.0;
            motion.spring.damping = 21.0;
        }
        motion
    });

    let workbench_code = Signal::derive(move || {
        let use_custom = workbench_custom_text.get();
        let use_motion = workbench_custom_motion.get();
        let is_disabled = workbench_disabled.get();

        let mut lines = vec![
            "let groups = vec![CommandGroup::new(\"Suggestions\", vec![CommandItem::new(\"calendar\", \"Calendar\")])];".to_string(),
            String::new(),
            "<Command".to_string(),
            "  id_base=\"docs-command-workbench\".into()".to_string(),
            "  groups=groups.clone()".to_string(),
            "  on_action=Callback::new(move |id: String| set_last_action.set(id))".to_string(),
        ];

        if use_custom {
            lines.push("  placeholder=\"Search docs actions...\".into()".to_string());
            lines.push("  empty_label=\"No docs action found.\".into()".to_string());
            lines.push("  aria_label=\"Docs command center\".into()".to_string());
            lines.push("  class_name=\"docs-command-custom\".into()".to_string());
        }

        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }

        if use_motion {
            lines.push("  motion=ui::CommandMotion {".to_string());
            lines.push("    spring: ui_motion::spring::SpringConfig {".to_string());
            lines.push("      stiffness: 280.0,".to_string());
            lines.push("      damping: 21.0,".to_string());
            lines.push("      ..ui_motion::presets::spring_slide()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/command/styles.rs */\n{}",
            ui::command::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let scenario = workbench_index.get().unwrap_or(0);
        format!(
            "CommandWorkbenchConfig {{\n  scenario: {scenario},\n  disabled: {},\n  custom_text: {},\n  custom_motion: {},\n  class_name: {},\n  on_query_change: \"n/a in scenario workbench (covered in API workbench)\",\n  on_action: \"last={:?}\",\n}}",
            workbench_disabled.get(),
            workbench_custom_text.get(),
            workbench_custom_motion.get(),
            if workbench_custom_text.get() {
                "\"docs-command-custom\""
            } else {
                "\"\""
            },
            last_workbench_action.get(),
        )
    });

    view! {
        <ComponentPage
            title="Command"
            slug="command"
            group="Collections"
            description="baseline-compatible command palette with grouped filtering, listbox keyboard semantics, baseline data contracts, and baseline-level spring active-highlight motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <Command id_base="docs-command-hello".to_string() groups=hello_groups />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=command_api_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
                test_config_signal=command_api_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="command-api-workbench-controls">
                        <Switch checked=command_api_disabled set_checked=set_command_api_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=command_api_rtl set_checked=set_command_api_rtl>
                            "lang/dir RTL"
                        </Switch>
                        <Switch checked=command_api_custom_motion set_checked=set_command_api_custom_motion>
                            "motion"
                        </Switch>
                        <Switch checked=command_api_custom_class set_checked=set_command_api_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Command
                        id_base="docs-command-api-workbench".to_string()
                        groups=command_api_groups.clone()
                        query=command_api_query
                        default_query="cal".to_string()
                        on_query_change=on_command_api_query_change
                        on_action=on_command_api_action
                        is_disabled=command_api_disabled.get()
                        motion=command_api_motion.get()
                        placeholder="Search docs actions...".to_string()
                        empty_label="No docs action found.".to_string()
                        aria_label="Docs command center".to_string()
                        lang=if command_api_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if command_api_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if command_api_custom_class.get() {
                            "docs-command-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "query: " {move || command_api_query_raw.get()}
                        " · on_query_change: " {move || command_api_query_change_runs.get()}
                        " · on_action: " {move || command_api_action_runs.get()}
                        " · last action: " {move || command_api_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Grouped Search + Keyboard Action"
                code_signal=code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-e2e-default">
                    <Command
                        id_base="docs-command-default".to_string()
                        groups=groups.clone()
                        on_action=on_action
                    />
                    <span
                        class="ui-muted"
                        data-slot="command-last-action"
                        data-scenario="default"
                        data-last-action=move || last_action.get()
                    >
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                description="Switch between idle/query-results/query-empty/disabled branches and verify state markers."
                code_signal=state_matrix_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-state-matrix">
                    <SegmentedControl
                        id_base="docs-command-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Command state matrix scenario".to_string()
                    />

                    <Command
                        id_base="docs-command-state-matrix".to_string()
                        groups=state_matrix_groups.clone()
                        default_query=state_matrix_default_query.get()
                        is_disabled=state_matrix_disabled.get()
                    />

                    <span class="ui-muted">
                        "state mode: "
                        {move || match state_matrix_selected.get() {
                            0 => "idle",
                            1 => "query-results",
                            2 => "query-empty",
                            _ => "disabled",
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side comparison of query+on_query_change control versus default_query-driven uncontrolled behavior."
                code_signal=controlled_uncontrolled_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-controlled-uncontrolled">
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Controlled"</strong>
                        <Command
                            id_base="docs-command-controlled".to_string()
                            groups=compare_groups.clone()
                            query=controlled_query
                            on_query_change=on_controlled_query_change
                            on_action=on_controlled_query_action
                        />
                        <span class="ui-muted">
                            "controlled query: "
                            {move || controlled_query_raw.get()}
                        </span>
                        <span class="ui-muted">
                            "last action: "
                            {move || last_controlled_query_action.get()}
                        </span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <strong>"Uncontrolled"</strong>
                        <Command
                            id_base="docs-command-uncontrolled".to_string()
                            groups=compare_groups.clone()
                            default_query="cal".to_string()
                            on_action=on_uncontrolled_query_action
                        />
                        <span class="ui-muted">
                            "default query: cal"
                        </span>
                        <span class="ui-muted">
                            "last action: "
                            {move || last_uncontrolled_query_action.get()}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Command is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="command-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-command-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Command requested stream mode".to_string()
                    />
                    <Command
                        id_base="docs-command-stream".to_string()
                        groups=stream_groups.clone()
                        default_query="cal".to_string()
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
                        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Custom Placeholder + Empty Label + Disabled Items"
                code_signal=states_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Command
                        id_base="docs-command-custom".to_string()
                        groups=custom_groups
                        on_action=on_custom_action
                        placeholder="Search pages, actions, and settings...".to_string()
                        empty_label="No command matches your search.".to_string()
                        class_name="docs-command-custom".to_string()
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_custom_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                code_signal=marker_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-e2e-markers">
                    <div class="ui-muted">
                        "Inspect data-id-source / data-placeholder-source / data-empty-label-source / data-aria-label-source / data-action-source / data-motion-source in DevTools."
                    </div>
                    <Command
                        id_base="docs-command-markers".to_string()
                        groups=marker_groups
                        on_action=on_marker_action
                        placeholder="Search workspace actions...".to_string()
                        empty_label="No workspace action found.".to_string()
                        aria_label="Workspace command center".to_string()
                        class_name="docs-command-custom".to_string()
                        motion=marker_motion
                    />
                    <span
                        class="ui-muted"
                        data-slot="command-last-action"
                        data-scenario="markers"
                        data-last-action=move || last_marker_action.get()
                    >
                        "last action: "
                        {move || last_marker_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test workbench for command state/source contract tuning."
                code_signal=workbench_code
                code_imports=COMMAND_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/command/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="command-workbench-controls">
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-command-workbench-scenario".to_string()
                            options=workbench_options.clone()
                            selected_index=workbench_index
                            set_selected_index=set_workbench_index
                            size=SegmentedControlSize::Sm
                            aria_label="Command scenario".to_string()
                        />

                        <div class="ui-muted">
                            "disabled: "
                            {move || workbench_disabled.get()}
                        </div>
                        <div class="ui-muted">
                            "custom labels: "
                            {move || workbench_custom_text.get()}
                        </div>
                        <div class="ui-muted">
                            "custom motion: "
                            {move || workbench_custom_motion.get()}
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-workbench">
                    <Command
                        id_base="docs-command-workbench".to_string()
                        groups=groups_for_workbench.clone()
                        on_action=on_workbench_action
                        is_disabled=workbench_disabled.get()
                        motion=workbench_motion.get()
                        placeholder=if workbench_custom_text.get() {
                            "Search docs actions...".to_string()
                        } else {
                            String::new()
                        }
                        empty_label=if workbench_custom_text.get() {
                            "No docs action found.".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_text.get() {
                            "Docs command center".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_text.get() {
                            "docs-command-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span
                        class="ui-muted"
                        data-slot="command-workbench-last-action"
                        data-last-action=move || last_workbench_action.get()
                    >
                        "last action: "
                        {move || last_workbench_action.get()}
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" data-slot="command-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground and the CodeBlock "
                    <code>"Copy"</code>
                    " action to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"COMMAND_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-command\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" data-slot="command-source-paths">
                    <li><code>"components/command/src/mod.rs"</code></li>
                    <li><code>"components/command/src/logic.rs"</code></li>
                    <li><code>"components/command/src/view.rs"</code></li>
                    <li><code>"components/command/src/styles.rs"</code></li>
                    <li><code>"components/command/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
