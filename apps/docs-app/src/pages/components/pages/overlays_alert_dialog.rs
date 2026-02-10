use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{AlertDialog, AlertDialogVariant, Button, ButtonVariant, OnPress};

pub(super) fn alert_dialog() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_alert: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let (confirmed, set_confirmed) = signal(0u32);
    let on_confirm: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(1));
    });

    let on_secondary: OnPress = Callback::new(move |_| {
        set_confirmed.update(|value| *value = value.saturating_add(10));
    });

    let code = r#"<AlertDialog
  open=open
  id_base="a".to_string()
  title="Confirm".to_string()
  on_close=close
  confirm_label="Confirm".to_string()
  on_confirm=on_confirm
/>"#;

    let marker_code = r#"<AlertDialog
  open=open
  id_base="a-marker".to_string()
  title="Delete workspace?".to_string()
  description="Inspect source markers.".to_string()
  on_close=close
  confirm_label="Delete now".to_string()
  cancel_label="Dismiss".to_string()
  secondary_label="Save draft".to_string()
  on_confirm=on_confirm
  on_secondary=on_secondary
  auto_focus_button=ui_components::AlertDialogAutoFocusButton::Secondary
  secondary_disabled=true
  motion=ui_components::AlertDialogMotion {
    overlay: ui_components::OverlayMotion {
      initial_scale: 0.95,
      initial_y_px: 12.0,
      ..ui_components::OverlayMotion::default()
    }
  }
/>"#;

    view! {
        <ComponentPage
            title="AlertDialog"
            slug="alert-dialog"
            group="Overlays"
            description="Alertdialog role composition with destructive/default variants."
        >
            <Playground title="AlertDialog" code=code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Destructive on_press=open_alert>
                        "Open destructive"
                    </Button>
                    <span class="ui-muted">
                        "confirmed: " {move || confirmed.get().to_string()}
                    </span>
                </div>

                <Show when=move || present.get()>
                    <AlertDialog
                        open=open
                        id_base="docs-alert".to_string()
                        title="Delete item?".to_string()
                        description="Uses role=alertdialog with Overlay semantics.".to_string()
                        on_close=on_close
                        confirm_label="Delete".to_string()
                        on_confirm=on_confirm
                        variant=AlertDialogVariant::Destructive
                        on_exit_complete=on_exit_complete
                    />
                </Show>
            </Playground>

            <Playground title="State + Source Markers" code=marker_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=open_alert>
                            "Open marker alert"
                        </Button>
                        <span class="ui-muted">
                            "confirmed: " {move || confirmed.get().to_string()}
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-title-source / data-description-source / data-cancel-source / data-secondary-source / data-motion-source in DevTools."
                    </div>
                </div>

                <Show when=move || present.get()>
                    <AlertDialog
                        open=open
                        id_base="docs-alert-marker".to_string()
                        title="Delete workspace?".to_string()
                        description="Custom labels, secondary action, autofocus, and motion markers."
                        on_close=on_close
                        confirm_label="Delete now".to_string()
                        cancel_label="Dismiss".to_string()
                        secondary_label="Save draft".to_string()
                        on_confirm=on_confirm
                        on_secondary=on_secondary
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
                        on_exit_complete=on_exit_complete
                    />
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
