use crate::underlay::{UnderlayPartStateInput, UnderlaySlot};
use crate::{OnPress, underlay::logic};
use leptos::prelude::*;

#[component]
pub fn Underlay(
    id_base: String,
    open: Signal<bool>,
    #[prop(optional)] transparent: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_close: Option<OnPress>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let on_close = StoredValue::new(on_close);
    let has_on_close = on_close.get_value().is_some();

    let has_custom_transparent = transparent != logic::DEFAULT_TRANSPARENT;
    let has_custom_disabled = disabled != logic::DEFAULT_DISABLED;

    let state = Memo::new(move |_| {
        logic::resolve_state(UnderlayPartStateInput {
            slot: UnderlaySlot::Root,
            open: open.get(),
            transparent,
            disabled,
            has_on_close,
            has_custom_transparent,
            has_custom_disabled,
            has_custom_close_handler: has_on_close,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let on_click = move |_| {
        let state = state.get();
        if !state.is_interactive {
            return;
        }

        if let Some(on_close) = on_close.get_value() {
            on_close.run(());
        }
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="presentation"
            aria-hidden="true"
            tabindex=-1
            on:click=on_click
            data-slot=move || state.get().slot_attr
            data-state=move || state.get().state_attr
            data-open=move || state.get().open_attr
            data-transparent=move || state.get().transparent_attr
            data-disabled=move || state.get().disabled_attr
            data-interactive=move || state.get().interactive_attr
            data-tone=move || state.get().tone_attr
            data-close-mode=move || state.get().close_mode_attr
            data-transparent-source=move || state.get().transparent_source_attr
            data-disabled-source=move || state.get().disabled_source_attr
            data-close-source=move || state.get().close_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-transparent=move || state.get().has_custom_transparent.then_some("true")
            data-custom-disabled=move || state.get().has_custom_disabled.then_some("true")
            data-custom-close=move || state.get().has_custom_close_handler.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        ></div>
    }
}
