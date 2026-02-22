use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::sync::Arc;
use ui::{
    Carousel, CarouselItem, CarouselOrientation, Command, CommandDialog, CommandGroup, CommandItem,
    ContextMenu, MenuItemKind, Menubar, MenubarMenu, NavigationMenu, NavigationMenuItem,
    SegmentedControl, SegmentedControlSize, Switch,
};
use ui_headless::A11yDirection;

const COMMAND_DIALOG_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{CommandDialog, CommandGroup, CommandItem};";
const COMMAND_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse std::sync::Arc;\nuse ui::{Command, CommandGroup, CommandItem};";
const CAROUSEL_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{Carousel, CarouselItem, CarouselOrientation};";

pub(super) fn command() -> AnyView {
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

pub(super) fn context_menu() -> AnyView {
    let workbench_items = vec![
        "Open".to_string(),
        "Rename".to_string(),
        "Delete".to_string(),
    ];
    let workbench_item_kinds = vec![
        MenuItemKind::Action,
        MenuItemKind::Action,
        MenuItemKind::Action,
    ];
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let (last_action, set_last_action) = signal("None".to_string());
    let on_workbench_action =
        Callback::new(move |index: usize| set_last_action.set(index.to_string()));
    let (open_change_count, set_open_change_count) = signal(0_u32);
    let on_workbench_open_change_with_count = Callback::new(move |next: bool| {
        set_open_change_count.update(|count| *count += 1);
        on_workbench_open_change.run(next);
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_force_close_on_action, set_workbench_force_close_on_action) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_disable_middle, set_workbench_disable_middle) = signal(false);
    let (workbench_placement_key, set_workbench_placement_key) = signal("bottom-start".to_string());

    let hello_code = Signal::derive(move || {
        r#"<ContextMenu
  id_base="docs-context-menu".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Delete".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Right click or press Shift+F10"
</ContextMenu>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let placement = if workbench_placement_key.get() == "top-start" {
            "ui_headless::PopoverPlacement::TopStart"
        } else {
            "ui_headless::PopoverPlacement::BottomStart"
        };
        format!(
            "<ContextMenu\n  id_base=\"docs-context-menu-workbench\".to_string()\n  items=vec![\"Open\".to_string(), \"Rename\".to_string(), \"Delete\".to_string()]\n  on_action=on_action\n  is_disabled={}\n  disabled={}\n  disabled_indices={}\n  item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action, MenuItemKind::Action]\n  is_close_on_action={}\n  close_on_action={}\n  placement={placement}\n  is_open=Signal::derive(move || open_raw.get())\n  open=Signal::derive(move || open_raw.get())\n  default_open={}\n  on_open_change=on_open_change\n  motion={}\n  lang={}\n  dir={}\n  aria_label=\"Workspace actions\".to_string()\n  class_name={}\n>\n  \"Right click to inspect\"\n</ContextMenu>",
            workbench_disabled.get(),
            workbench_disabled.get(),
            if workbench_disable_middle.get() {
                "vec![1]"
            } else {
                "vec![]"
            },
            workbench_force_close_on_action.get(),
            workbench_force_close_on_action.get(),
            workbench_open_raw.get(),
            if workbench_custom_motion.get() {
                "ui::ContextMenuMotion { popover: ui::PopoverMotion { initial_scale: 0.92, offset_y_px: 8.0, ..ui::PopoverMotion::default() } }"
            } else {
                "ui::ContextMenuMotion::default()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "ui_headless::A11yDirection::Rtl"
            } else {
                "ui_headless::A11yDirection::Ltr"
            },
            if workbench_custom_class.get() {
                "\"docs-context-menu-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_placement_key.get() == "top-start" {
            "TopStart"
        } else {
            "BottomStart"
        };
        format!(
            "ContextMenuWorkbenchConfig {{\n  id_base: \"docs-context-menu-workbench\",\n  items: [\"Open\", \"Rename\", \"Delete\"],\n  on_action: Some(\"Callback<usize>\"),\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {},\n  item_kinds: [Action, Action, Action],\n  is_close_on_action: Some({}),\n  close_on_action: {},\n  placement: {placement},\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some({}),\n  on_open_change: Some(\"Callback<bool>\"),\n  motion: {},\n  lang: {},\n  dir: {},\n  aria_label: Some(\"Workspace actions\"),\n  class_name: {},\n}}",
            workbench_disabled.get(),
            workbench_disabled.get(),
            if workbench_disable_middle.get() {
                "[1]"
            } else {
                "[]"
            },
            workbench_force_close_on_action.get(),
            workbench_force_close_on_action.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            if workbench_custom_motion.get() {
                "ContextMenuMotion::custom"
            } else {
                "ContextMenuMotion::default"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-context-menu-workbench\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ContextMenu id_base="ctx-default".to_string() items=vec!["Open".to_string(), "Rename".to_string(), "Delete".to_string()] on_action=Callback::new(move |_| {})>
  "Default"
</ContextMenu>
<ContextMenu id_base="ctx-keep-open".to_string() items=vec!["Copy".to_string(), "Paste".to_string(), "Inspect".to_string()] on_action=Callback::new(move |_| {}) close_on_action=false disabled_indices=vec![1]>
  "Keep open + disabled item"
</ContextMenu>
<ContextMenu id_base="ctx-disabled".to_string() items=vec!["Open".to_string()] on_action=Callback::new(move |_| {}) disabled=true>
  "Disabled trigger"
</ContextMenu>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ContextMenu"
            slug="context-menu"
            group="Collections"
            description="Context trigger menu with controlled open state and action callbacks."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ContextMenu
                    id_base="docs-context-menu-hello".to_string()
                    items=vec![
                        "Open".to_string(),
                        "Rename".to_string(),
                        "Delete".to_string(),
                    ]
                    on_action=Callback::new(|_: usize| {})
                >
                    "Right click or press Shift+F10"
                </ContextMenu>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full ContextMenu API and shows open/action callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="context-menu-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Placement"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_placement_key.set(event_target_value(&ev))>
                                <option value="bottom-start" selected=move || workbench_placement_key.get() == "bottom-start">"BottomStart"</option>
                                <option value="top-start" selected=move || workbench_placement_key.get() == "top-start">"TopStart"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_disable_middle set_checked=set_workbench_disable_middle>"Disabled middle item"</Switch>
                        <Switch checked=workbench_force_close_on_action set_checked=set_workbench_force_close_on_action>"Close on action"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="context-menu-workbench-preview">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(true)>"Open"</button>
                        <button type="button" on:click=move |_| set_workbench_open_raw.set(false)>"Close"</button>
                    </div>
                    <ContextMenu
                        id_base="docs-context-menu-workbench".to_string()
                        items=workbench_items
                        on_action=on_workbench_action
                        is_disabled=workbench_disabled.get()
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_middle.get() {
                            vec![1]
                        } else {
                            vec![]
                        }
                        item_kinds=workbench_item_kinds
                        is_close_on_action=workbench_force_close_on_action.get()
                        close_on_action=workbench_force_close_on_action.get()
                        placement=if workbench_placement_key.get() == "top-start" {
                            ui_headless::PopoverPlacement::TopStart
                        } else {
                            ui_headless::PopoverPlacement::BottomStart
                        }
                        is_open=workbench_open
                        open=workbench_open
                        default_open=workbench_open_raw.get()
                        on_open_change=on_workbench_open_change_with_count
                        motion=if workbench_custom_motion.get() {
                            ui::ContextMenuMotion {
                                popover: ui::PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 8.0,
                                    ..ui::PopoverMotion::default()
                                },
                            }
                        } else {
                            ui::ContextMenuMotion::default()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        aria_label="Workspace actions".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-context-menu-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        "Right click or press Shift+F10"
                    </ContextMenu>
                    <span class="ui-muted">
                        "open=" {move || workbench_open_raw.get()}
                        " · open_change_count=" {move || open_change_count.get()}
                        " · last_action=" {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <ContextMenu
                        id_base="docs-context-menu-matrix-default".to_string()
                        items=vec![
                            "Open".to_string(),
                            "Rename".to_string(),
                            "Delete".to_string(),
                        ]
                        on_action=Callback::new(|_: usize| {})
                    >
                        "Default"
                    </ContextMenu>
                    <ContextMenu
                        id_base="docs-context-menu-matrix-keep-open".to_string()
                        items=vec![
                            "Copy".to_string(),
                            "Paste".to_string(),
                            "Inspect".to_string(),
                        ]
                        on_action=Callback::new(|_: usize| {})
                        close_on_action=false
                        disabled_indices=vec![1]
                    >
                        "Keep open + disabled"
                    </ContextMenu>
                    <ContextMenu
                        id_base="docs-context-menu-matrix-disabled".to_string()
                        items=vec!["Open".to_string()]
                        on_action=Callback::new(|_: usize| {})
                        disabled=true
                    >
                        "Disabled trigger"
                    </ContextMenu>
                </div>
            </Playground>

        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menubar() -> AnyView {
    let default_menus = vec![
        MenubarMenu::new(
            "file",
            "File",
            vec![
                "New Tab".to_string(),
                "New Window".to_string(),
                "Save".to_string(),
            ],
        )
        .disabled_indices(vec![2])
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "edit",
            "Edit",
            vec!["Undo".to_string(), "Redo".to_string(), "Find".to_string()],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "view",
            "View",
            vec![
                "Zoom In".to_string(),
                "Zoom Out".to_string(),
                "Actual Size".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
    ];

    let controlled_menus = vec![
        MenubarMenu::new(
            "window",
            "Window",
            vec![
                "Minimize".to_string(),
                "Zoom".to_string(),
                "Bring All to Front".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "help",
            "Help",
            vec!["Search".to_string(), "Documentation".to_string()],
        )
        .item_kinds(vec![MenuItemKind::Action, MenuItemKind::Action]),
        MenubarMenu::new("tools", "Tools", vec!["Inspector".to_string()]).disabled(true),
    ];

    let marker_menus = vec![
        MenubarMenu::new(
            "workspace",
            "Workspace",
            vec![
                "Open File".to_string(),
                "Open Folder".to_string(),
                "Save All".to_string(),
            ],
        )
        .disabled_indices(vec![2])
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new(
            "run",
            "Run",
            vec![
                "Run".to_string(),
                "Debug".to_string(),
                "Profile".to_string(),
            ],
        )
        .item_kinds(vec![
            MenuItemKind::Action,
            MenuItemKind::Action,
            MenuItemKind::Action,
        ]),
        MenubarMenu::new("help", "Help", vec!["Docs".to_string()]),
    ];

    let (last_action, set_last_action) = signal(None::<(usize, usize)>);
    let on_action = Callback::new(move |action: (usize, usize)| set_last_action.set(Some(action)));

    let (last_controlled_action, set_last_controlled_action) = signal(None::<(usize, usize)>);
    let on_controlled_action =
        Callback::new(move |action: (usize, usize)| set_last_controlled_action.set(Some(action)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(None::<usize>);
    let controlled_open: Signal<Option<usize>> = Signal::derive(move || controlled_open_raw.get());
    let on_open_index_change = Callback::new(move |next: Option<usize>| {
        set_controlled_open_raw.set(next);
    });

    let (marker_open_raw, set_marker_open_raw) = signal(Some(0usize));
    let marker_open: Signal<Option<usize>> = Signal::derive(move || marker_open_raw.get());
    let on_marker_open_change =
        Callback::new(move |next: Option<usize>| set_marker_open_raw.set(next));

    let (last_marker_action, set_last_marker_action) = signal(None::<(usize, usize)>);
    let on_marker_action =
        Callback::new(move |action: (usize, usize)| set_last_marker_action.set(Some(action)));

    let menu_set_options = vec![
        "app".to_string(),
        "workspace".to_string(),
        "compact".to_string(),
    ];
    let (menu_set_index, set_menu_set_index) = signal(Some(0_usize));
    let (workbench_close_on_action, set_workbench_close_on_action) = signal(true);
    let (workbench_flip_placement, set_workbench_flip_placement) = signal(false);
    let (workbench_default_open, set_workbench_default_open) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_menus: Signal<Vec<MenubarMenu>> =
        Signal::derive(move || match menu_set_index.get().unwrap_or(0) {
            1 => vec![
                MenubarMenu::new(
                    "workspace",
                    "Workspace",
                    vec![
                        "Open File".to_string(),
                        "Open Folder".to_string(),
                        "Save All".to_string(),
                    ],
                )
                .disabled_indices(vec![2])
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "run",
                    "Run",
                    vec![
                        "Run".to_string(),
                        "Debug".to_string(),
                        "Profile".to_string(),
                    ],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
            ],
            2 => vec![MenubarMenu::new(
                "quick",
                "Quick",
                vec!["Command Palette".to_string(), "Recent".to_string()],
            )],
            _ => vec![
                MenubarMenu::new(
                    "file",
                    "File",
                    vec![
                        "New Tab".to_string(),
                        "New Window".to_string(),
                        "Save".to_string(),
                    ],
                )
                .disabled_indices(vec![2])
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "edit",
                    "Edit",
                    vec!["Undo".to_string(), "Redo".to_string(), "Find".to_string()],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
                MenubarMenu::new(
                    "view",
                    "View",
                    vec![
                        "Zoom In".to_string(),
                        "Zoom Out".to_string(),
                        "Actual Size".to_string(),
                    ],
                )
                .item_kinds(vec![
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                    MenuItemKind::Action,
                ]),
            ],
        });
    let (workbench_last_action, set_workbench_last_action) = signal(None::<(usize, usize)>);
    let on_workbench_action =
        Callback::new(move |action: (usize, usize)| set_workbench_last_action.set(Some(action)));
    let (workbench_open_raw, set_workbench_open_raw) = signal(None::<usize>);
    let on_workbench_open_change =
        Callback::new(move |next: Option<usize>| set_workbench_open_raw.set(next));

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<Menubar".to_string()];

        lines.push("  id_base=\"docs-menubar-workbench\".into()".to_string());
        lines.push("  menus=menus".to_string());
        lines.push("  on_action=on_action".to_string());
        lines.push(format!(
            "  close_on_action={}",
            workbench_close_on_action.get()
        ));
        lines.push(format!(
            "  is_close_on_action={}",
            workbench_close_on_action.get()
        ));

        if workbench_flip_placement.get() {
            lines.push("  placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()".to_string());
        }
        if workbench_default_open.get() {
            lines.push("  default_open_index=0".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-menubar-custom\".into()".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push("  motion=ui::MenubarMotion {".to_string());
            lines.push("    popover: ui::PopoverMotion {".to_string());
            lines.push("      initial_scale: 0.94,".to_string());
            lines.push("      offset_y_px: 10.0,".to_string());
            lines.push("      ..ui::PopoverMotion::default()".to_string());
            lines.push("    },".to_string());
            lines.push("  }".to_string());
        }
        lines.push(
            "  on_open_index_change=Callback::new(move |next| set_open.set(next))".to_string(),
        );
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/menubar/styles.rs */\n{}",
            ui::menubar::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let menus = workbench_menus.get();
        let mut class_tokens = vec!["ui-menubar".to_string()];
        if workbench_custom_class.get() {
            class_tokens.push("docs-menubar-custom".to_string());
        }
        format!(
            "MenubarActualConfig {{\n  id_base: {:?},\n  menus: {:?},\n  on_open_index_change: {:?},\n  on_action: \"last={:?}\",\n  menu_set: \"{}\",\n  menu_count: {},\n  close_on_action: {},\n  is_close_on_action: {:?},\n  placement: \"{}\",\n  default_open_index: {},\n  custom_motion: {},\n  custom_class_name: {},\n  class_name: {:?},\n  class: \"{}\",\n}}",
            "docs-menubar-workbench",
            menus
                .iter()
                .map(|menu| menu.id.as_str())
                .collect::<Vec<_>>(),
            "handler",
            workbench_last_action.get(),
            match menu_set_index.get().unwrap_or(0) {
                1 => "workspace",
                2 => "compact",
                _ => "app",
            },
            menus.len(),
            workbench_close_on_action.get(),
            Some(workbench_close_on_action.get()),
            if workbench_flip_placement.get() {
                "bottom-start-flipped"
            } else {
                "bottom-start"
            },
            if workbench_default_open.get() {
                "Some(0)"
            } else {
                "None"
            },
            workbench_custom_motion.get(),
            workbench_custom_class.get(),
            if workbench_custom_class.get() {
                "docs-menubar-custom"
            } else {
                ""
            },
            class_tokens.join(" ")
        )
    });

    let code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal(None::<(usize, usize)>);

<Menubar
  id_base="docs-menubar".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |action: (usize, usize)| set_last_action.set(Some(action)))
/>
<span class="ui-muted">
  "last action (menu:item): "
  {move || last_action.get().map(|(m, i)| format!("{m}:{i}")).unwrap_or_else(|| "None".to_string())}
</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open_menu, set_open_menu) = signal(None::<usize>);

<Menubar
  id_base="docs-menubar-controlled".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |_: (usize, usize)| {})
  close_on_action=false
  open_index=Signal::derive(move || open_menu.get())
  on_open_index_change=Callback::new(move |next| set_open_menu.set(next))
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(Some(0usize));

<Menubar
  id_base="docs-menubar-markers".to_string()
  menus=vec![
    MenubarMenu::new("file", "File", vec!["New Tab".to_string(), "New Window".to_string()]),
    MenubarMenu::new("edit", "Edit", vec!["Undo".to_string(), "Redo".to_string()]),
  ]
  on_action=Callback::new(move |_: (usize, usize)| {})
  close_on_action=false
  placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
  open_index=Signal::derive(move || open_raw.get())
  default_open_index=1
  on_open_index_change=Callback::new(move |next| set_open_raw.set(next))
  class_name="docs-menubar-custom".to_string()
  motion=ui::MenubarMotion {
    popover: ui::PopoverMotion {
      initial_scale: 0.94,
      offset_y_px: 10.0,
      ..ui::PopoverMotion::default()
    },
  }
/>"#
        .to_string()
    });

    let marker_motion = ui::MenubarMotion {
        popover: ui::PopoverMotion {
            initial_scale: 0.94,
            offset_y_px: 10.0,
            ..ui::PopoverMotion::default()
        },
    };
    let default_menus_for_hello = default_menus.clone();
    let default_menus_for_default = default_menus.clone();
    let controlled_menus_for_matrix = controlled_menus.clone();
    let controlled_menus_for_controlled = controlled_menus.clone();

    view! {
        <ComponentPage
            title="Menubar"
            slug="menubar"
            group="Collections"
            description="baseline-compatible persistent menubar with horizontal trigger navigation, baseline-style state/source attrs, and baseline-level spring popover motion reuse."
        >
            <Playground title="Hello World (Default API)" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-hello".to_string()
                        menus=default_menus_for_hello
                        on_action=on_action
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Menubar."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/menu/menubar/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Menu set"</div>
                        <SegmentedControl
                            id_base="docs-menubar-workbench-set".to_string()
                            options=menu_set_options.clone()
                            selected_index=menu_set_index
                            set_selected_index=set_menu_set_index
                            size=SegmentedControlSize::Sm
                            aria_label="Menubar menu set".to_string()
                        />
                        <Switch
                            checked=workbench_close_on_action
                            set_checked=set_workbench_close_on_action
                        >
                            "Close on action"
                        </Switch>
                        <Switch
                            checked=workbench_flip_placement
                            set_checked=set_workbench_flip_placement
                        >
                            "Flip placement"
                        </Switch>
                        <Switch
                            checked=workbench_default_open
                            set_checked=set_workbench_default_open
                        >
                            "Default open menu"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class
                            set_checked=set_workbench_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=workbench_custom_motion
                            set_checked=set_workbench_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            {move || {
                                let placement = if workbench_flip_placement.get() {
                                    ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
                                } else {
                                    ui::menubar::DEFAULT_PLACEMENT
                                };
                                let class_name = if workbench_custom_class.get() {
                                    "docs-menubar-custom".to_string()
                                } else {
                                    String::new()
                                };
                                let motion = if workbench_custom_motion.get() {
                                    ui::MenubarMotion {
                                        popover: ui::PopoverMotion {
                                            initial_scale: 0.94,
                                            offset_y_px: 10.0,
                                            ..ui::PopoverMotion::default()
                                        },
                                    }
                                } else {
                                    ui::MenubarMotion::default()
                                };

                                if workbench_default_open.get() {
                                    view! {
                                        <Menubar
                                            id_base="docs-menubar-workbench".to_string()
                                            menus=workbench_menus.get()
                                            on_action=on_workbench_action
                                            close_on_action=workbench_close_on_action.get()
                                            is_close_on_action=workbench_close_on_action.get()
                                            placement=placement
                                            default_open_index=0
                                            on_open_index_change=on_workbench_open_change
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <Menubar
                                            id_base="docs-menubar-workbench".to_string()
                                            menus=workbench_menus.get()
                                            on_action=on_workbench_action
                                            close_on_action=workbench_close_on_action.get()
                                            is_close_on_action=workbench_close_on_action.get()
                                            placement=placement
                                            on_open_index_change=on_workbench_open_change
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                    .into_any()
                                }
                            }}
                            <span class="ui-muted">
                                "open menu index: "
                                {move || {
                                    workbench_open_raw
                                        .get()
                                        .map(|index| index.to_string())
                                        .unwrap_or_else(|| "None".to_string())
                                }}
                            </span>
                            <span class="ui-muted">
                                "last action (menu:item): "
                                {move || {
                                    workbench_last_action
                                        .get()
                                        .map(|(menu_index, item_index)| {
                                            format!("{}:{}", menu_index, item_index)
                                        })
                                        .unwrap_or_else(|| "None".to_string())
                                }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Open / Close / Controlled Comparison)" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-matrix".to_string()
                        menus=controlled_menus_for_matrix
                        on_action=on_controlled_action
                        close_on_action=false
                        is_close_on_action=false
                        open_index=controlled_open
                        on_open_index_change=on_open_index_change
                        class_name="docs-menubar-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Desktop Menubar + Action Dispatch" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-default".to_string()
                        menus=default_menus_for_default
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open + Persistent + Disabled Menu" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <Menubar
                        id_base="docs-menubar-controlled".to_string()
                        menus=controlled_menus_for_controlled
                        on_action=on_controlled_action
                        close_on_action=false
                        open_index=controlled_open
                        on_open_index_change=on_open_index_change
                        class_name="docs-menubar-custom".to_string()
                    />
                    <span class="ui-muted">
                        "open menu index: "
                        {move || {
                            controlled_open_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_controlled_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_marker_open_raw.set(Some(0))>
                            "Open Menu 0"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(Some(1))>
                            "Open Menu 1"
                        </button>
                        <button type="button" on:click=move |_| set_marker_open_raw.set(None)>
                            "Close"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-class-source / data-close-on-action-source / data-open-index-source / data-motion-source in DevTools."
                    </div>
                    <Menubar
                        id_base="docs-menubar-markers".to_string()
                        menus=marker_menus
                        on_action=on_marker_action
                        close_on_action=false
                        placement=ui::menubar::DEFAULT_PLACEMENT.flip_vertical()
                        open_index=marker_open
                        default_open_index=1
                        on_open_index_change=on_marker_open_change
                        class_name="docs-menubar-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "open menu index: "
                        {move || {
                            marker_open_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                    <span class="ui-muted">
                        "last action (menu:item): "
                        {move || {
                            last_marker_action
                                .get()
                                .map(|(menu_index, item_index)| {
                                    format!("{}:{}", menu_index, item_index)
                                })
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn navigation_menu() -> AnyView {
    let base_items = vec![
        NavigationMenuItem::new("overview", "Overview", "/docs/overview"),
        NavigationMenuItem::new("components", "Components", "/docs/components"),
        NavigationMenuItem::new("patterns", "Patterns", "/docs/patterns"),
        NavigationMenuItem::new("tokens", "Tokens", "/docs/tokens").disabled(true),
    ];

    let controlled_items = vec![
        NavigationMenuItem::new("home", "Home", "/"),
        NavigationMenuItem::new("docs", "Docs", "/docs"),
        NavigationMenuItem::new("blog", "Blog", "/blog"),
    ];

    let marker_items = vec![
        NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
        NavigationMenuItem::new("projects", "Projects", "/projects"),
        NavigationMenuItem::new("settings", "Settings", "/settings"),
    ];

    let (last_selected, set_last_selected) = signal("none".to_string());
    let on_selected_id_change = Callback::new(move |next: Option<String>| {
        set_last_selected.set(next.unwrap_or_else(|| "none".to_string()))
    });

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some("docs".to_string()));
    let controlled_selected: Signal<Option<String>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<String>| {
        set_controlled_selected_raw.set(next);
    });

    let (marker_selected_raw, set_marker_selected_raw) = signal(Some("projects".to_string()));
    let marker_selected: Signal<Option<String>> = Signal::derive(move || marker_selected_raw.get());
    let on_marker_selected_change = Callback::new(move |next: Option<String>| {
        set_marker_selected_raw.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (last_selected, set_last_selected) = signal("none".to_string());

<NavigationMenu
  id_base="docs-navigation-menu".to_string()
  items=vec![
    NavigationMenuItem::new("overview", "Overview", "/overview"),
    NavigationMenuItem::new("components", "Components", "/components"),
    NavigationMenuItem::new("patterns", "Patterns", "/patterns"),
  ]
  default_selected_id="components".to_string()
  on_selected_id_change=Callback::new(move |next: Option<String>| {
    set_last_selected.set(next.unwrap_or_else(|| "none".to_string()));
  })
/>
<span class="ui-muted">"last selected: " {move || last_selected.get()}</span>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some("docs".to_string()));

<NavigationMenu
  id_base="docs-navigation-menu-controlled".to_string()
  items=vec![
    NavigationMenuItem::new("docs", "Docs", "/docs"),
    NavigationMenuItem::new("api", "API", "/api"),
    NavigationMenuItem::new("guides", "Guides", "/guides"),
  ]
  selected_id=Signal::derive(move || selected.get())
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some("projects".to_string()));
let mut custom_motion = ui::NavigationMenuMotion::default();
custom_motion.spring.stiffness = 260.0;
custom_motion.spring.damping = 24.0;

<NavigationMenu
  id_base="docs-navigation-menu-markers".to_string()
  items=vec![
    NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
    NavigationMenuItem::new("projects", "Projects", "/projects"),
    NavigationMenuItem::new("settings", "Settings", "/settings"),
  ]
  selected_id=Signal::derive(move || selected.get())
  default_selected_id="workspace".to_string()
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
  aria_label="Workspace navigation".to_string()
  class_name="docs-navigation-menu-custom".to_string()
  motion=custom_motion
/>"#
        .to_string()
    });

    let (workbench_selected_raw, set_workbench_selected_raw) = signal(Some("projects".to_string()));
    let workbench_selected: Signal<Option<String>> =
        Signal::derive(move || workbench_selected_raw.get());
    let on_workbench_selected_change = Callback::new(move |next: Option<String>| {
        set_workbench_selected_raw.set(next);
    });
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_activate_on_focus, set_workbench_activate_on_focus) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_stiffness, set_workbench_stiffness) = signal(260_u16);
    let (workbench_damping, set_workbench_damping) = signal(24_u16);

    let workbench_code = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let activate_on_focus = workbench_activate_on_focus.get();
        let disable_second = workbench_disable_second.get();
        let custom_class = workbench_custom_class.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_motion = workbench_custom_motion.get();
        let mode_line = if controlled {
            "  selected_id=Signal::derive(move || selected.get())\n  on_selected_id_change=Callback::new(move |next| set_selected.set(next))\n".to_string()
        } else {
            "  default_selected_id=\"workspace\".into()\n".to_string()
        };
        let class_line = if custom_class {
            "  class_name=\"docs-navigation-menu-custom\".into()\n".to_string()
        } else {
            String::new()
        };
        let aria_line = if custom_aria {
            "  aria_label=\"Workbench navigation\".into()\n".to_string()
        } else {
            String::new()
        };
        let motion_line = if custom_motion {
            format!(
                "  motion={{\n    let mut motion = ui::NavigationMenuMotion::default();\n    motion.spring.stiffness = {}.0;\n    motion.spring.damping = {}.0;\n    motion\n  }}\n",
                workbench_stiffness.get(),
                workbench_damping.get(),
            )
        } else {
            String::new()
        };
        format!(
            "<NavigationMenu\n  id_base=\"docs-navigation-menu-workbench\".into()\n  items=items /* second item disabled: {disable_second} */\n{mode_line}  activate_on_focus={activate_on_focus}\n{class_line}{aria_line}{motion_line}/>"
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/menu/navigation_menu/styles.rs */\n{}",
            ui::navigation_menu::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected_raw
            .get()
            .unwrap_or_else(|| "none".to_string());
        let controlled = workbench_controlled.get();
        let activate_on_focus = workbench_activate_on_focus.get();
        let disable_second = workbench_disable_second.get();
        let custom_class = workbench_custom_class.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_motion = workbench_custom_motion.get();
        let workbench_items = {
            let items = if disable_second {
                let items = vec![
                    NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                    NavigationMenuItem::new("projects", "Projects", "/projects").disabled(true),
                    NavigationMenuItem::new("settings", "Settings", "/settings"),
                ];
                items
            } else {
                vec![
                    NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                    NavigationMenuItem::new("projects", "Projects", "/projects"),
                    NavigationMenuItem::new("settings", "Settings", "/settings"),
                ]
            };
            items
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<String>>()
        };
        format!(
            "NavigationMenuActualConfig {{\n  id_base: {:?},\n  items: {:?},\n  default_selected_id: {:?},\n  on_selected_id_change: {:?},\n  class_name: {:?},\n  mode: \"{}\",\n  selected_id: \"{}\",\n  activate_on_focus: {},\n  disable_second_item: {},\n  custom_class: {},\n  custom_aria_label: {},\n  custom_motion: {},\n  spring_stiffness: {}.0,\n  spring_damping: {}.0,\n}}",
            "docs-navigation-menu-workbench",
            workbench_items,
            if controlled {
                None::<String>
            } else {
                Some("workspace".to_string())
            },
            "handler",
            if custom_class {
                "docs-navigation-menu-custom"
            } else {
                ""
            },
            if controlled {
                "controlled"
            } else {
                "uncontrolled"
            },
            selected,
            activate_on_focus,
            disable_second,
            custom_class,
            custom_aria,
            custom_motion,
            workbench_stiffness.get(),
            workbench_damping.get(),
        )
    });

    let mut marker_motion = ui::NavigationMenuMotion::default();
    marker_motion.spring.stiffness = 260.0;
    marker_motion.spring.damping = 24.0;

    view! {
        <ComponentPage
            title="NavigationMenu"
            slug="navigation-menu"
            group="Collections"
            description="baseline-compatible horizontal navigation menu with roving keyboard focus, controllable selection state, baseline data contracts, and baseline-level active-highlight spring motion reuse."
        >
            <Playground title="Default + Roving Focus + Selection" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-default".to_string()
                        items=base_items
                        default_selected_id="components".to_string()
                        on_selected_id_change=on_selected_id_change
                    />
                    <span class="ui-muted">
                        "last selected: "
                        {move || last_selected.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with baseline/configured comparison, live settings, copy-ready code, and scoped CSS test."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/menu/navigation_menu/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="navigation-menu-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_controlled.get()
                                on:change=move |ev| set_workbench_controlled.set(event_target_checked(&ev))
                            />
                            " Controlled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_activate_on_focus.get()
                                on:change=move |ev| set_workbench_activate_on_focus.set(event_target_checked(&ev))
                            />
                            " Activate on focus"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_second.get()
                                on:change=move |ev| set_workbench_disable_second.set(event_target_checked(&ev))
                            />
                            " Disable second item"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " Custom aria label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            "Stiffness "
                            <input
                                type="range"
                                min="160"
                                max="360"
                                prop:value=move || workbench_stiffness.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_stiffness.set(next.clamp(160, 360));
                                    }
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "Damping "
                            <input
                                type="range"
                                min="12"
                                max="40"
                                prop:value=move || workbench_damping.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_damping.set(next.clamp(12, 40));
                                    }
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="navigation-menu-workbench">
                    <span class="ui-muted">
                        "display: baseline vs configured"
                    </span>
                    <div class="docs-row">
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("workspace".to_string()))>
                            "Select Workspace"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("projects".to_string()))>
                            "Select Projects"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(Some("settings".to_string()))>
                            "Select Settings"
                        </button>
                        <button type="button" on:click=move |_| set_workbench_selected_raw.set(None)>
                            "Clear"
                        </button>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Baseline"</div>
                        <NavigationMenu
                            id_base="docs-navigation-menu-workbench-baseline".to_string()
                            items=vec![
                                NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                                NavigationMenuItem::new("projects", "Projects", "/projects"),
                                NavigationMenuItem::new("settings", "Settings", "/settings"),
                            ]
                            default_selected_id="workspace".to_string()
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Configured"</div>
                        {move || {
                            let items = vec![
                                NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                                NavigationMenuItem::new("projects", "Projects", "/projects")
                                    .disabled(workbench_disable_second.get()),
                                NavigationMenuItem::new("settings", "Settings", "/settings"),
                            ];
                            let custom_class = workbench_custom_class.get();
                            let custom_aria = workbench_custom_aria.get();
                            let mut motion = ui::NavigationMenuMotion::default();
                            if workbench_custom_motion.get() {
                                motion.spring.stiffness = f64::from(workbench_stiffness.get());
                                motion.spring.damping = f64::from(workbench_damping.get());
                            }

                            if workbench_controlled.get() {
                                match (custom_aria, custom_class) {
                                    (true, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (true, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            selected_id=workbench_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                }
                            } else {
                                let default_selected = workbench_selected_raw
                                    .get()
                                    .unwrap_or_else(|| "workspace".to_string());
                                match (custom_aria, custom_class) {
                                    (true, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (true, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            aria_label="Workbench navigation".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, true) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            class_name="docs-navigation-menu-custom".to_string()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                    (false, false) => view! {
                                        <NavigationMenu
                                            id_base="docs-navigation-menu-workbench".to_string()
                                            items=items
                                            default_selected_id=default_selected
                                            on_selected_id_change=on_workbench_selected_change
                                            activate_on_focus=workbench_activate_on_focus.get()
                                            motion=motion
                                        />
                                    }
                                    .into_any(),
                                }
                            }
                        }}
                    </div>
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Controlled / Marker Comparison)"
                code_signal=states_code
            >
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-matrix-default".to_string()
                        items=vec![
                            NavigationMenuItem::new("overview", "Overview", "/docs/overview"),
                            NavigationMenuItem::new("components", "Components", "/docs/components"),
                            NavigationMenuItem::new("patterns", "Patterns", "/docs/patterns"),
                        ]
                        default_selected_id="components".to_string()
                    />
                    <NavigationMenu
                        id_base="docs-navigation-menu-matrix-controlled".to_string()
                        items=vec![
                            NavigationMenuItem::new("home", "Home", "/"),
                            NavigationMenuItem::new("docs", "Docs", "/docs"),
                            NavigationMenuItem::new("blog", "Blog", "/blog"),
                        ]
                        selected_id=workbench_selected
                        on_selected_id_change=on_workbench_selected_change
                        activate_on_focus=false
                        class_name="docs-navigation-menu-custom".to_string()
                    />
                    <NavigationMenu
                        id_base="docs-navigation-menu-matrix-disabled".to_string()
                        items=vec![
                            NavigationMenuItem::new("workspace", "Workspace", "/workspace"),
                            NavigationMenuItem::new("projects", "Projects", "/projects").disabled(true),
                            NavigationMenuItem::new("settings", "Settings", "/settings"),
                        ]
                        default_selected_id="workspace".to_string()
                        activate_on_focus=true
                    />
                </div>
            </Playground>

            <Playground title="Controlled + Manual Activation" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <NavigationMenu
                        id_base="docs-navigation-menu-controlled".to_string()
                        items=controlled_items
                        selected_id=controlled_selected
                        on_selected_id_change=on_controlled_selected_change
                        activate_on_focus=false
                        class_name="docs-navigation-menu-custom".to_string()
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <button
                            type="button"
                            on:click=move |_| set_marker_selected_raw.set(Some("workspace".to_string()))
                        >
                            "Select Workspace"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_marker_selected_raw.set(Some("projects".to_string()))
                        >
                            "Select Projects"
                        </button>
                        <button type="button" on:click=move |_| set_marker_selected_raw.set(None)>
                            "Clear"
                        </button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-aria-label-source / data-activate-on-focus-source / data-selected-id-source / data-selected-id-change-source / data-motion-source in DevTools."
                    </div>
                    <NavigationMenu
                        id_base="docs-navigation-menu-markers".to_string()
                        items=marker_items
                        selected_id=marker_selected
                        default_selected_id="workspace".to_string()
                        on_selected_id_change=on_marker_selected_change
                        activate_on_focus=false
                        aria_label="Workspace navigation".to_string()
                        class_name="docs-navigation-menu-custom".to_string()
                        motion=marker_motion
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || marker_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn carousel() -> AnyView {
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

pub(super) fn command_dialog() -> AnyView {
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
