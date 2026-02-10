use crate::dialog::{DialogMotion, DialogSize, logic};
use crate::overlay::Overlay;
use crate::{ButtonSize, ButtonVariant, IconButton, OnPress};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn Dialog(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional)] size: DialogSize,
    #[prop(optional, default = true)] show_close_button: bool,
    #[prop(optional, default = "Close")] close_label: &'static str,
    #[prop(optional)] motion: DialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);

    let has_footer = footer.get_value().is_some();

    let view_state = logic::resolve_view_state(
        description.get_value().as_deref(),
        has_footer,
        show_close_button,
    );

    let base_class = format!("ui-dialog {}", size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);
    let class = StoredValue::new(class);

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if view_state.show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                <div
                    class=move || class.get_value()
                    data-slot="dialog"
                    data-state=move || if open.get() { "open" } else { "closed" }
                    data-open=move || open.get().then_some("true")
                    data-closed=move || (!open.get()).then_some("true")
                    data-with-description=view_state.show_description.then_some("true")
                    data-with-footer=view_state.show_footer.then_some("true")
                    data-close-visible=view_state.show_close_button.then_some("true")
                    data-motion-source=if motion == DialogMotion::default() {
                        "default"
                    } else {
                        "custom"
                    }
                    data-custom-motion=(motion != DialogMotion::default()).then_some("true")
                >
                    <Show when=move || view_state.show_close_button>
                        <span class="ui-dialog__close" data-slot="dialog-close">
                            <IconButton
                                aria_label=close_label
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                on_press=on_close
                            >
                                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                    <path
                                        d="M5 5l10 10M15 5L5 15"
                                        stroke="currentColor"
                                        stroke_width="1.5"
                                        stroke_linecap="round"
                                        stroke_linejoin="round"
                                    />
                                </svg>
                            </IconButton>
                        </span>
                    </Show>

                    <div class="ui-dialog__header" data-slot="dialog-header">
                        <h2 class="ui-dialog__title" id=move || title_id_attr.get() data-slot="dialog-title">
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || view_state.show_description>
                            <p class="ui-dialog__description" id=move || description_id_attr.get() data-slot="dialog-description">
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="ui-dialog__body" data-slot="dialog-body">
                        {children()}
                    </div>

                    <Show when=move || view_state.show_footer>
                        <div class="ui-dialog__footer" data-slot="dialog-footer">
                            {move || footer.get_value().map(|slot| slot.run())}
                        </div>
                    </Show>
                </div>
            </Overlay>
        }
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete
            >
                <div
                    class=move || class.get_value()
                    data-slot="dialog"
                    data-state=move || if open.get() { "open" } else { "closed" }
                    data-open=move || open.get().then_some("true")
                    data-closed=move || (!open.get()).then_some("true")
                    data-with-description=view_state.show_description.then_some("true")
                    data-with-footer=view_state.show_footer.then_some("true")
                    data-close-visible=view_state.show_close_button.then_some("true")
                    data-motion-source=if motion == DialogMotion::default() {
                        "default"
                    } else {
                        "custom"
                    }
                    data-custom-motion=(motion != DialogMotion::default()).then_some("true")
                >
                    <Show when=move || view_state.show_close_button>
                        <span class="ui-dialog__close" data-slot="dialog-close">
                            <IconButton
                                aria_label=close_label
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                on_press=on_close
                            >
                                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                    <path
                                        d="M5 5l10 10M15 5L5 15"
                                        stroke="currentColor"
                                        stroke_width="1.5"
                                        stroke_linecap="round"
                                        stroke_linejoin="round"
                                    />
                                </svg>
                            </IconButton>
                        </span>
                    </Show>

                    <div class="ui-dialog__header" data-slot="dialog-header">
                        <h2 class="ui-dialog__title" id=move || title_id_attr.get() data-slot="dialog-title">
                            {move || title.get_value()}
                        </h2>
                        <Show when=move || view_state.show_description>
                            <p class="ui-dialog__description" id=move || description_id_attr.get() data-slot="dialog-description">
                                {move || description.get_value().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="ui-dialog__body" data-slot="dialog-body">
                        {children()}
                    </div>

                    <Show when=move || view_state.show_footer>
                        <div class="ui-dialog__footer" data-slot="dialog-footer">
                            {move || footer.get_value().map(|slot| slot.run())}
                        </div>
                    </Show>
                </div>
            </Overlay>
        }
    }
}
