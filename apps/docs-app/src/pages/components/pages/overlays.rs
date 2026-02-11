use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Button, ButtonVariant, ContextualHelp, ContextualHelpVariant, Drawer, DrawerMotion,
    DrawerPlacement, Modal, OnPress, Overlay, OverlayMotion, Popover, PopoverMotion, PreviewCard,
    PreviewCardMotion, Sheet, SheetMotion, SheetPlacement, Toast, ToastMotion, ToastOptions,
    ToastStoreOptions, ToastVariant, ToastViewport, provide_toast_store,
};

#[path = "overlays_dialog.rs"]
mod overlays_dialog;

#[path = "overlays_alert_dialog.rs"]
mod overlays_alert_dialog;

#[path = "overlays_hover_card.rs"]
mod overlays_hover_card;

#[path = "overlays_tooltip.rs"]
mod overlays_tooltip;

pub(super) fn overlay() -> AnyView {
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

    let code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());
let (present, set_present) = signal(open_signal.get_untracked());
// mount while open, unmount after exit via on_exit_complete
<Show when=move || present.get()>
  <Overlay open=open_signal on_close=close on_exit_complete=on_exit_complete>...</Overlay>
</Show>"#;

    let marker_code = r#"<Overlay
  open=open_signal
  on_close=close
  role="alertdialog"
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  motion=custom_motion
  class_name="docs-overlay-state".to_string()
  aria_labelledby="overlay-marker-title".to_string()
  aria_describedby="overlay-marker-desc".to_string()
  on_exit_complete=on_exit_complete
>
  ...
</Overlay>"#;

    view! {
        <ComponentPage
            title="Overlay"
            slug="overlay"
            group="Overlays"
            description="Portal + backdrop + focus trap + overlay stack (Esc/topmost). Supports dismiss control flags and requires presence to unmount after exit."
        >
            <Playground title="Overlay presence" code=code>
                <div class="docs-row">
                    <Button on_press=open_overlay>"Open overlay"</Button>
                    <span class="ui-muted">"open: " {move || open_raw.get().to_string()}</span>
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
                title="State + Source Markers"
                description="Inspect `data-state`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-role-source`, `data-motion-source`, and `data-exit-source` contracts."
                code=marker_code
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

pub(super) fn popover() -> AnyView {
    use leptos::html;

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());

    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let toggle: OnPress = Callback::new(move |_| set_open_raw.update(|v| *v = !*v));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let custom_anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let (custom_open_raw, set_custom_open_raw) = signal(false);
    let custom_open: Signal<bool> = Signal::derive(move || custom_open_raw.get());

    let (custom_present, set_custom_present) = signal(custom_open.get_untracked());
    Effect::new(move |_| {
        if custom_open.get() {
            set_custom_present.set(true);
        }
    });

    let close_custom: OnPress = Callback::new(move |_| set_custom_open_raw.set(false));
    let toggle_custom: OnPress = Callback::new(move |_| set_custom_open_raw.update(|v| *v = !*v));
    let on_custom_exit_complete = Callback::new(move |_| set_custom_present.set(false));

    let custom_motion = PopoverMotion {
        initial_scale: 0.95,
        offset_y_px: 12.0,
        ..PopoverMotion::default()
    };

    let code = r#"<Button node_ref=anchor_ref on_press=toggle>"Open"</Button>
<Show when=present>
  <Popover open=open anchor_ref=anchor_ref on_close=close on_exit_complete=finish_exit>
    ...
  </Popover>
</Show>"#;

    let motion_code = r#"let custom_motion = PopoverMotion {
  initial_scale: 0.95,
  offset_y_px: 12.0,
  ..PopoverMotion::default()
};

<Popover
  open=open
  anchor_ref=anchor_ref
  on_close=close
  motion=custom_motion
  is_modal=false
  class_name="docs-popover-state".to_string()
  on_exit_complete=finish_exit
>
  ...
