use crate::{
    ButtonSize, ButtonVariant, IconButton, OnPress, Sheet, SheetPlacement, TrayMotion,
    tray::{TrayStateInput, logic},
};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn Tray(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional)] motion: TrayMotion,
    #[prop(optional, default = true)] show_close_button: bool,
    #[prop(optional, default = "Close tray")] close_label: &'static str,
    #[prop(optional)] is_fixed_height: bool,
    #[prop(optional, default = true)] is_dismissable: bool,
    #[prop(optional)] is_keyboard_dismiss_disabled: bool,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, "Tray");
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);

    let state = logic::resolve_state(TrayStateInput {
        has_description: description.get_value().is_some(),
        has_footer: footer.get_value().is_some(),
        show_close_button,
        is_fixed_height,
        has_custom_class_name: class_name.is_some(),
    });
    let class = StoredValue::new(logic::compose_class_name(class_name, state));

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if state.show_description {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=SheetPlacement::Bottom
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let children = children.get_value();
                    view! {
                        <div
                            class=move || class.get_value()
                            data-slot="tray"
                            data-state=state.state_attr
                            data-open=move || open.get().then_some("true")
                            data-closed=move || (!open.get()).then_some("true")
                            data-description=state.description_attr
                            data-footer=state.footer_attr
                            data-close-button=state.close_button_attr
                            data-size=state.size_attr
                            data-with-description=state.show_description.then_some("true")
                            data-with-footer=state.show_footer.then_some("true")
                            data-close-visible=state.show_close_button.then_some("true")
                            data-fixed-height=state.is_fixed_height.then_some("true")
                            data-custom-class=state.has_custom_class_name.then_some("true")
                            data-motion-source=if motion == TrayMotion::default() {
                                "default"
                            } else {
                                "custom"
                            }
                            data-custom-motion=(motion != TrayMotion::default()).then_some("true")
                        >
                            <Show when=move || state.show_close_button>
                                <span class="ui-tray__close" data-slot="tray-close">
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

                            <div class="ui-tray__header" data-slot="tray-header">
                                <h2 class="ui-tray__title" id=move || title_id_attr.get() data-slot="tray-title">
                                    {move || title.get_value()}
                                </h2>
                                <p
                                    class="ui-tray__description"
                                    id=move || description_id_attr.get()
                                    data-slot="tray-description"
                                >
                                    {move || description.get_value().unwrap_or_default()}
                                </p>
                            </div>

                            <div class="ui-tray__body" data-slot="tray-body">
                                {children()}
                            </div>

                            <Show when=move || state.show_footer>
                                <div class="ui-tray__footer" data-slot="tray-footer">
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
        .into_any()
    } else {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=SheetPlacement::Bottom
                aria_labelledby=title_id.clone()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let children = children.get_value();
                    view! {
                        <div
                            class=move || class.get_value()
                            data-slot="tray"
                            data-state=state.state_attr
                            data-open=move || open.get().then_some("true")
                            data-closed=move || (!open.get()).then_some("true")
                            data-description=state.description_attr
                            data-footer=state.footer_attr
                            data-close-button=state.close_button_attr
                            data-size=state.size_attr
                            data-with-description=state.show_description.then_some("true")
                            data-with-footer=state.show_footer.then_some("true")
                            data-close-visible=state.show_close_button.then_some("true")
                            data-fixed-height=state.is_fixed_height.then_some("true")
                            data-custom-class=state.has_custom_class_name.then_some("true")
                            data-motion-source=if motion == TrayMotion::default() {
                                "default"
                            } else {
                                "custom"
                            }
                            data-custom-motion=(motion != TrayMotion::default()).then_some("true")
                        >
                            <Show when=move || state.show_close_button>
                                <span class="ui-tray__close" data-slot="tray-close">
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

                            <div class="ui-tray__header" data-slot="tray-header">
                                <h2 class="ui-tray__title" id=move || title_id_attr.get() data-slot="tray-title">
                                    {move || title.get_value()}
                                </h2>
                            </div>

                            <div class="ui-tray__body" data-slot="tray-body">
                                {children()}
                            </div>

                            <Show when=move || state.show_footer>
                                <div class="ui-tray__footer" data-slot="tray-footer">
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
        .into_any()
    }
}
