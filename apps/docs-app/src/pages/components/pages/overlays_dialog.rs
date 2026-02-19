use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OnPress, OverlayMotion,
    SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn dialog() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_dialog: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<Dialog
  open=Signal::derive(move || open_raw.get())
  on_close=Callback::new(move |_| set_open_raw.set(false))
  id_base="d".to_string()
  title="Title".to_string()
>
  move || view! { ... }
</Dialog>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let on_exit_complete = Callback::new(move |_| {});

<Dialog
  open=Signal::derive(move || open_raw.get())
  on_close=Callback::new(move |_| set_open_raw.set(false))
  id_base="docs-dialog-marker".to_string()
  title="Marker dialog".to_string()
  description="Inspect source markers".to_string()
  size=DialogSize::Lg
  close_label="Dismiss dialog"
  class_name="docs-dialog-custom".to_string()
  motion=DialogMotion {
    overlay: OverlayMotion {
      initial_scale: 0.94,
      initial_y_px: 14.0,
      ..OverlayMotion::default()
    }
  }
  on_exit_complete=on_exit_complete
>
  ...
</Dialog>"#
            .to_string()
    });

    let size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => DialogSize::Sm,
        2 => DialogSize::Lg,
        _ => DialogSize::Md,
    });
    let (workbench_with_description, set_workbench_with_description) = signal(true);
    let (workbench_show_close, set_workbench_show_close) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let open_workbench_dialog: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_dialog: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_exit_complete = Callback::new(move |_| set_workbench_present.set(false));

    let workbench_code = Signal::derive(move || {
        let size_line = match workbench_size.get() {
            DialogSize::Sm => "  size=DialogSize::Sm\n",
            DialogSize::Lg => "  size=DialogSize::Lg\n",
            DialogSize::Md => "",
        };
        let description_line = if workbench_with_description.get() {
            "  description=\"Configurable description\".into()\n"
        } else {
            ""
        };
        let close_line = if !workbench_show_close.get() {
            "  show_close_button=false\n"
        } else {
            ""
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-dialog-workbench\".into()\n"
        } else {
            ""
        };
        let motion_line = if workbench_custom_motion.get() {
            "  motion=DialogMotion {\n    overlay: OverlayMotion {\n      initial_scale: 0.92,\n      initial_y_px: 20.0,\n      ..OverlayMotion::default()\n    }\n  }\n"
        } else {
            ""
        };

        format!(
            "let (open_raw, set_open_raw) = signal(false);\n\n<Dialog\n  open=Signal::derive(move || open_raw.get())\n  on_close=Callback::new(move |_| set_open_raw.set(false))\n  id_base=\"docs-dialog-workbench\".into()\n  title=\"Workbench dialog\".into()\n{size_line}{description_line}{close_line}{class_line}{motion_line}>\n  ...\n</Dialog>"
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/dialog/styles.rs */\n{}",
            ui_components::dialog::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let with_description = workbench_with_description.get();
        let show_close = workbench_show_close.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();

        format!(
            "DialogWorkbenchConfig {{\n  size: {size:?},\n  with_description: {with_description},\n  show_close_button: {show_close},\n  custom_motion: {custom_motion},\n  custom_class: {custom_class},\n}}"
        )
    });

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum DialogScenario {
        Default,
        Compact,
        Motion,
    }

    let (scenario_open_raw, set_scenario_open_raw) = signal(false);
    let scenario_open: Signal<bool> = Signal::derive(move || scenario_open_raw.get());
    let (scenario_present, set_scenario_present) = signal(scenario_open.get_untracked());
    Effect::new(move |_| {
        if scenario_open.get() {
            set_scenario_present.set(true);
        }
    });
    let (scenario_kind, set_scenario_kind) = signal(DialogScenario::Default);
    let open_default_scenario: OnPress = Callback::new(move |_| {
        set_scenario_kind.set(DialogScenario::Default);
        set_scenario_open_raw.set(true);
    });
    let open_compact_scenario: OnPress = Callback::new(move |_| {
        set_scenario_kind.set(DialogScenario::Compact);
        set_scenario_open_raw.set(true);
    });
    let open_motion_scenario: OnPress = Callback::new(move |_| {
        set_scenario_kind.set(DialogScenario::Motion);
        set_scenario_open_raw.set(true);
    });
    let close_scenario_dialog: OnPress = Callback::new(move |_| set_scenario_open_raw.set(false));
    let on_scenario_exit_complete = Callback::new(move |_| set_scenario_present.set(false));

    let scenario_code = Signal::derive(move || {
        r#"<Dialog title="Default comparison".to_string() />
<Dialog
  title="Title-only compact".to_string()
  show_close_button=false
  size=DialogSize::Sm
/>
<Dialog
  title="Custom motion".to_string()
  size=DialogSize::Lg
  motion=DialogMotion {
    overlay: OverlayMotion {
      initial_scale: 0.9,
      initial_y_px: 22.0,
      ..OverlayMotion::default()
    }
  }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Dialog"
            slug="dialog"
            group="Overlays"
            description="Dialog panel with header/body/footer structure on top of Overlay."
        >
            <Playground title="Dialog" code_signal=code>
                <div class="docs-row">
                    <Button on_press=open_dialog>"Open dialog"</Button>
                </div>

                <Show when=move || present.get()>
                    <Dialog
                        open=open
                        on_close=on_close
                        id_base="docs-dialog".to_string()
                        title="Dialog title".to_string()
                        description="Uses Overlay + header/body/footer layout.".to_string()
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Cancel"</Button>
                                <Button on_press=on_close>"Confirm"</Button>
                            </div>
                        }
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Dialog body"</div>
                            <div class="ui-muted">"Esc/backdrop closes, focus is trapped."</div>
                        </div>
                    </Dialog>
                </Show>
            </Playground>

            <Playground title="State + Source Markers" code_signal=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_dialog variant=ButtonVariant::Secondary>
                            "Open marker dialog"
                        </Button>
                        <span class="ui-muted">"open: " {move || open_raw.get()}</span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-title-source / data-description-source / data-close-source / data-motion-source in DevTools."
                    </div>
                </div>

                <Show when=move || present.get()>
                    <Dialog
                        open=open
                        on_close=on_close
                        id_base="docs-dialog-marker".to_string()
                        title="Marker dialog".to_string()
                        description="Custom size, class, close label, and motion for contract inspection."
                        size=DialogSize::Lg
                        close_label="Dismiss dialog"
                        class_name="docs-dialog-custom".to_string()
                        motion=DialogMotion {
                            overlay: OverlayMotion {
                                initial_scale: 0.94,
                                initial_y_px: 14.0,
                                ..OverlayMotion::default()
                            },
                        }
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Inspect root and part data-* markers."</div>
                            <div class="ui-muted">
                                "Includes size/id/title/description/close/class/motion source contracts."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Dialog>
                </Show>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-components/src/dialog/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-dialog-workbench-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Dialog workbench size".to_string()
                        />
                        <Switch
                            checked=workbench_with_description
                            set_checked=set_workbench_with_description
                        >
                            "With description"
                        </Switch>
                        <Switch checked=workbench_show_close set_checked=set_workbench_show_close>
                            "Show close button"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="dialog-workbench">
                    <div class="docs-row">
                        <Button on_press=open_workbench_dialog>"Open workbench dialog"</Button>
                        <span class="ui-muted">
                            "size: "
                            {move || workbench_size.get().as_attr()}
                            " / description: "
                            {move || if workbench_with_description.get() { "on" } else { "off" }}
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Use Config panel to compare close-button, motion, and class-source behaviors."
                    </div>
                </div>

                <Show when=move || workbench_present.get()>
                    <Dialog
                        open=workbench_open
                        on_close=close_workbench_dialog
                        id_base="docs-dialog-workbench".to_string()
                        title="Workbench dialog".to_string()
                        size=workbench_size.get()
                        show_close_button=workbench_show_close.get()
                        description=if workbench_with_description.get() {
                            "Toggle options to validate source markers and aria wiring.".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-dialog-workbench".to_string()
                        } else {
                            String::new()
                        }
                        motion=if workbench_custom_motion.get() {
                            DialogMotion {
                                overlay: OverlayMotion {
                                    initial_scale: 0.92,
                                    initial_y_px: 20.0,
                                    ..OverlayMotion::default()
                                },
                            }
                        } else {
                            DialogMotion::default()
                        }
                        on_exit_complete=on_workbench_exit_complete
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_workbench_dialog>
                                    "Cancel"
                                </Button>
                                <Button on_press=close_workbench_dialog>"Confirm"</Button>
                            </div>
                        }
                    >
                        <div class="docs-stack">
                            <div>"This dialog is controlled by the workbench config panel."</div>
                            <div class="ui-muted">
                                "Open test panel to live-edit scoped CSS and inspect actual config."
                            </div>
                        </div>
                    </Dialog>
                </Show>
            </Playground>

            <Playground title="Scenario Comparison" code_signal=scenario_code>
                <div class="docs-stack docs-stack--tight" data-slot="dialog-scenario-compare">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=open_default_scenario>
                            "Open default comparison"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=open_compact_scenario>
                            "Open compact comparison"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=open_motion_scenario>
                            "Open motion comparison"
                        </Button>
                    </div>
                    <div class="ui-muted">
                        "Compare default, title-only compact, and custom-motion states."
                    </div>
                </div>

                <Show when=move || scenario_present.get()>
                    {move || match scenario_kind.get() {
                        DialogScenario::Default => {
                            view! {
                                <Dialog
                                    open=scenario_open
                                    on_close=close_scenario_dialog
                                    id_base="docs-dialog-compare-default".to_string()
                                    title="Default comparison".to_string()
                                    description="Default size + description + close button.".to_string()
                                    on_exit_complete=on_scenario_exit_complete
                                >
                                    <div class="docs-stack">
                                        <div>"Default state contract."</div>
                                        <div class="docs-row docs-row--end">
                                            <Button variant=ButtonVariant::Secondary on_press=close_scenario_dialog>
                                                "Close"
                                            </Button>
                                        </div>
                                    </div>
                                </Dialog>
                            }
                                .into_any()
                        }
                        DialogScenario::Compact => {
                            view! {
                                <Dialog
                                    open=scenario_open
                                    on_close=close_scenario_dialog
                                    id_base="docs-dialog-compare-compact".to_string()
                                    title="Title-only compact".to_string()
                                    size=DialogSize::Sm
                                    show_close_button=false
                                    on_exit_complete=on_scenario_exit_complete
                                >
                                    <div class="docs-stack">
                                        <div>"Compact: no description, no close icon."</div>
                                        <div class="docs-row docs-row--end">
                                            <Button variant=ButtonVariant::Secondary on_press=close_scenario_dialog>
                                                "Dismiss"
                                            </Button>
                                        </div>
                                    </div>
                                </Dialog>
                            }
                                .into_any()
                        }
                        DialogScenario::Motion => {
                            view! {
                                <Dialog
                                    open=scenario_open
                                    on_close=close_scenario_dialog
                                    id_base="docs-dialog-compare-motion".to_string()
                                    title="Custom motion".to_string()
                                    description="Large dialog with custom overlay motion.".to_string()
                                    size=DialogSize::Lg
                                    close_label="Dismiss dialog"
                                    class_name="docs-dialog-custom".to_string()
                                    motion=DialogMotion {
                                        overlay: OverlayMotion {
                                            initial_scale: 0.9,
                                            initial_y_px: 22.0,
                                            ..OverlayMotion::default()
                                        },
                                    }
                                    on_exit_complete=on_scenario_exit_complete
                                >
                                    <div class="docs-stack">
                                        <div>"Custom motion source marker should be `custom`."</div>
                                        <div class="docs-row docs-row--end">
                                            <Button variant=ButtonVariant::Secondary on_press=close_scenario_dialog>
                                                "Close"
                                            </Button>
                                        </div>
                                    </div>
                                </Dialog>
                            }
                                .into_any()
                        }
                    }}
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