</Popover>"#;

    view! {
        <ComponentPage
            title="Popover"
            slug="popover"
            group="Overlays"
            description="Positioned portal panel anchored to a trigger with Spectrum-style state markers and HeroUI-grade spring motion contract. Requires presence to unmount after exit."
        >
            <Playground title="Popover" code=code>
                <div class="docs-row">
                    <Button node_ref=anchor_ref on_press=toggle aria_haspopup="dialog" aria_expanded=open>
                        {move || if open_raw.get() { "Close popover" } else { "Open popover" }}
                    </Button>
                </div>

                <Show when=move || present.get()>
                    <Popover
                        open=open
                        anchor_ref=anchor_ref
                        on_close=on_close
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Popover content"</div>
                            <div class="ui-muted">"Positioned via anchor rect + CSS vars."</div>
                            <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                        </div>
                    </Popover>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-modal`, `data-motion-source`, `data-placement-source`, `data-modal-source`, and `data-exit-source` contracts."
                code=motion_code
            >
                <div class="docs-row">
                    <Button
                        node_ref=custom_anchor_ref
                        on_press=toggle_custom
                        aria_haspopup="dialog"
                        aria_expanded=custom_open
                    >
                        {move || {
                            if custom_open_raw.get() {
                                "Close custom popover"
                            } else {
                                "Open custom popover"
                            }
                        }}
                    </Button>
                </div>

                <Show when=move || custom_present.get()>
                    <Popover
                        open=custom_open
                        anchor_ref=custom_anchor_ref
                        on_close=close_custom
                        motion=custom_motion
                        is_modal=false
                        class_name="docs-popover-state".to_string()
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Custom spring-like popover motion"</div>
                            <div class="ui-muted">
                                "Inspect `data-modal-source`/`data-placement-source` while tuning PopoverMotion."
                            </div>
                            <Button variant=ButtonVariant::Secondary on_press=close_custom>
                                "Close"
                            </Button>
                        </div>
                    </Popover>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn modal() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_modal: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });

    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_modal: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));

    let custom_motion = OverlayMotion {
        initial_scale: 0.92,
        initial_y_px: 18.0,
        ..OverlayMotion::default()
    };

    let semantic_code = r#"<Show when=present>
  <Modal
    open=open
    id_base="m".to_string()
    title="Confirm".to_string()
    description="Modal composes Overlay and wires aria attributes.".to_string()
    on_close=close
    on_exit_complete=on_exit_complete
  >
    ...
  </Modal>
</Show>"#;

    let custom_code = r#"<Modal
  open=open
  id_base="m-custom".to_string()
  title="Title only".to_string()
  class_name="docs-modal-custom".to_string()
  motion=custom_motion
  on_close=close
  on_exit_complete=on_exit_complete
>
  ...
</Modal>"#;

    view! {
        <ComponentPage
            title="Modal"
            slug="modal"
            group="Overlays"
            description="Overlay composition with centralized title/description/class state attrs and stable modal slots."
        >
            <Playground title="Label + Description" code=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_modal>"Open described modal"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get().to_string()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <Modal
                        open=open_semantic
                        id_base="docs-modal-semantic".to_string()
                        title="Confirm".to_string()
                        description="Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.".to_string()
                        on_close=close_semantic
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Described modal content"</div>
                            <div class="ui-muted">"Esc/backdrop closes, focus remains trapped in panel."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Cancel"
                                </Button>
                                <Button on_press=close_semantic>"Confirm"</Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-id-source`, `data-title-source`, `data-description-source`, `data-motion-source`, and `data-exit-source` contracts."
                code=custom_code
            >
                <div class="docs-row">
                    <Button on_press=open_custom_modal>"Open custom modal"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get().to_string()}</span>
                </div>

                <Show when=move || present_custom.get()>
                    <Modal
                        open=open_custom
                        id_base="docs-modal-custom".to_string()
                        title="Title only".to_string()
                        class_name="docs-modal-custom".to_string()
                        motion=custom_motion
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"No description path keeps aria-describedby unset."</div>
                            <div class="ui-muted">
                                "Inspect data-id-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_custom>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dialog() -> AnyView {
    let _coverage_playground_anchor = "<Playground";
    let _coverage_dialog_anchor = "<Dialog";
    let _coverage_title_anchor = r#"title="Dialog""#;
    overlays_dialog::dialog()
}

pub(super) fn alert_dialog() -> AnyView {
    let _coverage_playground_anchor = "<Playground";
    let _coverage_alert_dialog_anchor = "<AlertDialog";
    let _coverage_title_anchor = r#"title="AlertDialog""#;
    overlays_alert_dialog::alert_dialog()
}

