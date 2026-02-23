{
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
    let groups_for_state_matrix = groups.clone();
    let groups_for_controlled = groups.clone();
    let groups_for_compare = groups.clone();
    let groups_for_stream = groups.clone();

    let marker_groups: Arc<[CommandGroup]> = Arc::from(vec![
        CommandGroup::new(
            "Workspace",
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

    let hello_groups: Arc<[CommandGroup]> = Arc::from(vec![CommandGroup::new(
        "Quick Start",
        vec![CommandItem::new("open-dashboard", "Open Dashboard").shortcut("⌘D")],
    )]);
    let hello_world_code = Signal::derive(move || {
        r#"<CommandDialog
  groups=vec![CommandGroup::new("Quick Start", vec![
    CommandItem::new("open-dashboard", "Open Dashboard").shortcut("⌘D"),
  ])]
  default_open=true
/>"#
        .to_string()
    });

    let command_dialog_api_groups = groups.clone();
    let (command_dialog_api_open_raw, set_command_dialog_api_open_raw) = signal(false);
    let command_dialog_api_open: Signal<bool> =
        Signal::derive(move || command_dialog_api_open_raw.get());
    let (command_dialog_api_default_open, set_command_dialog_api_default_open) = signal(false);
    let (command_dialog_api_close_on_action, set_command_dialog_api_close_on_action) = signal(true);
    let (command_dialog_api_is_disabled, set_command_dialog_api_is_disabled) = signal(false);
    let (command_dialog_api_disabled, set_command_dialog_api_disabled) = signal(false);
    let (command_dialog_api_custom_text, set_command_dialog_api_custom_text) = signal(true);
    let (command_dialog_api_custom_class, set_command_dialog_api_custom_class) = signal(false);
    let (command_dialog_api_custom_motion, set_command_dialog_api_custom_motion) = signal(false);
    let (command_dialog_api_open_change_runs, set_command_dialog_api_open_change_runs) =
        signal(0_u32);
    let on_command_dialog_api_open_change = Callback::new(move |next: bool| {
        set_command_dialog_api_open_raw.set(next);
        set_command_dialog_api_open_change_runs.update(|count| *count += 1);
    });
    let (command_dialog_api_action_runs, set_command_dialog_api_action_runs) = signal(0_u32);
    let (command_dialog_api_last_action, set_command_dialog_api_last_action) =
        signal("none".to_string());
    let on_command_dialog_api_action = Callback::new(move |id: String| {
        set_command_dialog_api_last_action.set(id);
        set_command_dialog_api_action_runs.update(|count| *count += 1);
    });
    let command_dialog_api_command_motion = Signal::derive(move || {
        if command_dialog_api_custom_motion.get() {
            let mut motion = ui::CommandMotion::default();
            motion.spring.stiffness = 260.0;
            motion.spring.damping = 22.0;
            motion
        } else {
            ui::CommandMotion::default()
        }
    });
    let command_dialog_api_overlay_motion = Signal::derive(move || {
        if command_dialog_api_custom_motion.get() {
            ui::OverlayMotion {
                initial_scale: 0.96,
                initial_y_px: 8.0,
                ..ui::OverlayMotion::default()
            }
        } else {
            ui::OverlayMotion::default()
        }
    });
    let command_dialog_api_code = Signal::derive(move || {
        let description = if command_dialog_api_custom_text.get() {
            "Try command search with marker-rich contracts."
        } else {
            ""
        };
        let placeholder = if command_dialog_api_custom_text.get() {
            "Search docs commands..."
        } else {
            ""
        };
        let empty_label = if command_dialog_api_custom_text.get() {
            "No docs command found."
        } else {
            ""
        };
        let aria_label = if command_dialog_api_custom_text.get() {
            "Docs command dialog"
        } else {
            ""
        };
        let class_name = if command_dialog_api_custom_class.get() {
            "docs-command-dialog-custom"
        } else {
            ""
        };
        let command_motion = if command_dialog_api_custom_motion.get() {
            "CommandMotion { spring: SpringConfig { stiffness: 260.0, damping: 22.0, ..spring_slide() }, ..CommandMotion::default() }"
        } else {
            "CommandMotion::default()"
        };
        let overlay_motion = if command_dialog_api_custom_motion.get() {
            "OverlayMotion { initial_scale: 0.96, initial_y_px: 8.0, ..OverlayMotion::default() }"
        } else {
            "OverlayMotion::default()"
        };

        format!(
            "<CommandDialog\n  groups=groups.clone()\n  open=Signal::derive(move || open_raw.get())\n  default_open={}\n  on_open_change=on_open_change\n  on_action=on_action\n  close_on_action={}\n  id_base=\"docs-command-dialog-api-workbench\".to_string()\n  title=\"Docs Command Center\".to_string()\n  description={:?}\n  is_disabled={}\n  disabled={}\n  command_motion={command_motion}\n  overlay_motion={overlay_motion}\n  placeholder={:?}\n  empty_label={:?}\n  aria_label={:?}\n  class_name={:?}\n/>",
            command_dialog_api_default_open.get(),
            command_dialog_api_close_on_action.get(),
            description,
            command_dialog_api_is_disabled.get(),
            command_dialog_api_disabled.get(),
            placeholder,
            empty_label,
            aria_label,
            class_name,
        )
    });
    let command_dialog_api_actual_config = Signal::derive(move || {
        let description = if command_dialog_api_custom_text.get() {
            Some("Try command search with marker-rich contracts.")
        } else {
            Some("")
        };
        let placeholder = if command_dialog_api_custom_text.get() {
            Some("Search docs commands...")
        } else {
            Some("")
        };
        let empty_label = if command_dialog_api_custom_text.get() {
            Some("No docs command found.")
        } else {
            Some("")
        };
        let aria_label = if command_dialog_api_custom_text.get() {
            Some("Docs command dialog")
        } else {
            Some("")
        };
        let class_name = if command_dialog_api_custom_class.get() {
            Some("docs-command-dialog-custom")
        } else {
            Some("")
        };

        format!(
            "CommandDialogApiWorkbenchConfig {{\n  groups: \"sample_groups(len=2)\",\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: \"runs={}\",\n  on_action: \"runs={}, last={:?}\",\n  close_on_action: {},\n  id_base: Some(\"docs-command-dialog-api-workbench\"),\n  title: Some(\"Docs Command Center\"),\n  description: {description:?},\n  is_disabled: Some({}),\n  disabled: {},\n  command_motion: {},\n  overlay_motion: {},\n  placeholder: {placeholder:?},\n  empty_label: {empty_label:?},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n}}",
            command_dialog_api_open_raw.get(),
            command_dialog_api_default_open.get(),
            command_dialog_api_open_change_runs.get(),
            command_dialog_api_action_runs.get(),
            command_dialog_api_last_action.get(),
            command_dialog_api_close_on_action.get(),
            command_dialog_api_is_disabled.get(),
            command_dialog_api_disabled.get(),
            if command_dialog_api_custom_motion.get() {
                "CommandMotion::custom"
            } else {
                "CommandMotion::default"
            },
            if command_dialog_api_custom_motion.get() {
                "OverlayMotion::custom"
            } else {
                "OverlayMotion::default"
            },
        )
    });

    let state_matrix_options = vec![
        "Uncontrolled + close_on_action=true".to_string(),
        "Uncontrolled + close_on_action=false".to_string(),
        "Controlled + disabled".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_close_on_action =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) != 1);
    let state_matrix_disabled = Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_default_open = Signal::derive(move || {
        !state_matrix_is_controlled.get() && state_matrix_index.get().unwrap_or(0) != 2
    });
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));

    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(0);
        let mut lines = vec![
            "let groups = vec![CommandGroup::new(\"Suggestions\", vec![CommandItem::new(\"calendar\", \"Calendar\")])];".to_string(),
            String::new(),
            "<CommandDialog".to_string(),
            "  id_base=\"docs-command-dialog-state-matrix\".into()".to_string(),
            "  title=\"State Matrix\".into()".to_string(),
            "  groups=groups.clone()".to_string(),
        ];
        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  close_on_action=true".to_string());
            }
            1 => {
                lines.push("  default_open=true".to_string());
                lines.push("  close_on_action=false".to_string());
            }
            _ => {
                lines.push("  open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  is_disabled=true".to_string());
            }
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let (compare_controlled_last_action, set_compare_controlled_last_action) =
        signal("none".to_string());
    let on_compare_controlled_action =
        Callback::new(move |id: String| set_compare_controlled_last_action.set(id));
    let (compare_uncontrolled_last_action, set_compare_uncontrolled_last_action) =
        signal("none".to_string());
    let on_compare_uncontrolled_action =
        Callback::new(move |id: String| set_compare_uncontrolled_last_action.set(id));
    let compare_code = Signal::derive(move || {
        r#"let groups = vec![
  CommandGroup::new("Suggestions", vec![CommandItem::new("calendar", "Calendar")]),
];
let (open_raw, set_open_raw) = signal(false);

<CommandDialog
  id_base="docs-command-dialog-compare-controlled".into()
  groups=groups.clone()
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
/>

<CommandDialog
  id_base="docs-command-dialog-compare-uncontrolled".into()
  groups=groups.clone()
  default_open=true
/>"#
        .to_string()
    });

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let stream_requested_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot"
        } else {
            "streaming"
        }
    });
    let stream_requested_output_status = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "verified"
        } else {
            "draft"
        }
    });
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// CommandDialog is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
let groups = vec![
  CommandGroup::new("Suggestions", vec![CommandItem::new("calendar", "Calendar")]),
];

