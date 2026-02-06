use leptos::prelude::*;
use ui_components::{
    AlertDialog, AlertDialogVariant, Button, ContextualHelp, Dialog, Drawer, DrawerPlacement,
    InlineAlert, InlineAlertFill, InlineAlertTone, Input, InputSize, InputVariant, OnPress,
};

#[component]
pub fn MoreComponentsDemo() -> impl IntoView {
    // Input
    let (input_value, set_input_value) = signal(String::new());
    let (input_invalid, set_input_invalid) = signal(false);

    // Dialog
    let (dialog_open, set_dialog_open) = signal(false);
    let is_dialog_open = Signal::derive(move || dialog_open.get());
    let (is_dialog_present, set_dialog_present) = signal(is_dialog_open.get_untracked());
    Effect::new(move |_| {
        if is_dialog_open.get() {
            set_dialog_present.set(true);
        }
    });
    let close_dialog: OnPress = Callback::new(move |_| set_dialog_open.set(false));
    let open_dialog: OnPress = Callback::new(move |_| set_dialog_open.set(true));
    let on_dialog_exit_complete: Callback<()> =
        Callback::new(move |_| set_dialog_present.set(false));

    // Drawer
    let (drawer_open, set_drawer_open) = signal(false);
    let is_drawer_open = Signal::derive(move || drawer_open.get());
    let (is_drawer_present, set_drawer_present) = signal(is_drawer_open.get_untracked());
    Effect::new(move |_| {
        if is_drawer_open.get() {
            set_drawer_present.set(true);
        }
    });
    let close_drawer: OnPress = Callback::new(move |_| set_drawer_open.set(false));
    let open_drawer: OnPress = Callback::new(move |_| set_drawer_open.set(true));
    let on_drawer_exit_complete: Callback<()> =
        Callback::new(move |_| set_drawer_present.set(false));

    // AlertDialog
    let (alert_open, set_alert_open) = signal(false);
    let is_alert_open = Signal::derive(move || alert_open.get());
    let (is_alert_present, set_alert_present) = signal(is_alert_open.get_untracked());
    Effect::new(move |_| {
        if is_alert_open.get() {
            set_alert_present.set(true);
        }
    });
    let close_alert: OnPress = Callback::new(move |_| set_alert_open.set(false));
    let open_alert: OnPress = Callback::new(move |_| set_alert_open.set(true));
    let on_alert_exit_complete: Callback<()> = Callback::new(move |_| set_alert_present.set(false));

    let on_confirm: OnPress = Callback::new(move |_| {
        set_input_invalid.set(false);
    });

    view! {
        <>
            <Show when=move || is_dialog_present.get()>
                <Dialog
                    open=is_dialog_open
                    on_close=close_dialog
                    id_base="demo-dialog".to_string()
                    title="Dialog".to_string()
                    description="A structured wrapper over Overlay.".to_string()
                    on_exit_complete=on_dialog_exit_complete
                    footer=move || view! { <Button on_press=close_dialog>"Close"</Button> }
                >
                    move || {
                        view! {
                            <div class="demo-stack">
                                <div class="demo-kv">
                                    "Dialog body content. Esc / click outside closes."
                                </div>
                                <div class="demo-row demo-row--end">
                                    <Button on_press=close_dialog>"Done"</Button>
                                </div>
                            </div>
                        }
                    }
                </Dialog>
            </Show>

            <Show when=move || is_drawer_present.get()>
                <Drawer
                    open=is_drawer_open
                    on_close=close_drawer
                    placement=DrawerPlacement::Right
                    id_base="demo-drawer".to_string()
                    title="Drawer".to_string()
                    description="Sheet + header/footer structure.".to_string()
                    on_exit_complete=on_drawer_exit_complete
                    footer=move || view! { <Button on_press=close_drawer>"Close"</Button> }
                >
                    move || {
                        view! {
                            <div class="demo-stack">
                                <div class="demo-kv">"Drawer body"</div>
                                <div class="demo-kv">"Try resizing: width is capped."</div>
                            </div>
                        }
                    }
                </Drawer>
            </Show>

            <Show when=move || is_alert_present.get()>
                <AlertDialog
                    open=is_alert_open
                    id_base="demo-alert-dialog".to_string()
                    title="Confirm action".to_string()
                    description="This uses Overlay with role=alertdialog.".to_string()
                    on_close=close_alert
                    confirm_label="Confirm".to_string()
                    on_confirm=on_confirm
                    variant=AlertDialogVariant::Destructive
                    on_exit_complete=on_alert_exit_complete
                />
            </Show>

            <section id="more-components" class="demo-card">
                <div class="demo-row">
                    <h2>"More components"</h2>
                    <ContextualHelp
                        heading="Contextual help".to_string()
                        footer=move || view! { "Popover-based" }
                    >
                        <div class="demo-stack">
                            <div>"Uses Button + Popover + spring motion."</div>
                            <div>"Works in Light/Dark/OLED via tokens."</div>
                        </div>
                    </ContextualHelp>
                </div>
                <p>"Input / InlineAlert / Dialog / Drawer / AlertDialog / ContextualHelp"</p>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"Input"</div>
                        <Input
                            id="demo-input".to_string()
                            label="Name".to_string()
                            value=input_value
                            set_value=set_input_value
                            placeholder="Type something…".to_string()
                            is_clearable=true
                            invalid=Signal::derive(move || input_invalid.get())
                            size=InputSize::Md
                            variant=InputVariant::Bordered
                        />
                        <div class="demo-row">
                            <Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_input_invalid.update(|v| *v = !*v))
                            >
                                {move || if input_invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                            </Button>
                        </div>
                        <InlineAlert tone=InlineAlertTone::Info fill=InlineAlertFill::Subtle title="Value".to_string()>
                            {move || input_value.get()}
                        </InlineAlert>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"InlineAlert"</div>
                        <InlineAlert
                            tone=InlineAlertTone::Neutral
                            fill=InlineAlertFill::Border
                            title="Neutral".to_string()
                            description="Uses token-based styling.".to_string()
                        >
                            "Extra content slot."
                        </InlineAlert>
                        <InlineAlert
                            tone=InlineAlertTone::Negative
                            fill=InlineAlertFill::Border
                            title="Error".to_string()
                            description="Negative tone maps to danger tokens.".to_string()
                        >
                            ""
                        </InlineAlert>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-row">
                    <Button on_press=open_dialog>"Open Dialog"</Button>
                    <Button on_press=open_drawer>"Open Drawer"</Button>
                    <Button variant=ui_components::ButtonVariant::Destructive on_press=open_alert>
                        "Open AlertDialog"
                    </Button>
                </div>
            </section>
        </>
    }
}
