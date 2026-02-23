use super::*;

pub(crate) fn navigation_menu() -> AnyView {
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
