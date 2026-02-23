{
    let (open_hello_raw, set_open_hello_raw) = signal(false);
    let open_hello: Signal<bool> = Signal::derive(move || open_hello_raw.get());
    let (present_hello, set_present_hello) = signal(open_hello.get_untracked());
    Effect::new(move |_| {
        if open_hello.get() {
            set_present_hello.set(true);
        }
    });

    let close_hello: OnPress = Callback::new(move |_| set_open_hello_raw.set(false));
    let open_hello_sheet: OnPress = Callback::new(move |_| set_open_hello_raw.set(true));
    let on_hello_exit_complete = Callback::new(move |_| set_present_hello.set(false));

    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_sheet: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_detached_raw, set_open_detached_raw) = signal(false);
    let open_detached: Signal<bool> = Signal::derive(move || open_detached_raw.get());
    let (present_detached, set_present_detached) = signal(open_detached.get_untracked());
    Effect::new(move |_| {
        if open_detached.get() {
            set_present_detached.set(true);
        }
    });

    let close_detached: OnPress = Callback::new(move |_| set_open_detached_raw.set(false));
    let open_detached_sheet: OnPress = Callback::new(move |_| set_open_detached_raw.set(true));
    let on_detached_exit_complete = Callback::new(move |_| set_present_detached.set(false));

    let (open_custom_motion_raw, set_open_custom_motion_raw) = signal(false);
    let open_custom_motion: Signal<bool> = Signal::derive(move || open_custom_motion_raw.get());
    let (present_custom_motion, set_present_custom_motion) =
        signal(open_custom_motion.get_untracked());
    Effect::new(move |_| {
        if open_custom_motion.get() {
            set_present_custom_motion.set(true);
        }
    });

    let close_custom_motion: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(false));
    let open_custom_motion_sheet: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(true));
    let on_custom_motion_exit_complete =
        Callback::new(move |_| set_present_custom_motion.set(false));

    let hello_world_code = Signal::derive(move || {
        r#"<BottomSheet open=open id_base="bottom-sheet".to_string() title="Bottom sheet".to_string() on_close=on_close>
  <div>"..."</div>
</BottomSheet>"#
            .to_string()
    });

    let semantic_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet".to_string()
  title="Update available".to_string()
  description="A newer version with security improvements is ready to install.".to_string()
  on_close=Callback::new(move |_| {})
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let detached_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-detached".to_string()
  title="Quick actions".to_string()
  is_detached=true
  bottom_inset_px=16.0
  is_close_button_visible=false
  class_name="docs-bottom-sheet-custom".to_string()
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let custom_motion_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-motion".to_string()
  title="Motion tuned".to_string()
  description="Custom sheet motion demonstrates data-motion-source contract.".to_string()
  motion=BottomSheetMotion {
    sheet: ui::SheetMotion {
      initial_offset_px: 64.0,
      ..ui::SheetMotion::default()
    }
  }
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let state_matrix_options = vec![
        "Description".to_string(),
        "Title Only".to_string(),
        "Detached + Close Hidden".to_string(),
    ];
    let state_matrix_options_after_workbench = state_matrix_options.clone();
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_has_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_is_detached =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_hide_close =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);

    let (open_state_matrix_raw, set_open_state_matrix_raw) = signal(false);
    let open_state_matrix: Signal<bool> = Signal::derive(move || open_state_matrix_raw.get());
    let (present_state_matrix, set_present_state_matrix) =
        signal(open_state_matrix.get_untracked());
    Effect::new(move |_| {
        if open_state_matrix.get() {
            set_present_state_matrix.set(true);
        }
    });
    let open_state_matrix_sheet: OnPress =
        Callback::new(move |_| set_open_state_matrix_raw.set(true));
    let close_state_matrix_sheet: OnPress =
        Callback::new(move |_| set_open_state_matrix_raw.set(false));
    let on_state_matrix_exit_complete = Callback::new(move |_| set_present_state_matrix.set(false));

    let state_matrix_code = Signal::derive(move || {
        r#"let options = vec![
  "Description".to_string(),
  "Title Only".to_string(),
  "Detached + Close Hidden".to_string(),
];

<SegmentedControl ... />
<BottomSheet
  open=open
  id_base="bottom-sheet-matrix".to_string()
  title="State Matrix".to_string()
  on_close=on_close
/>"#
        .to_string()
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let (compare_present, set_compare_present) = signal(compare_controlled_open.get_untracked());
    Effect::new(move |_| {
        if compare_controlled_open.get() {
            set_compare_present.set(true);
        }
    });
    let open_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(true));
    let close_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(false));
    let on_compare_exit_complete = Callback::new(move |_| set_compare_present.set(false));

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"// BottomSheet is intentionally controlled at the component boundary.
// Uncontrolled open-state belongs to upstream primitives/adapters.
<BottomSheet
  open=open
  id_base="bottom-sheet-controlled".to_string()
  title="Controlled".to_string()
  on_close=on_close
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
    let stream_open_raw = RwSignal::new(true);
    let stream_open: Signal<bool> = Signal::derive(move || stream_open_raw.get());
    let stream_close: OnPress = Callback::new(move |_| stream_open_raw.set(false));
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// BottomSheet is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<BottomSheet
  open=open
  id_base="bottom-sheet-stream".to_string()
  title="Streaming Optional Contract".to_string()
  on_close=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    let workbench_title_options = vec![
        "Default title".to_string(),
        "Quick actions".to_string(),
        "Install update".to_string(),
    ];
    let (workbench_title_index, set_workbench_title_index) = signal(Some(0_usize));
    let workbench_title = Signal::derive(move || match workbench_title_index.get().unwrap_or(0) {
        1 => "Quick actions".to_string(),
        2 => "Install update".to_string(),
        _ => "Bottom sheet".to_string(),
    });

    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_footer, set_workbench_show_footer) = signal(true);
    let (workbench_is_detached, set_workbench_is_detached) = signal(false);
    let (workbench_show_close_button, set_workbench_show_close_button) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });

    let open_workbench_sheet: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_sheet: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_exit_complete = Callback::new(move |_| set_workbench_present.set(false));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            BottomSheetMotion {
                sheet: ui::SheetMotion {
                    initial_offset_px: 72.0,
                    ..ui::SheetMotion::default()
                },
            }
        } else {
            BottomSheetMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let title = workbench_title.get();
        let show_description = workbench_show_description.get();
        let show_footer = workbench_show_footer.get();
        let is_detached = workbench_is_detached.get();
        let show_close_button = workbench_show_close_button.get();
        let custom_motion = workbench_custom_motion.get();

        let description = match title.as_str() {
            "Quick actions" => "Choose one action, then dismiss.",
            "Install update" => "A newer version with security improvements is ready to install.",
            _ => "Default bottom-sheet copy from workbench.",
        };

        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<BottomSheet".to_string(),
            "  open=open".to_string(),
            "  id_base=\"docs-bottom-sheet-workbench\".to_string()".to_string(),
            format!("  title={}.to_string()", rust_string_literal(&title)),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  lang=Some(\"en\".to_string())".to_string(),
            "  dir=Some(A11yDirection::Ltr)".to_string(),
            "  is_handle_visible=Some(true)".to_string(),
            "  show_handle=Some(true)".to_string(),
            "  show_close_button=Some(true)".to_string(),
            "  is_dismissable=Some(true)".to_string(),
            "  is_keyboard_dismiss_disabled=Some(false)".to_string(),
            "  close_label=Some(\"Close bottom sheet\")".to_string(),
        ];
        push_line_when(
            &mut lines,
            show_description,
            format!(
                "  description={}.to_string()",
                rust_string_literal(description)
            ),
        );
        push_line_when(&mut lines, is_detached, "  is_detached=true".to_string());
        push_line_when(
            &mut lines,
            is_detached,
            "  bottom_inset_px=16.0".to_string(),
        );
        push_line_when(
            &mut lines,
            !show_close_button,
            "  is_close_button_visible=false".to_string(),
        );
        push_line_when(
            &mut lines,
            custom_motion,
            "  motion=BottomSheetMotion { sheet: ui::SheetMotion { initial_offset_px: 72.0, ..ui::SheetMotion::default() } }".to_string(),
        );
        push_line_when(
            &mut lines,
            show_footer,
            "  footer=move || view! { <div class=\"docs-row docs-row--end\"><Button variant=ButtonVariant::Secondary>\"Later\"</Button><Button>\"Confirm\"</Button></div> }".to_string(),
        );
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let is_detached = workbench_is_detached.get();
        format!(
            "BottomSheetWorkbenchConfig {{\n  open: {},\n  on_close: Some(\"OnPress\"),\n  id_base: \"docs-bottom-sheet-workbench\",\n  title: {:?},\n  description: {},\n  footer: {},\n  lang: Some(\"en\"),\n  dir: Some(\"ltr\"),\n  motion: {},\n  is_handle_visible: Some(true),\n  show_handle: Some(true),\n  is_close_button_visible: Some({}),\n  show_close_button: Some({}),\n  close_label: Some(\"Close bottom sheet\"),\n  is_detached: Some({}),\n  bottom_inset_px: Some({}),\n  is_dismissable: Some(true),\n  is_keyboard_dismiss_disabled: Some(false),\n  class_name: Some(\"docs-bottom-sheet-workbench\"),\n  show_description: {},\n  show_footer: {},\n  custom_motion: {},\n}}",
            bool_word(workbench_open_raw.get()),
            workbench_title.get(),
            bool_word(workbench_show_description.get()),
            bool_word(workbench_show_footer.get()),
            if workbench_custom_motion.get() {
                "BottomSheetMotion::custom"
            } else {
                "BottomSheetMotion::default"
            },
            bool_word(workbench_show_close_button.get()),
            bool_word(workbench_show_close_button.get()),
            bool_word(is_detached),
            if is_detached { "16.0" } else { "0.0" },
            bool_word(workbench_show_description.get()),
            bool_word(workbench_show_footer.get()),
            bool_word(workbench_custom_motion.get()),
        )
    });

    view! {
        <ComponentPage
            title="BottomSheet"
            slug="bottom-sheet"
            group="Overlays"
            description="baseline-style bottom sheet primitive composed from Sheet with centralized handle/description/footer/detached contracts and stable slot/data-state markers."
        >
            <Playground
                title="Hello World (Minimal Path)"
                description="Default path: no manual state-machine wiring, simple props only."
                code_signal=hello_world_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_hello_sheet>"Open bottom sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_hello_raw.get()}</span>
                </div>

                <Show when=move || present_hello.get()>
                    <BottomSheet
                        open=open_hello
                        id_base="docs-bottom-sheet-hello".to_string()
                        title="Bottom sheet".to_string()
                        on_close=close_hello
                        on_exit_complete=on_hello_exit_complete
                    >
                        <div class="ui-muted">"Hello World path with the minimum public API surface."</div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                description="Button-style workbench: tune props/state and inspect generated config + copy-ready code."
                code_signal=workbench_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-workbench-controls">
                        <div class="docs-search__label">"Title"</div>
                        <SegmentedControl
                            id_base="docs-bottom-sheet-workbench-title".to_string()
                            options=workbench_title_options.clone()
                            selected_index=workbench_title_index
                            set_selected_index=set_workbench_title_index
                            size=SegmentedControlSize::Sm
                            aria_label="BottomSheet workbench title".to_string()
                        />
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_description.get()
                                on:change=move |ev| set_workbench_show_description.set(event_target_checked(&ev))
                            />
                            " Show description"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_footer.get()
                                on:change=move |ev| set_workbench_show_footer.set(event_target_checked(&ev))
                            />
                            " Show footer actions"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_detached.get()
                                on:change=move |ev| set_workbench_is_detached.set(event_target_checked(&ev))
                            />
                            " Detached mode"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_close_button.get()
                                on:change=move |ev| set_workbench_show_close_button.set(event_target_checked(&ev))
                            />
                            " Show close button"
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
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-workbench">
                    <div class="docs-row">
                        <Button attr:data-slot="bottom-sheet-workbench-open" on_press=open_workbench_sheet>
                            "Open interactive sheet"
                        </Button>
                        <Button
                            attr:data-slot="bottom-sheet-workbench-close"
                            variant=ButtonVariant::Secondary
                            on_press=close_workbench_sheet
                        >
                            "Close"
                        </Button>
                        <span class="ui-muted">"open: " {move || workbench_open_raw.get()}</span>
                    </div>
                    <Show when=move || workbench_present.get()>
                        {move || {
                            let title = workbench_title.get();
                            let description = if workbench_show_description.get() {
                                match title.as_str() {
                                    "Quick actions" => "Choose one action, then dismiss.".to_string(),
                                    "Install update" => "A newer version with security improvements is ready to install.".to_string(),
                                    _ => "Default bottom-sheet copy from workbench.".to_string(),
                                }
                            } else {
                                String::new()
                            };
                            let motion = workbench_motion.get();

                            if workbench_show_footer.get() {
                                view! {
                                    <BottomSheet
                                        open=workbench_open
                                        id_base="docs-bottom-sheet-workbench".to_string()
                                        title=title
                                        description=description
                                        is_detached=workbench_is_detached.get()
                                        bottom_inset_px=if workbench_is_detached.get() { 16.0 } else { 0.0 }
                                        is_close_button_visible=workbench_show_close_button.get()
                                        motion=motion
                                        footer=move || {
                                            view! {
                                                <div class="docs-row docs-row--end">
                                                    <Button variant=ButtonVariant::Secondary on_press=close_workbench_sheet>
                                                        "Later"
                                                    </Button>
                                                    <Button on_press=close_workbench_sheet>"Confirm"</Button>
                                                </div>
                                            }
                                            .into_any()
                                        }
                                        on_close=close_workbench_sheet
                                        on_exit_complete=on_workbench_exit_complete
                                    >
                                        <div class="ui-muted">"Interactive preview surface for BottomSheet contracts."</div>
                                    </BottomSheet>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <BottomSheet
                                        open=workbench_open
                                        id_base="docs-bottom-sheet-workbench".to_string()
                                        title=title
                                        description=description
                                        is_detached=workbench_is_detached.get()
                                        bottom_inset_px=if workbench_is_detached.get() { 16.0 } else { 0.0 }
                                        is_close_button_visible=workbench_show_close_button.get()
                                        motion=motion
                                        on_close=close_workbench_sheet
                                        on_exit_complete=on_workbench_exit_complete
                                    >
                                        <div class="ui-muted">"Interactive preview surface for BottomSheet contracts."</div>
                                    </BottomSheet>
                                }
                                .into_any()
                            }
                        }}
                    </Show>
                </div>
            </Playground>



            <Playground
                title="Semantic Bottom Sheet"
                code_signal=semantic_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="bottom-sheet-e2e-semantic-controls">
                    <span data-slot="bottom-sheet-e2e-open-semantic">
                        <Button on_press=open_semantic_sheet>"Open bottom sheet"</Button>
                    </span>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <BottomSheet
                        open=open_semantic
                        id_base="docs-bottom-sheet-semantic".to_string()
                        title="Update available".to_string()
                        description="A newer version with security improvements is ready to install.".to_string()
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Later"
                                </Button>
                                <Button on_press=close_semantic>"Install"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>
                                "Bottom sheet uses Sheet(bottom) + spring motion with stable a11y wiring."
                            </div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap remains active."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Detached + Title Only + Custom Class"
                code_signal=detached_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_detached_sheet>"Open detached sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_detached_raw.get()}</span>
                </div>

                <Show when=move || present_detached.get()>
                    <BottomSheet
                        open=open_detached
                        id_base="docs-bottom-sheet-detached".to_string()
                        title="Quick actions".to_string()
                        is_detached=true
                        bottom_inset_px=16.0
                        is_close_button_visible=false
                        class_name="docs-bottom-sheet-custom".to_string()
                        on_close=close_detached
                        on_exit_complete=on_detached_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only mode keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">"Detached mode applies inset and rounded surface styling."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_detached>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Custom Motion Contract"
                code_signal=custom_motion_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="bottom-sheet-e2e-motion-controls">
                    <span data-slot="bottom-sheet-e2e-open-motion">
                        <Button on_press=open_custom_motion_sheet>"Open custom motion sheet"</Button>
                    </span>
                    <span class="ui-muted">"open: " {move || open_custom_motion_raw.get()}</span>
                </div>

                <Show when=move || present_custom_motion.get()>
                    <BottomSheet
                        open=open_custom_motion
                        id_base="docs-bottom-sheet-motion".to_string()
                        title="Motion tuned".to_string()
                        description="Custom sheet motion flips data-motion-source to custom.".to_string()
                        motion=BottomSheetMotion {
                            sheet: ui::SheetMotion {
                                initial_offset_px: 64.0,
                                ..ui::SheetMotion::default()
                            },
                        }
                        on_close=close_custom_motion
                        on_exit_complete=on_custom_motion_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Motion contract switches to custom markers for style/debug hooks."</div>
                            <div class="ui-muted">"Use devtools to inspect data-motion-source/custom-motion."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="State Scenarios"
                description="Matrix for description/title-only/detached state contracts."
                code_signal=state_matrix_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-state-matrix">
                    <SegmentedControl
                        id_base="docs-bottom-sheet-state-matrix".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_sheet>"Open state matrix sheet"</Button>
                        <span class="ui-muted">"open: " {move || open_state_matrix_raw.get()}</span>
                    </div>

                    <Show when=move || present_state_matrix.get()>
                        {move || {
                            let title = if state_matrix_is_detached.get() {
                                "Detached matrix".to_string()
                            } else {
                                "State matrix".to_string()
                            };
                            let description = if state_matrix_has_description.get() {
                                "Description branch keeps aria-describedby wired.".to_string()
                            } else {
                                String::new()
                            };
                            let is_detached = state_matrix_is_detached.get();
                            let is_close_button_visible = !state_matrix_hide_close.get();

                            view! {
                                <BottomSheet
                                    open=open_state_matrix
                                    id_base="docs-bottom-sheet-state-matrix-open".to_string()
                                    title=title
                                    description=description
                                    is_detached=is_detached
                                    is_close_button_visible=is_close_button_visible
                                    on_close=close_state_matrix_sheet
                                    on_exit_complete=on_state_matrix_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix branch for bottom-sheet semantic markers."</div>
                                        <span class="ui-muted">
                                            "description: "
                                            {move || if state_matrix_has_description.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "detached: "
                                            {move || if state_matrix_is_detached.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "close-button-visible: "
                                            {move || if state_matrix_hide_close.get() { "false" } else { "true" }}
                                        </span>
                                    </div>
                                </BottomSheet>
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="BottomSheet boundary is controlled-only; uncontrolled open state is adapter-level N/A."
                code_signal=controlled_vs_uncontrolled_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="bottom-sheet-controlled-uncontrolled"
                >
                    <div class="docs-row">
                        <Button on_press=open_compare_controlled>"Open controlled sheet"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_compare_controlled>
                            "Close controlled sheet"
                        </Button>
                    </div>

                    <Show when=move || compare_present.get()>
                        <BottomSheet
                            open=compare_controlled_open
                            id_base="docs-bottom-sheet-controlled".to_string()
                            title="Controlled".to_string()
                            description="`open: Signal<bool>` is the single source of truth at component boundary.".to_string()
                            on_close=close_compare_controlled
                            on_exit_complete=on_compare_exit_complete
                        >
                            <div>"Controlled path (component public API)."</div>
                        </BottomSheet>
                    </Show>

                    <div class="docs-stack docs-stack--tight">
                        <strong>"Uncontrolled (N/A for BottomSheet)"</strong>
                        <span class="ui-muted">
                            "`BottomSheet` requires `open: Signal<bool>` + `on_close`; uncontrolled behavior should be adapted upstream via primitives."
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="BottomSheet is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="bottom-sheet-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-bottom-sheet-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet stream mode".to_string()
                    />
                    <BottomSheet
                        open=stream_open
                        id_base="docs-bottom-sheet-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output remains snapshot-compatible while preserving machine-readable status markers.".to_string()
                        on_close=stream_close
                    >
                        <div>"requested mode and output status are displayed as docs contract markers."</div>
                    </BottomSheet>
                    <span class="ui-muted">"requested mode: " {move || stream_requested_mode.get()}</span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>



            <Playground
                title="State Matrix (Description / Title-only / Detached)"
                description="Workbench 后的多参数状态对比。"
                code_signal=state_matrix_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-state-matrix-after-workbench">
                    <SegmentedControl
                        id_base="docs-bottom-sheet-state-matrix-after-workbench".to_string()
                        options=state_matrix_options_after_workbench.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_sheet>"Open state matrix sheet"</Button>
                        <span class="ui-muted">"open: " {move || open_state_matrix_raw.get()}</span>
                    </div>

                    <Show when=move || present_state_matrix.get()>
                        {move || {
                            let title = if state_matrix_is_detached.get() {
                                "Detached matrix".to_string()
                            } else {
                                "State matrix".to_string()
                            };
                            let description = if state_matrix_has_description.get() {
                                "Description branch keeps aria-describedby wired.".to_string()
                            } else {
                                String::new()
                            };
                            let is_detached = state_matrix_is_detached.get();
                            let is_close_button_visible = !state_matrix_hide_close.get();

                            view! {
                                <BottomSheet
                                    open=open_state_matrix
                                    id_base="docs-bottom-sheet-state-matrix-open-after-workbench".to_string()
                                    title=title
                                    description=description
                                    is_detached=is_detached
                                    is_close_button_visible=is_close_button_visible
                                    on_close=close_state_matrix_sheet
                                    on_exit_complete=on_state_matrix_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix branch for bottom-sheet semantic markers."</div>
                                        <span class="ui-muted">
                                            "description: "
                                            {move || if state_matrix_has_description.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "detached: "
                                            {move || if state_matrix_is_detached.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "close-button-visible: "
                                            {move || if state_matrix_hide_close.get() { "false" } else { "true" }}
                                        </span>
                                    </div>
                                </BottomSheet>
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-defaults-contract">
                <h3>"Defaults + API Contract (logic.rs SSOT)"</h3>
                <p class="ui-muted">
                    "Source of truth: "
                    <code>"components/bottom-sheet/src/logic.rs"</code>
                </p>
                <ul class="docs-stack docs-stack--tight">
                    <li><code>"DEFAULT_TITLE = \"Bottom sheet\""</code></li>
                    <li><code>"DEFAULT_CLOSE_LABEL = \"Close bottom sheet\""</code></li>
                    <li><code>"DEFAULT_DISMISSABLE = true"</code></li>
                    <li><code>"DEFAULT_KEYBOARD_DISMISS_DISABLED = false"</code></li>
                    <li><code>"DEFAULT_BOTTOM_INSET_PX = 0.0"</code></li>
                    <li><code>"resolve_handle_visibility(is_handle_visible, show_handle)"</code></li>
                    <li>
                        <code>
                            "resolve_close_button_visibility(is_close_button_visible, show_close_button)"
                        </code>
                    </li>
                    <li><code>"resolve_attachment(is_detached, detached)"</code></li>
                    <li><code>"resolve_dismissable(is_dismissable)"</code></li>
                    <li>
                        <code>
                            "resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled)"
                        </code>
                    </li>
                </ul>
            </div>

            <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"BOTTOM_SHEET_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-bottom_sheet\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" data-slot="bottom-sheet-source-paths">
                    <li><code>"components/bottom-sheet/src/mod.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/logic.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/view.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/styles.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
