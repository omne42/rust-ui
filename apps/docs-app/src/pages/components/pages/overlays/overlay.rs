use super::*;

pub(crate) fn overlay() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());

    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_overlay: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let (marker_open_raw, set_marker_open_raw) = signal(false);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());

    let (marker_present, set_marker_present) = signal(marker_open.get_untracked());
    Effect::new(move |_| {
        if marker_open.get() {
            set_marker_present.set(true);
        }
    });

    let close_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(false));
    let open_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(true));
    let on_marker_exit_complete = Callback::new(move |_| set_marker_present.set(false));

    let marker_motion = OverlayMotion {
        initial_scale: 0.94,
        initial_y_px: 14.0,
        ..OverlayMotion::default()
    };

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let (present, set_present) = signal(open.get_untracked());
let on_close: OnPress = Callback::new(move |_| set_open.set(false));
let on_exit_complete = Callback::new(move |_| set_present.set(false));

<Show when=move || present.get()>
  <Overlay
    open=Signal::derive(move || open.get())
    on_close=on_close
    on_exit_complete=on_exit_complete
  >
    ...
  </Overlay>
</Show>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let motion = OverlayMotion {
  initial_scale: 0.94,
  initial_y_px: 14.0,
  ..OverlayMotion::default()
};

<Overlay
  open=open
  on_close=close
  role="alertdialog"
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  motion=motion
  class_name="docs-overlay-state".to_string()
  aria_labelledby="overlay-marker-title".to_string()
  aria_describedby="overlay-marker-desc".to_string()
  on_exit_complete=Callback::new(move |_| {})
>
  <div class="ui-card">
    <h4 id="overlay-marker-title">"Overlay markers"</h4>
    <p id="overlay-marker-desc">"Verifies controlled state and source markers."</p>
  </div>
