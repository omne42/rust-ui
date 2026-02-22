use crate::pages::components::ComponentPage;
use crate::pages::components::pages::playground_workbench::{
    bool_word, push_line_when, rust_string_literal,
};
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    AlertDialog, AlertDialogVariant, Button, ButtonVariant, OnPress, SegmentedControl,
    SegmentedControlSize, Snippet,
};

const ALERT_DIALOG_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{AlertDialog, AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogVariant, Button, ButtonVariant, OnPress, OverlayMotion, SegmentedControl, SegmentedControlSize};";

pub(super) fn alert_dialog() -> AnyView {
    let (hello_open_raw, set_hello_open_raw) = signal(false);
    let hello_open: Signal<bool> = Signal::derive(move || hello_open_raw.get());
    let (hello_present, set_hello_present) = signal(hello_open.get_untracked());
    Effect::new(move |_| {
        if hello_open.get() {
            set_hello_present.set(true);
        }
    });

    let close_hello_alert: OnPress = Callback::new(move |_| set_hello_open_raw.set(false));
    let open_hello_alert: OnPress = Callback::new(move |_| set_hello_open_raw.set(true));
    let on_hello_exit_complete = Callback::new(move |_| set_hello_present.set(false));

    let (confirmed, set_confirmed) = signal(0u32);
    let on_hello_confirm: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(1));
    });

    let hello_world_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let close_alert: OnPress = Callback::new(move |_| set_open_raw.set(false));

<AlertDialog
  open=open
  id_base="docs-alert".to_string()
  title="Delete item?".to_string()
  description="Uses role=alertdialog with Overlay semantics.".to_string()
  on_close=close_alert
  confirm_label="Delete".to_string()
  on_confirm=Callback::new(move |_| {})
  variant=AlertDialogVariant::Destructive
/>"#
        .to_string()
    });

    let (marker_open_raw, set_marker_open_raw) = signal(false);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let (marker_present, set_marker_present) = signal(marker_open.get_untracked());
    Effect::new(move |_| {
        if marker_open.get() {
            set_marker_present.set(true);
        }
    });

    let close_marker_alert: OnPress = Callback::new(move |_| set_marker_open_raw.set(false));
    let open_marker_alert: OnPress = Callback::new(move |_| set_marker_open_raw.set(true));
    let on_marker_exit_complete = Callback::new(move |_| set_marker_present.set(false));

    let on_marker_confirm: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(1));
    });

    let on_marker_secondary: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(10));
    });
    let on_matrix_secondary: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(10));
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);

<AlertDialog
  open=Signal::derive(move || open_raw.get())
  id_base="a-marker".to_string()
  title="Delete workspace?".to_string()
  description="Inspect source markers.".to_string()
  on_close=Callback::new(move |_| set_open_raw.set(false))
  confirm_label="Delete now".to_string()
  cancel_label="Dismiss".to_string()
  secondary_label="Save draft".to_string()
  on_confirm=Callback::new(move |_| {})
  on_secondary=Callback::new(move |_| {})
  auto_focus_button=ui::AlertDialogAutoFocusButton::Secondary
  secondary_disabled=true
  motion=ui::AlertDialogMotion {
    overlay: ui::OverlayMotion {
      initial_scale: 0.95,
      initial_y_px: 12.0,
      ..ui::OverlayMotion::default()
    }
  }