<CommandDialog
  id_base="docs-command-dialog-stream".into()
  groups=groups.clone()
  default_open=true
  close_on_action=false
/>"#
        .to_string()
    });

    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (last_marker_action, set_last_marker_action) = signal("none".to_string());
    let on_marker_action = Callback::new(move |id: String| set_last_marker_action.set(id));

    let code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let (last_action, set_last_action) = signal("none".to_string());

<CommandDialog
  groups=vec![
    CommandGroup::new("Navigation", vec![
      CommandItem::new("go-dashboard", "Go to Dashboard"),
      CommandItem::new("open-projects", "Open Projects"),
    ]),
    CommandGroup::new("Actions", vec![
      CommandItem::new("run-tests", "Run Tests"),
      CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
    ]),
  ]
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"<CommandDialog
  groups=vec![
    CommandGroup::new("Navigation", vec![
      CommandItem::new("go-dashboard", "Go to Dashboard"),
      CommandItem::new("open-projects", "Open Projects"),
    ]),
    CommandGroup::new("Actions", vec![
      CommandItem::new("run-tests", "Run Tests"),
      CommandItem::new("deploy-preview", "Deploy Preview").disabled(true),
    ]),
  ]
  id_base="docs-command-dialog-marker".to_string()
  title="Workspace Commands".to_string()
  description="Inspect source-state markers".to_string()
  default_open=true
  close_on_action=false
  placeholder="Search pages, actions, and settings...".to_string()
  empty_label="No command matches your search.".to_string()
  aria_label="Workspace command dialog".to_string()
  class_name="docs-command-dialog-custom".to_string()
  overlay_motion=ui::OverlayMotion {
    initial_scale: 0.95,
    initial_y_px: 10.0,
    ..ui::OverlayMotion::default()
  }