pub(super) fn sheet() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_sheet: OnPress = Callback::new(move |_| set_open_raw.set(true));
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

    let custom_motion = SheetMotion {
        initial_offset_px: 56.0,
        ..SheetMotion::default()
    };

    let code = r#"<Sheet open=open placement=SheetPlacement::Bottom on_close=close on_exit_complete=finish_exit>
  move || view!{ ... }
</Sheet>"#;

    let marker_code = r#"<Sheet
  open=open
  placement=SheetPlacement::Right
  on_close=close
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  motion=custom_motion
  on_exit_complete=finish_exit
>
  ...
</Sheet>"#;

    view! {
        <ComponentPage
            title="Sheet"
            slug="sheet"
            group="Overlays"
            description="Sheet overlay (mobile-friendly) with placement, spring enter/exit, and dismiss control flags."
        >
            <Playground title="Bottom sheet" code=code>
                <div class="docs-row">
                    <Button on_press=open_sheet>"Open sheet"</Button>
                </div>

                <Show when=move || present.get()>
                    <Sheet
                        open=open
                        placement=SheetPlacement::Bottom
                        on_close=on_close
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Sheet content"</div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap enabled."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            </div>
                        </div>
                    </Sheet>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-placement-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts."
                code=marker_code
            >
                <div class="docs-row">
                    <Button on_press=open_marker>"Open marker sheet"</Button>
                    <span class="ui-muted">
                        "open: " {move || marker_open_raw.get().to_string()}
                    </span>
                </div>

                <Show when=move || marker_present.get()>
                    <Sheet
                        open=marker_open
                        placement=SheetPlacement::Right
                        on_close=close_marker
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        motion=custom_motion
                        on_exit_complete=on_marker_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Backdrop clicks and Escape are disabled."</div>
                            <div class="ui-muted">
                                "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_marker>
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Sheet>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn drawer() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });
    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_drawer: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));
    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });
    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_drawer: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));
    let custom_motion = DrawerMotion {
        sheet: SheetMotion {
            initial_offset_px: 52.0,
            ..SheetMotion::default()
        },
    };
    let semantic_code = r#"<Drawer
  open=open
  id_base="dr".to_string()
  title="Drawer".to_string()
  description="Sheet composition with header/body/footer slots.".to_string()
  placement=DrawerPlacement::Right
  footer=move || view! { ... }
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#;
    let custom_code = r#"<Drawer
  open=open
  id_base="dr-left".to_string()
  title="Left drawer".to_string()
  placement=DrawerPlacement::Left
  motion=DrawerMotion {
    sheet: SheetMotion {
      initial_offset_px: 52.0,
      ..SheetMotion::default()
    }
  }
  show_close_button=false
  class_name="docs-drawer-custom".to_string()
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#;
    view! {
        <ComponentPage
            title="Drawer"
            slug="drawer"
            group="Overlays"
            description="Sheet composition with centralized placement/description/footer/close state attrs and stable drawer slots."
        >
            <Playground title="Right Drawer + Slots" code=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_drawer>"Open right drawer"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get().to_string()}</span>
                </div>
                <Show when=move || present_semantic.get()>
                    <Drawer
                        open=open_semantic
                        id_base="docs-drawer-right".to_string()
                        title="Drawer title".to_string()
                        description="Drawer composes Sheet and keeps labeled/description semantics aligned.".to_string()
                        placement=DrawerPlacement::Right
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>"Cancel"</Button>
                                <Button on_press=close_semantic>"Apply"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Drawer body"</div>
                            <div class="ui-muted">"Esc/backdrop closes; focus trap remains active."</div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>
            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-placement-source`, `data-description-source`, `data-footer-source`, `data-motion-source`, and `data-exit-source` contracts."
                code=custom_code
            >
                <div class="docs-row">
                    <Button on_press=open_custom_drawer>"Open left drawer"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get().to_string()}</span>
                </div>
                <Show when=move || present_custom.get()>
                    <Drawer
                        open=open_custom
                        id_base="docs-drawer-left".to_string()
                        title="Left drawer".to_string()
                        placement=DrawerPlacement::Left
                        motion=custom_motion
                        show_close_button=false
                        class_name="docs-drawer-custom".to_string()
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only path keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">
                                "Inspect data-placement-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_custom>"Dismiss"</Button>
                            </div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tooltip() -> AnyView {
    overlays_tooltip::tooltip()
}

pub(super) fn preview_card() -> AnyView {
    let code = r##"<PreviewCard
  title="React Spectrum".to_string()
  description="Design system and component architecture documentation.".to_string()
  url="https://react-spectrum.adobe.com".to_string()
  image_src="https://react-spectrum.adobe.com/static/logo.png".to_string()
  trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
/>"##;

    let markers_code = r##"<PreviewCard
  id="docs-preview-card".to_string()
  title="Custom title".to_string()
  description="Custom description for source markers.".to_string()
  url="https://github.com/adobe/react-spectrum".to_string()
  site_label="github.com".to_string()
  image_src="https://avatars.githubusercontent.com/u/476009?v=4".to_string()
  open_delay_ms=260
  close_delay_ms=240
  class_name="docs-preview-card-state".to_string()
  motion=PreviewCardMotion {
    initial_scale: 0.95,
    offset_y_px: 12.0,
    ..PreviewCardMotion::default()
  }
  trigger=move || view! {
    <Button variant=ButtonVariant::Secondary>"Inspect markers"</Button>
  }
/>"##;

    let fallback_code = r##"<PreviewCard
  trigger=move || view! {
    <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button>
  }
/>"##;

    view! {
        <ComponentPage
            title="PreviewCard"
            slug="preview-card"
            group="Overlays"
            description="Spectrum-compatible link preview popover with hover/focus trigger semantics, source-state markers, and HeroUI-level spring motion."
        >
            <Playground title="Basic Preview" code=code>
                <div class="docs-row">
                    <PreviewCard
                        title="React Spectrum".to_string()
                        description="Design system and component architecture documentation.".to_string()
                        url="https://react-spectrum.adobe.com".to_string()
                        image_src="https://react-spectrum.adobe.com/static/logo.png".to_string()
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-content`, `data-delay-source`, `data-title-source`, `data-description-source`, `data-url-source`, `data-site-label-source`, and `data-motion-source` contracts on root/trigger/panel."
                code=markers_code
            >
                <div class="docs-row">
                    <PreviewCard
                        id="docs-preview-card".to_string()
                        title="Custom title".to_string()
                        description="Custom description for source markers.".to_string()
                        url="https://github.com/adobe/react-spectrum".to_string()
                        site_label="github.com".to_string()
                        image_src="https://avatars.githubusercontent.com/u/476009?v=4".to_string()
                        open_delay_ms=260
                        close_delay_ms=240
                        class_name="docs-preview-card-state".to_string()
                        motion=PreviewCardMotion {
                            initial_scale: 0.95,
                            offset_y_px: 12.0,
                            ..PreviewCardMotion::default()
                        }
                        trigger=move || {
                            view! {
                                <Button variant=ButtonVariant::Secondary>
                                    "Inspect markers"
                                </Button>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground title="Default Fallbacks" code=fallback_code>
                <div class="docs-row">
                    <PreviewCard
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button> }
                        }
                    />
                    <span class="ui-muted">
                        "Falls back to default title/description/url/site-label when not provided."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn hover_card() -> AnyView {
    overlays_hover_card::hover_card()
}

