use super::*;

pub(crate) fn drawer() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });
    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_drawer: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));
    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });
    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_drawer: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));
    let custom_motion = DrawerMotion {
        sheet: SheetMotion {
            initial_offset_px: 52.0,
            ..SheetMotion::default()
        },
    };

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_footer, set_workbench_show_footer) = signal(true);
    let (workbench_left_placement, set_workbench_left_placement) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_show_close_button, set_workbench_show_close_button) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let open_workbench_drawer: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_drawer: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let on_workbench_exit_complete =
        Callback::new(move |_| set_workbench_exit_count.update(|count| *count += 1));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            DrawerMotion {
                sheet: SheetMotion {
                    initial_offset_px: 60.0,
                    ..SheetMotion::default()
                },
            }
        } else {
            DrawerMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let placement = if workbench_left_placement.get() {
            "Some(DrawerPlacement::Left)"
        } else {
            "Some(DrawerPlacement::Right)"
        };
        let description = if workbench_show_description.get() {
            "Some(\"Workbench description\".to_string())"
        } else {
            "None"
        };
        let footer = if workbench_show_footer.get() {
            "Some(ViewFn)"
        } else {
            "None"
        };
        let motion = if workbench_custom_motion.get() {
            "DrawerMotion { sheet: SheetMotion { initial_offset_px: 60.0, ..SheetMotion::default() } }"
        } else {
            "DrawerMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-drawer-workbench"
        } else {
            ""
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "<Drawer\n  is_open=Signal::derive(move || open_raw.get())\n  default_open=false\n  on_open_change=Callback::new(move |next| set_open_raw.set(next))\n  on_close=Callback::new(move |_| set_open_raw.set(false))\n  id_base=\"docs-drawer-workbench\".to_string()\n  title=\"Workbench drawer\".to_string()\n  description={description}\n  footer={footer}\n  placement={placement}\n  motion={motion}\n  is_close_button_visible=Some({})\n  close_label=Some(\"Close drawer\")\n  lang=Some({lang:?}.to_string())\n  dir=Some({dir})\n  on_exit_complete=Callback::new(move |_| {{}})\n  class_name=Some({class_name:?}.to_string())\n>\n  <div>\"Workbench body\"</div>\n</Drawer>",
            workbench_show_close_button.get()
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            Some("Workbench description")
        } else {
            None
        };
        let footer = if workbench_show_footer.get() {
            "Some(ViewFn)"
        } else {
            "None"
        };
        let placement = if workbench_left_placement.get() {
            "Some(DrawerPlacement::Left)"
        } else {
            "Some(DrawerPlacement::Right)"
        };
        let motion = if workbench_custom_motion.get() {
            "DrawerMotion::custom"
        } else {
            "DrawerMotion::default"
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-drawer-workbench")
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };

        format!(
            "DrawerActualConfig {{\n  is_open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  on_close: \"count={}\",\n  id_base: \"docs-drawer-workbench\",\n  title: \"Workbench drawer\",\n  description: {description:?},\n  footer: {footer},\n  placement: {placement},\n  motion: {motion},\n  is_close_button_visible: Some({}),\n  close_label: Some(\"Close drawer\"),\n  lang: {lang:?},\n  dir: {dir},\n  on_exit_complete: \"count={}\",\n  class_name: {class_name:?},\n}}",
            workbench_open_raw.get(),
            workbench_open_change_count.get(),
            workbench_close_count.get(),
            workbench_show_close_button.get(),
            workbench_exit_count.get(),
        )
    });
    let minimal_code = Signal::derive(move || {
        r#"<Drawer default_open=true id_base="dr".to_string() title="Drawer".to_string()>
  <div>"Drawer content"</div>
</Drawer>"#
            .to_string()
    });
    let semantic_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});

<Drawer
  is_open=Signal::derive(move || open_raw.get())
  id_base="dr".to_string()
  title="Drawer".to_string()
  description="Sheet composition with header/body/footer slots.".to_string()
  placement=DrawerPlacement::Right
  footer=move || view! { ... }
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#
            .to_string()
    });
    let custom_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});