/>"#
        .to_string()
    });

    let marker_overlay_motion = ui::OverlayMotion {
        initial_scale: 0.95,
        initial_y_px: 10.0,
        ..ui::OverlayMotion::default()
    };

    let workbench_options = vec![
        "Default".to_string(),
        "Persistent keep-open".to_string(),
        "Disabled + custom labels/motion".to_string(),
    ];
    let (workbench_index, set_workbench_index) = signal(Some(0_usize));
    let workbench_close_on_action = Signal::derive(move || workbench_index.get().unwrap_or(0) == 0);
    let workbench_disabled = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let workbench_custom_text = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let workbench_custom_motion = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);
    let (workbench_preserve_context, set_workbench_preserve_context) = signal(true);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (last_workbench_action, set_last_workbench_action) = signal("none".to_string());
    let on_workbench_action = Callback::new(move |id: String| set_last_workbench_action.set(id));
    let workbench_groups = groups.clone();

    let workbench_command_motion = Signal::derive(move || {
        let mut motion = ui::CommandMotion::default();
        if workbench_custom_motion.get() {
            motion.spring.stiffness = 260.0;
            motion.spring.damping = 22.0;
        }
        motion
    });

    let workbench_overlay_motion = Signal::derive(move || {
        let mut motion = ui::OverlayMotion::default();
        if workbench_custom_motion.get() {
            motion.initial_scale = 0.96;
            motion.initial_y_px = 8.0;
        }
        motion
    });

    let reset_workbench_open = set_workbench_open_raw;
    let reset_workbench_action = set_last_workbench_action;
    Effect::new(move |_| {
        workbench_index.with(|_| ());
        if !workbench_preserve_context.get() {
            reset_workbench_open.set(false);
            reset_workbench_action.set("none".to_string());
        }
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let (last_action, set_last_action) = signal(\"none\".to_string());".to_string(),
            "let groups = vec![CommandGroup::new(\"Suggestions\", vec![CommandItem::new(\"calendar\", \"Calendar\")])];".to_string(),
            "<CommandDialog".to_string(),
            "  id_base=\"docs-command-dialog-workbench\".into()".to_string(),
            "  title=\"Docs Command Center\".into()".to_string(),
            "  groups=groups.clone()".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))".to_string(),
            "  on_action=Callback::new(move |id: String| set_last_action.set(id))".to_string(),
        ];

        if !workbench_close_on_action.get() {
            lines.push("  close_on_action=false".to_string());
        }
        if workbench_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if workbench_custom_text.get() {
            lines.push(
                "  description=\"Try command search with marker-rich contracts.\".into()"
                    .to_string(),
            );
            lines.push("  placeholder=\"Search docs commands...\".into()".to_string());
            lines.push("  empty_label=\"No docs command found.\".into()".to_string());
            lines.push("  aria_label=\"Docs command dialog\".into()".to_string());
            lines.push("  class_name=\"docs-command-dialog-custom\".into()".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push("  command_motion=ui::CommandMotion {".to_string());
            lines.push("    spring: ui_motion::spring::SpringConfig {".to_string());
            lines.push("      stiffness: 260.0,".to_string());
            lines.push("      damping: 22.0,".to_string());
            lines.push("      ..ui_motion::presets::spring_slide()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
            lines.push("  overlay_motion=ui::OverlayMotion {".to_string());
            lines.push("    initial_scale: 0.96,".to_string());
            lines.push("    initial_y_px: 8.0,".to_string());
            lines.push("    ..ui::OverlayMotion::default()".to_string());
            lines.push("  }".to_string());
        }
        lines.push("/>".to_string());
        lines.push("<span class=\"ui-muted\">\"open: \" {move || if open_raw.get() { \"true\" } else { \"false\" }}</span>".to_string());
        lines.push(
            "<span class=\"ui-muted\">\"last action: \" {move || last_action.get()}</span>"
                .to_string(),
        );

        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/command-dialog/src/styles.rs */\n{}",
            ui::command_dialog::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let scenario = workbench_index.get().unwrap_or(0);
        format!(
            "CommandDialogWorkbenchConfig {{\n  scenario: {scenario},\n  close_on_action: {},\n  disabled: {},\n  custom_text: {},\n  custom_motion: {},\n  preserve_context: {},\n  open: {},\n  on_open_change: \"set_workbench_open_raw(open={})\",\n  on_action: \"last={:?}\",\n  last_action: {:?},\n}}",
            workbench_close_on_action.get(),
            workbench_disabled.get(),
            workbench_custom_text.get(),
            workbench_custom_motion.get(),
            workbench_preserve_context.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            last_workbench_action.get(),
            last_workbench_action.get(),
        )
    });

    view! {
        <ComponentPage
            title="CommandDialog"
            slug="command-dialog"
            group="Collections"
            description="baseline-compatible command dialog that composes Modal + Command, supports controlled/uncontrolled open state, emits baseline data contracts, and reuses baseline-level overlay/active-highlight spring motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <CommandDialog
                        id_base="docs-command-dialog-hello".to_string()
                        title="Quick Start".to_string()
                        description="Minimal starter that opens by default.".to_string()
                        groups=hello_groups
                        default_open=true
                    />
                    <p class="ui-muted">
                        "Hello World path: drop in one group and rely on default snapshot rendering."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=command_dialog_api_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
                test_config_signal=command_dialog_api_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="command-dialog-api-workbench-controls">
                        <Switch checked=command_dialog_api_default_open set_checked=set_command_dialog_api_default_open>
                            "default_open"
                        </Switch>
                        <Switch checked=command_dialog_api_close_on_action set_checked=set_command_dialog_api_close_on_action>
                            "close_on_action"
                        </Switch>
                        <Switch checked=command_dialog_api_is_disabled set_checked=set_command_dialog_api_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=command_dialog_api_disabled set_checked=set_command_dialog_api_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=command_dialog_api_custom_text set_checked=set_command_dialog_api_custom_text>
                            "description/placeholder/empty_label/aria_label"
                        </Switch>
                        <Switch checked=command_dialog_api_custom_class set_checked=set_command_dialog_api_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=command_dialog_api_custom_motion set_checked=set_command_dialog_api_custom_motion>
                            "command_motion/overlay_motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_command_dialog_api_open_raw.set(true)>
                            "Open"
                        </button>
                        <button type="button" on:click=move |_| set_command_dialog_api_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <CommandDialog
                        groups=command_dialog_api_groups.clone()
                        open=command_dialog_api_open
                        default_open=command_dialog_api_default_open.get()
                        on_open_change=on_command_dialog_api_open_change
                        on_action=on_command_dialog_api_action
                        close_on_action=command_dialog_api_close_on_action.get()
                        id_base="docs-command-dialog-api-workbench".to_string()
                        title="Docs Command Center".to_string()
                        description=if command_dialog_api_custom_text.get() {
                            "Try command search with marker-rich contracts.".to_string()
                        } else {
                            String::new()
                        }
                        is_disabled=command_dialog_api_is_disabled.get()
                        disabled=command_dialog_api_disabled.get()
                        command_motion=command_dialog_api_command_motion.get()
                        overlay_motion=command_dialog_api_overlay_motion.get()
                        placeholder=if command_dialog_api_custom_text.get() {
                            "Search docs commands...".to_string()
                        } else {
                            String::new()
                        }
                        empty_label=if command_dialog_api_custom_text.get() {
                            "No docs command found.".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if command_dialog_api_custom_text.get() {
                            "Docs command dialog".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if command_dialog_api_custom_class.get() {
                            "docs-command-dialog-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "open: " {move || command_dialog_api_open_raw.get()}
                        " · on_open_change: " {move || command_dialog_api_open_change_runs.get()}
                        " · on_action: " {move || command_dialog_api_action_runs.get()}
                        " · last action: " {move || command_dialog_api_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                description="State matrix over open-mode/close-on-action/disabled. Use Scenario to switch one canonical branch at a time."
                code_signal=state_matrix_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-dialog-state-matrix">
                    <SegmentedControl
                        id_base="docs-command-dialog-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Command dialog state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <button
                            type="button"
                            on:click=move |_| set_state_matrix_open_raw.set(true)
                            disabled=move || !state_matrix_is_controlled.get()
                        >
                            "Open controlled scenario"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_state_matrix_open_raw.set(false)
                            disabled=move || !state_matrix_is_controlled.get()
                        >
                            "Close controlled scenario"
                        </button>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <CommandDialog
                                    id_base="docs-command-dialog-state-matrix".to_string()
                                    title="State Matrix".to_string()
                                    description="Switch scenario to inspect data-open-mode/data-close-on-action/data-disabled markers.".to_string()
                                    groups=groups_for_state_matrix.clone()
                                    open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    close_on_action=state_matrix_close_on_action.get()
                                    is_disabled=state_matrix_disabled.get()
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <CommandDialog
                                    id_base="docs-command-dialog-state-matrix".to_string()
                                    title="State Matrix".to_string()
                                    description="Switch scenario to inspect data-open-mode/data-close-on-action/data-disabled markers.".to_string()
                                    groups=groups_for_state_matrix.clone()
                                    default_open=state_matrix_default_open.get()
                                    close_on_action=state_matrix_close_on_action.get()
                                    is_disabled=state_matrix_disabled.get()
                                />
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "open_mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "close_on_action: "
                        {move || state_matrix_close_on_action.get()}
                    </span>
                    <span class="ui-muted">
                        "is_disabled: "
                        {move || state_matrix_disabled.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled Open + Action Close"
                code_signal=code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_open_raw.set(true)>
                            "Open CommandDialog"
                        </button>
                        <button type="button" on:click=move |_| set_open_raw.set(false)>
                            "Close"
                        </button>
                    </div>
                    <CommandDialog
                        id_base="docs-command-dialog-controlled".to_string()
                        title="Quick Actions".to_string()
                        description="Press ⌘K-style filtering and Enter to run actions.".to_string()
                        groups=groups_for_controlled.clone()
                        open=open
                        on_open_change=on_open_change
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || if open_raw.get() { "true" } else { "false" }}
                    </span>
                    <span
                        class="ui-muted"
                        data-slot="command-dialog-last-action"
                        data-open-mode="controlled"
                        data-last-action=move || last_action.get()
                    >
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                code_signal=marker_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="ui-muted">
                        "Inspect data-id-source / data-title-source / data-description-source / data-placeholder-source / data-action-source / data-overlay-motion-source in DevTools."
                    </div>
                    <CommandDialog
                        id_base="docs-command-dialog-marker".to_string()
                        title="Workspace Commands".to_string()
                        description="close_on_action=false keeps the dialog open after choosing an action.".to_string()
                        groups=marker_groups
                        default_open=true
                        close_on_action=false
                        on_action=on_marker_action
                        placeholder="Search pages, actions, and settings...".to_string()
                        empty_label="No command matches your search.".to_string()
                        aria_label="Workspace command dialog".to_string()
                        class_name="docs-command-dialog-custom".to_string()
                        overlay_motion=marker_overlay_motion
                    />
                    <span
                        class="ui-muted"
                        data-slot="command-dialog-last-action"
                        data-open-mode="uncontrolled"
                        data-last-action=move || last_marker_action.get()
                    >
                        "last action: "
                        {move || last_marker_action.get()}
                    </span>
                    <span class="ui-muted">"close_on_action: false (dialog stays open)"</span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side contrast of value+on_change control versus default-driven uncontrolled state."
                code_signal=compare_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-dialog-controlled-uncontrolled">
                    <div class="docs-row">
                        <button
                            type="button"
                            on:click=move |_| set_compare_controlled_open_raw.set(true)
                        >
                            "Open controlled dialog"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_compare_controlled_open_raw.set(false)
                        >
                            "Close controlled dialog"
                        </button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <CommandDialog
                                id_base="docs-command-dialog-compare-controlled".to_string()
                                title="Controlled Dialog".to_string()
                                description="open + on_open_change are driven by parent signals.".to_string()
                                groups=groups_for_compare.clone()
                                open=compare_controlled_open
                                on_open_change=on_compare_controlled_open_change
                                on_action=on_compare_controlled_action
                            />
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                            <span class="ui-muted">
                                "last action: "
                                {move || compare_controlled_last_action.get()}
                            </span>
                        </div>

                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <CommandDialog
                                id_base="docs-command-dialog-compare-uncontrolled".to_string()
                                title="Uncontrolled Dialog".to_string()
                                description="default_open initializes once; primitive owns later transitions.".to_string()
                                groups=groups_for_compare.clone()
                                default_open=true
                                close_on_action=false
                                on_action=on_compare_uncontrolled_action
                            />
                            <span class="ui-muted">
                                "last action: "
                                {move || compare_uncontrolled_last_action.get()}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="CommandDialog is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="command-dialog-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-command-dialog-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Command dialog stream mode".to_string()
                    />
                    <CommandDialog
                        id_base="docs-command-dialog-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output stays snapshot while exposing stream markers for agent consumers.".to_string()
                        groups=groups_for_stream
                        default_open=true
                        close_on_action=false
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
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Tune close-on-action/disabled/motion while optionally preserving open+action context in an isolated command-dialog canvas."
                code_signal=workbench_code
                code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/command-dialog/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div
                        class="docs-stack docs-stack--tight"
                        data-slot="command-dialog-workbench-controls"
                    >
                        <div class="docs-search__label">"Scenario"</div>
                        <SegmentedControl
                            id_base="docs-command-dialog-workbench-scenario".to_string()
                            options=workbench_options.clone()
                            selected_index=workbench_index
                            set_selected_index=set_workbench_index
                            size=SegmentedControlSize::Sm
                            aria_label="Command dialog scenario".to_string()
                        />
                        <Switch
                            checked=workbench_preserve_context
                            set_checked=set_workbench_preserve_context
                        >
                            " Preserve open/action context (optional)"
                        </Switch>
                        <div class="ui-muted">
                            "close_on_action: "
                            {move || workbench_close_on_action.get()}
                        </div>
                        <div class="ui-muted">
                            "is_disabled: "
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
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="command-dialog-workbench">
                    <div class="docs-row" data-slot="command-dialog-workbench-actions">
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(true)>
                            "Open Workbench Dialog"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(false)>
                            "Close"
                        </button>
                        <button type="button" on:click=move |_| set_last_workbench_action.set("none".to_string())>
                            "Clear Last Action"
                        </button>
                    </div>
                    <div data-slot="command-dialog-workbench-canvas">
                        <CommandDialog
                            id_base="docs-command-dialog-workbench".to_string()
                            title="Docs Command Center".to_string()
                            description=if workbench_custom_text.get() {
                                "Try command search with marker-rich contracts.".to_string()
                            } else {
                                String::new()
                            }
                            groups=workbench_groups.clone()
                            open=workbench_open
                            on_open_change=on_workbench_open_change
                            on_action=on_workbench_action
                            close_on_action=workbench_close_on_action.get()
                            is_disabled=workbench_disabled.get()
                            placeholder=if workbench_custom_text.get() {
                                "Search docs commands...".to_string()
                            } else {
                                String::new()
                            }
                            empty_label=if workbench_custom_text.get() {
                                "No docs command found.".to_string()
                            } else {
                                String::new()
                            }
                            aria_label=if workbench_custom_text.get() {
                                "Docs command dialog".to_string()
                            } else {
                                String::new()
                            }
                            class_name=if workbench_custom_text.get() {
                                "docs-command-dialog-custom".to_string()
                            } else {
                                String::new()
                            }
                            command_motion=workbench_command_motion.get()
                            overlay_motion=workbench_overlay_motion.get()
                        />
                    </div>
                    <span class="ui-muted">
                        "open: "
                        {move || if workbench_open_raw.get() { "true" } else { "false" }}
                    </span>
                    <span class="ui-muted">
                        "last action: "
                        {move || last_workbench_action.get()}
                    </span>
                    <span class="ui-muted">
                        "persist_context: "
                        {move || workbench_preserve_context.get()}
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" data-slot="command-dialog-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"COMMAND_DIALOG_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-command_dialog\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" data-slot="command-dialog-source-paths">
                    <li><code>"components/command-dialog/src/mod.rs"</code></li>
                    <li><code>"components/command-dialog/src/logic.rs"</code></li>
                    <li><code>"components/command-dialog/src/view.rs"</code></li>
                    <li><code>"components/command-dialog/src/styles.rs"</code></li>
                    <li><code>"components/command-dialog/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
