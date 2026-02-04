use crate::alert_dialog::{AlertDialogMotion, AlertDialogVariant, logic};
use crate::overlay::Overlay;
use crate::{Button, ButtonVariant, OnPress};
use leptos::prelude::*;

#[component]
pub fn AlertDialog(
    open: Signal<bool>,
    id_base: String,
    title: String,
    on_close: OnPress,
    confirm_label: String,
    on_confirm: OnPress,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] cancel_label: Option<String>,
    #[prop(optional)] variant: AlertDialogVariant,
    #[prop(optional)] motion: AlertDialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
) -> impl IntoView {
    let cancel_label = cancel_label.unwrap_or_else(|| "Cancel".to_string());
    let view_state = logic::resolve_view_state(description.as_deref(), &cancel_label);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let cancel_label = StoredValue::new(cancel_label);
    let confirm_label = StoredValue::new(confirm_label);
    let class = StoredValue::new(format!("ui-alert-dialog {}", variant.class_name()));
    let on_close = StoredValue::new(on_close);
    let on_confirm = StoredValue::new(on_confirm);

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    let confirm_variant = match variant {
        AlertDialogVariant::Destructive => ButtonVariant::Destructive,
        AlertDialogVariant::Default => ButtonVariant::Accent,
    };

    if view_state.show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                <div class=move || class.get_value() data-slot="alert-dialog">
                    <div class="ui-alert-dialog__header" data-slot="alert-dialog-header">
                        <h2 class="ui-alert-dialog__title" id=move || title_id_attr.get() data-slot="alert-dialog-title">
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || view_state.show_description>
                            <p
                                class="ui-alert-dialog__description"
                                id=move || description_id_attr.get()
                                data-slot="alert-dialog-description"
                            >
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="ui-alert-dialog__footer" data-slot="alert-dialog-footer">
                        <Show when=move || view_state.show_cancel>
                            <Button variant=ButtonVariant::Secondary on_press=on_close.get_value()>
                                {move || cancel_label.get_value()}
                            </Button>
                        </Show>
                        <Button variant=confirm_variant on_press=on_confirm.get_value()>
                            {move || confirm_label.get_value()}
                        </Button>
                    </div>
                </div>
            </Overlay>
        }
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                <div class=move || class.get_value() data-slot="alert-dialog">
                    <div class="ui-alert-dialog__header" data-slot="alert-dialog-header">
                        <h2 class="ui-alert-dialog__title" id=move || title_id_attr.get() data-slot="alert-dialog-title">
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || view_state.show_description>
                            <p
                                class="ui-alert-dialog__description"
                                id=move || description_id_attr.get()
                                data-slot="alert-dialog-description"
                            >
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="ui-alert-dialog__footer" data-slot="alert-dialog-footer">
                        <Show when=move || view_state.show_cancel>
                            <Button variant=ButtonVariant::Secondary on_press=on_close.get_value()>
                                {move || cancel_label.get_value()}
                            </Button>
                        </Show>
                        <Button variant=confirm_variant on_press=on_confirm.get_value()>
                            {move || confirm_label.get_value()}
                        </Button>
                    </div>
                </div>
            </Overlay>
        }
    }
}
