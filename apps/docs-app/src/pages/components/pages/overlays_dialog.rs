use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, Dialog, DialogMotion, OnPress, OverlayMotion};

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

    let code = r#"<Dialog open=open on_close=close id_base="d".to_string() title="Title".to_string()>
  move || view!{ ... }
</Dialog>"#;

    let motion_code = r#"<Dialog
  open=open
  on_close=close
  id_base="d-motion".to_string()
  title="Motion dialog".to_string()
  motion=DialogMotion {
    overlay: OverlayMotion {
      initial_scale: 0.94,
      initial_y_px: 14.0,
      ..OverlayMotion::default()
    }
  }
>
  ...
</Dialog>"#;

    view! {
        <ComponentPage
            title="Dialog"
            slug="dialog"
            group="Overlays"
            description="Dialog panel with header/body/footer structure on top of Overlay."
        >
            <Playground title="Dialog" code=code>
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

            <Playground title="Dialog custom motion" code=motion_code>
                <div class="docs-row">
                    <Button on_press=open_dialog variant=ButtonVariant::Secondary>
                        "Open motion-tuned dialog"
                    </Button>
                </div>

                <Show when=move || present.get()>
                    <Dialog
                        open=open
                        on_close=on_close
                        id_base="docs-dialog-motion".to_string()
                        title="Motion tuned dialog".to_string()
                        description="Uses custom DialogMotion -> OverlayMotion spring offsets.".to_string()
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
                            <div>"Custom motion path"</div>
                            <div class="ui-muted">
                                "Tuned via DialogMotion { overlay: OverlayMotion { initial_scale, initial_y_px, spring } }."
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