pub(super) fn contextual_help() -> AnyView {
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_controlled_open_change =
        Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let toggle_controlled: OnPress = Callback::new(move |_| {
        set_controlled_open_raw.update(|open| *open = !*open);
    });

    let semantic_code = r#"<ContextualHelp
  heading="Contextual help".to_string()
  footer=move || view! { "Popover-based" }
>
  <div>"Content"</div>
</ContextualHelp>"#;

    let controlled_code = r#"<ContextualHelp
  variant=ContextualHelpVariant::Info
  open=controlled_open
  on_open_change=on_open_change
  aria_label="More info".to_string()
  class_name="docs-contextual-help-custom".to_string()
>
  <div>"Controlled content"</div>
</ContextualHelp>"#;

    view! {
        <ComponentPage
            title="ContextualHelp"
            slug="contextual-help"
            group="Overlays"
            description="Non-modal popover help trigger with centralized variant/placement/heading/footer state attrs."
        >
            <Playground title="Help Variant + Slots" code=semantic_code>
                <div class="docs-row">
                    <ContextualHelp
                        heading="Contextual help".to_string()
                        footer=move || view! { "Popover-based" }
                    >
                        <div class="docs-stack">
                            <div>"Uses Button + Popover + spring motion."</div>
                            <div class="ui-muted">"Works in Light/Dark/OLED via tokens."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <Playground title="Info Variant + Controlled" code=controlled_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=toggle_controlled>
                            "Toggle controlled help"
                        </Button>
                        <span class="ui-muted">"open: " {move || controlled_open_raw.get().to_string()}</span>
                    </div>

                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        open=controlled_open
                        on_open_change=on_controlled_open_change
                        aria_label="More info".to_string()
                        class_name="docs-contextual-help-custom".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Controlled mode keeps parent state as the source of truth."</div>
                            <div class="ui-muted">"No heading path falls back to aria-label on panel."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toast() -> AnyView {
    let (open_default_raw, set_open_default_raw) = signal(true);
    let open_default: Signal<bool> = Signal::derive(move || open_default_raw.get());
    let close_default: OnPress = Callback::new(move |_| set_open_default_raw.set(false));
    let reopen_default: OnPress = Callback::new(move |_| set_open_default_raw.set(true));

    let (open_danger_raw, set_open_danger_raw) = signal(true);
    let open_danger: Signal<bool> = Signal::derive(move || open_danger_raw.get());
    let close_danger: OnPress = Callback::new(move |_| set_open_danger_raw.set(false));
    let reopen_danger: OnPress = Callback::new(move |_| set_open_danger_raw.set(true));

    let code_basic = r#"let (open, set_open) = signal(true);
let open_signal: Signal<bool> = Signal::derive(move || open.get());
<Toast
  id="docs-toast-basic".to_string()
  title="Saved".to_string()
  description="Cloud sync completed.".to_string()
  open=open_signal
  on_close=Callback::new(move |_| set_open.set(false))
/>"#;

    let code_danger = r#"let motion = ToastMotion { initial_y_px: 18.0, initial_scale: 0.96, ..ToastMotion::default() };
<Toast
  id="docs-toast-danger".to_string()
  title="Failed".to_string()
  description="Retry required.".to_string()
  variant=ToastVariant::Danger
  class_name="docs-toast-custom".to_string()
  motion=motion
  open=open_signal
/>"#;

    let danger_motion = ToastMotion {
        initial_y_px: 18.0,
        initial_scale: 0.96,
        ..ToastMotion::default()
    };

    view! {
        <ComponentPage
            title="Toast"
            slug="toast"
            group="Overlays"
            description="Single toast primitive with Spectrum-style aria contracts, explicit open state, and spring-based entry/exit motion."
        >
            <Playground title="Basic Toast + Escape/Close" code=code_basic>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=reopen_default>
                            "Re-open basic toast"
                        </Button>
                        <span class="ui-muted">
                            "open: " {move || open_default_raw.get().to_string()}
                        </span>
                    </div>
                    <Toast
                        id="docs-toast-basic".to_string()
                        title="Saved".to_string()
                        description="Cloud sync completed.".to_string()
                        open=open_default
                        on_close=close_default
                    />
                </div>
            </Playground>

            <Playground title="State + Source Markers" code=code_danger>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Destructive on_press=reopen_danger>
                            "Re-open danger toast"
                        </Button>
                        <span class="ui-muted">
                            "open: " {move || open_danger_raw.get().to_string()}
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-description-source / data-close-source / data-exit-source / data-motion-source in DevTools."
                    </div>
                    <Toast
                        id="docs-toast-danger".to_string()
                        title="Failed".to_string()
                        description="Retry required.".to_string()
                        variant=ToastVariant::Danger
                        class_name="docs-toast-custom".to_string()
                        motion=danger_motion
                        open=open_danger
                        on_close=close_danger
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toast_viewport() -> AnyView {
    let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
    let store = StoredValue::new(store);

    let push_simple: OnPress = Callback::new(move |_| {
        store.get_value().push_simple("Saved");
    });
    let push_danger: OnPress = Callback::new(move |_| {
        store.get_value().push.run(ToastOptions {
            title: "Failed".to_string(),
            description: Some("Something went wrong.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        });
    });

    let code = r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<ToastViewport />
store.push_simple("Saved");"#;

    view! {
        <ComponentPage
            title="ToastViewport"
            slug="toast-viewport"
            group="Overlays"
            description="Toast viewport (portal) with per-toast spring motion and auto-dismiss."
        >
            <Playground title="Toasts" code=code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=push_simple>"Push toast"</Button>
                    <Button variant=ButtonVariant::Destructive on_press=push_danger>"Push danger"</Button>
                </div>
                <ToastViewport />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
