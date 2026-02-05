use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AlertDialog, AlertDialogVariant, Button, ButtonVariant, ContextualHelp, Dialog, Drawer,
    DrawerPlacement, HoverCard, Modal, OnPress, Overlay, Popover, Sheet, SheetPlacement,
    ToastOptions, ToastStoreOptions, ToastVariant, ToastViewport, Tooltip, provide_toast_store,
};

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

    let code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());
let (present, set_present) = signal(open_signal.get_untracked());
// mount while open, unmount after exit via on_exit_complete
<Show when=move || present.get()>
  <Overlay open=open_signal on_close=close on_exit_complete=on_exit_complete>...</Overlay>
</Show>"#;

    view! {
        <ComponentPage
            title="Overlay"
            slug="overlay"
            group="Overlays"
            description="Portal + backdrop + focus trap + overlay stack (Esc/topmost). Requires presence to unmount after exit."
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

    let code = r#"<Button node_ref=anchor_ref on_press=toggle>"Open"</Button>
<Show when=present>
  <Popover open=open anchor_ref=anchor_ref on_close=close on_exit_complete=finish_exit>
    ...
  </Popover>
</Show>"#;

    view! {
        <ComponentPage
            title="Popover"
            slug="popover"
            group="Overlays"
            description="Positioned portal panel anchored to a trigger. Requires presence to unmount after exit."
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn modal() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_modal: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let code = r#"<Show when=present>
  <Modal open=open id_base="m".to_string() title="Title".to_string()
    on_close=close on_exit_complete=on_exit_complete>
    ...
  </Modal>
</Show>"#;

    view! {
        <ComponentPage
            title="Modal"
            slug="modal"
            group="Overlays"
            description="A composition wrapper over Overlay that provides a title (aria-labelledby)."
        >
            <Playground title="Modal" code=code>
                <div class="docs-row">
                    <Button on_press=open_modal>"Open modal"</Button>
                    <span class="ui-muted">"open: " {move || open_raw.get().to_string()}</span>
                </div>

                <Show when=move || present.get()>
                    <Modal
                        open=open
                        id_base="docs-modal".to_string()
                        title="Confirm".to_string()
                        description="Modal composes Overlay with title/description slots.".to_string()
                        on_close=on_close
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-row docs-row--end">
                            <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            <Button on_press=on_close>"Ok"</Button>
                        </div>
                    </Modal>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

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
        </ComponentPage>
    }
    .into_any()
}

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

    let on_confirm: OnPress = Callback::new(move |_| set_open_raw.set(false));

    let code = r#"<AlertDialog open=open id_base="a".to_string() title="Confirm".to_string()
  on_close=close confirm_label="Confirm".to_string() on_confirm=on_confirm />"#;

    view! {
        <ComponentPage
            title="AlertDialog"
            slug="alert-dialog"
            group="Overlays"
            description="Alertdialog role composition with destructive/default variants."
        >
            <Playground title="AlertDialog" code=code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Destructive on_press=open_alert>"Open destructive"</Button>
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
        </ComponentPage>
    }
    .into_any()
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

    let code = r#"<Sheet open=open placement=SheetPlacement::Bottom on_close=close on_exit_complete=finish_exit>
  move || view!{ ... }
</Sheet>"#;

    view! {
        <ComponentPage
            title="Sheet"
            slug="sheet"
            group="Overlays"
            description="Sheet overlay (mobile-friendly) with placement and spring enter/exit."
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn drawer() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_drawer: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let code = r#"<Drawer open=open id_base="dr".to_string() title="Drawer".to_string()
  on_close=close on_exit_complete=finish_exit>...</Drawer>"#;

    view! {
        <ComponentPage
            title="Drawer"
            slug="drawer"
            group="Overlays"
            description="Drawer is a Sheet composition with header/body/footer slots."
        >
            <Playground title="Right drawer" code=code>
                <div class="docs-row">
                    <Button on_press=open_drawer>"Open drawer"</Button>
                </div>

                <Show when=move || present.get()>
                    <Drawer
                        open=open
                        id_base="docs-drawer".to_string()
                        title="Drawer title".to_string()
                        description="This uses Sheet + slots.".to_string()
                        placement=DrawerPlacement::Right
                        on_close=on_close
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            </div>
                        }
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Drawer body"</div>
                            <div class="ui-muted">"Try Esc/backdrop close."</div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tooltip() -> AnyView {
    let code = r#"<Tooltip content=move || view!{ "Tooltip" }>
  <Button>"Hover"</Button>
</Tooltip>"#;

    view! {
        <ComponentPage
            title="Tooltip"
            slug="tooltip"
            group="Overlays"
            description="Tooltip with delay/warmup/cooldown and anchor positioning."
        >
            <Playground title="Hover / focus" code=code>
                <div class="docs-row">
                    <Tooltip content=move || view! { "This is a tooltip" }>
                        <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
                    </Tooltip>
                    <Tooltip content=move || view! { "Disabled" } disabled=true>
                        <Button variant=ButtonVariant::Secondary disabled=true>"Disabled"</Button>
                    </Tooltip>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn hover_card() -> AnyView {
    let code = r##"<HoverCard content=move || view!{ <div>...</div> }>
  <a href="#">"Hover"</a>
</HoverCard>"##;

    view! {
        <ComponentPage
            title="HoverCard"
            slug="hover-card"
            group="Overlays"
            description="Hover/focus triggered card with open/close delays."
        >
            <Playground title="HoverCard" code=code>
                <div class="docs-row">
                    <HoverCard content=move || view! {
                        <div class="docs-stack">
                            <div>"HoverCard content"</div>
                            <div class="ui-muted">"Moves with placement + spring enter/exit."</div>
                        </div>
                    }>
                        <a href="#" class="ui-muted" on:click=move |ev| ev.prevent_default()>
                            "Hover me"
                        </a>
                    </HoverCard>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn contextual_help() -> AnyView {
    let code = r#"<ContextualHelp heading="Help".to_string() footer=move || view!{ "Popover" }>
  <div>"Content"</div>
</ContextualHelp>"#;

    view! {
        <ComponentPage
            title="ContextualHelp"
            slug="contextual-help"
            group="Overlays"
            description="Icon trigger that opens a non-modal popover with heading/content/footer."
        >
            <Playground title="Help popover" code=code>
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
