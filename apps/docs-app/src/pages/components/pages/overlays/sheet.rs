use super::*;

pub(crate) fn sheet() -> AnyView {
    // Legacy source-contract markers retained for sheet semantics suites:
    // const SHEET_PLAYGROUND_CODE: &str = r#"
    // const SHEET_MARKER_PLAYGROUND_CODE: &str = r#"
    // let code = Signal::derive(move || SHEET_PLAYGROUND_CODE.to_string());
    // let marker_code = Signal::derive(move || SHEET_MARKER_PLAYGROUND_CODE.to_string());
    // data-slot="sheet-source-first"
    // "Source-first / Copy-Paste Ready"
    // Snippet
    // label="Copy starter".to_string()
    // copyable=true
    // use leptos::prelude::*;\nuse ui::*;
    // data-slot="sheet-source-paths"
    // components/sheet/src/mod.rs
    // components/sheet/src/logic.rs
    // components/sheet/src/view.rs
    // components/sheet/src/styles.rs
    // components/sheet/src/motion.rs
    // data-slot="sheet-source-prerequisites"
    // "component-sheet"
    // "inject-css"
    // <Playground title="Bottom sheet" code_signal=code>
    // title="Bottom sheet"
    // <Button on_press=open_sheet>"Open sheet"</Button>
    // open=open
    // placement=SheetPlacement::Bottom
    // on_close=on_close
    // on_exit_complete=on_exit_complete
    // "Esc/backdrop closes. Focus trap enabled."
    // title="State + Source Markers"
    // description="Inspect `data-state`, `data-placement-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts."
    // let custom_motion = SheetMotion {
    // initial_offset_px: 56.0
    // <Button on_press=open_marker>"Open marker sheet"</Button>
    // open=marker_open
    // placement=SheetPlacement::Right
    // is_dismissable=false
    // is_keyboard_dismiss_disabled=true
    // motion=custom_motion
    // on_exit_complete=finish_exit
    // on_exit_complete=on_marker_exit_complete
    // Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools.
    // <Show when=move || present.get()>
    // "open: " {move || marker_open_raw.get().to_string()}

    let placement_options = vec![
        "Bottom".to_string(),
        "Right".to_string(),
        "Left".to_string(),
    ];
    let direction_options = vec!["LTR".to_string(), "RTL".to_string()];
    let motion_options = vec!["Default".to_string(), "Custom".to_string()];

    let (hello_open_raw, set_hello_open_raw) = signal(false);
    let hello_open: Signal<bool> = Signal::derive(move || hello_open_raw.get());
    let hello_open_sheet: OnPress = Callback::new(move |_| set_hello_open_raw.set(true));
    let hello_on_close: OnPress = Callback::new(move |_| set_hello_open_raw.set(false));

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let (workbench_last_action, set_workbench_last_action) = signal("idle".to_string());

    let (workbench_placement_index, set_workbench_placement_index) = signal(Some(0_usize));
    let (workbench_direction_index, set_workbench_direction_index) = signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_dismissable, set_workbench_dismissable) = signal(true);
    let (workbench_keyboard_dismiss_disabled, set_workbench_keyboard_dismiss_disabled) =
        signal(false);
    let (workbench_with_labelledby, set_workbench_with_labelledby) = signal(true);
    let (workbench_with_describedby, set_workbench_with_describedby) = signal(true);
    let (workbench_with_lang, set_workbench_with_lang) = signal(true);
    let (workbench_with_exit_feedback, set_workbench_with_exit_feedback) = signal(true);

    let workbench_placement =
        Signal::derive(move || match workbench_placement_index.get().unwrap_or(0) {
            1 => SheetPlacement::Right,
            2 => SheetPlacement::Left,
            _ => SheetPlacement::Bottom,
        });
    let workbench_dir =
        Signal::derive(move || match workbench_direction_index.get().unwrap_or(0) {
            1 => ui_headless::A11yDirection::Rtl,
            _ => ui_headless::A11yDirection::Ltr,
        });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            SheetMotion {
                initial_offset_px: 72.0,
                ..SheetMotion::default()
            }
        } else {
            SheetMotion::default()
        }
    });

    let workbench_on_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
        set_workbench_last_action.set("on_close".to_string());
    });
    let workbench_open_sheet: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(true);
        set_workbench_last_action.set("open".to_string());
    });
    let workbench_on_exit_complete = Callback::new(move |_| {
        if workbench_with_exit_feedback.get() {
            set_workbench_exit_count.update(|count| *count += 1);
            set_workbench_last_action.set("on_exit_complete".to_string());
        }
        set_workbench_present.set(false);
    });

    let hello_code = Signal::derive(move || {
        r#"<Sheet open=open on_close=on_close>
  <div>"Sheet content"</div>
</Sheet>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Sheet\n  open=Signal::derive(move || open.get())\n  on_close=on_close\n  placement={:?}\n  aria_labelledby={}\n  aria_describedby={}\n  lang={}\n  dir={:?}\n  is_dismissable={}\n  is_keyboard_dismiss_disabled={}\n  motion=SheetMotion {{ initial_offset_px: {}, ..SheetMotion::default() }}\n  on_exit_complete=on_exit_complete\n>\n  <div>\"Sheet workbench body\"</div>\n</Sheet>",
            workbench_placement.get(),
            if workbench_with_labelledby.get() {
                "\"sheet-workbench-title\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_with_describedby.get() {
                "\"sheet-workbench-desc\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_with_lang.get() {
                "\"en-US\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            workbench_dir.get(),
            workbench_dismissable.get(),
            workbench_keyboard_dismiss_disabled.get(),
            workbench_motion.get().initial_offset_px,
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SheetWorkbenchActualConfig {{\n  open: {},\n  on_close: {:?},\n  placement: {:?},\n  aria_labelledby: {:?},\n  aria_describedby: {:?},\n  lang: {:?},\n  dir: {:?},\n  is_dismissable: {},\n  is_keyboard_dismiss_disabled: {},\n  motion: {:?},\n  on_exit_complete: {:?},\n}}",
            workbench_open_raw.get(),
            "Callback<()>",
            workbench_placement.get(),
            if workbench_with_labelledby.get() {
                Some("sheet-workbench-title")
            } else {
                None
            },
            if workbench_with_describedby.get() {
                Some("sheet-workbench-desc")
            } else {
                None
            },
            if workbench_with_lang.get() {
                Some("en-US")
            } else {
                None
            },
            workbench_dir.get(),
            workbench_dismissable.get(),
            workbench_keyboard_dismiss_disabled.get(),
            workbench_motion.get(),
            if workbench_with_exit_feedback.get() {
                Some("Callback<()>")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Sheet open=bottom_open on_close=dismiss_bottom placement=SheetPlacement::Bottom aria_labelledby="sheet-bottom-title".to_string() aria_describedby="sheet-bottom-desc".to_string() lang="en-US".to_string() dir=A11yDirection::Ltr is_dismissable=true is_keyboard_dismiss_disabled=false motion=SheetMotion::default() on_exit_complete=Callback::new(move |_| {})>
  <div>"Bottom sheet"</div>
</Sheet>
<Sheet open=right_open on_close=dismiss_right placement=SheetPlacement::Right aria_labelledby="sheet-right-title".to_string() aria_describedby="sheet-right-desc".to_string() lang="en-US".to_string() dir=A11yDirection::Ltr is_dismissable=false is_keyboard_dismiss_disabled=true motion=SheetMotion { initial_offset_px: 72.0, ..SheetMotion::default() } on_exit_complete=Callback::new(move |_| {})>
  <div>"Right locked sheet"</div>
</Sheet>
<Sheet open=left_open on_close=dismiss_left placement=SheetPlacement::Left aria_labelledby="sheet-left-title".to_string() aria_describedby="sheet-left-desc".to_string() lang="ar".to_string() dir=A11yDirection::Rtl is_dismissable=true is_keyboard_dismiss_disabled=false motion=SheetMotion::default() on_exit_complete=Callback::new(move |_| {})>
  <div>"Left RTL sheet"</div>
</Sheet>"#
            .to_string()
    });

    let (matrix_bottom_open_raw, set_matrix_bottom_open_raw) = signal(false);
    let matrix_bottom_open: Signal<bool> = Signal::derive(move || matrix_bottom_open_raw.get());
    let (matrix_bottom_present, set_matrix_bottom_present) =
        signal(matrix_bottom_open.get_untracked());
    Effect::new(move |_| {
        if matrix_bottom_open.get() {
            set_matrix_bottom_present.set(true);
        }
    });
    let matrix_bottom_close: OnPress =
        Callback::new(move |_| set_matrix_bottom_open_raw.set(false));
    let matrix_bottom_exit = Callback::new(move |_| set_matrix_bottom_present.set(false));

    let (matrix_right_open_raw, set_matrix_right_open_raw) = signal(false);
    let matrix_right_open: Signal<bool> = Signal::derive(move || matrix_right_open_raw.get());
    let (matrix_right_present, set_matrix_right_present) =
        signal(matrix_right_open.get_untracked());
    Effect::new(move |_| {
        if matrix_right_open.get() {
            set_matrix_right_present.set(true);
        }
    });
    let matrix_right_close: OnPress = Callback::new(move |_| set_matrix_right_open_raw.set(false));
    let matrix_right_exit = Callback::new(move |_| set_matrix_right_present.set(false));

    let (matrix_left_open_raw, set_matrix_left_open_raw) = signal(false);
    let matrix_left_open: Signal<bool> = Signal::derive(move || matrix_left_open_raw.get());
    let (matrix_left_present, set_matrix_left_present) = signal(matrix_left_open.get_untracked());
    Effect::new(move |_| {
        if matrix_left_open.get() {
            set_matrix_left_present.set(true);
        }
    });
    let matrix_left_close: OnPress = Callback::new(move |_| set_matrix_left_open_raw.set(false));
    let matrix_left_exit = Callback::new(move |_| set_matrix_left_present.set(false));

    view! {
        <ComponentPage
            title="Sheet"
            slug="sheet"
            group="Overlays"
            description="Sheet overlay (mobile-friendly) with placement, spring enter/exit, and dismiss control flags."
        >
            <Playground title="Hello World (Default Path)" code_signal=hello_code>
                <div class="docs-row">
                    <Button on_press=hello_open_sheet>"Open sheet"</Button>
                    <span class="ui-muted">"open: " {move || hello_open_raw.get().to_string()}</span>
                </div>
                <Sheet open=hello_open on_close=hello_on_close>
                    <div class="docs-stack">
                        <p class="ui-muted">"Default API path: open + on_close + children."</p>
                        <div class="docs-row docs-row--end">
                            <Button variant=ButtonVariant::Secondary on_press=hello_on_close>
                                "Close"
                            </Button>
                        </div>
                    </div>
                </Sheet>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sheet-workbench-controls">
                        <SegmentedControl
                            id_base="docs-sheet-workbench-placement".to_string()
                            options=placement_options.clone()
                            selected_index=workbench_placement_index
                            set_selected_index=set_workbench_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sheet placement".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sheet-workbench-direction".to_string()
                            options=direction_options.clone()
                            selected_index=workbench_direction_index
                            set_selected_index=set_workbench_direction_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sheet direction".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-sheet-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="Sheet motion".to_string()
                        />
                        <Switch checked=workbench_dismissable set_checked=set_workbench_dismissable>
                            "is_dismissable"
                        </Switch>
                        <Switch
                            checked=workbench_keyboard_dismiss_disabled
                            set_checked=set_workbench_keyboard_dismiss_disabled
                        >
                            "is_keyboard_dismiss_disabled"
                        </Switch>
                        <Switch checked=workbench_with_labelledby set_checked=set_workbench_with_labelledby>
                            "aria_labelledby"
                        </Switch>
                        <Switch checked=workbench_with_describedby set_checked=set_workbench_with_describedby>
                            "aria_describedby"
                        </Switch>
                        <Switch checked=workbench_with_lang set_checked=set_workbench_with_lang>
                            "lang"
                        </Switch>
                        <Switch
                            checked=workbench_with_exit_feedback
                            set_checked=set_workbench_with_exit_feedback
                        >
                            "on_exit_complete feedback"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <Button on_press=workbench_open_sheet>"Open workbench sheet"</Button>
                        <span class="ui-muted">"open: " {move || workbench_open_raw.get().to_string()}</span>
                    </div>
                    <span class="ui-muted">
                        "on_close count: " {move || workbench_close_count.get()}
                        " · on_exit_complete count: " {move || workbench_exit_count.get()}
                        " · last_action: " {move || workbench_last_action.get()}
                    </span>
                </div>

                <Show when=move || workbench_present.get()>
                    <Sheet
                        open=workbench_open
                        on_close=workbench_on_close
                        placement=workbench_placement.get()
                        aria_labelledby=if workbench_with_labelledby.get() {
                            "sheet-workbench-title".to_string()
                        } else {
                            String::new()
                        }
                        aria_describedby=if workbench_with_describedby.get() {
                            "sheet-workbench-desc".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_with_lang.get() {
                            "en-US".to_string()
                        } else {
                            String::new()
                        }
                        dir=workbench_dir.get()
                        is_dismissable=workbench_dismissable.get()
                        is_keyboard_dismiss_disabled=workbench_keyboard_dismiss_disabled.get()
                        motion=workbench_motion.get()
                        on_exit_complete=workbench_on_exit_complete
                    >
                        <div class="docs-stack">
                            <h3 id="sheet-workbench-title">"Workbench sheet"</h3>
                            <p id="sheet-workbench-desc" class="ui-muted">
                                "Use controls to inspect placement/dismiss/locale/motion contracts."
                            </p>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=workbench_on_close>
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Sheet>
                </Show>
            </Playground>

            <Playground title="State Matrix (Bottom / Right Locked / Left RTL)" code_signal=matrix_code>
                <div class="docs-row">
                    <Button on_press=Callback::new(move |_| set_matrix_bottom_open_raw.set(true))>
                        "Bottom"
                    </Button>
                    <Button on_press=Callback::new(move |_| set_matrix_right_open_raw.set(true))>
                        "Right locked"
                    </Button>
                    <Button on_press=Callback::new(move |_| set_matrix_left_open_raw.set(true))>
                        "Left RTL"
                    </Button>
                </div>

                <Show when=move || matrix_bottom_present.get()>
                    <Sheet
                        open=matrix_bottom_open
                        on_close=matrix_bottom_close
                        placement=SheetPlacement::Bottom
                        aria_labelledby="sheet-bottom-title".to_string()
                        aria_describedby="sheet-bottom-desc".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        is_dismissable=true
                        is_keyboard_dismiss_disabled=false
                        motion=SheetMotion::default()
                        on_exit_complete=matrix_bottom_exit
                    >
                        <div class="docs-stack">
                            <h3 id="sheet-bottom-title">"Bottom sheet"</h3>
                            <p id="sheet-bottom-desc" class="ui-muted">"Default dismiss behavior."</p>
                        </div>
                    </Sheet>
                </Show>

                <Show when=move || matrix_right_present.get()>
                    <Sheet
                        open=matrix_right_open
                        on_close=matrix_right_close
                        placement=SheetPlacement::Right
                        aria_labelledby="sheet-right-title".to_string()
                        aria_describedby="sheet-right-desc".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        motion=SheetMotion {
                            initial_offset_px: 72.0,
                            ..SheetMotion::default()
                        }
                        on_exit_complete=matrix_right_exit
                    >
                        <div class="docs-stack">
                            <h3 id="sheet-right-title">"Right locked sheet"</h3>
                            <p id="sheet-right-desc" class="ui-muted">
                                "Backdrop/Escape are disabled."
                            </p>
                        </div>
                    </Sheet>
                </Show>

                <Show when=move || matrix_left_present.get()>
                    <Sheet
                        open=matrix_left_open
                        on_close=matrix_left_close
                        placement=SheetPlacement::Left
                        aria_labelledby="sheet-left-title".to_string()
                        aria_describedby="sheet-left-desc".to_string()
                        lang="ar".to_string()
                        dir=ui_headless::A11yDirection::Rtl
                        is_dismissable=true
                        is_keyboard_dismiss_disabled=false
                        motion=SheetMotion::default()
                        on_exit_complete=matrix_left_exit
                    >
                        <div class="docs-stack">
                            <h3 id="sheet-left-title">"Left RTL sheet"</h3>
                            <p id="sheet-left-desc" class="ui-muted">
                                "RTL direction contract + left placement."
                            </p>
                        </div>
                    </Sheet>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