<Drawer
  is_open=Signal::derive(move || open_raw.get())
  id_base="dr-left".to_string()
  title="Left drawer".to_string()
  placement=DrawerPlacement::Left
  motion=DrawerMotion {
    sheet: SheetMotion {
      initial_offset_px: 52.0,
      ..SheetMotion::default()
    }
  }
  is_close_button_visible=false
  class_name="docs-drawer-custom".to_string()
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#
            .to_string()
    });
    let state_matrix_options = vec![
        "Uncontrolled + default_open=true".to_string(),
        "Uncontrolled + no description".to_string(),
        "Controlled + custom title".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_default_open =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_with_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) != 1);
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));
    let open_state_matrix_drawer: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(true));
    let close_state_matrix_drawer: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(false));
    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(0);
        let mut lines = vec![
            "<Drawer".to_string(),
            "  id_base=\"docs-drawer-state-matrix\".into()".to_string(),
            "  title=\"State Matrix\".into()".to_string(),
        ];

        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  description=\"Uncontrolled baseline branch\".into()".to_string());
            }
            1 => {
                lines.push("  default_open=false".to_string());
            }
            _ => {
                lines.push("  is_open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  title=\"Controlled Matrix\".into()".to_string());
                lines.push("  description=\"Controlled branch\".into()".to_string());
            }
        }

        lines.push(">".to_string());
        lines.push("  <div>\"Matrix content\"</div>".to_string());
        lines.push("</Drawer>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let close_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(false));
    let (compare_uncontrolled_open_raw, set_compare_uncontrolled_open_raw) = signal(true);
    let on_compare_uncontrolled_open_change =
        Callback::new(move |next: bool| set_compare_uncontrolled_open_raw.set(next));
    let compare_code = Signal::derive(move || {
        r#"let (controlled_open_raw, set_controlled_open_raw) = signal(false);
let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
let (uncontrolled_open_raw, set_uncontrolled_open_raw) = signal(true);

<Drawer
  id_base="docs-drawer-compare-controlled".into()
  title="Controlled".into()
  is_open=controlled_open
  on_open_change=Callback::new(move |next: bool| set_controlled_open_raw.set(next))
  on_close=Callback::new(move |_| set_controlled_open_raw.set(false))
>
  <div>"Controlled content"</div>
</Drawer>

<Drawer
  id_base="docs-drawer-compare-uncontrolled".into()
  title="Uncontrolled".into()
  default_open=true
  on_open_change=Callback::new(move |next: bool| set_uncontrolled_open_raw.set(next))
>
  <div>"Uncontrolled content"</div>
</Drawer>"#
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
        r#"// Drawer is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<Drawer
  id_base="docs-drawer-stream".into()
  title="Streaming Optional Contract".into()
  default_open=true
>
  <div>"This component defaults to snapshot rendering."</div>
</Drawer>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Drawer"
            slug="drawer"
            group="Overlays"
            description="Sheet composition with centralized placement/description/footer/close state attrs and stable drawer slots."
        >
            <Playground
                title="Hello World (Minimal API)"
                description="No manual state wiring. Start with defaults, then opt into controlled/extended props only when needed."
                code_signal=minimal_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <Drawer
                    default_open=true
                    id_base="docs-drawer-minimal".to_string()
                    title="Hello drawer".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <div>"Minimal drawer body"</div>
                        <div class="ui-muted">
                            "Close button works with internal uncontrolled state by default."
                        </div>
                    </div>
                </Drawer>
            </Playground>
            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="drawer-workbench-controls">
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "description"
                        </Switch>
                        <Switch checked=workbench_show_footer set_checked=set_workbench_show_footer>
                            "footer"
                        </Switch>
                        <Switch checked=workbench_left_placement set_checked=set_workbench_left_placement>
                            "left placement"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch
                            checked=workbench_show_close_button
                            set_checked=set_workbench_show_close_button
                        >
                            "is_close_button_visible"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <Button variant=ButtonVariant::Secondary on_press=open_workbench_drawer>
                                "Open"
                            </Button>
                            <Button variant=ButtonVariant::Secondary on_press=close_workbench_drawer>
                                "Close"
                            </Button>
                        </div>
                    </div>
                }
            >
                <span class="ui-muted" data-slot="drawer-workbench-feedback">
                    "open: " {move || workbench_open_raw.get()}
                    " · on_open_change: " {move || workbench_open_change_count.get()}
                    " · on_close: " {move || workbench_close_count.get()}
                    " · on_exit_complete: " {move || workbench_exit_count.get()}
                </span>
                <Drawer
                    is_open=workbench_open
                    default_open=false
                    on_open_change=on_workbench_open_change
                    on_close=close_workbench_drawer
                    id_base="docs-drawer-workbench".to_string()
                    title="Workbench drawer".to_string()
                    description=if workbench_show_description.get() {
                        "Workbench description".to_string()
                    } else {
                        String::new()
                    }
                    footer=move || {
                        view! {
                            <Show when=move || workbench_show_footer.get()>
                                <div class="docs-row docs-row--end">
                                    <Button variant=ButtonVariant::Secondary on_press=close_workbench_drawer>
                                        "Cancel"
                                    </Button>
                                    <Button on_press=close_workbench_drawer>"Apply"</Button>
                                </div>
                            </Show>
                        }
                        .into_any()
                    }
                    placement=if workbench_left_placement.get() {
                        DrawerPlacement::Left
                    } else {
                        DrawerPlacement::Right
                    }
                    motion=workbench_motion.get()
                    is_close_button_visible=workbench_show_close_button.get()
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                    on_exit_complete=on_workbench_exit_complete
                    class_name=if workbench_custom_class.get() {
                        "docs-drawer-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <div>"Workbench drawer body"</div>
                        <div class="ui-muted">
                            "Use controls to compare placement/motion/close and locale contracts."
                        </div>
                    </div>
                </Drawer>
            </Playground>
            <Playground
                title="Right Drawer + Slots"
                code_signal=semantic_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="drawer-e2e-right-controls">
                    <Button attr:data-slot="drawer-e2e-open-right" on_press=open_semantic_drawer>
                        "Open right drawer"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>
                <Show when=move || present_semantic.get()>
                    <Drawer
                        is_open=open_semantic
                        id_base="docs-drawer-right".to_string()
                        title="Drawer title".to_string()
                        description="Drawer composes Sheet and keeps labeled/description semantics aligned.".to_string()
                        placement=DrawerPlacement::Right
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>"Cancel"</Button>
                                <Button on_press=close_semantic>"Apply"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Drawer body"</div>
                            <div class="ui-muted">"Esc/backdrop closes; focus trap remains active."</div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>
            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-placement-source`, `data-description-source`, `data-footer-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=custom_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="drawer-e2e-custom-controls">
                    <Button attr:data-slot="drawer-e2e-open-custom" on_press=open_custom_drawer>
                        "Open left drawer"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get()}</span>
                </div>
                <Show when=move || present_custom.get()>
                    <Drawer
                        is_open=open_custom
                        id_base="docs-drawer-left".to_string()
                        title="Left drawer".to_string()
                        placement=DrawerPlacement::Left
                        motion=custom_motion
                        is_close_button_visible=false
                        class_name="docs-drawer-custom".to_string()
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only path keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">
                                "Inspect data-placement-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button
                                    attr:data-slot="drawer-e2e-dismiss-custom"
                                    variant=ButtonVariant::Secondary
                                    on_press=close_custom
                                >
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>

            <Playground
                title="State Matrix"
                description="State matrix over controlled/uncontrolled + default_open + description branches."
                code_signal=state_matrix_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-state-matrix">
                    <SegmentedControl
                        id_base="docs-drawer-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Drawer state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_drawer>"Open matrix drawer"</Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=close_state_matrix_drawer
                        >
                            "Close matrix drawer"
                        </Button>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <Drawer
                                    id_base="docs-drawer-state-matrix".to_string()
                                    title="Controlled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Controlled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    is_open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    on_close=close_state_matrix_drawer
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix controlled branch"</div>
                                    </div>
                                </Drawer>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Drawer
                                    id_base="docs-drawer-state-matrix".to_string()
                                    title="Uncontrolled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Uncontrolled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    default_open=state_matrix_default_open.get()
                                    on_open_change=on_state_matrix_open_change
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix uncontrolled branch"</div>
                                    </div>
                                </Drawer>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "default_open: " {move || state_matrix_default_open.get()}
                    </span>
                    <span class="ui-muted">
                        "with_description: " {move || state_matrix_with_description.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side contrast of parent-controlled open state vs default-driven internal state."
                code_signal=compare_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-controlled-uncontrolled">
                    <div class="docs-row">
                        <Button on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(true))>
                            "Open controlled drawer"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_compare_controlled>
                            "Close controlled drawer"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <Drawer
                                id_base="docs-drawer-compare-controlled".to_string()
                                title="Controlled".to_string()
                                description="is_open + on_open_change are owned by parent signal.".to_string()
                                is_open=compare_controlled_open
                                on_open_change=on_compare_controlled_open_change
                                on_close=close_compare_controlled
                            >
                                <div>"Controlled content"</div>
                            </Drawer>
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <Drawer
                                id_base="docs-drawer-compare-uncontrolled".to_string()
                                title="Uncontrolled".to_string()
                                description="default_open initializes once; primitive owns later transitions.".to_string()
                                default_open=true
                                on_open_change=on_compare_uncontrolled_open_change
                            >
                                <div>"Uncontrolled content"</div>
                            </Drawer>
                            <span class="ui-muted">
                                "open (reported by on_open_change): "
                                {move || if compare_uncontrolled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight ui-muted" attr:data-slot="drawer-defaults-contract">
                <strong>"Drawer API/default contract"</strong>
                <span>
                    <code>"components/drawer/src/logic.rs"</code>
                    " re-exports defaults from "
                    <code>"crates/ui-state-primitives/src/drawer.rs"</code>
                </span>
                <span><code>"id_base=\"ui-drawer\""</code></span>
                <span><code>"title=\"Drawer\""</code></span>
                <span><code>"default_open=false"</code></span>
                <span>
                    <code>"is_open + on_open_change"</code>
                    " => controlled; "
                    <code>"default_open"</code>
                    " => uncontrolled initialization"
                </span>
            </div>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Drawer is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="drawer-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-drawer-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Drawer stream mode".to_string()
                    />
                    <Drawer
                        id_base="docs-drawer-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output stays snapshot while keeping output status machine-readable.".to_string()
                        default_open=true
                    >
                        <div>"This component defaults to snapshot rendering."</div>
                    </Drawer>
                    <span class="ui-muted">
                        "requested mode: " {move || stream_requested_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"DRAWER_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-drawer\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" attr:data-slot="drawer-source-paths">
                    <li><code>"components/drawer/src/mod.rs"</code></li>
                    <li><code>"components/drawer/src/logic.rs"</code></li>
                    <li><code>"components/drawer/src/view.rs"</code></li>
                    <li><code>"components/drawer/src/styles.rs"</code></li>
                    <li><code>"components/drawer/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