</Overlay>"#
            .to_string()
    });

    let role_options = vec!["dialog".to_string(), "alertdialog".to_string()];
    let motion_options = vec!["Default".to_string(), "Custom".to_string()];
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let (workbench_role_index, set_workbench_role_index) = signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_dismissable, set_workbench_dismissable) = signal(true);
    let (workbench_keyboard_dismiss_disabled, set_workbench_keyboard_dismiss_disabled) =
        signal(false);
    let (workbench_with_aria, set_workbench_with_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);

    let workbench_role = Signal::derive(move || {
        if workbench_role_index.get().unwrap_or(0) == 1 {
            "alertdialog"
        } else {
            "dialog"
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            OverlayMotion {
                initial_scale: 0.94,
                initial_y_px: 14.0,
                ..OverlayMotion::default()
            }
        } else {
            OverlayMotion::default()
        }
    });
    let open_workbench_overlay: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let workbench_on_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let workbench_on_exit_complete = Callback::new(move |_| {
        set_workbench_present.set(false);
        set_workbench_exit_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Overlay\n  open=Signal::derive(move || open.get())\n  on_close=on_close\n  aria_labelledby={}\n  aria_describedby={}\n  role={:?}\n  is_dismissable={}\n  is_keyboard_dismiss_disabled={}\n  motion=OverlayMotion {{ initial_scale: {}, initial_y_px: {}, ..OverlayMotion::default() }}\n  class_name={}\n>\n  <div>\"Overlay workbench panel\"</div>\n</Overlay>",
            if workbench_with_aria.get() {
                "\"overlay-workbench-title\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            if workbench_with_aria.get() {
                "\"overlay-workbench-desc\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            workbench_role.get(),
            workbench_dismissable.get(),
            workbench_keyboard_dismiss_disabled.get(),
            workbench_motion.get().initial_scale,
            workbench_motion.get().initial_y_px,
            if workbench_custom_class.get() {
                "\"docs-overlay-state\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "OverlayWorkbenchActualConfig {{\n  open: {},\n  on_close: {:?},\n  aria_labelledby: {:?},\n  aria_describedby: {:?},\n  role: {:?},\n  is_dismissable: {},\n  is_keyboard_dismiss_disabled: {},\n  motion: {:?},\n  class_name: {:?},\n}}",
            workbench_open_raw.get(),
            "Callback<()>",
            if workbench_with_aria.get() {
                Some("overlay-workbench-title")
            } else {
                None
            },
            if workbench_with_aria.get() {
                Some("overlay-workbench-desc")
            } else {
                None
            },
            workbench_role.get(),
            workbench_dismissable.get(),
            workbench_keyboard_dismiss_disabled.get(),
            workbench_motion.get(),
            if workbench_custom_class.get() {
                Some("docs-overlay-state")
            } else {
                None
            },
        )
    });

    view! {
        <ComponentPage
            title="Overlay"
            slug="overlay"
            group="Overlays"
            description="Portal + backdrop + focus trap + overlay stack (Esc/topmost). Supports dismiss control flags and requires presence to unmount after exit."
        >
            <Playground title="Hello World (Default Overlay)" code_signal=code>
                <div class="docs-row">
                    <Button on_press=open_overlay>"Open overlay"</Button>
                    <span class="ui-muted">"open: " {move || open_raw.get()}</span>
                </div>

                <Show when=move || present.get()>
                    <Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>
                        <div class="docs-stack">
                            <div>"Overlay panel"</div>
                            <div class="ui-muted">
                                "Esc or click backdrop closes. Tab is trapped."
                            </div>
                            <div class="docs-row">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            </div>
                        </div>
                    </Overlay>
                </Show>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="overlay-workbench-controls">
                        <SegmentedControl
                            id_base="docs-overlay-workbench-role".to_string()
                            options=role_options.clone()
                            selected_index=workbench_role_index
                            set_selected_index=set_workbench_role_index
                            size=SegmentedControlSize::Sm
                            aria_label="Overlay role".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-overlay-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="Overlay motion".to_string()
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
                        <Switch checked=workbench_with_aria set_checked=set_workbench_with_aria>
                            "aria_labelledby + aria_describedby"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-row">
                    <Button on_press=open_workbench_overlay>"Open workbench overlay"</Button>
                    <span class="ui-muted">"open: " {move || workbench_open_raw.get().to_string()}</span>
                    <span class="ui-muted">
                        "on_close: " {move || workbench_close_count.get()}
                        " · on_exit_complete: " {move || workbench_exit_count.get()}
                    </span>
                </div>
                <Show when=move || workbench_present.get()>
                    <Overlay
                        open=workbench_open
                        on_close=workbench_on_close
                        aria_labelledby=if workbench_with_aria.get() {
                            "overlay-workbench-title".to_string()
                        } else {
                            String::new()
                        }
                        aria_describedby=if workbench_with_aria.get() {
                            "overlay-workbench-desc".to_string()
                        } else {
                            String::new()
                        }
                        role=workbench_role.get()
                        is_dismissable=workbench_dismissable.get()
                        is_keyboard_dismiss_disabled=workbench_keyboard_dismiss_disabled.get()
                        motion=workbench_motion.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-overlay-state".to_string()
                        } else {
                            String::new()
                        }
                        on_exit_complete=workbench_on_exit_complete
                    >
                        <div class="docs-stack">
                            <div id="overlay-workbench-title">"Workbench overlay"</div>
                            <div id="overlay-workbench-desc" class="ui-muted">
                                "Toggle dismiss, role, motion and aria contracts."
                            </div>
                            <Button variant=ButtonVariant::Secondary on_press=workbench_on_close>
                                "Close"
                            </Button>
                        </div>
                    </Overlay>
                </Show>
            </Playground>

            <Playground
                title="State Matrix (Default / Locked Alertdialog)"
                description="Inspect state/source markers under different role and dismiss policies."
                code_signal=marker_code
            >
                <div class="docs-row">
                    <Button on_press=open_marker>"Open marker overlay"</Button>
                    <span class="ui-muted">
                        "open: " {move || marker_open_raw.get().to_string()}
                    </span>
                </div>

                <Show when=move || marker_present.get()>
                    <Overlay
                        open=marker_open
                        on_close=close_marker
                        role="alertdialog"
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        motion=marker_motion
                        class_name="docs-overlay-state".to_string()
                        aria_labelledby="overlay-marker-title".to_string()
                        aria_describedby="overlay-marker-desc".to_string()
                        on_exit_complete=on_marker_exit_complete
                    >
                        <div class="docs-stack">
                            <div id="overlay-marker-title">"Marker overlay"</div>
                            <div id="overlay-marker-desc" class="ui-muted">
                                "Inspect data-dismiss-source / data-keyboard-dismiss-source / data-role-source in DevTools."
                            </div>
                            <Button variant=ButtonVariant::Secondary on_press=close_marker>
                                "Close"
                            </Button>
                        </div>
                    </Overlay>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
