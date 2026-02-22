use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OnPress, OverlayMotion,
    SegmentedControl, SegmentedControlSize, Switch,
};

const DIALOG_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OverlayMotion};";

pub(super) fn dialog() -> AnyView {
    let (hello_open_raw, set_hello_open_raw) = signal(false);
    let hello_open: Signal<bool> = Signal::derive(move || hello_open_raw.get());
    let open_hello_dialog: OnPress = Callback::new(move |_| set_hello_open_raw.set(true));
    let close_hello_dialog: OnPress = Callback::new(move |_| set_hello_open_raw.set(false));

    let hello_code = Signal::derive(move || {
        r#"<Dialog id_base="docs-dialog-hello".to_string() title="Hello dialog".to_string() default_open=true>
  <div>"Hello dialog body"</div>
</Dialog>"#
            .to_string()
    });

    let state_matrix_options = vec![
        "Uncontrolled + default_open=true".to_string(),
        "Uncontrolled + default_open=false + no close".to_string(),
        "Controlled + on_open_change".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(1_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(1) == 2);
    let state_matrix_default_open =
        Signal::derive(move || state_matrix_index.get().unwrap_or(1) == 0);
    let state_matrix_show_close =
        Signal::derive(move || state_matrix_index.get().unwrap_or(1) != 1);
    let state_matrix_with_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(1) != 1);
    let state_matrix_size = Signal::derive(move || match state_matrix_index.get().unwrap_or(1) {
        1 => DialogSize::Sm,
        2 => DialogSize::Lg,
        _ => DialogSize::Md,
    });
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));
    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(1);
        let mut lines = vec![
            "<Dialog".to_string(),
            "  id_base=\"docs-dialog-state-matrix\".to_string()".to_string(),
            "  title=\"State Matrix\".to_string()".to_string(),
        ];
        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  is_close_button_visible=true".to_string());
                lines.push("  size=DialogSize::Md".to_string());
            }
            1 => {
                lines.push("  default_open=false".to_string());
                lines.push("  is_close_button_visible=false".to_string());
                lines.push("  size=DialogSize::Sm".to_string());
            }
            _ => {
                lines.push("  is_open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  is_close_button_visible=true".to_string());
                lines.push("  size=DialogSize::Lg".to_string());
            }
        }
        lines.push(">".to_string());
        lines.push("  <div>\"State matrix body\"</div>".to_string());
        lines.push("</Dialog>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let (compare_controlled_close_count, set_compare_controlled_close_count) = signal(0_u32);
    let on_compare_controlled_close: OnPress =
        Callback::new(move |_| set_compare_controlled_close_count.update(|value| *value += 1));

    let (compare_uncontrolled_mounted, set_compare_uncontrolled_mounted) = signal(false);
    let mount_uncontrolled_dialog: OnPress =
        Callback::new(move |_| set_compare_uncontrolled_mounted.set(true));
    let unmount_uncontrolled_dialog: OnPress =
        Callback::new(move |_| set_compare_uncontrolled_mounted.set(false));
    let (compare_uncontrolled_close_count, set_compare_uncontrolled_close_count) = signal(0_u32);
    let set_compare_uncontrolled_mounted_on_close = set_compare_uncontrolled_mounted;
    let on_compare_uncontrolled_close: OnPress = Callback::new(move |_| {
        set_compare_uncontrolled_close_count.update(|value| *value += 1);
        set_compare_uncontrolled_mounted_on_close.set(false);
    });
    let compare_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<Dialog
  id_base="docs-dialog-compare-controlled".to_string()
  title="Controlled dialog".to_string()
  is_open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
>
  <div>"Controlled body"</div>
</Dialog>

<Dialog
  id_base="docs-dialog-compare-uncontrolled".to_string()
  title="Uncontrolled dialog".to_string()
  default_open=true
>
  <div>"Uncontrolled body"</div>
</Dialog>"#
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
    let stream_output_status_options = vec![
        "Verified".to_string(),
        "Draft".to_string(),
        "Commit Ready".to_string(),
    ];
    let (stream_output_status_index, set_stream_output_status_index) = signal(Some(0_usize));
    let stream_requested_output_status =
        Signal::derive(
            move || match stream_output_status_index.get().unwrap_or(0) {
                1 => "draft",
                2 => "commit-ready",
                _ => "verified",
            },
        );
    let (stream_open_raw, set_stream_open_raw) = signal(false);
    let stream_open: Signal<bool> = Signal::derive(move || stream_open_raw.get());
    let (stream_present, set_stream_present) = signal(stream_open.get_untracked());
    Effect::new(move |_| {
        if stream_open.get() {
            set_stream_present.set(true);
        }
    });
    let open_stream_dialog: OnPress = Callback::new(move |_| set_stream_open_raw.set(true));
    let close_stream_dialog: OnPress = Callback::new(move |_| set_stream_open_raw.set(false));
    let on_stream_exit_complete = Callback::new(move |_| set_stream_present.set(false));
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Dialog is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<Dialog
  id_base="docs-dialog-stream".to_string()
  title="Streaming optional contract".to_string()
  default_open=false
>
  <div>"Dialog content"</div>
</Dialog>"#
            .to_string()
    });

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
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let open_workbench_dialog: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_dialog: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let on_workbench_exit_complete = Callback::new(move |_| {
        set_workbench_present.set(false);
        set_workbench_exit_count.update(|count| *count += 1);
    });

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
            "/* components/dialog/src/styles.rs */\n{}",
            ui::dialog::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let with_description = workbench_with_description.get();
        let show_close = workbench_show_close.get();
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();
        let description = if with_description {
            Some("Toggle options to validate source markers and aria wiring.")
        } else {
            None
        };
        let class_name = if custom_class {
            Some("docs-dialog-workbench")
        } else {
            None
        };
        let motion = if custom_motion {
            "DialogMotion::custom"
        } else {
            "DialogMotion::default"
        };

        format!(
            "DialogWorkbenchConfig {{\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  on_close: \"count={}\",\n  id_base: \"docs-dialog-workbench\",\n  title: \"Workbench dialog\",\n  description: {description:?},\n  footer: \"Some(ViewFn)\",\n  size: {size:?},\n  is_close_button_visible: {show_close},\n  show_close_button: Some({show_close}),\n  close_label: \"Close\",\n  motion: {motion},\n  on_exit_complete: \"count={}\",\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  with_description: {with_description},\n  custom_motion: {custom_motion},\n  custom_class: {custom_class},\n}}",
            workbench_open_raw.get(),
            workbench_open_raw.get(),
            workbench_open_change_count.get(),
            workbench_close_count.get(),
            workbench_exit_count.get(),
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
            <Playground
                title="Hello World"
                description="最小可用默认路径：不需要手动接线 primitives/headless 状态机。"
                code_signal=hello_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_hello_dialog>"Open hello dialog"</Button>
                </div>

                <Show when=move || hello_open_raw.get()>
                    <Dialog
                        open=hello_open
                        on_close=close_hello_dialog
                        id_base="docs-dialog-hello".to_string()
                        title="Hello dialog".to_string()
                    >
                        <div>"Hello dialog body"</div>
                    </Dialog>
                </Show>
            </Playground>

            <Playground
                title="Dialog"
                code_signal=code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button attr:data-slot="dialog-e2e-open-default" on_press=open_dialog>
                        "Open dialog"
                    </Button>
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

            <Playground
                title="State Scenarios"
                description="受控/非受控、default_open 与 close-button 可见性的状态矩阵切换。"
                code_signal=state_matrix_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="dialog-state-matrix">
                    <SegmentedControl
                        id_base="docs-dialog-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Dialog state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_state_matrix_open_raw.set(true))
                        >
                            "Open controlled scenario"
                        </Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_state_matrix_open_raw.set(false))
                        >
                            "Close controlled scenario"
                        </Button>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <Dialog
                                    is_open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    id_base="docs-dialog-state-matrix".to_string()
                                    title="State Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Switch scenario to inspect controlled/uncontrolled and source markers."
                                            .to_string()
                                    } else {
                                        String::new()
                                    }
                                    size=state_matrix_size.get()
                                    is_close_button_visible=state_matrix_show_close.get()
                                >
                                    <div class="docs-stack">
                                        <div>"State matrix body"</div>
                                        <div class="ui-muted">
                                            "Inspect data-open-mode/data-open-source/data-close-source."
                                        </div>
                                    </div>
                                </Dialog>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Dialog
                                    default_open=state_matrix_default_open.get()
                                    id_base="docs-dialog-state-matrix".to_string()
                                    title="State Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Switch scenario to inspect controlled/uncontrolled and source markers."
                                            .to_string()
                                    } else {
                                        String::new()
                                    }
                                    size=state_matrix_size.get()
                                    is_close_button_visible=state_matrix_show_close.get()
                                >
                                    <div class="docs-stack">
                                        <div>"State matrix body"</div>
                                        <div class="ui-muted">
                                            "Inspect data-open-mode/data-open-source/data-close-source."
                                        </div>
                                    </div>
                                </Dialog>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "open_mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "default_open: "
                        {move || state_matrix_default_open.get()}
                    </span>
                    <span class="ui-muted">
                        "close_button_visible: "
                        {move || state_matrix_show_close.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                code_signal=marker_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button
                            attr:data-slot="dialog-e2e-open-marker"
                            on_press=open_dialog
                            variant=ButtonVariant::Secondary
                        >
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
                                <Button
                                    attr:data-slot="dialog-e2e-close-marker"
                                    variant=ButtonVariant::Secondary
                                    on_press=on_close
                                >
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Dialog>
                </Show>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="显式对照 value+on_change 受控路径与 default_open 非受控路径。"
                code_signal=compare_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="dialog-controlled-uncontrolled">
                    <div class="docs-row">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(true))
                        >
                            "Open controlled dialog"
                        </Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(false))
                        >
                            "Close controlled dialog"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=mount_uncontrolled_dialog>
                            "Mount uncontrolled dialog"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=unmount_uncontrolled_dialog>
                            "Unmount uncontrolled dialog"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <Show when=move || compare_controlled_open_raw.get()>
                                <Dialog
                                    is_open=compare_controlled_open
                                    on_open_change=on_compare_controlled_open_change
                                    on_close=on_compare_controlled_close
                                    id_base="docs-dialog-compare-controlled".to_string()
                                    title="Controlled dialog".to_string()
                                    description="open + on_open_change are driven by parent state."
                                >
                                    <div>"Controlled body"</div>
                                </Dialog>
                            </Show>
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                            <span class="ui-muted">
                                "close events: "
                                {move || compare_controlled_close_count.get()}
                            </span>
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <Show when=move || compare_uncontrolled_mounted.get()>
                                <Dialog
                                    id_base="docs-dialog-compare-uncontrolled".to_string()
                                    title="Uncontrolled dialog".to_string()
                                    description="default_open initializes once; subsequent transitions stay internal."
                                    default_open=true
                                    on_close=on_compare_uncontrolled_close
                                >
                                    <div>"Uncontrolled body"</div>
                                </Dialog>
                            </Show>
                            <span class="ui-muted">
                                "mounted: "
                                {move || if compare_uncontrolled_mounted.get() { "true" } else { "false" }}
                            </span>
                            <span class="ui-muted">
                                "close events: "
                                {move || compare_uncontrolled_close_count.get()}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Dialog 非正文阅读面：Streaming optional，fallback 固定 snapshot。"
                code_signal=streaming_snapshot_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="dialog-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=open_stream_dialog>
                            "Open stream contract dialog"
                        </Button>
                        <span class="ui-muted">"Streaming is optional; fallback stays snapshot."</span>
                    </div>
                    <div class="docs-row">
                        <SegmentedControl
                            id_base="docs-dialog-stream-mode".to_string()
                            options=stream_mode_options.clone()
                            selected_index=stream_mode_index
                            set_selected_index=set_stream_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="Dialog requested stream mode".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-dialog-stream-output-status".to_string()
                            options=stream_output_status_options.clone()
                            selected_index=stream_output_status_index
                            set_selected_index=set_stream_output_status_index
                            size=SegmentedControlSize::Sm
                            aria_label="Dialog requested output status".to_string()
                        />
                    </div>
                    <Show when=move || stream_present.get()>
                        <Dialog
                            is_open=stream_open
                            on_open_change=Callback::new(move |next: bool| {
                                set_stream_open_raw.set(next);
                            })
                            on_close=close_stream_dialog
                            id_base="docs-dialog-stream".to_string()
                            title="Streaming optional contract".to_string()
                            description="Effective markers should stay stream_mode=snapshot and stream_fallback=snapshot."
                            on_exit_complete=on_stream_exit_complete
                        >
                            <div class="docs-stack">
                                <div>"Dialog content"</div>
                                <div class="ui-muted">
                                    "Inspect data-stream-mode/data-stream-fallback/data-output-status."
                                </div>
                            </div>
                        </Dialog>
                    </Show>
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
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="components/dialog/src/styles.rs".to_string()
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
                <div class="docs-stack docs-stack--tight" attr:data-slot="dialog-workbench">
                    <div class="docs-row">
                        <Button attr:data-slot="dialog-e2e-open-workbench" on_press=open_workbench_dialog>
                            "Open workbench dialog"
                        </Button>
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
                        on_open_change=on_workbench_open_change
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
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button
                                    attr:data-slot="dialog-e2e-close-workbench"
                                    variant=ButtonVariant::Secondary
                                    on_press=close_workbench_dialog
                                >
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

            <Playground
                title="Scenario Comparison"
                code_signal=scenario_code
                code_imports=DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="dialog-scenario-compare">
                    <div class="docs-row">
                        <Button
                            attr:data-slot="dialog-e2e-open-compare-default"
                            variant=ButtonVariant::Secondary
                            on_press=open_default_scenario
                        >
                            "Open default comparison"
                        </Button>
                        <Button
                            attr:data-slot="dialog-e2e-open-compare-compact"
                            variant=ButtonVariant::Secondary
                            on_press=open_compact_scenario
                        >
                            "Open compact comparison"
                        </Button>
                        <Button
                            attr:data-slot="dialog-e2e-open-compare-motion"
                            variant=ButtonVariant::Secondary
                            on_press=open_motion_scenario
                        >
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
                                            <Button
                                                attr:data-slot="dialog-e2e-close-compare-default"
                                                variant=ButtonVariant::Secondary
                                                on_press=close_scenario_dialog
                                            >
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
                                            <Button
                                                attr:data-slot="dialog-e2e-close-compare-compact"
                                                variant=ButtonVariant::Secondary
                                                on_press=close_scenario_dialog
                                            >
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
                                            <Button
                                                attr:data-slot="dialog-e2e-close-compare-motion"
                                                variant=ButtonVariant::Secondary
                                                on_press=close_scenario_dialog
                                            >
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

            <div class="docs-stack docs-stack--tight" attr:data-slot="dialog-source-first">
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
                    <code>"DIALOG_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-dialog\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" attr:data-slot="dialog-source-paths">
                    <li><code>"components/dialog/src/mod.rs"</code></li>
                    <li><code>"components/dialog/src/logic.rs"</code></li>
                    <li><code>"components/dialog/src/view.rs"</code></li>
                    <li><code>"components/dialog/src/styles.rs"</code></li>
                    <li><code>"components/dialog/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
