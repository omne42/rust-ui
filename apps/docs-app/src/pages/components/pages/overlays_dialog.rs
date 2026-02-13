use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OnPress, OverlayMotion,
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
                        <span class="ui-muted">"open: " {move || open_raw.get().to_string()}</span>
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
        </ComponentPage>
    }
    .into_any()
}
