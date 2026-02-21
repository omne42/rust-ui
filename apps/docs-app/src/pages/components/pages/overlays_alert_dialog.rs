use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AlertDialog, AlertDialogVariant, Button, ButtonVariant, OnPress, SegmentedControl,
    SegmentedControlSize, Snippet,
};

const ALERT_DIALOG_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_components::{AlertDialog, AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogVariant, Button, ButtonVariant, OnPress, OverlayMotion, SegmentedControl, SegmentedControlSize};";

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
  auto_focus_button=ui_components::AlertDialogAutoFocusButton::Secondary
  secondary_disabled=true
  motion=ui_components::AlertDialogMotion {
    overlay: ui_components::OverlayMotion {
      initial_scale: 0.95,
      initial_y_px: 12.0,
      ..ui_components::OverlayMotion::default()
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
                        auto_focus_button=ui_components::AlertDialogAutoFocusButton::Secondary
                        secondary_disabled=true
                        motion=ui_components::AlertDialogMotion {
                            overlay: ui_components::OverlayMotion {
                                initial_scale: 0.95,
                                initial_y_px: 12.0,
                                ..ui_components::OverlayMotion::default()
                            },
                        }
                        on_exit_complete=on_marker_exit_complete
                    />
                </Show>
            </Playground>

            <Playground
                title="State Matrix"
                description="Matrix for variant/disabled combinations under the same semantic contract."
                code_signal=state_matrix_code
                code_imports=ALERT_DIALOG_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="alert-dialog-state-matrix">
                    <SegmentedControl
                        id_base="docs-alert-dialog-matrix".to_string()
                        options=state_matrix_options.clone()
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
                                    title=title.to_string()
                                    description=description.to_string()
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
                        "ui-components = { workspace = true, default-features = false, features = [\"component-alert_dialog\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::{AlertDialog, AlertDialogVariant, OnPress};\n\nlet (open_raw, set_open_raw) = signal(false);\nlet open: Signal<bool> = Signal::derive(move || open_raw.get());\nlet on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));\n\n<AlertDialog open=open id_base=\"docs-alert\".to_string() title=\"Delete item?\".to_string() on_close=on_close confirm_label=\"Delete\".to_string() on_confirm=Callback::new(move |_| {}) variant=AlertDialogVariant::Destructive />".to_string()
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