/>"#
        .to_string()
    });

    let state_matrix_options = vec![
        "Destructive".to_string(),
        "Warning + Secondary Disabled".to_string(),
        "Error + Confirm Disabled".to_string(),
    ];
    let state_matrix_options_for_primary = state_matrix_options.clone();
    let state_matrix_options_for_after = state_matrix_options.clone();
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_warning =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 1);
    let state_matrix_is_error = Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);

    let (matrix_open_raw, set_matrix_open_raw) = signal(false);
    let matrix_open: Signal<bool> = Signal::derive(move || matrix_open_raw.get());
    let (matrix_present, set_matrix_present) = signal(matrix_open.get_untracked());
    Effect::new(move |_| {
        if matrix_open.get() {
            set_matrix_present.set(true);
        }
    });
    let open_matrix_alert: OnPress = Callback::new(move |_| set_matrix_open_raw.set(true));
    let close_matrix_alert: OnPress = Callback::new(move |_| set_matrix_open_raw.set(false));
    let on_matrix_exit_complete = Callback::new(move |_| set_matrix_present.set(false));
    let on_matrix_confirm: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(100));
    });

    let state_matrix_code = Signal::derive(move || {
        r#"let state_options = vec![
  "Destructive".to_string(),
  "Warning + Secondary Disabled".to_string(),
  "Error + Confirm Disabled".to_string(),
];

<SegmentedControl ... />
<AlertDialog
  open=open
  id_base="docs-alert-matrix".to_string()
  title="State Matrix".to_string()
  on_close=on_close
  confirm_label="Delete".to_string()
  on_confirm=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let (controlled_present, set_controlled_present) = signal(controlled_open.get_untracked());
    Effect::new(move |_| {
        if controlled_open.get() {
            set_controlled_present.set(true);
        }
    });
    let open_controlled_alert: OnPress = Callback::new(move |_| set_controlled_open_raw.set(true));
    let close_controlled_alert: OnPress =
        Callback::new(move |_| set_controlled_open_raw.set(false));
    let on_controlled_exit_complete = Callback::new(move |_| set_controlled_present.set(false));

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"// AlertDialog is intentionally controlled at the component boundary.
// Uncontrolled open-state belongs to upstream primitives/adapters.
<AlertDialog
  open=open
  id_base="docs-alert-controlled".to_string()
  title="Controlled".to_string()
  on_close=on_close
  confirm_label="Acknowledge".to_string()
  on_confirm=Callback::new(move |_| {})
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

    let (stream_open_raw, set_stream_open_raw) = signal(false);
    let stream_open: Signal<bool> = Signal::derive(move || stream_open_raw.get());
    let (stream_present, set_stream_present) = signal(stream_open.get_untracked());
    Effect::new(move |_| {
        if stream_open.get() {
            set_stream_present.set(true);
        }
    });
    let open_stream_alert: OnPress = Callback::new(move |_| set_stream_open_raw.set(true));
    let close_stream_alert: OnPress = Callback::new(move |_| set_stream_open_raw.set(false));
    let on_stream_exit_complete = Callback::new(move |_| set_stream_present.set(false));
    let on_controlled_confirm: OnPress = Callback::new(move |_| {});
    let on_stream_confirm: OnPress = Callback::new(move |_| {});

    let streaming_snapshot_code = Signal::derive(move || {
        r#"// AlertDialog is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<AlertDialog
  open=open
  id_base="docs-alert-stream".to_string()
  title="Streaming Optional Contract".to_string()
  description="Component output remains snapshot-compatible and machine-readable.".to_string()
  on_close=on_close
  confirm_label="Continue".to_string()
  on_confirm=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    let workbench_variant_options = vec![
        "Destructive".to_string(),
        "Warning".to_string(),
        "Error".to_string(),
    ];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => AlertDialogVariant::Warning,
            2 => AlertDialogVariant::Error,
            _ => AlertDialogVariant::Destructive,
        });

    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_secondary, set_workbench_show_secondary) = signal(false);
    let (workbench_confirm_disabled, set_workbench_confirm_disabled) = signal(false);
    let (workbench_secondary_disabled, set_workbench_secondary_disabled) = signal(false);
    let (workbench_auto_focus_secondary, set_workbench_auto_focus_secondary) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let open_workbench_alert: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_alert: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_exit_complete = Callback::new(move |_| set_workbench_present.set(false));

    let (workbench_confirm_count, set_workbench_confirm_count) = signal(0_u32);
    let on_workbench_confirm: OnPress = Callback::new(move |_| {
        set_workbench_confirm_count.update(|value| *value = value.saturating_add(1));
    });
    let on_workbench_secondary: OnPress = Callback::new(move |_| {
        set_workbench_confirm_count.update(|value| *value = value.saturating_add(10));
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::AlertDialogMotion {
                overlay: ui::OverlayMotion {
                    initial_scale: 0.92,
                    initial_y_px: 10.0,
                    ..ui::OverlayMotion::default()
                },
            }
        } else {
            ui::AlertDialogMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let variant = workbench_variant.get();
        let show_description = workbench_show_description.get();
        let show_secondary = workbench_show_secondary.get();
        let confirm_disabled = workbench_confirm_disabled.get();
        let secondary_disabled = workbench_secondary_disabled.get();
        let auto_focus_secondary = workbench_auto_focus_secondary.get();
        let custom_motion = workbench_custom_motion.get();

        let title = match variant {
            AlertDialogVariant::Warning => "Warning review required",
            AlertDialogVariant::Error => "Error acknowledgement required",
            _ => "Delete item?",
        };
        let description = match variant {
            AlertDialogVariant::Warning => {
                "Potentially destructive operation. Confirm after review."
            }
            AlertDialogVariant::Error => {
                "Critical issue detected. Confirm remains disabled for safe acknowledgment."
            }
            _ => "This operation cannot be undone.",
        };

        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<AlertDialog".to_string(),
            "  open=open".to_string(),
            "  id_base=\"docs-alert-workbench\".to_string()".to_string(),
            format!("  title={}.to_string()", rust_string_literal(title)),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  on_cancel=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  confirm_label=\"Confirm\".to_string()".to_string(),
            "  on_confirm=Callback::new(move |_| {})".to_string(),
            format!("  variant=AlertDialogVariant::{variant:?}"),
            "  lang=Some(\"en\".to_string())".to_string(),
            "  dir=Some(A11yDirection::Ltr)".to_string(),
        ];
        push_line_when(
            &mut lines,
            show_description,
            format!(
                "  description={}.to_string()",
                rust_string_literal(description)
            ),
        );
        push_line_when(
            &mut lines,
            show_secondary,
            "  secondary_label=\"Save draft\".to_string()".to_string(),
        );
        push_line_when(
            &mut lines,
            show_secondary,
            "  on_secondary=Callback::new(move |_| {})".to_string(),
        );
        push_line_when(
            &mut lines,
            confirm_disabled,
            "  confirm_disabled=true".to_string(),
        );
        push_line_when(
            &mut lines,
            confirm_disabled,
            "  is_confirm_disabled=Some(true)".to_string(),
        );
        push_line_when(
            &mut lines,
            show_secondary && secondary_disabled,
            "  secondary_disabled=true".to_string(),
        );
        push_line_when(
            &mut lines,
            show_secondary && secondary_disabled,
            "  is_secondary_disabled=Some(true)".to_string(),
        );
        push_line_when(
            &mut lines,
            auto_focus_secondary,
            "  auto_focus_button=ui::AlertDialogAutoFocusButton::Secondary".to_string(),
        );
        push_line_when(
            &mut lines,
            !auto_focus_secondary,
            "  auto_focus_button=ui::AlertDialogAutoFocusButton::Confirm".to_string(),
        );
        push_line_when(
            &mut lines,
            custom_motion,
            "  motion=ui::AlertDialogMotion { overlay: ui::OverlayMotion { initial_scale: 0.92, initial_y_px: 10.0, ..ui::OverlayMotion::default() } }".to_string(),
        );
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let auto_focus_button = if workbench_auto_focus_secondary.get() {
            "Secondary"
        } else {
            "Confirm"
        };
        let show_secondary = workbench_show_secondary.get();
        let secondary_disabled = show_secondary && workbench_secondary_disabled.get();

        format!(
            "AlertDialogWorkbenchConfig {{\n  open: Some(Signal<bool>({})),\n  id_base: \"docs-alert-workbench\",\n  title: {:?},\n  on_close: Some(\"OnPress\"),\n  confirm_label: Some(\"Confirm\"),\n  on_confirm: Some(\"OnPress\"),\n  description: {},\n  cancel_label: Some(\"Cancel\"),\n  secondary_label: {},\n  on_secondary: {},\n  on_cancel: Some(\"OnPress\"),\n  is_confirm_disabled: Some({}),\n  confirm_disabled: Some({}),\n  is_secondary_disabled: {},\n  secondary_disabled: {},\n  auto_focus_button: \"{}\",\n  variant: {:?},\n  motion: {},\n  on_exit_complete: Some(\"Callback<()>\"),\n  lang: Some(\"en\"),\n  dir: Some(\"ltr\"),\n  show_description: {},\n  show_secondary: {},\n  confirm_count: {},\n}}",
            bool_word(workbench_open_raw.get()),
            match workbench_variant.get() {
                AlertDialogVariant::Warning => "Warning review required",
                AlertDialogVariant::Error => "Error acknowledgement required",
                AlertDialogVariant::Destructive => "Delete item?",
                _ => "Confirm action?",
            },
            if workbench_show_description.get() {
                "Some(\"Variant-specific description\")"
            } else {
                "None"
            },
            if show_secondary {
                "Some(\"Save draft\")"
            } else {
                "None"
            },
            if show_secondary {
                "Some(\"OnPress\")"
            } else {
                "None"
            },
            bool_word(workbench_confirm_disabled.get()),
            bool_word(workbench_confirm_disabled.get()),
            if show_secondary {
                format!("Some({})", bool_word(secondary_disabled))
            } else {
                "None".to_string()
            },
            if show_secondary {
                format!("Some({})", bool_word(secondary_disabled))
            } else {
                "None".to_string()
            },
            auto_focus_button,
            workbench_variant.get(),
            if workbench_custom_motion.get() {
                "AlertDialogMotion::custom"
            } else {
                "AlertDialogMotion::default"
            },
            bool_word(workbench_show_description.get()),
            bool_word(show_secondary),
            workbench_confirm_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="AlertDialog"
            slug="alert-dialog"
            group="Overlays"
            description="Alertdialog role composition with destructive/default variants."
        >
            <Playground
                title="Hello World (Minimal Path)"
                description="Default path: one controlled open signal plus destructive intent."
                code_signal=hello_world_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="alert-dialog-e2e-hello-controls">
                    <span data-slot="alert-dialog-e2e-open-destructive">
                        <Button variant=ButtonVariant::Destructive on_press=open_hello_alert>
                            "Open destructive"
                        </Button>
                    </span>
                    <span class="ui-muted">
                        "confirmed: " {move || confirmed.get()}
                    </span>
                </div>

                <Show when=move || hello_present.get()>
                    <AlertDialog
                        open=hello_open
                        id_base="docs-alert".to_string()
                        title="Delete item?".to_string()
                        description="Uses role=alertdialog with Overlay semantics.".to_string()
                        on_close=close_hello_alert
                        confirm_label="Delete".to_string()
                        on_confirm=on_hello_confirm
                        variant=AlertDialogVariant::Destructive
                        on_exit_complete=on_hello_exit_complete
                    />
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                code_signal=marker_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-e2e-marker-controls">
                    <div class="docs-row">
                        <span data-slot="alert-dialog-e2e-open-marker">
                            <Button variant=ButtonVariant::Secondary on_press=open_marker_alert>
                                "Open marker alert"
                            </Button>
                        </span>
                        <span class="ui-muted">
                            "confirmed: " {move || confirmed.get()}
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-title-source / data-description-source / data-cancel-source / data-secondary-source / data-motion-source in DevTools."
                    </div>
                </div>

                <Show when=move || marker_present.get()>
                    <AlertDialog
                        open=marker_open
                        id_base="docs-alert-marker".to_string()
                        title="Delete workspace?".to_string()
                        description="Custom labels, secondary action, autofocus, and motion markers."
                        on_close=close_marker_alert
                        confirm_label="Delete now".to_string()
                        cancel_label="Dismiss".to_string()
                        secondary_label="Save draft".to_string()
                        on_confirm=on_marker_confirm
                        on_secondary=on_marker_secondary
                        variant=AlertDialogVariant::Warning
                        auto_focus_button=ui::AlertDialogAutoFocusButton::Secondary
                        secondary_disabled=true
                        motion=ui::AlertDialogMotion {
                            overlay: ui::OverlayMotion {
                                initial_scale: 0.95,
                                initial_y_px: 12.0,
                                ..ui::OverlayMotion::default()
                            },
                        }
                        on_exit_complete=on_marker_exit_complete
                    />
                </Show>
            </Playground>

            <Playground
                title="State Scenarios"
                description="Matrix for variant/disabled combinations under the same semantic contract."
                code_signal=state_matrix_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-state-matrix">
                    <SegmentedControl
                        id_base="docs-alert-dialog-matrix".to_string()
                        options=state_matrix_options_for_primary.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="AlertDialog state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_matrix_alert>"Open matrix alert"</Button>
                        <span class="ui-muted">"confirmed: " {move || confirmed.get()}</span>
                    </div>

                    <Show when=move || matrix_present.get()>
                        {move || {
                            let variant = if state_matrix_is_error.get() {
                                AlertDialogVariant::Error
                            } else if state_matrix_is_warning.get() {
                                AlertDialogVariant::Warning
                            } else {
                                AlertDialogVariant::Destructive
                            };
                            let title = if state_matrix_is_error.get() {
                                "Error path"
                            } else if state_matrix_is_warning.get() {
                                "Warning path"
                            } else {
                                "Destructive path"
                            };
                            let description = if state_matrix_is_error.get() {
                                "Confirm action is disabled to expose error-state marker behavior."
                            } else if state_matrix_is_warning.get() {
                                "Secondary action stays visible but disabled in warning matrix branch."
                            } else {
                                "Default destructive branch with minimal surface."
                            };
                            let title: String = title.into();
                            let description: String = description.into();
                            let secondary_label = if state_matrix_is_warning.get() {
                                "Save draft".to_string()
                            } else {
                                String::new()
                            };
                            let confirm_label = if state_matrix_is_error.get() {
                                "Acknowledge".to_string()
                            } else {
                                "Delete".to_string()
                            };

                            view! {
                                    <AlertDialog
                                        open=matrix_open
                                        id_base="docs-alert-matrix".to_string()
                                        title=title
                                        description=description
                                        on_close=close_matrix_alert
                                    confirm_label=confirm_label
                                    on_confirm=on_matrix_confirm
                                    secondary_label=secondary_label
                                    on_secondary=on_matrix_secondary
                                    secondary_disabled=state_matrix_is_warning.get()
                                    confirm_disabled=state_matrix_is_error.get()
                                    variant=variant
                                    on_exit_complete=on_matrix_exit_complete
                                />
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="AlertDialog boundary is controlled-only; uncontrolled open state is adapter-level N/A."
                code_signal=controlled_vs_uncontrolled_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="alert-dialog-controlled-uncontrolled"
                >
                    <div class="docs-row">
                        <Button on_press=open_controlled_alert>"Open controlled alert"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_controlled_alert>
                            "Close controlled alert"
                        </Button>
                    </div>

                    <Show when=move || controlled_present.get()>
                        <AlertDialog
                            open=controlled_open
                            id_base="docs-alert-controlled".to_string()
                            title="Controlled".to_string()
                            description="`open: Signal<bool>` is the single source of truth at component boundary.".to_string()
                            on_close=close_controlled_alert
                            confirm_label="Acknowledge".to_string()
                            on_confirm=on_controlled_confirm
                            on_exit_complete=on_controlled_exit_complete
                        />
                    </Show>

                    <div class="docs-stack docs-stack--tight">
                        <strong>"Uncontrolled (N/A for AlertDialog)"</strong>
                        <span class="ui-muted">
                            "`AlertDialog` requires `open: Signal<bool>` + `on_close`; uncontrolled behavior should be adapted upstream via primitives."
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="AlertDialog is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="alert-dialog-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-alert-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="AlertDialog stream mode".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_stream_alert>"Open streaming contract alert"</Button>
                    </div>
                    <Show when=move || stream_present.get()>
                        <AlertDialog
                            open=stream_open
                            id_base="docs-alert-stream".to_string()
                            title="Streaming Optional Contract".to_string()
                            description="Component output remains snapshot-compatible and machine-readable.".to_string()
                            on_close=close_stream_alert
                            confirm_label="Continue".to_string()
                            on_confirm=on_stream_confirm
                            variant=AlertDialogVariant::Warning
                            on_exit_complete=on_stream_exit_complete
                        />
                    </Show>
                    <span class="ui-muted">"requested mode: " {move || stream_requested_mode.get()}</span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted" data-slot="alert-dialog-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                description="Button-style workbench: adjust variant/state props and inspect generated config + copy-ready code."
                code_signal=workbench_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-alert-dialog-workbench-variant".to_string()
                            options=workbench_variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="AlertDialog workbench variant".to_string()
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
                                prop:checked=move || workbench_show_secondary.get()
                                on:change=move |ev| set_workbench_show_secondary.set(event_target_checked(&ev))
                            />
                            " Enable secondary action"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_confirm_disabled.get()
                                on:change=move |ev| set_workbench_confirm_disabled.set(event_target_checked(&ev))
                            />
                            " Disable confirm"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_secondary_disabled.get()
                                on:change=move |ev| set_workbench_secondary_disabled.set(event_target_checked(&ev))
                            />
                            " Disable secondary"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_auto_focus_secondary.get()
                                on:change=move |ev| set_workbench_auto_focus_secondary.set(event_target_checked(&ev))
                            />
                            " Auto-focus secondary"
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
                <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-workbench">
                    <div class="docs-row">
                        <Button attr:data-slot="alert-dialog-workbench-open" on_press=open_workbench_alert>
                            "Open interactive alert"
                        </Button>
                        <Button
                            attr:data-slot="alert-dialog-workbench-close"
                            variant=ButtonVariant::Secondary
                            on_press=close_workbench_alert
                        >
                            "Close"
                        </Button>
                        <span class="ui-muted">"confirm counter: " {move || workbench_confirm_count.get()}</span>
                    </div>
                    <Show when=move || workbench_present.get()>
                        {move || {
                            let variant = workbench_variant.get();
                            let description = if workbench_show_description.get() {
                                match variant {
                                    AlertDialogVariant::Warning => "Potentially destructive operation. Confirm after review.",
                                    AlertDialogVariant::Error => "Critical issue detected. Confirm remains disabled for safe acknowledgment.",
                                    _ => "This operation cannot be undone.",
                                }
                                .to_string()
                            } else {
                                String::new()
                            };
                            let secondary_label = if workbench_show_secondary.get() {
                                "Save draft".to_string()
                            } else {
                                String::new()
                            };
                            let auto_focus_button = if workbench_auto_focus_secondary.get() {
                                ui::AlertDialogAutoFocusButton::Secondary
                            } else {
                                ui::AlertDialogAutoFocusButton::Confirm
                            };
                            let motion = workbench_motion.get();

                            view! {
                                <AlertDialog
                                    open=workbench_open
                                    id_base="docs-alert-workbench".to_string()
                                    title=match variant {
                                        AlertDialogVariant::Warning => "Warning review required".to_string(),
                                        AlertDialogVariant::Error => "Error acknowledgement required".to_string(),
                                        _ => "Delete item?".to_string(),
                                    }
                                    description=description
                                    on_close=close_workbench_alert
                                    confirm_label="Confirm".to_string()
                                    on_confirm=on_workbench_confirm
                                    secondary_label=secondary_label
                                    on_secondary=on_workbench_secondary
                                    variant=variant
                                    confirm_disabled=workbench_confirm_disabled.get()
                                    secondary_disabled=workbench_show_secondary.get()
                                        && workbench_secondary_disabled.get()
                                    auto_focus_button=auto_focus_button
                                    motion=motion
                                    on_exit_complete=on_workbench_exit_complete
                                />
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Destructive / Warning / Error)"
                description="Workbench 后的多参数状态对比。"
                code_signal=state_matrix_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-state-matrix-after-workbench">
                    <SegmentedControl
                        id_base="docs-alert-dialog-matrix-after-workbench".to_string()
                        options=state_matrix_options_for_after.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="AlertDialog state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_matrix_alert>"Open matrix alert"</Button>
                        <span class="ui-muted">"confirmed: " {move || confirmed.get()}</span>
                    </div>

                    <Show when=move || matrix_present.get()>
                        {move || {
                            let variant = if state_matrix_is_error.get() {
                                AlertDialogVariant::Error
                            } else if state_matrix_is_warning.get() {
                                AlertDialogVariant::Warning
                            } else {
                                AlertDialogVariant::Destructive
                            };
                            let title = if state_matrix_is_error.get() {
                                "Error path"
                            } else if state_matrix_is_warning.get() {
                                "Warning path"
                            } else {
                                "Destructive path"
                            };
                            let description = if state_matrix_is_error.get() {
                                "Confirm action is disabled to expose error-state marker behavior."
                            } else if state_matrix_is_warning.get() {
                                "Secondary action stays visible but disabled in warning matrix branch."
                            } else {
                                "Default destructive branch with minimal surface."
                            };
                            let title: String = title.into();
                            let description: String = description.into();
                            let secondary_label = if state_matrix_is_warning.get() {
                                "Save draft".to_string()
                            } else {
                                String::new()
                            };
                            let confirm_label = if state_matrix_is_error.get() {
                                "Acknowledge".to_string()
                            } else {
                                "Delete".to_string()
                            };

                            view! {
                                <AlertDialog
                                    open=matrix_open
                                    id_base="docs-alert-matrix-after-workbench".to_string()
                                    title=title
                                    description=description
                                    on_close=close_matrix_alert
                                    confirm_label=confirm_label
                                    on_confirm=on_matrix_confirm
                                    secondary_label=secondary_label
                                    on_secondary=on_matrix_secondary
                                    secondary_disabled=state_matrix_is_warning.get()
                                    confirm_disabled=state_matrix_is_error.get()
                                    variant=variant
                                    on_exit_complete=on_matrix_exit_complete
                                />
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="alert-dialog-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p>
                    "Imports are auto-completed via "
                    <code>"ALERT_DIALOG_DOC_IMPORTS"</code>
                    " + "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-alert_dialog\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{AlertDialog, AlertDialogVariant, OnPress};\n\nlet (open_raw, set_open_raw) = signal(false);\nlet open: Signal<bool> = Signal::derive(move || open_raw.get());\nlet on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));\n\n<AlertDialog open=open id_base=\"docs-alert\".to_string() title=\"Delete item?\".to_string() on_close=on_close confirm_label=\"Delete\".to_string() on_confirm=Callback::new(move |_| {}) variant=AlertDialogVariant::Destructive />".to_string()
                    label="Copy alert-dialog starter".to_string()
                    copyable=true
                    class_name="docs-alert-dialog-source-copy".to_string()
                />
                <ul data-slot="alert-dialog-source-paths">
                    <li><code>"components/alert-dialog/src/mod.rs"</code></li>
                    <li><code>"components/alert-dialog/src/logic.rs"</code></li>
                    <li><code>"components/alert-dialog/src/view.rs"</code></li>
                    <li><code>"components/alert-dialog/src/styles.rs"</code></li>
                    <li><code>"components/alert-dialog/src/motion.rs"</code></li>
                    <li><code>"components/alert-dialog/src/protocol.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
